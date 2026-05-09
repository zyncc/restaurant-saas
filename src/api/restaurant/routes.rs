use axum::{Extension, Json, extract::State, http::status::StatusCode, response::IntoResponse};
use uuid::Uuid;

use crate::{
    api::restaurant::{
        dto::{
            CreateMenuCategoryRequest, CreateMenuItemRequest, CreateRestaurantRequest,
            CreateStaffMemberRequest,
        },
        services,
    },
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
    params(("Authorization" = String, Header, description = "Bearer token for authentication")),
    responses(
        (status = OK, body = SuccessResponse<Uuid>),
        (status = INTERNAL_SERVER_ERROR, body = ErrorResponse),
    )
)]
pub async fn create_restaurant(
    Extension(session): Extension<GetStaffSession>,
    State(app): State<AppConfig>,
    Json(body): Json<CreateRestaurantRequest>,
) -> Result<impl IntoResponse, ApiError> {
    let restaurant_id = services::create_restaurant(app, session, body).await?;

    Ok((
        StatusCode::CREATED,
        Json(SuccessResponse::<Uuid> {
            success: true,
            message: Some("created restaurant successfully".to_string()),
            data: Some(restaurant_id),
        }),
    ))
}

#[utoipa::path(
    post,
    path = "/restaurant/staff",
    description = "Register new staff members for the restaurant",
    request_body = CreateStaffMemberRequest,
    params(("Authorization" = String, Header, description = "Bearer token for authentication")),
    responses(
        (status = OK, body = SuccessResponse<Uuid>),
        (status = INTERNAL_SERVER_ERROR, body = ErrorResponse),
    )
)]
pub async fn create_staff_member(
    Extension(session): Extension<GetStaffSession>,
    State(app): State<AppConfig>,
    Json(body): Json<CreateStaffMemberRequest>,
) -> Result<impl IntoResponse, ApiError> {
    let staff_id = services::create_staff_member(app, session, body).await?;

    Ok((
        StatusCode::CREATED,
        Json(SuccessResponse::<Uuid> {
            success: true,
            message: Some("created staff member successfully".to_string()),
            data: Some(staff_id),
        }),
    ))
}

#[utoipa::path(
    post,
    path = "/restaurant/menu-categories",
    description = "Create New Menu categories for the restaurant",
    request_body = CreateMenuCategoryRequest,
    params(("Authorization" = String, Header, description = "Bearer token for authentication")),
    responses(
        (status = OK, body = SuccessResponse<String>),
        (status = INTERNAL_SERVER_ERROR, body = ErrorResponse),
    )
)]
pub async fn create_menu_category(
    Extension(session): Extension<GetStaffSession>,
    State(app): State<AppConfig>,
    Json(body): Json<CreateMenuCategoryRequest>,
) -> Result<impl IntoResponse, ApiError> {
    let category_id = services::create_menu_category(app, session, body).await?;

    Ok((
        StatusCode::CREATED,
        Json(SuccessResponse::<Uuid> {
            success: true,
            message: Some("created menu category successfully".to_string()),
            data: Some(category_id),
        }),
    ))
}

#[utoipa::path(
    post,
    path = "/restaurant/menu-item",
    description = "Create New Menu item for a menu category",
    request_body = CreateMenuItemRequest,
    params(("Authorization" = String, Header, description = "Bearer token for authentication")),
    responses(
        (status = OK, body = SuccessResponse<String>),
        (status = INTERNAL_SERVER_ERROR, body = ErrorResponse),
    )
)]
pub async fn create_menu_item(
    Extension(session): Extension<GetStaffSession>,
    State(app): State<AppConfig>,
    Json(body): Json<CreateMenuItemRequest>,
) -> Result<impl IntoResponse, ApiError> {
    let item_id = services::create_menu_item(app, session, body).await?;

    Ok((
        StatusCode::CREATED,
        Json(SuccessResponse::<Uuid> {
            success: true,
            message: Some("created menu category successfully".to_string()),
            data: Some(item_id),
        }),
    ))
}
