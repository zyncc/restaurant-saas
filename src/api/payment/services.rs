use std::str::FromStr;

use chrono::DateTime;
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
    utils::stripe::{self, types::invoice_payment_succeeded::InvoicePaymentSucceededPayload},
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
        stripe_customer_id: body.data.object.customer.clone(),
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
        ApiError::InternalServerError
    })?;

    SubscriptionRepository::create_subscription(&mut *tx, data)
        .await
        .map_err(|e| {
            tracing::error!("failed to create subscription {}", e);
            ApiError::InternalServerError
        })?;

    let user_id = &body
        .data
        .object
        .parent
        .subscription_details
        .metadata
        .user_id;

    StaffRepository::update_onboarding_step(
        &mut *tx,
        Uuid::from_str(user_id).map_err(|e| {
            tracing::error!("failed to parse user_id as UUID: {}", e);
            ApiError::BadRequest("failed to parse uuid".to_string())
        })?,
        &body.data.object.customer,
        "create_restaurant",
    )
    .await
    .map_err(|e| {
        tracing::error!("failed to update staff details: {}", e);
        ApiError::InternalServerError
    })?;

    tx.commit().await.map_err(|e| {
        tracing::error!("create / update subscription transaction failed: {e}");
        ApiError::InternalServerError
    })?;

    return Ok(());
}
