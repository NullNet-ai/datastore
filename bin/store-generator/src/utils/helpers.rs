//! Minimal helpers for the generator - to_singular and parse_tables.
//! Copied from store, without store-specific dependencies.

use crate::database::schema::system_tables::is_system_table;
use singularize::singularize;

fn diesel_type_to_proto(diesel_type: &str) -> &'static str {
    if diesel_type.contains("Int4") || diesel_type.contains("Integer") {
        "int32"
    } else if diesel_type.contains("BigInt") {
        "int64"
    } else if diesel_type.contains("Float4") {
        "float"
    } else if diesel_type.contains("Float8") {
        "double"
    } else if diesel_type.contains("Bool") {
        "bool"
    } else if diesel_type.contains("Uuid")
        || diesel_type.contains("Text")
        || diesel_type.contains("Varchar")
        || diesel_type.contains("Timestamp")
        || diesel_type.contains("Array")
        || diesel_type.contains("Inet")
    {
        "string"
    } else {
        "string"
    }
}

pub fn to_singular(table_name: &str) -> String {
    singularize(table_name)
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

        if line.is_empty() || line.starts_with("//") {
            continue;
        }

        if line.starts_with("table!") {
            in_table_def = true;
            bracket_depth = 0;
        }

        if in_table_def {
            bracket_depth += line.chars().filter(|&c| c == '{').count();
            bracket_depth -= line.chars().filter(|&c| c == '}').count();

            if table_name.is_empty() && line.contains('(') && !line.starts_with("table!") {
                let name_part = line.split('(').next().unwrap_or("").trim();
                if !name_part.is_empty() {
                    table_name = name_part.to_string();

                    if is_system_table(&table_name) {
                        in_table_def = false;
                        table_name = String::new();
                        continue;
                    }

                    current_table = Some(Table {
                        name: table_name.clone(),
                        fields: Vec::new(),
                    });
                }
            }

            if bracket_depth > 0 && line.contains("->") {
                if let Some(table) = &mut current_table {
                    let parts: Vec<&str> = line.split("->").collect();
                    if parts.len() == 2 {
                        let field_name = parts[0].trim().trim_end_matches(',');
                        let field_type = parts[1].trim().trim_end_matches(',');

                        table.fields.push(Field {
                            name: field_name.to_string(),
                            proto_type: diesel_type_to_proto(field_type),
                            is_optional: field_type.contains("Nullable"),
                            is_array: field_type.contains("Array"),
                        });
                    }
                }
            }

            if bracket_depth == 0 && !table_name.is_empty() {
                if let Some(table) = current_table.take() {
                    if !table.fields.is_empty() {
                        tables.push(table);
                    }
                }
                table_name = String::new();
                in_table_def = false;
            }
        }
    }

    tables
}
