use sqlx::PgPool;
use uuid::Uuid;

use crate::db::models::staff::{CreateOwnerParams, CreateStaffMemberParams, StaffMember};

pub struct StaffRepository {
    pool: PgPool,
}

impl StaffRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn create_staff_member(
        &self,
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
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    pub async fn create_owner(&self, data: CreateOwnerParams) -> Result<(), sqlx::Error> {
        sqlx::query!(
            "INSERT INTO restaurant_staff (id, name, email, password_hash, role, onboarding_step) VALUES ($1, $2, $3, $4, $5, $6)",
            data.id,
            data.name,
            data.email,
            data.password_hash,
            "owner",
            "subscription",
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    pub async fn find_by_email(&self, email: &str) -> Result<StaffMember, sqlx::Error> {
        let staff = sqlx::query_as!(
            StaffMember,
            "SELECT * FROM restaurant_staff WHERE email = $1",
            email
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(staff)
    }

    pub async fn update_onboarding_step(
        &self,
        id: Uuid,
        onboarding_step: &str,
    ) -> Result<(), sqlx::Error> {
        sqlx::query!(
            "UPDATE restaurant_staff SET onboarding_step = $1, updated_at = now() WHERE id = $2",
            onboarding_step,
            id
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }
}
