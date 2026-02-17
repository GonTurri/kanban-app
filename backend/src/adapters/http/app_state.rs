use std::sync::Arc;

use axum::extract::FromRef;

use crate::{infrastructure::config::AppConfig};
use crate::application::use_cases::user::UserUseCases;

#[derive(Clone)]
pub struct AppState {
    pub config: Arc<AppConfig>,
    pub user_use_cases: Arc<UserUseCases>
}


// impl FromRef for each use case
impl FromRef<AppState> for Arc<UserUseCases> {
    fn from_ref(app_state: &AppState) -> Self {
        app_state.user_use_cases.clone()
    }
}