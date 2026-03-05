#[cfg(test)]
mod tests {
    // use super::*;
    use crate::providers::operations::message_stream::{
        flush_connection_limiter::FlushConnectionLimiter,
        shared_state::SharedStreamingState,
        token_bucket::{Message, TokenBucket},
    };
    use serde_json::json;

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
    /// Tests that SharedStreamingState initializes with empty collections
    /// This ensures proper initialization of all internal state collections
    #[tokio::test]
    async fn should_initialize_shared_streaming_state_with_empty_collections() {
        println!("Testing SharedStreamingState initialization");

        let state = SharedStreamingState::new();

        println!("Checking all collections are empty after initialization");
        assert!(state.backpressured_channels.lock().await.is_empty());
        assert!(state.flushing_channels.lock().await.is_empty());
        assert!(state.processing_channels.lock().await.is_empty());
        assert!(state.active_channels.lock().await.is_empty());
        assert!(state.organization_channels.lock().await.is_empty());
        assert!(state.channel_organizations.lock().await.is_empty());
        assert!(state.processing_queue.lock().await.is_empty());

        println!("SharedStreamingState initialization test passed");
    }

    /// Tests that channel registration works correctly with valid parameters
    /// This ensures proper channel registration and state management
    #[tokio::test]
    async fn should_register_channel_successfully_with_valid_parameters() {
        println!("Testing channel registration functionality");

        let state = SharedStreamingState::new();
        let token_bucket = TokenBucket::new_without_consumer("test_channel", 10);

        println!("Registering channel: test_channel for organization: org_123");
        let result = state
            .register_channel("test_channel", "org_123", token_bucket.clone(), 10)
            .await;

        println!("Verifying registered channel name: {}", result.name());
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

    /// Tests that registering an existing channel returns the existing bucket
    /// This ensures proper handling of duplicate channel registrations
    #[tokio::test]
    async fn should_return_existing_bucket_when_registering_duplicate_channel() {
        println!("Testing duplicate channel registration handling");

        let state = SharedStreamingState::new();
        let token_bucket1 = TokenBucket::new_without_consumer("test_channel", 10);
        let token_bucket2 = TokenBucket::new_without_consumer("test_channel", 20);

        println!("Registering channel first time with capacity 10");
        let result1 = state
            .register_channel("test_channel", "org_123", token_bucket1.clone(), 10)
            .await;

        println!("Registering same channel again with capacity 20");
        let result2 = state
            .register_channel("test_channel", "org_123", token_bucket2.clone(), 20)
            .await;

        // Should return the existing bucket
        println!("Verifying both registrations return same bucket");
        assert_eq!(result1.name(), result2.name());
        assert_eq!(
            result1.get_high_watermark().await,
            result2.get_high_watermark().await
        );

        println!("Duplicate channel registration test passed");
    }

    /// Tests that backpressure management works correctly
    /// This ensures proper tracking of backpressured channels
    #[tokio::test]
    async fn should_manage_backpressure_state_correctly() {
        println!("Testing backpressure management functionality");

        let state = SharedStreamingState::new();
        let channel_name = "test_channel";

        println!("Checking initial backpressure state: false");
        assert!(!state.is_backpressured(channel_name).await);

        println!("Marking channel as backpressured");
        state.mark_backpressured(channel_name).await;
        assert!(state.is_backpressured(channel_name).await);

        println!("Removing backpressure from channel");
        state.remove_backpressured(channel_name).await;
        assert!(!state.is_backpressured(channel_name).await);

        println!("Backpressure management test passed");
    }

    /// Tests that flushing management works correctly
    /// This ensures proper tracking of channels being flushed
    #[tokio::test]
    async fn should_manage_flushing_state_correctly() {
        println!("Testing flushing management functionality");

        let state = SharedStreamingState::new();
        let channel_name = "test_channel";

        println!("Checking initial flushing state: false");
        assert!(!state.is_flushing(channel_name).await);

        println!("Marking channel as flushing");
        state.mark_flushing(channel_name).await;
        assert!(state.is_flushing(channel_name).await);

        println!("Removing flushing state from channel");
        state.remove_flushing(channel_name).await;
        assert!(!state.is_flushing(channel_name).await);

        println!("Flushing management test passed");
    }

    /// Tests that processing queue management works correctly
    /// This ensures proper queuing and dequeuing of channels for processing
    #[tokio::test]
    async fn should_manage_processing_queue_correctly() {
        println!("Testing processing queue management functionality");

        let state = SharedStreamingState::new();

        println!("Queuing channels for processing");
        // Queue channels for processing
        state.queue_for_processing("channel1").await;
        state.queue_for_processing("channel2").await;
        state.queue_for_processing("channel1").await; // Duplicate should be ignored

        println!("Dequeuing channels and verifying order");
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

        println!("Processing queue management test passed");
    }

    /// Tests that processing management works correctly
    /// This ensures proper tracking of processing state for channels
    #[tokio::test]
    async fn should_manage_processing_state_correctly() {
        println!("Testing processing management functionality");

        let state = SharedStreamingState::new();
        let channel_name = "test_channel";

        println!("Marking channel as processing (should return true for new channel)");
        // Mark as processing (returns true if newly added)
        assert!(state.mark_processing(channel_name).await);
        assert!(!state.mark_processing(channel_name).await); // Already processing

        println!("Removing processing state and re-marking");
        state.remove_processing(channel_name).await;
        assert!(state.mark_processing(channel_name).await); // Can be marked again

        println!("Processing management test passed");
    }

    /// Tests that channel organization retrieval works correctly
    /// This ensures proper mapping between channels and organizations
    #[tokio::test]
    async fn should_retrieve_channel_organization_correctly() {
        println!("Testing channel organization retrieval functionality");

        let state = SharedStreamingState::new();
        let token_bucket = TokenBucket::new_without_consumer("test_channel", 10);

        println!("Registering channel with organization org_123");
        state
            .register_channel("test_channel", "org_123", token_bucket, 10)
            .await;

        println!("Retrieving organization for registered channel");
        assert_eq!(
            state.get_channel_organization("test_channel").await,
            Some("org_123".to_string())
        );

        println!("Checking organization for non-existent channel");
        assert_eq!(state.get_channel_organization("nonexistent").await, None);

        println!("Channel organization retrieval test passed");
    }

    /// Tests that channel retrieval works correctly
    /// This ensures proper retrieval of registered channels
    #[tokio::test]
    async fn should_retrieve_registered_channel_correctly() {
        println!("Testing channel retrieval functionality");

        let state = SharedStreamingState::new();
        let token_bucket = TokenBucket::new_without_consumer("test_channel", 10);

        println!("Registering channel with token bucket");
        state
            .register_channel("test_channel", "org_123", token_bucket.clone(), 10)
            .await;

        println!("Retrieving registered channel");
        let retrieved = state.get_channel("test_channel").await;
        assert!(retrieved.is_some());
        assert_eq!(retrieved.unwrap().name(), "test_channel");

        println!("Checking retrieval of non-existent channel");
        assert!(state.get_channel("nonexistent").await.is_none());

        println!("Channel retrieval test passed");
    }

    // TokenBucket Tests
    /// Tests that TokenBucket initializes correctly with specified parameters
    /// This ensures proper token bucket creation and initial state
    #[tokio::test]
    async fn should_create_token_bucket_with_correct_initial_state() {
        println!("Testing TokenBucket initialization with consumer");

        let bucket = TokenBucket::new("test_bucket", 5);

        println!("Created token bucket: {} with capacity: 5", bucket.name());
        println!(
            "Initial available tokens: {}",
            bucket.get_tokens_remaining().await
        );

        assert_eq!(bucket.name(), "test_bucket");
        assert_eq!(bucket.get_tokens_remaining().await, 5);
        assert_eq!(bucket.get_high_watermark().await, 5);
        assert_eq!(bucket.buffer.lock().await.len(), 0);

        println!("TokenBucket initialization test passed");
    }

    /// Tests that token bucket creation without consumer works correctly
    /// This ensures proper initialization of token buckets without consumer
    #[tokio::test]
    async fn should_create_token_bucket_without_consumer_correctly() {
        println!("Testing token bucket creation without consumer");

        let bucket = TokenBucket::new_without_consumer("test_bucket", 3);

        println!("Verifying bucket name and initial state");
        assert_eq!(bucket.name(), "test_bucket");
        assert_eq!(bucket.get_tokens_remaining().await, 3);
        assert_eq!(bucket.get_high_watermark().await, 3);

        println!("Token bucket creation test passed");
    }

    /// Tests that message reception with available tokens works correctly
    /// This ensures proper token consumption when receiving messages
    #[tokio::test]
    async fn should_receive_message_when_tokens_available() {
        println!("Testing message reception with available tokens");

        let bucket = TokenBucket::new_without_consumer("test_bucket", 3);
        let message = create_simple_message("test");

        println!("Attempting to receive message with available tokens");
        let result = bucket.receive_message(message).await;

        println!("Verifying message reception and token consumption");
        assert!(result); // Should return true when remaining_tokens > 0 (2 > 0)
        assert_eq!(bucket.get_tokens_remaining().await, 2);
        assert_eq!(bucket.buffer.lock().await.len(), 1);

        println!("Message reception with tokens test passed");
    }

    /// Tests that message reception without available tokens fails correctly
    /// This ensures proper handling when no tokens are available
    #[tokio::test]
    async fn should_reject_message_when_no_tokens_available() {
        println!("Testing message reception without available tokens");

        let bucket = TokenBucket::new_without_consumer("test_bucket", 1);
        let message1 = create_simple_message("test1");
        let message2 = create_simple_message("test2");

        println!("Attempting to receive messages with limited tokens");
        let result1 = bucket.receive_message(message1).await;
        let result2 = bucket.receive_message(message2).await;

        println!("Verifying message handling and token consumption");
        assert!(!result1); // First message uses the only token, remaining_tokens = 0, so returns false
        assert!(!result2); // Second message should fail (no tokens available)
        assert_eq!(bucket.get_tokens_remaining().await, 0);
        assert_eq!(bucket.buffer.lock().await.len(), 2); // Both messages buffered

        println!("Message rejection without tokens test passed");
    }

    /// Tests that message emission works correctly
    /// This ensures proper message emission from buffer
    #[tokio::test]
    async fn should_emit_message_from_buffer_correctly() {
        println!("Testing message emission functionality");

        let bucket = TokenBucket::new_without_consumer("test_bucket", 2);
        let message = create_test_message("msg1", "org1");

        println!("Adding message to buffer");
        // Add message to buffer
        bucket.receive_message(message.clone()).await;

        println!("Emitting message from buffer");
        // Emit message
        let emitted = bucket.emit_message().await;

        println!("Verifying message emission and token restoration");
        assert!(emitted.is_some());
        assert_eq!(bucket.get_tokens_remaining().await, 2); // Token restored
        assert_eq!(bucket.buffer.lock().await.len(), 0); // Buffer empty

        println!("Message emission test passed");
    }

    /// Tests that message emission from empty buffer returns None
    /// This ensures proper handling of empty buffer emission
    #[tokio::test]
    async fn should_return_none_when_emitting_from_empty_buffer() {
        println!("Testing message emission from empty buffer");

        let bucket = TokenBucket::new_without_consumer("test_bucket", 2);

        println!("Attempting to emit message from empty buffer");
        let emitted = bucket.emit_message().await;

        println!("Verifying None is returned for empty buffer");
        assert!(emitted.is_none());
        assert_eq!(bucket.get_tokens_remaining().await, 2); // Tokens unchanged

        println!("Empty buffer emission test passed");
    }

    /// Tests that setting tokens increases capacity when needed
    /// This ensures proper capacity expansion when token count exceeds current capacity
    #[tokio::test]
    async fn should_increase_capacity_when_setting_more_tokens() {
        println!("Testing token capacity increase functionality");

        let bucket = TokenBucket::new_without_consumer("test_bucket", 2);

        println!("Using up initial tokens");
        // Use up tokens
        bucket.receive_message(create_simple_message("msg1")).await;
        bucket.receive_message(create_simple_message("msg2")).await;
        assert_eq!(bucket.get_tokens_remaining().await, 0);

        println!("Increasing capacity to 5 tokens");
        // Increase capacity
        bucket.set_tokens(5).await;

        println!("Verifying token count and capacity increase");
        assert_eq!(bucket.get_high_watermark().await, 5);
        assert_eq!(bucket.get_tokens_remaining().await, 3); // 0 + (5-2) = 3

        println!("Token capacity increase test passed");
    }

    /// Tests that setting tokens decreases capacity when needed
    /// This ensures proper capacity reduction when token count is less than current capacity
    #[tokio::test]
    async fn should_decrease_capacity_when_setting_fewer_tokens() {
        println!("Testing token capacity decrease functionality");

        let bucket = TokenBucket::new_without_consumer("test_bucket", 5);

        println!("Decreasing capacity from 5 to 3 tokens");
        // Decrease capacity
        bucket.set_tokens(3).await;

        println!("Verifying token count and capacity decrease");
        assert_eq!(bucket.get_high_watermark().await, 3);
        assert_eq!(bucket.get_tokens_remaining().await, 3); // Min of current and new capacity

        println!("Token capacity decrease test passed");
    }

    /// Tests that setting tokens with buffer overflow handles capacity correctly
    /// This ensures proper handling when buffer size exceeds new token capacity
    #[tokio::test]
    async fn should_handle_buffer_overflow_when_setting_tokens() {
        println!("Testing token setting with buffer overflow");

        let bucket = TokenBucket::new_without_consumer("test_bucket", 5);

        println!("Filling buffer with 4 messages");
        // Fill buffer beyond new capacity
        for i in 0..4 {
            bucket
                .receive_message(create_simple_message(&format!("msg{}", i)))
                .await;
        }

        println!("Decreasing capacity to 2 (below buffer size of 4)");
        // Decrease capacity below buffer size
        bucket.set_tokens(2).await;

        println!("Verifying token count and backpressure state");
        assert_eq!(bucket.get_high_watermark().await, 2);
        assert_eq!(bucket.get_tokens_remaining().await, 0); // Backpressured

        println!("Buffer overflow handling test passed");
    }

    // FlushConnectionLimiter Tests
    /// Tests that FlushConnectionLimiter initializes correctly
    /// This ensures proper initialization of connection limiter with specified capacity
    #[tokio::test]
    async fn should_initialize_flush_connection_limiter_correctly() {
        println!("Testing FlushConnectionLimiter initialization");

        let limiter = FlushConnectionLimiter::new(3);

        println!("Verifying limiter is not at capacity initially");
        assert!(!limiter.is_at_capacity());

        println!("FlushConnectionLimiter initialization test passed");
    }

    /// Tests that FlushConnectionLimiter capacity management works correctly
    /// This ensures proper capacity tracking when acquiring permits
    #[tokio::test]
    #[ignore]
    async fn should_manage_flush_connection_capacity_correctly() {
        println!("Testing FlushConnectionLimiter capacity management");

        let limiter = FlushConnectionLimiter::new(1);

        println!("Acquiring the only available permit");
        // Acquire the only permit
        let _result = limiter.acquire_flush_connection().await;

        println!("Verifying limiter is at capacity after permit acquisition");
        // Should be at capacity now
        assert!(limiter.is_at_capacity());

        println!("FlushConnectionLimiter capacity management test passed");
    }

    // Message Tests
    /// Tests that Message creation works correctly with proper field values
    /// This ensures proper message structure and field assignment
    #[tokio::test]
    async fn should_create_message_with_correct_fields() {
        println!("Testing Message creation functionality");

        let message = create_test_message("test_id", "org_123");

        println!("Parsing and verifying message fields");
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

        println!("Message creation test passed");
    }

    /// Tests that Message debug formatting works correctly
    /// This ensures proper debug representation of message content
    #[tokio::test]
    async fn should_format_message_debug_output_correctly() {
        println!("Testing Message debug formatting");

        let message = create_simple_message("test_content");
        let debug_str = format!("{:?}", message);

        println!("Verifying debug string contains expected elements");
        assert!(debug_str.contains("Message"));
        assert!(debug_str.contains("test_content"));

        println!("Message debug formatting test passed");
    }

    /// Tests that Message cloning works correctly
    /// This ensures proper cloning behavior preserving message content
    #[tokio::test]
    async fn should_clone_message_with_identical_content() {
        println!("Testing Message cloning functionality");

        let original = create_simple_message("original_content");
        let cloned = original.clone();

        println!("Verifying cloned message has identical content");
        // Verify both messages have the same content
        assert_eq!(original.0, cloned.0);

        println!("Message cloning test passed");
    }

    // Integration Tests
    /// Tests that token bucket backpressure recovery works correctly
    /// This ensures proper recovery from backpressure conditions through message emission
    #[tokio::test]
    async fn should_recover_from_backpressure_through_message_emission() {
        println!("Testing token bucket backpressure recovery");

        let bucket = TokenBucket::new_without_consumer("test_bucket", 1);

        println!("Filling bucket to capacity with first message");
        // Fill bucket to capacity
        let result1 = bucket.receive_message(create_simple_message("msg1")).await;
        assert!(!result1); // Returns false because remaining_tokens = 0 after consuming the only token
        assert_eq!(bucket.get_tokens_remaining().await, 0);

        println!("Adding second message (should trigger backpressure)");
        // Add another message (should be buffered)
        let result2 = bucket.receive_message(create_simple_message("msg2")).await;
        assert!(!result2); // Should indicate backpressure

        println!("Emitting message to recover from backpressure");
        // Emit a message to free up tokens
        let emitted = bucket.emit_message().await;
        assert!(emitted.is_some());
        assert_eq!(bucket.get_tokens_remaining().await, 1); // Token restored

        println!("Backpressure recovery test passed");
    }

    /// Tests that shared state channel lifecycle management works correctly
    /// This ensures proper handling of channel states throughout its lifecycle
    #[tokio::test]
    async fn should_manage_channel_lifecycle_states_correctly() {
        println!("Testing shared state channel lifecycle management");

        let state = SharedStreamingState::new();
        let token_bucket = TokenBucket::new_without_consumer("lifecycle_channel", 5);

        println!("Registering channel for lifecycle testing");
        // Register channel
        state
            .register_channel("lifecycle_channel", "org_456", token_bucket, 5)
            .await;

        println!("Testing backpressure state management");
        // Mark as backpressured
        state.mark_backpressured("lifecycle_channel").await;
        assert!(state.is_backpressured("lifecycle_channel").await);

        println!("Queuing channel for processing");
        // Queue for processing
        state.queue_for_processing("lifecycle_channel").await;

        println!("Marking channel as processing");
        // Mark as processing
        assert!(state.mark_processing("lifecycle_channel").await);

        println!("Removing backpressure state");
        // Remove backpressure
        state.remove_backpressured("lifecycle_channel").await;
        assert!(!state.is_backpressured("lifecycle_channel").await);

        println!("Removing processing state");
        // Remove from processing
        state.remove_processing("lifecycle_channel").await;

        println!("Verifying channel persistence after state changes");
        // Verify channel still exists
        assert!(state.get_channel("lifecycle_channel").await.is_some());
        assert_eq!(
            state.get_channel_organization("lifecycle_channel").await,
            Some("org_456".to_string())
        );

        println!("Channel lifecycle management test passed");
    }

    /// Tests that multiple organizations with channels are managed correctly
    /// This ensures proper organization-channel mapping and isolation
    #[tokio::test]
    async fn should_manage_multiple_organizations_and_channels_correctly() {
        println!("Testing multiple organizations and channels management");

        let state = SharedStreamingState::new();

        println!("Registering channels for different organizations");
        // Register channels for different organizations
        let bucket1 = TokenBucket::new_without_consumer("channel1", 5);
        let bucket2 = TokenBucket::new_without_consumer("channel2", 10);
        let bucket3 = TokenBucket::new_without_consumer("channel3", 3);

        state.register_channel("channel1", "org1", bucket1, 5).await;
        state
            .register_channel("channel2", "org1", bucket2, 10)
            .await;
        state.register_channel("channel3", "org2", bucket3, 3).await;

        println!("Verifying organization to channels mappings");
        // Verify organization mappings
        let org1_channels = state.organization_channels.lock().await;
        let org1_set = org1_channels.get("org1").unwrap();
        assert!(org1_set.contains("channel1"));
        assert!(org1_set.contains("channel2"));
        assert!(!org1_set.contains("channel3"));

        let org2_set = org1_channels.get("org2").unwrap();
        assert!(org2_set.contains("channel3"));
        assert!(!org2_set.contains("channel1"));

        println!("Verifying channel to organization reverse mappings");
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

        println!("Multiple organizations management test passed");
    }
}
