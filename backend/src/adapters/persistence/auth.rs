use crate::adapters::persistence::PostgresPersistence;
use crate::prelude::*;
use crate::use_cases::auth::{AuthPersistence, RefreshToken};
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use serde::Serialize;
use uuid::Uuid;

#[derive(sqlx::FromRow, Debug, Serialize)]
pub struct RefreshTokenDb {
    pub token: Uuid,
    pub user_id: Uuid,
    pub expires_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
}

impl From<RefreshTokenDb> for RefreshToken {
    fn from(value: RefreshTokenDb) -> Self {
        Self {
            token: value.token,
            user_id: value.user_id,
            expires_at: value.expires_at,
            created_at: value.created_at,
        }
    }
}

#[async_trait]
impl AuthPersistence for PostgresPersistence {
    async fn store_refresh_token(&self, token: &RefreshToken) -> Result<()> {
        sqlx::query!(
            r#"INSERT INTO refresh_tokens (token, user_id, expires_at, created_at)
        VALUES ($1, $2, $3, $4)"#,
            token.token,
            token.user_id,
            token.expires_at,
            token.created_at
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    async fn get_refresh_token(&self, token: Uuid) -> Result<Option<RefreshToken>> {
        let result = sqlx::query_as!(
            RefreshTokenDb,
            r#"SELECT token, user_id, expires_at, created_at
         FROM refresh_tokens WHERE token = $1"#,
            token
        )
        .fetch_optional(&self.pool)
        .await?
        .map(Into::into);

        Ok(result)
    }

    async fn delete_refresh_token(&self, token: Uuid) -> Result<()> {
        sqlx::query!(r#"DELETE FROM refresh_tokens WHERE token = $1"#, token)
            .execute(&self.pool)
            .await?;

        Ok(())
    }
}
