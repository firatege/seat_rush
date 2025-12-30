use actix_web::HttpResponse;
use actix_web::web::{Data, Path, Query};
use actix_web::http::StatusCode;
use crate::domain::dtos::tickets::{ReserveQuery, ReserveResponse};
use crate::domain::models::user::RequestUser;
use crate::infrastructure::state::AppState;
use crate::repository::seat::SeatRepo;
use crate::utils::error::GatewayError;
use crate::utils::responser::BodyBuilder;

pub async fn reserve(
    state: Data<AppState>,
    path: Query<ReserveQuery>,
    user: RequestUser

)-> Result<HttpResponse, GatewayError> {


    let seat_id = path.into_inner().seat_id;

    let seat = SeatRepo::find_by_id(&state.db,seat_id).await?
        .ok_or(GatewayError::NotFound("Seat not found".to_owned()))?;

    let mut con = state
        .redis
        .get_multiplexed_async_connection()
        .await
        .map_err(|e| GatewayError::Internal(format!("Redis connection error: {}", e)))?;

    let key = format!("seat:{}:lock", seat_id);
    let lua_script = r#"
    if redis.call('exists', KEYS[1]) == 1 then
        return 0
    else
        redis.call('set', KEYS[1], ARGV[1], 'EX', ARGV[2], 'NX')
        return 1
    end
    "#;

    let lua = redis::Script::new(lua_script);

    let locked: i32 = lua
        .key(key)
        .arg(user.id.to_string())
        .arg(180)
        .invoke_async(&mut con)
        .await
        .map_err(|e| GatewayError::Internal(format!("Redis error: {}", e)))?;

    if locked == 1 {
        let body = BodyBuilder::from("Seat reserved successfully")
            .status(StatusCode::OK)
            .add_data(
                ReserveResponse {
                    seat_id
                }
            );
        Ok(body.into())
    } else {
        Err(GatewayError::Conflict("Seat is already reserved".to_string()))
    }
}
