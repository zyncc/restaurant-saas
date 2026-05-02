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
use std::net::SocketAddr;

use crate::{
    api::auth::{
        dto::{LoginRequest, RegisterStaffMemberRequest},
        services,
    },
    config::AppConfig,
    db::models::session::GetStaffSession,
    error::ApiError,
    utils::api_responses::{ErrorResponse, SuccessResponse},
};

#[utoipa::path(
    post,
    path = "/auth/register",
    description = "Register a new owner",
    request_body = RegisterStaffMemberRequest,
    responses(
        (status = OK, body = SuccessResponse<String>),
        (status = INTERNAL_SERVER_ERROR, body = ErrorResponse),
    )
)]
pub async fn register(
    State(app): State<AppConfig>,
    Json(body): Json<RegisterStaffMemberRequest>,
) -> Result<impl IntoResponse, ApiError> {
    if let Err(e) = services::register(app, body).await {
        return Err(e);
    };

    Ok(Json(SuccessResponse::<()> {
        success: true,
        message: Some("registered successfully".to_string()),
        data: None,
    }))
}

#[utoipa::path(
    post,
    path = "/auth/login",
    description = "Login a user",
    request_body = LoginRequest,
    responses(
        (status = OK, body = SuccessResponse<String>),
        (status = INTERNAL_SERVER_ERROR, body = ErrorResponse),
    )
)]
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
        Json(SuccessResponse::<()> {
            success: true,
            message: Some("logged in successfully".to_string()),
            data: None,
        }),
    ))
}

#[utoipa::path(
    post,
    path = "/auth/signout",
    description = "Signout a user",
    responses(
        (status = OK, body = SuccessResponse<String>),
        (status = INTERNAL_SERVER_ERROR, body = ErrorResponse),
    )
)]
pub async fn signout(
    State(app): State<AppConfig>,
    jar: CookieJar,
) -> Result<impl IntoResponse, ApiError> {
    let session_token = jar
        .get("session_token")
        .map(|c| c.value().to_string())
        .ok_or_else(|| ApiError::UnAuthenticated)?;

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
        Json(SuccessResponse::<()> {
            success: true,
            data: None,
            message: Some("signed out successfully".to_string()),
        }),
    ))
}

#[utoipa::path(
    get,
    path = "/auth/get-session",
    params(
            ("Authorization" = String, Header, description = "Bearer token for authentication"),
        ),
    description = "Get Session Information for Logged in User",
    responses(
        (status = OK, body = SuccessResponse<GetStaffSession>),
        (status = INTERNAL_SERVER_ERROR, body = ErrorResponse),
    )
)]
pub async fn get_session(
    Extension(session): Extension<GetStaffSession>,
) -> Result<impl IntoResponse, ApiError> {
    let session = services::get_session(session).await?;

    Ok(Json(SuccessResponse::<GetStaffSession> {
        data: Some(session),
        success: true,
        message: None,
    }))
}
