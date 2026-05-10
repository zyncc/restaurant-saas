use std::str::FromStr;

use axum::{
    Extension, Json,
    extract::{Multipart, State},
    http::status::StatusCode,
    response::IntoResponse,
};
use bigdecimal::BigDecimal;
use bytes::Bytes;
use uuid::Uuid;

use crate::{
    api::restaurant::{
        dto::{
            CreateMenuCategoryRequest, CreateMenuItemRequest, CreateRestaurantRequest,
            CreateRestaurantTableRequest, CreateStaffMemberRequest,
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
    mut multipart: Multipart,
) -> Result<impl IntoResponse, ApiError> {
    let mut name = String::new();
    let mut description = String::new();
    let mut price: BigDecimal;
    let mut food_type = String::new();
    let mut image_bytes: Option<Bytes> = None;
    let mut category_id = String::new();
    let mut restaurant_id = String::new();
    let mut sort_order: i32 = 0;

    while let Some(field) = multipart.next_field().await.map_err(|e| {
        tracing::error!("Failed to read multipart field: {}", e);
        ApiError::BadRequest("Invalid multipart data".to_string())
    })? {
        let field_name = field.name().unwrap_or("").to_string();

        if field_name == "image" {
            image_bytes = Some(field.bytes().await.map_err(|e| {
                tracing::error!("Failed to read image bytes: {}", e);
                ApiError::BadRequest("Failed to read image data".to_string())
            })?);
            continue;
        }

        let value = field.text().await.map_err(|e| {
            tracing::error!("Failed to read field '{}': {}", field_name, e);
            ApiError::BadRequest(format!("Invalid value for field '{}'", field_name))
        })?;

        match field_name.as_str() {
            "name" => name = value,
            "description" => description = value,
            "food_type" => food_type = value,
            "category_id" => category_id = value,
            "restaurant_id" => restaurant_id = value,
            "price" => {
                price = BigDecimal::from_str(&value).map_err(|_| {
                    ApiError::BadRequest("Invalid value for field 'price'".to_string())
                })?;
            }
            "sort_order" => {
                sort_order = value.parse().map_err(|_| {
                    ApiError::BadRequest("Invalid value for field 'sort_order'".to_string())
                })?;
            }
            _ => {}
        }
    }

    Ok((
        StatusCode::CREATED,
        Json(SuccessResponse::<Uuid> {
            success: true,
            message: Some("created menu category successfully".to_string()),
            data: Some(Uuid::new_v4()),
        }),
    ))
}
