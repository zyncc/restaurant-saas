use argon2::{Argon2, PasswordHasher, password_hash::SaltString};
use rand::rngs::OsRng;

use crate::error::ApiError;

pub fn hash_password(password: &str) -> Result<String, ApiError> {
    let raw_password = password.as_bytes();
    let salt = SaltString::generate(&mut OsRng);

    let argon2 = Argon2::default();

    Ok(argon2
        .hash_password(raw_password, &salt)
        .map_err(|e| {
            tracing::error!("failed to hash password: {}", e);
            return ApiError::InternalServerError;
        })?
        .to_string())
}
