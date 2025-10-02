mod auth;
mod common;
mod config;
mod db;
mod router;
mod users;
mod vault;

use axum::serve;
use std::sync::Arc;
use tokio::net::TcpListener;

use crate::{
    common::AppState,
    config::{
        init_env,
        Configuration
    },
    db::mysql_init,
    router::init_router
};


pub async fn run() {
    init_env();

    let cfg = Configuration::new();
    let mysql_db = mysql_init(&cfg.db_uri, cfg.db_max_pool_size)
        .await
        .expect("Could not load mysql db");
    let shared_state = Arc::new(
        AppState { 
            mysql_pool: mysql_db,
            cors_origin: cfg.cors_origin
        }
    );
    let app = init_router(shared_state);

    let listener = TcpListener::bind(&cfg.app_addr)
        .await
        .expect("Could not load TCP listener");
    
    tracing::info!("Serving on {}", cfg.app_addr);

    serve(listener, app)
        .await
        .expect("Could not initalize app");
}
