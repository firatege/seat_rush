use sea_orm::DatabaseConnection;
use redis::Client;

#[derive(Clone)]
pub struct AppState {
    pub db: DatabaseConnection,
    pub redis: Client, // Redis istemcisi
    pub jwt_secret: String,
    pub environment: Environment,
}

#[derive(Clone)]
pub struct Environment {
    pub lax_domain: String,
    pub access_token_expiration: i64,
    pub refresh_token_expiration: i64,
}