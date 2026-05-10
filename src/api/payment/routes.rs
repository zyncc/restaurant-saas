use axum::{Extension, Json, extract::State, http::HeaderMap, response::IntoResponse};
use bytes::Bytes;

use crate::{
    api::payment::{dto::CreateCheckoutSessionRequest, services},
    config::AppConfig,
    db::models::session::GetStaffSession,
    error::ApiError,
    utils::{
        api_responses::{ErrorResponse, SuccessResponse},
        stripe::{types::subscription_created::SubscriptionCreated, validate_stripe_signature},
    },
};

#[utoipa::path(
    post,
    path = "/payment/create-checkout",
    description = "Create a Stripe checkout session",
    request_body = CreateCheckoutSessionRequest,
    responses(
        (status = OK, body = SuccessResponse<String>),
        (status = INTERNAL_SERVER_ERROR, body = ErrorResponse),
    )
)]
pub async fn create_stripe_checkout(
    Extension(session): Extension<GetStaffSession>,
    State(app): State<AppConfig>,
    Json(body): Json<CreateCheckoutSessionRequest>,
) -> Result<impl IntoResponse, ApiError> {
    let checkout_url = services::create_checkout(session, app, body).await?;

    Ok(Json(SuccessResponse::<String> {
        success: true,
        data: Some(checkout_url),
        message: None,
    }))
}

#[utoipa::path(
    post,
    path = "/payment/webhook/subscription-created",
    description = "Webhook for customer.subscription.created Event",
    request_body = SubscriptionCreated,
    responses(
        (status = OK, body = SuccessResponse<String>),
        (status = INTERNAL_SERVER_ERROR, body = ErrorResponse),
    )
)]
pub async fn webhook_subscription_created(
    State(app): State<AppConfig>,
    headers: HeaderMap,
    body: Bytes,
) -> Result<impl IntoResponse, ApiError> {
    // validate stripe signature
    let sig = headers
        .get("stripe-signature")
        .and_then(|v| v.to_str().ok())
        .ok_or(ApiError::BadRequest(
            "missing Stripe-Signature header".to_string(),
        ))?;

    if !validate_stripe_signature(
        &body,
        sig,
        std::env::var("STRIPE_WEBHOOK_SECRET").unwrap().as_str(),
        300,
    ) {
        return Err(ApiError::UnAuthorized);
    }

    let event: SubscriptionCreated = serde_json::from_slice(&body).map_err(|e| {
        tracing::error!("error deserializing : {e}");
        ApiError::BadRequest("".to_string())
    })?;

    services::webhook_subscription_created(app, event).await?;

    Ok(Json(SuccessResponse::<()> {
        success: true,
        data: None,
        message: Some("webhook processed successfully".to_string()),
    }))
}
