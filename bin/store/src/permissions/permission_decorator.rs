use std::collections::HashMap;

use crate::auth::structs::Session;
use crate::permissions::permission_utils::{get_cached_permissions, PermissionsContext};
use crate::permissions::permissions_queries::PermissionQueryParams;
use crate::permissions::structs::{DataPermissions, SchemaItem};
use crate::structs::structs::{ApiResponse, Auth};
use crate::utils::request_type_handler::{RequestType, RequestTypeHandler};
use actix_web::web::BytesMut;
use actix_web::ResponseError;
use actix_web::{dev::Payload, FromRequest, HttpMessage, HttpRequest};
use futures::FutureExt;
use futures_util::future::LocalBoxFuture;
use futures_util::stream::StreamExt;
use serde::{Deserialize, Serialize};
use serde_json::Value;

// Custom extractor for permissions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PermissionExtractor {
    pub data_permissions: DataPermissions,
    pub request_body: serde_json::Value,
    pub request_type: RequestType,
}
#[allow(warnings)]
impl FromRequest for PermissionExtractor {
    type Error = Box<dyn ResponseError>;
    type Future = LocalBoxFuture<'static, Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, payload: &mut Payload) -> Self::Future {
        // Extract everything you need outside the async block
        let auth = req.extensions().get::<Auth>().cloned();
        let session = req.extensions().get::<Session>().cloned();
        let session_data = req.extensions().get::<HashMap<String, Value>>().cloned();
        let user_role_id = session
            .as_ref()
            .map(|s| s.user.role_id.clone())
            .unwrap_or_default();
        let table = req
            .match_info()
            .get("table")
            .unwrap_or_default()
            .to_string();
        let id = req.match_info().get("id").map(|s| s.to_string());
        let method = req.method().clone();
        let host = req.connection_info().host().to_string();
        let headers = req.headers().clone(); // Clone headers if needed
        let metadata: HashMap<String, serde_json::Value> = HashMap::new();
        let query_params = req
            .query_string()
            .split('&')
            .filter(|s| !s.is_empty())
            .filter_map(|pair| {
                let mut parts = pair.splitn(2, '=');
                match (parts.next(), parts.next()) {
                    (Some(key), Some(value)) => Some((key.to_string(), value.to_string())),
                    _ => None,
                }
            })
            .collect::<HashMap<String, String>>();
        let path = req.path().to_string();
        let uri = req.uri().clone();
        let experimental_permissions =
            std::env::var("EXPERIMENTAL_PERMISSIONS").unwrap_or_else(|_| "false".into()) == "true";

        // move payload safely
        let mut payload = std::mem::replace(payload, Payload::None);

        let fut = async move {
            if session.is_none() {
                return Err(Box::new(ApiResponse {
                    success: false,
                    message: "Session not found, please log in".into(),
                    count: 0,
                    data: vec![],
                }) as Box<dyn ResponseError>);
            }
            // Read body
            let mut body = BytesMut::new();
            while let Some(chunk) = payload.next().await {
                let chunk = chunk.map_err(|e| {
                    Box::new(ApiResponse {
                        success: false,
                        message: format!("Failed to read request body: {}", e),
                        count: 0,
                        data: vec![],
                    }) as Box<dyn ResponseError>
                })?;
                body.extend_from_slice(&chunk);
            }

            let body_json: Value =
                serde_json::from_slice(&body).unwrap_or_else(|_| serde_json::json!({}));

            let request_type =
                RequestTypeHandler::get_request_type(&method, &path).unwrap_or(RequestType::Read);

            if !experimental_permissions {
                return Ok(PermissionExtractor {
                    data_permissions: DataPermissions::default(),
                    request_body: body_json,
                    request_type,
                });
            }

            let mut data_permissions = DataPermissions::default();

            if let Some(a) = auth {
                // Build permission logic...
                let mut requested_fields: Vec<String> = Vec::new();
                let mut main_fields: Vec<String> = Vec::new();
                let mut tables: Vec<String> = Vec::new();

                match request_type {
                    RequestType::Write => {
                        // Create params object for accumulate_write_information
                        let params_obj = serde_json::json!({
                            "params": {
                                "id": id,
                            },
                            "body": body_json.clone(),
                            "table": table,
                            "tables": Vec::<String>::new(),
                            "main_fields": Vec::<String>::new(),
                            "schema": Vec::<SchemaItem>::new(),
                        });

                        // Call static accumulate_write_information
                        let result = PermissionExtractor::accumulate_write_information(
                            &mut data_permissions,
                            &params_obj,
                        );

                        // Extract requested_fields
                        requested_fields = result["requested_fields"]
                            .as_array()
                            .unwrap_or(&Vec::new())
                            .iter()
                            .map(|v| v.as_str().unwrap_or_default().to_string())
                            .collect::<Vec<String>>();

                        main_fields = result["main_fields"]
                            .as_array()
                            .unwrap_or(&Vec::new())
                            .iter()
                            .map(|v| v.as_str().unwrap_or_default().to_string())
                            .collect::<Vec<String>>();

                        tables = result["tables"]
                            .as_array()
                            .unwrap_or(&Vec::new())
                            .iter()
                            .map(|v| v.as_str().unwrap_or_default().to_string())
                            .collect::<Vec<String>>();

                        // Update data_permissions requested_fields
                    }
                    RequestType::Read => {
                        let mut schema = vec![];
                        main_fields = body_json
                            .as_object()
                            .map(|m| m.keys().cloned().collect::<Vec<String>>())
                            .unwrap_or_default();

                        for field in &main_fields {
                            let value = body_json
                                .get(field)
                                .map(|v| v.to_string())
                                .unwrap_or_default();
                            schema.push(SchemaItem {
                                entity: table.clone(),
                                alias: "".into(),
                                field: field.clone(),
                                property_name: "".into(),
                                path: format!("{field} = {value}"),
                            });
                        }

                        data_permissions.schema = schema;
                    } // _ => {}
                }
                data_permissions.account_organization_id = a.responsible_account.clone();
                data_permissions.role_permissions_query_params =
                    PermissionQueryParams::RolePermissions {
                        role_id: user_role_id.clone(),
                    };
                data_permissions.data_permissions_query_params =
                    PermissionQueryParams::DataPermissions {
                        tables: tables.clone(),
                        main_fields: main_fields.clone(),
                        sensitivity_level: a.sensitivity_level,
                        account_organization_id: a.responsible_account.clone(),
                    };

                data_permissions.valid_pass_keys_query_params =
                    PermissionQueryParams::ValidPassKeys {
                        organization_id: a.organization_id.clone(),
                        table: table.clone(),
                        pgp_sym_key: std::env::var("PGP_SYM_KEY").unwrap_or_default(),
                    };

                data_permissions.group_by_field_record_permissions_params =
                    PermissionQueryParams::GroupByFieldRecordPermissions {
                        table: table.clone(),
                        role_id: user_role_id.clone(),
                    };

                data_permissions.requested_fields = requested_fields;

                let session_unwrapped = session.unwrap();

                // println!("{:?}---------------------------host", host);
                // println!("{:?}---------------------------uri", uri);
                // println!("{:?}---------------------------headers", headers.get("user-agent"));
                // println!("{:?}---------------------------session", session_unwrapped);

                let permissions_result = get_cached_permissions(
                    request_type,
                    PermissionsContext {
                        permissions_query: data_permissions.clone(),
                        host,
                        headers,
                        table,
                        account_organization_id: a.responsible_account,
                        body: body_json.clone(),
                        metadata,
                        account_id: a.account_id,
                        query: query_params,
                        method,
                        uri,
                        session: session_unwrapped,
                        session_data,
                    },
                )
                .await
                .map_err(|e| {
                    Box::new(ApiResponse {
                        success: false,
                        message: format!("Permission error: {}", e),
                        count: 0,
                        data: vec![],
                    }) as Box<dyn ResponseError>
                })?;

                Ok(PermissionExtractor {
                    data_permissions,
                    request_body: body_json,
                    request_type,
                })
            } else {
                return Err(Box::new(ApiResponse {
                    success: false,
                    message: "Authentication required".into(),
                    count: 0,
                    data: vec![],
                }) as Box<dyn ResponseError>);
            }
        };
        fut.map(|r| r).boxed_local()
    }
}
#[allow(warnings)]
fn api_error<T: Into<String>>(msg: T) -> Box<dyn ResponseError> {
    Box::new(ApiResponse {
        success: false,
        message: msg.into(),
        count: 0,
        data: vec![],
    })
}
#[allow(warnings)]
impl PermissionExtractor {
    pub fn accumulate_write_information(
        data_permissions: &mut DataPermissions,
        params: &serde_json::Value,
    ) -> serde_json::Value {
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
        let mut schema = data_permissions.schema.clone();
        let mut main_fields = params["main_fields"]
            .as_array()
            .unwrap_or(&Vec::new())
            .iter()
            .map(|v| v.as_str().unwrap_or_default().to_string())
            .collect::<Vec<String>>();
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
        data_permissions.schema = schema;

        // Return the result
        serde_json::json!({
            "tables": tables,
            "main_fields": Vec::<String>::new(),
            "requested_fields": main_fields,
        })
    }
}
