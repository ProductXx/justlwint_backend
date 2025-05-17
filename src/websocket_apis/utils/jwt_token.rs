use axum::http::{HeaderMap, HeaderValue};
use jsonwebtoken::{Algorithm, DecodingKey, Validation, decode};

use crate::structures::{auth_structures::login_structures::Claims, static_vars::JWT_SECRET};

pub fn extract_token(headers: &HeaderMap<HeaderValue>) -> Result<Claims, String> {
    if let Some(auth_header) = headers.get("Authorization") {
        if let Ok(auth_str) = auth_header.to_str() {
            if auth_str.starts_with("Bearer ") {
                let token = auth_str.trim_start_matches("Bearer ").to_string();

                match decode::<Claims>(
                    &token,
                    &DecodingKey::from_secret(JWT_SECRET.as_bytes()),
                    &Validation::new(Algorithm::HS256),
                ) {
                    Ok(decoded) => {
                        return Ok(decoded.claims);
                    }
                    Err(_) => {
                        return Err("Invalid JWT".to_string());
                    }
                }
            }
            return Err("Invalid Authorization Token header format?".to_string());
        }
    }

    Err("No Authorization Token header found!".to_string())
}
