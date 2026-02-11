use crate::controllers::root_controller::{
    root_aggregation_filter, root_batch_delete_records, root_batch_insert_records,
    root_batch_update_records, root_create_record, root_delete_record, root_get_by_filter,
    root_get_by_id, root_search_suggestions, root_switch_account, root_update_account_password,
    root_update_record, root_upsert,
};
use crate::middlewares::auth_middleware::Authentication;
use crate::middlewares::session_middleware::SessionMiddleware;
use crate::middlewares::shutdown_middleware::ShutdownGuard;
use actix_web::web;
use actix_web::web::ServiceConfig;

pub fn configure_root_store_routes(cfg: &mut ServiceConfig) {
    cfg.service(
        web::scope("/api/store/root")
            .wrap(ShutdownGuard)
            .wrap(Authentication)
            .wrap(SessionMiddleware::default())
            .route(
                "/accounts/password/{account_id}",
                web::patch().to(root_update_account_password),
            )
            .route("/aggregate", web::post().to(root_aggregation_filter))
            .route("/{table}", web::post().to(root_create_record))
            .route("/upsert/{table}", web::post().to(root_upsert))
            .route("/batch/{table}", web::patch().to(root_batch_update_records))
            .route(
                "/batch/{table}",
                web::delete().to(root_batch_delete_records),
            )
            .route("/{table}/filter", web::post().to(root_get_by_filter))
            .route("/{table}/{id}", web::get().to(root_get_by_id))
            .route("/{table}/{id}", web::patch().to(root_update_record))
            .route("/{table}/{id}", web::delete().to(root_delete_record))
            .route("/batch/{table}", web::post().to(root_batch_insert_records))
            .route("/switch_account", web::post().to(root_switch_account))
            .route(
                "/{table}/filter/suggestions",
                web::post().to(root_search_suggestions),
            ),
    );
}
