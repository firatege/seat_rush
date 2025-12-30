use actix_web::{post, web, HttpResponse};
use actix_web::web::Json;
use crate::domain::dtos::tickets::{BuyRequest, CancelRequest};
use crate::infrastructure::state::AppState;
use crate::utils::error::GatewayError;

#[post("/cancel")]
pub async fn cancel(
    state: web::Data<AppState>,
    user: crate::domain::models::user::RequestUser,
    path: web::Query<CancelRequest>,
) -> Result<HttpResponse, GatewayError> {
    crate::service::tickets::cancel::cancel(state,path,user).await
}
