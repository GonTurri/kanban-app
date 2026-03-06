use crate::domain_error::DomainError::{TodoLimitExceeded, WipLimitExceeded};
use crate::domain_error::{DomainError, DomainResult};
use crate::entities::column_type::ColumnType;
use crate::entities::item::Item;
use crate::entities::item_history::ItemHistory;
use crate::entities::item_priority::ItemPriority;
use uuid::Uuid;

#[derive(Debug)]
pub struct BoardColumn {
    pub id: Uuid,
    pub board_id: Uuid,
    pub name: String,
    pub kind: ColumnType,
    pub order_index: usize,
}

impl BoardColumn {
    pub fn new(board_id: Uuid, name: String, kind: ColumnType, order_index: usize) -> Self {
        Self {
            id: Uuid::new_v4(),
            board_id,
            name,
            kind,
            order_index,
        }
    }

    pub fn new_item(
        &self,
        current_item_count: usize,
        title: String,
        description: Option<String>,
        assigned_to: Option<Uuid>,
        priority: ItemPriority,
    ) -> DomainResult<(Item, ItemHistory)> {
        self.can_accept_new_item(current_item_count)?;

        Ok(Item::new(
            title,
            description,
            assigned_to,
            priority,
            self.id,
            self.board_id,
        ))
    }

    pub fn move_item(
        &self,
        current_item_count: usize,
        item: &mut Item,
    ) -> DomainResult<ItemHistory> {
        if item.board_id != self.board_id {
            return Err(DomainError::Static(
                "Item is not in the same board as column",
            ));
        }

        self.can_accept_new_item(current_item_count)?;

        let is_done = match self.kind {
            ColumnType::Done => true,
            _ => false,
        };

        item.move_to_column(self.id, is_done)
    }

    fn can_accept_new_item(&self, current_items_count: usize) -> DomainResult<()> {
        match self.kind {
            ColumnType::Todo { limit: Some(limit) } if limit <= current_items_count => {
                Err(TodoLimitExceeded(limit))
            }
            ColumnType::Wip { limit: Some(limit) } if limit <= current_items_count => {
                Err(WipLimitExceeded(limit))
            }
            _ => Ok(()),
        }
    }
}
