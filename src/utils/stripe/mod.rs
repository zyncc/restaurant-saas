pub mod types;

use reqwest::Client;

use crate::{
    api::payment::dto::CreateCheckoutSessionResponse, error::ApiError,
    utils::stripe::types::create_portal_session::CreatePortalSessionResponse,
};

pub async fn create_checkout_session(
    email: &str,
    stripe_price_id: &str,
    user_id: &str,
    plan: &str,
    duration: &str,
) -> Result<String, ApiError> {
    let client = Client::new();
    let stripe_secret_key = std::env::var("STRIPE_SECRET_KEY").map_err(|e| {
        tracing::error!("stripe secret key is required: {}", e);
        return ApiError::InternalServerError();
    })?;

    let response = client
        .post("https://api.stripe.com/v1/checkout/sessions")
        .basic_auth(stripe_secret_key, Option::<&str>::None)
        .form(&[
            ("success_url", "http://localhost:3000/onboarding"),
            ("cancel_url", "http://localhost:3000/pricing"),
            ("line_items[0][price]", &stripe_price_id),
            ("line_items[0][quantity]", "1"),
            ("mode", "subscription"),
            ("customer_email", &email),
            ("payment_method_types[0]", "card"),
            ("branding_settings[border_style]", "pill"),
            ("branding_settings[display_name]", "Restaurant SAAS"),
            ("branding_settings[logo][type]", "url"),
            (
                "branding_settings[logo][url]",
                "https://cdn.jsdelivr.net/gh/homarr-labs/dashboard-icons/png/youtube.png",
            ),
            ("metadata[user_id]", &user_id),
            ("metadata[plan]", &plan),
            ("metadata[duration]", &duration),
            ("subscription_data[metadata][user_id]", &user_id),
            ("subscription_data[metadata][plan]", &plan),
            ("subscription_data[metadata][duration]", &duration),
        ])
        .send()
        .await
        .map_err(|e| {
            tracing::error!("failed to create stripe checkout session: {}", e);
            ApiError::InternalServerError()
        })?
        .json::<CreateCheckoutSessionResponse>()
        .await
        .map_err(|e| {
            tracing::error!("failed to parse stripe checkout session response: {}", e);
            ApiError::InternalServerError()
        })?;

    Ok(response.url)
}

pub async fn create_portal_session(customer_id: &str) -> Result<String, ApiError> {
    let client = Client::new();

    let stripe_secret_key = std::env::var("STRIPE_SECRET_KEY").map_err(|e| {
        tracing::error!("stripe secret key is required: {}", e);
        return ApiError::InternalServerError();
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
            ApiError::InternalServerError()
        })?
        .json::<CreatePortalSessionResponse>()
        .await
        .map_err(|e| {
            tracing::error!("failed to parse stripe checkout session response: {}", e);
            ApiError::InternalServerError()
        })?;

    Ok(response.url)
}
