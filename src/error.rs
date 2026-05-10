use crate::utils::api_responses::ErrorResponse;
use axum::{
    Json,
    extract::{FromRequest, rejection::JsonRejection},
    http::{Request, StatusCode},
    response::{IntoResponse, Response},
};
use thiserror::Error;

#[derive(Debug, Error)]
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

pub struct ValidatedJson<T>(pub T);

impl<S, T> FromRequest<S> for ValidatedJson<T>
where
    Json<T>: FromRequest<S, Rejection = JsonRejection>,
    S: Send + Sync,
{
    type Rejection = ApiError;

    async fn from_request(
        req: Request<axum::body::Body>,
        state: &S,
    ) -> Result<Self, Self::Rejection> {
        match Json::<T>::from_request(req, state).await {
            Ok(value) => Ok(Self(value.0)),
            Err(rejection) => {
                let message = match rejection {
                    JsonRejection::JsonDataError(_) => "Invalid request body",
                    JsonRejection::JsonSyntaxError(_) => "Malformed JSON",
                    JsonRejection::MissingJsonContentType(_) => {
                        "Content-Type must be application/json"
                    }
                    _ => "Bad request",
                };
                Err(ApiError::BadRequest(message.to_string()))
            }
        }
    }
}
