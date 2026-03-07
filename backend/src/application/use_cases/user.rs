use std::sync::Arc;

use crate::prelude::*;
use async_trait::async_trait;
use secrecy::{ExposeSecret, SecretString};
use tracing::{info, instrument};
use uuid::Uuid;
use crate::entities::user::User;

#[async_trait]
pub trait UserPersistence: Send + Sync {
    async fn create_user(&self, user: &User) -> Result<()>;
    async fn get_user(&self, id: Uuid) -> Result<Option<User>>;

    async fn get_by_email(&self, email: &str) -> Result<Option<User>>;

    async fn exists_by_id(&self, id: Uuid) -> Result<bool>;
}

pub trait UserCredentialsHasher: Send + Sync {
    fn hash_password(&self, password: &str) -> Result<String>;
}

pub trait UserCredentialsVerifier: Send + Sync {
    fn verify_user_password(&self, password: &str, stored_hash: &str) -> bool;
}

#[derive(Clone)]
pub struct UserUseCases {
    hasher: Arc<dyn UserCredentialsHasher>,
    persistence: Arc<dyn UserPersistence>,
}
impl UserUseCases {
    pub fn new(
        hasher: Arc<dyn UserCredentialsHasher>,
        persistence: Arc<dyn UserPersistence>,
    ) -> Self {
        Self {
            hasher,
            persistence,
        }
    }

    #[instrument(skip(self))]
    pub async fn register(&self, username: String, email: String, password: &SecretString) -> Result<()> {
        info!("Registering user...");

        let hash = self.hasher.hash_password(password.expose_secret())?;
        let user = User::new(username, email, hash);
        self.persistence.create_user(&user).await?;

        info!("Registering user finished.");

        Ok(())
    }

    pub async fn get_user_by_id(&self, id: Uuid) -> Result<User>{
       self.persistence.get_user(id).await?
            .ok_or(AppError::ResourceNotFound("User", id))
    }
}

#[cfg(test)]
mod test {
    use async_trait::async_trait;

    use super::*;

    struct MockUserPersistence;

    #[async_trait]
    impl UserPersistence for MockUserPersistence {
        async fn create_user(
            &self,
            user: &User,
        ) -> Result<()> {
            assert_eq!(user.username, "testuser");
            assert_eq!(user.email, "testuser@gmail.com");
            Ok(())
        }

        async fn get_user(&self, id: Uuid) -> Result<Option<User>> {
            todo!()
        }

        async fn get_by_email(&self, email: &str) -> Result<Option<User>> {
            todo!()
        }

        async fn exists_by_id(&self, id: Uuid) -> Result<bool> {
            todo!()
        }
    }

    struct MockUserCredentialsHasher;

    impl UserCredentialsHasher for MockUserCredentialsHasher {
        fn hash_password(&self, password: &str) -> Result<String> {
            Ok(format!("{}_hash", password))
        }
    }

    #[tokio::test]
    async fn add_user_works() {
        let user_use_cases = UserUseCases::new(
            Arc::new(MockUserCredentialsHasher),
            Arc::new(MockUserPersistence),
        );

        let result = user_use_cases
            .register("testuser".to_string(), "testuser@gmail.com".to_string(), &"testuser_pw".into())
            .await;

        assert!(result.is_ok());
    }
}
