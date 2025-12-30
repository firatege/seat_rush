use actix_web::web;
use actix_web::web::scope;
pub mod buy;
pub  mod rez;
pub mod cancel;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        scope("/tickets")
            .service(buy::buy)
            .service(rez::rez)
            .service(buy::buy)
            .service(cancel::cancel)
    );}