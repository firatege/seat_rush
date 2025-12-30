use sea_orm::*;
use uuid::Uuid;
use crate::domain::entities::{prelude::*, user};
use crate::utils::result::GatewayResult;

pub struct UserRepo;


impl UserRepo {
    pub async fn find_by_email(
        db: &DatabaseConnection,
        email: &str
    ) -> GatewayResult<Option<user::Model>> {
     let user =   User::find()
            .filter(user::Column::Email.eq(email))
            .one(db)
            .await?;                                                                
        Ok(user)
    }

    pub async fn create(
        db: &DatabaseConnection,
        username: String,
        email: String,
        password_hashed: String,
    ) -> Result<user::Model, DbErr> {
        let new_user = user::ActiveModel {
            id: Set(Uuid::new_v4()),
            username: Set(username),
            email: Set(email),
            password_hashed: Set(password_hashed),
            status: Set(user::UserStatus::Active),
            ..Default::default()
        };

        new_user.insert(db).await
    }
}
