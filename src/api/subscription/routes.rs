use axum::{Router, routing::get};

use crate::{api::subscription::services, config::AppConfig};

pub fn subscription_handler() -> Router<AppConfig> {
    Router::new().route("/user:id", get(services::get_subscription_by_user_id))
}
