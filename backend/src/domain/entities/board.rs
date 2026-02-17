use uuid::Uuid;
use crate::domain_error::{DomainError, DomainResult};
use crate::entities::board_member::BoardMember;
use crate::entities::board_role::BoardRole;
use crate::prelude::*;

const MAX_MEMBERS: usize = 1000;

#[derive(Debug)]
pub struct Board {
    pub id: Uuid,
    pub title: String,
    pub description: String,
    pub owner_id: Uuid,
    pub members: Vec<BoardMember>,
}

impl Board {
    
    pub fn new (title: String, description: String, owner_id: Uuid) -> Self {
        // use case should create a new member for owner
        Self {
            id: Uuid::new_v4(),
            title,
            description,
            owner_id,
            members: Vec::new(),
        }
    }
    pub fn add_member(&mut self, action_user: Uuid, member: BoardMember) -> DomainResult<()>{
        if self.members.len() >= MAX_MEMBERS {
            return Err(DomainError::BoardMemberLimitExceeded);
        }

        let is_admin = self.owner_id == action_user || self.members.iter().any(|m| m.user_id == action_user
        && m.role == BoardRole::Owner);

        if !is_admin {
            return Err(DomainError::Static("Must be Owner to add members to a board"));
        }

        if self.members.iter().any(|m| m.user_id == member.user_id) {
            return Err(DomainError::Static("User already is a member of this board"));
        }

        self.members.push(member);
        Ok(())
    }
}