use chrono::{
    DateTime,
    Utc
};
use serde::Serialize;
use sqlx::FromRow;

#[derive(FromRow)]
pub struct UserLoginOut {
    pub id: String,
    pub hashed_password: String
}

#[derive(FromRow, Serialize)]
pub struct UserOut {
    pub id: String,
    pub username: String,
    pub email: String,
    pub created_at: Option<DateTime<Utc>>,
    pub modified_at: Option<DateTime<Utc>>
}