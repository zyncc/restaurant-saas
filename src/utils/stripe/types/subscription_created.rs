use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, ToSchema)]
pub struct SubscriptionCreated {
    pub data: Data,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, ToSchema)]
pub struct Data {
    pub object: SubscriptionObject,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, ToSchema)]
pub struct SubscriptionObject {
    pub customer: String,
    pub items: Items,
    pub metadata: Metadata,
    #[serde(with = "chrono::serde::ts_seconds_option")]
    pub trial_start: Option<DateTime<Utc>>,
    #[serde(with = "chrono::serde::ts_seconds_option")]
    pub trial_end: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, ToSchema)]
pub struct Items {
    pub data: Vec<Item>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, ToSchema)]
pub struct Item {
    pub subscription: String,
    pub price: Price,

    #[serde(with = "chrono::serde::ts_seconds_option")]
    pub current_period_start: Option<DateTime<Utc>>,

    #[serde(with = "chrono::serde::ts_seconds_option")]
    pub current_period_end: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, ToSchema)]
pub struct Price {
    pub id: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, ToSchema)]
pub struct Metadata {
    pub duration: String,
    pub user_id: Uuid,
    pub plan: String,
}
