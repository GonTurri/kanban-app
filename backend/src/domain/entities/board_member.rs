use crate::entities::board_role::BoardRole;
use uuid::Uuid;

#[derive(Debug, Clone, Copy)]
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
    
    pub fn can_edit_board(&self) -> bool {
        self.role == BoardRole::Owner || self.role == BoardRole::Editor
    }
    
    pub fn can_view_board(&self) -> bool {
        self.role == BoardRole::Owner || self.role == BoardRole::Editor || self.role == BoardRole::Viewer
    }
     
    pub fn is_owner(&self) -> bool {
        self.role == BoardRole::Owner
    }
    
}