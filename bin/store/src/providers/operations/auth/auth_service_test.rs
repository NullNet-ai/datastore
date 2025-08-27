use super::auth_service::{password_hash, password_verify};

#[tokio::test]
async fn test_password_hash_consistency() {
    let password = "test_password";
    let hash1 = password_hash(password).await.unwrap();
    let hash2 = password_hash(password).await.unwrap();
    
    // Hashes should be different due to salt
    assert_ne!(hash1, hash2);
    
    // But both should verify correctly
    assert!(password_verify(&hash1, password).await.unwrap());
    assert!(password_verify(&hash2, password).await.unwrap());
}

#[tokio::test]
async fn test_password_hash_different_passwords() {
    let password1 = "password1";
    let password2 = "password2";
    
    let hash1 = password_hash(password1).await.unwrap();
    let hash2 = password_hash(password2).await.unwrap();
    
    // Different passwords should produce different hashes
    assert_ne!(hash1, hash2);
    
    // Cross-verification should fail
    assert!(!password_verify(&hash2, password1).await.unwrap());
    assert!(!password_verify(&hash1, password2).await.unwrap());
}

#[tokio::test]
async fn test_password_hash_empty_string() {
    let password = "";
    let hash = password_hash(password).await.unwrap();
    
    assert!(password_verify(&hash, password).await.unwrap());
    assert!(!password_verify(&hash, "not_empty").await.unwrap());
}

#[tokio::test]
async fn test_password_hash_special_characters() {
    let password = "p@ssw0rd!#$%^&*()";
    let hash = password_hash(password).await.unwrap();
    
    assert!(password_verify(&hash, password).await.unwrap());
    assert!(!password_verify(&hash, "different").await.unwrap());
}

#[tokio::test]
async fn test_password_hash_unicode() {
    let password = "пароль密码パスワード";
    let hash = password_hash(password).await.unwrap();
    
    assert!(password_verify(&hash, password).await.unwrap());
    assert!(!password_verify(&hash, "different").await.unwrap());
}