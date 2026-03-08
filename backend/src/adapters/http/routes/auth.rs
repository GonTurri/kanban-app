use crate::adapters::http::app_state::AppState;
use crate::prelude::*;
use crate::use_cases::auth::AuthUseCases;
use axum::extract::State;
use axum::routing::post;
use axum::{Json, Router};
use axum_extra::extract::CookieJar;
use axum_extra::extract::cookie::{Cookie, SameSite};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tracing::{info, instrument};
use uuid::Uuid;
use validator::{Validate};
const REFRESH_TOKEN_COOKIE_NAME: &str = "refresh_token";

#[derive(Deserialize, Debug, Validate)]
pub struct LoginPayload {
    #[validate(email(message = "Invalid email format"))]
    pub email: String,
    #[validate(length(max = 255, message = "Password too long"))]
    pub password: String,
}

#[derive(Serialize)]
pub struct AuthResponse {
    pub access_token: String,
}

#[instrument(
    skip(auth_use_cases, jar, payload),
    fields(email = %payload.email)
)]
pub async fn login_handler(
    State(auth_use_cases): State<Arc<AuthUseCases>>,
    jar: CookieJar,
    Json(payload): Json<LoginPayload>,
) -> Result<(CookieJar, Json<AuthResponse>)> {
    info!("Starting login process");

    payload.validate()?;

    let (access_token, refresh_token) = auth_use_cases
        .login(&payload.email, &payload.password)
        .await?;

    let cookie = Cookie::build((REFRESH_TOKEN_COOKIE_NAME, refresh_token.to_string()))
        .http_only(true)
        .secure(true)
        .same_site(SameSite::Strict)
        .path("/api/auth")
        .build();

    info!("Login successful");

    Ok((jar.add(cookie), Json(AuthResponse { access_token })))
}

#[instrument(skip(auth_use_cases, jar))]
pub async fn refresh_handler(
    State(auth_use_cases): State<Arc<AuthUseCases>>,
    jar: CookieJar,
) -> Result<Json<AuthResponse>> {
    info!("Starting token refresh process");
    let cookie = jar
        .get(REFRESH_TOKEN_COOKIE_NAME)
        .ok_or(AppError::InvalidCredentials)?;
    let token_uuid = Uuid::parse_str(cookie.value()).map_err(|_| AppError::InvalidCredentials)?;

    let access_token = auth_use_cases.refresh(token_uuid).await?;

    info!("Token refresh successful");

    Ok(Json(AuthResponse { access_token }))
}

#[instrument(skip(auth_use_cases, jar))]
pub async fn logout_handler(
    State(auth_use_cases): State<Arc<AuthUseCases>>,
    jar: CookieJar,
) -> Result<CookieJar> {
    info!("Starting logout process");
    if let Some(cookie) = jar.get(REFRESH_TOKEN_COOKIE_NAME) {
        if let Ok(token_uuid) = Uuid::parse_str(cookie.value()) {
            let _ = auth_use_cases.logout(token_uuid).await;
            info!("Refresh token revoked in database");
        } else {
            info!("Invalid token format found in cookie during logout");
        }
    } else {
        info!("No refresh token found during logout");
    }

    let remove_cookie = Cookie::build((REFRESH_TOKEN_COOKIE_NAME, ""))
        .path("/api/auth")
        .http_only(true)
        .max_age(time::Duration::ZERO)
        .build();

    info!("Logout successful, cookie cleared");

    Ok(jar.add(remove_cookie))
}

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/login", post(login_handler))
        .route("/refresh", post(refresh_handler))
        .route("/logout", post(logout_handler))
}
