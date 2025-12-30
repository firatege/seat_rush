mod infrastructure;
mod api;
mod domain;
mod repository;
mod service;
mod utils;
mod extension;

use actix_web::{web, App, HttpServer};
use dotenv::dotenv;
use std::env;
use infrastructure::db::establish_connection; // connection fonksiyonun adı db.rs'te neyse onu kullan
use infrastructure::state::AppState;

#[actix_web::main]
// 1. DÜZELTME: Fonksiyon artık bir Result döndürüyor
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
    let redis_url = env::var("REDIS_URL").expect("REDIS_URL must be set");
    let jwt_secret = env::var("SECRET_KEY").expect("JWT_SECRET must be set");
    let lax_domain = env::var("LAX_DOMAIN").unwrap_or_else(|_| "localhost".to_string());
    let access_token_expiration = env::var("ACCESS_TOKEN_EXPIRATION").expect("ACCESS_TOKEN_EXPIRATION must be set").parse::<i64>().unwrap();
    let refresh_token_expiration = env::var("REFRESH_TOKEN_EXPIRATION").expect("REFRESH_TOKEN_EXPIRATION must be set").parse::<i64>().unwrap();
    log::info!("PostgreSQL connection: {}", &db_url);

    let db = establish_connection(&db_url).await.expect("failed to connect to database");

    log::info!("Redis is located at `{}`", redis_url);
    let redis = redis::Client::open(redis_url).expect("Redis connection error");

    let app_state = AppState {
        db,
        redis,
        jwt_secret,
        environment: infrastructure::state::Environment {
            lax_domain,
            access_token_expiration,
            refresh_token_expiration
        },
    };

    log::info!("Starting server at 127.0.0.1:8080");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(app_state.clone()))
         .configure(api::configure)
    })
        .bind(("127.0.0.1", 8080))? // 2. ARTIK ÇALIŞIR: Hata olursa main'den yukarı fırlatır
        .run()
        .await
}