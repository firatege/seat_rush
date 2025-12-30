use actix_web::{HttpRequest, HttpResponse, web::{Data}, http::StatusCode};
use crate::domain::dtos::auth::AuthResponse;
use crate::domain::models::calims::Claims;
use crate::infrastructure::state::AppState;
use crate::repository::auth::SessionRepo;
use crate::utils::error::GatewayError;
use crate::utils::jwt::create_access_token;
use crate::utils::responser::BodyBuilder;

pub async fn refresh(
    state: Data<AppState>,
    req: HttpRequest,  
) -> Result<HttpResponse, GatewayError> {

    let refresh_token_cookie = req.cookie("refresh_token")
        .ok_or(GatewayError::Auth("Refresh token not found in cookies".into()))?
        .value()
        .to_string();

    let session = SessionRepo::find_by_hash(&state.db, &refresh_token_cookie).await?
        .ok_or(GatewayError::Auth("Session not found".into()))?;

    if session.expires_at < chrono::Utc::now() {
        return Err(GatewayError::Auth("Refresh token expired".into()));
    }

    let new_access = create_access_token(&state.jwt_secret, Claims {
        sub: session.user_id.to_string(),
        iat: chrono::Utc::now().timestamp(),
        exp: (chrono::Utc::now() + chrono::Duration::minutes(state.environment.access_token_expiration)).timestamp(),
    })?;

    Ok(BodyBuilder::from("Refresh Success")
        .status(StatusCode::OK)
        .add_data(AuthResponse { access_token: new_access })
        .into()
    )
}
