use sqlx::postgres::types::PgCube::Point;
use uuid::Uuid;
use crate::entities::board_role::BoardRole;

#[derive(Debug)]
pub struct BoardMember {
    pub id: Uuid,
    pub user_id: Uuid,
    pub role: BoardRole,
}

impl BoardMember {
    pub fn new (user_id: Uuid, role: BoardRole) -> Self {
        Self {
            id: Uuid::new_v4(),
            user_id,
            role
        }
    }
}