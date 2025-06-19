use crate::organizations::auth_service::Claims;
use actix_web::http::StatusCode;
use argon2::password_hash::{rand_core::OsRng, SaltString};
use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use jsonwebtoken::{Algorithm, DecodingKey, Validation};
use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::env;
use std::io::{Error, ErrorKind};
use std::sync::Mutex;

use crate::controllers::store_controller::ApiError;

// Simple in-memory cache implementation
static TOKEN_CACHE: Lazy<Mutex<HashMap<String, String>>> = Lazy::new(|| Mutex::new(HashMap::new()));

pub async fn password_hash(password: &str) -> Result<String, Box<dyn std::error::Error>> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();

    match argon2.hash_password(password.as_bytes(), &salt) {
        Ok(hash) => Ok(hash.to_string()),
        Err(err) => {
            log::error!("Failed to hash password: {}", err);
            Err(Box::new(Error::new(
                ErrorKind::Other,
                "Failed to hash password",
            )))
        }
    }
}

pub async fn password_verify(hash: &str, password: &str) -> Result<bool, ApiError> {
    let parsed_hash = match PasswordHash::new(hash) {
        Ok(h) => h,
        Err(err) => {
            log::error!("Error parsing hash: {}", err);
            return Err(ApiError::new(
                StatusCode::BAD_REQUEST,
                "Invalid password hash format",
            ));
        }
    };

    let argon2 = Argon2::default();
    match argon2.verify_password(password.as_bytes(), &parsed_hash) {
        Ok(_) => Ok(true),
        Err(_) => {
            // Password did not match
            log::error!("Password did not match");
            Ok(false)
        }
    }
}

pub async fn verify(token: &str) -> Result<Claims, Box<dyn std::error::Error>> {
    // Check cache first
    let cached_token = {
        let cache = TOKEN_CACHE.lock().unwrap();
        cache.get(token).cloned()
    };

    if let Some(cached_data) = cached_token {
        return Ok(serde_json::from_str(&cached_data)?);
    }

    // If not in cache, verify with JWT
    let jwt_secret = env::var("JWT_SECRET").unwrap_or_else(|_| "Ch@ng3m3Pl3@s3!!".to_string());

    // Simplified verification similar to the JavaScript version
    match jsonwebtoken::decode::<Claims>(
        token,
        &DecodingKey::from_secret(jwt_secret.as_bytes()),
        &Validation::new(Algorithm::HS256),
    ) {
        Ok(token_data) => {
            // Cache the result
            let claims_json = serde_json::to_string(&token_data.claims)?;
            let mut cache = TOKEN_CACHE.lock().unwrap();
            cache.insert(token.to_string(), claims_json.clone());

            Ok(token_data.claims)
        }
        Err(err) => {
            log::error!("Token verification failed: {}", err);
            Err(Box::new(err))
        }
    }
}
