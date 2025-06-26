use crate::structs::structs::{ApiResponse, Auth};
use actix_web::HttpMessage;
use actix_web::dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform};
use actix_web::http::Method;
use actix_web::web::Json;
use actix_web::{Error, HttpRequest};
use futures::future::{ok, Ready};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::env;
use std::future::Future;
use std::pin::Pin;
use std::collections::HashMap;
use std::sync::Arc;
use std::path::Path;

// Data permissions structure that matches the TypeScript implementation
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DataPermissions {
    pub requested_fields: Vec<String>,
    pub query: String,
    pub account_organization_id: String,
    pub schema: Vec<SchemaItem>,
    pub valid_pass_keys_query: String,
    pub record_valid_pass_keys_query: String,
    pub role_permissions_query: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SchemaItem {
    pub entity: String,
    pub alias: String,
    pub field: String,
    pub property_name: String,
    pub path: String,
}

// Permissions middleware struct
pub struct Permissions;

// Middleware factory implementation
impl<S, B> Transform<S, ServiceRequest> for Permissions
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = PermissionsMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(PermissionsMiddleware { service })
    }
}

pub struct PermissionsMiddleware<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for PermissionsMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        // Get the auth data from the request extensions
        let auth = req.extensions().get::<Auth>().cloned();
        
        // Extract request information
        let method = req.method().clone();
        let path = req.path().to_string();
        let query_string = req.query_string().to_string();
        
        // Parse the path to extract table and id
        let path_segments: Vec<&str> = path.split('/').collect();
        let table = path_segments.get(3).map(|s| s.to_string());
        let id = if path_segments.len() > 4 { path_segments.get(4).map(|s| s.to_string()) } else { None };
        
        // Create a clone of the service
        let service = self.service.clone();
        
        Box::pin(async move {
            // Check if experimental permissions are enabled
            let experimental_permissions = env::var("EXPERIMENTAL_PERMISSIONS")
                .unwrap_or_else(|_| "false".to_string()) == "true";
            
            if !experimental_permissions {
                // If permissions are not enabled, just pass through
                let data_permissions = DataPermissions::default();
                req.extensions_mut().insert(data_permissions);
                return service.call(req).await;
            }
            
            // Get the auth data
            let auth = match auth {
                Some(auth) => auth,
                None => {
                    let error_response = ApiResponse {
                        success: false,
                        message: "Authentication required".to_string(),
                        count: 0,
                        data: vec![],
                    };
                    
                    return Err(actix_web::error::ErrorUnauthorized(error_response));
                }
            };
            
            // Extract request body if available
            let mut data_permissions = DataPermissions::default();
            
            // Process based on request method and path
            if let (Some(table), Some(method_str)) = (table, method_to_string(&method)) {
                let is_read_request = is_read_method(&method);
                let is_write_request = is_write_method(&method);
                
                // Construct endpoint strings similar to TypeScript implementation
                let read_method = if is_read_request { "GET" } else { "POST" };
                let single_read_record = if is_read_request && id.is_some() { id.clone().unwrap() } else { String::new() };
                let read_endpoint = format!("{read_method}:/api/store/{table}{}{query_str}", 
                    if single_read_record.is_empty() { "/filter" } else { &format!("/{}" , single_read_record) },
                    query_str = if !query_string.is_empty() { format!("?{}", query_string) } else { String::new() });
                
                let write_method = if is_write_method(&method) { method_str } else { String::new() };
                let single_write_record = if (method == Method::PATCH || method == Method::DELETE) && id.is_some() { 
                    id.clone().unwrap() 
                } else { 
                    String::new() 
                };
                let write_endpoint = format!("{write_method}:/api/store/{table}{}{query_str}",
                    if single_write_record.is_empty() { "" } else { &format!("/{}" , single_write_record) },
                    query_str = if !query_string.is_empty() { format!("?{}", query_string) } else { String::new() });
                
                // Construct the request info string
                let request_info = format!("{method_str}:{path}{query_str}",
                    query_str = if !query_string.is_empty() { format!("?{}", query_string) } else { String::new() });
                
                // Determine if we need to process read or write permissions
                if request_info == read_endpoint {
                    // Process read permissions
                    // In a real implementation, you would call functions similar to accumulateReadInformation
                    // For now, we'll just set up the basic structure
                    data_permissions.account_organization_id = auth.responsible_account.clone();
                    
                    // Here you would populate the data_permissions fields based on the read request
                    // This would involve parsing the request body and query parameters
                    
                    // For role-based permissions, you would use the role information from auth
                    if let Some(role_name) = &auth.role_name {
                        data_permissions.role_permissions_query = crate::permissions::permissions_queries::get_role_permissions_query(role_name);
                    }
                    
                    // Set up queries for permissions
                    let tables = vec![table.clone()];
                    let role_level = auth.role_level.unwrap_or(0) as i32;
                    data_permissions.query = crate::permissions::permissions_queries::get_permissions_query(
                        &tables, 
                        None, 
                        role_level,
                        &auth.responsible_account
                    );
                } else if request_info == write_endpoint {
                    // Process write permissions
                    // Similar to read permissions, but for write operations
                    data_permissions.account_organization_id = auth.responsible_account.clone();
                    
                    // Here you would populate the data_permissions fields based on the write request
                    
                    // For role-based permissions
                    if let Some(role_name) = &auth.role_name {
                        data_permissions.role_permissions_query = crate::permissions::permissions_queries::get_role_permissions_query(role_name);
                    }
                    
                    // Set up queries for permissions
                    let tables = vec![table.clone()];
                    let role_level = auth.role_level.unwrap_or(0) as i32;
                    data_permissions.query = crate::permissions::permissions_queries::get_permissions_query(
                        &tables, 
                        None, 
                        role_level,
                        &auth.responsible_account
                    );
                }
            }
            
            // Store the data permissions in the request extensions
            req.extensions_mut().insert(data_permissions);
            
            // Continue with the request
            service.call(req).await
        })
    }
}

// Helper functions

fn method_to_string(method: &Method) -> Option<String> {
    Some(method.as_str().to_string())
}

fn is_read_method(method: &Method) -> bool {
    method == Method::GET || method == Method::POST
}

fn is_write_method(method: &Method) -> bool {
    method == Method::POST || method == Method::PATCH || method == Method::DELETE
}

// In a complete implementation, you would add these functions:
// - accumulateReadInformation: to process read requests and extract field information
// - accumulateWriteInformation: to process write requests and extract field information