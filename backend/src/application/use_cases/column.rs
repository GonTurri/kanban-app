use crate::application::use_cases::board::BoardPersistence;
use crate::entities::board_column::BoardColumn;
use crate::entities::column_type::ColumnType;
use crate::prelude::AppError;
use crate::prelude::*;
use async_trait::async_trait;
use std::sync::Arc;
use uuid::Uuid;

#[async_trait]
pub trait ColumnPersistence: Send + Sync {
    async fn get_item_count(&self, column_id: Uuid) -> Result<usize>;

    async fn create_column(&self, column: &BoardColumn) -> Result<Uuid>;

    async fn save_all(&self, columns: &[BoardColumn]) -> Result<()>;
    async fn get_column(&self, id: Uuid) -> Result<Option<BoardColumn>>;

    async fn get_by_board_id(&self, board_id: Uuid) -> Result<Vec<BoardColumn>>;
    async fn update_column(&self, column: &BoardColumn) -> Result<()>;

    async fn delete_column(&self, id: Uuid) -> Result<()>;
}

#[derive(Clone)]
pub struct ColumnUseCases {
    column_persistence: Arc<dyn ColumnPersistence>,
    board_persistence: Arc<dyn BoardPersistence>,
}

impl ColumnUseCases {
    pub fn new(
        column_persistence: Arc<dyn ColumnPersistence>,
        board_persistence: Arc<dyn BoardPersistence>,
    ) -> Self {
        Self {
            column_persistence,
            board_persistence,
        }
    }
    pub async fn add_board_column(
        &self,
        board_id: Uuid,
        action_user: Uuid,
        name: String,
        kind: ColumnType,
        target_index: usize,
    ) -> Result<Uuid> {
        let board = self
            .board_persistence
            .get_board(board_id)
            .await?
            .ok_or(AppError::ResourceNotFound("Board", board_id))?;

        if !board.can_edit_board(action_user) {
            return Err(AppError::InvalidCredentials);
        }

        let mut current_columns = self.column_persistence.get_by_board_id(board_id).await?;

        let target_index = target_index.min(current_columns.len());

        let new_column = BoardColumn::new(board_id, name, kind, target_index);
        let new_id = new_column.id;

        current_columns.sort_by_key(|c| c.order_index);

        current_columns.insert(target_index, new_column);

        for (index, column) in current_columns.iter_mut().enumerate() {
            column.order_index = index;
        }

        self.column_persistence.save_all(&current_columns).await?;

        Ok(new_id)
    }

    pub async fn update_column(
        &self,
        column_id: Uuid,
        action_user: Uuid,
        name: String,
        kind: ColumnType,
    ) -> Result<()> {
        let mut column = self
            .column_persistence
            .get_column(column_id)
            .await?
            .ok_or(AppError::ResourceNotFound("Column", column_id))?;

        let board = self
            .board_persistence
            .get_board(column.board_id)
            .await?
            .ok_or(AppError::ResourceNotFound("Board", column.board_id))?;

        if !board.can_edit_board(action_user) {
            return Err(AppError::InvalidCredentials);
        }

        column.name = name;
        column.kind = kind;

        self.column_persistence.update_column(&column).await?;

        Ok(())
    }

    pub async fn move_column(
        &self,
        board_id: Uuid,
        column_id: Uuid,
        action_user: Uuid,
        target_index: usize,
    ) -> Result<()> {
        let board = self
            .board_persistence
            .get_board(board_id)
            .await?
            .ok_or(AppError::ResourceNotFound("Board", board_id))?;

        if !board.can_edit_board(action_user) {
            return Err(AppError::InvalidCredentials);
        }

        let mut current_columns = self.column_persistence.get_by_board_id(board_id).await?;

        current_columns.sort_by_key(|c| c.order_index);

        let current_position = current_columns
            .iter()
            .position(|c| c.id == column_id)
            .ok_or(AppError::ResourceNotFound("Column", column_id))?;

        let column = current_columns.remove(current_position);

        let target_index = target_index.min(current_columns.len());

        current_columns.insert(target_index, column);

        for (i, col) in current_columns.iter_mut().enumerate() {
            col.order_index = i;
        }

        self.column_persistence.save_all(&current_columns).await?;

        Ok(())
    }

    pub async fn delete_column(&self, column_id: Uuid, action_user: Uuid) -> Result<()> {
        let column = self
            .column_persistence
            .get_column(column_id)
            .await?
            .ok_or(AppError::ResourceNotFound("Column", column_id))?;

        let board = self
            .board_persistence
            .get_board(column.board_id)
            .await?
            .ok_or(AppError::ResourceNotFound("Board", column.board_id))?;

        if !board.can_edit_board(action_user) {
            return Err(AppError::InvalidCredentials);
        }

        // Items and item history should be ON DELETE CASCADE
        self.column_persistence.delete_column(column_id).await?;

        Ok(())
    }
}
