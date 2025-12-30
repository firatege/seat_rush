use actix_web::web;
use actix_web::web::scope;

pub mod register;
pub mod login;


pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        scope("/auth")
            .service(register::register)
            .service(login::login)
    );}