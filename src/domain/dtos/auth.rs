use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Serialize, Deserialize, Validate)]
pub struct LoginRequest {
    #[validate(email(message="invalid_email"))]
    pub email: String,
    #[validate(length(min = 6,max=61, message = "Password must be between 6 and 61 characters"))]
    pub password: String,
}


#[derive(Serialize, Deserialize, Validate)]
pub struct RegisterRequest {
    #[validate(length(min = 3,max=25,message="username must be between 3 and 25"))]
    pub username: String,
    #[validate(email(message="invalid_email"))]
    pub email: String,
    #[validate(length(min = 6,max=61, message = "Password must be between 6 and 61 characters"))]
    pub password: String,
}

#[derive(Serialize, Deserialize, Validate)]
pub struct RegisterResponse {

}

#[derive(Serialize, Deserialize, Validate)]
pub struct RefreshTokenRequest {
    pub refresh_token: String,
}

#[derive(Serialize, Deserialize, Validate)]
pub struct AuthResponse {
    pub access_token: String,
}


#[derive(Serialize, Deserialize, Validate)]
pub struct RefreshTokenResponse {
    pub access_token: String,
}


