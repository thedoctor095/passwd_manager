use anyhow::Result;
use sqlx::{
    mysql::{
        MySqlPool,
        MySqlPoolOptions
    }
};

pub async fn mysql_init(db_uri: &str, db_max_pool_size: u32) -> Result<MySqlPool> {
    let pool = MySqlPoolOptions::new()
        .max_connections(db_max_pool_size)
        .connect(db_uri)
        .await?;
    Ok(pool)
}
