use serde::Deserialize;
use utoipa::ToSchema;

#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateCheckoutSessionRequest {
    pub plan: String,
    pub duration: String,
    pub trial: bool,
}

#[derive(Debug, Deserialize)]
pub struct CreateCheckoutSessionResponse {
    pub url: String,
}
