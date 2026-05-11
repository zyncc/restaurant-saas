use sqlx::PgExecutor;

use crate::{
    db::models::subscription::{CreateSubscriptionDto, Subscription},
    error::ApiError,
};

pub struct SubscriptionRepository;

impl SubscriptionRepository {
    pub async fn create_subscription(
        executor: impl PgExecutor<'_>,
        data: CreateSubscriptionDto,
    ) -> Result<(), ApiError> {
        sqlx::query!(
            "
        INSERT INTO subscriptions (
        id,
        staff_id,
        stripe_subscription_id,
        stripe_customer_id,
        stripe_price_id,
        plan,
        duration,
        status,
        current_period_start,
        current_period_end,
        trial_started_at,
        trial_ends_at)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12)",
            data.id,
            data.staff_id,
            data.stripe_subscription_id,
            data.stripe_customer_id,
            data.stripe_price_id,
            data.plan,
            data.duration,
            data.status,
            data.current_period_start,
            data.current_period_end,
            data.trial_started_at,
            data.trial_ends_at,
        )
        .execute(executor)
        .await
        .map_err(|e| {
            tracing::error!("db error: {}", e);
            ApiError::InternalServerError
        })?;

        Ok(())
    }

    pub async fn check_active_subscription(
        executor: impl PgExecutor<'_>,
    ) -> Result<Option<Subscription>, ApiError> {
        let subscription = sqlx::query_as!(
            Subscription,
            "SELECT * FROM subscriptions WHERE status = 'active'"
        )
        .fetch_optional(executor)
        .await
        .map_err(|e| {
            tracing::error!("db error: {}", e);
            ApiError::InternalServerError
        })?;

        Ok(subscription)
    }
}
