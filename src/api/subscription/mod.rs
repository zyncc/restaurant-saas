use axum::{Router, middleware, routing::get};

use crate::{config::AppConfig, middleware::auth::auth_middleware};

pub mod dto;
pub mod routes;
mod services;

pub fn subscription_handler(state: AppConfig) -> Router<AppConfig> {
    let protected_routes = Router::new().route(
        "/manage",
        get(routes::manage_subscription)
            .layer(middleware::from_fn_with_state(state, auth_middleware)),
    );

    let public_routes = Router::new();

    Router::new().merge(protected_routes).merge(public_routes)
}
