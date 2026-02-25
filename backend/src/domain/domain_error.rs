use thiserror::Error;
use uuid::Uuid;

#[derive(Debug, Error)]
pub enum DomainError {

    #[error("Generic error: {0}")]
    Static(&'static str),

    #[error("this column has exceeded its WIP ({0})")]
    WipLimitExceeded(usize),

    #[error("this column has exceeded its TODO ({0})")]
    TodoLimitExceeded(usize),
    
    #[error("this item is already in this column")]
    AlreadyInColumn,

    #[error("this board has reach its maximum member capacity")]
    BoardMemberLimitExceeded,
    
    #[error("this board no member of id {0}")]
    MemberNotFound(Uuid),
}

pub type DomainResult<T> = Result<T, DomainError>;