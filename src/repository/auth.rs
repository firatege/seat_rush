use sea_orm::*;
use chrono::{Utc, DateTime, FixedOffset};
use sea_orm::prelude::Expr;
use uuid::Uuid;
use crate::domain::entities::{prelude::*, sessions};

pub struct SessionRepo;

impl SessionRepo {
    pub async fn create(
        db: &DatabaseConnection,
        user_id: Uuid,
        token_hash: String,
        expires_at: DateTime<FixedOffset>,
    ) -> Result<sessions::Model, DbErr> {
        let new_session = sessions::ActiveModel {
            id: Set(Uuid::new_v4()),
            user_id: Set(user_id),
            refresh_token_hash: Set(token_hash),
            expires_at: Set(expires_at),
            is_revoked: Set(false),
            created_at: Set(Utc::now().into()),
            last_used_at: Set(Utc::now().into()),
            ..Default::default()
        };

        new_session.insert(db).await
    }

    pub async fn find_by_hash(
        db: &DatabaseConnection,
        hash: &str
    ) -> Result<Option<sessions::Model>, DbErr> {
        Sessions::find()
            .filter(sessions::Column::RefreshTokenHash.eq(hash))
            .one(db)
            .await
    }


    pub async fn update_last_used(
        db: &DatabaseConnection,
        session_id: Uuid,
    ) -> Result<(), DbErr> {
        let session = sessions::ActiveModel {
            id: Set(session_id),
            last_used_at: Set(Utc::now().into()),
            ..Default::default()
        };

        session.update(db).await?;
        Ok(())
    }


    pub async fn revoke(
        db: &DatabaseConnection,
        session_id: Uuid
    ) -> Result<(), DbErr> {
        let session = sessions::ActiveModel {
            id: Set(session_id),
            is_revoked: Set(true),
            revoked_at: Set(Some(Utc::now().into())),
            ..Default::default()
        };

        session.update(db).await?;
        Ok(())
    }

    
    
}