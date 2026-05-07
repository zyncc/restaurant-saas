pub mod dto;
pub mod routes;
pub mod services;

use axum::{Router, middleware, routing::post};

use crate::{config::AppConfig, middleware::auth::protect_owner_route};

pub fn restaurant_handler(state: AppConfig) -> Router<AppConfig> {
    let owner_routes = Router::new()
        .route("/", post(routes::create_restaurant))
        .layer(middleware::from_fn_with_state(state, protect_owner_route));

    // let public = Router::new();

    Router::new().merge(owner_routes)
}
