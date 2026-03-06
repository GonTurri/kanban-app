use async_trait::async_trait;
use chrono::{DateTime, Utc};
use serde::Serialize;
use uuid::Uuid;

use crate::{
    adapters::persistence::PostgresPersistence,
    entities::user::User,
    prelude::*,
    use_cases::user::UserPersistence,
};

// User struct as stored in the db.
#[derive(sqlx::FromRow, Debug, Serialize)]
pub struct UserDb {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    pub password_hash: String,
    pub created_at: DateTime<Utc>,
}

impl From<UserDb> for User {
    fn from(user_db: UserDb) -> Self {
        User {
            id: user_db.id,
            username: user_db.username,
            email: user_db.email,
            password_hash: user_db.password_hash,
            created_at: user_db.created_at,
        }
    }
}

#[async_trait]
impl UserPersistence for PostgresPersistence {
    async fn create_user(&self, user: &User) -> Result<()> {
        sqlx::query!(
            "INSERT INTO users (id, username, email, password_hash) VALUES ($1, $2, $3, $4)",
            user.id,
            user.username,
            user.email,
            user.password_hash
        )
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    async fn get_user(&self, id: Uuid) -> Result<Option<User>> {
        let result = sqlx::query_as!(UserDb , "SELECT * FROM users WHERE id = $1", id)
            .fetch_optional(&self.pool)
            .await?
            .map(Into::into);

        Ok(result)
    }

    async fn get_by_email(&self, email: &str) -> Result<Option<User>> {
        let result = sqlx::query_as!(UserDb , "SELECT * FROM users WHERE email = $1", email)
            .fetch_optional(&self.pool)
            .await?
            .map(Into::into);

        Ok(result)
    }

    async fn exists_by_id(&self, id: Uuid) -> Result<bool> {
        let result = sqlx::query_scalar!(
            "SELECT EXISTS(SELECT 1 FROM users WHERE id = $1)",
            id
        )
            .fetch_one(&self.pool)
            .await?;


        Ok(result.unwrap_or(false))
    }
}