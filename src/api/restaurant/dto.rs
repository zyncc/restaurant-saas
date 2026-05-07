use serde::Deserialize;
use utoipa::ToSchema;

#[derive(Deserialize, ToSchema)]
pub struct CreateRestaurantRequest {
    pub name: String,
    pub slug: String,
    pub description: String,
    pub phone: String,
    pub address: String,
}
