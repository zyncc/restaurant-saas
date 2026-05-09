pub mod dto;
pub mod routes;
pub mod services;

use axum::{Router, middleware, routing::post};

use crate::{
    config::AppConfig,
    middleware::auth::{protect_manager_route, protect_owner_route},
};

pub fn restaurant_handler(state: AppConfig) -> Router<AppConfig> {
    let owner_routes = Router::new()
        .route("/", post(routes::create_restaurant))
        .layer(middleware::from_fn_with_state(
            state.clone(),
            protect_owner_route,
        ));

    let manager_routes = Router::new()
        .route("/staff", post(routes::create_staff_member))
        .route("/menu-categories", post(routes::create_menu_category))
        .route("/menu-items", post(routes::create_menu_item))
        .layer(middleware::from_fn_with_state(state, protect_manager_route));

    Router::new().merge(owner_routes).merge(manager_routes)
}
