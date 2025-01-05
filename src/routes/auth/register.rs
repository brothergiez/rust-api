use actix_web::web;
use crate::handlers::register_handler::register_handler;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(register_handler);
}