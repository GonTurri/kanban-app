use crate::application::use_cases::board::BoardPersistence;
use crate::application::use_cases::column::ColumnPersistence;
use crate::application::use_cases::item::ItemPersistence;
use crate::entities::column_type::ColumnType;
use crate::prelude::*;
use crate::services::metrics_calculator::{ItemMetrics, ItemMetricsCalculator};
use std::sync::Arc;
use uuid::Uuid;

pub struct GetItemMetricsQuery {
    board_persistence: Arc<dyn BoardPersistence>,
    column_persistence: Arc<dyn ColumnPersistence>,
    item_persistence: Arc<dyn ItemPersistence>,
}

impl GetItemMetricsQuery {
    pub fn new(
        board_persistence: Arc<dyn BoardPersistence>,
        column_persistence: Arc<dyn ColumnPersistence>,
        item_persistence: Arc<dyn ItemPersistence>,
    ) -> Self {
        Self {
            column_persistence,
            board_persistence,
            item_persistence,
        }
    }
    pub async fn execute(&self, item_id: Uuid, action_user: Uuid) -> Result<Option<ItemMetrics>> {
        let item = self
            .item_persistence
            .get_item(item_id)
            .await?
            .ok_or(AppError::ResourceNotFound("Item", item_id))?;

        let board = self
            .board_persistence
            .get_board(item.board_id)
            .await?
            .ok_or(AppError::ResourceNotFound("Board", item.board_id))?;

        if !board.can_view_board(action_user) {
            return Err(AppError::InvalidCredentials);
        }

        if !item.is_done {
            return Ok(None);
        }

        let mut history = self.item_persistence.get_item_history(item_id).await?;
        let columns = self
            .column_persistence
            .get_by_board_id(item.board_id)
            .await?;

        let wip_columns: Vec<Uuid> = columns
            .iter()
            .filter(|c| matches!(c.kind, ColumnType::Wip { .. }))
            .map(|c| c.id)
            .collect();

        let done_columns: Vec<Uuid> = columns
            .iter()
            .filter(|c| matches!(c.kind, ColumnType::Done))
            .map(|c| c.id)
            .collect();

        let metrics =
            ItemMetricsCalculator::calculate(&item, &mut history, &wip_columns, &done_columns);

        Ok(metrics)
    }
}
