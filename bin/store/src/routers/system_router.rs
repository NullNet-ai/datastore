use crate::controllers::initializer_controller::{
    run_background_services_config, run_generate_schema_config, run_global_organization_config,
    run_initial_entity_data_config, run_initializers, run_root_account_config,
    run_system_code_config, run_system_device_config,
};
use crate::middlewares::auth_middleware::Authentication;
// use crate::middlewares::session_middleware::SessionMiddleware;
use crate::middlewares::shutdown_middleware::ShutdownGuard;
use actix_web::web;
use actix_web::web::ServiceConfig;

pub fn configure_system_routes(cfg: &mut ServiceConfig) {
    cfg.service(
        web::scope("/api/system")
            .wrap(ShutdownGuard)
            .wrap(Authentication)
            // .wrap(SessionMiddleware)
            .route("/initialize_all", web::post().to(run_initializers))
            .route(
                "/initialize/system_code_config",
                web::post().to(run_system_code_config),
            )
            .route(
                "/initialize/root_account_config",
                web::post().to(run_root_account_config),
            )
            .route(
                "/initialize/global_organization_config",
                web::post().to(run_global_organization_config),
            )
            .route(
                "/initialize/system_device_config",
                web::post().to(run_system_device_config),
            )
            .route(
                "/initialize/background_services_config",
                web::post().to(run_background_services_config),
            )
            .route(
                "/initialize/initial_entity_data_config",
                web::post().to(run_initial_entity_data_config),
            )
            .route(
                "/initialize/generate_schema_config",
                web::post().to(run_generate_schema_config),
            ),
    );
}
