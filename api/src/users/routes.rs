use axum::{
    Router, 
    routing::{
        delete, get, post
    }, 
    middleware
};
use std::sync::Arc;

use crate::{
    auth::middleware::auth_middleware,
    common::AppState,
    users::handlers::{
        create_user,
        delete_user,
        list_users,
        login_user, 
        validate_token
    }
};

pub fn login_route(state: Arc<AppState>) -> Router {
    Router::new()
        .route("/login", post(login_user))
        .with_state(state)
}

pub fn user_routes(state: Arc<AppState>) -> Router {
    Router::new()
        .route("/check", get(validate_token))
        .route("/register", post(create_user))
        .route("/delete", delete(delete_user))
        .route("/get", get(list_users))
        .route_layer(middleware::from_fn(auth_middleware))
        .with_state(state)
}
