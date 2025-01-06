use actix_web::{post, web, HttpResponse, Result};
use serde_json::json;
use validator::Validate;
use crate::schemas::auth::login_schema::LoginSchema;
use crate::utils::validation_error::validation_error;

#[post("/login")]
pub async fn login_handler(
    payload: Result<web::Json<LoginSchema>, actix_web::Error>
) -> HttpResponse {
    match payload {
        Ok(payload) => {
            // Validasi payload
            if let Err(errors) = payload.validate() {
                let error_map = validation_error(errors);

                return HttpResponse::BadRequest().json(json!({
                    "message": "Validation failed",
                    "errors": error_map
                }));
            }

            // Jika validasi berhasil
            HttpResponse::Ok().json(json!({
                "message": "Login successful",
                "username": payload.username
            }))
        }
        Err(e) => {
            // Tangani kesalahan deserialisasi JSON
            HttpResponse::BadRequest().json(json!({
                "message": "Invalid JSON payload",
                "error": e.to_string()
            }))
        }
    }
}