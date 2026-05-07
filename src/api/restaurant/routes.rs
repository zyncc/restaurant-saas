use axum::{Extension, Json, extract::State, response::IntoResponse};

use crate::{
    api::restaurant::{dto::CreateRestaurantRequest, services},
    config::AppConfig,
    db::models::session::GetStaffSession,
    error::ApiError,
    utils::api_responses::{ErrorResponse, SuccessResponse},
};

#[utoipa::path(
    post,
    path = "/restaurant",
    description = "Create a new restaurant with staff members",
    request_body = CreateRestaurantRequest,
    responses(
        (status = OK, body = SuccessResponse<String>),
        (status = INTERNAL_SERVER_ERROR, body = ErrorResponse),
    )
)]
pub async fn create_restaurant(
    Extension(session): Extension<GetStaffSession>,
    State(app): State<AppConfig>,
    Json(body): Json<CreateRestaurantRequest>,
) -> Result<impl IntoResponse, ApiError> {
    services::create_restaurant(app, session, body).await?;

    Ok(Json(SuccessResponse::<()> {
        success: true,
        message: Some("created restaurant successfully".to_string()),
        data: None,
    }))
}
