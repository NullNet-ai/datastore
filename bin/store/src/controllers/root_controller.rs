use actix_web::{HttpMessage, HttpRequest};

fn extract_and_store_type(req: HttpRequest) -> HttpRequest {
    // Since we're using /api/store/root, hardcode the type as 'root'
    let controller_type: Option<String> = Some("root".to_string());
    req.extensions_mut().insert(controller_type);
    req
}

macro_rules! create_root_wrapper {
    // Standardized pattern: HttpRequest is always first parameter
    ($root_name:ident => $original_name:ident, auth: HttpRequest $(, $param:ident: $param_type:ty)*) => {
        pub async fn $root_name(
            auth: HttpRequest,
            $($param: $param_type,)*
        ) -> impl actix_web::Responder {
            let auth = extract_and_store_type(auth);
            crate::controllers::store_controller::$original_name(auth, $($param,)*).await
        }
    };
}

// Generate wrapper functions using the macro
create_root_wrapper!(root_create_record => create_record,
    auth: HttpRequest,
    table: actix_web::web::Path<String>,
    body: actix_web::web::Json<serde_json::Value>,
    query: actix_web::web::Query<crate::structs::core::QueryParams>,
    app_state: Option<actix_web::web::Data<crate::providers::storage::AppState>>
);

create_root_wrapper!(root_update_record => update_record,
    auth: HttpRequest,
    path_params: actix_web::web::Path<(String, String)>,
    request: actix_web::web::Json<crate::structs::core::RequestBody>,
    query: actix_web::web::Query<crate::structs::core::QueryParams>
);

create_root_wrapper!(root_get_by_id => get_by_id,
    auth: HttpRequest,
    path_params: actix_web::web::Path<(String, String)>,
    query: actix_web::web::Query<crate::structs::core::QueryParams>
);

create_root_wrapper!(root_batch_insert_records => batch_insert_records,
    auth: HttpRequest,
    table: actix_web::web::Path<String>,
    records: actix_web::web::Json<crate::controllers::store_controller::BatchInsertBody>
);

create_root_wrapper!(root_batch_update_records => batch_update_records,
    auth: HttpRequest,
    table: actix_web::web::Path<String>,
    request: actix_web::web::Json<crate::structs::core::BatchUpdateBody>
);

create_root_wrapper!(root_delete_record => delete_record,
    auth: HttpRequest,
    path_params: actix_web::web::Path<(String, String)>,
    query: actix_web::web::Query<crate::structs::core::QueryParams>
);

create_root_wrapper!(root_batch_delete_records => batch_delete_records,
    auth: HttpRequest,
    table: actix_web::web::Path<String>,
    body: actix_web::web::Json<crate::structs::core::BatchUpdateBody>
);

create_root_wrapper!(root_get_by_filter => get_by_filter,
    auth: HttpRequest,
    table: actix_web::web::Path<String>,
    request_body: actix_web::web::Json<crate::structs::core::GetByFilter>
);

create_root_wrapper!(root_count_by_filter => count_by_filter,
    auth: HttpRequest,
    table: actix_web::web::Path<String>,
    request_body: actix_web::web::Json<crate::structs::core::GetByFilter>
);

create_root_wrapper!(root_upsert => upsert,
    auth: HttpRequest,
    table: actix_web::web::Path<String>,
    body: actix_web::web::Json<crate::structs::core::UpsertRequestBody>,
    query: actix_web::web::Query<crate::structs::core::QueryParams>
);

create_root_wrapper!(root_aggregation_filter => aggregation_filter,
    auth: HttpRequest,
    request_body: actix_web::web::Json<crate::structs::core::AggregationFilter>
);

// Note: switch_account doesn't follow the standard pattern with HttpRequest
pub async fn root_switch_account(
    request_body: actix_web::web::Json<crate::structs::core::SwitchAccountRequest>,
) -> impl actix_web::Responder {
    crate::controllers::store_controller::switch_account(request_body).await
}

create_root_wrapper!(root_search_suggestions => search_suggestions,
    auth: HttpRequest,
    table: actix_web::web::Path<String>,
    request_body: actix_web::web::Json<crate::structs::core::SearchSuggestionParams>
);
