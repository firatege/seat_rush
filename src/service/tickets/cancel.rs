use actix_web::{HttpResponse, web::Data, web};
use actix_web::http::StatusCode;
use crate::domain::models::user::RequestUser;
use crate::domain::dtos::tickets::{CancelRequest, CancelResponse};
use crate::infrastructure::state::AppState;
use crate::utils::error::GatewayError;
use crate::utils::responser::BodyBuilder;
use redis::AsyncCommands;

pub async fn cancel(
    state: Data<AppState>,
    query: web::Query<CancelRequest>,
    user: RequestUser
) -> Result<HttpResponse, GatewayError> {
    let seat_id = query.seat_id;

    // Redis lock kontrolü
    let mut con = state.redis.get_multiplexed_async_connection().await
        .map_err(|e| GatewayError::Internal(format!("Redis connection error: {}", e)))?;

    let key = format!("seat:{}:lock", seat_id);
    let current_owner: Option<String> = con.get(&key).await.ok();

    if current_owner.as_ref().map(|s| s.as_str()) != Some(user.id.to_string().as_str()) {
        return Err(GatewayError::Conflict("Seat not reserved by you".into()));
    }

    let _: () = con.del(&key).await.unwrap_or(());

    let body = BodyBuilder::from("Reservation canceled")
        .status(StatusCode::OK)
        .add_data(CancelResponse {
            seat_id,
            canceled_by: user.id,
        });

    Ok(body.into())
}
