pub mod organizations_router;
pub mod store_router;
pub mod root_store_router;
pub mod listener_router;
pub mod file_router;
pub mod sync_router;

// Re-export the configuration functions for easy access
pub use organizations_router::{configure_organizations_routes, configure_token_routes};
pub use store_router::configure_store_routes;
pub use root_store_router::configure_root_store_routes;
pub use listener_router::configure_listener_routes;
pub use file_router::configure_file_routes;
pub use sync_router::configure_sync_routes;