use sea_orm::ColumnTrait;
use sea_orm::QueryFilter;
use sea_orm::{DatabaseConnection, EntityTrait};
use crate::domain::entities::seat;
use crate::utils::error::GatewayError;

pub struct SeatRepo;

impl SeatRepo {
pub async fn find_by_id(
    db: &DatabaseConnection, seat_id: i32
) -> Result<Option<seat::Model>, GatewayError> {
    let seat = seat::Entity::find_by_id(seat_id).one(db).await?;
    Ok(seat)
}
}