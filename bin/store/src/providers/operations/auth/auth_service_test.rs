#[cfg(test)]
mod tests {
    use crate::providers::operations::auth::auth_service::{password_hash, password_verify};

    /// Tests that password hashing produces consistent bcrypt format but different salts
    /// This ensures proper security through unique salt generation for each hash
    #[tokio::test]
    async fn should_generate_different_hashes_for_same_password_with_consistent_format() {
        println!("Testing password hashing consistency with different salts");

        let password = "test_password";
        let hash1 = password_hash(password).await.unwrap();
        let hash2 = password_hash(password).await.unwrap();

        println!("Generated hash1: {}", hash1);
        println!("Generated hash2: {}", hash2);

        // Hashes should be different due to salt
        assert_ne!(hash1, hash2);

        // But both should verify correctly
        assert!(password_verify(&hash1, password).await.unwrap());
        assert!(password_verify(&hash2, password).await.unwrap());

        println!("Password hashing consistency test passed");
    }

    /// Tests that different passwords produce different hashes and cross-verification fails
    /// This ensures password uniqueness and prevents hash collision vulnerabilities
    #[tokio::test]
    async fn should_produce_different_hashes_for_different_passwords() {
        println!("Testing different passwords produce different hashes");

        let password1 = "password1";
        let password2 = "password2";

        let hash1 = password_hash(password1).await.unwrap();
        let hash2 = password_hash(password2).await.unwrap();

        println!("Hash for password1: {}", hash1);
        println!("Hash for password2: {}", hash2);

        // Different passwords should produce different hashes
        assert_ne!(hash1, hash2);

        // Cross-verification should fail
        assert!(!password_verify(&hash2, password1).await.unwrap());
        assert!(!password_verify(&hash1, password2).await.unwrap());

        println!("Different passwords test passed");
    }

    /// Tests that empty string passwords can be hashed and verified correctly
    /// This ensures edge case handling for empty password inputs
    #[tokio::test]
    async fn should_handle_empty_string_passwords_correctly() {
        println!("Testing empty string password handling");

        let password = "";
        let hash = password_hash(password).await.unwrap();

        println!("Hash for empty password: {}", hash);

        assert!(password_verify(&hash, password).await.unwrap());
        assert!(!password_verify(&hash, "not_empty").await.unwrap());

        println!("Empty string password test passed");
    }

    /// Tests that passwords with special characters are handled correctly
    /// This ensures proper encoding and security for complex passwords
    #[tokio::test]
    async fn should_handle_special_characters_in_passwords() {
        println!("Testing special characters in passwords");

        let password = "p@ssw0rd!#$%^&*()";
        let hash = password_hash(password).await.unwrap();

        println!("Hash for special character password: {}", hash);

        assert!(password_verify(&hash, password).await.unwrap());
        assert!(!password_verify(&hash, "different").await.unwrap());

        println!("Special characters password test passed");
    }

    /// Tests that Unicode passwords are handled correctly across different character sets
    /// This ensures international character support and proper UTF-8 encoding
    #[tokio::test]
    async fn should_handle_unicode_passwords_correctly() {
        println!("Testing Unicode password handling");

        let password = "пароль密码パスワード";
        let hash = password_hash(password).await.unwrap();

        println!("Hash for Unicode password: {}", hash);

        assert!(password_verify(&hash, password).await.unwrap());
        assert!(!password_verify(&hash, "different").await.unwrap());

        println!("Unicode password test passed");
    }
}
