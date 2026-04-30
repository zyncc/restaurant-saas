use axum::{Router, routing::post};

use crate::{api::payment::services, config::AppConfig};

pub fn payment_handler() -> Router<AppConfig> {
    Router::new()
        .route("/create-checkout", post(services::create_checkout))
        .route("/webhook/stripe", post(services::stripe_webhook))
}
