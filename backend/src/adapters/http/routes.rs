mod user;

use axum::handler::HandlerWithoutStateExt;
use axum::http::{StatusCode, Uri};
use axum::response::IntoResponse;
use crate::adapters::http::app_state::AppState;
use axum::Router;
use axum::routing::get;

pub fn router() -> Router<AppState> {
    Router::new().
        nest("/user", user::router()).
        route("/hello", get(handler))
        .fallback(fallback)
}

async fn handler() -> impl IntoResponse {
    "Hello world!"
}

async fn fallback(uri: Uri) -> (StatusCode, String) {
    (StatusCode::NOT_FOUND, format!("No route for {uri}"))
}