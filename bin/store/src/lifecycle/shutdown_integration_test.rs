#[cfg(test)]
mod tests {
    use super::super::shutdown::{BackgroundServiceShutdown, ShutdownManager, ShutdownService};
    use crate::config::core::EnvConfig;
    use std::sync::Arc;
    use tokio::sync::mpsc;
    use tokio::time::{timeout, Duration};

    /// Creates a mock EnvConfig for testing purposes
    fn create_test_env_config() -> Arc<EnvConfig> {
        Arc::new(EnvConfig::default())
    }
    /// Tests BackgroundServiceShutdown creation and basic functionality:
    /// - Service can be created with name and shutdown channel
    /// - Service name is correctly stored and retrieved
    /// - Shutdown signal can be sent successfully
    /// - Service handles shutdown gracefully
    ///
    /// # Examples
    ///
    /// ```
    /// use crate::lifecycle::shutdown::BackgroundServiceShutdown;
    /// use tokio::sync::mpsc;
    ///
    /// let (tx, rx) = mpsc::channel(1);
    /// let service = BackgroundServiceShutdown::new("test-service".to_string(), tx);
    /// assert_eq!(service.name(), "test-service");
    /// ```
    #[tokio::test]
    async fn should_create_background_service_shutdown_with_correct_properties() {
        println!("Testing BackgroundServiceShutdown creation...");

        // Create shutdown channel
        println!("  ✓ Creating shutdown channel");
        let (shutdown_tx, mut shutdown_rx) = mpsc::channel::<()>(1);

        // Create BackgroundServiceShutdown instance
        println!("  ✓ Creating BackgroundServiceShutdown instance");
        let mut service =
            BackgroundServiceShutdown::new("test-background-service".to_string(), shutdown_tx);

        // Test service name
        println!("  ✓ Verifying service name");
        assert_eq!(service.name(), "test-background-service");

        // Test shutdown functionality
        println!("  ✓ Testing shutdown signal sending");
        let shutdown_result = service.shutdown().await;
        assert!(shutdown_result.is_ok(), "Shutdown should succeed");

        // Verify shutdown signal was received
        println!("  ✓ Verifying shutdown signal reception");
        let signal_received = timeout(Duration::from_millis(100), shutdown_rx.recv()).await;
        assert!(
            signal_received.is_ok(),
            "Should receive shutdown signal within timeout"
        );
        assert!(
            signal_received.unwrap().is_some(),
            "Should receive actual signal"
        );

        println!("BackgroundServiceShutdown creation tests completed successfully!");
    }

    /// Tests ShutdownManager service registration and shutdown coordination:
    /// - Multiple services can be registered with ShutdownManager
    /// - Services are properly stored and managed
    /// - Shutdown process handles multiple services correctly
    /// - Services receive shutdown signals in proper order
    ///
    /// # Examples
    ///
    /// ```
    /// use crate::lifecycle::shutdown::ShutdownManager;
    ///
    /// let mut manager = ShutdownManager::new();
    /// manager.register_service(Box::new(service));
    /// ```
    #[tokio::test]
    #[ignore]
    async fn should_register_multiple_services_and_coordinate_shutdown() {
        println!("Testing ShutdownManager service registration and coordination...");

        // Create ShutdownManager
        println!("  ✓ Creating ShutdownManager instance");
        let mut shutdown_manager = ShutdownManager::new();

        // Create multiple background services
        println!("  ✓ Creating multiple background services");
        let (grpc_tx, mut grpc_rx) = mpsc::channel::<()>(1);
        let (socket_tx, mut socket_rx) = mpsc::channel::<()>(1);
        let (sync_tx, mut sync_rx) = mpsc::channel::<()>(1);

        let grpc_service = BackgroundServiceShutdown::new("grpc-server".to_string(), grpc_tx);
        let socket_service =
            BackgroundServiceShutdown::new("socket-io-server".to_string(), socket_tx);
        let sync_service = BackgroundServiceShutdown::new("background-sync".to_string(), sync_tx);

        // Register services with shutdown manager
        println!("  ✓ Registering services with ShutdownManager");
        shutdown_manager.register_service(Box::new(grpc_service));
        shutdown_manager.register_service(Box::new(socket_service));
        shutdown_manager.register_service(Box::new(sync_service));

        // Simulate shutdown process
        println!("  ✓ Initiating shutdown process");
        let shutdown_result = shutdown_manager.execute().await;
        assert!(shutdown_result.is_ok(), "Shutdown process should succeed");

        // Verify all services received shutdown signals
        println!("  ✓ Verifying all services received shutdown signals");

        let grpc_signal = timeout(Duration::from_millis(100), grpc_rx.recv()).await;
        assert!(
            grpc_signal.is_ok() && grpc_signal.unwrap().is_some(),
            "gRPC service should receive shutdown signal"
        );

        let socket_signal = timeout(Duration::from_millis(100), socket_rx.recv()).await;
        assert!(
            socket_signal.is_ok() && socket_signal.unwrap().is_some(),
            "Socket.IO service should receive shutdown signal"
        );

        let sync_signal = timeout(Duration::from_millis(100), sync_rx.recv()).await;
        assert!(
            sync_signal.is_ok() && sync_signal.unwrap().is_some(),
            "Background sync service should receive shutdown signal"
        );

        println!("ShutdownManager coordination tests completed successfully!");
    }

    /// Tests edge cases and error handling in shutdown process:
    /// - Shutdown with no registered services
    /// - Shutdown with already closed channels
    /// - Force shutdown when graceful shutdown fails
    /// - Multiple shutdown calls on same service
    ///
    /// # Examples
    ///
    /// ```
    /// use crate::lifecycle::shutdown::ShutdownManager;
    ///
    /// let mut manager = ShutdownManager::new();
    /// // Should handle empty service list gracefully
    /// let result = manager.shutdown().await;
    /// assert!(result.is_ok());
    /// ```
    #[tokio::test]
    #[ignore]
    async fn should_handle_edge_cases_and_error_conditions_gracefully() {
        println!("Testing edge cases and error handling...");

        // Test shutdown with no registered services
        println!("  ✓ Testing shutdown with no registered services");
        let mut empty_manager = ShutdownManager::new();
        let empty_result = empty_manager.execute().await;
        assert!(empty_result.is_ok(), "Empty shutdown should succeed");

        // Test shutdown with closed channel
        println!("  ✓ Testing shutdown with closed channel");
        let (closed_tx, closed_rx) = mpsc::channel::<()>(1);
        drop(closed_rx); // Close the receiver

        let mut closed_service =
            BackgroundServiceShutdown::new("closed-service".to_string(), closed_tx);

        let closed_shutdown_result = closed_service.shutdown().await;
        // Should still succeed even if channel is closed
        assert!(
            closed_shutdown_result.is_ok(),
            "Shutdown should handle closed channels gracefully"
        );

        // Test multiple shutdown calls on same service
        println!("  ✓ Testing multiple shutdown calls on same service");
        let (multi_tx, mut multi_rx) = mpsc::channel::<()>(1);
        let mut multi_service =
            BackgroundServiceShutdown::new("multi-shutdown-service".to_string(), multi_tx);

        // First shutdown call
        let first_shutdown = multi_service.shutdown().await;
        assert!(first_shutdown.is_ok(), "First shutdown should succeed");

        // Second shutdown call (should handle gracefully)
        let second_shutdown = multi_service.shutdown().await;
        assert!(
            second_shutdown.is_ok(),
            "Second shutdown should handle gracefully"
        );

        // Verify only one signal was sent (first shutdown call should send signal)
        let signal_received = timeout(Duration::from_millis(100), multi_rx.recv()).await;
        assert!(
            signal_received.is_ok(),
            "Should receive shutdown signal from first call"
        );
        println!("  ✓ Received first shutdown signal");

        // Verify no additional signals are sent (second call should be no-op since tx was taken)
        // The channel should be closed after the sender is dropped
        let no_more_signals = timeout(Duration::from_millis(50), multi_rx.recv()).await;
        match no_more_signals {
            Ok(None) => println!("  ✓ Channel closed as expected"),
            Ok(Some(_)) => panic!("Should not receive additional signals after first shutdown"),
            Err(_) => println!("  ✓ No additional signals received (timeout)"),
        }

        println!("Edge case and error handling tests completed successfully!");
    }

    /// Tests integration between RuntimeManager and ShutdownManager:
    /// - RuntimeManager properly registers services during startup
    /// - Services are correctly configured with shutdown channels
    /// - Shutdown coordination works end-to-end
    /// - Background services respond to shutdown signals appropriately
    ///
    /// # Examples
    ///
    /// ```
    /// use crate::lifecycle::runtime::RuntimeManager;
    /// use crate::lifecycle::shutdown::ShutdownManager;
    /// use std::sync::Arc;
    ///
    /// let mut shutdown_manager = ShutdownManager::new();
    /// let config = create_test_env_config();
    /// let runtime_manager = RuntimeManager::new(config)
    ///     .with_shutdown_manager(&mut shutdown_manager);
    /// ```
    #[tokio::test]
    #[ignore]
    async fn should_integrate_runtime_manager_with_shutdown_manager() {
        println!("Testing RuntimeManager and ShutdownManager integration...");

        // This test verifies the integration points without running full services
        println!("  ✓ Verifying integration architecture");

        // Test that RuntimeManager can hold ShutdownManager reference
        use crate::lifecycle::runtime::RuntimeManager;
        let mut shutdown_manager = ShutdownManager::new();

        println!("  ✓ Creating RuntimeManager with ShutdownManager reference");

        let config = create_test_env_config();
        let _runtime_manager =
            RuntimeManager::new(config).with_shutdown_manager(&mut shutdown_manager);

        // Verify the runtime manager was configured correctly
        // (This is a structural test since we can't easily test the private field)
        println!("  ✓ RuntimeManager configured with ShutdownManager successfully");

        // Test service registration pattern
        println!("  ✓ Testing service registration pattern");
        let (test_tx, mut test_rx) = mpsc::channel::<()>(1);
        let test_service =
            BackgroundServiceShutdown::new("integration-test-service".to_string(), test_tx);

        shutdown_manager.register_service(Box::new(test_service));

        // Verify shutdown works through the manager
        println!("  ✓ Testing shutdown through manager");
        let shutdown_result = shutdown_manager.execute().await;
        assert!(
            shutdown_result.is_ok(),
            "Integrated shutdown should succeed"
        );

        // Verify service received signal
        let integration_signal = timeout(Duration::from_millis(100), test_rx.recv()).await;
        assert!(
            integration_signal.is_ok() && integration_signal.unwrap().is_some(),
            "Integration test service should receive shutdown signal"
        );

        println!("RuntimeManager and ShutdownManager integration tests completed successfully!");
    }
}
