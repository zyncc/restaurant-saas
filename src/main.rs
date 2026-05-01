mod api;
mod config;
mod db;
mod error;
mod middleware;
mod utils;

use std::net::SocketAddr;

use crate::{
    api::{auth::auth_controller, payment::payment_handler, subscription::subscription_handler},
    config::AppConfig,
    db::pool::connect_to_db,
    middleware::cors::cors,
};
use axum::Router;
use tokio::net::TcpListener;
use tracing::info;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

#[tokio::main]
async fn main() {
    config::init();

    let pg_pool = connect_to_db().await;
    let app_config = AppConfig {
        db: pg_pool.clone(),
    };

    let app = Router::new()
        .nest("/auth", auth_controller(app_config.clone()))
        .nest("/payment", payment_handler(app_config.clone()))
        .nest("/subscription", subscription_handler(app_config.clone()))
        .layer(cors())
        .with_state(app_config)
        .merge(SwaggerUi::new("/swagger-ui").url("/docs/openapi.json", config::ApiDoc::openapi()));

    let listener = TcpListener::bind("0.0.0.0:8000").await.unwrap();

    info!(
        "Server running on port :{}",
        listener.local_addr().unwrap().port()
    );
    axum::serve(
        listener,
        app.into_make_service_with_connect_info::<SocketAddr>(),
    )
    .await
    .unwrap();
}
