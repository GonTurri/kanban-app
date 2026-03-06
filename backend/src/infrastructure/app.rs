use axum::http::header::{AUTHORIZATION, CONTENT_TYPE};
use axum::{http, Router};
use tower_http::{cors::{CorsLayer}, trace::TraceLayer};
use uuid::Uuid;

use crate::{
    adapters::{self, http::app_state::AppState},
    infrastructure::setup::init_tracing,
};

pub fn create_app(app_state: AppState) -> Router {
    init_tracing();

    let frontend_origin = app_state.config.frontend_url
        .parse::<http::HeaderValue>()
        .expect("Invalid FRONTEND_URL format");

    let cors = CorsLayer::new()
        .allow_origin(frontend_origin)
        .allow_methods([
            http::Method::GET,
            http::Method::POST,
            http::Method::PUT,
            http::Method::DELETE,
            http::Method::OPTIONS,
            http::Method::HEAD
        ])
        .allow_headers([CONTENT_TYPE, AUTHORIZATION])
        .allow_credentials(true);

    Router::new()
        .nest("/api", adapters::http::routes::router())
        .with_state(app_state)
        .layer(cors)
        .layer(
            TraceLayer::new_for_http().make_span_with(|request: &http::Request<_>| {
                let request_id = Uuid::new_v4();
                tracing::info_span!(
                    "http-request",
                    method = %request.method(),
                    uri = %request.uri(),
                    version = ?request.version(),
                    request_id = %request_id
                )
            }),
        )
}