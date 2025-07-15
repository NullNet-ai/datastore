use super::function_validators::FunctionValidator;
use crate::controllers::common_controller::process_and_insert_record;
use crate::db::get_async_connection;
use crate::structs::structs::{ApiResponse, Auth};
use actix_web::{web, HttpMessage, HttpRequest, HttpResponse, Responder};
use diesel::sql_query;
use diesel::sql_types::Text;
use diesel::QueryableByName;
use diesel_async::RunQueryDsl;
use regex::Regex;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::collections::HashMap;

#[derive(QueryableByName, Debug)]
struct FunctionRow {
    #[diesel(sql_type = Text)]
    name: String,
}

#[derive(QueryableByName, Debug)]
struct TriggerRow {
    #[diesel(sql_type = Text)]
    name: String,
    #[diesel(sql_type = Text)]
    table_name: String,
}

#[derive(Deserialize, Clone)]
pub struct CreateFunctionRequest {
    #[serde(rename = "function")]
    function_string: String,
    table_name: String,
}

#[derive(Deserialize)]
pub struct TestFunctionRequest {
    #[serde(rename = "function")]
    function_string: String,
}

struct PgFunctionService;

impl PgFunctionService {
    async fn create_pg_function(
        &self,
        request: CreateFunctionRequest,
        create_trigger: bool,
    ) -> Result<ApiResponse, String> {
        let function_string = &request.function_string;
        let table_name = &request.table_name;

        if function_string.is_empty() {
            return Err("No function string found".to_string());
        }

        if table_name.is_empty() && create_trigger {
            return Err("No table name found, it is required to create trigger".to_string());
        }

        // Use the comprehensive validator to validate the function
        let validator = FunctionValidator::new();

        // Perform all validation checks before creating the function
        validator.validate_function(function_string).await?;
        validator.validate_channel_function_format(function_string)?;

        // Extract function name and channel name for trigger creation
        let function_name = validator.extract_function_name(function_string)?;
        let channel_name = validator.extract_channel_name(function_string)?;

        log::debug!("🦒 PgFunctionService -> function_name: {}", function_name);

        // Create database connection using Diesel async pool
        let mut conn = get_async_connection().await;

        // Execute the function creation using diesel::sql_query
        match sql_query(function_string).execute(&mut conn).await {
            Ok(_) => {
                log::debug!("✅ Function created successfully");
            }
            Err(e) => {
                let error_msg = format!("Error executing function string: {}", e);
                log::error!("🚜 PgFunctionService -> error: {}", error_msg);
                return Err(error_msg);
            }
        }

        // Create trigger if requested
        if create_trigger {
            self.create_trigger(&channel_name, table_name).await?;
        }

        let message = if create_trigger {
            "pgFunction and trigger created successfully".to_string()
        } else {
            "pgFunction created successfully (trigger skipped)".to_string()
        };

        Ok(ApiResponse {
            success: true,
            message,
            count: 0,
            data: vec![],
        })
    }

    async fn create_trigger(&self, channel_name: &str, table_name: &str) -> Result<(), String> {
        let trigger_sql = format!(
            r#"DO $$ 
    BEGIN 
      IF NOT EXISTS ( 
        SELECT 1 FROM pg_trigger WHERE tgname = '{}_trigger' 
      ) THEN 
        CREATE TRIGGER {}_trigger 
        AFTER INSERT OR UPDATE ON {} 
        FOR EACH ROW 
        WHEN (NEW.sync_status = 'complete') 
        EXECUTE FUNCTION {}(); 
      END IF; 
    END; 
    $$;"#,
            channel_name, channel_name, table_name, channel_name
        );

        let mut conn = get_async_connection().await;
        match sql_query(&trigger_sql).execute(&mut conn).await {
            Ok(_) => Ok(()),
            Err(e) => {
                let error_msg = format!("Error creating trigger: {}", e);
                log::debug!("🚜 PgFunctionService -> trigger error: {}", error_msg);
                Err(error_msg)
            }
        }
    }

    async fn get_listener(&self, _req: &HttpRequest) -> Result<ApiResponse, String> {
        let mut conn = get_async_connection().await;

        // Query for functions
        let function_query = r#"
            SELECT p.proname AS name 
            FROM pg_proc p 
            JOIN pg_namespace n ON n.oid = p.pronamespace 
            WHERE n.nspname NOT IN ('pg_catalog', 'information_schema') 
              AND pg_function_is_visible(p.oid) 
              AND p.prorettype = 'trigger'::regtype 
            ORDER BY p.proname;
        "#;

        // Query for triggers
        let trigger_query = r#"
            SELECT 
                tg.tgname AS name, 
                cls.relname AS table_name 
            FROM pg_trigger tg 
                JOIN pg_class cls ON cls.oid = tg.tgrelid 
                JOIN pg_namespace n ON n.oid = cls.relnamespace 
            WHERE tg.tgisinternal = false 
              AND n.nspname NOT IN ('pg_catalog', 'information_schema', 'pg_toast') 
              AND tg.tgname NOT LIKE 'pg_%' 
            ORDER BY tg.tgname;
        "#;

        // Execute function query
        let function_results = match sql_query(function_query)
            .load::<FunctionRow>(&mut conn)
            .await
        {
            Ok(results) => results,
            Err(e) => {
                return Err(format!("Failed to query functions: {}", e));
            }
        };

        // Execute trigger query
        let trigger_results = match sql_query(trigger_query).load::<TriggerRow>(&mut conn).await {
            Ok(results) => results,
            Err(e) => {
                return Err(format!("Failed to query triggers: {}", e));
            }
        };

        // Convert results to JSON
        let functions: Vec<String> = function_results.into_iter().map(|row| row.name).collect();
        let triggers: Vec<Value> = trigger_results
            .into_iter()
            .map(|row| {
                json!({
                    "name": row.name,
                    "table_name": row.table_name
                })
            })
            .collect();

        let result = json!({
            "functions": functions,
            "triggers": triggers
        });

        Ok(ApiResponse {
            success: true,
            message: "pgListenerGet Message".to_string(),
            count: 0,
            data: vec![result],
        })
    }

    async fn delete_listener(
        &self,
        req: &HttpRequest,
        function_name: &str,
    ) -> Result<ApiResponse, String> {
        // Extract table_name from query parameters
        let query_string = req.query_string();
        let table_name = if let Some(table_param) = query_string
            .split('&')
            .find(|param| param.starts_with("table_name="))
        {
            table_param.split('=').nth(1).unwrap_or("")
        } else {
            return Err("table_name query parameter is required".to_string());
        };

        if table_name.is_empty() {
            return Err("table_name cannot be empty".to_string());
        }

        let mut conn = get_async_connection().await;

        // Use transaction to ensure all operations succeed or fail together
        let result = conn
            .build_transaction()
            .run::<_, diesel::result::Error, _>(|conn| {
                Box::pin(async move {
                    // Delete from postgres_channels table
                    let delete_query = format!(
                        "DELETE FROM postgres_channels WHERE channel_name = '{}'",
                        function_name
                    );
                    sql_query(&delete_query).execute(conn).await?;

                    // Drop the function using CASCADE
                    let drop_function_query =
                        format!("DROP FUNCTION IF EXISTS {} CASCADE", function_name);
                    sql_query(&drop_function_query).execute(conn).await?;

                    // Drop the trigger
                    let trigger_name = format!("{}_trigger", function_name);
                    let drop_trigger_query = format!(
                        "DROP TRIGGER IF EXISTS {} ON {} CASCADE",
                        trigger_name, table_name
                    );
                    sql_query(&drop_trigger_query).execute(conn).await?;

                    Ok(())
                })
            })
            .await;

        match result {
            Ok(_) => Ok(ApiResponse {
                success: true,
                message: format!(
                    "Successfully deleted pgListener components for {}: {} and trigger {}_trigger",
                    table_name, function_name, function_name
                ),
                count: 0,
                data: vec![],
            }),
            Err(error) => Err(format!("Error deleting pgListener: {}", error)),
        }
    }

    async fn test_function_syntax(
        &self,
        request: TestFunctionRequest,
    ) -> Result<ApiResponse, String> {
        let function_string = &request.function_string;

        if function_string.is_empty() {
            return Err("No function string found".to_string());
        }

        // Use the validator to test function syntax without creating it
        let validator = FunctionValidator::new();

        // Perform comprehensive validation including syntax testing
        match validator.validate_function(function_string).await {
            Ok(_) => Ok(ApiResponse {
                success: true,
                message: "Function syntax is valid and would execute successfully".to_string(),
                count: 0,
                data: vec![],
            }),
            Err(error) => Ok(ApiResponse {
                success: false,
                message: format!("Function validation failed: {}", error),
                count: 0,
                data: vec![],
            }),
        }
    }
}

pub async fn create_pg_function(
    req: HttpRequest,
    body: web::Json<CreateFunctionRequest>,
    query: web::Query<HashMap<String, String>>,
) -> impl Responder {
    let service = PgFunctionService;

    // Extract trigger query parameter, default to true
    let create_trigger = query
        .get("trigger")
        .map(|v| v.parse::<bool>().unwrap_or(true))
        .unwrap_or(true);
    let auth_data = match req.extensions().get::<Auth>() {
        Some(data) => data.clone(),
        None => {
            log::warn!("Auth data not found in request extensions");
            return HttpResponse::InternalServerError().json(ApiResponse {
                success: false,
                message: "Authentication information not available".to_string(),
                count: 0,
                data: vec![],
            });
        }
    };

    let body_data = body.into_inner();

    // First, try to create the PG function
    match service
        .create_pg_function(body_data.clone(), create_trigger)
        .await
    {
        Ok(response) => {
            // Extract channel_name from function string using validator
            let validator = FunctionValidator::new();
            let channel_name = match validator.extract_channel_name(&body_data.function_string) {
                Ok(name) => name,
                Err(error) => {
                    return HttpResponse::BadRequest().json(ApiResponse {
                        success: false,
                        message: format!("Failed to extract channel name: {}", error),
                        count: 0,
                        data: vec![],
                    });
                }
            };

            // If PG function creation is successful, then process and insert record
            let record: Value = json!({
                "channel_name": channel_name,
                "function": body_data.function_string,
            });

            match process_and_insert_record("postgres_channels", record, None, &auth_data).await {
                Ok(_) => {
                    // Both PG function creation and record insertion successful
                    HttpResponse::Ok().json(response)
                }
                Err(insert_error) => {
                    // Check if it's a duplicate key constraint violation
                    if insert_error
                        .message
                        .contains("duplicate key value violates unique constraint")
                    {
                        // Function and trigger already exist in database, treat as success
                        HttpResponse::Ok().json(ApiResponse {
                            success: true,
                            message: "Function and trigger already exist in the database"
                                .to_string(),
                            count: 0,
                            data: vec![],
                        })
                    } else {
                        // Other insertion errors should still be treated as failures
                        HttpResponse::InternalServerError().json(ApiResponse {
                            success: false,
                            message: format!(
                                "PostgreSQL function created but failed to insert record: {}",
                                insert_error.message
                            ),
                            count: 0,
                            data: vec![],
                        })
                    }
                }
            }
        }
        Err(error) => {
            // If PG function creation fails, return error without processing record
            HttpResponse::BadRequest().json(ApiResponse {
                success: false,
                message: format!("Failed to create PostgreSQL function: {}", error),
                count: 0,
                data: vec![],
            })
        }
    }
}

/// Test PostgreSQL function syntax endpoint
/// POST /api/listener/test
pub async fn test_pg_function_syntax(
    req: HttpRequest,
    body: web::Json<TestFunctionRequest>,
) -> impl Responder {
    let service = PgFunctionService;

    match service.test_function_syntax(body.into_inner()).await {
        Ok(response) => HttpResponse::Ok().json(response),
        Err(error) => HttpResponse::BadRequest().json(ApiResponse {
            success: false,
            message: error,
            count: 0,
            data: vec![],
        }),
    }
}

/// Get listener endpoint
/// GET /api/listener/
pub async fn pg_listener_get(req: HttpRequest) -> impl Responder {
    let service = PgFunctionService;

    match service.get_listener(&req).await {
        Ok(response) => HttpResponse::Ok().json(response),
        Err(error) => HttpResponse::InternalServerError().json(ApiResponse {
            success: false,
            message: error,
            count: 0,
            data: vec![],
        }),
    }
}

/// Delete listener endpoint
/// DELETE /api/listener/{function_name}
pub async fn pg_listener_delete(req: HttpRequest, path: web::Path<String>) -> impl Responder {
    let function_name = path.into_inner();
    let service = PgFunctionService;

    match service.delete_listener(&req, &function_name).await {
        Ok(response) => HttpResponse::Ok().json(response),
        Err(error) => HttpResponse::InternalServerError().json(ApiResponse {
            success: false,
            message: error,
            count: 0,
            data: vec![],
        }),
    }
}
