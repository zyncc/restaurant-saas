use chrono::{DateTime, Utc};
use serde::Serialize;

#[derive(Debug, Serialize)]
#[allow(dead_code)]
pub struct SubscriptionResponse {
    pub id: String,
    pub user_id: String,
    pub stripe_subscription_id: String,
    pub stripe_customer_id: String,
    pub stripe_price_id: String,
    pub status: String,
    pub plan: String,
    pub duration: String,
    pub current_period_start: DateTime<Utc>,
    pub current_period_end: DateTime<Utc>,
    pub cancel_at: Option<DateTime<Utc>>,
    pub cancelled_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
