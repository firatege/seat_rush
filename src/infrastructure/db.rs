use sea_orm::{Database, DatabaseConnection, DbErr};
use std::time::Duration;
use sea_orm::ConnectOptions;

pub async fn establish_connection(database_url: &str) -> Result<DatabaseConnection, DbErr> {
    let mut opt = ConnectOptions::new(database_url.to_owned());

    // 🔥 Production Ayarları (Önemli)
    opt.max_connections(100)
        .min_connections(5)
        .connect_timeout(Duration::from_secs(8))
        .idle_timeout(Duration::from_secs(8))
        .sqlx_logging(true); // Konsolda SQL sorgularını görmek için

    Database::connect(opt).await
}