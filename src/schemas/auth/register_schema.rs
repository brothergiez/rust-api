use serde::Deserialize;
use validator::Validate;

#[derive(Deserialize, Validate)]
pub struct RegisterSchema {
    #[validate(length(min = 3, message = "Email must be at least 3 characters long"))]
    pub email: String,

    #[validate(length(min = 3, message = "Username must be at least 3 characters long"))]
    pub username: String,

    #[validate(length(min = 8, message = "Password must be at least 8 characters long"))]
    pub password: String,
}
