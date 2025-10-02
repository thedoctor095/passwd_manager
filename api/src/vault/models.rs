use serde::Serialize;
use sqlx::FromRow;

#[derive(Debug, Serialize, FromRow)]
pub struct EntryOut {
    pub id: i32,
    pub domain: String,
    pub email: Option<String>,
    pub passwd: String,
    pub username: Option<String>,
    pub comment: Option<String> 
}