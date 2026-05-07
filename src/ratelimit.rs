use std::sync::Arc;

use governor::clock::QuantaInstant;
use governor::middleware::NoOpMiddleware;
use tower_governor::governor::{GovernorConfig, GovernorConfigBuilder};
use tower_governor::key_extractor::SmartIpKeyExtractor;

use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use tower_governor::GovernorError;

use crate::utils::api_responses::ErrorResponse;

pub fn strict_ratelimitter()
-> Arc<GovernorConfig<SmartIpKeyExtractor, NoOpMiddleware<QuantaInstant>>> {
    Arc::new(
        GovernorConfigBuilder::default()
            .per_second(1)
            .burst_size(100)
            .key_extractor(SmartIpKeyExtractor)
            .error_handler(|err| handle_governor_error(err))
            .finish()
            .unwrap(),
    )
}

fn handle_governor_error(err: GovernorError) -> Response {
    let (status, msg) = match err {
        GovernorError::TooManyRequests { wait_time, .. } => (
            StatusCode::TOO_MANY_REQUESTS,
            format!("too many requests, retry after {}s", wait_time),
        ),
        GovernorError::UnableToExtractKey => {
            (StatusCode::BAD_REQUEST, "missing ip address".to_string())
        }
        e => {
            tracing::error!("rate limitter error: {e}");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Unknown error".to_string(),
            )
        }
    };

    (
        status,
        Json(ErrorResponse {
            success: false,
            error: msg,
        }),
    )
        .into_response()
}
