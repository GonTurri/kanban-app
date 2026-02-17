use dotenvy::dotenv;
use tracing::info;

use backend::prelude::*;

use backend::infrastructure::{app::create_app, setup::init_app_state};

#[tokio::main]
async fn main() -> Result<()>{
    dotenv().ok();

    let app_state = init_app_state().await?;

    let app = create_app(app_state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080")
        .await
        .unwrap();

    info!("Backend listening at {}", &listener.local_addr().unwrap());

    axum::serve(listener, app).await.unwrap();

    Ok(())
}