use crate::prelude::*;
use crate::use_cases::auth::AuthUseCases;
use crate::use_cases::board::BoardUseCases;
use crate::use_cases::column::ColumnUseCases;
use crate::use_cases::get_item_metrics_query::GetItemMetricsQuery;
use crate::use_cases::item::ItemUseCases;
use crate::{
    adapters::http::app_state::AppState,
    infrastructure::{argon2_password_hasher, config::AppConfig, postgres_persistence},
    use_cases::user::UserUseCases,
};
use std::fs::File;
use std::sync::Arc;
use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

pub async fn init_app_state() -> Result<AppState> {
    let config = AppConfig::from_env();
    let postgres = Arc::new(postgres_persistence().await?);
    let argon_hasher = Arc::new(argon2_password_hasher());

    let user_use_cases = UserUseCases::new(argon_hasher.clone(), postgres.clone());
    let auth_use_cases = AuthUseCases::new(
        postgres.clone(),
        postgres.clone(),
        argon_hasher.clone(),
        config.jwt_secret.clone(),
        config.access_token_ttl,
        config.refresh_token_ttl,
    );

    let board_use_cases = BoardUseCases::new(postgres.clone(), postgres.clone(), postgres.clone(), postgres.clone());
    let column_use_cases = ColumnUseCases::new(postgres.clone(), postgres.clone());
    let item_use_cases = ItemUseCases::new(postgres.clone(), postgres.clone(), postgres.clone());
    let get_items_metrics_query = GetItemMetricsQuery::new(postgres.clone(), postgres.clone(), postgres.clone());
    


    Ok(AppState {
        config: Arc::new(config),
        user_use_cases: Arc::new(user_use_cases),
        auth_use_cases: Arc::new(auth_use_cases),
        board_use_cases: Arc::new(board_use_cases),
        item_use_cases: Arc::new(item_use_cases),
        column_use_cases: Arc::new(column_use_cases),
        get_item_metrics_query: Arc::new(get_items_metrics_query)
    })
}

pub fn init_tracing() {
    let filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| "axum_trainer=debug,tower_http=debug".into());

    // Console (pretty logs)
    let console_layer = fmt::layer()
        .with_target(false) // don’t show target (module path)
        .with_level(true) // show log level
        .pretty(); // human-friendly, with colors

    // File (structured JSON logs)
    let file = File::create("app.log").expect("cannot create log file");
    let json_layer = fmt::layer()
        .json()
        .with_writer(file)
        .with_current_span(true)
        .with_span_list(true);

    tracing_subscriber::registry()
        .with(filter)
        .with(console_layer)
        .with(json_layer)
        .try_init()
        .ok();
}
