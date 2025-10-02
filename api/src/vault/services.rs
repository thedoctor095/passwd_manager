use anyhow::Result;
use sqlx::MySqlPool;

use crate::vault::{models::EntryOut, schemas::EntryCreate};


pub async fn create_vault_entry(
    uuid: &str,
    payload: &EntryCreate,
    db_pool: &MySqlPool
) -> Result<bool> {
    sqlx::query!(
        "
        INSERT INTO vault (uuid, domain, email, passwd, username, comment)
        VALUES( ?, ?, ?, ?, ?, ?)
        ",
        uuid, payload.domain, payload.email, payload.password, payload.username, payload.comment
    )
        .execute(db_pool)
        .await?;
    Ok(true)
}

pub async fn read_vault_entries(
    user_id: &str,
    db_pool: &MySqlPool
) -> Result<Vec<EntryOut>> {
    let entries = sqlx::query_as!(
        EntryOut,
        "
        SELECT id, domain, email, passwd, username, comment FROM vault
        WHERE uuid=(?)
        ",
        user_id
    )
        .fetch_all(db_pool)
        .await?;
    Ok(entries)
}

pub async fn delete_vault_entry(
    entry_id: &u32,
    user_id: &str,
    db_pool: &MySqlPool
) -> Result<bool> {
    sqlx::query!(
        "
        DELETE FROM vault
        WHERE id=(?) AND uuid=(?)
        ",
        entry_id, user_id
    )
        .execute(db_pool)
        .await?;
    Ok(true)
}