use axum::http::{
    HeaderValue, Method,
    header::{AUTHORIZATION, CONTENT_TYPE},
};
use tower_http::cors::CorsLayer;

pub fn cors() -> CorsLayer {
    CorsLayer::new()
        .allow_origin("http://localhost:3000".parse::<HeaderValue>().unwrap())
        .allow_methods([
            Method::GET,
            Method::POST,
            Method::DELETE,
            Method::PUT,
            Method::PATCH,
        ])
        .allow_credentials(true)
        .allow_headers([AUTHORIZATION, CONTENT_TYPE])
}
