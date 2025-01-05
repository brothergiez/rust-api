use actix_web::{post, web, HttpResponse};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct RegisterRequest {
    pub username: String,
    // pub password: String,
    // pub email: String,
}

#[post("/register")]
pub async fn register_handler(form: web::Json<RegisterRequest>) -> HttpResponse {
    HttpResponse::Ok().body(format!("User {} registered successfully!", form.username))
}
