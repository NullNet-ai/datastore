use crate::config::core::EnvConfig;
use actix_web::{HttpMessage, HttpRequest, HttpResponse, Responder};
use reqwest;
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

create_root_wrapper!(root_call_procedure => call_procedure,
    auth: HttpRequest,
    name: actix_web::web::Path<String>,
    request_body: actix_web::web::Json<serde_json::Value>
);

create_root_wrapper!(root_call_function => call_function,
    auth: HttpRequest,
    name: actix_web::web::Path<String>,
    request_body: actix_web::web::Json<serde_json::Value>
);

create_root_wrapper!(root_create_trigger => create_trigger,
    auth: HttpRequest,
    table: actix_web::web::Path<String>,
    request_body: actix_web::web::Json<serde_json::Value>
);

create_root_wrapper!(root_list_triggers => list_triggers,
    auth: HttpRequest,
    query: actix_web::web::Query<crate::controllers::store_controller::TriggerListQuery>
);

create_root_wrapper!(root_cron_schedule_job => cron_schedule_job,
    auth: HttpRequest,
    request_body: actix_web::web::Json<serde_json::Value>
);

create_root_wrapper!(root_unsafe_select_query => unsafe_select_query,
    auth: HttpRequest,
    request_body: actix_web::web::Json<serde_json::Value>
);

create_root_wrapper!(root_unsafe_transaction_query => unsafe_transaction_query,
    auth: HttpRequest,
    request_body: actix_web::web::Json<serde_json::Value>
);

create_root_wrapper!(root_delete_materialized_view => delete_materialized_view,
    auth: HttpRequest,
    table: actix_web::web::Path<String>
);

create_root_wrapper!(root_delete_procedure => delete_procedure,
    auth: HttpRequest,
    name: actix_web::web::Path<String>,
    request_body: actix_web::web::Json<serde_json::Value>
);

create_root_wrapper!(root_delete_function => delete_function,
    auth: HttpRequest,
    name: actix_web::web::Path<String>,
    request_body: actix_web::web::Json<serde_json::Value>
);

create_root_wrapper!(root_delete_trigger => delete_trigger,
    auth: HttpRequest,
    path: actix_web::web::Path<(String, String)>
);
pub async fn root_prometheus_queries() -> impl Responder {
    let body = serde_json::json!({
        "api": {
            "throughput": "sum(rate(http_server_request_duration_count[1m]))",
            "errors": "(sum(rate(http_server_request_duration_count{http_response_status_code=~\\\"5..\\\"}[5m])))/(sum(rate(http_server_request_duration_count[5m])))",
            "errors_by_code": "sum by (http_response_status_code)(rate(http_server_request_duration_count{http_response_status_code=~\\\"5..\\\"}[5m]))",
            "client_errors": "(sum(rate(http_server_request_duration_count{http_response_status_code=~\\\"4..\\\"}[5m])))/(sum(rate(http_server_request_duration_count[5m])))",
            "client_errors_by_code": "sum by (http_response_status_code)(rate(http_server_request_duration_count{http_response_status_code=~\\\"4..\\\"}[5m]))"
        },
        "logging": {
            "writes_debug": "rate(logs_written_total_debug[5m])",
            "writes_info": "rate(logs_written_total_info[5m])",
            "writes_warn": "rate(logs_written_total_warn[5m])",
            "writes_error": "rate(logs_written_total_error[5m])",
            "writes_trace": "rate(logs_written_total_trace[5m])",
            "writes_critical": "rate(logs_written_total_critical[5m])",
            "write_errors": "rate(log_write_failed_total[5m])",
            "file_size_bytes": "log_file_size_bytes"
        },
        "system": {
            "cpu_usage": "rate(process_cpu_seconds_total[1m])",
            "memory_usage": "process_resident_memory_bytes"
        }
    });
    HttpResponse::Ok().json(body)
}

async fn query_prometheus_sum(base_url: &str, query: &str) -> Option<f64> {
    let url = format!("{}/api/v1/query", base_url);
    let client = reqwest::Client::new();
    let resp = client
        .get(url)
        .query(&[("query", query)])
        .send()
        .await
        .ok()?;
    let json: serde_json::Value = resp.json().await.ok()?;
    let result = json.get("data")?.get("result")?;
    let mut sum = 0.0;
    if let Some(arr) = result.as_array() {
        for item in arr {
            if let Some(value) = item.get("value") {
                if let Some(v_arr) = value.as_array() {
                    if let Some(v_str) = v_arr.get(1).and_then(|v| v.as_str()) {
                        if let Ok(v) = v_str.parse::<f64>() {
                            sum += v;
                        }
                    }
                }
            }
        }
        Some(sum)
    } else {
        None
    }
}

async fn query_prometheus_group_sum(
    base_url: &str,
    query: &str,
    group_label: &str,
) -> std::collections::BTreeMap<String, f64> {
    let url = format!("{}/api/v1/query", base_url);
    let client = reqwest::Client::new();
    let mut map = std::collections::BTreeMap::new();
    if let Ok(resp) = client.get(url).query(&[("query", query)]).send().await {
        if let Ok(json) = resp.json::<serde_json::Value>().await {
            if let Some(arr) = json
                .get("data")
                .and_then(|d| d.get("result"))
                .and_then(|r| r.as_array())
            {
                for item in arr {
                    let key = item
                        .get("metric")
                        .and_then(|m| m.get(group_label))
                        .and_then(|v| v.as_str())
                        .unwrap_or("unknown")
                        .to_string();
                    let val = item
                        .get("value")
                        .and_then(|v| v.as_array())
                        .and_then(|a| a.get(1))
                        .and_then(|v| v.as_str())
                        .and_then(|s| s.parse::<f64>().ok())
                        .unwrap_or(0.0);
                    let entry = map.entry(key).or_insert(0.0);
                    *entry += val;
                }
            }
        }
    }
    map
}

pub async fn root_prometheus_results() -> impl Responder {
    let base_url = EnvConfig::default().prometheus_base_url;
    let throughput_q = "sum(rate(http_server_request_duration_count[1m]))";
    let errors_q = "(sum(rate(http_server_request_duration_count{http_response_status_code=~\"5..\"}[5m])))/(sum(rate(http_server_request_duration_count[5m])))";
    let client_errors_q = "(sum(rate(http_server_request_duration_count{http_response_status_code=~\"4..\"}[5m])))/(sum(rate(http_server_request_duration_count[5m])))";
    let cpu_q = "rate(process_cpu_seconds_total[1m])";
    let mem_q = "process_resident_memory_bytes";
    let writes_debug_q = "rate(logs_written_total_debug[5m])";
    let writes_info_q = "rate(logs_written_total_info[5m])";
    let writes_warn_q = "rate(logs_written_total_warn[5m])";
    let writes_error_q = "rate(logs_written_total_error[5m])";
    let writes_trace_q = "rate(logs_written_total_trace[5m])";
    let writes_critical_q = "rate(logs_written_total_critical[5m])";
    let write_errors_q = "rate(log_write_failed_total[5m])";
    let file_size_bytes_q = "log_file_size_bytes";

    let throughput = query_prometheus_sum(&base_url, throughput_q).await;
    let errors = query_prometheus_sum(&base_url, errors_q).await;
    let client_errors = query_prometheus_sum(&base_url, client_errors_q).await;
    let cpu = query_prometheus_sum(&base_url, cpu_q).await;
    let memory = query_prometheus_sum(&base_url, mem_q).await;
    let writes_debug = query_prometheus_sum(&base_url, writes_debug_q).await;
    let writes_info = query_prometheus_sum(&base_url, writes_info_q).await;
    let writes_warn = query_prometheus_sum(&base_url, writes_warn_q).await;
    let writes_error = query_prometheus_sum(&base_url, writes_error_q).await;
    let writes_trace = query_prometheus_sum(&base_url, writes_trace_q).await;
    let writes_critical = query_prometheus_sum(&base_url, writes_critical_q).await;
    let write_errors = query_prometheus_sum(&base_url, write_errors_q).await;
    let file_size_bytes = query_prometheus_sum(&base_url, file_size_bytes_q).await;

    let errors_by_code = query_prometheus_group_sum(
        &base_url,
        "sum by (http_response_status_code)(rate(http_server_request_duration_count{http_response_status_code=~\"5..\"}[5m]))",
        "http_response_status_code",
    )
    .await;

    let client_errors_by_code = query_prometheus_group_sum(
        &base_url,
        "sum by (http_response_status_code)(rate(http_server_request_duration_count{http_response_status_code=~\"4..\"}[5m]))",
        "http_response_status_code",
    )
    .await;

    let body = serde_json::json!({
        "api": {
            "throughput": throughput,
            "errors": errors,
            "errors_by_code": errors_by_code,
            "client_errors": client_errors,
            "client_errors_by_code": client_errors_by_code
        },
        "system": {
            "cpu_usage": cpu,
            "memory_usage": memory
        },
        "logging": {
            "writes_debug": writes_debug,
            "writes_info": writes_info,
            "writes_warn": writes_warn,
            "writes_error": writes_error,
            "writes_trace": writes_trace,
            "writes_critical": writes_critical,
            "write_errors": write_errors,
            "file_size_bytes": file_size_bytes
        }
    });
    HttpResponse::Ok().json(body)
}
