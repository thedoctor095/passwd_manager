use argon2::{
    password_hash::{
        rand_core::OsRng, 
        SaltString
    }, 
    Argon2, 
    PasswordHash, 
    PasswordHasher,
    PasswordVerifier
};
use anyhow::{
    anyhow,
    Result
};

pub fn hash_password(password: &str) -> Result<String> {
    let salt = SaltString::generate(&mut OsRng);
    let password = Argon2::default()
        .hash_password(password.as_bytes(), &salt)
        .map_err(|e| anyhow!(e))?
        .to_string();
    Ok(password)
}

pub fn verify_password(
    password: &str, hash: &str
) -> Result<bool> {
    let parsed_hash = PasswordHash::new(hash)
    .map_err(|e| anyhow!(e))?;
    Ok(
        Argon2::default()
            .verify_password(&password.as_bytes(), &parsed_hash)
            .is_ok()
    )
}