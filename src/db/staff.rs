use sqlx::PgExecutor;
use uuid::Uuid;

use crate::{
    db::models::staff::{CreateOwnerParams, CreateStaffMemberParams, StaffMember},
    error::ApiError,
};

pub struct StaffRepository;

#[allow(dead_code)]
impl StaffRepository {
    pub async fn create_staff_member(
        executor: impl PgExecutor<'_>,
        data: CreateStaffMemberParams,
    ) -> Result<(), sqlx::Error> {
        sqlx::query!(
            "INSERT INTO restaurant_staff (id, restaurant_id, name, email, password_hash, role) VALUES ($1, $2, $3, $4, $5, $6)",
            data.id,
            data.restaurant_id,
            data.name,
            data.email,
            data.password_hash,
            data.role,
        )
        .execute(executor)
        .await?;
        Ok(())
    }

    pub async fn create_owner(
        executor: impl PgExecutor<'_>,
        data: CreateOwnerParams,
    ) -> Result<(), sqlx::Error> {
        sqlx::query!(
            "INSERT INTO restaurant_staff (id, name, email, password_hash, role, onboarding_step) VALUES ($1, $2, $3, $4, $5, $6)",
            data.id,
            data.name,
            data.email,
            data.password_hash,
            "owner",
            "subscription",
        )
        .execute(executor)
        .await?;
        Ok(())
    }

    pub async fn find_by_email(
        executor: impl PgExecutor<'_>,
        email: &str,
    ) -> Result<StaffMember, sqlx::Error> {
        let staff = sqlx::query_as!(
            StaffMember,
            "SELECT * FROM restaurant_staff WHERE email = $1",
            email
        )
        .fetch_one(executor)
        .await?;

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
                     SET onboarding_step = $1, stripe_customer_id = $2, updated_at = now()
                     WHERE id = $3
                     AND onboarding_step NOT IN ('complete', 'create_restaurant')",
            onboarding_step,
            stripe_customer_id,
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

    pub async fn update_restaurant(
        executor: impl PgExecutor<'_>,
        id: Uuid,
        restaurant_id: Uuid,
    ) -> Result<(), ApiError> {
        sqlx::query!(
            "UPDATE restaurant_staff
                     SET restaurant_id = $1, updated_at = now()
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
