use axum::{Extension, Json, extract::Query, response::IntoResponse};
use serde::Deserialize;

use crate::{
    db::models::session::GetStaffSession,
    error::ApiError,
    utils::{
        api_responses::{ErrorResponse, SuccessResponse},
        stripe,
    },
};

#[derive(Deserialize)]
pub struct ManageSubscriptionParams {
    #[serde(rename = "custId")]
    pub cust_id: String,
}

#[utoipa::path(
    get,
    path = "/subscription/manage",
    params(
            ("Authorization" = String, Header, description = "Bearer token for authentication"),
            ("custId" = String, Query, description = "Stripe customer ID"),
        ),
    description = "Create a Stripe portal session for the customer",
    responses(
        (status = OK, body = SuccessResponse<String>),
        (status = INTERNAL_SERVER_ERROR, body = ErrorResponse),
    ),
)]
pub async fn manage_subscription(
    Extension(session): Extension<GetStaffSession>,
    Query(params): Query<ManageSubscriptionParams>,
) -> Result<impl IntoResponse, ApiError> {
    let session_customer_id = session
        .stripe_customer_id
        .ok_or_else(|| ApiError::UnAuthorized)?;

    if params.cust_id != session_customer_id {
        return Err(ApiError::UnAuthorized);
    }

    if params.cust_id != session_customer_id {
        return Err(ApiError::UnAuthorized);
    }

    let url = stripe::create_portal_session(&session_customer_id)
        .await
        .map_err(|e| {
            tracing::error!("failed to create portal session: {e}");
            ApiError::InternalServerError
        })?;

    Ok(Json(SuccessResponse::<String> {
        success: true,
        data: Some(url),
        message: None,
    }))
}
