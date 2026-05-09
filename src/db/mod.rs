pub mod audit;
pub mod models;
pub mod restaurant;
pub mod session;
pub mod staff;
pub mod subscription;

use std::time::Duration;

use sqlx::{PgPool, postgres::PgPoolOptions};

pub async fn connect_to_db() -> PgPool {
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    PgPoolOptions::new()
        .max_connections(100)
        .acquire_timeout(Duration::from_secs(3))
        .connect(&database_url)
        .await
        .expect("Failed to connect to database")
}
