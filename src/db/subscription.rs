use sqlx::PgExecutor;

use crate::db::models::subscription::{CreateSubscriptionDto, Subscription};

pub struct SubscriptionRepository;

#[allow(dead_code)]
impl SubscriptionRepository {
    pub async fn create_subscription(
        executor: impl PgExecutor<'_>,
        data: CreateSubscriptionDto,
    ) -> Result<(), sqlx::Error> {
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
        current_period_end
    ) VALUES (
        $1, $2, $3, $4, $5, $6, $7, $8, $9, $10
    )
    ON CONFLICT (staff_id) DO UPDATE SET
        stripe_subscription_id = EXCLUDED.stripe_subscription_id,
        stripe_customer_id     = EXCLUDED.stripe_customer_id,
        stripe_price_id        = EXCLUDED.stripe_price_id,
        plan                   = EXCLUDED.plan,
        duration               = EXCLUDED.duration,
        status                 = EXCLUDED.status,
        current_period_start   = EXCLUDED.current_period_start,
        current_period_end     = EXCLUDED.current_period_end,
        cancel_at              = NULL,
        cancelled_at           = NULL,
        ended_at               = NULL,
        updated_at             = now()
    ",
            data.id,
            data.staff_id,
            data.stripe_subscription_id,
            data.stripe_customer_id,
            data.stripe_price_id,
            data.plan,
            data.duration,
            data.status,
            data.current_period_start,
            data.current_period_end
        )
        .execute(executor)
        .await?;

        Ok(())
    }

    pub async fn check_active_subscription(
        executor: impl PgExecutor<'_>,
    ) -> Result<Option<Subscription>, sqlx::Error> {
        let subscription = sqlx::query_as!(
            Subscription,
            "SELECT * FROM subscriptions WHERE status = 'active'"
        )
        .fetch_optional(executor)
        .await?;

        Ok(subscription)
    }
}
