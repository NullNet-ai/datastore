use crate::controllers::root_controller::{
    root_aggregation_filter, root_batch_delete_records, root_batch_insert_records,
    root_batch_update_records, root_call_function, root_call_procedure, root_count_by_filter,
    root_create_function, root_create_materialized_view, root_create_procedure, root_create_record,
    root_create_trigger, root_cron_schedule_job, root_delete_function,
    root_delete_materialized_view, root_delete_procedure, root_delete_record, root_delete_trigger,
    root_get_by_filter, root_get_by_id, root_search_suggestions, root_switch_account,
    root_update_account_password, root_update_record, root_upsert, root_verify_schema,
    root_unsafe_select_query, root_unsafe_transaction_query,
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
            .route("/switch_account", web::post().to(root_switch_account))
            .route("/verify_schema", web::post().to(root_verify_schema))
            .route("/unsafe_select_query", web::post().to(root_unsafe_select_query))
            .route("/unsafe_transaction_query", web::post().to(root_unsafe_transaction_query))
            .route(
                "/materialized_view/{table}",
                web::post().to(root_create_materialized_view),
            )
            .route(
                "/materialized_view/{table}",
                web::delete().to(root_delete_materialized_view),
            )
            .route("/procedure/{name}", web::post().to(root_create_procedure))
            .route("/procedure/{name}", web::delete().to(root_delete_procedure))
            .route(
                "/procedure/call/{name}",
                web::post().to(root_call_procedure),
            )
            .route("/function/{name}", web::post().to(root_create_function))
            .route("/function/{name}", web::delete().to(root_delete_function))
            .route("/function/call/{name}", web::post().to(root_call_function))
            .route("/trigger/{table}", web::post().to(root_create_trigger))
            .route(
                "/trigger/{table}/{name}",
                web::delete().to(root_delete_trigger),
            )
            .route("/cron-schedule-job", web::post().to(root_cron_schedule_job))
            .route("/{table}", web::post().to(root_create_record))
            .route("/upsert/{table}", web::post().to(root_upsert))
            .route("/batch/{table}", web::patch().to(root_batch_update_records))
            .route(
                "/batch/{table}",
                web::delete().to(root_batch_delete_records),
            )
            .route("/{table}/filter", web::post().to(root_get_by_filter))
            .route("/{table}/count", web::post().to(root_count_by_filter))
            .route("/{table}/{id}", web::get().to(root_get_by_id))
            .route("/{table}/{id}", web::patch().to(root_update_record))
            .route("/{table}/{id}", web::delete().to(root_delete_record))
            .route("/batch/{table}", web::post().to(root_batch_insert_records))
            .route(
                "/{table}/filter/suggestions",
                web::post().to(root_search_suggestions),
            ),
    );
}
