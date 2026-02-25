use thiserror::Error;
use uuid::Uuid;
use crate::domain;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),

    #[error("Database migration error: {0}")]
    DatabaseMigrations(#[from] sqlx::migrate::MigrateError),

    #[error("Invalid credentials")]
    InvalidCredentials,

    #[error("Internal error: {0}")]
    Internal(String),
    
    #[error("{0}")]
    Domain(#[from] domain::domain_error::DomainError),
    
    #[error("Resource not found {0} of id: {1}")]
    ResourceNotFound(&'static str, Uuid),
}