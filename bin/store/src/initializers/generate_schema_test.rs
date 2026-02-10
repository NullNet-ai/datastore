use crate::initializers::generate_schema::GenerateSchemaService;
use redis::Client;

#[tokio::test]
async fn test_generate_schema_service_creation() {
    // Create a mock Redis client for testing
    let redis_client = match Client::open("redis://127.0.0.1:6379") {
        Ok(client) => client,
        Err(_) => {
            // Skip test if Redis is not available
            println!("Redis not available, skipping test");
            return;
        }
    };

    let _service = GenerateSchemaService::new(redis_client);

    // Test that the service can be created without panicking
    assert!(true);
}
