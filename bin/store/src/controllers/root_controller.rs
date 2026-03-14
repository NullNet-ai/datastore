use actix_web::{HttpMessage, HttpRequest, Responder};
use serde_json::Value;

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

// Password update function for accounts table - delegates to update_record
pub async fn root_update_account_password(
    auth: HttpRequest,
    path_params: actix_web::web::Path<String>,
    request_body: actix_web::web::Json<serde_json::Value>,
) -> impl Responder {
    let auth = extract_and_store_type(auth);

    // Extract account_id from path parameters - we expect just the account_id from /accounts/password/{account_id}
    let account_id = path_params.into_inner();
    // Extract password from request body
    let password = match request_body.get("password") {
        Some(Value::String(pwd)) => pwd.clone(),
        _ => {
            // For invalid password format, we'll let the update_record handle the validation
            // by passing an empty password which should fail validation downstream
            String::new()
        }
    };

    // Hash the password - if this fails, we'll still pass the error to update_record
    // which will handle it appropriately
    let hashed_password =
        match crate::providers::operations::auth::auth_service::password_hash(&password).await {
            Ok(hash) => hash,
            Err(e) => {
                log::error!("Failed to hash password: {}", e);
                // Pass the error to the underlying function to handle
                String::new()
            }
        };

    // Create update request for accounts table with account_secret field
    let update_request = actix_web::web::Json(crate::structs::core::RequestBody {
        record: serde_json::json!({
            "account_secret": hashed_password
        }),
    });

    // Use the existing root_update_record wrapper function
    // The path params should be (table_name, record_id) for the update_record function
    let update_path_params = actix_web::web::Path::from(("accounts".to_string(), account_id));
    let query = actix_web::web::Query(crate::structs::core::QueryParams {
        pluck: "id".to_string(),
    });

    root_update_record(auth, update_path_params, update_request, query).await
}

// Schema verification wrapper
create_root_wrapper!(root_verify_schema => verify_schema,
    auth: HttpRequest,
    request_body: actix_web::web::Json<crate::controllers::store_controller::SchemaVerificationRequest>
);

create_root_wrapper!(root_create_materialized_view => create_materialized_view,
    auth: HttpRequest,
    table: actix_web::web::Path<String>,
    request_body: actix_web::web::Json<serde_json::Value>
);

create_root_wrapper!(root_create_procedure => create_procedure,
    auth: HttpRequest,
    name: actix_web::web::Path<String>,
    request_body: actix_web::web::Json<serde_json::Value>
);

create_root_wrapper!(root_create_function => create_function,
    auth: HttpRequest,
    name: actix_web::web::Path<String>,
    request_body: actix_web::web::Json<serde_json::Value>
);

create_root_wrapper!(root_create_trigger => create_trigger,
    auth: HttpRequest,
    table: actix_web::web::Path<String>,
    request_body: actix_web::web::Json<serde_json::Value>
);
