use base64::{Engine as _, engine::general_purpose};
use rand::RngCore;
use sha2::{Digest, Sha256};
use sqlx::PgPool;

use crate::{
    db::{models::session::GetStaffSession, session::SessionRepository},
    error::ApiError,
};

pub fn generate_session_token() -> String {
    let mut bytes = [0u8; 32];
    rand::rngs::OsRng.fill_bytes(&mut bytes);
    let raw_bytes = general_purpose::URL_SAFE_NO_PAD.encode(bytes);
    hash_token(&raw_bytes)
}

fn hash_token(token: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(token);
    hex::encode(hasher.finalize())
}

pub async fn get_session(db: PgPool, token: &str) -> Result<GetStaffSession, ApiError> {
    let session = SessionRepository::fetch_staff_session(&db, token)
        .await
        .map_err(|e| {
            tracing::error!("Failed to fetch session: {}", e);
            ApiError::UnAuthenticated
        })?
        .ok_or(ApiError::UnAuthenticated)?;

    Ok(session)
}
