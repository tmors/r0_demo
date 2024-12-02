#![no_main]

use risc0_zkvm::guest::env;
use jwt_core::Validator;
use serde::{Deserialize, Serialize};
use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine};
use thiserror::Error;
risc0_zkvm::guest::entry!(main);

#[derive(Debug, Serialize, Deserialize, Clone)]
struct GoogleClaims {
    iss: String,           // Issuer
    aud: String,           // Audience
    sub: String,           // Subject
    email: String,         // Email
    name: String,          // Name
    iat: u64,              // Issued at
    exp: u64,              // Expiration time
}

#[derive(Error, Debug)]
pub enum JwtError {
    #[error("Invalid JWT format")]
    Format,
    #[error("Base64 decode error: {0}")]
    Base64(#[from] base64::DecodeError),
    #[error("JSON decode error: {0}")]
    Json(#[from] serde_json::Error),
}

pub fn parse_jwt_claims(token: &str) -> Result<(GoogleClaims, String), JwtError> {
    // Split the JWT into parts
    let parts: Vec<&str> = token.split('.').collect();
    if parts.len() != 3 {
        return Err(JwtError::Format);
    }

    // Decode the payload (second part)
    let payload = URL_SAFE_NO_PAD.decode(parts[1])?;
    let payload_str = String::from_utf8_lossy(&payload);
    println!("{}", payload_str);
    // Parse into Claims struct
    let claims = serde_json::from_str(&payload_str)?;

    Ok((claims, payload_str.into_owned()))
}

fn main() {
    println!("this is guest zklogin");
    let jwt: String = env::read();
    let pubkey: String = env::read();

    // 1. check sign is right
    // 2. check exp is not expired
    // 3. check publicWtns is right
    let validator = pubkey.parse::<Validator>().unwrap();
    let valid_token = validator.validate_token_integrity(&jwt).unwrap();
    let (claims, payload_str) = parse_jwt_claims(jwt.as_str())
        .expect("failed to parse JWT claims");
    env::commit(&(claims.iss, claims.iat, claims.exp));
}