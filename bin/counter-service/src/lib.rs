//! Counter service: gRPC service and client for generating unique codes via Redis.

pub mod redis_code;
pub mod server;

// Generated gRPC types and client (from proto/code_service.proto via build.rs → src/generated/)
pub mod generated {
    include!("generated/code_service.rs");
}

pub use generated::code_service_client::CodeServiceClient;
pub use generated::{
    CounterConfig, GetCodeRequest, GetCodeResponse, InitCountersRequest, InitCountersResponse,
};
