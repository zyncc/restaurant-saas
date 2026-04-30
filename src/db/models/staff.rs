use chrono::{DateTime, Utc};
use serde::Serialize;
use uuid::Uuid;

#[derive(Debug, sqlx::FromRow, Serialize)]
pub struct StaffMember {
    pub id: Uuid,
    pub restaurant_id: Option<Uuid>,
    pub role: String,
    pub name: String,
    pub email: String,
    pub password_hash: String,
    pub onboarding_step: Option<String>,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

pub struct CreateStaffMemberParams {
    pub id: Uuid,
    pub restaurant_id: Uuid,
    pub name: String,
    pub email: String,
    pub password_hash: String,
    pub role: String,
}

pub struct CreateOwnerParams {
    pub id: Uuid,
    pub name: String,
    pub email: String,
    pub password_hash: String,
}
