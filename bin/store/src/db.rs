use diesel_async::pooled_connection::deadpool::Pool as PoolAsync;
use diesel_async::pooled_connection::AsyncDieselConnectionManager;
use diesel_async::AsyncPgConnection;
use dotenv::dotenv;
use once_cell::sync::Lazy;
use std::env;

use tokio_postgres::{Client, NoTls};
use serde_json::{json, Value};
use std::collections::HashMap;
use tokio_postgres::types::Type;
use base64::prelude::*;



// -- Async Types --
pub type AsyncDbPool = PoolAsync<AsyncPgConnection>;
pub type AsyncDbPooledConnection =
    diesel_async::pooled_connection::deadpool::Object<AsyncPgConnection>;

static ASYNC_POOL: Lazy<AsyncDbPool> = Lazy::new(|| establish_async_pool());

pub fn establish_async_pool() -> AsyncDbPool {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let config = AsyncDieselConnectionManager::<AsyncPgConnection>::new(database_url);
    PoolAsync::builder(config)
        .max_size(20)
        .build()
        .expect("Failed to create async pool")
}

pub fn get_async_pool() -> &'static AsyncDbPool {
    &ASYNC_POOL
}

pub async fn get_async_connection() -> AsyncDbPooledConnection {
    get_async_pool().get().await.unwrap_or_else(|e| {
        log::error!("Failed to get async connection: {}", e);
        panic!("Async connection failure");
    })
}

//raw database connection
pub async fn create_connection() -> Result<Client, Box<dyn std::error::Error>> {
    let user = env::var("POSTGRES_USER").unwrap_or_else(|_| "admin".to_string());
    let password = env::var("POSTGRES_PASSWORD").unwrap_or_else(|_| "admin".to_string());
    let dbname = env::var("POSTGRES_DB").unwrap_or_else(|_| "test".to_string());
    let host = env::var("POSTGRES_HOST").unwrap_or_else(|_| "localhost".to_string());
    let port = env::var("POSTGRES_PORT").unwrap_or_else(|_| "5433".to_string());

    let connection_string = format!(
        "host={} port={} user={} password={} dbname={}",
        host, port, user, password, dbname
    );

    let (client, connection) = tokio_postgres::connect(&connection_string, NoTls)
        .await
        .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            log::error!("PostgreSQL connection error: {}", e);
        }
    });

    Ok(client)
}

// Raw database connection with access to connection for polling (used by pg_listener_service)
pub async fn create_connection_with_polling() -> Result<
    (
        Client,
        tokio_postgres::Connection<tokio_postgres::Socket, tokio_postgres::tls::NoTlsStream>,
    ),
    Box<dyn std::error::Error + Send + Sync>,
> {
    let user = env::var("POSTGRES_USER").unwrap_or_else(|_| "admin".to_string());
    let password = env::var("POSTGRES_PASSWORD").unwrap_or_else(|_| "admin".to_string());
    let dbname = env::var("POSTGRES_DB").unwrap_or_else(|_| "test".to_string());
    let host = env::var("POSTGRES_HOST").unwrap_or_else(|_| "localhost".to_string());
    let port = env::var("POSTGRES_PORT").unwrap_or_else(|_| "5433".to_string());

    let connection_string = format!(
        "host={} port={} user={} password={} dbname={}",
        host, port, user, password, dbname
    );

    let (client, connection) = tokio_postgres::connect(&connection_string, NoTls)
        .await
        .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)?;

    Ok((client, connection))
}

/// Centralized database type converter for bidirectional conversions
/// between PostgreSQL types and JSON values
pub struct DatabaseTypeConverter;

impl DatabaseTypeConverter {
    /// Convert serde_json::Value parameters to PostgreSQL-compatible types
    /// This replaces the panic-prone convert_params_to_sql_types function
    pub fn values_to_sql_params(
        params: &[Value],
    ) -> Result<Vec<Box<dyn tokio_postgres::types::ToSql + Sync + Send>>, String> {
        let mut converted_values: Vec<Box<dyn tokio_postgres::types::ToSql + Sync + Send>> = Vec::new();

        for (index, param) in params.iter().enumerate() {
            let boxed_value = Self::value_to_sql_param(param)
                .map_err(|e| format!("Error converting parameter at index {}: {}", index, e))?;
            converted_values.push(boxed_value);
        }

        Ok(converted_values)
    }

    /// Convert a single JSON value to SQL parameter
    fn value_to_sql_param(
        value: &Value,
    ) -> Result<Box<dyn tokio_postgres::types::ToSql + Sync + Send>, String> {
        let boxed_value: Box<dyn tokio_postgres::types::ToSql + Sync + Send> = match value {
            Value::Null => Box::new(None::<String>),
            Value::Bool(b) => Box::new(*b),
            Value::Number(n) => {
                if let Some(i) = n.as_i64() {
                    // Handle integer range appropriately
                    if i >= i32::MIN as i64 && i <= i32::MAX as i64 {
                        Box::new(i as i32)
                    } else {
                        Box::new(i)
                    }
                } else if let Some(u) = n.as_u64() {
                    // Handle unsigned integers
                    if u <= i32::MAX as u64 {
                        Box::new(u as i32)
                    } else if u <= i64::MAX as u64 {
                        Box::new(u as i64)
                    } else {
                        // For very large numbers, convert to string
                        Box::new(u.to_string())
                    }
                } else if let Some(f) = n.as_f64() {
                    Box::new(f)
                } else {
                    return Err("Invalid number format".to_string());
                }
            }
            Value::String(s) => {
                // Try to parse as IP address for inet fields
                if let Ok(ip) = s.parse::<std::net::IpAddr>() {
                    Box::new(ip)
                } else {
                    Box::new(s.clone())
                }
            }
            Value::Array(arr) => {
                // Convert array elements to Vec<String>
                let string_array: Vec<String> = arr
                    .iter()
                    .map(|v| match v {
                        Value::String(s) => s.clone(),
                        _ => v.to_string(),
                    })
                    .collect();
                Box::new(string_array)
            }
            Value::Object(_) => {
                // For objects, serialize to JSON string
                Box::new(value.to_string())
            }
        };
        Ok(boxed_value)
    }

    /// Convert PostgreSQL row to JSON value
    /// This replaces the row_to_value function with comprehensive type support
    pub fn row_to_json(row: &tokio_postgres::Row) -> Result<Value, String> {
        let mut obj = serde_json::Map::new();

        for i in 0..row.len() {
            let column_name = row.columns()[i].name();
            let column_type = row.columns()[i].type_();

            let value = Self::extract_column_value(row, i, column_type)
                .map_err(|e| format!("Error extracting column '{}': {}", column_name, e))?;

            if let Some(v) = value {
                obj.insert(column_name.to_string(), v);
            }
        }

        Ok(Value::Object(obj))
    }

    /// Extract value from a specific column with comprehensive type handling
    fn extract_column_value(
        row: &tokio_postgres::Row,
        column_index: usize,
        column_type: &Type,
    ) -> Result<Option<Value>, String> {
        let value = match column_type {
            // String types
            &Type::VARCHAR | &Type::TEXT | &Type::BPCHAR | &Type::NAME | &Type::CHAR => {
                row.try_get::<_, Option<String>>(column_index)
                    .map_err(|e| e.to_string())?
                    .map(|v| json!(v))
            }

            // Integer types
            &Type::INT2 => row
                .try_get::<_, Option<i16>>(column_index)
                .map_err(|e| e.to_string())?
                .map(|v| json!(v)),
            &Type::INT4 => row
                .try_get::<_, Option<i32>>(column_index)
                .map_err(|e| e.to_string())?
                .map(|v| json!(v)),
            &Type::INT8 => row
                .try_get::<_, Option<i64>>(column_index)
                .map_err(|e| e.to_string())?
                .map(|v| json!(v)),

            // Floating point types
            &Type::FLOAT4 => row
                .try_get::<_, Option<f32>>(column_index)
                .map_err(|e| e.to_string())?
                .map(|v| json!(v)),
            &Type::FLOAT8 => row
                .try_get::<_, Option<f64>>(column_index)
                .map_err(|e| e.to_string())?
                .map(|v| json!(v)),

            // Numeric type
            &Type::NUMERIC => row
                .try_get::<_, Option<String>>(column_index)
                .map_err(|e| e.to_string())?
                .map(|v| json!(v)),

            // Boolean type
            &Type::BOOL => row
                .try_get::<_, Option<bool>>(column_index)
                .map_err(|e| e.to_string())?
                .map(|v| json!(v)),

            // Date and time types
            &Type::DATE => {
                row.try_get::<_, Option<chrono::NaiveDate>>(column_index)
                    .map_err(|e| e.to_string())?
                    .map(|v| json!(v.to_string()))
            }
            &Type::TIME => {
                row.try_get::<_, Option<chrono::NaiveTime>>(column_index)
                    .map_err(|e| e.to_string())?
                    .map(|v| json!(v.to_string()))
            }
            &Type::TIMESTAMP => {
                row.try_get::<_, Option<chrono::NaiveDateTime>>(column_index)
                    .map_err(|e| e.to_string())?
                    .map(|v| json!(v.to_string()))
            }
            &Type::TIMESTAMPTZ => {
                row.try_get::<_, Option<chrono::DateTime<chrono::Utc>>>(column_index)
                    .map_err(|e| e.to_string())?
                    .map(|v| json!(v.to_rfc3339()))
            }
            &Type::TIMETZ => {
                row.try_get::<_, Option<String>>(column_index)
                    .map_err(|e| e.to_string())?
                    .map(|v| json!(v))
            }

            // Network types
            &Type::INET => {
                row.try_get::<_, Option<std::net::IpAddr>>(column_index)
                    .map_err(|e| e.to_string())?
                    .map(|v| json!(v.to_string()))
            }
            &Type::CIDR => {
                row.try_get::<_, Option<String>>(column_index)
                    .map_err(|e| e.to_string())?
                    .map(|v| json!(v))
            }
            &Type::MACADDR | &Type::MACADDR8 => {
                row.try_get::<_, Option<String>>(column_index)
                    .map_err(|e| e.to_string())?
                    .map(|v| json!(v))
            }

            // JSON types
            &Type::JSON | &Type::JSONB => {
                if let Ok(Some(json_str)) = row.try_get::<_, Option<String>>(column_index) {
                    // Try to parse as JSON, fallback to string if parsing fails
                    match serde_json::from_str::<Value>(&json_str) {
                        Ok(parsed) => Some(parsed),
                        Err(_) => Some(json!(json_str)),
                    }
                } else {
                    None
                }
            }

            // UUID type
            &Type::UUID => row
                .try_get::<_, Option<String>>(column_index)
                .map_err(|e| e.to_string())?
                .map(|v| json!(v)),

            // Binary data
            &Type::BYTEA => {
                if let Ok(Some(bytes)) = row.try_get::<_, Option<Vec<u8>>>(column_index) {
                    Some(json!(BASE64_STANDARD.encode(&bytes)))
                } else {
                    None
                }
            }

            // Array types
            &Type::TEXT_ARRAY => {
                if let Ok(Some(arr)) = row.try_get::<_, Option<Vec<String>>>(column_index) {
                    Some(json!(arr))
                } else {
                    None
                }
            }
            &Type::INT4_ARRAY => {
                if let Ok(Some(arr)) = row.try_get::<_, Option<Vec<i32>>>(column_index) {
                    Some(json!(arr))
                } else {
                    None
                }
            }
            &Type::INT8_ARRAY => {
                if let Ok(Some(arr)) = row.try_get::<_, Option<Vec<i64>>>(column_index) {
                    Some(json!(arr))
                } else {
                    None
                }
            }
            &Type::FLOAT4_ARRAY => {
                if let Ok(Some(arr)) = row.try_get::<_, Option<Vec<f32>>>(column_index) {
                    Some(json!(arr))
                } else {
                    None
                }
            }
            &Type::FLOAT8_ARRAY => {
                if let Ok(Some(arr)) = row.try_get::<_, Option<Vec<f64>>>(column_index) {
                    Some(json!(arr))
                } else {
                    None
                }
            }
            &Type::BOOL_ARRAY => {
                if let Ok(Some(arr)) = row.try_get::<_, Option<Vec<bool>>>(column_index) {
                    Some(json!(arr))
                } else {
                    None
                }
            }

            // Geometric types
            &Type::POINT | &Type::LINE | &Type::LSEG | &Type::BOX | &Type::PATH
            | &Type::POLYGON | &Type::CIRCLE => row
                .try_get::<_, Option<String>>(column_index)
                .map_err(|e| e.to_string())?
                .map(|v| json!(v)),

            // Range types
            &Type::INT4_RANGE | &Type::INT8_RANGE | &Type::NUM_RANGE | &Type::TS_RANGE
            | &Type::TSTZ_RANGE | &Type::DATE_RANGE => row
                .try_get::<_, Option<String>>(column_index)
                .map_err(|e| e.to_string())?
                .map(|v| json!(v)),

            // Bit string types
            &Type::BIT | &Type::VARBIT => row
                .try_get::<_, Option<String>>(column_index)
                .map_err(|e| e.to_string())?
                .map(|v| json!(v)),

            // Money type
            &Type::MONEY => row
                .try_get::<_, Option<String>>(column_index)
                .map_err(|e| e.to_string())?
                .map(|v| json!(v)),

            // OID types
            &Type::OID | &Type::REGPROC | &Type::REGPROCEDURE | &Type::REGOPER
            | &Type::REGOPERATOR | &Type::REGCLASS | &Type::REGTYPE | &Type::REGCONFIG
            | &Type::REGDICTIONARY => row
                .try_get::<_, Option<String>>(column_index)
                .map_err(|e| e.to_string())?
                .map(|v| json!(v)),

            // Text search types
            &Type::TS_VECTOR | &Type::TSQUERY => row
                .try_get::<_, Option<String>>(column_index)
                .map_err(|e| e.to_string())?
                .map(|v| json!(v)),

            // XML type
            &Type::XML => row
                .try_get::<_, Option<String>>(column_index)
                .map_err(|e| e.to_string())?
                .map(|v| json!(v)),

            // HSTORE type
            _ if column_type.name() == "hstore" => {
                if let Ok(Some(hstore)) = row.try_get::<_, Option<HashMap<String, Option<String>>>>(column_index) {
                    let mut obj = serde_json::Map::new();
                    for (key, value) in hstore {
                        if let Some(v) = value {
                            obj.insert(key, json!(v));
                        }
                    }
                    Some(Value::Object(obj))
                } else {
                    None
                }
            }

            // Custom types (PostGIS, LTREE, etc.)
            _ => {
                // For unknown/custom types, try to get as string
                row.try_get::<_, Option<String>>(column_index)
                    .map_err(|e| e.to_string())?
                    .map(|v| json!(v))
            }
        };

        Ok(value)
    }


}
