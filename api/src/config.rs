use std::net::SocketAddr;
use tracing_subscriber::{prelude::*, EnvFilter};

use crate::common::get_from_env;

#[derive(Clone, Debug)]
pub struct Configuration {
    pub app_addr: SocketAddr,
    pub db_uri: String,
    pub db_max_pool_size: u32,
    pub cors_origin: String
}

impl Configuration {
    pub fn new() -> Self {
        let listen_addr = get_from_env("LISTEN_ADDR")
            .parse::<String>()
            .expect("Could not parse app listen address");
        let listen_port = get_from_env("LISTEN_PORT")
            .parse::<String>()
            .expect("Could not parse app listen port");
        let app_addr = format!("{}:{}", listen_addr, listen_port)
        .parse::<SocketAddr>().expect("Could not parse app IP address and port");

        let db_uri = get_from_env("DATABASE_URL");
        let db_max_pool_size = get_from_env("DATABASE_MAX_POOL")
        .parse::<u32>().expect("Could not parse mysql max size value");

        let cors_origin = get_from_env("ALLOWED_ORIGIN");

        Self {
            app_addr: app_addr,
            db_uri: db_uri, 
            db_max_pool_size: db_max_pool_size,
            cors_origin: cors_origin
        }

    }
}


pub fn init_env() {
    dotenvy::dotenv().ok();
    let fmt_layer = tracing_subscriber::fmt::layer().compact();
    let filter_layer = EnvFilter::try_from_default_env()
        .or_else(|_| EnvFilter::try_new("debug"))
        .expect("Could not initialize tracing filter layer");
    tracing_subscriber::registry()
        .with(filter_layer)
        .with(fmt_layer)
        .init();
}