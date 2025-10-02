use anyhow::Result;
use axum::{
    extract::State, 
    http::StatusCode,
    response::IntoResponse,
    Extension,
    Json
};
use validator::Validate;
use std::sync::Arc;
use tokio::task;

use crate::{
    auth::{
        argon::{
            hash_password,
            verify_password
        },
        jwt::encode_jwt
    },
    common::{APIAnswer, AppState, UserIdentifier},
    users::{
        models::UserOut,
        schemas::{
            UserCreate,
            UserLogin
        }, services::{
            create_db_user,
            delete_db_user,
            list_db_users,
            read_db_user
        }
    }
};

pub async fn create_user(
    State(shared_state): State<Arc<AppState>>,
    Json(mut payload): Json<UserCreate>
) -> Result<StatusCode, StatusCode> {
    payload
        .validate()
        .map_err(|e| {
            tracing::error!("UserCreate validation error: {e}");
            StatusCode::FORBIDDEN
        })?;
    payload.password = task::spawn_blocking(move || {
        hash_password(&payload.password)
            .map_err(|e| {
                tracing::error!("Could not hash password: {e}");
                StatusCode::INTERNAL_SERVER_ERROR
            })
    })
        .await
        .map_err(
        |e| {
            tracing::error!(
                "Could not retrieve password hash thread result: {e}"
            );
            StatusCode::INTERNAL_SERVER_ERROR
        })??;

    create_db_user(&payload, &shared_state.mysql_pool)
        .await
        .map_err(|e| {
            tracing::error!("Could not create db user: {e}");
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    Ok(StatusCode::CREATED)
}


pub async fn delete_user(
    Extension(user_id): Extension<UserIdentifier>,
    State(shared_state): State<Arc<AppState>>
) -> Result<StatusCode, StatusCode> {
    delete_db_user(&user_id.id, &shared_state.mysql_pool)
        .await
        .map_err(|e| {
            tracing::error!("Could not delete db user {e}");
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    Ok(StatusCode::NO_CONTENT)
}

pub async fn login_user(
    State(shared_state): State<Arc<AppState>>,
    Json(payload): Json<UserLogin>
) -> Result<impl IntoResponse, StatusCode> {

    payload
        .validate()
        .map_err(|e| {
            tracing::error!("UserLogin validation error: {e}");
            StatusCode::FORBIDDEN
        })?;
    let user_creds = read_db_user(&payload.username, &shared_state.mysql_pool)
        .await
        .map_err(|e| {
            tracing::error!("Could not read db user {} {e}", payload.username);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    task::spawn_blocking(move || {
        verify_password(&payload.password, &user_creds.hashed_password)
        .map_err(|e| {
            tracing::error!("Could not validate db user password{e}");
            StatusCode::FORBIDDEN
        })
    })
        .await
        .map_err(
                    |e| {
                tracing::error!(
                    "Could not retrieve password verification thread result: {e}"
                );
                StatusCode::INTERNAL_SERVER_ERROR
        })??;

    let token = task::spawn_blocking(move || {
        encode_jwt(user_creds.id)
        .map_err(|e| {
            tracing::error!("Could not generate jwt token {e}");
            StatusCode::FORBIDDEN
        })
    })
        .await
        .map_err(
        |e| {
            tracing::error!(
                "Could not retrieve jwt generation thread result: {e}"
            );
            StatusCode::INTERNAL_SERVER_ERROR
        })??;

    Ok((
        StatusCode::OK,
        APIAnswer { message: token }
    ))
}

pub async fn list_users(
    State(shared_state): State<Arc<AppState>>
) -> Result<(StatusCode, Json<Vec<UserOut>>), StatusCode> {
    let users = list_db_users(&shared_state.mysql_pool)
    .await
    .map_err(|e| {
        tracing::error!("Could not retrieve users from db {e}");
        StatusCode::INTERNAL_SERVER_ERROR
    })?;
    Ok((
        StatusCode::OK,
        Json(users)
    ))
}

// middleware takes care of validation
pub async fn validate_token(
) -> Result<StatusCode, StatusCode>{
    Ok(StatusCode::OK)
}
