use actix_web::{web::{self, Path, ServiceConfig}, HttpResponse, Responder};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct PathParams {
    table: String,
    id: String,
}

pub async fn get(path: Path<PathParams>) -> impl Responder {
    let params = path.into_inner();
    HttpResponse::Ok().body(format!("Get from table: {}, id: {}", params.table, params.id))
}

pub async fn create_user() -> impl Responder {
    HttpResponse::Ok().body("Create user")
}

pub fn init_routes(cfg: &mut ServiceConfig) {
    cfg.service(
        web::scope("/{table}/{id}")
            .route("", web::get().to(get))
    );
}