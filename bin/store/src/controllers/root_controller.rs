use crate::controllers::store_controller::{
    aggregation_filter, batch_delete_records, batch_insert_records, batch_update_records,
    create_record, delete_record, get_by_filter, get_by_id, update_record, upsert,
};
use crate::permissions::permission_decorator::PermissionExtractor;
use crate::structs::structs::{
    AggregationFilter, BatchUpdateBody, GetByFilter, QueryParams, RequestBody, UpsertRequestBody,
};
use crate::db;
use actix_web::{web, HttpRequest, Responder, HttpMessage};

// Wrapper functions that extract the type parameter and delegate to the original functions

pub async fn root_create_record(
    permissions: PermissionExtractor,
    request: HttpRequest,
    pool: web::Data<db::AsyncDbPool>,
    path_params: web::Path<(String, String)>, // (type, table)
    query: web::Query<QueryParams>,
) -> impl Responder {
    let (controller_type, table) = path_params.into_inner();
    
    // Store the controller type in request extensions for potential future use
    request.extensions_mut().insert(controller_type);
    
    // Delegate to the original function
    create_record(
        permissions,
        request,
        pool,
        web::Path::from(table),
        query,
    )
    .await
}

pub async fn root_update_record(
    auth: HttpRequest,
    pool: web::Data<db::AsyncDbPool>,
    path_params: web::Path<(String, String, String)>, // (type, table, id)
    request: web::Json<RequestBody>,
    query: web::Query<QueryParams>,
) -> impl Responder {
    let (controller_type, table, id) = path_params.into_inner();
    
    // Store the controller type in request extensions for potential future use
    auth.extensions_mut().insert(controller_type);
    
    // Delegate to the original function
    update_record(
        auth,
        pool,
        web::Path::from((table, id)),
        request,
        query,
    )
    .await
}

pub async fn root_get_by_id(
    auth: HttpRequest,
    pool: web::Data<db::AsyncDbPool>,
    path_params: web::Path<(String, String, String)>, // (type, table, id)
    query: web::Query<QueryParams>,
) -> impl Responder {
    let (controller_type, table, id) = path_params.into_inner();
    
    // Store the controller type in request extensions for potential future use
    auth.extensions_mut().insert(controller_type);
    
    // Delegate to the original function
    get_by_id(
        auth,
        pool,
        web::Path::from((table, id)),
        query,
    )
    .await
}

pub async fn root_delete_record(
    auth: HttpRequest,
    pool: web::Data<db::AsyncDbPool>,
    path_params: web::Path<(String, String, String)>, // (type, table, id)
) -> impl Responder {
    let (controller_type, table, id) = path_params.into_inner();
    
    // Store the controller type in request extensions for potential future use
    auth.extensions_mut().insert(controller_type);
    
    // Delegate to the original function
    delete_record(
        auth,
        pool,
        web::Path::from((table, id)),
    )
    .await
}

pub async fn root_batch_insert_records(
    auth: HttpRequest,
    path_params: web::Path<(String, String)>, // (type, table)
    records: web::Json<crate::controllers::store_controller::BatchInsertBody>,
) -> impl Responder {
    let (controller_type, table) = path_params.into_inner();
    
    // Store the controller type in request extensions for potential future use
    auth.extensions_mut().insert(controller_type);
    
    // Delegate to the original function
    batch_insert_records(
        auth,
        web::Path::from(table),
        records,
    )
    .await
}

pub async fn root_batch_update_records(
    auth: HttpRequest,
    pool: web::Data<db::AsyncDbPool>,
    path_params: web::Path<(String, String)>, // (type, table)
    request: web::Json<BatchUpdateBody>,
) -> impl Responder {
    let (controller_type, table) = path_params.into_inner();
    
    // Store the controller type in request extensions for potential future use
    auth.extensions_mut().insert(controller_type);
    
    // Delegate to the original function
    batch_update_records(
        auth,
        pool,
        web::Path::from(table),
        request,
    )
    .await
}

pub async fn root_batch_delete_records(
    auth: HttpRequest,
    pool: web::Data<db::AsyncDbPool>,
    path_params: web::Path<(String, String)>, // (type, table)
    request: web::Json<BatchUpdateBody>,
) -> impl Responder {
    let (controller_type, table) = path_params.into_inner();
    
    // Store the controller type in request extensions for potential future use
    auth.extensions_mut().insert(controller_type);
    
    // Delegate to the original function
    batch_delete_records(
        auth,
        pool,
        web::Path::from(table),
        request,
    )
    .await
}

pub async fn root_get_by_filter(
    auth: HttpRequest,
    pool: web::Data<db::AsyncDbPool>,
    path_params: web::Path<(String, String)>, // (type, table)
    request_body: web::Json<GetByFilter>,
) -> impl Responder {
    let (controller_type, table) = path_params.into_inner();
    
    // Store the controller type in request extensions for potential future use
    auth.extensions_mut().insert(controller_type);
    
    // Delegate to the original function
    get_by_filter(
        auth,
        pool,
        web::Path::from(table),
        request_body,
    )
    .await
}

pub async fn root_upsert(
    auth: HttpRequest,
    pool: web::Data<db::AsyncDbPool>,
    path_params: web::Path<(String, String)>, // (type, table)
    request_body: web::Json<UpsertRequestBody>,
    query: web::Query<QueryParams>,
) -> impl Responder {
    let (controller_type, table) = path_params.into_inner();
    
    // Store the controller type in request extensions for potential future use
    auth.extensions_mut().insert(controller_type);
    
    // Delegate to the original function
    upsert(
        auth,
        pool,
        web::Path::from(table),
        request_body,
        query,
    )
    .await
}

pub async fn root_aggregation_filter(
    auth: HttpRequest,
    path_params: web::Path<String>, // type
    request_body: web::Json<AggregationFilter>,
) -> impl Responder {
    let controller_type = path_params.into_inner();
    
    // Store the controller type in request extensions for potential future use
    auth.extensions_mut().insert(controller_type);
    
    // Delegate to the original function
    aggregation_filter(auth, request_body).await
}

// Helper function to get the controller type from request extensions
pub fn get_controller_type(request: &HttpRequest) -> Option<String> {
    request.extensions().get::<String>().cloned()
}

// Helper function to check if the request is coming from root controller
pub fn is_root_controller(request: &HttpRequest) -> bool {
    get_controller_type(request)
        .map(|controller_type| controller_type == "root")
        .unwrap_or(false)
}

// Helper function to check if the request is coming from none/default controller
pub fn is_none_controller(request: &HttpRequest) -> bool {
    get_controller_type(request)
        .map(|controller_type| controller_type == "none")
        .unwrap_or(true) // Default to true if no type is set (original controller)
}