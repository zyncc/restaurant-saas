use chrono::{DateTime, Utc};
use serde::Serialize;
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug, sqlx::FromRow, Serialize)]
#[allow(dead_code)]
pub struct StaffSession {
    pub id: Uuid,
    pub staff_id: Uuid,
    pub session_token: String,
    pub ip_address: Option<String>,
    pub user_agent: Option<String>,
    pub created_at: DateTime<Utc>,
    pub expires_at: DateTime<Utc>,
}

#[derive(Debug, Clone, sqlx::FromRow, Serialize, ToSchema)]
pub struct GetStaffSession {
    pub id: Uuid,
    pub restaurant_id: Option<Uuid>,
    pub name: String,
    pub email: String,
    pub role: String,
    pub onboarding_step: Option<String>,
    pub stripe_customer_id: Option<String>,
    pub is_active: bool,
    pub sub_plan: Option<String>,
    pub sub_status: Option<String>,
    pub sub_current_period_end: Option<DateTime<Utc>>,
}

pub struct CreateStaffSessionParams {
    pub id: Uuid,
    pub staff_id: Uuid,
    pub session_token: String,
    pub ip_address: Option<String>,
    pub user_agent: Option<String>,
}
