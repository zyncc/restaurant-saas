use sqlx::{PgConnection, PgPool, Postgres, Transaction};

use crate::db::models::subscription::{CreateSubscriptionDto, Subscription};

pub struct SubscriptionRepository<'a> {
    db: SubscriptionDb<'a>,
}

enum SubscriptionDb<'a> {
    Pool(PgPool),
    Transaction(&'a mut Transaction<'static, Postgres>),
}

impl<'a> SubscriptionRepository<'a> {
    pub fn new(pool: PgPool) -> Self {
        Self {
            db: SubscriptionDb::Pool(pool),
        }
    }

    pub fn new_with_transaction(tx: &'a mut Transaction<'static, Postgres>) -> Self {
        Self {
            db: SubscriptionDb::Transaction(tx),
        }
    }

    fn get_conn(&mut self) -> &mut PgConnection {
        match &mut self.db {
            SubscriptionDb::Pool(pool) => panic!("use get_conn on pool directly"),
            SubscriptionDb::Transaction(tx) => tx.as_mut(),
        }
    }

    pub async fn create_subscription(
        &self,
        data: CreateSubscriptionDto,
        conn: &mut PgConnection,
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
        .execute(conn)
        .await?;

        Ok(())
    }

    pub async fn check_active_subscription(
        &self,
        conn: &mut PgConnection,
    ) -> Result<Option<Subscription>, sqlx::Error> {
        let subscription = sqlx::query_as!(
            Subscription,
            "SELECT * FROM subscriptions WHERE status = 'active'"
        )
        .fetch_optional(conn)
        .await?;

        Ok(subscription)
    }
}
