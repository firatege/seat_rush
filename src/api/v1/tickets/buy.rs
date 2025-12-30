use actix_web::{get, post, web, HttpResponse};
use actix_web::web::Json;
use crate::domain::dtos::tickets::{BuyRequest, BuyRequestPath};
use crate::infrastructure::state::AppState;
use crate::service::tickets::buy::buy as buy_service;
use crate::utils::error::GatewayError;

#[post("/buy")]
pub async fn buy(
    state: web::Data<AppState>,
    user: crate::domain::models::user::RequestUser,
    path: web::Query<BuyRequestPath>,
    body: Json<BuyRequest>,
) -> Result<HttpResponse, GatewayError> {
    buy_service(state,path,body,user).await
}
