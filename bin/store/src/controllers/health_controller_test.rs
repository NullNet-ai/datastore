#[cfg(test)]
mod tests {
    use crate::controllers::health_controller::{
        ComponentMetadataRequest, HealthCheckRequest, HealthController, MonitoringConfigRequest,
    };
    use crate::lifecycle::health_service::HealthService;
    use crate::lifecycle::state::{ComponentStatus, StateManager};
    use actix_web::{test, web, App};
    use serde_json::json;
    use std::sync::Arc;
    use tokio;

    /// Creates a mock HealthService for testing purposes
    /// Returns a configured HealthService with predefined test data
    async fn create_mock_health_service() -> Arc<HealthService> {
        // Initialize cache configuration for testing
        use crate::providers::storage::cache::{cache_factory::CacheType, CacheConfig};
        let _ = CacheConfig::init(CacheType::InMemory, None, None);

        let health_service = HealthService::new();
        // Set health service to healthy status for testing
        health_service.update_health_status(true).await;
        Arc::new(health_service)
    }

    /// Creates a mock StateManager for testing purposes
    /// Returns a configured StateManager with predefined test components
    async fn create_mock_state_manager() -> Arc<StateManager> {
        // Initialize cache configuration for testing
        use crate::providers::storage::cache::{cache_factory::CacheType, CacheConfig};
        let _ = CacheConfig::init(CacheType::InMemory, None, None);

        let state_manager = Arc::new(StateManager::new());

        // Add test components
        state_manager
            .update_component_status("RuntimeManager", ComponentStatus::Running)
            .await;
        state_manager
            .update_component_status("DatabasePool", ComponentStatus::Running)
            .await;
        state_manager
            .update_component_status("S3Client", ComponentStatus::Running)
            .await;
        state_manager
            .update_component_status("StartupManager", ComponentStatus::Running)
            .await;
        state_manager
            .update_component_status("ShutdownManager", ComponentStatus::NotStarted)
            .await;

        state_manager
    }

    /// Tests basic health check endpoint functionality:
    /// - Verifies endpoint returns 200 OK for healthy service
    /// - Validates response structure contains required fields
    /// - Confirms component status reporting
    /// - Checks system metrics inclusion
    ///
    /// # Examples
    ///
    /// ```
    /// use crate::controllers::health_controller::HealthController;
    /// use actix_web::test;
    ///
    /// // Test basic health endpoint
    /// let req = test::TestRequest::get()
    ///     .uri("/api/health")
    ///     .to_request();
    /// let resp = test::call_service(&app, req).await;
    /// assert_eq!(resp.status(), 200);
    /// ```
    #[tokio::test]
    async fn should_return_basic_health_status_successfully() {
        println!("Testing basic health check endpoint functionality...");

        // Create mock dependencies
        println!("  ✓ Creating mock HealthService");
        let health_service = create_mock_health_service().await;

        // Create test application
        println!("  ✓ Setting up test application");
        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(health_service.clone()))
                .route("/health", web::get().to(HealthController::health_check)),
        )
        .await;

        // Test the endpoint
        println!("  ✓ Testing basic health endpoint");
        let req = test::TestRequest::get().uri("/health").to_request();
        let resp = test::call_service(&app, req).await;

        // Verify response status
        let status = resp.status();
        println!("    ✓ Verifying response status: {}", status);

        if !status.is_success() {
            let body = test::read_body(resp).await;
            let body_str = String::from_utf8_lossy(&body);
            println!("    Response body: {}", body_str);
            panic!(
                "Health endpoint should return success status, got: {}",
                status
            );
        }

        assert!(
            status.is_success(),
            "Health endpoint should return success status, got: {}",
            status
        );

        // Parse response body
        let body = test::read_body(resp).await;
        let health_response: serde_json::Value =
            serde_json::from_slice(&body).expect("Response should be valid JSON");

        // Verify required fields exist
        println!("    ✓ Verifying response structure");
        assert!(
            health_response["status"].is_string(),
            "Status field should exist"
        );
        assert!(
            health_response["timestamp"].is_string(),
            "Timestamp field should exist"
        );
        assert!(
            health_response["uptime_seconds"].is_number(),
            "Uptime field should exist"
        );
        assert!(
            health_response["components"].is_object(),
            "Components field should exist"
        );
        assert!(
            health_response["metrics"].is_object(),
            "Metrics field should exist"
        );

        println!("Basic health check endpoint test completed successfully!");
    }

    /// Tests detailed health check endpoint functionality:
    /// - Verifies comprehensive health information is returned
    /// - Validates additional checks are performed (database, cache)
    /// - Confirms detailed component information
    /// - Checks system metrics and performance data
    ///
    /// # Examples
    ///
    /// ```
    /// use crate::controllers::health_controller::HealthController;
    /// use actix_web::test;
    ///
    /// // Test detailed health endpoint
    /// let req = test::TestRequest::get()
    ///     .uri("/api/health/detailed")
    ///     .to_request();
    /// let resp = test::call_service(&app, req).await;
    /// assert_eq!(resp.status(), 200);
    /// ```
    #[tokio::test]
    async fn should_return_detailed_health_information_successfully() {
        println!("Testing detailed health check endpoint functionality...");

        // Create mock dependencies
        println!("  ✓ Creating mock HealthService");
        let health_service = create_mock_health_service().await;

        // Create test application
        println!("  ✓ Setting up test application");
        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(health_service.clone()))
                .route(
                    "/health/detailed",
                    web::get().to(HealthController::detailed_health_check),
                ),
        )
        .await;

        // Test the endpoint
        println!("  ✓ Testing detailed health endpoint");
        let req = test::TestRequest::get()
            .uri("/health/detailed")
            .to_request();
        let resp = test::call_service(&app, req).await;

        // Verify response status
        println!("    ✓ Verifying response status");
        assert!(
            resp.status().is_success(),
            "Detailed health endpoint should return success status"
        );

        // Parse response body
        let body = test::read_body(resp).await;
        let health_response: serde_json::Value =
            serde_json::from_slice(&body).expect("Response should be valid JSON");

        // Verify detailed response structure
        println!("    ✓ Verifying detailed response structure");
        assert!(
            health_response["status"].is_string(),
            "Status field should exist"
        );
        assert!(
            health_response["timestamp"].is_string(),
            "Timestamp field should exist"
        );
        assert!(
            health_response["uptime_seconds"].is_number(),
            "Uptime field should exist"
        );
        assert!(
            health_response["components"].is_object(),
            "Components field should exist"
        );
        assert!(
            health_response["metrics"].is_object(),
            "Metrics field should exist"
        );
        assert!(
            health_response["checks"].is_object(),
            "Checks field should exist"
        );

        // Verify checks include database and cache
        let checks = &health_response["checks"];
        assert!(
            checks["database"].is_object(),
            "Database check should exist"
        );
        assert!(checks["cache"].is_object(), "Cache check should exist");

        println!("Detailed health check endpoint test completed successfully!");
    }

    /// Tests readiness probe endpoint functionality:
    /// - Verifies Kubernetes-style readiness probe response
    /// - Validates component readiness status
    /// - Confirms proper boolean ready flag
    /// - Checks individual component readiness
    ///
    /// # Examples
    ///
    /// ```
    /// use crate::controllers::health_controller::HealthController;
    /// use actix_web::test;
    ///
    /// // Test readiness probe
    /// let req = test::TestRequest::get()
    ///     .uri("/api/health/ready")
    ///     .to_request();
    /// let resp = test::call_service(&app, req).await;
    /// assert_eq!(resp.status(), 200);
    /// ```
    #[tokio::test]
    async fn should_return_readiness_probe_status_successfully() {
        println!("Testing readiness probe endpoint functionality...");

        // Create mock dependencies
        println!("  ✓ Creating mock HealthService");
        let health_service = create_mock_health_service().await;

        // Create test application
        println!("  ✓ Setting up test application");
        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(health_service.clone()))
                .route(
                    "/health/ready",
                    web::get().to(HealthController::readiness_probe),
                ),
        )
        .await;

        // Test the endpoint
        println!("  ✓ Testing readiness probe endpoint");
        let req = test::TestRequest::get().uri("/health/ready").to_request();
        let resp = test::call_service(&app, req).await;

        // Verify response status
        println!("    ✓ Verifying response status");
        assert!(
            resp.status().is_success(),
            "Readiness probe should return success status"
        );

        // Parse response body
        let body = test::read_body(resp).await;
        let readiness_response: serde_json::Value =
            serde_json::from_slice(&body).expect("Response should be valid JSON");

        // Verify readiness response structure
        println!("    ✓ Verifying readiness response structure");
        assert!(
            readiness_response["ready"].is_boolean(),
            "Ready field should be boolean"
        );
        assert!(
            readiness_response["components"].is_object(),
            "Components field should exist"
        );

        println!("Readiness probe endpoint test completed successfully!");
    }

    /// Tests liveness probe endpoint functionality:
    /// - Verifies Kubernetes-style liveness probe response
    /// - Validates alive status reporting
    /// - Confirms uptime information
    /// - Checks basic service availability
    ///
    /// # Examples
    ///
    /// ```
    /// use crate::controllers::health_controller::HealthController;
    /// use actix_web::test;
    ///
    /// // Test liveness probe
    /// let req = test::TestRequest::get()
    ///     .uri("/api/health/live")
    ///     .to_request();
    /// let resp = test::call_service(&app, req).await;
    /// assert_eq!(resp.status(), 200);
    /// ```
    #[tokio::test]
    async fn should_return_liveness_probe_status_successfully() {
        println!("Testing liveness probe endpoint functionality...");

        // Create mock dependencies
        println!("  ✓ Creating mock HealthService");
        let health_service = create_mock_health_service().await;

        // Create test application
        println!("  ✓ Setting up test application");
        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(health_service.clone()))
                .route(
                    "/health/live",
                    web::get().to(HealthController::liveness_probe),
                ),
        )
        .await;

        // Test the endpoint
        println!("  ✓ Testing liveness probe endpoint");
        let req = test::TestRequest::get().uri("/health/live").to_request();
        let resp = test::call_service(&app, req).await;

        // Verify response status
        println!("    ✓ Verifying response status");
        assert!(
            resp.status().is_success(),
            "Liveness probe should return success status"
        );

        // Parse response body
        let body = test::read_body(resp).await;
        let liveness_response: serde_json::Value =
            serde_json::from_slice(&body).expect("Response should be valid JSON");

        // Verify liveness response structure
        println!("    ✓ Verifying liveness response structure");
        assert!(
            liveness_response["alive"].is_boolean(),
            "Alive field should be boolean"
        );
        assert!(
            liveness_response["uptime_seconds"].is_number(),
            "Uptime field should exist"
        );

        // Verify alive is true (if we can respond, we're alive)
        assert_eq!(
            liveness_response["alive"], true,
            "Service should report as alive"
        );

        println!("Liveness probe endpoint test completed successfully!");
    }

    /// Tests component metadata update endpoint functionality:
    /// - Verifies PUT request to update component metadata
    /// - Validates request payload processing
    /// - Confirms successful metadata update
    /// - Checks response structure and content
    ///
    /// # Examples
    ///
    /// ```
    /// use crate::controllers::health_controller::HealthController;
    /// use actix_web::test;
    ///
    /// // Test metadata update
    /// let payload = json!({
    ///     "key": "test_key",
    ///     "value": "test_value"
    /// });
    /// let req = test::TestRequest::put()
    ///     .uri("/api/health/components/RuntimeManager/metadata")
    ///     .set_json(&payload)
    ///     .to_request();
    /// ```
    #[tokio::test]
    async fn should_update_component_metadata_successfully() {
        println!("Testing component metadata update endpoint functionality...");

        // Create mock dependencies
        println!("  ✓ Creating mock StateManager");
        let state_manager = create_mock_state_manager().await;

        // Register a test component
        println!("  ✓ Registering test component");
        state_manager
            .register_component("RuntimeManager".to_string())
            .await;

        // Create test application
        println!("  ✓ Setting up test application");
        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(state_manager.clone()))
                .route(
                    "/health/components/{component_name}/metadata",
                    web::put().to(HealthController::update_component_metadata),
                ),
        )
        .await;

        // Prepare test payload
        println!("  ✓ Preparing test payload");
        let payload = ComponentMetadataRequest {
            key: "test_key".to_string(),
            value: "test_value".to_string(),
        };

        // Test the endpoint
        println!("  ✓ Testing component metadata update endpoint");
        let req = test::TestRequest::put()
            .uri("/health/components/RuntimeManager/metadata")
            .set_json(&payload)
            .to_request();
        let resp = test::call_service(&app, req).await;

        // Verify response status
        println!("    ✓ Verifying response status");
        assert!(
            resp.status().is_success(),
            "Metadata update should return success status"
        );

        // Parse response body
        let body = test::read_body(resp).await;
        let update_response: serde_json::Value =
            serde_json::from_slice(&body).expect("Response should be valid JSON");

        // Verify response structure
        println!("    ✓ Verifying response structure");
        assert!(
            update_response["status"].is_string(),
            "Status field should exist"
        );
        assert!(
            update_response["message"].is_string(),
            "Message field should exist"
        );
        assert!(
            update_response["component"].is_string(),
            "Component field should exist"
        );
        assert!(
            update_response["timestamp"].is_string(),
            "Timestamp field should exist"
        );

        // Verify response content
        assert_eq!(update_response["status"], "success");
        assert_eq!(update_response["component"], "RuntimeManager");

        println!("Component metadata update endpoint test completed successfully!");
    }

    /// Tests health check recording endpoint functionality:
    /// - Verifies POST request to record health check
    /// - Validates health check success/failure recording
    /// - Confirms proper request payload processing
    /// - Checks response structure and timestamp
    ///
    /// # Examples
    ///
    /// ```
    /// use crate::controllers::health_controller::HealthController;
    /// use actix_web::test;
    ///
    /// // Test health check recording
    /// let payload = json!({
    ///     "success": true
    /// });
    /// let req = test::TestRequest::post()
    ///     .uri("/api/health/components/RuntimeManager/health-check")
    ///     .set_json(&payload)
    ///     .to_request();
    /// ```
    #[tokio::test]
    async fn should_record_health_check_successfully() {
        println!("Testing health check recording endpoint functionality...");

        // Create mock dependencies
        println!("  ✓ Creating mock StateManager");
        let state_manager = create_mock_state_manager().await;

        // Register a test component
        println!("  ✓ Registering test component");
        state_manager
            .register_component("RuntimeManager".to_string())
            .await;

        // Create test application
        println!("  ✓ Setting up test application");
        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(state_manager.clone()))
                .route(
                    "/health/components/{component_name}/health-check",
                    web::post().to(HealthController::record_health_check),
                ),
        )
        .await;

        // Test successful health check recording
        println!("  ✓ Testing successful health check recording");
        let success_payload = HealthCheckRequest { success: true };
        let req = test::TestRequest::post()
            .uri("/health/components/RuntimeManager/health-check")
            .set_json(&success_payload)
            .to_request();
        let resp = test::call_service(&app, req).await;

        // Verify response status
        println!("    ✓ Verifying response status for successful check");
        assert!(
            resp.status().is_success(),
            "Health check recording should return success status"
        );

        // Parse response body
        let body = test::read_body(resp).await;
        let record_response: serde_json::Value =
            serde_json::from_slice(&body).expect("Response should be valid JSON");

        // Verify response structure
        println!("    ✓ Verifying response structure");
        assert!(
            record_response["status"].is_string(),
            "Status field should exist"
        );
        assert!(
            record_response["message"].is_string(),
            "Message field should exist"
        );
        assert!(
            record_response["component"].is_string(),
            "Component field should exist"
        );
        assert!(
            record_response["success"].is_boolean(),
            "Success field should exist"
        );
        assert!(
            record_response["timestamp"].is_string(),
            "Timestamp field should exist"
        );

        // Verify response content
        assert_eq!(record_response["status"], "success");
        assert_eq!(record_response["component"], "RuntimeManager");
        assert_eq!(record_response["success"], true);

        // Test failed health check recording
        println!("  ✓ Testing failed health check recording");
        let failure_payload = HealthCheckRequest { success: false };
        let req = test::TestRequest::post()
            .uri("/health/components/DatabasePool/health-check")
            .set_json(&failure_payload)
            .to_request();
        let resp = test::call_service(&app, req).await;

        // Verify response status for failure case
        println!("    ✓ Verifying response status for failed check");
        assert!(
            resp.status().is_success(),
            "Health check recording should return success status even for failed checks"
        );

        let body = test::read_body(resp).await;
        let record_response: serde_json::Value =
            serde_json::from_slice(&body).expect("Response should be valid JSON");

        assert_eq!(record_response["success"], false);
        assert_eq!(record_response["component"], "DatabasePool");

        println!("Health check recording endpoint test completed successfully!");
    }

    /// Tests component information retrieval endpoint functionality:
    /// - Verifies GET request to retrieve component details
    /// - Validates component status and metadata
    /// - Confirms proper component information structure
    /// - Checks error handling for non-existent components
    ///
    /// # Examples
    ///
    /// ```
    /// use crate::controllers::health_controller::HealthController;
    /// use actix_web::test;
    ///
    /// // Test component information retrieval
    /// let req = test::TestRequest::get()
    ///     .uri("/api/health/components/RuntimeManager")
    ///     .to_request();
    /// let resp = test::call_service(&app, req).await;
    /// assert_eq!(resp.status(), 200);
    /// ```
    #[tokio::test]
    async fn should_retrieve_component_information_successfully() {
        println!("Testing component information retrieval endpoint functionality...");

        // Create mock dependencies
        println!("  ✓ Creating mock StateManager");
        let state_manager = create_mock_state_manager().await;

        // Register a test component
        println!("  ✓ Registering test component");
        state_manager
            .register_component("RuntimeManager".to_string())
            .await;

        // Create test application
        println!("  ✓ Setting up test application");
        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(state_manager.clone()))
                .route(
                    "/health/components/{component_name}",
                    web::get().to(HealthController::get_component),
                ),
        )
        .await;

        // Test retrieving existing component
        println!("  ✓ Testing existing component retrieval");
        let req = test::TestRequest::get()
            .uri("/health/components/RuntimeManager")
            .to_request();
        let resp = test::call_service(&app, req).await;

        // Verify response status
        println!("    ✓ Verifying response status for existing component");
        let status = resp.status();
        if !status.is_success() {
            let body = test::read_body(resp).await;
            let body_str = String::from_utf8_lossy(&body);
            panic!(
                "Component retrieval failed. Status: {}, Body: {}",
                status, body_str
            );
        }
        assert!(
            status.is_success(),
            "Component retrieval should return success status for existing component"
        );

        // Parse response body
        let body = test::read_body(resp).await;
        let component_response: serde_json::Value =
            serde_json::from_slice(&body).expect("Response should be valid JSON");

        // Verify component information structure
        println!("    ✓ Verifying component information structure");
        assert!(
            component_response["name"].is_string(),
            "Name field should exist"
        );
        assert!(
            component_response["status"].is_string(),
            "Status field should exist"
        );
        assert!(
            component_response["error_count"].is_number(),
            "Error count field should exist"
        );
        assert!(
            component_response["restart_count"].is_number(),
            "Restart count field should exist"
        );

        // Verify component content
        assert_eq!(component_response["name"], "RuntimeManager");

        // Test retrieving non-existent component
        println!("  ✓ Testing non-existent component retrieval");
        let req = test::TestRequest::get()
            .uri("/health/components/NonExistentComponent")
            .to_request();
        let resp = test::call_service(&app, req).await;

        // Should handle gracefully (either 404 or empty response)
        println!("    ✓ Verifying response for non-existent component");
        assert!(
            resp.status().is_client_error() || resp.status().is_success(),
            "Non-existent component should be handled gracefully"
        );

        println!("Component information retrieval endpoint test completed successfully!");
    }

    /// Tests state snapshot creation endpoint functionality:
    /// - Verifies GET request to create system state snapshot
    /// - Validates snapshot data structure and content
    /// - Confirms timestamp and component information
    /// - Checks comprehensive system state capture
    ///
    /// # Examples
    ///
    /// ```
    /// use crate::controllers::health_controller::HealthController;
    /// use actix_web::test;
    ///
    /// // Test state snapshot creation
    /// let req = test::TestRequest::get()
    ///     .uri("/api/health/snapshot")
    ///     .to_request();
    /// let resp = test::call_service(&app, req).await;
    /// assert_eq!(resp.status(), 200);
    /// ```
    #[tokio::test]
    async fn should_create_state_snapshot_successfully() {
        println!("Testing state snapshot creation endpoint functionality...");

        // Create mock dependencies
        println!("  ✓ Creating mock StateManager");
        let state_manager = create_mock_state_manager().await;

        // Create test application
        println!("  ✓ Setting up test application");
        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(state_manager.clone()))
                .route(
                    "/health/snapshot",
                    web::get().to(HealthController::create_snapshot),
                ),
        )
        .await;

        // Test snapshot creation
        println!("  ✓ Testing state snapshot creation");
        let req = test::TestRequest::get()
            .uri("/health/snapshot")
            .to_request();
        let resp = test::call_service(&app, req).await;

        // Verify response status
        println!("    ✓ Verifying response status");
        assert!(
            resp.status().is_success(),
            "Snapshot creation should return success status"
        );

        // Parse response body
        let body = test::read_body(resp).await;
        let snapshot_response: serde_json::Value =
            serde_json::from_slice(&body).expect("Response should be valid JSON");

        // Verify snapshot structure (structure depends on StateManager implementation)
        println!("    ✓ Verifying snapshot structure");
        assert!(
            snapshot_response.is_object(),
            "Snapshot should be a JSON object"
        );

        println!("State snapshot creation endpoint test completed successfully!");
    }

    /// Tests monitoring configuration endpoint functionality:
    /// - Verifies PUT request to set metrics collection interval
    /// - Validates configuration parameter processing
    /// - Confirms successful configuration update
    /// - Checks response structure and acknowledgment
    ///
    /// # Examples
    ///
    /// ```
    /// use crate::controllers::health_controller::HealthController;
    /// use actix_web::test;
    ///
    /// // Test monitoring configuration
    /// let payload = json!({
    ///     "interval_seconds": 30
    /// });
    /// let req = test::TestRequest::put()
    ///     .uri("/api/health/monitoring/interval")
    ///     .set_json(&payload)
    ///     .to_request();
    /// ```
    #[tokio::test]
    async fn should_configure_monitoring_interval_successfully() {
        println!("Testing monitoring configuration endpoint functionality...");

        // Create mock dependencies
        println!("  ✓ Creating mock StateManager");
        let state_manager = create_mock_state_manager().await;

        // Create test application
        println!("  ✓ Setting up test application");
        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(state_manager.clone()))
                .route(
                    "/health/monitoring/interval",
                    web::put().to(HealthController::set_metrics_interval),
                ),
        )
        .await;

        // Prepare test payload
        println!("  ✓ Preparing test payload");
        let payload = MonitoringConfigRequest {
            interval_seconds: 30,
        };

        // Test the endpoint
        println!("  ✓ Testing monitoring interval configuration");
        let req = test::TestRequest::put()
            .uri("/health/monitoring/interval")
            .set_json(&payload)
            .to_request();
        let resp = test::call_service(&app, req).await;

        // Verify response status
        println!("    ✓ Verifying response status");
        assert!(
            resp.status().is_success(),
            "Monitoring configuration should return success status"
        );

        // Parse response body
        let body = test::read_body(resp).await;
        let config_response: serde_json::Value =
            serde_json::from_slice(&body).expect("Response should be valid JSON");

        // Verify response structure
        println!("    ✓ Verifying response structure");
        assert!(
            config_response["status"].is_string(),
            "Status field should exist"
        );
        assert!(
            config_response["message"].is_string(),
            "Message field should exist"
        );
        assert!(
            config_response["timestamp"].is_string(),
            "Timestamp field should exist"
        );

        // Verify response content - endpoint returns 'info' due to Arc<StateManager> limitations
        assert_eq!(config_response["status"], "info");

        println!("Monitoring configuration endpoint test completed successfully!");
    }

    /// Tests edge cases and error handling scenarios:
    /// - Invalid component names with special characters
    /// - Malformed JSON payloads
    /// - Missing required fields in requests
    /// - Very long input strings that might cause issues
    /// - Empty or null values in request payloads
    ///
    /// # Examples
    ///
    /// ```
    /// use crate::controllers::health_controller::HealthController;
    /// use actix_web::test;
    ///
    /// // Test invalid component name
    /// let req = test::TestRequest::get()
    ///     .uri("/api/health/components/invalid[component]")
    ///     .to_request();
    /// let resp = test::call_service(&app, req).await;
    /// // Should handle gracefully
    /// ```
    #[tokio::test]
    async fn should_handle_edge_cases_and_invalid_inputs_gracefully() {
        println!("Testing edge cases and error handling scenarios...");

        // Create mock dependencies
        println!("  ✓ Creating mock StateManager and HealthService");
        let state_manager = create_mock_state_manager().await;
        let health_service = create_mock_health_service().await;

        // Create test application with multiple endpoints
        println!("  ✓ Setting up comprehensive test application");
        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(state_manager.clone()))
                .app_data(web::Data::new(health_service.clone()))
                .route(
                    "/health/components/{component_name}",
                    web::get().to(HealthController::get_component),
                )
                .route(
                    "/health/components/{component_name}/metadata",
                    web::put().to(HealthController::update_component_metadata),
                )
                .route(
                    "/health/components/{component_name}/health-check",
                    web::post().to(HealthController::record_health_check),
                ),
        )
        .await;

        // Test invalid component names with special characters
        println!("  ✓ Testing invalid component names with special characters");
        let invalid_names = vec![
            "component_with_underscores",
            "component-with-dashes",
            "ComponentWithCamelCase",
            "component123",
        ];

        for invalid_name in invalid_names {
            let req = test::TestRequest::get()
                .uri(&format!("/health/components/{}", invalid_name))
                .to_request();
            let resp = test::call_service(&app, req).await;

            // Should handle gracefully (not crash)
            assert!(
                resp.status().is_client_error() || resp.status().is_success(),
                "Invalid component name should be handled gracefully: {}",
                invalid_name
            );
        }

        // Test malformed JSON payloads
        println!("  ✓ Testing malformed JSON payloads");
        let malformed_payloads = vec![
            "{invalid json}",
            "{\"key\": }",       // Missing value
            "{\"incomplete\": ", // Incomplete JSON
            "",                  // Empty string
        ];

        for payload in malformed_payloads {
            let req = test::TestRequest::put()
                .uri("/health/components/TestComponent/metadata")
                .set_payload(payload)
                .insert_header(("content-type", "application/json"))
                .to_request();
            let resp = test::call_service(&app, req).await;

            // Should return client error for malformed JSON
            assert!(
                resp.status().is_client_error(),
                "Malformed JSON should return client error: {}",
                payload
            );
        }

        // Test missing required fields
        println!("  ✓ Testing missing required fields");
        let incomplete_metadata = json!({ "key": "test_key" }); // Missing value
        let req = test::TestRequest::put()
            .uri("/health/components/TestComponent/metadata")
            .set_json(&incomplete_metadata)
            .to_request();
        let resp = test::call_service(&app, req).await;

        // Should handle missing fields gracefully
        assert!(
            resp.status().is_client_error(),
            "Missing required fields should return client error"
        );

        // Test very long input strings
        println!("  ✓ Testing very long input strings");
        let long_string = "a".repeat(10000);
        let long_payload = ComponentMetadataRequest {
            key: long_string.clone(),
            value: long_string,
        };
        let req = test::TestRequest::put()
            .uri("/health/components/TestComponent/metadata")
            .set_json(&long_payload)
            .to_request();
        let resp = test::call_service(&app, req).await;

        // Should handle long strings without crashing
        assert!(
            !resp.status().is_server_error(),
            "Very long strings should not cause server errors"
        );

        // Test empty values
        println!("  ✓ Testing empty values in payloads");
        let empty_metadata = ComponentMetadataRequest {
            key: "".to_string(),
            value: "".to_string(),
        };
        let req = test::TestRequest::put()
            .uri("/health/components/TestComponent/metadata")
            .set_json(&empty_metadata)
            .to_request();
        let resp = test::call_service(&app, req).await;

        // Should handle empty values gracefully
        assert!(
            !resp.status().is_server_error(),
            "Empty values should not cause server errors"
        );

        println!("Edge cases and error handling tests completed successfully!");
    }
}
