use sqlx::PgPool;

use crate::db::models::session::{CreateStaffSessionParams, GetStaffSession};

pub struct SessionRepository {
    pool: PgPool,
}

impl SessionRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn create_session(&self, data: CreateStaffSessionParams) -> Result<(), sqlx::Error> {
        sqlx::query!(
            "INSERT INTO staff_sessions (
            id,
            staff_id,
            session_token,
            ip_address,
            user_agent
            ) VALUES (
            $1, $2, $3, $4, $5
            )",
            data.id,
            data.staff_id,
            data.session_token,
            data.ip_address,
            data.user_agent
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    pub async fn delete_session(&self, session_token: &str) -> Result<(), sqlx::Error> {
        sqlx::query!(
            "DELETE FROM staff_sessions WHERE session_token = $1",
            session_token
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    pub async fn fetch_staff_session(
        &self,
        session_token: &str,
    ) -> Result<Option<GetStaffSession>, sqlx::Error> {
        let session = sqlx::query_as!(
            GetStaffSession,
            r#"SELECT
    staff.id,
    staff.restaurant_id,
    staff.name,
    staff.email,
    staff.role,
    staff.onboarding_step,
    staff.is_active,
    staff.stripe_customer_id,
    sub.plan as "sub_plan?",
    sub.status as "sub_status?",
    sub.current_period_end as "sub_current_period_end?"
    FROM staff_sessions as session
    JOIN restaurant_staff as staff ON staff.id = session.staff_id
    LEFT JOIN subscriptions as sub ON sub.staff_id = staff.id
    WHERE session.session_token = $1 AND session.expires_at > NOW()"#,
            session_token
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(session)
    }
}
