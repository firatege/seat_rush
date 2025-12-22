use actix_web::web::{scope, ServiceConfig};


pub fn configure(cfg: &mut ServiceConfig) {
    cfg.service(
        scope("/v1")
    );
}
