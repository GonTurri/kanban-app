use std::sync::Arc;
use axum::extract::{Path, State};
use axum::{Json, Router};
use axum::http::StatusCode;
use axum::routing::{delete, get, post, put};
use serde::{Deserialize, Serialize};
use tracing::{info, instrument};
use uuid::Uuid;
use crate::adapters::http::app_state::AppState;
use crate::adapters::http::extractors::AuthUser;
use crate::entities::board_role::BoardRole;
use crate::use_cases::board::{BoardResponseDto, BoardUseCases};
use crate::prelude::*;

#[derive(Deserialize, Debug)]
pub struct CreateBoardPayload {
    pub title: String,
    pub description: String,
}

#[derive(Serialize)]
pub struct CreateBoardResponse {
    pub id: Uuid,
}

#[derive(Deserialize, Debug)]
pub struct AddMemberPayload {
    pub user_id: Uuid,
    pub role: BoardRole,
}

#[derive(Deserialize, Debug)]
pub struct ChangeRolePayload {
    pub role: BoardRole,
}

#[instrument(skip(board_use_cases))]
pub async fn get_board_handler(State(board_use_cases): State<Arc<BoardUseCases>>, Path(id): Path<Uuid>,
user: AuthUser) -> Result<Json<BoardResponseDto>> {
    info!("User {} fetching board {}", user.id, id);
    let board = board_use_cases.get_full_board(id, user.id).await?;

    Ok(Json(board))
}

#[instrument(skip(board_use_cases, payload))]
pub async fn create_board_handler(
    State(board_use_cases): State<Arc<BoardUseCases>>,
    user: AuthUser,
    Json(payload): Json<CreateBoardPayload>,
) -> Result<(StatusCode, Json<CreateBoardResponse>)> {

    info!("User {} creating a new board", user.id);

    let board_id = board_use_cases
        .create_board(payload.title, payload.description, user.id)
        .await?;

    info!("Board {} created successfully", board_id);

    Ok((StatusCode::CREATED, Json(CreateBoardResponse { id: board_id })))
}

#[instrument(skip(board_use_cases, payload))]
pub async fn add_member_handler(
    State(board_use_cases): State<Arc<BoardUseCases>>,
    Path(board_id): Path<Uuid>,
    user: AuthUser,
    Json(payload): Json<AddMemberPayload>,
) -> Result<StatusCode> {

    info!("User {} adding member {} to board {}", user.id, payload.user_id, board_id);

    board_use_cases
        .add_member(board_id, user.id, payload.user_id, payload.role)
        .await?;

    Ok(StatusCode::OK)
}

#[instrument(skip(board_use_cases, payload))]
pub async fn change_member_role_handler(
    State(board_use_cases): State<Arc<BoardUseCases>>,
    Path((board_id, target_user_id)): Path<(Uuid, Uuid)>,
    user: AuthUser,
    Json(payload): Json<ChangeRolePayload>,
) -> Result<StatusCode> {

    info!("User {} changing role of {} in board {}", user.id, target_user_id, board_id);

    board_use_cases
        .change_member_role(board_id, user.id, target_user_id, payload.role)
        .await?;

    Ok(StatusCode::OK)
}

#[instrument(skip(board_use_cases))]
pub async fn remove_member_handler(
    State(board_use_cases): State<Arc<BoardUseCases>>,
    Path((board_id, target_user_id)): Path<(Uuid, Uuid)>,
    user: AuthUser,
) -> Result<StatusCode> {

    info!("User {} removing member {} from board {}", user.id, target_user_id, board_id);

    board_use_cases
        .remove_member_from_board(board_id, user.id, target_user_id)
        .await?;

    Ok(StatusCode::NO_CONTENT)
}


pub fn router() -> Router<AppState> {
    Router::new()
        .route("/", post(create_board_handler))
        .route("/{id}", get(get_board_handler))
        .route("/{id}/members", post(add_member_handler))
        .route("/{id}/members/{user_id}", put(change_member_role_handler))
        .route("/{id}/members/{user_id}", delete(remove_member_handler))
}