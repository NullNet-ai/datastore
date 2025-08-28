use std::env;
use crate::builders::templates::proto_generator::diesel_type_to_proto;
use crate::controllers::store_controller::ApiError;
use crate::database::db;
use crate::database::schema::system_tables::is_system_table;
use crate::generated::models::counter_model::CounterModel;
use crate::generated::schema::counters;
use crate::generated::table_enum::Table as TableEnum;
use crate::providers::storage::cache::cache_factory::CacheType;
use crate::structs::structs::{CommandArgs, EnvConfig};
use actix_web::http;
use diesel::prelude::*;
use diesel::result::Error as DieselError;
use diesel_async::RunQueryDsl;
use log::debug;
use singularize::singularize;
use std::time::Duration;

pub fn to_singular(table_name: &str) -> String {
    let singular = singularize(table_name);
    singular
}

#[derive(Clone)]
pub struct Table {
    pub name: String,
    pub fields: Vec<Field>,
}

#[derive(Clone)]
pub struct Field {
    pub name: String,
    pub proto_type: &'static str,
    pub is_optional: bool,
    pub is_array: bool,
}

pub fn parse_tables(schema: &str) -> Vec<Table> {
    let mut tables = Vec::new();
    let mut current_table: Option<Table> = None;
    let mut bracket_depth = 0;
    let mut in_table_def = false;
    let mut table_name = String::new();

    for line in schema.lines() {
        let line = line.trim();

        // Skip empty lines and comments
        if line.is_empty() || line.starts_with("//") {
            continue;
        }

        // Start of table definition
        if line.starts_with("table!") {
            in_table_def = true;
            bracket_depth = 0;
        }

        if in_table_def {
            // Count brackets for nesting level
            bracket_depth += line.chars().filter(|&c| c == '{').count();
            bracket_depth -= line.chars().filter(|&c| c == '}').count();

            // Extract table name from the line after "table!"
            if table_name.is_empty() && line.contains("(") && !line.starts_with("table!") {
                let name_part = line.split('(').next().unwrap_or("").trim();
                if !name_part.is_empty() {
                    table_name = name_part.to_string();

                    if is_system_table(&table_name) {
                        debug!("Skipping system table: {}", table_name);
                        in_table_def = false;
                        table_name = String::new();
                        continue;
                    }

                    debug!("Found table: {}", table_name);
                    current_table = Some(Table {
                        name: table_name.clone(),
                        fields: Vec::new(),
                    });
                }
            }

            // Parse field definitions
            if bracket_depth > 0 && line.contains("->") {
                if let Some(table) = &mut current_table {
                    let parts: Vec<&str> = line.split("->").collect();
                    if parts.len() == 2 {
                        let field_name = parts[0].trim().trim_end_matches(',');
                        let field_type = parts[1].trim().trim_end_matches(',');

                        debug!("Found field: {} -> {}", field_name, field_type);

                        table.fields.push(Field {
                            name: field_name.to_string(),
                            proto_type: diesel_type_to_proto(field_type),
                            is_optional: field_type.contains("Nullable"),
                            is_array: field_type.contains("Array"),
                        });
                    }
                }
            }

            // End of table definition
            if bracket_depth == 0 && !table_name.is_empty() {
                if let Some(table) = current_table.take() {
                    if !table.fields.is_empty() {
                        tables.push(table.clone());
                        debug!(
                            "Added table: {} with {} fields",
                            table_name,
                            table.fields.len()
                        );
                    }
                }
                table_name = String::new();
                in_table_def = false;
            }
        }
    }

    // Add any remaining table
    if let Some(table) = current_table {
        if !table.fields.is_empty() {
            tables.push(table);
        }
    }

    tables
}

pub fn _token_data_extractor(_token: &str) -> String {
    todo!()
}

pub fn table_exists(table_name: &str) -> Result<TableEnum, ApiError> {
    TableEnum::from_str(table_name).ok_or_else(|| {
        ApiError::new(
            http::StatusCode::BAD_REQUEST,
            format!("Unknown table: {}", table_name),
        )
    })
}

pub async fn generate_code(entity: &str) -> Result<Option<String>, DieselError> {
    let mut conn = db::get_async_connection().await;

    // Create a new counter with default values
    let new_counter = CounterModel {
        entity: entity.to_string(),
        counter: 1,
        prefix: "".to_string(),
        default_code: 0,
        digits_number: 0,
    };

    // Insert or update the counter
    let result = diesel::insert_into(counters::table)
        .values(&new_counter)
        .on_conflict(counters::entity)
        .do_update()
        .set(counters::counter.eq(counters::counter + 1))
        .returning((
            counters::prefix,
            counters::default_code,
            counters::counter,
            counters::digits_number,
        ))
        .get_result::<(String, i32, i32, i32)>(&mut conn)
        .await
        .map_err(|e| {
            log::error!("Error generating code: {}", e);
            e
        })?;

    // Format the code based on the returned values
    let (prefix, default_code, counter, digits_number) = result;

    // Format the code according to the digits_number
    let code = if digits_number > 0 {
        // Calculate how many digits the counter has
        let counter_digits = counter.to_string().len() as i32;

        // Calculate how many leading zeros to add
        let zero_digits = digits_number - counter_digits;

        if zero_digits > 0 {
            // Add leading zeros
            let zeros = "0".repeat(zero_digits as usize);
            format!("{}{}{}", prefix, zeros, counter)
        } else {
            // No leading zeros needed
            format!("{}{}", prefix, counter)
        }
    } else {
        // Use default_code + counter
        format!("{}{}", prefix, default_code + counter)
    };

    Ok(Some(code))
}

pub fn time_string_to_ms(time_str: &str) -> Result<u64, Box<dyn std::error::Error>> {
    // Format: 1d 2h 30m 45s
    if let Some(captures) =
        regex::Regex::new(r"^((?:\d+)d\s*)?((?:\d+)h\s*)?((?:\d+)m\s*)?((?:\d+)s\s*)?$")
            .unwrap()
            .captures(time_str)
    {
        let days = captures.get(1).map_or(0, |m| {
            m.as_str()
                .trim_end_matches('d')
                .trim()
                .parse::<u64>()
                .unwrap_or(0)
        });
        let hours = captures.get(2).map_or(0, |m| {
            m.as_str()
                .trim_end_matches('h')
                .trim()
                .parse::<u64>()
                .unwrap_or(0)
        });
        let minutes = captures.get(3).map_or(0, |m| {
            m.as_str()
                .trim_end_matches('m')
                .trim()
                .parse::<u64>()
                .unwrap_or(0)
        });
        let seconds = captures.get(4).map_or(0, |m| {
            m.as_str()
                .trim_end_matches('s')
                .trim()
                .parse::<u64>()
                .unwrap_or(0)
        });

        let total_ms = days * 24 * 60 * 60 * 1000
            + hours * 60 * 60 * 1000
            + minutes * 60 * 1000
            + seconds * 1000;
        return Ok(total_ms);
    }

    // Format: HH:mm:ss
    if let Some(captures) = regex::Regex::new(r"^(\d{1,2}):(\d{2}):(\d{2})$")
        .unwrap()
        .captures(time_str)
    {
        let hours = captures
            .get(1)
            .map_or(0, |m| m.as_str().parse::<u64>().unwrap_or(0));
        let minutes = captures
            .get(2)
            .map_or(0, |m| m.as_str().parse::<u64>().unwrap_or(0));
        let seconds = captures
            .get(3)
            .map_or(0, |m| m.as_str().parse::<u64>().unwrap_or(0));

        let total_ms = hours * 60 * 60 * 1000 + minutes * 60 * 1000 + seconds * 1000;
        return Ok(total_ms);
    }

    // Format: mm:ss
    if let Some(captures) = regex::Regex::new(r"^(\d{1,2}):(\d{2})$")
        .unwrap()
        .captures(time_str)
    {
        let minutes = captures
            .get(1)
            .map_or(0, |m| m.as_str().parse::<u64>().unwrap_or(0));
        let seconds = captures
            .get(2)
            .map_or(0, |m| m.as_str().parse::<u64>().unwrap_or(0));

        let total_ms = minutes * 60 * 1000 + seconds * 1000;
        return Ok(total_ms);
    }

    // If none of the formats match
    Err(Box::new(std::io::Error::new(
        std::io::ErrorKind::InvalidInput,
        "Invalid time format",
    )))
}

/// Parse command-line arguments
pub fn parse_command_args() -> CommandArgs {
    let args: Vec<String> = env::args().collect();

    CommandArgs {
        cleanup: args.contains(&"--cleanup".to_string()),
        init_db: args.contains(&"--init-db".to_string()),
        generate_proto: env::var("GENERATE_PROTO").unwrap_or_else(|_| "false".to_string())
            == "true",
        generate_grpc: env::var("GENERATE_GRPC").unwrap_or_else(|_| "false".to_string()) == "true",
        generate_table_enum: env::var("GENERATE_TABLE_ENUM")
            .unwrap_or_else(|_| "false".to_string())
            == "true",
        create_schema: env::var("CREATE_SCHEMA").unwrap_or_else(|_| "false".to_string()) == "true",
    }
}

/// Parse environment configuration
pub fn parse_env_config() -> EnvConfig {
    let cache_type_str = env::var("CACHE_TYPE").unwrap_or_else(|_| "inmemory".to_string());
    let cache_type = CacheType::from_str(&cache_type_str).unwrap_or(CacheType::InMemory);
    let redis_connection = env::var("REDIS_CONNECTION").ok();
    let ttl = env::var("CACHE_TTL")
        .ok()
        .and_then(|ttl_str| ttl_str.parse::<u64>().ok())
        .map(Duration::from_secs);

    EnvConfig {
        host: env::var("HOST").unwrap_or_else(|_| "0.0.0.0".to_string()),
        port: env::var("PORT").unwrap_or_else(|_| "5000".to_string()),
        grpc_port: env::var("GRPC_PORT").unwrap_or_else(|_| "6000".to_string()),
        grpc_url: env::var("GRPC_URL").unwrap_or_else(|_| "127.0.0.1".to_string()),
        socket_host: env::var("SOCKET_HOST").unwrap_or_else(|_| "0.0.0.0".to_string()),
        socket_port: env::var("SOCKET_PORT").unwrap_or_else(|_| "3001".to_string()),
        cache_type,
        redis_connection,
        ttl,
    }
}