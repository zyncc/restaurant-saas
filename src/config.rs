use sqlx::PgPool;
use utoipa::OpenApi;

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

#[derive(OpenApi)]
#[openapi(paths(
    crate::api::auth::routes::register,
    crate::api::auth::routes::login,
    crate::api::auth::routes::signout,
    crate::api::auth::routes::get_session,
    crate::api::payment::routes::create_stripe_checkout,
    crate::api::payment::routes::webhook_subscription_created,
    crate::api::subscription::routes::manage_subscription,
    crate::api::restaurant::routes::create_restaurant,
    crate::api::restaurant::routes::create_staff_member,
    crate::api::restaurant::routes::create_menu_category,
    crate::api::restaurant::routes::create_menu_item,
))]
pub struct ApiDoc;
