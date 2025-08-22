use crate::constants::paths;
use crate::database::schema::verify::field_exists_in_table;
use crate::proto_generator::{Case, CaseConvert};
use crate::utils::utils::{parse_tables, to_singular};
use log::{error, info, warn};
use std::fs::{self, File};
use std::io::{self, Write};
use std::path::Path;
use std::process::Command;

pub fn generate_table_enum(schema_path: &str, output_path: &str) -> io::Result<()> {
    info!("Generating Table enum from schema file: {}", schema_path);

    // Read the schema file
    let schema = fs::read_to_string(schema_path)?;

    // Parse tables from schema
    let tables = parse_tables(&schema);

    // Create output directory if it doesn't exist
    if let Some(parent) = Path::new(output_path).parent() {
        fs::create_dir_all(parent)?;
    }

    // Generate the table enum file
    let mut file = File::create(output_path)?;

    // Write imports
    writeln!(file, "use crate::{{generate_get_by_id_match, generate_hypertable_timestamp_match, generate_insert_record_match, generate_upsert_record_match, generate_upsert_record_with_timestamp_match}};")?;
    for table in &tables {
        let singular_name = to_singular(&table.name);
        let model_name = format!("{}_model", singular_name.to_case(Case::Snake));
        writeln!(
            file,
            "use crate::database::models::{}::{}Model;",
            model_name,
            singular_name.to_case(Case::Pascal)
        )?;
    }
    writeln!(file, "use crate::database::schema;")?;
    writeln!(file, "use crate::structs::structs::{{Auth, RequestBody}};")?;
    writeln!(file, "use actix_web::web;")?;
    writeln!(file, "use diesel::associations::HasTable;")?;
    writeln!(file, "use diesel::prelude::*;")?;
    writeln!(file, "use diesel::result::Error as DieselError;")?;
    writeln!(file, "use diesel_async::AsyncPgConnection;")?;
    writeln!(file, "use diesel_async::RunQueryDsl;")?;
    writeln!(file, "use serde_json::{{Map, Value}};")?;
    writeln!(file, "use crate::database::db;")?;
    writeln!(
        file,
        "use crate::database::models::counter_model::CounterModel;"
    )?;

    writeln!(file, "")?;

    // Generate Table enum
    writeln!(file, "#[derive(Debug)]")?;
    writeln!(file, "pub enum Table {{")?;

    // Add table variants
    for table in &tables {
        writeln!(file, "    {},", table.name.to_case(Case::Pascal))?;
    }
    writeln!(file, "    // Add other tables here")?;
    writeln!(file, "}}")?;
    writeln!(file, "")?;

    // Implement Table methods
    writeln!(file, "impl Table {{")?;

    // from_str method
    writeln!(file, "    pub fn from_str(name: &str) -> Option<Self> {{")?;
    writeln!(file, "        match name {{")?;
    for table in &tables {
        let snake_name = table.name.to_case(Case::Snake);
        writeln!(
            file,
            "            \"{}\" => Some(Table::{}),",
            snake_name,
            table.name.to_case(Case::Pascal)
        )?;
    }
    writeln!(file, "            // Add other tables here")?;
    writeln!(file, "            _ => None,")?;
    writeln!(file, "        }}")?;
    writeln!(file, "    }}")?;
    writeln!(file, "")?;

    // pluck_fields method
    writeln!(file, "    pub fn pluck_fields(&self, record_value: &Value, pluck_fields: Vec<String>) -> Value {{")?;
    writeln!(
        file,
        "        if !pluck_fields.is_empty() && record_value.is_object() {{"
    )?;
    writeln!(
        file,
        "            if let Some(obj) = record_value.as_object() {{"
    )?;
    writeln!(file, "                let mut filtered = Map::new();")?;
    writeln!(file, "")?;
    writeln!(file, "                for field in pluck_fields {{")?;
    writeln!(
        file,
        "                    if let Some(val) = obj.get(&field) {{"
    )?;
    writeln!(
        file,
        "                        filtered.insert(field, val.clone());"
    )?;
    writeln!(file, "                    }}")?;
    writeln!(file, "                }}")?;
    writeln!(file, "")?;
    writeln!(file, "                Value::Object(filtered)")?;
    writeln!(file, "            }} else {{")?;
    writeln!(
        file,
        "                record_value.clone() // fallback: return original value"
    )?;
    writeln!(file, "            }}")?;
    writeln!(file, "        }} else {{")?;
    writeln!(file, "            record_value.clone()")?;
    writeln!(file, "        }}")?;
    writeln!(file, "    }}")?;
    writeln!(file, "")?;

    // get_hypertable_timestamp method
    writeln!(file, "    pub async fn get_hypertable_timestamp(")?;
    writeln!(file, "        &self,")?;
    writeln!(file, "        conn: &mut AsyncPgConnection,")?;
    writeln!(file, "        id: &str,")?;
    writeln!(file, "    ) -> Result<Option<String>, DieselError> {{")?;

    // Create a comma-separated list of table names for the macro
    let tables_with_timestamp = tables
        .iter()
        .filter(|t| field_exists_in_table(&t.name.to_lowercase(), "hypertable_timestamp"))
        .map(|t| t.name.to_case(Case::Pascal))
        .collect::<Vec<_>>()
        .join(", ");

    writeln!(
        file,
        "        generate_hypertable_timestamp_match!(self, conn, id, {})",
        tables_with_timestamp
    )?;
    writeln!(file, "    }}")?;
    writeln!(file, "")?;

    // insert_record method
    writeln!(file, "    #[allow(dead_code)]")?;
    writeln!(file, "    pub async fn insert_record(")?;
    writeln!(file, "        &self,")?;
    writeln!(file, "        conn: &mut AsyncPgConnection,")?;
    writeln!(file, "        record: Value,")?;
    writeln!(file, "        request: web::Json<RequestBody>,")?;
    writeln!(file, "        auth: &Auth,")?;

    writeln!(file, "    ) -> Result<String, DieselError> {{")?;

    // Create a comma-separated list of table names and models for the macro
    let table_model_list = tables
        .iter()
        .map(|t| {
            let pascal_name = t.name.to_case(Case::Pascal);
            let singular = to_singular(t.name.as_str());
            let model_name = format!("{}Model", singular.to_case(Case::Pascal));
            format!("{}, {}", pascal_name, model_name)
        })
        .collect::<Vec<_>>()
        .join(", ");

    writeln!(file, "        generate_insert_record_match!(")?;
    writeln!(file, "            self,")?;
    writeln!(file, "            auth,")?;
    writeln!(file, "            conn,")?;
    writeln!(file, "            record,")?;
    writeln!(file, "            request,")?;
    writeln!(
        file,
        "            {} // Add other tables and their models here as needed",
        table_model_list
    )?;
    writeln!(file, "        )")?;
    writeln!(file, "    }}")?;
    writeln!(file, "")?;

    // get_by_id method
    writeln!(file, "    pub async fn get_by_id(")?;
    writeln!(file, "        &self,")?;
    writeln!(file, "        conn: &mut AsyncPgConnection,")?;
    writeln!(file, "        id: &str,")?;
    writeln!(file, "        is_root_account: bool,")?;
    writeln!(file, "        organization_id: Option<String>,")?;
    writeln!(file, "    ) -> Result<Option<Value>, DieselError> {{")?;
    writeln!(file, "        generate_get_by_id_match!(")?;
    writeln!(file, "            self,")?;
    writeln!(file, "            conn,")?;
    writeln!(file, "            id,")?;
    writeln!(file, "            is_root_account,")?;
    writeln!(file, "            organization_id,")?;
    writeln!(
        file,
        "            {} // Add other tables and their models here as needed",
        table_model_list
    )?;
    writeln!(file, "        )")?;
    writeln!(file, "    }}")?;
    writeln!(file, "")?;

    // upsert_record method
    writeln!(file, "    pub async fn upsert_record_with_id(")?;
    writeln!(file, "        &self,")?;
    writeln!(file, "        conn: &mut AsyncPgConnection,")?;
    writeln!(file, "        record: Value,")?;
    writeln!(file, "    ) -> Result<(), DieselError> {{")?;
    writeln!(file, "        generate_upsert_record_match!(")?;
    writeln!(file, "            self,")?;
    writeln!(file, "            conn,")?;
    writeln!(file, "            record,")?;
    writeln!(
        file,
        "            {} // Add other tables and their models here as needed",
        table_model_list
    )?;
    writeln!(file, "        )")?;
    writeln!(file, "    }}")?;
    writeln!(file, "")?;

    // upsert_record_with_id_timestamp method
    writeln!(file, "    pub async fn upsert_record_with_id_timestamp(")?;
    writeln!(file, "        &self,")?;
    writeln!(file, "        conn: &mut AsyncPgConnection,")?;
    writeln!(file, "        record: Value,")?;
    writeln!(file, "    ) -> Result<(), DieselError> {{")?;
    writeln!(
        file,
        "        generate_upsert_record_with_timestamp_match!("
    )?;
    writeln!(file, "            self,")?;
    writeln!(file, "            conn,")?;
    writeln!(file, "            record,")?;
    writeln!(
        file,
        "            {} // Add other tables and their models here as needed",
        table_model_list
    )?;
    writeln!(file, "        )")?;
    writeln!(file, "    }}")?;

    // Close the impl block
    writeln!(file, "}}")?;

    writeln!(file, "pub async fn generate_code(")?;
    writeln!(file, "    table: &str,")?;
    writeln!(file, "    prefix_param: &str,")?;
    writeln!(file, "    default_code_param: i32,")?;
    writeln!(file, ") -> Result<String, DieselError> {{")?;
    writeln!(file, "")?;
    writeln!(file, "    let mut conn = db::get_async_connection().await;")?;
    writeln!(file, "")?;
    writeln!(file, "    let new_counter = CounterModel {{")?;
    writeln!(file, "        entity: table.to_string(),")?;
    writeln!(file, "        counter: 1,")?;
    writeln!(file, "        prefix: prefix_param.to_string(),")?;
    writeln!(file, "        default_code: default_code_param,")?;
    writeln!(file, "        digits_number: 1,")?;
    writeln!(file, "    }};")?;
    writeln!(file, "    ")?;
    writeln!(file, "    // Attempt the insert with conflict handling")?;
    writeln!(
        file,
        "    let result = diesel::insert_into(schema::counters::dsl::counters::table())"
    )?;
    writeln!(file, "    .values(&new_counter)")?;
    writeln!(file, "        .on_conflict(schema::counters::entity)")?;
    writeln!(file, "        .do_update()")?;
    writeln!(
        file,
        "        .set(schema::counters::counter.eq(schema::counters::counter + 1))"
    )?;
    writeln!(file, "        .returning((schema::counters::prefix, schema::counters::default_code, schema::counters::counter))")?;
    writeln!(
        file,
        "        .get_result::<(String, i32, i32)>(&mut conn).await"
    )?;
    writeln!(file, "        .map_err(|e| {{")?;
    writeln!(
        file,
        "            log::error!(\"Error generating code: {{}}\", e);"
    )?;
    writeln!(file, "            e")?;
    writeln!(file, "        }})?;")?;
    writeln!(file, "    ")?;
    writeln!(file, "    // Format the code")?;
    writeln!(
        file,
        "    let (prefix_val, default_code_val, counter_val) = result;"
    )?;
    writeln!(file, "    let code = format!(")?;
    writeln!(file, "        \"{{}}{{}}\",")?;
    writeln!(file, "        prefix_val,")?;
    writeln!(file, "        default_code_val + counter_val")?;
    writeln!(file, "    );")?;
    writeln!(file, "    ")?;
    writeln!(file, "    Ok(code)")?;
    writeln!(file, "}}")?;

    // Format the generated code with rustfmt
    info!("Formatting generated code...");
    match Command::new("rustfmt").arg(output_path).status() {
        Ok(_) => info!("Code formatting completed"),
        Err(e) => warn!("Failed to format code: {}", e),
    }

    info!("Successfully generated Table enum at: {}", output_path);
    Ok(())
}

pub fn run_generator() -> io::Result<()> {
    info!("Starting Table enum generator");

    // Default paths
    let schema_path = paths::database::SCHEMA_FILE;
    let output_path = "src/table_enum.rs";

    // Generate the Table enum
    match generate_table_enum(schema_path, output_path) {
        Ok(_) => {
            info!("Successfully generated Table enum");

            // Format the generated code with rustfmt
            info!("Formatting generated code...");
            match Command::new("rustfmt").arg(output_path).status() {
                Ok(_) => info!("Code formatting completed"),
                Err(e) => warn!("Failed to format code: {}", e),
            }

            Ok(())
        }
        Err(e) => {
            error!("Error generating Table enum: {}", e);
            Err(e)
        }
    }
}
