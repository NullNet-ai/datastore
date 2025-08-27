#[cfg(test)]
mod tests {
    // use super::*;
    use crate::providers::operations::message_stream::{
        flush_connection_limiter::FlushConnectionLimiter,
        shared_state::SharedStreamingState,
        token_bucket::{Message, TokenBucket},
    };
    use serde_json::{json};

    // Helper functions
    fn create_test_message(id: &str, org_id: &str) -> Message {
        Message(json!({
            "id": id,
            "organization_id": org_id,
            "event_name": "test_event",
            "data": "test_data"
        }))
    }

    fn create_simple_message(content: &str) -> Message {
        Message(json!({
            "content": content
        }))
    }

    // SharedStreamingState Tests
    #[tokio::test]
    async fn test_shared_streaming_state_new() {
        let state = SharedStreamingState::new();

        assert!(state.backpressured_channels.lock().await.is_empty());
        assert!(state.flushing_channels.lock().await.is_empty());
        assert!(state.processing_channels.lock().await.is_empty());
        assert!(state.active_channels.lock().await.is_empty());
        assert!(state.organization_channels.lock().await.is_empty());
        assert!(state.channel_organizations.lock().await.is_empty());
        assert!(state.processing_queue.lock().await.is_empty());
    }

    #[tokio::test]
    async fn test_register_channel() {
        let state = SharedStreamingState::new();
        let token_bucket = TokenBucket::new_without_consumer("test_channel", 10);

        let result = state
            .register_channel("test_channel", "org_123", token_bucket.clone(), 10)
            .await;

        assert_eq!(result.name(), "test_channel");
        assert!(state
            .active_channels
            .lock()
            .await
            .contains_key("test_channel"));

        let org_channels = state.organization_channels.lock().await;
        assert!(org_channels
            .get("org_123")
            .unwrap()
            .contains("test_channel"));

        let channel_orgs = state.channel_organizations.lock().await;
        assert_eq!(channel_orgs.get("test_channel").unwrap(), "org_123");
    }

    #[tokio::test]
    async fn test_register_existing_channel() {
        let state = SharedStreamingState::new();
        let token_bucket1 = TokenBucket::new_without_consumer("test_channel", 10);
        let token_bucket2 = TokenBucket::new_without_consumer("test_channel", 20);

        let result1 = state
            .register_channel("test_channel", "org_123", token_bucket1.clone(), 10)
            .await;

        let result2 = state
            .register_channel("test_channel", "org_123", token_bucket2.clone(), 20)
            .await;

        // Should return the existing bucket
        assert_eq!(result1.name(), result2.name());
        assert_eq!(
            result1.get_high_watermark().await,
            result2.get_high_watermark().await
        );
    }

    #[tokio::test]
    async fn test_backpressure_management() {
        let state = SharedStreamingState::new();
        let channel_name = "test_channel";

        assert!(!state.is_backpressured(channel_name).await);

        state.mark_backpressured(channel_name).await;
        assert!(state.is_backpressured(channel_name).await);

        state.remove_backpressured(channel_name).await;
        assert!(!state.is_backpressured(channel_name).await);
    }

    #[tokio::test]
    async fn test_flushing_management() {
        let state = SharedStreamingState::new();
        let channel_name = "test_channel";

        assert!(!state.is_flushing(channel_name).await);

        state.mark_flushing(channel_name).await;
        assert!(state.is_flushing(channel_name).await);

        state.remove_flushing(channel_name).await;
        assert!(!state.is_flushing(channel_name).await);
    }

    #[tokio::test]
    async fn test_processing_queue() {
        let state = SharedStreamingState::new();

        // Queue channels for processing
        state.queue_for_processing("channel1").await;
        state.queue_for_processing("channel2").await;
        state.queue_for_processing("channel1").await; // Duplicate should be ignored

        // Dequeue and verify order
        assert_eq!(
            state.dequeue_for_processing().await,
            Some("channel1".to_string())
        );
        assert_eq!(
            state.dequeue_for_processing().await,
            Some("channel2".to_string())
        );
        assert_eq!(state.dequeue_for_processing().await, None);
    }

    #[tokio::test]
    async fn test_processing_management() {
        let state = SharedStreamingState::new();
        let channel_name = "test_channel";

        // Mark as processing (returns true if newly added)
        assert!(state.mark_processing(channel_name).await);
        assert!(!state.mark_processing(channel_name).await); // Already processing

        state.remove_processing(channel_name).await;
        assert!(state.mark_processing(channel_name).await); // Can be marked again
    }

    #[tokio::test]
    async fn test_get_channel_organization() {
        let state = SharedStreamingState::new();
        let token_bucket = TokenBucket::new_without_consumer("test_channel", 10);

        state
            .register_channel("test_channel", "org_123", token_bucket, 10)
            .await;

        assert_eq!(
            state.get_channel_organization("test_channel").await,
            Some("org_123".to_string())
        );
        assert_eq!(state.get_channel_organization("nonexistent").await, None);
    }

    #[tokio::test]
    async fn test_get_channel() {
        let state = SharedStreamingState::new();
        let token_bucket = TokenBucket::new_without_consumer("test_channel", 10);

        state
            .register_channel("test_channel", "org_123", token_bucket.clone(), 10)
            .await;

        let retrieved = state.get_channel("test_channel").await;
        assert!(retrieved.is_some());
        assert_eq!(retrieved.unwrap().name(), "test_channel");

        assert!(state.get_channel("nonexistent").await.is_none());
    }

    // TokenBucket Tests
    #[tokio::test]
    async fn test_token_bucket_new() {
        let bucket = TokenBucket::new("test_bucket", 5);

        assert_eq!(bucket.name(), "test_bucket");
        assert_eq!(bucket.get_tokens_remaining().await, 5);
        assert_eq!(bucket.get_high_watermark().await, 5);
        assert_eq!(bucket.buffer.lock().await.len(), 0);
    }

    #[tokio::test]
    async fn test_token_bucket_new_without_consumer() {
        let bucket = TokenBucket::new_without_consumer("test_bucket", 3);

        assert_eq!(bucket.name(), "test_bucket");
        assert_eq!(bucket.get_tokens_remaining().await, 3);
        assert_eq!(bucket.get_high_watermark().await, 3);
    }

    #[tokio::test]
    async fn test_receive_message_with_tokens() {
        let bucket = TokenBucket::new_without_consumer("test_bucket", 3);
        let message = create_simple_message("test");

        let result = bucket.receive_message(message).await;

        assert!(result); // Should return true when remaining_tokens > 0 (2 > 0)
        assert_eq!(bucket.get_tokens_remaining().await, 2);
        assert_eq!(bucket.buffer.lock().await.len(), 1);
    }

    #[tokio::test]
    async fn test_receive_message_without_tokens() {
        let bucket = TokenBucket::new_without_consumer("test_bucket", 1);
        let message1 = create_simple_message("test1");
        let message2 = create_simple_message("test2");

        let result1 = bucket.receive_message(message1).await;
        let result2 = bucket.receive_message(message2).await;

        assert!(!result1); // First message uses the only token, remaining_tokens = 0, so returns false
        assert!(!result2); // Second message should fail (no tokens available)
        assert_eq!(bucket.get_tokens_remaining().await, 0);
        assert_eq!(bucket.buffer.lock().await.len(), 2); // Both messages buffered
    }

    #[tokio::test]
    async fn test_emit_message() {
        let bucket = TokenBucket::new_without_consumer("test_bucket", 2);
        let message = create_test_message("msg1", "org1");

        // Add message to buffer
        bucket.receive_message(message.clone()).await;

        // Emit message
        let emitted = bucket.emit_message().await;

        assert!(emitted.is_some());
        assert_eq!(bucket.get_tokens_remaining().await, 2); // Token restored
        assert_eq!(bucket.buffer.lock().await.len(), 0); // Buffer empty
    }

    #[tokio::test]
    async fn test_emit_message_empty_buffer() {
        let bucket = TokenBucket::new_without_consumer("test_bucket", 2);

        let emitted = bucket.emit_message().await;

        assert!(emitted.is_none());
        assert_eq!(bucket.get_tokens_remaining().await, 2); // Tokens unchanged
    }

    #[tokio::test]
    async fn test_set_tokens_increase_capacity() {
        let bucket = TokenBucket::new_without_consumer("test_bucket", 2);

        // Use up tokens
        bucket.receive_message(create_simple_message("msg1")).await;
        bucket.receive_message(create_simple_message("msg2")).await;
        assert_eq!(bucket.get_tokens_remaining().await, 0);

        // Increase capacity
        bucket.set_tokens(5).await;

        assert_eq!(bucket.get_high_watermark().await, 5);
        assert_eq!(bucket.get_tokens_remaining().await, 3); // 0 + (5-2) = 3
    }

    #[tokio::test]
    async fn test_set_tokens_decrease_capacity() {
        let bucket = TokenBucket::new_without_consumer("test_bucket", 5);

        // Decrease capacity
        bucket.set_tokens(3).await;

        assert_eq!(bucket.get_high_watermark().await, 3);
        assert_eq!(bucket.get_tokens_remaining().await, 3); // Min of current and new capacity
    }

    #[tokio::test]
    async fn test_set_tokens_with_buffer_overflow() {
        let bucket = TokenBucket::new_without_consumer("test_bucket", 5);

        // Fill buffer beyond new capacity
        for i in 0..4 {
            bucket
                .receive_message(create_simple_message(&format!("msg{}", i)))
                .await;
        }

        // Decrease capacity below buffer size
        bucket.set_tokens(2).await;

        assert_eq!(bucket.get_high_watermark().await, 2);
        assert_eq!(bucket.get_tokens_remaining().await, 0); // Backpressured
    }

    // FlushConnectionLimiter Tests
    #[tokio::test]
    async fn test_flush_connection_limiter_new() {
        let limiter = FlushConnectionLimiter::new(3);

        assert!(!limiter.is_at_capacity());
    }

    #[tokio::test]
    async fn test_flush_connection_limiter_capacity() {
        let limiter = FlushConnectionLimiter::new(1);

        // Acquire the only permit
        let _result = limiter.acquire_flush_connection().await;

        // Should be at capacity now
        assert!(limiter.is_at_capacity());
    }

    // Message Tests
    #[tokio::test]
    async fn test_message_creation() {
        let message = create_test_message("test_id", "org_123");

        if let Ok(parsed) =
            serde_json::from_value::<serde_json::Map<String, serde_json::Value>>(message.0.clone())
        {
            assert_eq!(parsed.get("id").and_then(|v| v.as_str()), Some("test_id"));
            assert_eq!(
                parsed.get("organization_id").and_then(|v| v.as_str()),
                Some("org_123")
            );
            assert_eq!(
                parsed.get("event_name").and_then(|v| v.as_str()),
                Some("test_event")
            );
        } else {
            panic!("Failed to parse test message");
        }
    }

    #[tokio::test]
    async fn test_message_debug_format() {
        let message = create_simple_message("test_content");
        let debug_str = format!("{:?}", message);

        assert!(debug_str.contains("Message"));
        assert!(debug_str.contains("test_content"));
    }

    #[tokio::test]
    async fn test_message_clone() {
        let original = create_simple_message("original_content");
        let cloned = original.clone();

        // Verify both messages have the same content
        assert_eq!(original.0, cloned.0);
    }

    // Integration Tests
    #[tokio::test]
    async fn test_token_bucket_backpressure_recovery() {
        let bucket = TokenBucket::new_without_consumer("test_bucket", 1);

        // Fill bucket to capacity
        let result1 = bucket.receive_message(create_simple_message("msg1")).await;
        assert!(!result1); // Returns false because remaining_tokens = 0 after consuming the only token
        assert_eq!(bucket.get_tokens_remaining().await, 0);

        // Add another message (should be buffered)
        let result2 = bucket.receive_message(create_simple_message("msg2")).await;
        assert!(!result2); // Should indicate backpressure

        // Emit a message to free up tokens
        let emitted = bucket.emit_message().await;
        assert!(emitted.is_some());
        assert_eq!(bucket.get_tokens_remaining().await, 1); // Token restored
    }

    #[tokio::test]
    async fn test_shared_state_channel_lifecycle() {
        let state = SharedStreamingState::new();
        let token_bucket = TokenBucket::new_without_consumer("lifecycle_channel", 5);

        // Register channel
        state
            .register_channel("lifecycle_channel", "org_456", token_bucket, 5)
            .await;

        // Mark as backpressured
        state.mark_backpressured("lifecycle_channel").await;
        assert!(state.is_backpressured("lifecycle_channel").await);

        // Queue for processing
        state.queue_for_processing("lifecycle_channel").await;

        // Mark as processing
        assert!(state.mark_processing("lifecycle_channel").await);

        // Remove backpressure
        state.remove_backpressured("lifecycle_channel").await;
        assert!(!state.is_backpressured("lifecycle_channel").await);

        // Remove from processing
        state.remove_processing("lifecycle_channel").await;

        // Verify channel still exists
        assert!(state.get_channel("lifecycle_channel").await.is_some());
        assert_eq!(
            state.get_channel_organization("lifecycle_channel").await,
            Some("org_456".to_string())
        );
    }

    #[tokio::test]
    async fn test_multiple_organizations_channels() {
        let state = SharedStreamingState::new();

        // Register channels for different organizations
        let bucket1 = TokenBucket::new_without_consumer("channel1", 5);
        let bucket2 = TokenBucket::new_without_consumer("channel2", 10);
        let bucket3 = TokenBucket::new_without_consumer("channel3", 3);

        state.register_channel("channel1", "org1", bucket1, 5).await;
        state
            .register_channel("channel2", "org1", bucket2, 10)
            .await;
        state.register_channel("channel3", "org2", bucket3, 3).await;

        // Verify organization mappings
        let org1_channels = state.organization_channels.lock().await;
        let org1_set = org1_channels.get("org1").unwrap();
        assert!(org1_set.contains("channel1"));
        assert!(org1_set.contains("channel2"));
        assert!(!org1_set.contains("channel3"));

        let org2_set = org1_channels.get("org2").unwrap();
        assert!(org2_set.contains("channel3"));
        assert!(!org2_set.contains("channel1"));

        // Verify reverse mappings
        assert_eq!(
            state.get_channel_organization("channel1").await,
            Some("org1".to_string())
        );
        assert_eq!(
            state.get_channel_organization("channel2").await,
            Some("org1".to_string())
        );
        assert_eq!(
            state.get_channel_organization("channel3").await,
            Some("org2".to_string())
        );
    }
}
