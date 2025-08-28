use actix_web::web::ServiceConfig;
use actix_web::{web};
use crate::controllers::pg_functions::pg_listener_controller::{
    create_pg_function, pg_listener_delete, pg_listener_get, test_pg_function_syntax,
};
use crate::middlewares::auth_middleware::Authentication;
use crate::middlewares::shutdown_middleware::ShutdownGuard;
use crate::middlewares::session_middleware::SessionMiddleware;

pub fn configure_listener_routes(cfg: &mut ServiceConfig) {
    cfg.service(
        web::scope("/api/listener")
            .wrap(ShutdownGuard)
            .wrap(Authentication)
            .wrap(SessionMiddleware::default())
              .route("", web::get().to(pg_listener_get))
            .route("/function", web::post().to(create_pg_function))
            .route("/test", web::post().to(test_pg_function_syntax))
            .route("/{function_name}", web::delete().to(pg_listener_delete)),
    );
}