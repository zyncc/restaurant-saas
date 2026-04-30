use axum::{extract::Request, middleware::Next, response::Response};

use crate::{
    config::AppConfig,
    error::ApiError,
    utils::{extract_token, session::get_session},
};
use axum::extract::State;

pub async fn auth_middleware(
    State(state): State<AppConfig>,
    mut req: Request,
    next: Next,
) -> Result<Response, ApiError> {
    let token =
        extract_token(&req).ok_or(ApiError::UnAuthenticated("Unauthenticated".to_string()))?;

    let session = get_session(state.db, &token).await?;
    req.extensions_mut().insert(session);
    Ok(next.run(req).await)
}

pub async fn protect_owner_route(
    State(state): State<AppConfig>,
    mut req: Request,
    next: Next,
) -> Result<Response, ApiError> {
    let token =
        extract_token(&req).ok_or(ApiError::UnAuthenticated("Unauthenticated".to_string()))?;

    let session = get_session(state.db, &token).await?;

    let onboarding_incomplete = session.onboarding_step.as_deref() != Some("complete");

    if session.role != "owner" || onboarding_incomplete {
        return Err(ApiError::UnAuthorized(
            "you cannot access this resource".to_string(),
        ));
    }

    req.extensions_mut().insert(session);
    Ok(next.run(req).await)
}

pub async fn protect_manager_route(
    State(state): State<AppConfig>,
    mut req: Request,
    next: Next,
) -> Result<Response, ApiError> {
    let token =
        extract_token(&req).ok_or(ApiError::UnAuthenticated("Unauthenticated".to_string()))?;

    let session = get_session(state.db, &token).await?;

    if session.role != "owner" || session.role != "manager" {
        return Err(ApiError::UnAuthorized(
            "you cannot access this resource".to_string(),
        ));
    }

    req.extensions_mut().insert(session);
    Ok(next.run(req).await)
}

pub async fn protect_staff_route(
    State(state): State<AppConfig>,
    mut req: Request,
    next: Next,
) -> Result<Response, ApiError> {
    let token =
        extract_token(&req).ok_or(ApiError::UnAuthenticated("Unauthenticated".to_string()))?;

    let session = get_session(state.db, &token).await?;

    if session.role != "owner" || session.role != "manager" || session.role != "staff" {
        return Err(ApiError::UnAuthorized(
            "you cannot access this resource".to_string(),
        ));
    }

    req.extensions_mut().insert(session);
    Ok(next.run(req).await)
}
