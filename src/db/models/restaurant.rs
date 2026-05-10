use bigdecimal::BigDecimal;
use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Debug, sqlx::FromRow)]
pub struct Restaurant {
    pub id: Uuid,
    pub name: String,
    pub slug: String,
    pub logo_url: Option<String>,
    pub description: String,
    pub phone: String,
    pub address: String,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, sqlx::FromRow)]
pub struct MenuCategory {
    pub id: Uuid,
    pub restaurant_id: Uuid,
    pub name: String,
    pub description: String,
    pub sort_order: i32,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, sqlx::FromRow)]
pub struct MenuItem {
    pub id: Uuid,
    pub restaurant_id: Uuid,
    pub category_id: Uuid,
    pub name: String,
    pub description: String,
    pub price: BigDecimal,
    pub image_url: String,
    pub is_available: bool,
    pub food_type: String,
    pub sort_order: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

pub struct CreateRestaurantParams {
    pub id: Uuid,
    pub name: String,
    pub slug: String,
    pub description: String,
    pub phone: String,
    pub address: String,
}

pub struct CreateMenuCategoryParams {
    pub id: Uuid,
    pub restaurant_id: Uuid,
    pub name: String,
    pub description: String,
    pub sort_order: i32,
    pub is_active: bool,
}

pub struct RestaurantTable {
    pub id: Uuid,
    pub restaurant_id: Uuid,
    pub table_number: String,
    pub label: Option<String>,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
}

pub struct CreateRestaurantTableParams {
    pub id: Uuid,
    pub restaurant_id: Uuid,
    pub table_number: String,
    pub label: Option<String>,
    pub is_active: bool,
}

pub struct CreateMenuItemParams {
    pub id: Uuid,
    pub restaurant_id: Uuid,
    pub category_id: Uuid,
    pub name: String,
    pub description: String,
    pub price: BigDecimal,
    pub image_url: String,
    pub is_available: bool,
    pub food_type: String,
    pub sort_order: i32,
}
