use std::str::FromStr;

use chrono::DateTime;
use uuid::Uuid;

use crate::{
    api::payment::dto::CreateCheckoutSessionRequest,
    config::AppConfig,
    db::{
        models::{session::GetStaffSession, subscription::CreateSubscriptionDto},
        subscription::SubscriptionRepository,
    },
    error::ApiError,
    utils::stripe::{self, types::invoice_payment_succeeded::InvoicePaymentSucceededPayload},
};

pub async fn create_checkout(
    session: GetStaffSession,
    app: AppConfig,
    body: CreateCheckoutSessionRequest,
) -> Result<String, ApiError> {
    // check if user already has an active subscription

    // TODO: UNCOMMENT THIS LATER
    // let subscription = SubscriptionRepository::new(app.db)
    //     .check_active_subscription()
    //     .await
    //     .map_err(|e| {
    //         tracing::error!("failed to fetch active subscription: {}", e);
    //         ApiError::SqlxError(e.to_string())
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
                stripe_price_id = std::env::var("STRIPE_PRICE_ID_BASIC_1M")
                    .map_err(|_| ApiError::InternalServerError())?;
            }
            "1-year" => {
                plan = "basic";
                duration = "1-year";
                stripe_price_id = std::env::var("STRIPE_PRICE_ID_BASIC_1Y")
                    .map_err(|e| ApiError::BadRequest(e.to_string()))?
            }
            _ => return Err(ApiError::BadRequest("invalid duration".to_string())),
        },
        "pro" => match body.duration.as_str() {
            "1-month" => {
                plan = "pro";
                duration = "1-month";
                stripe_price_id = std::env::var("STRIPE_PRICE_ID_PRO_1M")
                    .map_err(|_| ApiError::InternalServerError())?
            }
            "1-year" => {
                plan = "pro";
                duration = "1-year";
                stripe_price_id = std::env::var("STRIPE_PRICE_ID_PRO_1Y")
                    .map_err(|_| ApiError::InternalServerError())?
            }
            _ => return Err(ApiError::BadRequest("invalid duration".to_string())),
        },
        "ultimate" => match body.duration.as_str() {
            "1-month" => {
                plan = "ultimate";
                duration = "1-month";
                stripe_price_id = std::env::var("STRIPE_PRICE_ID_ULTIMATE_1M")
                    .map_err(|_| ApiError::InternalServerError())?
            }
            "1-year" => {
                plan = "ultimate";
                duration = "1-year";
                stripe_price_id = std::env::var("STRIPE_PRICE_ID_ULTIMATE_1Y")
                    .map_err(|_| ApiError::InternalServerError())?
            }
            _ => return Err(ApiError::BadRequest("invalid duration".to_string())),
        },
        _ => return Err(ApiError::BadRequest("invalid plan".to_string())),
    }

    // call stripe api to create checkout session
    let checkout_url = stripe::create_checkout_session(
        &session.email,
        &stripe_price_id,
        &session.id.to_string(),
        plan,
        duration,
    )
    .await?;

    Ok(checkout_url)
}

pub async fn stripe_webhook(
    app: AppConfig,
    body: InvoicePaymentSucceededPayload,
) -> Result<(), ApiError> {
    let data = CreateSubscriptionDto {
        id: Uuid::new_v4(),
        staff_id: Uuid::from_str(
            &body
                .data
                .object
                .parent
                .subscription_details
                .metadata
                .user_id,
        )
        .map_err(|e| {
            tracing::error!("failed to parse user_id as UUID: {}", e);
            ApiError::BadRequest("failed to parse uuid".to_string())
        })?,
        stripe_subscription_id: body.data.object.parent.subscription_details.subscription,
        stripe_customer_id: body.data.object.customer,
        stripe_price_id: body.data.object.lines.data[0]
            .pricing
            .price_details
            .price
            .clone(),
        plan: body.data.object.parent.subscription_details.metadata.plan,
        duration: body
            .data
            .object
            .parent
            .subscription_details
            .metadata
            .duration,
        status: "active".to_string(),
        current_period_start: DateTime::from_timestamp(
            body.data.object.lines.data[0].period.start,
            0,
        )
        .ok_or_else(|| {
            tracing::error!("failed to parse current_period_start");
            ApiError::BadRequest("failed to parse timestamp".to_string())
        })?,
        current_period_end: DateTime::from_timestamp(body.data.object.lines.data[0].period.end, 0)
            .ok_or_else(|| {
                tracing::error!("failed to parse current_period_end");
                ApiError::BadRequest("failed to parse timestamp".to_string())
            })?,
    };

    let mut tx = app.db.begin().await.map_err(|e| {
        tracing::error!("failed to create database transaction: {}", e);
        ApiError::InternalServerError()
    })?;

    SubscriptionRepository::new(app.db.clone())
        .create_subscription(data, &mut *tx)
        .await
        .map_err(|e| {
            tracing::error!("failed to create subscription {}", e);
            ApiError::SqlxError("failed to create subscription".to_string())
        })?;
    return Ok(());
}
