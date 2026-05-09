use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Debug, sqlx::FromRow)]
pub struct AuditLog {
    pub id: Uuid,
    pub restaurant_id: Uuid,
    pub staff_id: Uuid,
    pub staff_name: String,
    pub staff_role: String,
    pub action: String,
    pub entity: String,
    pub created_at: DateTime<Utc>,
}

pub struct CreateAuditLogPayload {
    pub id: Uuid,
    pub restaurant_id: Uuid,
    pub staff_id: Uuid,
    pub staff_name: String,
    pub staff_role: String,
    pub action: String,
    pub entity: String,
}
