use crate::prelude::AppError;
use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        // Log the error before it gets converted into a status response.
        tracing::error!(error = ?self, "Request failed");

        match self {
            AppError::Database(_) => {
                (StatusCode::INTERNAL_SERVER_ERROR, "Database error").into_response()
            }
            AppError::InvalidCredentials => {
                (StatusCode::UNAUTHORIZED, "Invalid credentials").into_response()
            }
            AppError::Internal(_) => {
                (StatusCode::INTERNAL_SERVER_ERROR, "Internal error").into_response()
            }
            AppError::Domain(reason) => {
                (StatusCode::BAD_REQUEST, reason.to_string()).into_response()
            }
            
            AppError::ResourceNotFound(resource_name, id) => {
                (StatusCode::NOT_FOUND, format!("{} of id {} not found", resource_name, id)).into_response()
            }
            _ => (StatusCode::INTERNAL_SERVER_ERROR, "Internal error").into_response(),
        }
    }
}