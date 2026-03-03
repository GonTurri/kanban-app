use argon2::{Argon2, password_hash::{PasswordHasher, SaltString, rand_core::OsRng}, PasswordHash, PasswordVerifier};

use crate::prelude::*;
use crate::use_cases::user::{UserCredentialsHasher, UserCredentialsVerifier};
#[derive(Default)]
pub struct ArgonPasswordHasher {
    hasher: Argon2<'static>,
}

impl UserCredentialsHasher for ArgonPasswordHasher {
    fn hash_password(&self, password: &str) -> Result<String> {
        let salt = SaltString::generate(&mut OsRng);
        let hash = self
            .hasher
            .hash_password(password.as_bytes(), &salt)
            .map_err(|_| AppError::Internal("Password hashing failed.".into()))?
            .to_string();

        Ok(hash)
    }
}

impl UserCredentialsVerifier for ArgonPasswordHasher {
    fn verify_user_password(&self, password: &str, stored_hash: &str) -> bool {
        let parsed_hash = match PasswordHash::new(stored_hash) {
            Ok(hash) => hash,
            Err(_) => return false,
        };

        self.hasher
            .verify_password(password.as_bytes(), &parsed_hash)
            .is_ok()
    }
}