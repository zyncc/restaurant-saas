pub mod dto;
pub mod routes;
mod services;

use axum::{Router, middleware, routing::post};

use crate::{config::AppConfig, middleware::auth::auth_middleware};

pub fn payment_handler(state: AppConfig) -> Router<AppConfig> {
    let protected_routes = Router::new().route(
        "/create-checkout",
        post(routes::create_stripe_checkout)
            .layer(middleware::from_fn_with_state(state, auth_middleware)),
    );

    let public_routes = Router::new().route("/webhook/stripe", post(routes::stripe_webhook));

    Router::new().merge(protected_routes).merge(public_routes)
}
