use axum::{
    Router, routing::{delete, get, post}, middleware
};
use std::sync::Arc;
use crate::{
    auth::middleware::auth_middleware,
    common::AppState,
    vault::handlers::{
        create_entry,
        delete_entry,
        read_entries
    }
};

pub fn vault_routes(state: Arc<AppState>) -> Router {
    Router::new()
        .route("/create", post(create_entry))
        .route("/delete/{entry_id}", delete(delete_entry))
        .route("/get", get(read_entries))
        .route_layer(middleware::from_fn(auth_middleware))
        .with_state(state)
}