pub mod types;

use reqwest::Client;

use crate::{
    api::payment::dto::CreateCheckoutSessionResponse, error::ApiError,
    utils::stripe::types::create_portal_session::CreatePortalSessionResponse,
};
use hex;
use hmac::{Hmac, Mac, digest::KeyInit};
use sha2::Sha256;

type HmacSha256 = Hmac<Sha256>;

pub fn validate_stripe_signature(
    payload: &[u8],
    sig_header: &str,
    secret: &str,
    tolerance_secs: u64,
) -> bool {
    // Parse t= and v1= from header
    let mut timestamp: Option<u64> = None;
    let mut signatures: Vec<&str> = vec![];

    for part in sig_header.split(',') {
        if let Some(t) = part.strip_prefix("t=") {
            timestamp = t.parse().ok();
        } else if let Some(v) = part.strip_prefix("v1=") {
            signatures.push(v);
        }
    }

    let Some(ts) = timestamp else { return false };
    if signatures.is_empty() {
        return false;
    };

    // Reject stale webhooks
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0);

    if now.abs_diff(ts) > tolerance_secs {
        return false;
    }

    // Compute HMAC-SHA256 of "{timestamp}.{payload}"
    let Ok(mut mac) = HmacSha256::new_from_slice(secret.as_bytes()) else {
        return false;
    };
    mac.update(format!("{ts}.").as_bytes());
    mac.update(payload);
    let expected = hex::encode(mac.finalize().into_bytes());

    // Compare against any v1= signature present
    signatures.iter().any(|s| *s == expected)
}

pub async fn create_checkout_session(
    trial: bool,
    email: &str,
    stripe_price_id: &str,
    user_id: &str,
    plan: &str,
    duration: &str,
) -> Result<String, ApiError> {
    let client = Client::new();
    let stripe_secret_key = std::env::var("STRIPE_SECRET_KEY").map_err(|e| {
        tracing::error!("stripe secret key is required: {}", e);
        ApiError::InternalServerError
    })?;

    let mut params = vec![
        ("success_url", "http://localhost:3000/account"),
        ("cancel_url", "http://localhost:3000/pricing"),
        ("line_items[0][price]", stripe_price_id),
        ("line_items[0][quantity]", "1"),
        ("mode", "subscription"),
        ("customer_email", email),
        ("payment_method_types[0]", "card"),
        ("metadata[user_id]", user_id),
        ("metadata[plan]", plan),
        ("metadata[duration]", duration),
        ("subscription_data[metadata][user_id]", user_id),
        ("subscription_data[metadata][plan]", plan),
        ("subscription_data[metadata][duration]", duration),
    ];

    if trial {
        params.push(("subscription_data[trial_period_days]", "14"));
    }

    let response = client
        .post("https://api.stripe.com/v1/checkout/sessions")
        .basic_auth(stripe_secret_key, Option::<&str>::None)
        .form(&params)
        .send()
        .await
        .map_err(|e| {
            tracing::error!("failed to create stripe checkout session: {}", e);
            ApiError::InternalServerError
        })?
        .json::<CreateCheckoutSessionResponse>()
        .await
        .map_err(|e| {
            tracing::error!("failed to parse stripe checkout session response: {}", e);
            ApiError::InternalServerError
        })?;

    Ok(response.url)
}

pub async fn create_portal_session(customer_id: &str) -> Result<String, ApiError> {
    let client = Client::new();

    let stripe_secret_key = std::env::var("STRIPE_SECRET_KEY").map_err(|e| {
        tracing::error!("stripe secret key is required: {}", e);
        ApiError::InternalServerError
    })?;

    let response = client
        .post("https://api.stripe.com/v1/billing_portal/sessions")
        .basic_auth(stripe_secret_key, Option::<&str>::None)
        .form(&[
            ("customer", customer_id),
            ("return_url", "http://localhost:3000/pricing"),
        ])
        .send()
        .await
        .map_err(|e| {
            tracing::error!("failed to create stripe checkout session: {}", e);
            ApiError::InternalServerError
        })?
        .json::<CreatePortalSessionResponse>()
        .await
        .map_err(|e| {
            tracing::error!("failed to parse stripe checkout session response: {}", e);
            ApiError::InternalServerError
        })?;

    Ok(response.url)
}
