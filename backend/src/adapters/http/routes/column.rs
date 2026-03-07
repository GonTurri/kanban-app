use axum::{
    extract::{Path, State},
    http::StatusCode,
    routing::{delete, post, put},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tracing::{info, instrument};
use uuid::Uuid;

use crate::adapters::http::app_state::AppState;
use crate::adapters::http::extractors::AuthUser;
use crate::entities::column_type::ColumnType;
use crate::prelude::*;
use crate::use_cases::column::ColumnUseCases;

#[derive(Deserialize, Debug)]
pub struct CreateColumnPayload {
    pub name: String,
    pub kind: ColumnType,
    pub target_index: usize,
}

#[derive(Serialize)]
pub struct CreateResponse {
    pub id: Uuid,
}

#[derive(Deserialize, Debug)]
pub struct UpdateColumnPayload {
    pub name: String,
    pub kind: ColumnType,
}

#[derive(Deserialize, Debug)]
pub struct MoveColumnPayload {
    pub target_index: usize,
}

#[instrument(skip(column_use_cases, payload))]
pub async fn add_column_handler(
    State(column_use_cases): State<Arc<ColumnUseCases>>,
    Path(board_id): Path<Uuid>,
    user: AuthUser,
    Json(payload): Json<CreateColumnPayload>,
) -> Result<(StatusCode, Json<CreateResponse>)> {
    info!("User {} adding column to board {}", user.id, board_id);

    let column_id = column_use_cases
        .add_board_column(board_id, user.id, payload.name, payload.kind, payload.target_index)
        .await?;

    Ok((StatusCode::CREATED, Json(CreateResponse { id: column_id })))
}

#[instrument(skip(column_use_cases, payload))]
pub async fn update_column_handler(
    State(column_use_cases): State<Arc<ColumnUseCases>>,
    Path(column_id): Path<Uuid>,
    user: AuthUser,
    Json(payload): Json<UpdateColumnPayload>,
) -> Result<StatusCode> {
    info!("User {} updating column {}", user.id, column_id);

    column_use_cases.update_column(column_id, user.id, payload.name, payload.kind).await?;

    Ok(StatusCode::OK)
}

#[instrument(skip(column_use_cases, payload))]
pub async fn move_column_handler(
    State(column_use_cases): State<Arc<ColumnUseCases>>,
    Path((column_id, board_id)): Path<(Uuid, Uuid)>,
    user: AuthUser,
    Json(payload): Json<MoveColumnPayload>,
) -> Result<StatusCode> {
    info!("User {} moving column {} in board {}", user.id, column_id, board_id);

    column_use_cases.move_column(board_id, column_id, user.id, payload.target_index).await?;

    Ok(StatusCode::OK)
}

#[instrument(skip(column_use_cases))]
pub async fn delete_column_handler(
    State(column_use_cases): State<Arc<ColumnUseCases>>,
    Path(column_id): Path<Uuid>,
    user: AuthUser,
) -> Result<StatusCode> {
    info!("User {} deleting column {}", user.id, column_id);

    column_use_cases.delete_column(column_id, user.id).await?;

    Ok(StatusCode::NO_CONTENT)
}

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/board/{board_id}", post(add_column_handler))
        .route("/{column_id}/board/{board_id}/move", put(move_column_handler))
        .route("/{column_id}", put(update_column_handler))
        .route("/{column_id}", delete(delete_column_handler))
}