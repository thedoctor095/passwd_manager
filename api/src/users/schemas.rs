use serde::Deserialize;
use validator::Validate;

use crate::common::password_validator;

#[derive(Deserialize, Debug, Validate)]
pub struct UserCreate {
    #[validate(length(min = 5))]
    pub username: String,
    #[validate(email)]
    pub email: String,
    #[validate(custom(function = "password_validator"))]
    pub password: String
}

#[derive(Deserialize, Validate)]
pub struct UserLogin {
    #[validate(length(min = 5))]
    pub username: String,
    #[validate(custom(function = "password_validator"))]
    pub password: String
}
