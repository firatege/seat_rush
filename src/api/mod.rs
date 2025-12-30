use std::thread::scope;
use actix_web::web::{service, ServiceConfig};
use actix_web::{get, HttpResponse};
pub mod v1;

pub fn configure(cfg: &mut ServiceConfig) {
    cfg.configure(v1::configure)
        .service(health);
    
}

#[get("/health")]
pub async fn health() -> HttpResponse {
    HttpResponse::Ok().body("Ok")
}


