use crate::diesel::DbPool;
use crate::providers;
use crate::types::controller_types::CreateQuery;
use actix_web::{web::{self, Path, ServiceConfig}, HttpResponse, Responder};
use providers::mutation_provider;
use serde::Deserialize;
use serde_json::Value;
use std::collections::HashMap;

#[derive(Deserialize)]
pub struct PathParams {
    table: String,
    id: String,
    record: HashMap<String, Value>,
}

pub async fn get(path: Path<PathParams>) -> impl Responder {
    let params = path.into_inner();
    HttpResponse::Ok().body(format!("Get from table: {}, id: {}", params.table, params.id))
}

pub async fn create(
    path: web::Path<String>,
    query: web::Query<CreateQuery>,
    record: web::Json<Value>,
    db_pool: web::Data<DbPool>,
) -> impl Responder {
    let table = path.into_inner();
    let pluck = query.pluck.clone().unwrap_or_else(|| "id".to_string());
    let table_name = table.clone();

    let result = db_pool
        .get()
        .await
        .expect("Failed to get DB connection")
        .interact(move |conn| mutation_provider::create(conn, &table, &record))
        .await;

    match result {
        Ok(Ok(_)) => HttpResponse::Ok().json(format!("Record inserted into '{}'", table_name)),
        Ok(Err(e)) => HttpResponse::InternalServerError().json(format!("Failed to insert record: {}", e)),
        Err(e) => HttpResponse::InternalServerError().json(format!("Failed to interact with database: {}", e)),
    }
}

pub fn init_routes(cfg: &mut ServiceConfig) {
    cfg.service(
        web::scope("/{table}/{id}")
            .route("", web::get().to(get))
    );
    cfg.service(
        web::scope("/{table}")
            .route("", web::post().to(create))
    );
}