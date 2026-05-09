use bigdecimal::BigDecimal;
use serde::Deserialize;
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateRestaurantRequest {
    pub name: String,
    pub slug: String,
    pub description: String,
    pub phone: String,
    pub address: String,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateStaffMemberRequest {
    pub restaurant_id: Uuid,
    pub name: String,
    pub email: String,
    pub password: String,
    pub role: String,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateMenuCategoryRequest {
    pub restaurant_id: Uuid,
    pub name: String,
    pub description: String,
    pub sort_order: i32,
    pub is_active: bool,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateMenuItemRequest {
    pub restaurant_id: Uuid,
    pub category_id: Uuid,
    pub name: String,
    pub description: String,
    #[schema(value_type = f64)]
    pub price: BigDecimal,
    pub image_url: String,
    pub is_available: bool,
    pub food_type: String,
    pub sort_order: i32,
}
