use sqlx::PgPool;

pub struct PaymentRepository {
    pub db: PgPool,
}

impl PaymentRepository {
    pub fn new(db: PgPool) -> Self {
        Self { db }
    }
}
