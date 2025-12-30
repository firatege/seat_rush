use sea_orm::ColumnTrait;
use sea_orm::{ActiveModelTrait, DatabaseConnection, EntityTrait, Set, QueryFilter};
use uuid::Uuid;
use crate::domain::entities::{prelude::*, ticket,seat};
use crate::utils::error::GatewayError;

pub struct TicketRepo;



impl TicketRepo {
    pub(crate) async fn create(
        db: &DatabaseConnection,
        user_id: Uuid,
        seat_id: i32,
        cost: i32
    ) -> Result<ticket::Model, GatewayError> {
        let new_ticket = ticket::ActiveModel {
            id: Set(Uuid::new_v4()),
            user_id: Set(user_id),
            seat_id: Set(seat_id),
            cost: Set(cost.into()),
            created: Default::default(),
        };

        new_ticket.insert(db).await
            .map_err(|err| GatewayError::DbError(err))
    }

    async fn get_user_tickets(
        db: &DatabaseConnection, user_id: Uuid
    ) -> Result<Vec<ticket::Model>, GatewayError> {
        let tickets = ticket::Entity::find()
            .filter(ticket::Column::UserId.eq(user_id))
            .all(db).await?;

        Ok(tickets)
    }

    
        async fn delete(db: &DatabaseConnection, seat_id: i32) -> Result<(), GatewayError> {
            let seat = seat::Entity::find_by_id(seat_id)
                .one(db)
                .await?
                .ok_or(GatewayError::NotFound("Seat not found".into()))?;

            seat::Entity::delete_by_id(seat.id).exec(db).await?;

            Ok(())
        }
    }



