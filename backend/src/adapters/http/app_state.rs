use std::sync::Arc;

use axum::extract::FromRef;

use crate::{infrastructure::config::AppConfig};
use crate::use_cases::user::UserUseCases;
use crate::use_cases::auth::AuthUseCases;
use crate::use_cases::board::BoardUseCases;
use crate::use_cases::column::ColumnUseCases;
use crate::use_cases::item::ItemUseCases;
use crate::use_cases::get_item_metrics_query::GetItemMetricsQuery;

#[derive(Clone)]
pub struct AppState {
    pub config: Arc<AppConfig>,
    pub user_use_cases: Arc<UserUseCases>,
    pub auth_use_cases: Arc<AuthUseCases>,
    pub board_use_cases: Arc<BoardUseCases>,
    pub column_use_cases: Arc<ColumnUseCases>,
    pub item_use_cases: Arc<ItemUseCases>,
    pub get_item_metrics_query: Arc<GetItemMetricsQuery>
}


// impl FromRef for each use case
impl FromRef<AppState> for Arc<UserUseCases> {
    fn from_ref(app_state: &AppState) -> Self {
        app_state.user_use_cases.clone()
    }
}

impl FromRef<AppState> for Arc<AuthUseCases> {
    fn from_ref(app_state: &AppState) -> Self {
        app_state.auth_use_cases.clone()
    }
}


impl FromRef<AppState> for Arc<BoardUseCases> {
    fn from_ref(app_state: &AppState) -> Self {
        app_state.board_use_cases.clone()
    }
}

impl FromRef<AppState> for Arc<ColumnUseCases> {
    fn from_ref(app_state: &AppState) -> Self {
        app_state.column_use_cases.clone()
    }
}

impl FromRef<AppState> for Arc<ItemUseCases> {
    fn from_ref(app_state: &AppState) -> Self {
        app_state.item_use_cases.clone()
    }
}

impl FromRef<AppState> for Arc<GetItemMetricsQuery> {
    fn from_ref(app_state: &AppState) -> Self {
        app_state.get_item_metrics_query.clone()
    }
}

impl FromRef<AppState> for Arc<AppConfig> {
    fn from_ref(app_state: &AppState) -> Self {
        app_state.config.clone()
    }
}

