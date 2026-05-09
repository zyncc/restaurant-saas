pub mod dto;
pub mod routes;
mod services;

use axum::{
    Router, middleware,
    routing::{get, post},
};

use crate::{config::AppConfig, middleware::auth::auth_middleware};

pub fn auth_controller(state: AppConfig) -> Router<AppConfig> {
    let auth_routes = Router::new()
        .route("/get-session", get(routes::get_session))
        .layer(middleware::from_fn_with_state(state, auth_middleware));

    let public = Router::new()
        .route("/login", post(routes::login))
        .route("/register", post(routes::register))
        .route("/signout", post(routes::signout));

    Router::new().merge(auth_routes).merge(public)
}
