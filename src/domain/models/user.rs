use actix_http::Payload;
use actix_web::{FromRequest, HttpRequest};
use actix_web::web::Data;
use futures_core::future::LocalBoxFuture;
use log::debug;
use sea_orm::EntityTrait;
use uuid::Uuid;
use crate::domain::entities::{user};
use crate::domain::entities::user::UserStatus;
use crate::infrastructure::state::AppState;
use crate::utils::error::GatewayError;
use crate::utils::jwt::verify_jwt;
use crate::utils::result::GatewayResult;

#[derive(Debug, Clone,)]
pub struct RequestUser {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    pub status: UserStatus,
}



impl From<user::Model> for RequestUser {
    fn from(model: user::Model) -> Self {
        Self {
            id: model.id,
            username: model.username,
            email: model.email,
            status: model.status,
        }
    }
}

impl FromRequest for RequestUser {
    type Error = GatewayError;
    type Future = LocalBoxFuture<'static, GatewayResult<Self>>;

    fn from_request(req: &HttpRequest, _payload: &mut Payload) -> Self::Future {
        let auth_header = req.headers().get("Authorization")
            .cloned();
        let app_state = req.app_data::<Data<AppState>>().cloned();

        Box::pin(async move {

            let auth_header_value = auth_header
                .ok_or(GatewayError::Auth("Authorization header missing".into()))?;
            let auth_header = auth_header_value
                .to_str()
                .map_err(|_| GatewayError::Auth("Authorization header malformed".into()))?;

            if !auth_header.starts_with("Bearer ") {
                return Err(GatewayError::Auth("Invalid auth scheme".into()));
            }

            let token = auth_header.trim_start_matches("Bearer ").trim();

       
            let state = app_state
                .ok_or(GatewayError::Internal("AppState missing".into()))?;
            let claims = verify_jwt(token, &state.jwt_secret)?;
            debug!("Claims {:?}", claims);


            let user_id = Uuid::parse_str(&claims.sub)
                .map_err(|_| GatewayError::Auth("Invalid token subject".into()))?;

            let user_model = user::Entity::find_by_id(user_id)
                .one(&state.db)
                .await?
                .ok_or(GatewayError::Auth("User not found".into()))?;

            Ok(RequestUser::from(user_model))
        })
    }
}
