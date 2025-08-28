use crate::controllers::organization_controller::OrganizationsController;
use crate::middlewares::session_middleware::SessionMiddleware;
use actix_web::web;
use actix_web::web::ServiceConfig;

pub fn configure_organizations_routes(cfg: &mut ServiceConfig) {
    cfg.service(
        web::scope("/api/organizations")
            .wrap(SessionMiddleware)
            .route(
                "/register",
                web::post().to(OrganizationsController::register),
            )
            .route(
                "/register/{id}",
                web::put().to(OrganizationsController::reregister_existing_account),
            )
            .route("/auth", web::post().to(OrganizationsController::auth))
            .route("/logout", web::post().to(OrganizationsController::logout)),
    );
}

pub fn configure_token_routes(cfg: &mut ServiceConfig) {
    cfg.service(
        web::scope("/api/token")
            .wrap(SessionMiddleware::default())
            .route(
                "/verify",
                web::post().to(OrganizationsController::verify_token),
            ),
    );
}
