use crate::controllers::sync_endpoints_controller::{create_endpoint, get_sync_endpoints};
use actix_web::web::ServiceConfig;

pub fn configure_sync_routes(cfg: &mut ServiceConfig) {
    cfg.service(get_sync_endpoints).service(create_endpoint);
}
