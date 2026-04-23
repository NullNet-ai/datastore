//! Global test initialization module
//!
//! This module provides centralized test initialization to ensure all tests
//! have proper path configuration and other global state set up.

use std::sync::Once;

static INIT: Once = Once::new();

/// Initialize global test state. This should be called at the start of any test
/// that depends on path configuration or other global state.
pub fn init_test_state() {
    INIT.call_once(|| {
        // Initialize path configuration with empty args (no --init-db flag)
        crate::constants::paths::init_path_config(&[]);

        // Add any other global test initialization here as needed
    });
}

/// Test helper macro that automatically initializes test state
#[macro_export]
macro_rules! test {
    ($name:ident, $body:expr) => {
        #[test]
        fn $name() {
            $crate::test_init::init_test_state();
            $body
        }
    };
    ($name:ident, async $body:expr) => {
        #[tokio::test]
        async fn $name() {
            $crate::test_init::init_test_state();
            $body
        }
    };
}
