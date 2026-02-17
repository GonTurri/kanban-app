pub mod db;
pub mod config;
pub mod setup;
pub mod app;

use crate::prelude::*;

use crate::{
    adapters::{crypto::argon2::ArgonPasswordHasher, persistence::PostgresPersistence},
    infrastructure::db::init_db,
};

pub async fn postgres_persistence() -> Result<PostgresPersistence> {
    let pool = init_db().await?;
    let persistence = PostgresPersistence::new(pool);
    Ok(persistence)
}

pub fn argon2_password_hasher() -> ArgonPasswordHasher {
    ArgonPasswordHasher::default()
}