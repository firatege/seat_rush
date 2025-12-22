use sea_orm::DatabaseConnection;
use redis::Client;

#[derive(Clone)]
pub struct AppState {
    pub db: DatabaseConnection,
    pub redis: Client, // Redis istemcisi
}