use uuid::Uuid;

#[derive(Debug, sqlx::FromRow)]
pub struct Restaurant {
    pub id: Uuid,
    pub name: String,
    pub slug: String,
    pub logo_url: Option<String>,
    pub description: String,
    pub phone: String,
    pub address: String,
    pub is_active: bool,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

pub struct CreateRestaurantPayload {
    pub id: Uuid,
    pub name: String,
    pub slug: String,
    pub description: String,
    pub phone: String,
    pub address: String,
}
