use sqlx::PgPool;

#[derive(Clone)]
pub struct AppConfig {
    pub db: PgPool,
}

pub fn init() {
    match dotenvy::dotenv() {
        Ok(_) => {}
        Err(_) => panic!("missing environment variables"),
    };

    tracing_subscriber::fmt()
        .with_line_number(true)
        .with_file(true)
        // .json()
        .init();
}
