use uuid::Uuid;

use crate::{
    api::restaurant::dto::CreateRestaurantRequest,
    config::AppConfig,
    db::{
        models::{restaurant::CreateRestaurantPayload, session::GetStaffSession},
        restaurant::RestaurantRepository,
        staff::StaffRepository,
    },
    error::ApiError,
};

pub async fn create_restaurant(
    app: AppConfig,
    session: GetStaffSession,
    body: CreateRestaurantRequest,
) -> Result<(), ApiError> {
    let restaurant_id = Uuid::new_v4();
    let payload = CreateRestaurantPayload {
        id: restaurant_id,
        name: body.name,
        address: body.address,
        description: body.description,
        phone: body.phone,
        slug: body.slug,
    };

    // check if owner already has a restaurant
    let find_restaurant_id = session.restaurant_id.ok_or(ApiError::InternalServerError)?;
    let find_restaurant = RestaurantRepository::get_by_id(&app.db, find_restaurant_id).await?;

    if find_restaurant.is_some() {
        return Err(ApiError::BadRequest(
            "owner already has a restaurant".to_string(),
        ));
    }

    let mut tx = app.db.begin().await.map_err(|e| {
        tracing::error!("failed to begin transaction: {e}");
        ApiError::InternalServerError
    })?;

    RestaurantRepository::create_restaurant(&mut *tx, payload).await?;
    StaffRepository::update_restaurant(&mut *tx, session.id, restaurant_id).await?;

    tx.commit().await.map_err(|e| {
        tracing::error!("failed to commit transaction: {e}");
        ApiError::InternalServerError
    })?;

    Ok(())
}
