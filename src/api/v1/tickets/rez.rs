use actix_web::{post, web, HttpResponse};
use actix_web::web::{Query};
use crate::domain::dtos::tickets::ReserveQuery;
use crate::domain::models::user::RequestUser;
use crate::infrastructure::state::AppState;
use crate::utils::error::GatewayError;
use crate::service::tickets::reserve::reserve;

#[post("/rez")]
pub async fn rez(
    state: web::Data<AppState>,
    user: RequestUser,
    path: Query<ReserveQuery>,
) -> Result<HttpResponse, GatewayError> {
    reserve(state,path,user).await
}