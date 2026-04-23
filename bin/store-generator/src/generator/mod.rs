//! Entry point for schema generation.

use crate::builders::generator::generator_service::GeneratorService;

/// Run schema generation (CREATE_SCHEMA flow).
pub fn run_schema() -> Result<(), String> {
    GeneratorService::run()
}
