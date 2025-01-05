use actix_web::web;
use crate::routes::auth::{login, register};

pub fn init(cfg: &mut web::ServiceConfig) {
    login::config(cfg);
    register::config(cfg);
}
