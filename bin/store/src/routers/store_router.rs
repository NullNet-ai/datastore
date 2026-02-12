use crate::controllers::store_controller::{
    aggregation_filter, batch_delete_records, batch_insert_records, batch_update_records,
    count_by_filter, create_record, delete_record, get_by_filter, get_by_id, search_suggestions,
    switch_account, update_record, upsert,
};
use crate::middlewares::auth_middleware::Authentication;
use crate::middlewares::session_middleware::SessionMiddleware;
use crate::middlewares::shutdown_middleware::ShutdownGuard;
use crate::providers::storage::AppState;
use actix_web::web;
use actix_web::web::ServiceConfig;

/// Configure store routes
///
/// This function sets up all routes for the main store API including:
/// - CRUD operations (create, read, update, delete)
/// - Batch operations
/// - Filtering and search
/// - Account switching
/// - Aggregation operations
pub fn configure_store_routes(cfg: &mut ServiceConfig, app_state: AppState) {
    cfg.service(
        web::scope("/api/store")
            .app_data(web::Data::new(app_state))
            .wrap(ShutdownGuard)
            .wrap(Authentication)
            .wrap(SessionMiddleware)
            .route("/aggregate", web::post().to(aggregation_filter))
            .route("/{table}", web::post().to(create_record))
            .route("/upsert/{table}", web::post().to(upsert))
            .route("/batch/{table}", web::patch().to(batch_update_records))
            .route("/batch/{table}", web::delete().to(batch_delete_records))
            .route("/{table}/filter", web::post().to(get_by_filter))
            .route("/{table}/count", web::post().to(count_by_filter))
            .route("/{table}/{id}", web::get().to(get_by_id))
            .route("/{table}/{id}", web::patch().to(update_record))
            .route("/{table}/{id}", web::delete().to(delete_record))
            .route("/batch/{table}", web::post().to(batch_insert_records))
            .route("/switch_account", web::post().to(switch_account))
            .route(
                "/{table}/filter/suggestions",
                web::post().to(search_suggestions),
            ),
    );
}
