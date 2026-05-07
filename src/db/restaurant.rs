use sqlx::PgExecutor;
use uuid::Uuid;

use crate::{
    db::models::restaurant::{CreateRestaurantPayload, Restaurant},
    error::ApiError,
};

pub struct RestaurantRepository;

impl RestaurantRepository {
    pub async fn create_restaurant(
        executor: impl PgExecutor<'_>,
        data: CreateRestaurantPayload,
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
}
