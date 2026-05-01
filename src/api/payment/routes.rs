use axum::{Extension, Json, extract::State, response::IntoResponse};

use crate::{
    api::payment::{dto::CreateCheckoutSessionRequest, services},
    config::AppConfig,
    db::models::session::GetStaffSession,
    error::ApiError,
    utils::{
        api_responses::{ErrorResponse, SuccessResponse},
        stripe::types::invoice_payment_succeeded::InvoicePaymentSucceededPayload,
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
    path = "/payment/webhook/stripe",
    description = "Handle Stripe webhook events",
    request_body = InvoicePaymentSucceededPayload,
    responses(
        (status = OK, body = SuccessResponse<String>),
        (status = INTERNAL_SERVER_ERROR, body = ErrorResponse),
    )
)]
pub async fn stripe_webhook(
    State(app): State<AppConfig>,
    Json(body): Json<InvoicePaymentSucceededPayload>,
) -> Result<impl IntoResponse, ApiError> {
    services::stripe_webhook(app, body).await?;

    Ok(Json(SuccessResponse::<()> {
        success: true,
        data: None,
        message: Some("webhook processed successfully".to_string()),
    }))
}
