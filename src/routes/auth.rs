use actix_web::web;
use crate::handlers::auth::login_handler::login_handler;
use crate::handlers::auth::register_handler::register_handler;


pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/auth")
        .service(login_handler)
        .service(register_handler));
}