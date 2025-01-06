use actix_web::web;
use crate::routes::auth;

pub fn init(cfg: &mut web::ServiceConfig) {
    auth::config(cfg);
}
