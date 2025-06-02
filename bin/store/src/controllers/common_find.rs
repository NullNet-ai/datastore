use serde_json::Value;
use tokio_postgres::types::Field;
use std::collections::HashMap;

use crate::{
    schema::verify::field_exists_in_table, structs::structs::{FieldRelation, GroupAdvanceFilter, Join, ParsedConcatenatedFields, FilterCriteria}
};

pub fn validate_pluck_object(
    table: &str,
    pluck: &[String],
) -> Option<HashMap<String, serde_json::Value>> {
    let table_schema = field_exists_in_table;

    let plucked_fields = pluck.iter().fold(HashMap::new(), |mut acc, field| {
        if table_schema(table, field) {
            acc.insert(field.clone(), serde_json::Value::String(field.clone()));
        }
        acc
    });

    if plucked_fields.is_empty() {
        None
    } else {
        Some(plucked_fields)
    }
}

pub fn format_if_date(field: &str, date_format: &str, entity: &str) -> String {
    if field.to_lowercase().ends_with("date") {
        format!(
            "'{}', to_char(\"{}\".\"{}\"::date, '{}')",
            field, entity, field, date_format
        )
    } else {
        format!("'{}', \"{}\".\"{}\")", field, entity, field)
    }
}

pub fn create_selections(
    table: String,
    pluck_object: HashMap<String, Vec<String>>,
    joins: &[Join],
    date_format: String,
    parsed_concatenated_fields: &ParsedConcatenatedFields,
) -> HashMap<String, String> {
    let mut selections = HashMap::new();

    // Get pluck_object keys directly from the HashMap
    let pluck_object_keys: Vec<String> = pluck_object.keys().cloned().collect();

    let concatenated_fields = &parsed_concatenated_fields.fields;
    let expressions = &parsed_concatenated_fields.expressions;

    // Process main table selections
    if let Some(fields) = pluck_object.get(&table) {
        for field in fields {
            // Use format_if_date for main table fields
            let selection = if field.to_lowercase().ends_with("date") {
                format!(
                    "to_char(\"{}\".\"{}\"::timestamp, '{}') AS \"{}\"",
                    table, field, &date_format, field
                )
            } else {
                format!("\"{}\".\"{}\" AS \"{}\"", table, field, field)
            };
            selections.insert(field.to_string(), selection);
        }
    }

    // Process main table concatenated fields
    if let Some(main_concatenate_selections) = expressions.get(&table) {
        for selection in main_concatenate_selections {
            if let Some(pos) = selection.find(" AS ") {
                let field_name = &selection[pos + 4..].replace("\"", "").replace("/", "");
                selections.insert(field_name.to_string(), selection.clone());
            }
        }
    };

    // Process join selections
    if !joins.is_empty() {
        for join in joins {
            let to_entity = &join.field_relation.to.entity;
            let to_alias = join.field_relation.to.alias.as_ref().unwrap_or(to_entity);

            // Only process if the entity has pluck_object fields
            if pluck_object_keys.contains(to_alias) {
                let mut entity_fields = Vec::new();

                // Get fields from pluck_object
                if let Some(fields) = pluck_object.get(to_alias) {
                    // Add all fields directly from the Vec<String>
                    entity_fields.extend(fields.clone());
                }

                // Add concatenated fields
                if let Some(concat_fields) = concatenated_fields.get(to_alias) {
                    for field in concat_fields {
                        if !entity_fields.contains(field) {
                            entity_fields.push(field.clone());
                        }
                    }
                }

                if !entity_fields.is_empty() {
                    // Create JSON_AGG with JSON_BUILD_OBJECT using format_if_date
                    let json_agg_fields: Vec<String> = entity_fields
                        .iter()
                        .map(|field| format_if_date(field, &date_format, to_alias))
                        .collect();

                    let json_agg_selection = format!(
                            "COALESCE(JSONB_AGG(DISTINCT JSONB_BUILD_OBJECT({})) FILTER (WHERE \"{}\".\"id\" IS NOT NULL), '[]') AS \"{}\"",
                            json_agg_fields.join(", "),
                            to_alias,
                            to_alias
                        );
                    selections.insert(to_alias.to_string(), json_agg_selection);
                }
            }
        }
    }

    selections
}

pub fn get_sort_field(
    by_field: String,
    aliased_entities: HashMap<String, String>,
    concatenations: ParsedConcatenatedFields,
    by_direction: String,
    is_case_sensitive_sorting: Option<String>,
    main_table: String,
) -> Result<String, String> {
    let case_sensitive = is_case_sensitive_sorting.unwrap_or_else(|| String::from("false"));

    // Split the order_by string by '.'
    let by_entity_field: Vec<&str> = by_field.split('.').collect();

    // Check if the split resulted in at least 2 parts (entity and field)
    if by_entity_field.len() < 2 {
        return Err(format!(
            "Invalid by_field format: '{}'. It should be separated by dot like 'table.field'",
            by_field.clone()
        ));
    }

    let sort_entity = by_entity_field[0];
    let sort_field = by_entity_field[1];
    let field_exists_in_schema = field_exists_in_table(sort_entity, sort_field);
    let field_exists_in_concatenations = concatenations.fields.contains_key(sort_entity)
        && concatenations
            .fields
            .get(sort_entity)
            .unwrap()
            .contains(&sort_field.to_string());

    let field_is_aliased = aliased_entities.contains_key(sort_field);
    let entity_is_main_table = sort_entity == main_table;

    if !field_exists_in_schema && !field_exists_in_concatenations && !field_is_aliased {
        return Err(format!(
            "Field '{}' does not exist in the schema, aliases or concatenations",
            by_field
        ));
    }
    let is_case_sensitive = case_sensitive == "true";

    let mut sort_query: String = String::new();

    if !field_exists_in_schema && field_exists_in_concatenations {
        // Get the index of the field in the concatenated fields
        if let Some(fields) = concatenations.fields.get(sort_entity) {
            if let Some(index) = fields.iter().position(|f| f == sort_field) {
                // Get the corresponding expression
                if let Some(expressions) = concatenations.expressions.get(sort_entity) {
                    if index < expressions.len() {
                        let expression = &expressions[index];

                        // Simply split by " AS " and extract the field name
                        let parts: Vec<&str> = expression.split(" AS ").collect();
                        if parts.len() >= 2 {
                            // Extract the field name and remove quotes
                            sort_query = parts[0].trim().trim_matches('"').to_string();
                        } else {
                            return Err(format!(
                                "Error while sorting by concatenated field '{}', invalid expression format, missing AS keyword",
                                by_field
                            ));
                        }
                    } else {
                        return Err(format!(
                            "Error while sorting by concatenated field '{}', expressions and fields length mismatch",
                            by_field
                        ));
                    }
                }
            }
        }
    } else {
        // If the field is aliased, use the alias
        sort_query = format!("\"{}\".\"{}\"", sort_entity, sort_field)
    }

    if (is_case_sensitive) {
        sort_query = format!("lower({})", sort_query);
    }

    if (!entity_is_main_table) {
        if by_direction.to_lowercase() == "asc" {
            sort_query = format!("MIN({})", sort_query);
        } else {
            sort_query = format!("MAX({})", sort_query);
        }
    }
    //remove joined_ keyword from sort_query
    sort_query = sort_query.replace("joined_", "");
    let direction =
        if by_direction.to_lowercase() == "asc" || by_direction.to_lowercase() == "ascending" {
            "ASC"
        } else {
            "DESC"
        };
    sort_query = format!("{} {}", sort_query, direction);
    // Add the direction to the sort_query
    // Return the sort_query valu
    Ok(sort_query)
}


pub fn filter_analyzer(
    table_name: String,
    advance_filters: Vec<FilterCriteria>,
    pluck_object: HashMap<String, Vec<String>>,
    organization_id: String,
    joins: &[Join],
    concatenations: ParsedConcatenatedFields,
    group_advance_filters : Vec<GroupAdvanceFilter>,
    aliased_entites: HashMap<String, String>,
    selections: HashMap<String, String>,
)->Result<String, String>{
    let fields= concatenations.fields;
    let expressions = concatenations.expressions;
    let mut result_query = String::new();

    if !joins.is_empty() {
        // Use a for loop instead of for_each
        for join in joins.iter() {
            let FieldRelation{from, to} = &join.field_relation;

            let join_type=join.r#type.to_uppercase();
            let to_entity = to.entity.clone();
            let to_alias = to.alias.clone().unwrap_or(to_entity.clone());
            let concatenate_query = expressions.get(&to_alias).cloned().unwrap_or_default();

            match join_type.as_str() {
                "LEFT" => {
                    let fields=pluck_object.get(&to_alias).cloned().unwrap_or_default();
                    let where_clause ="".to_string(); //to be created
                    let join_order_direction = to.order_direction.clone().unwrap_or_else(|| String::from("ASC"));
                    let order_by = to.order_by.clone().unwrap_or_else(|| String::from("created_date"));

                    let field_selection = fields
                    .iter()
                    .map(|field| format!("joined_{}.{}", to_alias, field))
                    .collect::<Vec<String>>()
                    .join(", ");
                
                    // Add concatenate query if present
                    let concatenate_part = if !concatenate_query.is_empty() {
                        format!(", {}", concatenate_query.join(", ").trim_end_matches(','))
                    } else {
                        String::new()
                    };

                    // Build ORDER BY clause if present
                    let order_clause = if to.order_by.is_some() {
                        format!("ORDER BY joined_{}.{} {}", to_alias, order_by, join_order_direction.to_uppercase())
                    } else {
                        String::new()
                    };
                    
                    // Build LIMIT clause if present
                    let limit_clause = if let Some(limit) = to.limit {
                        format!("LIMIT {}", limit)
                    } else {
                        String::new()
                    };

                    let from_clause = format!("FROM {} AS joined_{}", to_entity, to_alias);

                    let lateral_join = format!(
                        "LEFT JOIN LATERAL ( SELECT {} {}
                        {} 
                        {} {} {} {} ) AS {} ON TRUE",
                        field_selection,
                        concatenate_part,
                        from_clause,
                        "where_clause", // placeholder 
                        format!("AND {}.{} = joined_{}.{}", from.entity, from.field, to_alias, to.field),
                        order_clause,
                        limit_clause,
                        to_alias
                    );

                    if !result_query.is_empty() {
                        result_query.push_str(" ");
                    }
                    result_query.push_str(&lateral_join);
                },
                _ => {
                    // Now this return will exit the entire function
                    return Err(format!("Invalid join type: {}, supported types are: LEFT", join.r#type));
                }
            }
        }
    }

    Ok(result_query)
}