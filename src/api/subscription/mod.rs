use axum::{Router, middleware, routing::get};

use crate::{config::AppConfig, middleware::auth::protect_owner_route};

pub mod dto;
pub mod routes;
mod services;

pub fn subscription_handler(state: AppConfig) -> Router<AppConfig> {
    let owner_routes = Router::new().route(
        "/manage",
        get(routes::manage_subscription)
            .layer(middleware::from_fn_with_state(state, protect_owner_route)),
    );

    Router::new().merge(owner_routes)
}
