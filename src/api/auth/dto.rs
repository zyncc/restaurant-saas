use serde::Deserialize;
use utoipa::ToSchema;
use validator::Validate;

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct RegisterStaffMemberRequest {
    #[validate(length(min = 1, message = "name cannot be empty"))]
    pub name: String,

    #[validate(email(message = "invalid email address"))]
    pub email: String,

    // #[validate(regex(
    //     path = "*PASSWORD_REGEX",
    //     message = "password must be at least 8 characters, include 1 number and 1 special character"
    // ))]
    pub password: String,
    pub confirm_password: String,
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct LoginRequest {
    #[validate(email(message = "invalid email address"))]
    pub email: String,
    pub password: String,
}

// #[derive(Debug, Deserialize)]
// pub struct CreateCheckoutRequest {
//     pub plan: String,
//     pub duration: String,
// }
