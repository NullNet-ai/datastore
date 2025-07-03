use crate::auth::structs::Session;
use crate::cache::cache;
use crate::db;
use crate::permissions::permissions_queries::{
    execute_permission_query, GroupByFieldRecordPermissionsResult, PermissionQueryOutput,
    PermissionQueryParams, PermissionQueryResult, PermissionQueryType, RolePermissionResult,
    ValidPassKeyResult,
};
use crate::permissions::structs::DataPermissions;
use crate::utils::request_type_handler::RequestType;
use actix_web::dev::Extensions;
use actix_web::http::header::HeaderMap;
use actix_web::{HttpMessage, HttpRequest};
use serde_json::Value;
use sha1::{Digest, Sha1};
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;

pub struct PermissionsContext {
    pub permissions_query: DataPermissions,
    pub host: String,
    pub headers: HeaderMap,
    pub table: String,
    pub account_organization_id: String,
    pub body: Value,
    pub metadata: HashMap<String, Value>,
    pub account_id: String,
    pub query: HashMap<String, String>,
    pub method: actix_web::http::Method,
    pub uri: actix_web::http::Uri,
    pub session: Session,
    pub session_data: Option<HashMap<String, Value>>,
}

// Helper function to create SHA1 hash for cache keys
fn sha1(input: &str) -> String {
    let mut hasher = Sha1::new();
    hasher.update(input.as_bytes());
    format!("{:x}", hasher.finalize())
}

// Helper function to check for permission errors
fn error_check_permission(account_id: &str, role: &str) -> Option<String> {
    if account_id.is_empty() {
        return Some("Account ID is required".to_string());
    }
    if role.is_empty() {
        return Some("Role is required".to_string());
    }
    None
}

// Helper function to convert JWT expiry time to milliseconds
fn get_time_ms(jwt_expires: &str) -> u64 {
    let last_char = jwt_expires.chars().last().unwrap_or('d');
    let num_str = &jwt_expires[0..jwt_expires.len() - 1];
    let num = num_str.parse::<u64>().unwrap_or(2);

    match last_char {
        's' => num * 1000,                    // seconds to ms
        'm' => num * 60 * 1000,               // minutes to ms
        'h' => num * 60 * 60 * 1000,          // hours to ms
        'd' | _ => num * 24 * 60 * 60 * 1000, // days to ms (default)
    }
}

pub async fn get_cached_permissions(
    request_type: RequestType,
    context: PermissionsContext,
) -> Result<(Option<String>, Value), Box<dyn std::error::Error>> {
    let PermissionsContext {
        permissions_query,
        host,
        headers,
        table,
        account_organization_id,
        body,
        metadata,
        account_id,
        query,
        uri,
        method,
        session,
        session_data,
    } = context;

    let DataPermissions {
        schema: _aliased_schema,
        valid_pass_keys_query_params,
        role_permissions_query_params,
        requested_fields,
        data_permissions_query_params,
        account_organization_id,
        group_by_field_record_permissions_params,
    } = permissions_query;

    let user_agent = headers.get("user-agent").ok_or_else(|| {
        actix_web::error::ErrorUnauthorized("User agent not found, session malformed")
    })?;
    let custom_suffix = format!(
        "{}{}{}{}{}{}",
        session.session_id,
        method,
        uri,
        session.user.role_id,
        host,
        user_agent.to_str().unwrap_or(""),
    );

    log::debug!("custom_suffix: {}", custom_suffix);
    let mut role = session.user.role_id.clone();

    // Get JWT expiry from env or default to 2 days
    let jwt_expires = std::env::var("JWT_EXPIRES_IN").unwrap_or_else(|_| "2d".to_string());
    let expiry_ms = get_time_ms(&jwt_expires);

    // Helper function to get data from cache or execute query
    async fn get_from_cache_or_execute<T: serde::de::DeserializeOwned + serde::Serialize>(
        cache_key: &str,
        session_data: Option<&std::collections::HashMap<String, Value>>,
        session_key: &str,
        query_type: PermissionQueryType,
        params: PermissionQueryParams,
        account_organization_id: &str,
        expiry_ms: u64,
    ) -> Result<Vec<T>, Box<dyn std::error::Error>> {
        // Try to get from cache first
        if let Some(cached_data) = cache.get(cache_key) {
            if let Some(data_array) = cached_data["data"].as_array() {
                let results: Vec<T> =
                    serde_json::from_value(cached_data["data"].clone()).unwrap_or_default();
                return Ok(results);
            }
        }

        // Try to get from session
        if let Some(session_map) = session_data {
            if let Some(session_value) = session_map.get(session_key) {
                if session_value["cached"].as_bool().unwrap_or(false) {
                    let results: Vec<T> =
                        serde_json::from_value(session_value["data"].clone()).unwrap_or_default();
                    return Ok(results);
                }
            }
        }

        // If not in cache or session, execute the query
        let mut conn = db::get_async_connection().await;
        let output = execute_permission_query(&mut conn, query_type, params).await?;

        // Extract the results based on the output type
        let results = match output {
            PermissionQueryOutput::Permissions(data) => {
                serde_json::to_value(data).unwrap_or_default()
            }
            PermissionQueryOutput::ValidPassKeys(data) => {
                serde_json::to_value(data).unwrap_or_default()
            }
            PermissionQueryOutput::GroupByFieldRecordPermission(data) => {
                serde_json::to_value(data).unwrap_or_default()
            }
            PermissionQueryOutput::RolePermissions(data) => {
                serde_json::to_value(data).unwrap_or_default()
            }
        };

        // Store in cache
        let response = serde_json::json!({
            "data": results,
            "account_organization_id": account_organization_id,
            "cache": false,
        });

        cache.insert_with_ttl(
            cache_key.to_string(),
            response.clone(),
            Duration::from_millis(expiry_ms),
        );

        // Convert back to the expected type
        let typed_results: Vec<T> = serde_json::from_value(results).unwrap_or_default();
        Ok(typed_results)
    }

    // Get field permissions
    let field_permissions_cache_key =
        sha1(&format!("{}_data_permissions:{}", table, custom_suffix));
    // Use the query_params from DataPermissions
    let field_permissions_params = data_permissions_query_params;

    let field_permissions_results: Vec<PermissionQueryResult> = get_from_cache_or_execute(
        &field_permissions_cache_key,
        session_data.as_ref(),
        "field_permissions",
        PermissionQueryType::Permissions,
        field_permissions_params,
        &account_organization_id,
        expiry_ms,
    )
    .await?;

    // Convert field permissions to JSON for compatibility with existing code
    let field_permissions = serde_json::json!({
        "data": field_permissions_results,
        "account_organization_id": account_organization_id,
        "cache": true,
    });

    log::debug!("Getting field_permissions permissions");
    log::debug!("Getting field_permissions permissions completed.");

    // Debug output if enabled
    if std::env::var("DEBUG").unwrap_or_else(|_| "false".to_string()) == "true" {
        println!("Field Permissions Table:");
        for item in &field_permissions_results {
            if let Some(field) = &item.field {
                if requested_fields.contains(field) {
                    println!("{:?}", item);
                }
            }
        }
    }

    // Store field permissions in session if not already there
    if session_data.is_none()
        || !session_data
            .as_ref()
            .unwrap()
            .contains_key("field_permissions")
    {
        // In a real implementation, you would update the session here
        log::debug!("Would store field_permissions in session");
    }

    // Get role permissions
    let role_permissions_cache_key = sha1(&format!("{}_role_permissions:{}", table, custom_suffix));
    // Use the role_permissions_query_params from DataPermissions
    let role_permissions_params = role_permissions_query_params;

    let role_permissions_results: Vec<RolePermissionResult> = get_from_cache_or_execute(
        &role_permissions_cache_key,
        session_data.as_ref(),
        "role_permissions",
        PermissionQueryType::RolePermissions,
        role_permissions_params,
        &account_organization_id,
        expiry_ms,
    )
    .await?;

    // Update role if needed from role permissions
    if let Some(first_role) = role_permissions_results.first() {
        if let Some(role_from_data) = &first_role.role {
            if role.is_empty() {
                role = role_from_data.clone();
            }
        }
    }

    // Check for errors
    let error = error_check_permission(&account_id, &role);

    // Convert role permissions to JSON for compatibility with existing code
    let role_permissions = serde_json::json!({
        "data": role_permissions_results,
        "account_organization_id": account_organization_id,
        "cache": true,
    });

    log::debug!("Getting role_permissions permissions");
    log::debug!("Getting role_permissions permissions completed.");

    // Debug output if enabled
    if std::env::var("DEBUG").unwrap_or_else(|_| "false".to_string()) == "true" {
        println!("Role Permissions Table:");
        for item in &role_permissions_results {
            println!("{:?}", item);
        }
    }

    // Store role permissions in session if not already there
    if session_data.is_none()
        || !session_data
            .as_ref()
            .unwrap()
            .contains_key("role_permissions")
    {
        // In a real implementation, you would update the session here
        log::debug!("Would store role_permissions in session");
    }

    // Get record permissions
    let record_permissions_cache_key = sha1(&format!(
        "{}_record_permissions:{}:{}",
        table, custom_suffix, account_organization_id
    ));
    // Use the record_valid_pass_keys_query_params from DataPermissions
    let record_permissions_params = group_by_field_record_permissions_params;

    let record_permissions_results: Vec<GroupByFieldRecordPermissionsResult> =
        get_from_cache_or_execute(
            &record_permissions_cache_key,
            session_data.as_ref(),
            "record_permissions",
            PermissionQueryType::GroupByFieldRecordPermissions,
            record_permissions_params,
            &account_organization_id,
            expiry_ms,
        )
        .await?;

    // Convert record permissions to JSON for compatibility with existing code
    let record_permissions = serde_json::json!({
        "data": record_permissions_results,
        "account_organization_id": account_organization_id,
        "cache": true,
    });

    log::debug!("Getting record_permissions permissions");
    log::debug!("Getting record_permissions permissions completed.");

    // Debug output if enabled
    if std::env::var("DEBUG").unwrap_or_else(|_| "false".to_string()) == "true" {
        println!("Record Permissions Table:");
        for item in &record_permissions_results {
            println!("{:?}", item);
        }
    }

    // Store record permissions in session if not already there
    if session_data.is_none()
        || !session_data
            .as_ref()
            .unwrap()
            .contains_key("record_permissions")
    {
        // In a real implementation, you would update the session here
        log::debug!("Would store record_permissions in session");
    }

    // Get valid pass keys
    let valid_pass_keys_cache_key = sha1(&format!("{}_valid_pass_keys:{}", table, custom_suffix));
    // Use the valid_pass_keys_query_params from DataPermissions
    let valid_pass_keys_params = valid_pass_keys_query_params;

    let valid_pass_keys_results: Vec<ValidPassKeyResult> = get_from_cache_or_execute(
        &valid_pass_keys_cache_key,
        session_data.as_ref(),
        "valid_pass_keys",
        PermissionQueryType::ValidPassKeys,
        valid_pass_keys_params,
        &account_organization_id,
        expiry_ms,
    )
    .await?;

    // Convert valid pass keys to JSON for compatibility with existing code
    let valid_pass_keys = serde_json::json!({
        "data": valid_pass_keys_results,
        "account_organization_id": account_organization_id,
        "cache": true,
    });

    log::debug!("Getting valid_pass_keys permissions");
    log::debug!("Getting valid_pass_keys permissions completed.");

    // Debug output if enabled
    if std::env::var("DEBUG").unwrap_or_else(|_| "false".to_string()) == "true" {
        println!("Valid Pass Keys Table:");
        for item in &valid_pass_keys_results {
            println!("{:?}", item);
        }
    }

    // Store valid pass keys in session if not already there
    if session_data.is_none()
        || !session_data
            .as_ref()
            .unwrap()
            .contains_key("valid_pass_keys")
    {
        // In a real implementation, you would update the session here
        log::debug!("Would store valid_pass_keys in session");
    }

    // Return the combined permissions data
    let combined_permissions = serde_json::json!({
        "field_permissions": field_permissions,
        "role_permissions": role_permissions,
        "record_permissions": record_permissions,
        "valid_pass_keys": valid_pass_keys,
        "request_type": match request_type {
            RequestType::Read => "read",
            RequestType::Write => "write",
        },
    });

    Ok((error, combined_permissions))
}
