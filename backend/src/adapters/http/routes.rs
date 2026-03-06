mod auth;
mod board;
mod column;
mod item;
mod user;

use crate::adapters::http::app_state::AppState;
use axum::Router;
use axum::http::{StatusCode, Uri};
use axum::response::IntoResponse;
use axum::routing::get;

pub fn router() -> Router<AppState> {
    Router::new()
        .nest("/users", user::router())
        .nest("/auth", auth::router())
        .nest("/boards", board::router())
        .nest("/columns", column::router())
        .nest("/items", item::router())
        .route("/hello", get(handler))
        .fallback(fallback)
}

async fn handler() -> impl IntoResponse {
    "Hello world!"
}

async fn fallback(uri: Uri) -> (StatusCode, String) {
    (StatusCode::NOT_FOUND, format!("No route for {uri}"))
}
