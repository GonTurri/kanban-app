use uuid::Uuid;
use crate::domain_error::{DomainError, DomainResult};
use crate::entities::board_column::BoardColumn;
use crate::entities::board_member::BoardMember;
use crate::entities::board_role::BoardRole;
use crate::entities::column_type::ColumnType;

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

    pub fn create_with_defaults(title: String, description: String, owner_id: Uuid) -> (Self, Vec<BoardColumn>) {
        let mut board = Self::new(title, description, owner_id);

        let owner_member = BoardMember::new(owner_id, BoardRole::Owner);

        let _ = board.add_member(owner_id, owner_member);

        let default_columns = vec![
            BoardColumn::new(board.id, "To Do".into(), ColumnType::Todo { limit: None }, 0),
            BoardColumn::new(board.id, "In Progress".into(), ColumnType::Wip { limit: Some(5) }, 1),
            BoardColumn::new(board.id, "Done".into(), ColumnType::Done, 2),
        ];

        (board, default_columns)
    }
    pub fn add_member(&mut self, action_user: Uuid, member: BoardMember) -> DomainResult<()>{
        if self.members.len() >= MAX_MEMBERS {
            return Err(DomainError::BoardMemberLimitExceeded);
        }

        if !self.is_owner(action_user) {
            return Err(DomainError::Static("Must be Owner to add members to a board"));
        }

        if self.members.iter().any(|m| m.user_id == member.user_id) {
            return Err(DomainError::Static("User already is a member of this board"));
        }

        self.members.push(member);
        Ok(())
    }

    pub fn change_member_role(&mut self, action_user: Uuid, target_user: Uuid, new_role: BoardRole) -> DomainResult<BoardMember>{
        if !self.is_owner(action_user) {
            return Err(DomainError::Static("Must be Owner of this board to change a members role"));
        }

        // swap owners case

        if new_role != BoardRole::Owner {
            let target_is_owner = target_user == self.owner_id || self.members.iter().any(|m| m.user_id == target_user && m.user_id == self.owner_id);

            if target_is_owner {
               self.check_owner_count_for_removal()?;
            }
        }

        if let Some(member) = self.members.iter_mut().find(|m| m.user_id == target_user) {
            member.role = new_role;
            Ok(member.clone())
        } else {
            Err(DomainError::MemberNotFound(target_user))
        }

    }

    pub fn remove_member(&mut self, action_user: Uuid, target_user: Uuid) -> DomainResult<BoardMember> {
        if !self.is_owner(action_user) {
            return Err(DomainError::Static("Must be Owner of this board to delete a member on this board"));
        }

        let is_target_owner = target_user == self.owner_id || self.members.iter().any(|m| m.user_id == target_user && m.role == BoardRole::Owner);

        if is_target_owner {
           self.check_owner_count_for_removal()?;
        }

        if let Some(index) = self.members.iter().position(|m| m.user_id == target_user) {
            let removed = self.members.remove(index);
            return Ok(removed);
        }

        Err(DomainError::Static("User is not a member of this board"))
    }

    pub fn can_edit_board(&self, user_id: Uuid) -> bool {
        self.members.iter().any(|m| m.user_id == user_id && m.can_edit_board())
    }

    pub fn can_view_board(&self, user_id: Uuid) -> bool {
        self.members.iter().any(|m| m.user_id == user_id && m.can_view_board() )
    }

    fn is_owner(&self, action_user: Uuid) -> bool {
        self.owner_id == action_user || self.members.iter().any(|m| m.user_id == action_user
            && m.is_owner())
    }

    fn check_owner_count_for_removal(&self) -> DomainResult<()> {
        let owner_count = self.members.iter().filter(|m| m.is_owner()).count();
        if owner_count <= 1 {
           return Err(DomainError::Static("Cannot leave board without owner"));
        }

        Ok(())
    }
}