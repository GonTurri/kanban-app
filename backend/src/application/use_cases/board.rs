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
use tracing::{info, instrument};
use uuid::Uuid;

#[derive(Serialize)]
pub struct BoardResponseDto {
    pub id: Uuid,
    pub title: String,
    pub description: String,
    pub columns: Vec<ColumnResponseDto>,
}

#[derive(Serialize)]
pub struct ColumnResponseDto {
    pub id: Uuid,
    pub name: String,
    pub items: Vec<ItemResponseDto>,
}

#[derive(Serialize)]
pub struct ItemResponseDto {
    pub id: Uuid,
    pub title: String,
}

#[async_trait]
pub trait BoardPersistence: Send + Sync {
    async fn create_board(&self, board: &Board, columns: &[BoardColumn]) -> Result<Uuid>;
    async fn get_board(&self, id: Uuid) -> Result<Option<Board>>;

    async fn add_member_to_board(&self, board_id: Uuid, member: &BoardMember) -> Result<Uuid>;

    async fn update_member(&self, member: &BoardMember) -> Result<()>;

    async fn remove_member_from_board(&self, member_id: Uuid) -> Result<()>;

    async fn exists_by_id(&self, id: Uuid) -> Result<bool>;
}

#[derive(Clone)]
pub struct BoardUseCases {
    board_persistence: Arc<dyn BoardPersistence>,
    user_persistence: Arc<dyn UserPersistence>,
}

impl BoardUseCases {
    pub fn new(board_persistence: Arc<dyn BoardPersistence>, user_persistence: Arc<dyn UserPersistence>) -> Self {
        Self { board_persistence, user_persistence }
    }

    pub async fn get_full_board(&self, board_id: Uuid) -> Result<()> {
        todo!()
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
    pub async fn add_member(&self,board_id: Uuid, action_user: Uuid, user: Uuid, role: BoardRole) -> Result<()> {

        info!("Adding new board member to board {:?}...", board_id);

        self.validate_user_exists(action_user).await?;
        self.validate_user_exists(user).await?;

        let mut board = self.board_persistence.get_board(board_id).await?.ok_or(AppError::ResourceNotFound("Board", board_id))?;
        let new_member = BoardMember::new(user, role);

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
