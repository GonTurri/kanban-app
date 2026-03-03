use sqlx::PgPool;

pub mod user;
pub mod item;
pub mod column;
pub mod board;
mod auth;

#[derive(Clone)]
pub struct PostgresPersistence {
    pool: PgPool,
}

impl PostgresPersistence {
    pub fn new(pool: PgPool) -> Self {
        PostgresPersistence { pool }
    }
}