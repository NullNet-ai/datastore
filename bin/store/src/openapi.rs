#![allow(dead_code)]

use serde::{Deserialize, Serialize};
use utoipa::openapi::security::{HttpAuthScheme, HttpBuilder, SecurityScheme};
use utoipa::{Modify, OpenApi, ToSchema};

#[derive(Serialize, Deserialize, ToSchema)]
struct StandardOkResponse {
    #[schema(example = "ok")]
    message: String,
    #[schema(example = "success")]
    status: String,
    #[schema(example = json!({"id": "123", "result": "done"}))]
    data: Option<serde_json::Value>,
}

#[derive(Serialize, Deserialize, ToSchema)]
struct StandardErrorResponse {
    #[schema(example = "error")]
    status: String,
    #[schema(example = "Request failed")]
    message: String,
}

#[derive(Serialize, Deserialize, ToSchema)]
struct GenericJsonBody {
    #[schema(example = json!({"key": "value"}))]
    payload: serde_json::Value,
}

#[derive(Serialize, Deserialize, ToSchema)]
struct AuthBody {
    #[schema(example = "email@example.com")]
    email: String,
    #[schema(example = "password123")]
    password: String,
}

struct SecurityAddon;

impl Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        if let Some(components) = openapi.components.as_mut() {
            components.add_security_scheme(
                "bearer_auth",
                SecurityScheme::Http(
                    HttpBuilder::new()
                        .scheme(HttpAuthScheme::Bearer)
                        .bearer_format("JWT")
                        .build(),
                ),
            );
        }
    }
}

#[derive(OpenApi)]
#[openapi(
    modifiers(&SecurityAddon),
    paths(
        get_sync_endpoints_doc,
        create_endpoint_doc,
        org_register_doc,
        org_reregister_existing_account_doc,
        org_auth_doc,
        org_auth_sso_doc,
        org_auth_token_doc,
        org_refresh_doc,
        org_logout_doc,
        token_verify_doc,
        token_password_verify_doc,
        root_update_account_password_doc,
        root_aggregation_filter_doc,
        root_switch_account_doc,
        root_verify_schema_doc,
        root_unsafe_select_query_doc,
        root_unsafe_transaction_query_doc,
        root_create_materialized_view_doc,
        root_delete_materialized_view_doc,
        root_create_procedure_doc,
        root_delete_procedure_doc,
        root_call_procedure_doc,
        root_create_function_doc,
        root_delete_function_doc,
        root_call_function_doc,
        root_list_triggers_doc,
        root_create_trigger_doc,
        root_delete_trigger_doc,
        root_cron_schedule_job_doc,
        root_prometheus_queries_doc,
        root_prometheus_results_doc,
        root_create_record_doc,
        root_upsert_doc,
        root_batch_update_records_doc,
        root_batch_delete_records_doc,
        root_get_by_filter_doc,
        root_count_by_filter_doc,
        root_get_by_id_doc,
        root_update_record_doc,
        root_delete_record_doc,
        root_batch_insert_records_doc,
        root_search_suggestions_doc,
        system_initialize_all_doc,
        system_initialize_system_code_config_doc,
        system_initialize_root_account_config_doc,
        system_initialize_global_organization_config_doc,
        system_initialize_system_device_config_doc,
        system_initialize_background_services_config_doc,
        system_initialize_initial_entity_data_config_doc,
        system_initialize_generate_schema_config_doc,
        store_aggregation_filter_doc,
        store_verify_schema_doc,
        store_unsafe_select_query_doc,
        store_create_record_doc,
        store_upsert_doc,
        store_batch_update_records_doc,
        store_batch_delete_records_doc,
        store_get_by_filter_doc,
        store_count_by_filter_doc,
        store_get_by_id_doc,
        store_update_record_doc,
        store_delete_record_doc,
        store_batch_insert_records_doc,
        store_switch_account_doc,
        store_search_suggestions_doc,
        listener_get_doc,
        listener_create_function_doc,
        listener_test_function_doc,
        listener_delete_doc,
        file_download_doc,
        file_get_doc,
        file_upload_doc,
        health_check_doc,
        metrics_doc,
        health_detailed_doc,
        health_ready_doc,
        health_live_doc,
        health_phase_doc,
        health_component_counts_doc,
        health_component_statistics_doc,
        health_update_component_metadata_doc,
        health_record_component_check_doc,
        health_get_component_doc,
        health_snapshot_doc,
        health_events_doc,
        health_monitoring_status_doc,
        health_monitoring_interval_doc
    ),
    components(schemas(
        StandardOkResponse,
        StandardErrorResponse,
        GenericJsonBody,
        AuthBody,
        crate::controllers::sync_endpoints_controller::EndpointRequest,
        crate::controllers::organization_controller::RegisterDto,
        crate::controllers::organization_controller::AuthDto,
        crate::controllers::organization_controller::AuthByTokenDto,
        crate::structs::organizations_structs::VerifyPasswordParams,
        crate::structs::core::ApiResponse,
        crate::structs::core::SwitchAccountRequest,
        crate::structs::core::RequestBody,
        crate::structs::core::UpsertRequestBody,
        crate::structs::core::BatchUpdateBody,
        crate::structs::core::GetByFilter,
        crate::structs::core::AggregationFilter,
        crate::structs::core::SearchSuggestionParams,
        crate::controllers::store_controller::BatchInsertBody,
        crate::controllers::store_controller::SchemaVerificationRequest,
        crate::controllers::store_controller::SchemaVerificationResponse,
        crate::controllers::store_controller::TriggerListQuery,
        crate::controllers::pg_functions::pg_listener_controller::CreateFunctionRequest,
        crate::controllers::pg_functions::pg_listener_controller::TestFunctionRequest,
        crate::controllers::health_controller::HealthResponse,
        crate::controllers::health_controller::DetailedHealthResponse,
        crate::controllers::health_controller::ReadinessResponse,
        crate::controllers::health_controller::LivenessResponse,
        crate::controllers::health_controller::ComponentMetadataRequest,
        crate::controllers::health_controller::HealthCheckRequest,
        crate::controllers::health_controller::MonitoringConfigRequest,
        crate::controllers::health_controller::ComponentStatisticsResponse,
        crate::controllers::health_controller::MonitoringStatusResponse
    )),
    tags(
        (name = "Sync", description = "Sync endpoint management"),
        (name = "Organizations", description = "Organization authentication and account endpoints"),
        (name = "Token", description = "Token and password verification"),
        (name = "Root Store", description = "Privileged root store operations"),
        (name = "System", description = "System initialization operations"),
        (name = "Store", description = "Regular store CRUD and query operations"),
        (name = "Listener", description = "PostgreSQL listener function operations"),
        (name = "File", description = "File retrieval and upload"),
        (name = "Health", description = "Health and metrics endpoints")
    ),
    info(
        title = "Store Service API",
        version = "0.2.84",
        description = "API documentation for the Store service, covering organization authentication, token verification, store and root data operations, system initialization, PostgreSQL listener management, file services, and health/monitoring endpoints.",
        contact(
            name = "DNA Micro DB Team",
            url = "https://www.dnamicro.com/"
        )
    )
)]
pub struct ApiDoc;

#[utoipa::path(
    get,
    path = "/api/sync_endpoints",
    tag = "Sync",
    summary = "List sync endpoints",
    responses((status = 200, description = "Sync endpoints fetched", body = serde_json::Value))
)]
fn get_sync_endpoints_doc() {}

#[utoipa::path(
    post,
    path = "/api/sync_endpoints",
    tag = "Sync",
    summary = "Create a sync endpoint",
    request_body(content = crate::controllers::sync_endpoints_controller::EndpointRequest),
    responses((status = 200, description = "Sync endpoint created", body = serde_json::Value, example = json!({"message":"ok"})))
)]
fn create_endpoint_doc() {}

#[utoipa::path(post, path = "/api/organizations/register", tag = "Organizations", summary = "Register organization", request_body(content = crate::controllers::organization_controller::RegisterDto), responses((status = 200, description = "Registered", body = serde_json::Value)))]
fn org_register_doc() {}
#[utoipa::path(put, path = "/api/organizations/register/{id}", tag = "Organizations", summary = "Re-register existing account", params(("id" = String, Path, description = "Account identifier")), request_body(content = crate::controllers::organization_controller::RegisterDto), responses((status = 200, description = "Re-registered", body = serde_json::Value)))]
fn org_reregister_existing_account_doc() {}
#[utoipa::path(post, path = "/api/organizations/auth", tag = "Organizations", summary = "Authenticate organization user", request_body(content = crate::controllers::organization_controller::AuthDto), responses((status = 200, description = "Authenticated", body = serde_json::Value), (status = 401, description = "Unauthorized", body = StandardErrorResponse)))]
fn org_auth_doc() {}
#[utoipa::path(post, path = "/api/organizations/auth/sso", tag = "Organizations", summary = "Authenticate with SSO", request_body(content = crate::controllers::organization_controller::AuthDto), responses((status = 200, description = "Authenticated with SSO", body = serde_json::Value)))]
fn org_auth_sso_doc() {}
#[utoipa::path(post, path = "/api/organizations/auth/token", tag = "Organizations", summary = "Authenticate by token", request_body(content = crate::controllers::organization_controller::AuthByTokenDto), responses((status = 200, description = "Token authenticated", body = serde_json::Value)))]
fn org_auth_token_doc() {}
#[utoipa::path(post, path = "/api/organizations/refresh", tag = "Organizations", summary = "Refresh auth token", responses((status = 200, description = "Token refreshed", body = serde_json::Value)))]
fn org_refresh_doc() {}
#[utoipa::path(post, path = "/api/organizations/logout", tag = "Organizations", summary = "Logout current session", responses((status = 200, description = "Logged out")))]
fn org_logout_doc() {}
#[utoipa::path(post, path = "/api/token/verify", tag = "Token", summary = "Verify token", params(("t" = Option<String>, Query, description = "Token value"), ("authorization" = Option<String>, Header, description = "Bearer token")), responses((status = 200, description = "Token validity checked", body = crate::structs::core::ApiResponse)))]
fn token_verify_doc() {}
#[utoipa::path(post, path = "/api/token/api/password/verify", tag = "Token", summary = "Verify password", request_body(content = crate::structs::organizations_structs::VerifyPasswordParams), responses((status = 200, description = "Password checked", body = crate::structs::core::ApiResponse)))]
fn token_password_verify_doc() {}

#[utoipa::path(patch, path = "/api/store/root/accounts/password/{account_id}", tag = "Root Store", security(("bearer_auth" = [])), summary = "Update account password", params(("account_id" = String, Path, description = "Account identifier")), request_body(content = serde_json::Value, example = json!({"new_password":"secret"})), responses((status = 200, description = "Password updated", body = StandardOkResponse)))]
fn root_update_account_password_doc() {}
#[utoipa::path(post, path = "/api/store/root/aggregate", tag = "Root Store", security(("bearer_auth" = [])), summary = "Run root aggregate filter", request_body(content = crate::structs::core::AggregationFilter), responses((status = 200, description = "Aggregation result", body = crate::structs::core::ApiResponse)))]
fn root_aggregation_filter_doc() {}
#[utoipa::path(post, path = "/api/store/root/switch_account", tag = "Root Store", security(("bearer_auth" = [])), summary = "Switch account context", request_body(content = crate::structs::core::SwitchAccountRequest), responses((status = 200, description = "Account switched", body = crate::structs::core::ApiResponse)))]
fn root_switch_account_doc() {}
#[utoipa::path(post, path = "/api/store/root/verify_schema", tag = "Root Store", security(("bearer_auth" = [])), summary = "Verify schema integrity", request_body(content = crate::controllers::store_controller::SchemaVerificationRequest), responses((status = 200, description = "Schema verification output", body = crate::controllers::store_controller::SchemaVerificationResponse)))]
fn root_verify_schema_doc() {}
#[utoipa::path(post, path = "/api/store/root/unsafe_select_query", tag = "Root Store", security(("bearer_auth" = [])), summary = "Run unsafe select query", request_body(content = serde_json::Value, example = json!({"query":"SELECT * FROM users LIMIT 10"})), responses((status = 200, description = "Query results", body = serde_json::Value)))]
fn root_unsafe_select_query_doc() {}
#[utoipa::path(post, path = "/api/store/root/unsafe_transaction_query", tag = "Root Store", security(("bearer_auth" = [])), summary = "Run unsafe transactional query", request_body(content = serde_json::Value, example = json!({"queries":["UPDATE ...","INSERT ..."]})), responses((status = 200, description = "Transaction result", body = serde_json::Value)))]
fn root_unsafe_transaction_query_doc() {}
#[utoipa::path(post, path = "/api/store/root/materialized_view/{table}", tag = "Root Store", security(("bearer_auth" = [])), summary = "Create materialized view", params(("table" = String, Path, description = "Table name")), request_body(content = serde_json::Value, example = json!({"definition":"SELECT ..."})), responses((status = 200, description = "Materialized view created", body = StandardOkResponse)))]
fn root_create_materialized_view_doc() {}
#[utoipa::path(delete, path = "/api/store/root/materialized_view/{table}", tag = "Root Store", security(("bearer_auth" = [])), summary = "Delete materialized view", params(("table" = String, Path, description = "Table name")), responses((status = 200, description = "Materialized view deleted", body = StandardOkResponse)))]
fn root_delete_materialized_view_doc() {}
#[utoipa::path(post, path = "/api/store/root/procedure/{name}", tag = "Root Store", security(("bearer_auth" = [])), summary = "Create procedure", params(("name" = String, Path, description = "Procedure name")), request_body(content = serde_json::Value, example = json!({"sql":"CREATE PROCEDURE ..."})), responses((status = 200, description = "Procedure created", body = StandardOkResponse)))]
fn root_create_procedure_doc() {}
#[utoipa::path(delete, path = "/api/store/root/procedure/{name}", tag = "Root Store", security(("bearer_auth" = [])), summary = "Delete procedure", params(("name" = String, Path, description = "Procedure name")), responses((status = 200, description = "Procedure deleted", body = StandardOkResponse)))]
fn root_delete_procedure_doc() {}
#[utoipa::path(post, path = "/api/store/root/procedure/call/{name}", tag = "Root Store", security(("bearer_auth" = [])), summary = "Call procedure", params(("name" = String, Path, description = "Procedure name")), request_body(content = serde_json::Value, example = json!({"args":{"id":1}})), responses((status = 200, description = "Procedure call result", body = serde_json::Value)))]
fn root_call_procedure_doc() {}
#[utoipa::path(post, path = "/api/store/root/function/{name}", tag = "Root Store", security(("bearer_auth" = [])), summary = "Create function", params(("name" = String, Path, description = "Function name")), request_body(content = serde_json::Value, example = json!({"sql":"CREATE FUNCTION ..."})), responses((status = 200, description = "Function created", body = StandardOkResponse)))]
fn root_create_function_doc() {}
#[utoipa::path(delete, path = "/api/store/root/function/{name}", tag = "Root Store", security(("bearer_auth" = [])), summary = "Delete function", params(("name" = String, Path, description = "Function name")), responses((status = 200, description = "Function deleted", body = StandardOkResponse)))]
fn root_delete_function_doc() {}
#[utoipa::path(post, path = "/api/store/root/function/call/{name}", tag = "Root Store", security(("bearer_auth" = [])), summary = "Call function", params(("name" = String, Path, description = "Function name")), request_body(content = serde_json::Value, example = json!({"args":{"id":1}})), responses((status = 200, description = "Function call result", body = serde_json::Value)))]
fn root_call_function_doc() {}
#[utoipa::path(get, path = "/api/store/root/triggers", tag = "Root Store", security(("bearer_auth" = [])), summary = "List triggers", responses((status = 200, description = "Trigger list", body = serde_json::Value)))]
fn root_list_triggers_doc() {}
#[utoipa::path(post, path = "/api/store/root/trigger/{table}", tag = "Root Store", security(("bearer_auth" = [])), summary = "Create trigger", params(("table" = String, Path, description = "Table name")), request_body(content = serde_json::Value, example = json!({"name":"trg_name","sql":"..."})), responses((status = 200, description = "Trigger created", body = StandardOkResponse)))]
fn root_create_trigger_doc() {}
#[utoipa::path(delete, path = "/api/store/root/trigger/{table}/{name}", tag = "Root Store", security(("bearer_auth" = [])), summary = "Delete trigger", params(("table" = String, Path, description = "Table name"), ("name" = String, Path, description = "Trigger name")), responses((status = 200, description = "Trigger deleted", body = StandardOkResponse)))]
fn root_delete_trigger_doc() {}
#[utoipa::path(post, path = "/api/store/root/cron-schedule-job", tag = "Root Store", security(("bearer_auth" = [])), summary = "Schedule cron job", request_body(content = serde_json::Value, example = json!({"name":"job1","schedule":"0 * * * *"})), responses((status = 200, description = "Cron job scheduled", body = StandardOkResponse)))]
fn root_cron_schedule_job_doc() {}
#[utoipa::path(get, path = "/api/store/root/monitoring/prometheus_queries", tag = "Root Store", security(("bearer_auth" = [])), summary = "Get Prometheus query templates", responses((status = 200, description = "Prometheus query definitions", body = serde_json::Value)))]
fn root_prometheus_queries_doc() {}
#[utoipa::path(get, path = "/api/store/root/monitoring/prometheus_results", tag = "Root Store", security(("bearer_auth" = [])), summary = "Get Prometheus query results", responses((status = 200, description = "Prometheus results", body = serde_json::Value)))]
fn root_prometheus_results_doc() {}
#[utoipa::path(post, path = "/api/store/root/{table}", tag = "Root Store", security(("bearer_auth" = [])), summary = "Create root-scoped record", params(("table" = String, Path, description = "Table name")), request_body(content = serde_json::Value, example = json!({"name":"sample"})), responses((status = 200, description = "Record created", body = serde_json::Value)))]
fn root_create_record_doc() {}
#[utoipa::path(post, path = "/api/store/root/upsert/{table}", tag = "Root Store", security(("bearer_auth" = [])), summary = "Upsert root-scoped record", params(("table" = String, Path, description = "Table name")), request_body(content = crate::structs::core::UpsertRequestBody), responses((status = 200, description = "Record upserted", body = crate::structs::core::ApiResponse)))]
fn root_upsert_doc() {}
#[utoipa::path(patch, path = "/api/store/root/batch/{table}", tag = "Root Store", security(("bearer_auth" = [])), summary = "Batch update root-scoped records", params(("table" = String, Path, description = "Table name")), request_body(content = crate::structs::core::BatchUpdateBody), responses((status = 200, description = "Batch updated", body = crate::structs::core::ApiResponse)))]
fn root_batch_update_records_doc() {}
#[utoipa::path(delete, path = "/api/store/root/batch/{table}", tag = "Root Store", security(("bearer_auth" = [])), summary = "Batch delete root-scoped records", params(("table" = String, Path, description = "Table name")), responses((status = 200, description = "Batch deleted", body = StandardOkResponse)))]
fn root_batch_delete_records_doc() {}
#[utoipa::path(post, path = "/api/store/root/{table}/filter", tag = "Root Store", security(("bearer_auth" = [])), summary = "Filter root-scoped records", params(("table" = String, Path, description = "Table name")), request_body(content = crate::structs::core::GetByFilter), responses((status = 200, description = "Filtered records", body = crate::structs::core::ApiResponse)))]
fn root_get_by_filter_doc() {}
#[utoipa::path(post, path = "/api/store/root/{table}/count", tag = "Root Store", security(("bearer_auth" = [])), summary = "Count root-scoped records", params(("table" = String, Path, description = "Table name")), request_body(content = crate::structs::core::GetByFilter), responses((status = 200, description = "Count result", body = crate::structs::core::ApiResponse)))]
fn root_count_by_filter_doc() {}
#[utoipa::path(get, path = "/api/store/root/{table}/{id}", tag = "Root Store", security(("bearer_auth" = [])), summary = "Get root-scoped record by id", params(("table" = String, Path, description = "Table name"), ("id" = String, Path, description = "Record id")), responses((status = 200, description = "Record fetched", body = serde_json::Value)))]
fn root_get_by_id_doc() {}
#[utoipa::path(patch, path = "/api/store/root/{table}/{id}", tag = "Root Store", security(("bearer_auth" = [])), summary = "Update root-scoped record by id", params(("table" = String, Path, description = "Table name"), ("id" = String, Path, description = "Record id")), request_body(content = serde_json::Value, example = json!({"name":"new value"})), responses((status = 200, description = "Record updated", body = StandardOkResponse)))]
fn root_update_record_doc() {}
#[utoipa::path(delete, path = "/api/store/root/{table}/{id}", tag = "Root Store", security(("bearer_auth" = [])), summary = "Delete root-scoped record by id", params(("table" = String, Path, description = "Table name"), ("id" = String, Path, description = "Record id")), responses((status = 200, description = "Record deleted", body = StandardOkResponse)))]
fn root_delete_record_doc() {}
#[utoipa::path(post, path = "/api/store/root/batch/{table}", tag = "Root Store", security(("bearer_auth" = [])), summary = "Batch insert root-scoped records", params(("table" = String, Path, description = "Table name")), request_body(content = crate::controllers::store_controller::BatchInsertBody), responses((status = 200, description = "Batch inserted", body = crate::structs::core::ApiResponse)))]
fn root_batch_insert_records_doc() {}
#[utoipa::path(post, path = "/api/store/root/{table}/filter/suggestions", tag = "Root Store", security(("bearer_auth" = [])), summary = "Get root-scoped filter suggestions", params(("table" = String, Path, description = "Table name")), request_body(content = crate::structs::core::SearchSuggestionParams), responses((status = 200, description = "Suggestions list", body = crate::structs::core::ApiResponse)))]
fn root_search_suggestions_doc() {}

#[utoipa::path(post, path = "/api/system/initialize_all", tag = "System", security(("bearer_auth" = [])), summary = "Run all initializers", request_body(content = serde_json::Value, example = json!({})), responses((status = 200, description = "Initialization started", body = StandardOkResponse)))]
fn system_initialize_all_doc() {}
#[utoipa::path(post, path = "/api/system/initialize/system_code_config", tag = "System", security(("bearer_auth" = [])), summary = "Initialize system code config", request_body(content = serde_json::Value, example = json!({})), responses((status = 200, description = "Initialized", body = StandardOkResponse)))]
fn system_initialize_system_code_config_doc() {}
#[utoipa::path(post, path = "/api/system/initialize/root_account_config", tag = "System", security(("bearer_auth" = [])), summary = "Initialize root account config", request_body(content = serde_json::Value, example = json!({})), responses((status = 200, description = "Initialized", body = StandardOkResponse)))]
fn system_initialize_root_account_config_doc() {}
#[utoipa::path(post, path = "/api/system/initialize/global_organization_config", tag = "System", security(("bearer_auth" = [])), summary = "Initialize global organization config", request_body(content = serde_json::Value, example = json!({})), responses((status = 200, description = "Initialized", body = StandardOkResponse)))]
fn system_initialize_global_organization_config_doc() {}
#[utoipa::path(post, path = "/api/system/initialize/system_device_config", tag = "System", security(("bearer_auth" = [])), summary = "Initialize system device config", request_body(content = serde_json::Value, example = json!({})), responses((status = 200, description = "Initialized", body = StandardOkResponse)))]
fn system_initialize_system_device_config_doc() {}
#[utoipa::path(post, path = "/api/system/initialize/background_services_config", tag = "System", security(("bearer_auth" = [])), summary = "Initialize background services config", request_body(content = serde_json::Value, example = json!({})), responses((status = 200, description = "Initialized", body = StandardOkResponse)))]
fn system_initialize_background_services_config_doc() {}
#[utoipa::path(post, path = "/api/system/initialize/initial_entity_data_config", tag = "System", security(("bearer_auth" = [])), summary = "Initialize initial entity data", request_body(content = serde_json::Value, example = json!({})), responses((status = 200, description = "Initialized", body = StandardOkResponse)))]
fn system_initialize_initial_entity_data_config_doc() {}
#[utoipa::path(post, path = "/api/system/initialize/generate_schema_config", tag = "System", security(("bearer_auth" = [])), summary = "Generate schema config", request_body(content = serde_json::Value, example = json!({})), responses((status = 200, description = "Schema generated", body = StandardOkResponse)))]
fn system_initialize_generate_schema_config_doc() {}

#[utoipa::path(post, path = "/api/store/aggregate", tag = "Store", security(("bearer_auth" = [])), summary = "Run store aggregate filter", request_body(content = crate::structs::core::AggregationFilter), responses((status = 200, description = "Aggregation result", body = crate::structs::core::ApiResponse)))]
fn store_aggregation_filter_doc() {}
#[utoipa::path(post, path = "/api/store/verify_schema", tag = "Store", security(("bearer_auth" = [])), summary = "Verify store schema", request_body(content = crate::controllers::store_controller::SchemaVerificationRequest), responses((status = 200, description = "Schema verification output", body = crate::controllers::store_controller::SchemaVerificationResponse)))]
fn store_verify_schema_doc() {}
#[utoipa::path(post, path = "/api/store/unsafe_select_query", tag = "Store", security(("bearer_auth" = [])), summary = "Run unsafe store select query", request_body(content = serde_json::Value, example = json!({"query":"SELECT 1"})), responses((status = 200, description = "Query result", body = serde_json::Value)))]
fn store_unsafe_select_query_doc() {}
#[utoipa::path(post, path = "/api/store/{table}", tag = "Store", security(("bearer_auth" = [])), summary = "Create record", params(("table" = String, Path, description = "Table name")), request_body(content = serde_json::Value, example = json!({"name":"sample"})), responses((status = 200, description = "Record created", body = serde_json::Value)))]
fn store_create_record_doc() {}
#[utoipa::path(post, path = "/api/store/upsert/{table}", tag = "Store", security(("bearer_auth" = [])), summary = "Upsert record", params(("table" = String, Path, description = "Table name")), request_body(content = crate::structs::core::UpsertRequestBody), responses((status = 200, description = "Record upserted", body = crate::structs::core::ApiResponse)))]
fn store_upsert_doc() {}
#[utoipa::path(patch, path = "/api/store/batch/{table}", tag = "Store", security(("bearer_auth" = [])), summary = "Batch update records", params(("table" = String, Path, description = "Table name")), request_body(content = crate::structs::core::BatchUpdateBody), responses((status = 200, description = "Batch updated", body = crate::structs::core::ApiResponse)))]
fn store_batch_update_records_doc() {}
#[utoipa::path(delete, path = "/api/store/batch/{table}", tag = "Store", security(("bearer_auth" = [])), summary = "Batch delete records", params(("table" = String, Path, description = "Table name")), responses((status = 200, description = "Batch deleted", body = StandardOkResponse)))]
fn store_batch_delete_records_doc() {}
#[utoipa::path(post, path = "/api/store/{table}/filter", tag = "Store", security(("bearer_auth" = [])), summary = "Filter records", params(("table" = String, Path, description = "Table name")), request_body(content = crate::structs::core::GetByFilter), responses((status = 200, description = "Filtered records", body = crate::structs::core::ApiResponse)))]
fn store_get_by_filter_doc() {}
#[utoipa::path(post, path = "/api/store/{table}/count", tag = "Store", security(("bearer_auth" = [])), summary = "Count records", params(("table" = String, Path, description = "Table name")), request_body(content = crate::structs::core::GetByFilter), responses((status = 200, description = "Count result", body = crate::structs::core::ApiResponse)))]
fn store_count_by_filter_doc() {}
#[utoipa::path(get, path = "/api/store/{table}/{id}", tag = "Store", security(("bearer_auth" = [])), summary = "Get record by id", params(("table" = String, Path, description = "Table name"), ("id" = String, Path, description = "Record id")), responses((status = 200, description = "Record fetched", body = serde_json::Value)))]
fn store_get_by_id_doc() {}
#[utoipa::path(patch, path = "/api/store/{table}/{id}", tag = "Store", security(("bearer_auth" = [])), summary = "Update record by id", params(("table" = String, Path, description = "Table name"), ("id" = String, Path, description = "Record id")), request_body(content = serde_json::Value, example = json!({"name":"new value"})), responses((status = 200, description = "Record updated", body = StandardOkResponse)))]
fn store_update_record_doc() {}
#[utoipa::path(delete, path = "/api/store/{table}/{id}", tag = "Store", security(("bearer_auth" = [])), summary = "Delete record by id", params(("table" = String, Path, description = "Table name"), ("id" = String, Path, description = "Record id")), responses((status = 200, description = "Record deleted", body = StandardOkResponse)))]
fn store_delete_record_doc() {}
#[utoipa::path(post, path = "/api/store/batch/{table}", tag = "Store", security(("bearer_auth" = [])), summary = "Batch insert records", params(("table" = String, Path, description = "Table name")), request_body(content = crate::controllers::store_controller::BatchInsertBody), responses((status = 200, description = "Batch inserted", body = crate::structs::core::ApiResponse)))]
fn store_batch_insert_records_doc() {}
#[utoipa::path(post, path = "/api/store/switch_account", tag = "Store", security(("bearer_auth" = [])), summary = "Switch account context", request_body(content = crate::structs::core::SwitchAccountRequest), responses((status = 200, description = "Account switched", body = crate::structs::core::ApiResponse)))]
fn store_switch_account_doc() {}
#[utoipa::path(post, path = "/api/store/{table}/filter/suggestions", tag = "Store", security(("bearer_auth" = [])), summary = "Get filter suggestions", params(("table" = String, Path, description = "Table name")), request_body(content = crate::structs::core::SearchSuggestionParams), responses((status = 200, description = "Suggestions list", body = crate::structs::core::ApiResponse)))]
fn store_search_suggestions_doc() {}

#[utoipa::path(get, path = "/api/listener", tag = "Listener", security(("bearer_auth" = [])), summary = "Get listener configuration", responses((status = 200, description = "Listener config", body = crate::structs::core::ApiResponse)))]
fn listener_get_doc() {}
#[utoipa::path(post, path = "/api/listener/function", tag = "Listener", security(("bearer_auth" = [])), summary = "Create listener function", request_body(content = crate::controllers::pg_functions::pg_listener_controller::CreateFunctionRequest), responses((status = 200, description = "Function created", body = crate::structs::core::ApiResponse)))]
fn listener_create_function_doc() {}
#[utoipa::path(post, path = "/api/listener/test", tag = "Listener", security(("bearer_auth" = [])), summary = "Test listener function syntax", request_body(content = crate::controllers::pg_functions::pg_listener_controller::TestFunctionRequest), responses((status = 200, description = "Syntax result", body = crate::structs::core::ApiResponse)))]
fn listener_test_function_doc() {}
#[utoipa::path(delete, path = "/api/listener/{function_name}", tag = "Listener", security(("bearer_auth" = [])), summary = "Delete listener function", params(("function_name" = String, Path, description = "Function name")), responses((status = 200, description = "Function deleted", body = StandardOkResponse)))]
fn listener_delete_doc() {}

#[utoipa::path(get, path = "/api/file/{file_id}/download", tag = "File", security(("bearer_auth" = [])), summary = "Download file by id", params(("file_id" = String, Path, description = "File id")), responses((status = 200, description = "File stream")))]
fn file_download_doc() {}
#[utoipa::path(get, path = "/api/file/{file_id}", tag = "File", security(("bearer_auth" = [])), summary = "Get file metadata by id", params(("file_id" = String, Path, description = "File id")), responses((status = 200, description = "File metadata", body = serde_json::Value)))]
fn file_get_doc() {}
#[utoipa::path(
    post,
    path = "/api/file/upload",
    tag = "File",
    security(("bearer_auth" = [])),
    summary = "Upload file",
    request_body(content = serde_json::Value, content_type = "multipart/form-data", example = json!({"file":"<binary>","table":"documents"})),
    responses((status = 200, description = "File uploaded", body = serde_json::Value))
)]
fn file_upload_doc() {}

#[utoipa::path(get, path = "/api/health", tag = "Health", summary = "Basic health check", responses((status = 200, description = "Service health", body = crate::controllers::health_controller::HealthResponse)))]
fn health_check_doc() {}
#[utoipa::path(get, path = "/api/metrics", tag = "Health", summary = "Prometheus metrics endpoint", responses((status = 200, description = "Metrics response")))]
fn metrics_doc() {}
#[utoipa::path(get, path = "/api/health/detailed", tag = "Health", summary = "Detailed health check", responses((status = 200, description = "Detailed health info", body = crate::controllers::health_controller::DetailedHealthResponse)))]
fn health_detailed_doc() {}
#[utoipa::path(get, path = "/api/health/ready", tag = "Health", summary = "Readiness probe", responses((status = 200, description = "Readiness state", body = crate::controllers::health_controller::ReadinessResponse)))]
fn health_ready_doc() {}
#[utoipa::path(get, path = "/api/health/live", tag = "Health", summary = "Liveness probe", responses((status = 200, description = "Liveness state", body = crate::controllers::health_controller::LivenessResponse)))]
fn health_live_doc() {}
#[utoipa::path(get, path = "/api/health/phase", tag = "Health", summary = "Current deployment phase", responses((status = 200, description = "Current phase", body = serde_json::Value)))]
fn health_phase_doc() {}
#[utoipa::path(get, path = "/api/health/components/counts", tag = "Health", summary = "Get component counts", responses((status = 200, description = "Component counts", body = serde_json::Value)))]
fn health_component_counts_doc() {}
#[utoipa::path(get, path = "/api/health/components/statistics", tag = "Health", summary = "Get component statistics", responses((status = 200, description = "Component statistics", body = crate::controllers::health_controller::ComponentStatisticsResponse)))]
fn health_component_statistics_doc() {}
#[utoipa::path(put, path = "/api/health/components/{component_name}/metadata", tag = "Health", summary = "Update component metadata", params(("component_name" = String, Path, description = "Component name")), request_body(content = crate::controllers::health_controller::ComponentMetadataRequest), responses((status = 200, description = "Metadata updated", body = serde_json::Value)))]
fn health_update_component_metadata_doc() {}
#[utoipa::path(post, path = "/api/health/components/{component_name}/health-check", tag = "Health", summary = "Record component health check", params(("component_name" = String, Path, description = "Component name")), request_body(content = crate::controllers::health_controller::HealthCheckRequest), responses((status = 200, description = "Health check recorded", body = serde_json::Value)))]
fn health_record_component_check_doc() {}
#[utoipa::path(get, path = "/api/health/components/{component_name}", tag = "Health", summary = "Get component health details", params(("component_name" = String, Path, description = "Component name")), responses((status = 200, description = "Component details", body = serde_json::Value)))]
fn health_get_component_doc() {}
#[utoipa::path(get, path = "/api/health/snapshot", tag = "Health", summary = "Create health snapshot", responses((status = 200, description = "Health snapshot", body = serde_json::Value)))]
fn health_snapshot_doc() {}
#[utoipa::path(get, path = "/api/health/events", tag = "Health", summary = "Get recent health events", responses((status = 200, description = "Recent events", body = serde_json::Value)))]
fn health_events_doc() {}
#[utoipa::path(get, path = "/api/health/monitoring/status", tag = "Health", summary = "Get monitoring status", responses((status = 200, description = "Monitoring status", body = crate::controllers::health_controller::MonitoringStatusResponse)))]
fn health_monitoring_status_doc() {}
#[utoipa::path(put, path = "/api/health/monitoring/interval", tag = "Health", summary = "Set monitoring interval", request_body(content = crate::controllers::health_controller::MonitoringConfigRequest), responses((status = 200, description = "Monitoring interval updated", body = serde_json::Value)))]
fn health_monitoring_interval_doc() {}
