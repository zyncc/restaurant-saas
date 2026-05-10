use uuid::Uuid;

use crate::{
    api::payment::dto::CreateCheckoutSessionRequest,
    config::AppConfig,
    db::{
        models::{session::GetStaffSession, subscription::CreateSubscriptionDto},
        staff::StaffRepository,
        subscription::SubscriptionRepository,
    },
    error::ApiError,
    utils::stripe::{self, types::subscription_created::SubscriptionCreated},
};

pub async fn create_checkout(
    session: GetStaffSession,
    app: AppConfig,
    body: CreateCheckoutSessionRequest,
) -> Result<String, ApiError> {
    // check if user already has an active subscription
    // TODO: UNCOMMENT THIS LATER
    // let subscription = SubscriptionRepository::check_active_subscription(&app.db)
    //     .await
    //     .map_err(|e| {
    //         tracing::error!("failed to fetch active subscription: {}", e);
    //         ApiError::InternalServerError
    //     })?;

    // if subscription.is_some() {
    //     return Err(ApiError::BadRequest(
    //         "you already have an active subscription".to_string(),
    //     ));
    // }

    let stripe_price_id;
    let plan;
    let duration;
    match body.plan.as_str() {
        "basic" => match body.duration.as_str() {
            "1-month" => {
                plan = "basic";
                duration = "1-month";
                stripe_price_id = std::env::var("STRIPE_PRICE_ID_BASIC_1M").map_err(|e| {
                    tracing::error!("environment variable not set: {}", e.to_string());
                    ApiError::InternalServerError
                })?;
            }
            "1-year" => {
                plan = "basic";
                duration = "1-year";
                stripe_price_id = std::env::var("STRIPE_PRICE_ID_BASIC_1Y").map_err(|e| {
                    tracing::error!("environment variable not set: {}", e.to_string());
                    ApiError::InternalServerError
                })?
            }
            _ => {
                tracing::error!("invalid duratipn");
                return Err(ApiError::BadRequest("invalid duration".to_string()));
            }
        },
        "pro" => match body.duration.as_str() {
            "1-month" => {
                plan = "pro";
                duration = "1-month";
                stripe_price_id = std::env::var("STRIPE_PRICE_ID_PRO_1M").map_err(|e| {
                    tracing::error!("environment variable not set: {}", e.to_string());
                    ApiError::InternalServerError
                })?
            }
            "1-year" => {
                plan = "pro";
                duration = "1-year";
                stripe_price_id = std::env::var("STRIPE_PRICE_ID_PRO_1Y").map_err(|e| {
                    tracing::error!("environment variable not set: {}", e.to_string());
                    ApiError::InternalServerError
                })?
            }
            _ => {
                tracing::error!("invalid duratipn");
                return Err(ApiError::BadRequest("invalid duration".to_string()));
            }
        },
        "ultimate" => match body.duration.as_str() {
            "1-month" => {
                plan = "ultimate";
                duration = "1-month";
                stripe_price_id = std::env::var("STRIPE_PRICE_ID_ULTIMATE_1M").map_err(|e| {
                    tracing::error!("environment variable not set: {}", e.to_string());
                    ApiError::InternalServerError
                })?
            }
            "1-year" => {
                plan = "ultimate";
                duration = "1-year";
                stripe_price_id = std::env::var("STRIPE_PRICE_ID_ULTIMATE_1Y").map_err(|e| {
                    tracing::error!("environment variable not set: {}", e.to_string());
                    ApiError::InternalServerError
                })?
            }
            _ => {
                tracing::error!("invalid duratipn");
                return Err(ApiError::BadRequest("invalid duration".to_string()));
            }
        },
        _ => {
            tracing::error!("invalid plan");
            return Err(ApiError::BadRequest("invalid duration".to_string()));
        }
    }

    // call stripe api to create checkout session
    let checkout_url = stripe::create_checkout_session(
        body.trial,
        &session.email,
        &stripe_price_id,
        &session.id.to_string(),
        plan,
        duration,
    )
    .await?;

    Ok(checkout_url)
}

pub async fn webhook_subscription_created(
    app: AppConfig,
    event: SubscriptionCreated,
) -> Result<(), ApiError> {
    let staff_id = event.data.object.metadata.user_id;

    let data = CreateSubscriptionDto {
        id: Uuid::new_v4(),
        staff_id: staff_id,
        stripe_subscription_id: event.data.object.items.data[0].subscription.clone(),
        stripe_customer_id: event.data.object.customer.clone(),
        stripe_price_id: event.data.object.items.data[0].price.id.clone(),
        plan: event.data.object.metadata.plan.clone(),
        duration: event.data.object.metadata.duration.clone(),
        status: "active".to_string(),
        current_period_start: event.data.object.items.data[0].current_period_start,
        current_period_end: event.data.object.items.data[0].current_period_end,
        trial_started_at: event.data.object.trial_start,
        trial_ends_at: event.data.object.trial_end,
    };

    let mut tx = app.db.begin().await.map_err(|e| {
        tracing::error!("failed to create database transaction: {}", e);
        ApiError::InternalServerError
    })?;

    SubscriptionRepository::create_subscription(&mut *tx, data)
        .await
        .map_err(|e| {
            tracing::error!("failed to create subscription {}", e);
            ApiError::InternalServerError
        })?;

    StaffRepository::update_onboarding_step(
        &mut *tx,
        staff_id,
        &event.data.object.customer,
        "create_restaurant",
    )
    .await?;

    tx.commit().await.map_err(|e| {
        tracing::error!("create / update subscription transaction failed: {e}");
        ApiError::InternalServerError
    })?;

    Ok(())
}
