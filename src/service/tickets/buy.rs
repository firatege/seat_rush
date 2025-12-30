use actix_web::{HttpResponse, web::Data, web};
use actix_web::http::StatusCode;
use actix_web::web::Query;
use crate::domain::dtos::tickets::{BuyRequest, BuyRequestPath, BuyResponse, ReserveQuery};
use crate::domain::models::user::RequestUser;
use crate::infrastructure::state::AppState;
use crate::repository::{seat::SeatRepo};
use crate::utils::error::GatewayError;
use crate::utils::responser::BodyBuilder;
use redis::AsyncCommands;
use crate::repository::tickets::TicketRepo;

pub async fn buy(
    state: Data<AppState>,
    path: Query<BuyRequestPath>,
    body: web::Json<BuyRequest>,
    user: RequestUser
) -> Result<HttpResponse, GatewayError> {
    let seat_id = path.seat_id;

    let mut con = state.redis.get_multiplexed_async_connection().await
        .map_err(|e| GatewayError::Internal(format!("Redis connection error: {}", e)))?;

    let key = format!("seat:{}:lock", seat_id);
    let current_owner: Option<String> = con.get(&key).await.ok();

    if current_owner.as_ref().map(|s| s.as_str()) != Some(user.id.to_string().as_str()) {
        return Err(GatewayError::Conflict("Seat not reserved by you".into()));
    }


    let cost = body.cost;


    let ticket = TicketRepo::create(&state.db, user.id, seat_id, cost).await?;

    let _: () = con.del(&key).await.unwrap_or(());

    let body = BodyBuilder::from("Purchase successful")
        .status(StatusCode::OK)
        .add_data(
            BuyResponse {
                seat_id,
                cost,
                user_id: user.id,
                ticket_id: ticket.id
            }
        );

    Ok(body.into())
}
