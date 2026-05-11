use axum::{
    Extension, Json,
    extract::{Query, State},
    http::status::StatusCode,
    response::IntoResponse,
};
use serde::Deserialize;
use uuid::Uuid;

use crate::{
    api::restaurant::{
        dto::{
            CreateMenuCategoryRequest, CreateRestaurantRequest, CreateRestaurantTableRequest,
            CreateStaffMemberRequest, FetchAuditLogsQuery,
        },
        services,
    },
    config::AppConfig,
    db::models::{audit::AuditLog, session::GetStaffSession},
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
    path = "/restaurant/table",
    description = "Create a new table for the restaurant",
    request_body = CreateRestaurantTableRequest,
    params(("Authorization" = String, Header, description = "Bearer token for authentication")),
    responses(
        (status = CREATED, body = SuccessResponse<Uuid>),
        (status = INTERNAL_SERVER_ERROR, body = ErrorResponse),
    )
)]
pub async fn create_restaurant_table(
    Extension(session): Extension<GetStaffSession>,
    State(app): State<AppConfig>,
    Json(body): Json<CreateRestaurantTableRequest>,
) -> Result<impl IntoResponse, ApiError> {
    let table_id = services::create_restaurant_table(app, session, body).await?;

    Ok((
        StatusCode::CREATED,
        Json(SuccessResponse::<Uuid> {
            success: true,
            message: Some("created restaurant table successfully".to_string()),
            data: Some(table_id),
        }),
    ))
}

#[utoipa::path(
    post,
    path = "/restaurant/menu-category",
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

#[derive(Deserialize)]
pub struct RestaurantIdQuery {
    pub restaurant_id: Uuid,
}

#[utoipa::path(
    get,
    path = "/restaurant/audit-logs",
    description = "Create New Menu categories for the restaurant",
    request_body = CreateMenuCategoryRequest,
    params(
        ("Authorization" = String, Header, description = "Bearer token for authentication"),
        ("restaurant_id" = Uuid, Query, description = "Restaurant ID")
    ),
    responses(
        (status = OK, body = SuccessResponse<Vec<AuditLog>>),
        (status = INTERNAL_SERVER_ERROR, body = ErrorResponse),
    )
)]
pub async fn fetch_audit_logs(
    Extension(session): Extension<GetStaffSession>,
    State(app): State<AppConfig>,
    Query(params): Query<RestaurantIdQuery>,
) -> Result<impl IntoResponse, ApiError> {
    let restaurant_id = params.restaurant_id;
    if session.restaurant_id != Some(restaurant_id) {
        return Err(ApiError::UnAuthorized);
    }

    let logs = services::fetch_audit_logs(app, session, restaurant_id).await?;

    Ok((
        StatusCode::OK,
        Json(SuccessResponse::<Vec<AuditLog>> {
            success: true,
            message: None,
            data: Some(logs),
        }),
    ))
}
