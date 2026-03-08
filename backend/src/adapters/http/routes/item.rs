use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    routing::{delete, get, post, put},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tracing::{info, instrument};
use uuid::Uuid;
use validator::Validate;
use crate::adapters::http::app_state::AppState;
use crate::adapters::http::extractors::AuthUser;
use crate::entities::item_priority::ItemPriority;
use crate::prelude::*;
use crate::use_cases::item::{ItemHistoryDto, ItemUseCases};
use crate::use_cases::get_item_metrics_query::GetItemMetricsQuery;
use crate::services::metrics_calculator::ItemMetrics;
use crate::use_cases::board::ItemResponseDto;
#[derive(Deserialize, Debug)]
pub struct PaginationQuery {
    pub limit: Option<usize>,
    pub offset: Option<usize>,
}

#[derive(Deserialize, Debug, Validate)]
pub struct AddItemPayload {
    #[validate(length(min = 1, max = 255, message = "Title must be between 1 and 255 characters"))]
    pub title: String,
    #[validate(length(max = 2000, message = "Description is too long"))]
    pub description: Option<String>,
    pub assigned_to: Option<Uuid>,
    pub priority: ItemPriority,
}

#[derive(Deserialize, Debug, Validate)]
pub struct UpdateItemPayload {
    #[validate(length(min = 1, max = 255, message = "Title must be between 1 and 255 characters"))]
    pub title: String,
    #[validate(length(max = 2000, message = "Description is too long"))]
    pub description: Option<String>,
    pub assigned_to: Option<Uuid>,
    pub priority: ItemPriority,
}

#[derive(Serialize)]
pub struct CreateResponse {
    pub id: Uuid,
}

#[derive(Deserialize, Debug)]
pub struct MoveItemPayload {
    pub new_column_id: Uuid,
}
#[instrument(skip(item_use_cases))]
pub async fn get_items_handler(
    State(item_use_cases): State<Arc<ItemUseCases>>,
    Path(column_id): Path<Uuid>,
    Query(pagination): Query<PaginationQuery>,
    user: AuthUser,
) -> Result<Json<Vec<ItemResponseDto>>> {

    let limit = pagination.limit.unwrap_or(10);
    let offset = pagination.offset.unwrap_or(0);

    info!("User {} fetching items for column {} (limit: {}, offset: {})", user.id, column_id, limit, offset);

    let items = item_use_cases.get_items(column_id, limit, offset, user.id).await?
        .into_iter()
        .map(Into::into)
        .collect();

    Ok(Json(items))
}

#[instrument(skip(item_use_cases, payload))]
pub async fn add_item_handler(
    State(item_use_cases): State<Arc<ItemUseCases>>,
    Path(column_id): Path<Uuid>,
    user: AuthUser,
    Json(payload): Json<AddItemPayload>,
) -> Result<(StatusCode, Json<CreateResponse>)> {
    info!("User {} adding item to column {}", user.id, column_id);

    payload.validate()?;

    let item_id = item_use_cases
        .add_item(column_id, payload.title, payload.description, payload.assigned_to, payload.priority, user.id)
        .await?;

    Ok((StatusCode::CREATED, Json(CreateResponse { id: item_id })))
}

#[instrument(skip(item_use_cases, payload))]
pub async fn update_item_handler(
    State(item_use_cases): State<Arc<ItemUseCases>>,
    Path(item_id): Path<Uuid>,
    user: AuthUser,
    Json(payload): Json<UpdateItemPayload>,
) -> Result<StatusCode> {
    info!("User {} updating item {}", user.id, item_id);

    payload.validate()?;

    item_use_cases
        .update_item_details(item_id, payload.title, payload.description, payload.assigned_to, payload.priority, user.id)
        .await?;

    Ok(StatusCode::OK)
}

#[instrument(skip(item_use_cases, payload))]
pub async fn move_item_handler(
    State(item_use_cases): State<Arc<ItemUseCases>>,
    Path(item_id): Path<Uuid>,
    user: AuthUser,
    Json(payload): Json<MoveItemPayload>,
) -> Result<StatusCode> {
    info!("User {} moving item {} to column {}", user.id, item_id, payload.new_column_id);

    item_use_cases.move_item(item_id, payload.new_column_id, user.id).await?;

    Ok(StatusCode::OK)
}

#[instrument(skip(item_use_cases))]
pub async fn delete_item_handler(
    State(item_use_cases): State<Arc<ItemUseCases>>,
    Path(item_id): Path<Uuid>,
    user: AuthUser,
) -> Result<StatusCode> {
    info!("User {} deleting item {}", user.id, item_id);

    item_use_cases.delete_item(item_id, user.id).await?;

    Ok(StatusCode::NO_CONTENT)
}

#[instrument(skip(item_use_cases))]
pub async fn get_item_history_handler(
    State(item_use_cases): State<Arc<ItemUseCases>>,
    Path(item_id): Path<Uuid>,
    user: AuthUser,
) -> Result<Json<Vec<ItemHistoryDto>>> {
    info!("User {} fetching history for item {}", user.id, item_id);

    let history = item_use_cases.get_item_history(item_id, user.id).await?;

    Ok(Json(history))
}

#[instrument(skip(metrics_query))]
pub async fn get_item_metrics_handler(
    State(metrics_query): State<Arc<GetItemMetricsQuery>>,
    Path(item_id): Path<Uuid>,
    user: AuthUser,
) -> Result<Json<Option<ItemMetrics>>> {
    info!("User {} fetching metrics for item {}", user.id, item_id);

    let metrics = metrics_query.execute(item_id, user.id).await?;

    Ok(Json(metrics))
}


pub fn router() -> Router<AppState> {
    Router::new()
        .route("/column/{column_id}", get(get_items_handler))
        .route("/column/{column_id}", post(add_item_handler))
        .route("/{item_id}", put(update_item_handler))
        .route("/{item_id}", delete(delete_item_handler))
        .route("/{item_id}/move", put(move_item_handler))
        .route("/{item_id}/history", get(get_item_history_handler))
        .route("/{item_id}/metrics", get(get_item_metrics_handler))
}