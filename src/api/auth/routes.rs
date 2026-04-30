use axum::{
    Extension, Json,
    extract::{ConnectInfo, State},
    http::HeaderMap,
    response::IntoResponse,
};
use axum_extra::extract::{
    CookieJar,
    cookie::{Cookie, SameSite},
};
use serde_json::json;
use std::net::SocketAddr;

use crate::{
    api::auth::{
        dto::{LoginRequest, RegisterStaffMemberRequest},
        services,
    },
    config::AppConfig,
    db::models::session::GetStaffSession,
    error::ApiError,
};

pub async fn register(
    State(app): State<AppConfig>,
    Json(body): Json<RegisterStaffMemberRequest>,
) -> Result<impl IntoResponse, ApiError> {
    if let Err(e) = services::register(app, body).await {
        return Err(e);
    };

    Ok(Json(
        json!({"success": true, "message": "registered successfully"}),
    ))
}

pub async fn login(
    State(app): State<AppConfig>,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    headers: HeaderMap,
    jar: CookieJar,
    Json(body): Json<LoginRequest>,
) -> Result<impl IntoResponse, ApiError> {
    let ip_address = addr.ip().to_string();
    let user_agent = headers
        .get("user-agent")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("")
        .to_string();

    let existing_token = jar.get("session_token").map(|c| c.value().to_string());

    let session_token = services::login(app, body, ip_address, user_agent, existing_token)
        .await
        .map_err(|e| e)?;

    let cookie = Cookie::build(("session_token", session_token))
        .path("/")
        .http_only(true)
        .secure(true)
        .max_age(time::Duration::days(1))
        .same_site(axum_extra::extract::cookie::SameSite::Lax)
        .build();

    Ok((
        jar.add(cookie),
        Json(json!({"success": true, "message": "logged in successfully"})),
    ))
}

pub async fn signout(
    State(app): State<AppConfig>,
    jar: CookieJar,
) -> Result<impl IntoResponse, ApiError> {
    let session_token = jar
        .get("session_token")
        .map(|c| c.value().to_string())
        .ok_or_else(|| ApiError::UnAuthenticated("session token is missing".to_string()))?;

    services::signout(app, &session_token).await?;

    let cookie = Cookie::build(("session_token", ""))
        .path("/")
        .http_only(true)
        .secure(true)
        .same_site(SameSite::Strict)
        .max_age(time::Duration::ZERO)
        .build();

    Ok((
        jar.remove(cookie),
        Json(json!({"success": true, "message": "signed out successfully"})),
    ))
}

pub async fn get_session(
    Extension(session): Extension<GetStaffSession>,
) -> Result<impl IntoResponse, ApiError> {
    let session = services::get_session(session).await?;
    Ok(Json(json!({"success": true, "data": session})))
}
