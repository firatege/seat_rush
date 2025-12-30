use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use uuid::Uuid;
use crate::domain::models::calims::Claims;
use crate::utils::error::GatewayError;

pub fn verify_jwt(token: &str, secret: &str) -> Result<Claims, GatewayError> {
    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::default(),
    )?;

    Ok(token_data.claims)
}

pub fn create_access_token(secret: &str, payload: Claims) -> Result<String, GatewayError> {
    let token = encode(
        &Header::default(),
        &payload,
        &EncodingKey::from_secret(secret.as_bytes()),
    )?;
    Ok(token)
}

pub fn create_refresh_token() -> String {
    Uuid::new_v4().to_string()
}
