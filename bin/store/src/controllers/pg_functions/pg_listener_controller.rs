use actix_web::{web, HttpRequest, HttpResponse, Responder};
use crate::structs::structs::ApiResponse;
use crate::db::get_async_connection;
use serde_json::Value;
use serde::{Deserialize, Serialize};
use regex::Regex;
use diesel::sql_query;
use diesel_async::RunQueryDsl;
use super::function_validators::FunctionValidator;

#[derive(Deserialize)]
struct CreateFunctionRequest {
    #[serde(rename = "function")]
    function_string: String,
    table_name: String,
}

#[derive(Deserialize)]
struct TestFunctionRequest {
    #[serde(rename = "function")]
    function_string: String,
}

struct PgFunctionService;

impl PgFunctionService {
    async fn create_pg_function(&self, request: CreateFunctionRequest) -> Result<ApiResponse, String> {
        let function_string = &request.function_string;
        let table_name = &request.table_name;

        if function_string.is_empty() {
            return Err("No function string found".to_string());
        }

        if table_name.is_empty() {
            return Err("No table name found, it is used to create trigger".to_string());
        }

        // Use the comprehensive validator to validate the function
        let validator = FunctionValidator::new();
        
        // Perform all validation checks before creating the function
        validator.validate_function(function_string).await?;
        
        // Extract function name and channel name for trigger creation
        let function_name = validator.extract_function_name(function_string)?;
        let channel_name = validator.extract_channel_name(function_string)?;

        println!("🦒 PgFunctionService -> function_name: {}", function_name);

        // Create database connection using Diesel async pool
        let mut conn = get_async_connection().await;

        // Execute the function creation using diesel::sql_query
        match sql_query(function_string).execute(&mut conn).await {
            Ok(_) => {
                println!("✅ Function created successfully");
            }
            Err(e) => {
                let error_msg = format!("Error executing function string: {}", e);
                println!("🚜 PgFunctionService -> error: {}", error_msg);
                return Err(error_msg);
            }
        }

        // Create trigger if it doesn't exist
        let trigger_sql = format!(
            r#"DO $$ 
            BEGIN 
              IF NOT EXISTS ( 
                SELECT 1 FROM pg_trigger WHERE tgname = '{}_trigger' 
              ) THEN 
                CREATE TRIGGER {}_trigger 
                AFTER INSERT ON {} 
                FOR EACH ROW EXECUTE FUNCTION {}(); 
              END IF; 
            END; 
            $$;"
            "#,
            channel_name, channel_name, table_name, channel_name
        );

        match sql_query(&trigger_sql).execute(&mut conn).await {
            Ok(_) => {
                println!("🚳 PgFunctionService -> trigger created successfully");
            }
            Err(e) => {
                let error_msg = format!("Error creating trigger: {}", e);
                println!("🚜 PgFunctionService -> trigger error: {}", error_msg);
                return Err(error_msg);
            }
        }

        Ok(ApiResponse {
            success: true,
            message: "pgFunction created successfully".to_string(),
            count: 0,
            data: vec![],
        })
    }

    async fn get_listener(&self, _req: &HttpRequest) -> Result<ApiResponse, String> {
        // TODO: Implement get listener logic
        Ok(ApiResponse {
            success: true,
            message: "Listener retrieved successfully".to_string(),
            count: 0,
            data: vec![],
        })
    }

    async fn delete_listener(&self, _req: &HttpRequest, function_name: &str) -> Result<ApiResponse, String> {
        // TODO: Implement delete listener logic
        Ok(ApiResponse {
            success: true,
            message: format!("Listener '{}' deleted successfully", function_name),
            count: 0,
            data: vec![],
        })
    }

    async fn test_function_syntax(&self, request: TestFunctionRequest) -> Result<ApiResponse, String> {
        let function_string = &request.function_string;

        if function_string.is_empty() {
            return Err("No function string found".to_string());
        }

        // Use the validator to test function syntax without creating it
        let validator = FunctionValidator::new();
        
        // Perform comprehensive validation including syntax testing
        match validator.validate_function(function_string).await {
            Ok(_) => {
                Ok(ApiResponse {
                    success: true,
                    message: "Function syntax is valid and would execute successfully".to_string(),
                    count: 0,
                    data: vec![],
                })
            }
            Err(error) => {
                Ok(ApiResponse {
                    success: false,
                    message: format!("Function validation failed: {}", error),
                    count: 0,
                    data: vec![],
                })
            }
        }
    }
}

pub async fn create_pg_function(
    req: HttpRequest,
    body: web::Json<CreateFunctionRequest>,
) -> impl Responder {
    let service = PgFunctionService;
    
    match service.create_pg_function(body.into_inner()).await {
        Ok(response) => HttpResponse::Ok().json(response),
        Err(error) => HttpResponse::BadRequest().json(ApiResponse {
            success: false,
            message: error,
            count: 0,
            data: vec![],
        }),
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
pub async fn pg_listener_get(
    req: HttpRequest,
) -> impl Responder {
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
pub async fn pg_listener_delete(
    req: HttpRequest,
    path: web::Path<String>,
) -> impl Responder {
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

/// Configure routes for the PG Listener controller
pub fn configure_pg_listener_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/listener")
            .route("/function", web::post().to(create_pg_function))
            .route("/test", web::post().to(test_pg_function_syntax))
            .route("/", web::get().to(pg_listener_get))
            .route("/{function_name}", web::delete().to(pg_listener_delete))
    );
}