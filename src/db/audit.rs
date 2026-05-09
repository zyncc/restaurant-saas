use sqlx::PgExecutor;

use crate::db::models::audit::CreateAuditLogPayload;

pub struct AuditRepository;

impl AuditRepository {
    pub async fn create_audit(executor: impl PgExecutor<'_>, data: CreateAuditLogPayload) {
        let _ = sqlx::query!(
            "INSERT INTO audit_logs (
            id,
            restaurant_id,
            staff_id,
            staff_name,
            staff_role,
            action,
            entity
            ) VALUES ($1, $2, $3, $4, $5, $6, $7)",
            data.id,
            data.restaurant_id,
            data.staff_id,
            data.staff_name,
            data.staff_role,
            data.action,
            data.entity,
        )
        .execute(executor)
        .await
        .map_err(|e| tracing::error!("audit log error: {}", e));
    }
}
