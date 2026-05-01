use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};

use thiserror::Error;

use crate::utils::api_responses::ErrorResponse;

#[derive(Debug, Error)]
#[allow(dead_code)]
pub enum ApiError {
    #[error("Bad request: {0}")]
    BadRequest(String),

    #[error("Unauthenticated: {0}")]
    UnAuthenticated(String),

    #[error("Unauthorized: {0}")]
    UnAuthorized(String),

    #[error("Notfound: {0}")]
    NotFound(String),

    #[error("Database query failed: {0}")]
    SqlxError(String),

    #[error("Unexpected Server Error")]
    InternalServerError(),
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let (status, message) = match &self {
            ApiError::SqlxError(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg),
            ApiError::BadRequest(msg) => (StatusCode::BAD_REQUEST, msg),
            ApiError::UnAuthenticated(msg) => (StatusCode::UNAUTHORIZED, msg),
            ApiError::UnAuthorized(msg) => (StatusCode::FORBIDDEN, msg),
            ApiError::NotFound(msg) => (StatusCode::NOT_FOUND, msg),
            ApiError::InternalServerError() => (
                StatusCode::INTERNAL_SERVER_ERROR,
                &"internal server error".to_string(),
            ),
        };

        let body = Json(ErrorResponse {
            success: false,
            error: message.to_string(),
        })
        .into_response();
        (status, body).into_response()
    }
}
