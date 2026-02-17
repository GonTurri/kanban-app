use chrono::Utc;
use uuid::Uuid;

#[derive(Debug)]
pub struct ItemHistory {
    pub id: Uuid,
    pub item_id: Uuid,
    prev_column_id: Option<Uuid>,
    new_column_id: Option<Uuid>,
    timestamp: chrono::DateTime<Utc>,
}

impl ItemHistory {
    pub fn new(item_id: Uuid, prev_column_id: Option<Uuid>, new_column_id: Option<Uuid>) -> Self {
        Self {
            id: Uuid::new_v4(),
            item_id,
            prev_column_id,
            new_column_id,
            timestamp: Utc::now(),
        }
    }
}