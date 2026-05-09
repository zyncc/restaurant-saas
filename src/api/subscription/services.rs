use crate::{error::ApiError, utils::stripe};

pub async fn create_portal_session(customer_id: &str) -> Result<String, ApiError> {
    let url = stripe::create_portal_session(customer_id)
        .await
        .map_err(|e| {
            tracing::error!("failed to create portal session: {e}");
            ApiError::InternalServerError
        })?;

    Ok(url)
}
