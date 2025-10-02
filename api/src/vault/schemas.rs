use serde::Deserialize;
use validator::Validate;

#[derive(Debug, Deserialize, Validate)]
pub struct EntryCreate {
    pub domain: String,
    pub email: Option<String>,
    pub password: String,
    pub username: Option<String>,
    pub comment: Option<String> 
}
