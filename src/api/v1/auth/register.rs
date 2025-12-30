use actix_web::{post, HttpResponse, web};
use crate::domain::dtos::auth::RegisterRequest;
use crate::infrastructure::state::AppState;
use crate::service::auth::register::register as register_service;
use crate::utils::error::GatewayError;

#[post("/register")]
pub async fn register(
    state: web::Data<AppState>,
    body: web::Json<RegisterRequest>,
) -> Result<HttpResponse, GatewayError> {
    register_service(state, body).await
}