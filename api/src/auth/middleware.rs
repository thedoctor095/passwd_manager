use axum::{
    extract::Request,
    http::{
        HeaderMap,
        StatusCode
    },
    middleware::Next,
    response::Response
};
use tokio::task;

use crate::auth::jwt::decode_jwt;

pub async fn auth_middleware(
    headers: HeaderMap, 
    mut request: Request, 
    next: Next
) -> Result<Response, StatusCode> {
    match get_token(headers) {
        Some(token) => {
            let user_id = task::spawn_blocking(move || {
                decode_jwt(&token)
            })
                .await
                .map_err(|e| {
                    tracing::error!(
                        "Could not retrieve user id thread result: {e}"
                    );
                        StatusCode::INTERNAL_SERVER_ERROR
                })?;
            match user_id {
                Some(value) => {
                    request.extensions_mut().insert(value);
                    let response = next.run(request).await;
                    Ok(response)
                },
                _ => Err(StatusCode::FORBIDDEN)
            }

        },
        _ => Err(StatusCode::FORBIDDEN)
    }
}

fn get_token(headers: HeaderMap) -> Option<String> {
    match headers.get("Authorization") {
        Some(token) => {
            match token.to_str() {
                Ok(value) => Some(value.to_string()),
                Err(_) => None
            }
        },
        None => None
    }
}

