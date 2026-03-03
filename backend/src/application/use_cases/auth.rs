use std::sync::Arc;
use serde::{Deserialize, Serialize};
use std::time::Duration;
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use jsonwebtoken::{encode, EncodingKey, Header};
use uuid::Uuid;
use crate::prelude::*;
use crate::use_cases::user::{UserCredentialsVerifier, UserPersistence};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: Uuid,
    pub exp: usize
}

impl Claims {
    pub fn new(user_id: Uuid, valid_duration: Duration ) -> Claims {
        let exp = std::time::SystemTime::now()
            .checked_add(valid_duration)
            .unwrap()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs() as usize;

        Self {sub: user_id, exp}
    }
}

#[derive(Debug)]
pub struct RefreshToken {
    pub token: Uuid,
    pub user_id: Uuid,
    pub expires_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>
}

impl RefreshToken {
    pub fn new(user_id: Uuid, valid_for: Duration) -> Self {
        let now = Utc::now();
        Self {
            token: Uuid::new_v4(),
            user_id,
            expires_at: now + valid_for,
            created_at: now
        }
    }

    pub fn is_expired(&self) -> bool {
        Utc::now() > self.expires_at
    }
}

#[async_trait]
pub trait AuthPersistence: Send + Sync {
    async fn store_refresh_token(&self, token: &RefreshToken) -> Result<()>;
    async fn get_refresh_token(&self, token: Uuid) -> Result<Option<RefreshToken>>;
    async fn delete_refresh_token(&self, token: Uuid) -> Result<()>;
}

#[derive(Clone)]
pub struct AuthUseCases {
    auth_persistence: Arc<dyn AuthPersistence>,
    user_persistence: Arc<dyn UserPersistence>,
    user_credentials_verifier: Arc<dyn UserCredentialsVerifier>,
    jwt_secret: String,
    access_token_ttl: Duration,
    refresh_token_ttl: Duration,
}

impl AuthUseCases {
pub fn new(
    auth_persistence: Arc<dyn AuthPersistence>,
    user_persistence: Arc<dyn UserPersistence>,
    user_credentials_verifier: Arc<dyn UserCredentialsVerifier>,
    jwt_secret: String,
    access_token_ttl: std::time::Duration,
    refresh_token_ttl: Duration,
) -> Self {
    Self {
        auth_persistence,
        user_persistence,
        user_credentials_verifier,
        jwt_secret,
        access_token_ttl,
        refresh_token_ttl
    }
}

    fn generate_jwt(&self, user_id: Uuid) -> Result<String>{
        let claims = Claims::new(user_id, self.access_token_ttl);
        encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(self.jwt_secret.as_bytes())
        )
            .map_err(|_| AppError::Internal(format!("Error creating jwt for user: {user_id}")))
    }

    pub async fn login(&self, email: &str, password: &str) -> Result<(String, Uuid)> {
        let user = self.user_persistence.get_by_email(email).await
            ?.ok_or(AppError::InvalidCredentials)?;

        if !self.user_credentials_verifier.verify_user_password(password, &user.password_hash) {
            return Err(AppError::InvalidCredentials);
        }

        let access_token = self.generate_jwt(user.id)?;
        let refresh_token = RefreshToken::new(user.id, self.refresh_token_ttl);


        self.auth_persistence.store_refresh_token(&refresh_token).await?;

        Ok((access_token, refresh_token.token))

    }

    pub async fn refresh(&self, refresh_token_uuid: Uuid) -> Result<String> {
        let refresh_token = self.auth_persistence.get_refresh_token(refresh_token_uuid).await?
            .ok_or(AppError::InvalidCredentials)?;

        if refresh_token.is_expired() {
            self.auth_persistence.delete_refresh_token(refresh_token_uuid).await?;
            return Err(AppError::InvalidCredentials);
        }

        self.generate_jwt(refresh_token.user_id)
    }

    pub async fn logout(&self, refresh_token_uuid: Uuid) -> Result<()> {
        self.auth_persistence.delete_refresh_token(refresh_token_uuid).await
    }

}


