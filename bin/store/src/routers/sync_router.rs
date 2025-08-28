use actix_web::web::ServiceConfig;
use crate::controllers::sync_endpoints_controller::{get_sync_endpoints, create_endpoint};

pub fn configure_sync_routes(cfg: &mut ServiceConfig) {
   cfg.service(get_sync_endpoints).service(create_endpoint);
}
