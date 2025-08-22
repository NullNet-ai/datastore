use sha2::{Digest, Sha256};
pub mod auth_service;
pub mod structs;

pub struct AuthService;
#[allow(warnings)]
impl AuthService {
    pub fn new() -> Self {
        Self {}
    }

    pub async fn password_hash(&self, password: &str) -> String {
        // Simple SHA-256 hash for password
        // In a production environment, you would want to use a proper password hashing algorithm
        // like bcrypt, argon2, or pbkdf2 with salt
        let mut hasher = Sha256::new();
        hasher.update(password.as_bytes());
        hex::encode(hasher.finalize())
    }
}
