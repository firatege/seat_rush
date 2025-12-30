use actix_web::{HttpResponse, web::{Data, Json}, web, Responder};
use actix_http::StatusCode;
use crate::domain::dtos::auth::{AuthResponse, LoginRequest};
use crate::infrastructure::state::AppState;
use argon2::{
    password_hash::{
        PasswordHash, PasswordVerifier
    },
    Argon2
};
use crate::repository::user::UserRepo;
use crate::utils::jwt::{create_access_token, create_refresh_token};
use crate::utils::responser::BodyBuilder;
use crate::utils::error::GatewayError;
use crate::domain::models::calims::Claims;
use chrono::{Utc, Duration};
use crate::extension::tokens_cookie::TokensCookie;
use crate::repository::auth::SessionRepo;
  
pub async fn login(
    state: Data<AppState>,
    body:Json<LoginRequest>,

)-> Result<HttpResponse, GatewayError>{
    let user_data= body.into_inner();

    let user = UserRepo::find_by_email(&state.db, &user_data.email)
        .await
        .map_err(|e| GatewayError::Internal(e.to_string()))?
        .ok_or(GatewayError::NotFound("User not found".to_string()))?;

    let parsed_hash = PasswordHash::new(&user.password_hashed)
        .map_err(|_| GatewayError::Internal("Invalid password hash".to_string()))?;

    Argon2::default()
        .verify_password(user_data.password.as_bytes(), &parsed_hash)
        .map_err(|_| GatewayError::Auth("Invalid password".to_string()))?;

    let claims = Claims {
        sub: user.id.to_string(),
        exp: (Utc::now() + Duration::minutes(state.environment.access_token_expiration)).timestamp(),
        iat: Utc::now().timestamp(),
    };

    let token = create_access_token(&state.jwt_secret, claims)?;
    let refresh_token = create_refresh_token();

    let _session = SessionRepo::create(
        &state.db,
        user.id,
        refresh_token.clone(),
        (Utc::now() + Duration::days(state.environment.refresh_token_expiration)).into(),
    ).await
    .map_err(|e| GatewayError::Internal(e.to_string()))?;
    let cookies = TokensCookie::from_tokens(Some(&refresh_token), &state.environment.lax_domain);

    let body = BodyBuilder::from("User Login is Success")
        .status(StatusCode::OK)
        .add_data(AuthResponse { access_token: token })
        .build_with_cookie(cookies.refresh_token);

    Ok(body)







}

