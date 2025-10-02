use axum::response::{
    IntoResponse,
    Json,
    Response
};

use serde::{Deserialize, Serialize};
use sqlx::MySqlPool;
use validator::{Validate, ValidationError};

#[derive(Debug, Clone)]
pub struct AppState {
    pub mysql_pool: MySqlPool,
    pub cors_origin: String
}

#[derive(Serialize)]
pub struct APIAnswer {
    pub message: String
}

impl IntoResponse for APIAnswer {
    fn into_response(self) -> Response {
        Json(self).into_response()
    }
}

#[derive(Clone, Debug, Deserialize, Validate)]
pub struct UserIdentifier {
    #[validate(length(min = 16))]
    pub id: String,
}

pub fn get_from_env(variable: &str) -> String {
    std::env::var(variable).map_err(
        |_| format!("{variable}")
    ).expect("Configuration error, missing env var")
}

pub fn password_validator(password: &str) -> Result<(), ValidationError> {
    // check ASCII
    if password
        .chars()
        .any(|elem| !elem.is_ascii()) {
            return Err(
                ValidationError::new(
                    r"Password must contain only lowercase and uppercase letters (a-z), digits (0-9), 
                    and special characters from this set: !\#$%&'()*+,-.\/:;<=>?@[\\]^_\{|}~`.
                    Spaces and other characters are not allowed."
            ))
        }
    if password.len() < 12 {
        return Err(
            ValidationError::new("Password must contain at least 12 characters.")
        )
    }
    // check digits
    if password
        .chars()
        .filter(|elem| elem.is_ascii_digit())
        .count() < 2 {
            return Err(ValidationError::new("Password must contain at least 2 digits."))
    }
    // check ASCII uppercase
    if password
        .chars()
        .filter(|elem| elem.is_ascii_uppercase())
        .count() < 2 {
            return Err(
                ValidationError::new("Password must contain at least 2 uppercase characters.")
            )
        }
    // check ASCII lowercase
    if password
        .chars()
        .filter(|elem| elem.is_ascii_lowercase())
        .count() < 2 {
            return Err(
                ValidationError::new("Password must contain at least 2 lowercase characters.")
            )
        }
    // check special chars
    if password
        .chars()
        .filter(|elem| r"!\#$%&'()*+,-.\/:;<=>?@[\\]^_\{|}~".contains(*elem))
        .count() < 1{
            return Err(
                ValidationError::new(r"Password must contain at least on special character: !\#$%&'()*+,-.\/:;<=>?@[\\]^_\{|}~")
            )
        }
    Ok(())
}


