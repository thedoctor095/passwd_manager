use anyhow::Result;
use sqlx::MySqlPool;
use uuid::Uuid;

use crate::users::{
    models::{
        UserLoginOut,
        UserOut
    },
    schemas::UserCreate
};

pub async fn create_db_user(
    payload: &UserCreate,
    db_pool: &MySqlPool
) -> Result<bool> {

    let user_id = Uuid::new_v4().to_string();

    sqlx::query!(
        "
        INSERT INTO users (id, username, email, hashed_password) 
        VALUES( ?, ?, ?, ? )",
        user_id, payload.username, payload.email, payload.password
    )
        .execute(db_pool)
        .await?;
    Ok(true)
}

pub async fn read_db_user(
    username: &str,
    db_pool: &MySqlPool
) -> Result<UserLoginOut>{
    let user_info = sqlx::query_as!(
        UserLoginOut,
        "
        SELECT id, hashed_password FROM users 
        WHERE username=(?)",
        username
    )
        .fetch_one(db_pool)
        .await?;


    Ok(user_info)
}

pub async fn delete_db_user(
    user_id: &str, 
    db_pool: &MySqlPool
) -> Result<bool> {
    sqlx::query!(
        "
        DELETE FROM users 
        WHERE id=(?)
        ",
        user_id
    )
        .execute(db_pool)
        .await?;
    Ok(true)
}

pub async fn list_db_users(
    db_pool: &MySqlPool
) -> Result<Vec<UserOut>> {
    let users = sqlx::query_as!(
        UserOut,
        "
        SELECT id, username, email, created_at, modified_at
        FROM users
        "
    )
        .fetch_all(db_pool)
        .await?;
    Ok(users)
}
