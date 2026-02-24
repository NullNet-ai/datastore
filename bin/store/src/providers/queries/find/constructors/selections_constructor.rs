use crate::{
    providers::queries::find::sql_constructor::QueryFilter,
    structs::core::{ConcatenateField, FilterCriteria, Join},
};

/// Selections constructor module for SQL query building
pub struct SelectionsConstructor;
impl SelectionsConstructor {
    /// Constructs the SELECT clause for SQL queries
    pub fn construct_selections<T: QueryFilter>(
        request_body: &T,
        table: &str,
        timezone: Option<&str>,
        normalize_entity_name: impl Fn(&str) -> String,
        get_field: impl Fn(&str, &str, &str, &str, Option<&str>, bool) -> String,
        get_field_with_parse_as: impl Fn(
            &str,
            &str,
            &str,
            Option<&str>,
            &str,
            Option<&str>,
            bool,
        ) -> String,
        build_system_where_clause: impl Fn(&str) -> Result<String, String>,
        build_infix_expression: impl Fn(&[FilterCriteria]) -> Result<String, String>,
    ) -> String {
        let mut selections = Vec::new();

        // Handle group_by scenario
        // This is tested from the following:
        // should_construct_group_by_without_count
        if let Some(group_by) = request_body.get_group_by() {
            if !group_by.fields.is_empty() {
                let group_by_selections = Self::construct_group_by_selections(
                    request_body,
                    table,
                    timezone,
                    &mut selections,
                    &normalize_entity_name,
                    &get_field,
                );
                return group_by_selections;
            }
        }

        // Handle distinct_by scenario
        if let Some(distinct_field) = request_body.get_distinct_by() {
            if !distinct_field.is_empty() {
                let field_selection = get_field(
                    table,
                    distinct_field,
                    request_body.get_date_format(),
                    table,
                    timezone,
                    true,
                );
                return format!("DISTINCT {}", field_selection);
            }
        }

        // Determine if main table is in pluck_object
        let main_in_pluck_object = request_body.get_pluck_object().contains_key(table);

        // Include aggregated selections for related entities specified in pluck_object
        if !request_body.get_pluck_object().is_empty() {
            // // If main table exists in pluck_object, add aggregated selection for it
            // if let Some(main_agg) = Self::build_main_table_aggregation(
            //     request_body,
            //     table,
            //     timezone,
            //     &get_field,
            //     &build_system_where_clause,
            // ) {
            //     selections.push(main_agg);
            // }

            let join_selections = Self::construct_join_selections(
                request_body,
                table,
                timezone,
                &normalize_entity_name,
                &get_field,
                &build_system_where_clause,
                &build_infix_expression,
                &get_field_with_parse_as,
            );
            selections.extend(join_selections);
        }

        // Include main table pluck fields when present, unless pluck_object already specifies main table
        if main_in_pluck_object && !request_body.get_pluck().is_empty() {
            let pluck_sel = Self::construct_pluck_object(
                request_body,
                table,
                timezone,
                &get_field,
                &get_field_with_parse_as,
            );
            if !pluck_sel.is_empty() {
                selections.push(pluck_sel);
            }
        } else {
            let pluck_sel = Self::construct_pluck(
                request_body,
                table,
                timezone,
                &get_field,
                &get_field_with_parse_as,
            );
            if !pluck_sel.is_empty() {
                selections.push(pluck_sel);
            }
        }

        // Fallback: if no selections were added, select main table id
        if selections.is_empty() {
            let default_id = get_field(
                table,
                "id",
                request_body.get_date_format(),
                table,
                timezone,
                false,
            );
            selections.push(default_id);
        }

        selections.join(", ")
    }

    /// Resolve a concatenated field to its SELECT expression (expr AS "field_name") if it exists.
    fn get_concatenated_selection_expression(
        concatenate_fields: &[ConcatenateField],
        entity: &str,
        field_name: &str,
        main_table: &str,
        normalize_entity_name: &impl Fn(&str) -> String,
    ) -> Option<String> {
        let normalized_entity = if entity == "self" {
            main_table.to_string()
        } else {
            normalize_entity_name(entity)
        };
        concatenate_fields
            .iter()
            .find(|cf| {
                cf.field_name == field_name
                    && (cf.entity == entity
                        || cf.entity == normalized_entity
                        || cf
                            .aliased_entity
                            .as_deref()
                            .map_or(false, |a| a == entity || a == normalized_entity))
            })
            .map(|cf| {
                format!(
                    "{} AS \"{}\"",
                    cf.to_group_by_expression(&normalized_entity),
                    cf.field_name
                )
            })
    }

    /// Constructs GROUP BY selections with COUNT(*) and grouped fields
    fn construct_group_by_selections<T: QueryFilter>(
        request_body: &T,
        table: &str,
        timezone: Option<&str>,
        acc_selections: &mut Vec<String>,
        normalize_entity_name: &impl Fn(&str) -> String,
        get_field: &impl Fn(&str, &str, &str, &str, Option<&str>, bool) -> String,
    ) -> String {
        if let Some(group_by) = request_body.get_group_by() {
            if !group_by.fields.is_empty() {
                if group_by.has_count {
                    acc_selections.push("COUNT(*) AS count".to_string());
                }

                let concatenate_fields = request_body.get_concatenate_fields();

                // Add group_by fields to selections
                for field in &group_by.fields {
                    let field_parts: Vec<&str> = field.trim().split('.').collect();
                    if field_parts.len() > 1 {
                        // Handle entity.field format
                        let entity_name = field_parts[0];
                        let field_name = field_parts[1];
                        let normalized_entity = normalize_entity_name(entity_name);

                        if let Some(expr) = Self::get_concatenated_selection_expression(
                            concatenate_fields,
                            entity_name,
                            field_name,
                            table,
                            normalize_entity_name,
                        ) {
                            acc_selections.push(expr);
                        } else {
                            let field_selection = get_field(
                                &normalized_entity,
                                field_name,
                                request_body.get_date_format(),
                                table,
                                timezone,
                                true,
                            );
                            acc_selections.push(field_selection);
                        }
                    } else {
                        // Handle single field format (defaults to main table)
                        let field_name = field.as_str();
                        if let Some(expr) = Self::get_concatenated_selection_expression(
                            concatenate_fields,
                            table,
                            field_name,
                            table,
                            normalize_entity_name,
                        ) {
                            acc_selections.push(expr);
                        } else {
                            let field_selection = get_field(
                                table,
                                field,
                                request_body.get_date_format(),
                                table,
                                timezone,
                                true,
                            );
                            acc_selections.push(field_selection);
                        }
                    }
                }
            }
        }

        acc_selections.join(", ")
    }

    /// Helper function to process fields with concatenated field prioritization
    fn process_fields_with_concatenation<T: QueryFilter>(
        fields: &[String],
        request_body: &T,
        table: &str,
        timezone: Option<&str>,
        get_field: &impl Fn(&str, &str, &str, &str, Option<&str>, bool) -> String,
        get_field_with_parse_as: &impl Fn(
            &str,
            &str,
            &str,
            Option<&str>,
            &str,
            Option<&str>,
            bool,
        ) -> String,
    ) -> Vec<String> {
        let mut selections = Vec::new();

        // Collect concatenated field names and their individual fields for this table to check for conflicts
        let mut concatenated_field_names = std::collections::HashSet::new();
        let mut concatenated_source_fields = std::collections::HashSet::new();
        let mut aliased_entities_for_table = std::collections::HashSet::new();
        let mut all_aliased_entities = std::collections::HashSet::new();

        for concat_field in request_body.get_concatenate_fields() {
            if concat_field
                .aliased_entity
                .as_deref()
                .map(|a| a == table)
                .unwrap_or(false)
                || concat_field.entity == table
            {
                // Add the concatenated field name itself
                concatenated_field_names.insert(concat_field.field_name.clone());
                // Add all source fields that are part of this concatenation
                for source_field in &concat_field.fields {
                    concatenated_source_fields.insert(source_field.clone());
                }
                // Track aliased entities that should override pluck_object fields
                if let Some(aliased_entity) = &concat_field.aliased_entity {
                    if concat_field.entity == table {
                        aliased_entities_for_table.insert(aliased_entity.clone());
                    }
                }
            }
            
            // Collect all aliased entities that have concatenated fields
            if let Some(aliased_entity) = &concat_field.aliased_entity {
                all_aliased_entities.insert(aliased_entity.clone());
            }
        }

        // Handle regular pluck fields - only add if not conflicting with concatenated fields
        for field in fields {
            // Skip this field if it's being handled by concatenated fields (prioritize concatenated)
            // Skip both concatenated field names, their source fields, and aliased entities to avoid conflicts
            if concatenated_field_names.contains(field)
                || concatenated_source_fields.contains(field)
                || aliased_entities_for_table.contains(field)
                || all_aliased_entities.contains(field)
            {
                continue;
            }

            let with_alias = field.ends_with("_date")
                || field.ends_with("_time")
                || field.eq_ignore_ascii_case("timestamp");
            let field_selection = get_field(
                table,
                field,
                request_body.get_date_format(),
                table,
                timezone,
                with_alias,
            );
            selections.push(field_selection);
        }

        // Handle concatenated fields only for main table entity
        for concat_field in request_body.get_concatenate_fields() {
            if concat_field
                .aliased_entity
                .as_deref()
                .map(|a| a == table)
                .unwrap_or(false)
                || concat_field.entity == table
            {
                let concatenated_expression = concat_field
                    .fields
                    .iter()
                    .map(|f| {
                        format!(
                            "COALESCE({}, '')",
                            get_field_with_parse_as(
                                table,
                                f,
                                request_body.get_date_format(),
                                None,
                                table,
                                timezone,
                                false,
                            )
                        )
                    })
                    .collect::<Vec<_>>()
                    .join(&format!(" || '{}' || ", concat_field.separator));

                selections.push(format!(
                    "({}) AS {}",
                    concatenated_expression, concat_field.field_name
                ));
            }
        }
        
        selections
    }

    /// Constructs PLUCK selections for specific fields
    fn construct_pluck<T: QueryFilter>(
        request_body: &T,
        table: &str,
        timezone: Option<&str>,
        get_field: &impl Fn(&str, &str, &str, &str, Option<&str>, bool) -> String,
        get_field_with_parse_as: &impl Fn(
            &str,
            &str,
            &str,
            Option<&str>,
            &str,
            Option<&str>,
            bool,
        ) -> String,
    ) -> String {
        println!("Pluck fields: {:?}", request_body.get_pluck());
        let pluck_selections = Self::process_fields_with_concatenation(
            request_body.get_pluck(),
            request_body,
            table,
            timezone,
            get_field,
            get_field_with_parse_as,
        );
        pluck_selections.join(", ")
    }

    /// Constructs PLUCK selections for specific fields in pluck_object
    fn construct_pluck_object<T: QueryFilter>(
        request_body: &T,
        table: &str,
        timezone: Option<&str>,
        get_field: &impl Fn(&str, &str, &str, &str, Option<&str>, bool) -> String,
        get_field_with_parse_as: &impl Fn(
            &str,
            &str,
            &str,
            Option<&str>,
            &str,
            Option<&str>,
            bool,
        ) -> String,
    ) -> String {
        println!("Pluck object fields: {:?}", request_body.get_pluck_object());
        let default_fields = Vec::new();
        let fields = request_body
            .get_pluck_object()
            .get(table)
            .unwrap_or(&default_fields);
         
         let pluck_object_selections = Self::process_fields_with_concatenation(
             fields,
             request_body,
             table,
             timezone,
             get_field,
             get_field_with_parse_as,
         );
        pluck_object_selections.join(", ")
    }

    /// Helper method to build field pairs for JSONB_BUILD_OBJECT
    fn build_field_pairs<T: QueryFilter>(
        request_body: &T,
        table: &str,
        timezone: Option<&str>,
        fields: &[String],
        to_alias: &str,
        get_field: &impl Fn(&str, &str, &str, &str, Option<&str>, bool) -> String,
    ) -> Vec<String> {
        fields
            .iter()
            .map(|field| {
                let field_query = get_field(
                    to_alias,
                    field,
                    request_body.get_date_format(),
                    table,
                    timezone,
                    false,
                );
                let parts: Vec<String> = field_query
                    .split(" AS ")
                    .map(|part| part.to_string())
                    .collect::<Vec<String>>();
                let formatted_field = parts.first().unwrap().clone();
                format!("'{}', {}", field, formatted_field)
            })
            .collect()
    }

    /// Helper method to add concatenated field pairs
    fn add_concatenated_field_pairs<T: QueryFilter>(
        request_body: &T,
        table: &str,
        timezone: Option<&str>,
        field_pairs: &mut Vec<String>,
        to_alias: &str,
        normalize_entity_name: &impl Fn(&str) -> String,
        get_field: &impl Fn(&str, &str, &str, &str, Option<&str>, bool) -> String,
    ) {
        if !request_body.get_concatenate_fields().is_empty() {
            request_body
                .get_concatenate_fields()
                .iter()
                .filter(|field| {
                    let normalized_entity = normalize_entity_name(&field.entity);
                    field.aliased_entity.as_deref() == Some(to_alias)
                        || field.entity == to_alias
                        || normalized_entity == to_alias
                })
                .for_each(|field| {
                    // Use normalized entity name when no aliased_entity is present
                    let normalized_entity = normalize_entity_name(&field.entity);
                    let table_name = field
                        .aliased_entity
                        .as_deref()
                        .unwrap_or(&normalized_entity);
                    let concatenated_expression = field
                        .fields
                        .iter()
                        .map(|f| {
                            format!(
                                "COALESCE({}, '')",
                                get_field(
                                    table_name,
                                    f,
                                    request_body.get_date_format(),
                                    table,
                                    timezone,
                                    false,
                                )
                            )
                        })
                        .collect::<Vec<_>>()
                        .join(&format!(" || '{}' || ", field.separator));

                    field_pairs.push(format!(
                        "'{}', ({})",
                        field.field_name, concatenated_expression
                    ));
                });
        }
    }

    /// Helper to build a nested aggregated selection for a child join embedded within a parent alias
    fn build_nested_child_selection<T: QueryFilter>(
        request_body: &T,
        table: &str,
        timezone: Option<&str>,
        parent_alias: &str,
        child_join: &Join,
        fields: &[String],
        normalize_entity_name: &impl Fn(&str) -> String,
        get_field: &impl Fn(&str, &str, &str, &str, Option<&str>, bool) -> String,
        build_system_where_clause: &impl Fn(&str) -> Result<String, String>,
        build_infix_expression: &impl Fn(&[FilterCriteria]) -> Result<String, String>,
    ) -> String {
        let to_alias = child_join
            .field_relation
            .to
            .alias
            .as_deref()
            .unwrap_or(&child_join.field_relation.to.entity);
        let target_table = &child_join.field_relation.to.entity;
        let mut field_pairs =
            Self::build_field_pairs(request_body, table, timezone, fields, to_alias, get_field);
        Self::add_concatenated_field_pairs(
            request_body,
            table,
            timezone,
            &mut field_pairs,
            to_alias,
            normalize_entity_name,
            get_field,
        );

        let mut where_conditions = Vec::new();
        let standard_where = match build_system_where_clause(to_alias) {
            Ok(clause) => clause,
            Err(_) => format!("({}.tombstone = 0)", to_alias),
        };
        where_conditions.push(standard_where);

        // Child join condition referencing the parent alias
        let join_condition = format!(
            "\"{}\".\"{}\" = \"{}\".\"{}\"",
            to_alias,
            child_join.field_relation.to.field,
            parent_alias,
            child_join.field_relation.from.field
        );
        where_conditions.push(join_condition);

        // Add filters if present on child join
        if !child_join.field_relation.to.filters.is_empty() {
            if let Ok(filter_expression) =
                build_infix_expression(&child_join.field_relation.to.filters)
            {
                if !filter_expression.is_empty() {
                    where_conditions.push(filter_expression);
                }
            }
        }

        let combined_where = where_conditions.join(" AND ");
        let order_by_clause = Self::build_join_order_by_clause(child_join, "elem");

        format!(
            "COALESCE( ( SELECT JSONB_AGG(elem {}) FROM (SELECT JSONB_BUILD_OBJECT({}) AS elem FROM {} {} WHERE {}) sub ), '[]' )",
            order_by_clause,
            field_pairs.join(", "),
            target_table,
            to_alias,
            combined_where
        )
    }

    /// Helper method to build the final join selection
    fn build_join_selection<T: QueryFilter>(
        _request_body: &T,
        join: &Join,
        previous_join: Option<&Join>,
        to_alias: &str,
        target_table: &str,
        field_pairs: &[String],
        join_condition: &str,
        build_system_where_clause: &impl Fn(&str) -> Result<String, String>,
        build_infix_expression: &impl Fn(&[FilterCriteria]) -> Result<String, String>,
    ) -> String {
        let mut where_conditions = Vec::new();

        // Add system where clause
        let standard_where = match build_system_where_clause(to_alias) {
            Ok(clause) => clause,
            Err(_) => format!("({}.tombstone = 0)", to_alias),
        };
        where_conditions.push(standard_where);

        // Add join condition
        where_conditions.push(join_condition.to_string());

        // Add filters from 'to' RelationEndpoint if present
        if !join.field_relation.to.filters.is_empty() {
            match build_infix_expression(&join.field_relation.to.filters) {
                Ok(filter_expression) if !filter_expression.is_empty() => {
                    where_conditions.push(filter_expression);
                }
                Err(_) => {
                    // Log error or handle gracefully - for now, continue without the filter
                }
                _ => {}
            }
        }

        let combined_where = where_conditions.join(" AND ");

        // Build order_by clause with join-specific override logic
        let order_by_clause = Self::build_join_order_by_clause(join, "elem");

        if join.nested {
            if let Some(prev_join) = previous_join {
                let _prev_join_to_alias = if prev_join.r#type == "self" {
                    prev_join
                        .field_relation
                        .from
                        .alias
                        .as_deref()
                        .unwrap_or(&prev_join.field_relation.to.entity)
                } else {
                    prev_join
                        .field_relation
                        .to
                        .alias
                        .as_deref()
                        .unwrap_or(&prev_join.field_relation.to.entity)
                };
                format!(
                    "COALESCE( ( SELECT JSONB_AGG(elem {}) FROM (SELECT JSONB_BUILD_OBJECT({}) AS elem FROM {} {} WHERE {}) sub ), '[]' ) AS {}",
                    order_by_clause,
                    field_pairs.join(", "),
                    target_table,
                    to_alias,
                    combined_where,
                    to_alias
                )
            } else {
                // This should not happen for nested joins, but provide a fallback
                format!(
                    "COALESCE( ( SELECT JSONB_AGG(elem {}) FROM (SELECT JSONB_BUILD_OBJECT({}) AS elem FROM {} {} WHERE {}) sub ), '[]' ) AS {}",
                    order_by_clause,
                    field_pairs.join(", "),
                    target_table,
                    to_alias,
                    combined_where,
                    to_alias
                )
            }
        } else {
            format!(
                "COALESCE( ( SELECT JSONB_AGG(elem {}) FROM (SELECT JSONB_BUILD_OBJECT({}) AS elem FROM {} {} WHERE {}) sub ), '[]' ) AS {}",
                order_by_clause,
                field_pairs.join(", "),
                target_table,
                to_alias,
                combined_where,
                to_alias
            )
        }
    }

    /// Builds ORDER BY clause for join selections with join-specific override logic
    fn build_join_order_by_clause(join: &Join, alias_elem: &str) -> String {
        // Check if join has specific order_by and order_direction
        if let (Some(join_order_by), Some(join_order_direction)) = (
            &join.field_relation.to.order_by,
            &join.field_relation.to.order_direction,
        ) {
            if !join_order_by.is_empty() && !join_order_direction.is_empty() {
                // Handle case sensitivity (default to case-insensitive for joins)
                let final_field = format!("'{}'", join_order_by);

                return format!(
                    " ORDER BY {}->>{} {}",
                    alias_elem,
                    final_field,
                    join_order_direction.to_uppercase()
                );
            }
        }

        // Fallback to request body order_by if no join-specific ordering
        String::from("")
    }

    /// Builds join condition for alias
    fn build_join_condition_for_alias<T: QueryFilter>(
        request_body: &T,
        table: &str,
        alias: &str,
        join: &Join,
        previous_join: Option<&Join>,
    ) -> String {
        let is_nested = join.nested;
        let from_field = &join.field_relation.from.field;
        let to_field = &join.field_relation.to.field;

        // For nested joins, we need to build the correct join condition
        if is_nested {
            if let Some(prev_join) = previous_join {
                // For nested joins, the current join's "from" entity should reference the previous join's "to" result
                let _prev_join_to_alias = if prev_join.r#type == "self" {
                    prev_join
                        .field_relation
                        .from
                        .alias
                        .as_deref()
                        .unwrap_or(&prev_join.field_relation.to.entity)
                } else {
                    prev_join
                        .field_relation
                        .to
                        .alias
                        .as_deref()
                        .unwrap_or(&prev_join.field_relation.to.entity)
                };

                // The join condition should be: current_to_field = previous_from_field
                // where current_to_field is the field in the current table being joined TO
                // and previous_from_field is the field in the previous result that we're joining FROM
                return format!(
                    "\"{}\".\"{}\" = \"{}\".\"{}\"",
                    alias, to_field, _prev_join_to_alias, from_field
                );
            }
        }

        // Determine the correct from table reference
        // If the from entity has an alias, use it; otherwise check if it matches an existing alias from previous joins
        let from_table_ref = if let Some(from_alias) = &join.field_relation.from.alias {
            from_alias.as_str()
        } else {
            // Check if the from entity matches any previous join's alias
            let from_entity = &join.field_relation.from.entity;

            // Look for a previous join that created this alias
            let matching_alias = request_body.get_joins().iter().find_map(|j| {
                if let Some(alias) = &j.field_relation.to.alias {
                    if alias == from_entity {
                        return Some(alias.as_str());
                    }
                }
                None
            });

            matching_alias.unwrap_or_else(|| {
                // If no alias found and from_entity equals main table, use main table
                if from_entity == table {
                    table
                } else {
                    // Otherwise use the from_entity as-is (it should be an alias)
                    from_entity
                }
            })
        };

        format!(
            "\"{}\".\"{}\" = \"{}\".\"{}\"",
            from_table_ref, from_field, alias, to_field
        )
    }

    fn build_join_condition_for_from_alias<T: QueryFilter>(
        request_body: &T,
        table: &str,
        alias: &str,
        join: &Join,
        _previous_join: Option<&Join>,
    ) -> String {
        let to_field = &join.field_relation.to.field;
        let from_field = &join.field_relation.from.field;
        let to_table_ref = if let Some(to_alias) = &join.field_relation.to.alias {
            to_alias.as_str()
        } else {
            let to_entity = &join.field_relation.to.entity;
            let matching_alias = request_body.get_joins().iter().find_map(|j| {
                if let Some(alias) = &j.field_relation.to.alias {
                    if alias == to_entity {
                        return Some(alias.as_str());
                    }
                }
                None
            });
            matching_alias.unwrap_or_else(|| if to_entity == table { table } else { to_entity })
        };
        format!(
            "\"{}\".\"{}\" = \"{}\".\"{}\"",
            alias, from_field, to_table_ref, to_field
        )
    }

    /// Constructs JOIN selections for related entities when pluck_object is provided
    fn construct_join_selections<T: QueryFilter>(
        request_body: &T,
        table: &str,
        timezone: Option<&str>,
        normalize_entity_name: &impl Fn(&str) -> String,
        get_field: &impl Fn(&str, &str, &str, &str, Option<&str>, bool) -> String,
        build_system_where_clause: &impl Fn(&str) -> Result<String, String>,
        build_infix_expression: &impl Fn(&[FilterCriteria]) -> Result<String, String>,
        _get_field_with_parse_as: &impl Fn(
            &str,
            &str,
            &str,
            Option<&str>,
            &str,
            Option<&str>,
            bool,
        ) -> String,
    ) -> Vec<String> {
        let mut join_selections = Vec::new();
        let mut added_entity_selection = std::collections::HashSet::new();
        // Process each join
        for (join_index, join) in request_body.get_joins().iter().enumerate() {
            println!(
                "DEBUG: Processing join {}: from={}, to={}",
                join_index, join.field_relation.from.entity, join.field_relation.to.entity
            );
            let from_alias = join
                .field_relation
                .from
                .alias
                .as_deref()
                .unwrap_or(&join.field_relation.from.entity);
            let to_alias = join
                .field_relation
                .to
                .alias
                .as_deref()
                .unwrap_or(&join.field_relation.to.entity);

            // Skip adding duplicate selection for main table in 'from' branch; still process 'to' branch
            let _skip_from_branch = from_alias == table;

            // Handle fields for this join tables from "from"
            if request_body.get_pluck_object().contains_key(from_alias)
                && !added_entity_selection.contains(from_alias)
                && from_alias != table
            // Skip main table - it's handled separately
            {
                if let Some(fields) = request_body.get_pluck_object().get(from_alias) {
                    let target_table: String = request_body
                        .get_joins()
                        .iter()
                        .find_map(|j| {
                            if let Some(alias) = &j.field_relation.to.alias {
                                if alias == from_alias {
                                    return Some(j.field_relation.to.entity.clone());
                                }
                            }
                            None
                        })
                        .unwrap_or_else(|| join.field_relation.from.entity.clone());
                    let mut field_pairs = Self::build_field_pairs(
                        request_body,
                        table,
                        timezone,
                        fields,
                        from_alias,
                        get_field,
                    );
                    Self::add_concatenated_field_pairs(
                        request_body,
                        table,
                        timezone,
                        &mut field_pairs,
                        from_alias,
                        normalize_entity_name,
                        get_field,
                    );
                    // Embed nested children selections that reference this alias
                    for child_join in request_body
                        .get_joins()
                        .iter()
                        .filter(|cj| cj.nested && cj.field_relation.from.entity == from_alias)
                    {
                        let child_to_alias = child_join
                            .field_relation
                            .to
                            .alias
                            .as_deref()
                            .unwrap_or(&child_join.field_relation.to.entity);
                        if let Some(child_fields) =
                            request_body.get_pluck_object().get(child_to_alias)
                        {
                            let nested_sel = Self::build_nested_child_selection(
                                request_body,
                                table,
                                timezone,
                                from_alias,
                                child_join,
                                child_fields,
                                normalize_entity_name,
                                get_field,
                                build_system_where_clause,
                                build_infix_expression,
                            );
                            field_pairs.push(format!("'{}', {}", child_to_alias, nested_sel));
                            added_entity_selection.insert(child_to_alias.to_string());
                        }
                    }
                    let join_condition = Self::build_join_condition_for_from_alias(
                        request_body,
                        table,
                        from_alias,
                        join,
                        None,
                    );
                    let mut where_conditions = Vec::new();
                    let standard_where = match build_system_where_clause(from_alias) {
                        Ok(clause) => clause,
                        Err(_) => format!("({}.tombstone = 0)", from_alias),
                    };
                    where_conditions.push(standard_where);
                    where_conditions.push(join_condition);
                    let combined_where = where_conditions.join(" AND ");
                    let order_by_clause = String::from("");
                    join_selections.push(format!(
                        "COALESCE( ( SELECT JSONB_AGG(elem {}) FROM (SELECT JSONB_BUILD_OBJECT({}) AS elem FROM {} {} WHERE {}) sub ), '[]' ) AS {}",
                        order_by_clause,
                        field_pairs.join(", "),
                        target_table,
                        from_alias,
                        combined_where,
                        from_alias
                    ));
                    added_entity_selection.insert(from_alias.to_string());
                }
            }

            // Handle fields for this join tables from "to"
            if let Some(fields) = request_body.get_pluck_object().get(to_alias) {
                // Skip main table - it's handled separately
                if to_alias == table {
                    continue;
                }

                let target_table = &join.field_relation.to.entity;

                // Find previous join in chain if exists
                let previous_join = if join.nested && join_index > 0 {
                    Some(&request_body.get_joins()[join_index - 1])
                } else {
                    request_body.get_joins().iter().find(|j| {
                        let j_to_ref = j
                            .field_relation
                            .to
                            .alias
                            .as_deref()
                            .unwrap_or(&j.field_relation.to.entity);
                        let current_from_ref = join
                            .field_relation
                            .from
                            .alias
                            .as_deref()
                            .unwrap_or(&join.field_relation.from.entity);
                        j_to_ref == current_from_ref
                    })
                };

                let join_condition = Self::build_join_condition_for_alias(
                    request_body,
                    table,
                    to_alias,
                    join,
                    previous_join,
                );

                // Build field pairs for JSONB_BUILD_OBJECT
                let mut field_pairs = Self::build_field_pairs(
                    request_body,
                    table,
                    timezone,
                    fields,
                    to_alias,
                    get_field,
                );

                // Add concatenated fields if any match this alias
                Self::add_concatenated_field_pairs(
                    request_body,
                    table,
                    timezone,
                    &mut field_pairs,
                    to_alias,
                    normalize_entity_name,
                    get_field,
                );

                // Build the selection
                let selection = Self::build_join_selection(
                    request_body,
                    join,
                    previous_join,
                    to_alias,
                    target_table,
                    &field_pairs,
                    &join_condition,
                    build_system_where_clause,
                    build_infix_expression,
                );

                join_selections.push(selection);
            }
        }

        join_selections
    }

    // /// Builds aggregated JSON selection for the main table when pluck_object includes it
    // fn build_main_table_aggregation<T: QueryFilter>(
    //     request_body: &T,
    //     table: &str,
    //     timezone: Option<&str>,
    //     get_field: &impl Fn(&str, &str, &str, &str, Option<&str>, bool) -> String,
    //     build_system_where_clause: &impl Fn(&str) -> Result<String, String>,
    // ) -> Option<String> {
    //     if let Some(fields) = request_body.get_pluck_object().get(table) {
    //         let joined_alias = format!("joined_{}", table);
    //         // Build field pairs using the actual table name (not the joined alias)
    //         let field_pairs = fields
    //             .iter()
    //             .map(|field| {
    //                 let field_query = get_field(
    //                     table,
    //                     field,
    //                     request_body.get_date_format(),
    //                     table,
    //                     timezone,
    //                     false,
    //                 );
    //                 let parts: Vec<String> = field_query
    //                     .split(" AS ")
    //                     .map(|part| part.to_string())
    //                     .collect::<Vec<String>>();
    //                 let formatted_field = parts.first().unwrap().clone();
    //                 format!("'{}', {}", field, formatted_field)
    //             })
    //             .collect::<Vec<String>>();

    //         // Build WHERE with system constraints on the joined alias and correlate by id
    //         let mut where_conditions = Vec::new();
    //         let standard_where = match build_system_where_clause(&joined_alias) {
    //             Ok(clause) => clause,
    //             Err(_) => format!("({}.tombstone = 0)", joined_alias),
    //         };
    //         where_conditions.push(standard_where);
    //         where_conditions.push(format!(
    //             "\"{}\".\"id\" = \"{}\".\"id\"",
    //             joined_alias, table
    //         ));
    //         let combined_where = where_conditions.join(" AND ");

    //         return Some(format!(
    //             "COALESCE( ( SELECT JSONB_AGG(elem ) FROM (SELECT JSONB_BUILD_OBJECT({}) AS elem FROM {} {} WHERE {}) sub ), '[]' ) AS {}",
    //             field_pairs.join(", "),
    //             table,
    //             joined_alias,
    //             combined_where,
    //             table
    //         ));
    //     }
    //     None
    // }
}
