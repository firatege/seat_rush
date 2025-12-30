use actix_web::{post, HttpResponse, web};
use crate::domain::dtos::auth::LoginRequest;
use crate::infrastructure::state::AppState;
use crate::service::auth::login::login as login_service;
use crate::utils::error::GatewayError;

#[post("/login")]
pub async fn login(
    state: web::Data<AppState>,
    body: web::Json<LoginRequest>,
) -> Result<HttpResponse, GatewayError> {
    login_service(state, body).await
}

