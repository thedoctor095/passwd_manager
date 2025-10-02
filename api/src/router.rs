use axum::{
    http::{
        header,
        Method
    },
    Router
};
use std::sync::Arc;
use tower_http::{
    cors::{
        AllowOrigin, CorsLayer
    },
    trace::{
        DefaultMakeSpan, DefaultOnResponse, TraceLayer
    },
    validate_request::ValidateRequestHeaderLayer};
use tracing::Level;

use crate::{
    common::AppState,
    users::routes::{
        login_route,
        user_routes
    },
    vault::routes::vault_routes
};

pub fn init_router(state: Arc<AppState>) -> Router {
    Router::new()
        .merge(login_route(state.clone()))
        .nest("/user", user_routes(state.clone()))
        .nest("/vault", vault_routes(state.clone()))
        .layer(ValidateRequestHeaderLayer::accept("application/json"))
        .layer(cors(&state.cors_origin))
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(DefaultMakeSpan::new()
                    .level(Level::DEBUG))
                .on_response(DefaultOnResponse::new()
                    .level(Level::DEBUG))
        )
}

pub fn cors(allowed_origin: &String) -> CorsLayer {
    let origin = AllowOrigin::exact(
        allowed_origin.parse().unwrap()
    );
    CorsLayer::new()
        .allow_origin(origin)
        .allow_methods([Method::DELETE, Method::GET, Method::POST])
        .allow_headers([header::AUTHORIZATION, header::ACCEPT, header::CONTENT_TYPE])
        .allow_credentials(true)
}