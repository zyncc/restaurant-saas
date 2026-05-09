use uuid::Uuid;

use crate::{
    api::restaurant::dto::{CreateRestaurantRequest, CreateStaffMemberRequest},
    config::AppConfig,
    db::{
        models::{
            restaurant::CreateRestaurantPayload, session::GetStaffSession,
            staff::CreateStaffMemberParams,
        },
        restaurant::RestaurantRepository,
        staff::StaffRepository,
    },
    error::ApiError,
    utils::{db::create_audit_log, password::hash_password},
};

pub async fn create_restaurant(
    app: AppConfig,
    session: GetStaffSession,
    body: CreateRestaurantRequest,
) -> Result<Uuid, ApiError> {
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
    let find_restaurant_id = session.restaurant_id;

    if find_restaurant_id.is_some() {
        let find_restaurant =
            RestaurantRepository::get_by_id(&app.db, find_restaurant_id.unwrap()).await?;
        if find_restaurant.is_some() {
            tracing::debug!("owner already has a restaurant");
            return Err(ApiError::BadRequest(
                "owner already has a restaurant".to_string(),
            ));
        }
    }

    let mut tx = app.db.begin().await.map_err(|e| {
        tracing::error!("failed to begin transaction: {e}");
        ApiError::InternalServerError
    })?;

    RestaurantRepository::create_restaurant(&mut *tx, payload).await?;
    StaffRepository::update_restaurant_info(&mut *tx, session.id, restaurant_id).await?;

    tx.commit().await.map_err(|e| {
        tracing::error!("failed to commit transaction: {e}");
        ApiError::InternalServerError
    })?;

    tokio::spawn(async move {
        create_audit_log(
            &app.db,
            restaurant_id,
            session.id,
            session.name.clone(),
            session.role.clone(),
            "restaurant.created".to_string(),
            "restaurant".to_string(),
        )
        .await
    });

    Ok(restaurant_id)
}

pub async fn create_staff_member(
    app: AppConfig,
    session: GetStaffSession,
    body: CreateStaffMemberRequest,
) -> Result<Uuid, ApiError> {
    let find_restaurant = RestaurantRepository::get_by_id(&app.db, body.restaurant_id).await?;
    if find_restaurant.is_none() {
        tracing::debug!("restaurant with that id not found");
        return Err(ApiError::BadRequest("restaurant not found".to_string()));
    }

    if session.restaurant_id != Some(body.restaurant_id) {
        return Err(ApiError::UnAuthorized);
    }

    // hash password using argon2
    let hashed_password = hash_password(&body.password)?;

    let staff_id = Uuid::new_v4();
    let payload = CreateStaffMemberParams {
        id: staff_id,
        name: body.name,
        email: body.email,
        password_hash: hashed_password,
        role: body.role,
        restaurant_id: body.restaurant_id,
    };

    StaffRepository::create_staff_member(&app.db, payload).await?;

    tokio::spawn(async move {
        create_audit_log(
            &app.db,
            session.restaurant_id.unwrap(),
            session.id,
            session.name.clone(),
            session.role.clone(),
            "member.created".to_string(),
            "staff".to_string(),
        )
        .await
    });

    Ok(staff_id)
}

pub async fn create_menu_category(
    app: AppConfig,
    session: GetStaffSession,
    body: CreateStaffMemberRequest,
) -> Result<Uuid, ApiError> {
    Ok(Uuid::new_v4())
}

pub async fn create_menu_item(
    app: AppConfig,
    session: GetStaffSession,
    body: CreateStaffMemberRequest,
) -> Result<Uuid, ApiError> {
    Ok(Uuid::new_v4())
}
