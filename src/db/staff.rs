use sqlx::PgExecutor;
use uuid::Uuid;

use crate::{
    db::models::staff::{CreateOwnerParams, CreateStaffMemberParams, StaffMember},
    error::ApiError,
};

pub struct StaffRepository;

impl StaffRepository {
    pub async fn create_staff_member(
        executor: impl PgExecutor<'_>,
        data: CreateStaffMemberParams,
    ) -> Result<(), ApiError> {
        sqlx::query!(
            "INSERT INTO restaurant_staff (id, restaurant_id, name, email, phone, password_hash, role)
             VALUES ($1, $2, $3, $4, $5, $6, $7)",
            data.id,
            data.restaurant_id,
            data.name,
            data.email,
            data.phone,
            data.password_hash,
            data.role,
        )
        .execute(executor)
        .await
        .map_err(|e| match &e {
            sqlx::Error::Database(db_err) if db_err.code().as_deref() == Some("23505") => {
                ApiError::BadRequest("staff member with that email already exists".to_string())
            }
            _ => {
                tracing::error!("db error: {}", e);
                ApiError::InternalServerError
            }
        })?;

        Ok(())
    }

    pub async fn create_owner(
        executor: impl PgExecutor<'_>,
        data: CreateOwnerParams,
    ) -> Result<(), ApiError> {
        sqlx::query!(
            "INSERT INTO restaurant_staff (id, name, email, phone, password_hash, role, onboarding_step) VALUES ($1, $2, $3, $4, $5, $6, $7)",
            data.id,
            data.name,
            data.email,
            data.phone,
            data.password_hash,
            "user",
            "subscription",
        )
        .execute(executor)
        .await.map_err(|e| {
            if e.to_string().contains("violates unique constraint") {
                return ApiError::BadRequest(
                    "user with these credentials already exists".to_string(),
                );
            }
            tracing::error!("db error: {}", e);
            ApiError::InternalServerError
        })?;

        Ok(())
    }

    pub async fn find_by_email(
        executor: impl PgExecutor<'_>,
        email: &str,
    ) -> Result<Option<StaffMember>, ApiError> {
        let staff = sqlx::query_as!(
            StaffMember,
            "SELECT * FROM restaurant_staff WHERE email = $1",
            email
        )
        .fetch_optional(executor)
        .await
        .map_err(|e| {
            tracing::error!("db error: {}", e);
            ApiError::InternalServerError
        })?;

        Ok(staff)
    }

    pub async fn update_onboarding_step(
        executor: impl PgExecutor<'_>,
        id: Uuid,
        stripe_customer_id: &str,
        onboarding_step: &str,
    ) -> Result<(), ApiError> {
        sqlx::query!(
            "UPDATE restaurant_staff
                     SET onboarding_step = $1, stripe_customer_id = $2, role = $3, updated_at = now()
                     WHERE id = $4
                     AND onboarding_step NOT IN ('complete', 'create_restaurant')",
            onboarding_step,
            stripe_customer_id,
            "owner",
            id
        )
        .execute(executor)
        .await
        .map_err(|e| {
            tracing::error!("failed to update staff member: {e}");
            ApiError::InternalServerError
        })?;
        Ok(())
    }

    pub async fn update_restaurant_info(
        executor: impl PgExecutor<'_>,
        id: Uuid,
        restaurant_id: Uuid,
    ) -> Result<(), ApiError> {
        sqlx::query!(
            "UPDATE restaurant_staff
                     SET restaurant_id = $1, onboarding_step = 'complete', updated_at = now()
                     WHERE id = $2",
            restaurant_id,
            id,
        )
        .execute(executor)
        .await
        .map_err(|e| {
            tracing::error!("failed to update staff member: {e}");
            ApiError::InternalServerError
        })?;
        Ok(())
    }
}
