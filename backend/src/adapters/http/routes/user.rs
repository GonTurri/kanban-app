use std::sync::Arc;

use axum::{Json, Router, extract::State, http::StatusCode, response::IntoResponse, routing::post};
use secrecy::SecretString;
use serde::{Deserialize, Serialize};
use tracing::{info, instrument};

use crate::adapters::http::app_state::AppState;
use crate::prelude::*;
use crate::use_cases::user::UserUseCases;

#[derive(Debug, Clone, Deserialize)]
struct RegisterPayload {
    username: String,
    email: String,
    password: SecretString,
}

#[derive(Debug, Clone, Serialize)]
struct RegisterResponse {
    success: bool,
}

#[instrument(skip(user_use_cases))]
pub async fn register(
    State(user_use_cases): State<Arc<UserUseCases>>,
    Json(payload): Json<RegisterPayload>,
) -> Result<impl IntoResponse> {
    info!("Register user called");

    user_use_cases.add(&payload.username, &payload.email, &payload.password).await?;
    Ok(
        (
            StatusCode::CREATED,
            Json(RegisterResponse { success: true }),
        )
    )
}

pub fn router() -> Router<AppState> {
    Router::new().route("/register", post(register))
}
