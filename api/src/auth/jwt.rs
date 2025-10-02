use anyhow::Result;
use chrono::{
    Duration,
    Utc
};
use jsonwebtoken::{
    decode, 
    encode,
    Algorithm,
    DecodingKey,
    EncodingKey,
    Header,
    Validation
};
use serde::{
    Deserialize,
    Serialize
};
use std::sync::LazyLock;

use crate::common::{
    get_from_env,
    UserIdentifier
};

// only this file should deal with JWT related data
// https://doc.rust-lang.org/std/sync/struct.LazyLock.html
// Note: static items do not call [`Drop`] on program termination, so this won't be deallocated.
// this is fine, as the OS can deallocate the terminated program faster than we can free memory
// but tools like valgrind might report "memory leaks" as it isn't obvious this is intentional
static SECRET: LazyLock<String> = LazyLock::new(|| get_from_env("JWT_SECRET"));
static JWT_DURATION: LazyLock<i64> = LazyLock::new(|| get_from_env("JWT_DURATION").parse().expect("Could not parse JWT_DURATION"));

#[derive(Serialize, Deserialize, Debug)]
pub struct Claims {
    pub exp: usize,
    pub iat: usize,
    pub tag: String
}

pub fn encode_jwt(identifier: String) -> Result<String> {
    let now = Utc::now();
    let expire = Duration::minutes(*JWT_DURATION);
    let exp: usize = (now + expire).timestamp() as usize;
    let iat: usize = now.timestamp() as usize;
    let claim = Claims { 
        exp: exp, 
        iat: iat, 
        tag: identifier 
    };
    let token = encode(
        &Header::new(Algorithm::HS512),
        &claim,
        &EncodingKey::from_secret(SECRET.as_ref()),
    )?.to_string();
    Ok(token)
}

pub fn decode_jwt(token: &str) -> Option<UserIdentifier> {
    match decode::<Claims>(
        token, 
        &DecodingKey::from_secret(SECRET.as_ref()), 
        &Validation::new(Algorithm::HS512)
    ) {
        Ok(result) => {
            tracing::info!("Validated token for user {}", result.claims.tag);
            Some(UserIdentifier { id: result.claims.tag })
        },
        Err(_) => {
            tracing::error!("Could not validate token {token}");
            None
        }
    }
}