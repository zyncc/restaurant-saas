use crate::utils::api_responses::ErrorResponse;
use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use thiserror::Error;

#[derive(Debug, Error)]
#[allow(dead_code)]
pub enum ApiError {
    #[error("Bad request: {0}")]
    BadRequest(String),

    #[error("Unauthenticated")]
    UnAuthenticated,

    #[error("Unauthorized")]
    UnAuthorized,

    #[error("Notfound: {0}")]
    NotFound(String),

    #[error("Unexpected Server Error")]
    InternalServerError,
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let (status, message) = match &self {
            ApiError::BadRequest(msg) => (StatusCode::BAD_REQUEST, msg.clone()),
            ApiError::UnAuthenticated => (
                StatusCode::UNAUTHORIZED,
                "You are not authenticated".to_string(),
            ),
            ApiError::UnAuthorized => (
                StatusCode::FORBIDDEN,
                "You are not authorized to access this resource".to_string(),
            ),
            ApiError::NotFound(msg) => (StatusCode::NOT_FOUND, msg.clone()),
            ApiError::InternalServerError => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "internal server error".to_string(),
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
