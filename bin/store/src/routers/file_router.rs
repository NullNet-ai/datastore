use crate::controllers::store_controller::{download_file_by_id, get_file_by_id, upload_file};
use crate::middlewares::auth_middleware::Authentication;
use crate::middlewares::session_middleware::SessionMiddleware;
use crate::middlewares::shutdown_middleware::ShutdownGuard;
use crate::providers::storage::minio::AppState;
use actix_web::web;
use actix_web::web::ServiceConfig;

pub fn configure_file_routes(cfg: &mut ServiceConfig, app_state: AppState) {
    cfg.service(
        web::scope("/api/file")
            .app_data(web::Data::new(app_state))
            .wrap(ShutdownGuard)
            .wrap(Authentication)
            .wrap(SessionMiddleware::default())
            .app_data(
                web::PayloadConfig::new(50 * 1024 * 1024), // 50MB limit for JSON
            )
            .app_data(
                web::FormConfig::default().limit(100 * 1024 * 1024), // 100MB limit for form data
            )
            .route("/download/{file_id}", web::get().to(download_file_by_id))
            .route("/get/{file_id}", web::get().to(get_file_by_id))
            .route("/upload", web::post().to(upload_file)),
    );
}
