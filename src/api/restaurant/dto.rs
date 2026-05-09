use serde::Deserialize;
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Deserialize, ToSchema)]
pub struct CreateRestaurantRequest {
    pub name: String,
    pub slug: String,
    pub description: String,
    pub phone: String,
    pub address: String,
}

#[derive(Deserialize, ToSchema)]
pub struct CreateStaffMemberRequest {
    pub restaurant_id: Uuid,
    pub name: String,
    pub email: String,
    pub password: String,
    pub role: String,
}
