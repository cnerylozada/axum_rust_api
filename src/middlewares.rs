use std::env;

use axum::http::HeaderMap;
use jsonwebtoken::{DecodingKey, Validation, decode};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Claims {
    sub: String,
    exp: usize,
}

fn verify_jwt(token: &str) -> Result<Claims, String> {
    let secret = env::var("AUTH_SECRET").expect("AUTH_SECRET env is not found");

    let result = decode::<Claims>(
        &token,
        &DecodingKey::from_secret(secret.as_ref()),
        &Validation::default(),
    )
    .map_err(|error| error.to_string())?;

    Ok(result.claims)
}

pub fn manage_authentication(headers: HeaderMap) -> Result<Claims, String> {
    let auth_header = headers
        .get("Authorization")
        .and_then(|h| h.to_str().ok())
        .ok_or("Missing headers".to_string())?;

    let token = auth_header.trim_start_matches("Bearer ").trim();
    verify_jwt(token)
}
