use anyhow::Result;
use axum::{
    extract::{Path, State}, 
    http::StatusCode,
    Extension,
    Json
};
use validator::Validate;
use std::sync::Arc;

use crate::{
    common::{
        AppState,
        UserIdentifier
    },
    vault::{
        models::EntryOut,
        schemas::EntryCreate,
        services::{
            create_vault_entry,
            delete_vault_entry,
            read_vault_entries
        }
    }
};

pub async fn create_entry(
    Extension(user_id): Extension<UserIdentifier>,
    State(shared_state): State<Arc<AppState>>,
    Json(payload): Json<EntryCreate>
) -> Result<StatusCode, StatusCode> {
    payload
        .validate()
        .map_err(|e| {
            tracing::error!("Vault entry validation error: {e}");
            StatusCode::FORBIDDEN
        })?;
    create_vault_entry(&user_id.id, &payload, &shared_state.mysql_pool)
        .await
        .map_err(|e| {
            tracing::error!("Could not create vault secret: {e}");
            StatusCode::INTERNAL_SERVER_ERROR
        })?;
    Ok(StatusCode::CREATED)
}

pub async fn delete_entry(
    Path(entry_id): Path<u32>,
    Extension(user_id): Extension<UserIdentifier>,
    State(shared_state): State<Arc<AppState>>
)  -> Result<StatusCode, StatusCode> {
    user_id
        .validate()
        .map_err(|e| {
            tracing::error!("UserIdentifier validation error: {e}");
            StatusCode::FORBIDDEN
        })?;
    delete_vault_entry(&entry_id, &user_id.id, &shared_state.mysql_pool)
        .await
        .map_err(|e| {
            tracing::error!("Could not delete vault entry {e}");
            StatusCode::INTERNAL_SERVER_ERROR
        })?;
    Ok(StatusCode::NO_CONTENT)
}

pub async fn read_entries(
    Extension(user_id): Extension<UserIdentifier>,
    State(shared_state): State<Arc<AppState>>,
) -> Result<(StatusCode, Json<Vec<EntryOut>>), StatusCode> {
    user_id
        .validate()
        .map_err(|e| {
            tracing::error!("UserIdentifier validation error: {e}");
            StatusCode::FORBIDDEN
        })?;
    let vault_entries = read_vault_entries(&user_id.id, &shared_state.mysql_pool)
        .await
        .map_err(|e| {
            tracing::error!("Could not find vault entries for {} {e}", user_id.id);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;
    Ok((
        StatusCode::OK,
        Json(vault_entries)
    ))
}