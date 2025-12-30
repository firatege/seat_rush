pub mod auth;
mod payment;
mod tickets;
mod user;

use actix_web::web::{scope, ServiceConfig};


pub fn configure(cfg: &mut ServiceConfig) {
    cfg.service(
        scope("/v1")
        .configure(auth::configure)
            .configure(tickets::configure)
    );
}
