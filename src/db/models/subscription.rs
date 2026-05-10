use chrono::{DateTime, Utc};
use uuid::Uuid;

pub struct Subscription {
    pub id: Uuid,
    pub staff_id: Uuid,
    pub stripe_subscription_id: String,
    pub stripe_customer_id: String,
    pub stripe_price_id: String,
    pub plan: String,
    pub duration: String,
    pub status: String,
    pub trial_started_at: Option<DateTime<Utc>>,
    pub trial_ends_at: Option<DateTime<Utc>>,
    pub current_period_start: DateTime<Utc>,
    pub current_period_end: DateTime<Utc>,
    pub cancel_at: Option<DateTime<Utc>>,
    pub cancelled_at: Option<DateTime<Utc>>,
    pub ended_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

pub struct CreateSubscriptionDto {
    pub id: Uuid,
    pub staff_id: Uuid,
    pub stripe_subscription_id: String,
    pub stripe_customer_id: String,
    pub stripe_price_id: String,
    pub plan: String,
    pub duration: String,
    pub status: String,
    pub current_period_start: Option<DateTime<Utc>>,
    pub current_period_end: Option<DateTime<Utc>>,
    pub trial_started_at: Option<DateTime<Utc>>,
    pub trial_ends_at: Option<DateTime<Utc>>,
}
