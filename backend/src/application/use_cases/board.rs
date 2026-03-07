use std::collections::HashMap;
use crate::application::use_cases::user::UserPersistence;
use crate::entities::board::Board;
use crate::entities::board_column::BoardColumn;
use crate::entities::board_member::BoardMember;
use crate::entities::board_role::BoardRole;
use crate::entities::column_type::ColumnType;
use crate::prelude::*;
use async_trait::async_trait;
use serde::Serialize;
use std::sync::Arc;
use chrono::{DateTime, Utc};
use tracing::{info, instrument};
use uuid::Uuid;
use crate::adapters::persistence::board::{BoardMemberWithUserDb, BoardWithMembersViewDb};
use crate::entities::item::Item;
use crate::entities::item_priority::ItemPriority;
use crate::use_cases::column::ColumnPersistence;
use crate::use_cases::item::ItemPersistence;

const ITEM_FETCH_LIMIT_BY_BOARD: i64 = 10;

#[derive(Serialize, Debug)]
pub struct BoardResponseDto {
    pub id: Uuid,
    pub title: String,
    pub description: String,
    pub owner_id: Uuid,
    pub members: Vec<BoardMemberResponseDto>,
    pub columns: Vec<ColumnResponseDto>,
}

#[derive(Serialize, Debug)]
pub struct BoardMemberResponseDto {
    pub id: Uuid,
    pub user_id: Uuid,
    pub email: String,
    pub user_name: String,
    pub role: BoardRole,
}

#[derive(Serialize, Debug)]
pub struct ColumnResponseDto {
    pub id: Uuid,
    pub name: String,
    pub order_index: usize,
    pub kind: ColumnType,
    pub items: Vec<ItemResponseDto>,
}

#[derive(Serialize, Debug)]
pub struct ItemResponseDto {
    pub id: Uuid,
    pub title: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    pub priority: ItemPriority,
    pub assigned_to: Option<Uuid>,
    pub is_done: bool,
    pub created_at: DateTime<Utc>,
}

#[derive(Serialize, Debug)]
pub struct BoardSummaryDto {
    pub id: Uuid,
    pub title: String,
    pub description: String,
    pub role: BoardRole,
}

impl From<Item> for ItemResponseDto {
    fn from(value: Item) -> Self {
        Self {
            id: value.id,
            title: value.title,
            description: value.description,
            priority: value.priority,
            assigned_to: value.assigned_to,
            is_done: value.is_done,
            created_at: value.created_at
        }
    }
}

impl From<BoardMemberWithUserDb> for BoardMemberResponseDto {
    fn from(value: BoardMemberWithUserDb) -> Self {
        Self {
            id: value.id,
            user_id: value.user_id,
            role: value.role.into(),
            user_name: value.username,
            email: value.email

        }
    }
}

impl From<BoardMemberWithUserDb> for BoardMember {
    fn from(value: BoardMemberWithUserDb) -> Self {
        Self {
            id: value.id,
            user_id: value.user_id,
            role: value.role.into(),
        }
    }
}
impl From<BoardWithMembersViewDb> for Board {
    fn from(value: BoardWithMembersViewDb) -> Self {
        Self {
            id: value.id,
            owner_id: value.owner_id,
            title: value.title,
            description: value.description,
            members: value.members.0.into_iter().map(Into::into).collect(),
        }
    }
}

#[async_trait]
pub trait BoardPersistence: Send + Sync {
    async fn create_board(&self, board: &Board, columns: &[BoardColumn]) -> Result<Uuid>;
    async fn get_board(&self, id: Uuid) -> Result<Option<Board>>;
    async fn get_board_with_member_users(
        &self,
        id: Uuid
    ) -> Result<Option<BoardWithMembersViewDb>>;

    async fn add_member_to_board(&self, board_id: Uuid, member: &BoardMember) -> Result<Uuid>;

    async fn update_member(&self, member: &BoardMember) -> Result<()>;

    async fn remove_member_from_board(&self, member_id: Uuid) -> Result<()>;

    async fn exists_by_id(&self, id: Uuid) -> Result<bool>;

    async fn get_user_boards(&self, user_id: Uuid) -> Result<Vec<Board>>;
}

#[derive(Clone)]
pub struct BoardUseCases {
    board_persistence: Arc<dyn BoardPersistence>,
    column_persistence: Arc<dyn ColumnPersistence>,
    item_persistence: Arc<dyn ItemPersistence>,
    user_persistence: Arc<dyn UserPersistence>,
}

impl BoardUseCases {
    pub fn new(board_persistence: Arc<dyn BoardPersistence>,column_persistence: Arc<dyn ColumnPersistence>, item_persistence: Arc<dyn ItemPersistence> , user_persistence: Arc<dyn UserPersistence>, ) -> Self {
        Self { board_persistence, column_persistence, item_persistence, user_persistence }
    }

    pub async fn get_user_boards(&self, user_id: Uuid) -> Result<Vec<BoardSummaryDto>>{
        self.validate_user_exists(user_id).await?;

        let boards = self.board_persistence.get_user_boards(user_id).await?;

        let result = boards.into_iter().map(|b| {
            let role = b.get_member(user_id).map(|m| m.role).unwrap_or(BoardRole::Viewer);

            BoardSummaryDto {
                id: b.id,
                title: b.title,
                description: b.description,
                role
            }

        }).collect();

        Ok(result)
    }

    pub async fn get_full_board(
        &self,
        board_id: Uuid,
        action_user: Uuid
    ) -> Result<BoardResponseDto> {

        let board_view = self.board_persistence
            .get_board_with_member_users(board_id)
            .await?
            .ok_or(AppError::ResourceNotFound("Board", board_id))?;

        let board: Board = board_view.clone().into();

        if !board.can_view_board(action_user) {
            return Err(AppError::InvalidCredentials)
        }

        let columns = self.column_persistence.get_by_board_id(board_id).await?;

        let items = self.item_persistence
            .get_top_items_by_board(board_id, ITEM_FETCH_LIMIT_BY_BOARD)
            .await?;

        let mut items_by_column: HashMap<Uuid, Vec<ItemResponseDto>> =
            HashMap::with_capacity(columns.len());

        for item in items {
            items_by_column
                .entry(item.column_id)
                .or_default()
                .push(item.into())
        }

        let columns_dto: Vec<ColumnResponseDto> = columns
            .into_iter()
            .map(|col| ColumnResponseDto {
                id: col.id,
                name: col.name,
                kind: col.kind,
                order_index: col.order_index,
                items: items_by_column.remove(&col.id).unwrap_or_default()
            })
            .collect();

        let BoardWithMembersViewDb {
            id,
            owner_id,
            title,
            description,
            members,
        } = board_view;

        let board_dto = BoardResponseDto {
            id,
            title,
            description,
            owner_id,
            members: members.0.into_iter().map(Into::into).collect(),
            columns: columns_dto
        };

        Ok(board_dto)
    }

    #[instrument(skip(self))]
    pub async fn create_board(&self, title: String, description: String, owner_id: Uuid) -> Result<Uuid> {
        info!("Adding new board...");

        self.validate_user_exists(owner_id).await?;

        let (board, columns) =  Board::create_with_defaults(title, description, owner_id);
        
        self.board_persistence.create_board(&board, &columns).await?;

        info!("Adding Board finished.");
        
        Ok(board.id)
    }

    #[instrument(skip(self))]
    pub async fn add_member(&self, board_id: Uuid, action_user: Uuid, user_email: &str, role: BoardRole) -> Result<()> {

        info!("Adding new board member to board {:?}...", board_id);

        self.validate_user_exists(action_user).await?;
        let user = self.user_persistence.get_by_email(user_email).await?.ok_or(AppError::UserEmailNotFound(user_email.to_owned()))?;

        let mut board = self.board_persistence.get_board(board_id).await?.ok_or(AppError::ResourceNotFound("Board", board_id))?;
        let new_member = BoardMember::new(user.id, role);

        board.add_member(action_user, new_member)?;

        self.board_persistence.add_member_to_board(board_id, &new_member).await?;

        info!("Finished adding member to board");
        Ok(())
    }

    #[instrument(skip(self))]
    pub async fn change_member_role(&self, board_id: Uuid, action_user: Uuid, target_user: Uuid, role: BoardRole) -> Result<()> {

        info!("chaging role of member {:?} in board {:?} to {:?}...", target_user, board_id, role);

        self.validate_user_exists(action_user).await?;
        self.validate_user_exists(target_user).await?;

        let mut board = self.board_persistence.get_board(board_id).await?.ok_or(AppError::ResourceNotFound("Board", board_id))?;
        let changed = board.change_member_role(action_user, target_user, role)?;

        self.board_persistence.update_member(&changed).await?;

        info!("Finished changing role");

        Ok(())
    }

    pub async fn remove_member_from_board(&self, board_id: Uuid,action_user:Uuid ,member_id: Uuid) -> Result<()> {
        self.validate_user_exists(action_user).await?;
        self.validate_user_exists(member_id).await?;

        let mut board = self.board_persistence.get_board(board_id).await?.ok_or(AppError::ResourceNotFound("Board", board_id))?;

        let removed = board.remove_member(action_user, member_id)?;

        self.board_persistence.remove_member_from_board(removed.id).await?;

        Ok(())
    }

    async fn validate_user_exists(&self, user: Uuid) -> Result<()> {
        let exists_user = self.user_persistence.exists_by_id(user).await?;

        if !exists_user {
            return Err(AppError::ResourceNotFound("User", user));
        }

        Ok(())
    }
}
