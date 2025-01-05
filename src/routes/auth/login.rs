use actix_web::web;
use crate::handlers::login_handler::login_handler;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(login_handler);
}