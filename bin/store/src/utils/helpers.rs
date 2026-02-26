use crate::config::core::EnvConfig;
use crate::controllers::store_controller::ApiError;
use crate::database::schema::verify::field_type_in_table;
use crate::generated::table_enum::Table as TableEnum;
use crate::structs::core::CommandArgs;
use actix_web::http;
use diesel::result::Error as DieselError;
use std::env;

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

/// Generate next unique code for the entity via counter-service.
pub async fn generate_code(entity: &str) -> Result<Option<String>, DieselError> {
    crate::utils::code_generator::generate_code_optional(entity).await
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

/// Normalizes date format for PostgreSQL compatibility (e.g. yyyy -> YYYY).
/// Validation accepts formats case-insensitively; this ensures correct casing for TO_CHAR.
pub fn normalize_date_format(format: &str) -> String {
    format.replace("yyyy", "YYYY").replace("yy", "YY")
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
