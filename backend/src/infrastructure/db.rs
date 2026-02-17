use std::env;

use sqlx::{PgPool, postgres::PgPoolOptions};
use tracing::info;
use crate::prelude::*;

const CONNECTION_POOL_SIZE: u32 = 5;

pub async fn init_db() -> Result<PgPool> {
    let database_url = env::var("DATABASE_URL").map_err(|e| AppError::Internal(e.to_string()))?;

    let pool = PgPoolOptions::new()
        .max_connections(CONNECTION_POOL_SIZE)
        .connect(&database_url)
        .await?;

    sqlx::migrate!("./migrations")
        .run(&pool)
        .await?;

    info!("Connected to database!");
    Ok(pool)
}