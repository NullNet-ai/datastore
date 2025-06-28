use crate::auth::structs::Session;
use crate::permissions::permissions_queries::{get_permissions_query, get_role_permissions_query};
use crate::permissions::structs::{DataPermissions, SchemaItem};
use crate::schema::schema::account_organizations::role_id;
use crate::structs::structs::{ApiResponse, Auth};
use actix_web::error::ErrorUnauthorized;
use actix_web::http::Method;
use actix_web::{dev::Payload, Error, FromRequest, HttpMessage, HttpRequest};
use futures::future::{ready, Ready};
use serde::{Deserialize, Serialize};
use std::env;
// Custom extractor for permissions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PermissionExtractor {
    pub data_permissions: DataPermissions,
}

impl FromRequest for PermissionExtractor {
    type Error = Error;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        // Get auth data from request extensions
        let auth = req.extensions().get::<Auth>().cloned();
        let session = req.extensions().get::<Session>().cloned();
        println!("{:?}", auth);

        let user_role_id = session
            .as_ref()
            .map(|s| s.user.role_id.clone())
            .unwrap_or_else(|| String::new());

        // Get path parameters
        let table = req
            .match_info()
            .get("table")
            .map(|s| s.to_string())
            .unwrap_or_default();

        let id = req.match_info().get("id").map(|s| s.to_string());

        // Get method and query string
        let method = req.method().clone();
        let query_string = req.query_string().to_string();

        let experimental_permissions =
            env::var("EXPERIMENTAL_PERMISSIONS").unwrap_or_else(|_| "false".to_string()) == "true";

        // Process permissions
        if !experimental_permissions {
            // Return default permissions
            ready(Ok(PermissionExtractor {
                data_permissions: DataPermissions::default(),
            }))
        } else if let Some(a) = auth {
            let mut data_permissions = DataPermissions::default();

            // Only process if we have a table
            if !table.is_empty() {
                let method_str = method.as_str().to_string();
                let is_read_request = method == Method::GET || method == Method::POST;
                let is_write_request =
                    method == Method::POST || method == Method::PATCH || method == Method::DELETE;

                // Build read endpoint
                let read_method = if is_read_request { "GET" } else { "POST" };
                let single_read_record = if is_read_request && id.is_some() {
                    id.clone().unwrap_or_default()
                } else {
                    String::new()
                };

                let read_path_segment = if single_read_record.is_empty() {
                    "/filter".to_string()
                } else {
                    format!("/{}", single_read_record)
                };

                let read_query_str = if !query_string.is_empty() {
                    format!("?{}", query_string)
                } else {
                    String::new()
                };

                let read_endpoint = format!(
                    "{}:/api/store/{}{}{}",
                    read_method, table, read_path_segment, read_query_str
                );

                // Build write endpoint
                let write_method = if is_write_request {
                    method_str.clone()
                } else {
                    String::new()
                };

                let single_write_record =
                    if (method == Method::PATCH || method == Method::DELETE) && id.is_some() {
                        id.clone().unwrap_or_default()
                    } else {
                        String::new()
                    };

                let write_path_segment = if single_write_record.is_empty() {
                    "".to_string()
                } else {
                    format!("/{}", single_write_record)
                };

                let write_query_str = if !query_string.is_empty() {
                    format!("?{}", query_string)
                } else {
                    String::new()
                };

                let write_endpoint = format!(
                    "{}:/api/store/{}{}{}",
                    write_method, table, write_path_segment, write_query_str
                );

                // Compare with actual request
                let path = req.path().to_string();
                let request_info = format!("{method_str}:{path}{}", read_query_str);

                match request_info {
                    ref endpoint if endpoint == &read_endpoint => {
                        // Handle read endpoint
                        data_permissions.account_organization_id = a.responsible_account.clone();

                        // Set up role permissions query
                        data_permissions.role_permissions_query =
                            get_role_permissions_query(&user_role_id);

                        data_permissions.query = get_permissions_query(
                            &[table.clone()],
                            None,
                            a.sensitivity_level,
                            &a.responsible_account,
                        );
                    }
                    ref endpoint if endpoint == &write_endpoint => {
                        // Handle write endpoint
                        data_permissions.account_organization_id = a.responsible_account.clone();

                        // Set up role permissions query
                        data_permissions.role_permissions_query =
                            get_role_permissions_query(&user_role_id);

                        data_permissions.query = get_permissions_query(
                            &[table.clone()],
                            None,
                            a.sensitivity_level,
                            &a.responsible_account,
                        );

                        // Note: The actual call to accumulateWriteInformation would happen in the controller
                        // after the PermissionExtractor is created, since it needs the request body
                    }
                    _ => {
                        // No match with known endpoints
                    }
                }
            }

            // Return the extractor with permissions
            ready(Ok(PermissionExtractor { data_permissions }))
        } else {
            // Handle unauthorized case
            let error_response = ApiResponse {
                success: false,
                message: "Authentication required".to_string(),
                count: 0,
                data: vec![],
            };

            ready(Err(ErrorUnauthorized(error_response)))
        }
    }
}

// ... existing code ...

impl PermissionExtractor {
    pub fn accumulateWriteInformation(&mut self, params: &serde_json::Value) -> serde_json::Value {
        // Extract the necessary fields from params
        let table = params["table"].as_str().unwrap_or_default().to_string();
        let body = params["body"]
            .as_object()
            .unwrap_or(&serde_json::Map::new())
            .clone();
        let mut tables = params["tables"]
            .as_array()
            .unwrap_or(&Vec::new())
            .iter()
            .map(|v| v.as_str().unwrap_or_default().to_string())
            .collect::<Vec<String>>();
        let mut schema = self.data_permissions.schema.clone();
        let mut main_fields = params["main_fields"]
            .as_array()
            .unwrap_or(&Vec::new())
            .iter()
            .map(|v| v.as_str().unwrap_or_default().to_string())
            .collect::<Vec<String>>();

        // Debug log
        println!("accumulateWriteInformation");

        // Push table to tables
        tables.push(table.clone());

        // Get main fields from body
        main_fields = body.keys().map(|k| k.to_string()).collect();

        // Add id to main_fields if it exists in params
        if let Some(id) = params["params"].get("id") {
            if !id.is_null() {
                main_fields.push("id".to_string());
            }
        }

        // Process each field
        for field in &main_fields {
            let field_value = body.get(field).map(|v| v.to_string()).unwrap_or_default();

            schema.push(SchemaItem {
                entity: table.clone(),
                alias: String::new(),
                field: field.clone(),
                property_name: String::new(),
                path: format!("{} = {}", field, field_value),
            });
        }

        // Update data_permissions schema
        self.data_permissions.schema = schema;

        // Return the result
        serde_json::json!({
            "tables": tables,
            "main_fields": Vec::<String>::new(),
            "requested_fields": main_fields,
        })
    }
}
