use sqlx::PgExecutor;
use uuid::Uuid;

use crate::db::{audit::AuditRepository, models::audit::CreateAuditLogPayload};

pub async fn create_audit_log(
    executor: impl PgExecutor<'_>,
    restaurant_id: Uuid,
    staff_id: Uuid,
    staff_name: String,
    staff_role: String,
    action: String,
    entity: String,
) {
    let payload = CreateAuditLogPayload {
        id: Uuid::new_v4(),
        restaurant_id,
        staff_id,
        staff_name,
        staff_role,
        action,
        entity,
    };

    AuditRepository::create_audit(executor, payload).await;
}
