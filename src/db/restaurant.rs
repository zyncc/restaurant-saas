use sqlx::PgExecutor;
use uuid::Uuid;

use crate::{
    db::models::restaurant::{
        CreateMenuCategoryParams, CreateMenuItemParams, CreateRestaurantParams,
        CreateRestaurantTableParams, Restaurant,
    },
    error::ApiError,
};

pub struct RestaurantRepository;

impl RestaurantRepository {
    pub async fn create_restaurant(
        executor: impl PgExecutor<'_>,
        data: CreateRestaurantParams,
    ) -> Result<(), ApiError> {
        sqlx::query!(
            "INSERT INTO restaurants (id, name, slug, description, phone, address) VALUES ($1, $2, $3, $4, $5, $6)",
            data.id,
            data.name,
            data.slug,
            data.description,
            data.phone,
            data.address
        )
        .execute(executor)
        .await
        .map_err(|e| {
            tracing::error!("db error: {e}");
            ApiError::InternalServerError
        })?;

        Ok(())
    }

    pub async fn get_by_id(
        executor: impl PgExecutor<'_>,
        id: Uuid,
    ) -> Result<Option<Restaurant>, ApiError> {
        sqlx::query_as!(Restaurant, "SELECT * FROM restaurants WHERE id = $1", id)
            .fetch_optional(executor)
            .await
            .map_err(|e| {
                tracing::error!("db error: {e}");
                ApiError::InternalServerError
            })
    }

    pub async fn create_menu_category(
        executor: impl PgExecutor<'_>,
        data: CreateMenuCategoryParams,
    ) -> Result<(), ApiError> {
        sqlx::query!(
            "INSERT INTO menu_categories
            (id, restaurant_id, name, description, sort_order, is_active)
            VALUES ($1, $2, $3, $4, $5, $6)",
            data.id,
            data.restaurant_id,
            data.name,
            data.description,
            data.sort_order,
            data.is_active
        )
        .execute(executor)
        .await
        .map_err(|e| {
            tracing::error!("db error: {e}");
            ApiError::InternalServerError
        })?;

        Ok(())
    }

    pub async fn create_table(
        executor: impl PgExecutor<'_>,
        data: CreateRestaurantTableParams,
    ) -> Result<(), ApiError> {
        sqlx::query!(
            "INSERT INTO tables (id, restaurant_id, table_number, label, is_active)
            VALUES ($1, $2, $3, $4, $5)",
            data.id,
            data.restaurant_id,
            data.table_number,
            data.label,
            data.is_active
        )
        .execute(executor)
        .await
        .map_err(|e| {
            if e.to_string().contains("unique constraint") {
                return ApiError::BadRequest("table already exists".to_string());
            }
            tracing::error!("db error: {e}");
            ApiError::InternalServerError
        })?;

        Ok(())
    }

    pub async fn create_menu_item(
        executor: impl PgExecutor<'_>,
        data: CreateMenuItemParams,
    ) -> Result<(), ApiError> {
        sqlx::query!(
            "INSERT INTO menu_items
            (id, restaurant_id, category_id, name, description, price, image_url, is_available, food_type, sort_order)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)",
            data.id,
            data.restaurant_id,
            data.category_id,
            data.name,
            data.description,
            data.price,
            data.image_url,
            data.is_available,
            data.food_type,
            data.sort_order
        )
        .execute(executor)
        .await
        .map_err(|e| {
            tracing::error!("db error: {e}");
            ApiError::InternalServerError
        })?;

        Ok(())
    }
}
