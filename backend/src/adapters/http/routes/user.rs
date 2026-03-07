use std::sync::Arc;

use axum::{Json, Router, extract::State, http::StatusCode, response::IntoResponse, routing::post};
use axum::routing::get;
use secrecy::SecretString;
use serde::{Deserialize, Serialize};
use tracing::{info, instrument};
use uuid::Uuid;
use crate::adapters::http::app_state::AppState;
use crate::adapters::http::extractors::AuthUser;
use crate::entities::user::User;
use crate::prelude::*;
use crate::use_cases::user::UserUseCases;

#[derive(Debug, Clone, Deserialize)]
pub struct RegisterPayload {
    username: String,
    email: String,
    password: SecretString,
}

#[derive(Debug, Clone, Serialize)]
pub struct RegisterResponse {
    success: bool,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProfileResponse {
    id: Uuid,
    username: String,
    email: String
}

impl From<User> for ProfileResponse {
    fn from(value: User) -> Self {
        Self {
            id: value.id,
            email: value.email,
            username: value.username
        }
    }
}

#[instrument(skip(user_use_cases))]
pub async fn register(
    State(user_use_cases): State<Arc<UserUseCases>>,
    Json(payload): Json<RegisterPayload>,
) -> Result<impl IntoResponse> {
    info!("Register user called");

    user_use_cases.register(payload.username, payload.email, &payload.password).await?;
    Ok(
        (
            StatusCode::CREATED,
            Json(RegisterResponse { success: true }),
        )
    )
}

pub async fn get_profile_handler(
    State(user_use_cases): State<Arc<UserUseCases>>,
    user: AuthUser
) -> Result<Json<ProfileResponse>> {
    info!("Fetching profile for user {}", user.id);

    let profile: ProfileResponse = user_use_cases.get_user_by_id(user.id).await?.into();

    Ok(Json(profile))
}

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/register", post(register))
        .route("/me", get(get_profile_handler))
}
