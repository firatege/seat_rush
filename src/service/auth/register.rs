use actix_web::{HttpResponse, web::{Data, Json}};
use actix_http::StatusCode;
use crate::domain::dtos::auth::{RegisterRequest, RegisterResponse};
use crate::infrastructure::state::AppState;
use argon2::{
    password_hash::{
        rand_core::OsRng,
        PasswordHasher, SaltString
    },
    Argon2
};
use crate::repository::user::UserRepo;
use crate::utils::responser::BodyBuilder;
use crate::utils::error::GatewayError;

pub async fn register(
    state: Data<AppState>,
    body:Json<RegisterRequest>

)-> Result<HttpResponse, GatewayError>{
    let user_data= body.into_inner();

    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let password_hash = argon2.hash_password(user_data.password.as_bytes(), &salt)
        .map_err(|_| GatewayError::Internal("Password hashing failed".to_string()))?
        .to_string();

    UserRepo::create(&state.db,user_data.username.clone(),user_data.email,password_hash).await
        .map_err(|err| GatewayError::Internal(err.to_string()))?;

    Ok(
        BodyBuilder::from("User created successfully")
            .status(StatusCode::CREATED)
           // .add_data(RegisterResponse {})
            .into()
    )
}