use crate::entities::item_history::ItemHistory;
use crate::entities::item_priority::ItemPriority;
use uuid::Uuid;
use crate::domain_error::{DomainError, DomainResult};

#[derive(Debug)]
pub struct Item {
    pub id: Uuid,
    pub title: String,
    pub description: Option<String>,
    pub priority: ItemPriority,
    pub assigned_to: Option<Uuid>,
    pub column_id: Uuid,
    pub board_id: Uuid,
}

impl Item {
    pub fn new(
        title: String,
        description: Option<String>,
        assigned_to: Option<Uuid>,
        priority: ItemPriority,
        column_id: Uuid,
        board_id: Uuid,
    ) -> (Self, ItemHistory) {
       let item = Self {
            id: Uuid::new_v4(),
            title: title.into(),
            description,
            priority,
            assigned_to,
            column_id,
            board_id,
        };
        let first_history = ItemHistory::new(item.id, None, Some(item.column_id));

        (item, first_history)
    }

    pub fn move_to_column(&mut self, column_id: Uuid) -> DomainResult<ItemHistory> {

        if(self.column_id == column_id) {
           return Err(DomainError::Static("Item is already in this column"));
        }

        let prev_column = self.column_id;
        self.column_id = column_id;
        Ok(ItemHistory::new(self.id, Some(prev_column), Some(self.column_id)))
    }

    pub fn assign(&mut self, user: Uuid){
        self.assigned_to = Some(user);
    }
}
