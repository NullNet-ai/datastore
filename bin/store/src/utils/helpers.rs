use crate::builders::templates::proto_generator::diesel_type_to_proto;
use crate::config::core::EnvConfig;
use crate::controllers::store_controller::ApiError;
use crate::database::db;
use crate::database::schema::system_tables::is_system_table;
use crate::database::schema::verify::field_type_in_table;
use crate::generated::models::counter_model::CounterModel;
use crate::generated::schema::counters;
use crate::generated::table_enum::Table as TableEnum;
use crate::structs::core::CommandArgs;
use actix_web::http;
use diesel::prelude::*;
use diesel::result::Error as DieselError;
use diesel_async::RunQueryDsl;
use log::debug;
use singularize::singularize;
use std::env;
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
    let config = EnvConfig::default();

    CommandArgs {
        cleanup: args.contains(&"--cleanup".to_string()),
        init_db: args.contains(&"--init-db".to_string()),
        generate_proto: config.generate_proto,
        generate_grpc: config.generate_grpc,
        generate_table_enum: config.generate_table_enum,
        create_schema: config.create_schema,
    }
}

/// Parse environment configuration
pub fn parse_env_config() -> EnvConfig {
    EnvConfig::default()
}

pub fn date_format_wrapper(
    table: &str,
    field: &str,
    format_str: Option<&str>,
    timezone: Option<&str>,
    with_alias: bool,
) -> String {
    let format = format_str.unwrap_or("mm/dd/YYYY");

    if !field.ends_with("_date") {
        let alias = if with_alias {
            format!(" AS \"{}\"", field)
        } else {
            "".to_string()
        };
        log::warn!("Parsing field {} as date", field);
        return format!(
            "COALESCE(TO_CHAR(\"{}\".\"{}\"::DATE, '{}'), ''){}",
            table, field, format, alias
        );
    }
    let field_prefix = field.strip_suffix("_date").unwrap_or(field);
    let time_field = format!("{}_time", field_prefix);

    let field_type_exists = field_type_in_table(table, field);

    let timestamp_cast = if let Some(field_type_info) = field_type_exists {
        let field_type_str = &field_type_info.field_type;
        if field_type_str.to_lowercase().contains("timestamp") {
            ""
        } else {
            "::TIMESTAMP"
        }
    } else {
        "::TIMESTAMP"
    };

    let server_timezone = std::env::var("TZ").unwrap_or_else(|_| "UTC".to_string());
    let timezone_query = if let Some(target_timezone) = timezone {
        format!(
            "AT TIME ZONE '{}' AT TIME ZONE '{}'",
            server_timezone, target_timezone
        )
    } else {
        "".to_string()
    };

    // Format the field with conditional timestamp cast
    let formatted_field = if timezone_query.is_empty() {
        format!("\"{}\".\"{}\"{}", table, field, timestamp_cast)
    } else {
        format!(
            "\"{}\".\"{}\"{} + \"{}\".\"{}\"::INTERVAL",
            table, field, timestamp_cast, table, time_field
        )
    };

    let field_with_timezone = if timezone_query.is_empty() {
        format!("({})", formatted_field)
    } else {
        format!("(({}) {})", formatted_field, timezone_query)
    };
    let alias = if with_alias {
        format!(" AS \"{}\"", field)
    } else {
        "".to_string()
    };

    format!(
        "COALESCE(TO_CHAR({}::DATE, '{}'), ''){}",
        field_with_timezone, format, alias
    )
}

pub fn time_format_wrapper(
    table: &str,
    field: &str,
    timezone: Option<&str>,
    main_table: &str,
    with_alias: bool,
    time_format: &str,
) -> String {
    let field_parts: Vec<&str> = field.split('.').collect();
    let (table_name, partial_field_name, field_with_table) = if field_parts.len() > 1 {
        (
            field_parts[0].replace("\"", ""),
            field_parts[1].replace("\"", ""),
            field.to_string(),
        )
    } else {
        (
            table.to_string(),
            field_parts[0].replace("\"", ""),
            format!("\"{}\".\"{}\"", table, field),
        )
    };
    let cloned_partial_field_name = partial_field_name.clone();
    let field_name = if field_parts.len() == 2 {
        if table_name != main_table {
            format!("{}_{}", table_name, partial_field_name)
        } else {
            partial_field_name
        }
    } else {
        field.to_string()
    };
    if !cloned_partial_field_name.ends_with("_time") {
        let alias = if with_alias {
            format!(" AS {}", field_name)
        } else {
            "".to_string()
        };
        log::warn!("Parsing field {} as time", field);
        return format!(
            "TO_CHAR(({})::time, '{}')::text{}",
            field, time_format, alias
        );
    }
    let field_prefix = cloned_partial_field_name
        .strip_suffix("_time")
        .unwrap_or(field);

    let date_field = format!("\"{}\".\"{}_date\"", table_name, field_prefix);
    let field_type_exists = field_type_in_table(&table_name, field);

    let timestamp_cast = if let Some(field_type_info) = field_type_exists {
        let field_type_str = &field_type_info.field_type;
        if field_type_str.to_lowercase().contains("timestamp") {
            ""
        } else {
            "::TIMESTAMP"
        }
    } else {
        "::TIMESTAMP"
    };

    let server_timezone = std::env::var("TZ").unwrap_or_else(|_| "UTC".to_string());
    let timezone_query = if let Some(target_timezone) = timezone {
        format!(
            "AT TIME ZONE '{}' AT TIME ZONE '{}'",
            server_timezone, target_timezone
        )
    } else {
        "".to_string()
    };
    let formatted_field = if timezone_query.is_empty() {
        format!("{}::INTERVAL", field_with_table)
    } else {
        format!(
            "{}{} + {}::INTERVAL",
            date_field, timestamp_cast, field_with_table
        )
    };

    let field_with_timezone = if timezone_query.is_empty() {
        format!("({})", formatted_field)
    } else {
        format!("(({}) {})", formatted_field, timezone_query)
    };
    let alias = if with_alias {
        format!(" AS {}", field_name)
    } else {
        "".to_string()
    };
    format!(
        "TO_CHAR(({})::time, '{}')::text{}",
        field_with_timezone, time_format, alias
    )
}

/// Formats a timestamp/timestamptz field with optional timezone conversion.
/// Uses ISO 8601 format (YYYY-MM-DD HH24:MI:SS.US) to stay consistent with the original stored value.
/// Used for the `timestamp` column (e.g. hypertable timestamp) in find, count, and search suggestions.
pub fn timestamp_format_wrapper(
    table: &str,
    field: &str,
    _date_format: &str,
    _time_format: &str,
    timezone: Option<&str>,
    with_alias: bool,
) -> String {
    // ISO format: YYYY-MM-DD HH24:MI:SS.US (matches e.g. 2025-08-21 01:18:18.604966+00)
    let iso_format = "YYYY-MM-DD HH24:MI:SS.US";
    let field_expr = format!("\"{}\".\"{}\"", table, field);
    let timezone_expr = if let Some(tz) = timezone {
        format!("({} AT TIME ZONE '{}')", field_expr, tz)
    } else {
        format!("({})", field_expr)
    };
    let alias = if with_alias {
        format!(" AS \"{}\"", field)
    } else {
        String::new()
    };
    format!(
        "COALESCE(TO_CHAR({}::TIMESTAMP, '{}'), ''){}",
        timezone_expr, iso_format, alias
    )
}
