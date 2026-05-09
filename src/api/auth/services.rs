use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier, password_hash::SaltString};
use rand::rngs::OsRng;
use uuid::Uuid;
use validator::Validate;

use crate::{
    api::auth::dto::{LoginRequest, RegisterStaffMemberRequest},
    config::AppConfig,
    db::{
        models::{
            session::{CreateStaffSessionParams, GetStaffSession},
            staff::CreateOwnerParams,
        },
        session::SessionRepository,
        staff::StaffRepository,
    },
    error::ApiError,
    utils::{password::hash_password, session::generate_session_token},
};

pub async fn register(app: AppConfig, body: RegisterStaffMemberRequest) -> Result<(), ApiError> {
    body.validate().map_err(|e| {
        let message = e
            .field_errors()
            .into_values()
            .flatten()
            .filter_map(|e| e.message.as_ref())
            .next()
            .map(|m| m.to_string())
            .unwrap_or_else(|| "invalid request".to_string());
        tracing::warn!("validation failed: {}", message);
        ApiError::BadRequest(message)
    })?;

    if body.password != body.confirm_password {
        return Err(ApiError::BadRequest("passwords do not match".to_string()));
    }

    // hash password using argon2
    let hashed_password = hash_password(&body.password)?;

    let owner = CreateOwnerParams {
        id: Uuid::new_v4(),
        name: body.name,
        email: body.email,
        password_hash: hashed_password,
    };

    StaffRepository::create_owner(&app.db, owner)
        .await
        .map_err(|e| {
            if e.to_string().contains("violates unique constraint") {
                return ApiError::BadRequest(
                    "user with these credentials already exists".to_string(),
                );
            }
            tracing::error!("failed to insert staff member to db, {}", e);
            ApiError::InternalServerError
        })?;

    tracing::info!("registered staff member");
    Ok(())
}

pub async fn login(
    app: AppConfig,
    body: LoginRequest,
    ip_address: String,
    user_agent: String,
    existing_session: Option<String>,
) -> Result<String, ApiError> {
    if let Some(token) = existing_session {
        let session = SessionRepository::fetch_staff_session(&app.db, &token)
            .await
            .map_err(|e| {
                tracing::error!("failed to fetch staff session: {}", e);
                ApiError::InternalServerError
            })?;

        if session.is_some() {
            return Err(ApiError::BadRequest("already logged in".to_string()));
        }
    }

    let find_staff = StaffRepository::find_by_email(&app.db, &body.email).await?;

    // compare password with hashed password
    let parsed_hash = PasswordHash::new(&find_staff.password_hash).map_err(|e| {
        tracing::error!("failed to parse password hash: {}", e);
        ApiError::InternalServerError
    })?;

    Argon2::default()
        .verify_password(body.password.as_bytes(), &parsed_hash)
        .map_err(|_| ApiError::BadRequest("invalid credentials".to_string()))?;

    // create session token and cookie
    let session_token = generate_session_token();

    let session = CreateStaffSessionParams {
        id: Uuid::new_v4(),
        ip_address: Some(ip_address),
        staff_id: find_staff.id,
        session_token: session_token.clone(),
        user_agent: Some(user_agent),
    };

    SessionRepository::create_session(&app.db, session)
        .await
        .map_err(|e| {
            tracing::error!("failed to create database user session: {}", e);
            ApiError::InternalServerError
        })?;

    tracing::info!("user {} logged in successfully", find_staff.id);
    Ok(session_token)
}

pub async fn signout(app: AppConfig, session_token: &str) -> Result<(), ApiError> {
    SessionRepository::delete_session(&app.db, &session_token)
        .await
        .map_err(|e| {
            tracing::error!("failed to delete session: {}", e);
            ApiError::InternalServerError
        })?;

    Ok(())
}

pub async fn get_session(session: GetStaffSession) -> Result<GetStaffSession, ApiError> {
    Ok(session)
}
