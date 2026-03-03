use crate::application::use_cases::board::BoardPersistence;
use crate::application::use_cases::column::ColumnPersistence;
use crate::entities::item::Item;
use crate::entities::item_history::ItemHistory;
use crate::entities::item_priority::ItemPriority;
use crate::prelude::*;
use async_trait::async_trait;
use std::sync::Arc;
use uuid::Uuid;

#[async_trait]
pub trait ItemPersistence: Send + Sync {
    async fn get_items_by_column_with_limit_offset(
        &self,
        column_id: Uuid,
        limit: usize,
        offset: usize,
    ) -> Result<Vec<Item>>;

    async fn get_top_items_by_board(
        &self,
        board_id: Uuid,
        limit_by_column: i64,
    ) -> Result<Vec<Item>>;
    async fn get_item_history(&self, item_id: Uuid) -> Result<Vec<ItemHistory>>;
    async fn get_item(&self, id: Uuid) -> Result<Option<Item>>;
    async fn create_item(&self, item: &Item, history: &ItemHistory) -> Result<Uuid>;

    async fn update_item(&self, item: &Item, history: Option<&ItemHistory>) -> Result<()>;

    async fn delete_item(&self, id: Uuid) -> Result<()>;
}

#[derive(Clone)]
pub struct ItemUseCases {
    board_persistence: Arc<dyn BoardPersistence>,
    column_persistence: Arc<dyn ColumnPersistence>,
    item_persistence: Arc<dyn ItemPersistence>,
}

impl ItemUseCases {
    pub fn new(
        board_persistence: Arc<dyn BoardPersistence>,
        column_persistence: Arc<dyn ColumnPersistence>,
        item_persistence: Arc<dyn ItemPersistence>,
    ) -> Self {
        Self {
            board_persistence,
            column_persistence,
            item_persistence,
        }
    }

    pub async fn get_item_history(
        &self,
        item_id: Uuid,
        action_user: Uuid,
    ) -> Result<Vec<ItemHistory>> {
        let item = self
            .item_persistence
            .get_item(item_id)
            .await?
            .ok_or(AppError::ResourceNotFound("Item", item_id))?;

        let board = self
            .board_persistence
            .get_board(item.board_id)
            .await?
            .unwrap();

        if !board.can_view_board(action_user) {
            return Err(AppError::InvalidCredentials);
        }

        self.item_persistence.get_item_history(item_id).await
    }

    pub async fn get_items(
        &self,
        column_id: Uuid,
        limit: usize,
        offset: usize,
        action_user: Uuid,
    ) -> Result<Vec<Item>> {
        let column = self
            .column_persistence
            .get_column(column_id)
            .await?
            .ok_or(AppError::ResourceNotFound("Column", column_id))?;

        let board = self
            .board_persistence
            .get_board(column.board_id)
            .await?
            .unwrap();

        if !board.can_view_board(action_user) {
            return Err(AppError::InvalidCredentials);
        }

        self.item_persistence
            .get_items_by_column_with_limit_offset(column_id, limit, offset)
            .await
    }
    pub async fn add_item(
        &self,
        column_id: Uuid,
        title: String,
        description: Option<String>,
        assigned_to: Option<Uuid>,
        priority: ItemPriority,
        action_user: Uuid,
    ) -> Result<Uuid> {
        let column = self
            .column_persistence
            .get_column(column_id)
            .await?
            .ok_or(AppError::ResourceNotFound("Column", column_id))?;

        let board = self
            .board_persistence
            .get_board(column.board_id)
            .await?
            .unwrap();

        if !board.can_edit_board(action_user) {
            return Err(AppError::InvalidCredentials);
        }

        let current_item_count = self.column_persistence.get_item_count(column_id).await?;

        let (item, history) = column.new_item(
            current_item_count,
            title,
            description,
            assigned_to,
            priority,
        )?;

        self.item_persistence.create_item(&item, &history).await?;

        Ok(item.id)
    }

    pub async fn update_item_details(
        &self,
        item_id: Uuid,
        title: String,
        description: Option<String>,
        assigned_to: Option<Uuid>,
        priority: ItemPriority,
        action_user: Uuid,
    ) -> Result<()> {
        let mut item = self
            .item_persistence
            .get_item(item_id)
            .await?
            .ok_or(AppError::ResourceNotFound("Item", item_id))?;

        let board = self
            .board_persistence
            .get_board(item.board_id)
            .await?
            .unwrap();

        if !board.can_edit_board(action_user) {
            return Err(AppError::InvalidCredentials);
        }

        item.title = title;
        item.description = description;
        item.assigned_to = assigned_to;
        item.priority = priority;

        self.item_persistence.update_item(&item, None).await?;

        Ok(())
    }

    pub async fn move_item(
        &self,
        item_id: Uuid,
        new_column: Uuid,
        action_user: Uuid,
    ) -> Result<()> {
        let mut item = self
            .item_persistence
            .get_item(item_id)
            .await?
            .ok_or(AppError::ResourceNotFound("Item", item_id))?;

        let column = self
            .column_persistence
            .get_column(new_column)
            .await?
            .ok_or(AppError::ResourceNotFound("Column", new_column))?;

        let board = self
            .board_persistence
            .get_board(column.board_id)
            .await?
            .unwrap();

        if !board.can_edit_board(action_user) {
            return Err(AppError::InvalidCredentials);
        }

        let current_item_count = self.column_persistence.get_item_count(column.id).await?;

        let history = column.move_item(current_item_count, &mut item)?;

        self.item_persistence
            .update_item(&item, Some(&history))
            .await?;

        Ok(())
    }

    pub async fn delete_item(&self, item_id: Uuid, action_user: Uuid) -> Result<()> {
        let item = self
            .item_persistence
            .get_item(item_id)
            .await?
            .ok_or(AppError::ResourceNotFound("Item", item_id))?;

        let board = self
            .board_persistence
            .get_board(item.board_id)
            .await?
            .unwrap();

        if !board.can_edit_board(action_user) {
            return Err(AppError::InvalidCredentials);
        }

        self.item_persistence.delete_item(item_id).await?;

        Ok(())
    }
}
