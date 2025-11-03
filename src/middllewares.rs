use std::env;

use jsonwebtoken::{DecodingKey, Validation, decode};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Claims {
    sub: String,
    exp: usize,
    role: String,
}

pub fn verify_jwt(token: &str) -> Result<Claims, String> {
    let secret = env::var("AUTH_SECRET").expect("AUTH_SECRET env is not found");

    let result = decode::<Claims>(
        &token,
        &DecodingKey::from_secret(secret.as_ref()),
        &Validation::default(),
    )
    .map_err(|error| error.to_string())?;

    Ok(result.claims)
}
