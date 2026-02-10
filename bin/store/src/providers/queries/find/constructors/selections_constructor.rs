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

        // This is tested from the following:
        // should_construct_selections_with_pluck_fields_pluck_object
        // should_construct_selections_with_pluck_fields_joins_pluck_object
        // should_construct_concatenated_fields_for_pluck_object_join_selections_with_aliased_entity
        if !request_body.get_pluck_object().is_empty() {
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
        // This is tested from from the following:
        // should_construct_default_selections
        else if !request_body.get_pluck().is_empty() {
            return Self::construct_pluck(
                request_body,
                table,
                timezone,
                &get_field,
                &get_field_with_parse_as,
            );
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
        let mut pluck_selections = Vec::new();
        // Handle regular pluck fields
        for field in request_body.get_pluck() {
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
            pluck_selections.push(field_selection);
        }

        // Handle concatenated fields
        for concat_field in request_body.get_concatenate_fields() {
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

            pluck_selections.push(format!(
                "({}) AS {}",
                concatenated_expression, concat_field.field_name
            ));
        }
        pluck_selections.join(", ")
    }

    /// Constructs PLUCK selections with pluck_object support for main table
    fn construct_pluck_with_object_for_main<T: QueryFilter>(
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
        let mut pluck_selections = Vec::new();

        // Check if main table has fields in pluck_object, otherwise use regular pluck
        let fields_to_use =
            if let Some(main_table_fields) = request_body.get_pluck_object().get(table) {
                println!(
                    "Using pluck_object fields for table {}: {:?}",
                    table, main_table_fields
                );
                main_table_fields
            } else {
                println!("Using regular pluck fields: {:?}", request_body.get_pluck());
                request_body.get_pluck()
            };

        // Handle the selected fields
        for field in fields_to_use {
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
            pluck_selections.push(field_selection);
        }

        // Handle concatenated fields
        for concat_field in request_body.get_concatenate_fields() {
            if concat_field.entity != table {
                continue;
            }
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

            pluck_selections.push(format!(
                "({}) AS {}",
                concatenated_expression, concat_field.field_name
            ));
        }

        pluck_selections.join(", ")
    }

    /// Constructs JOIN selections for related entities
    fn construct_join_selections<T: QueryFilter>(
        request_body: &T,
        table: &str,
        timezone: Option<&str>,
        normalize_entity_name: &impl Fn(&str) -> String,
        get_field: &impl Fn(&str, &str, &str, &str, Option<&str>, bool) -> String,
        build_system_where_clause: &impl Fn(&str) -> Result<String, String>,
        build_infix_expression: &impl Fn(&[FilterCriteria]) -> Result<String, String>,
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
        let mut join_selections = Vec::new();
        if request_body.get_joins().is_empty() {
            join_selections.push(Self::construct_pluck(
                request_body,
                table,
                timezone,
                &get_field,
                &get_field_with_parse_as,
            ));

            return join_selections;
        }

        let mut added_entity_selection = std::collections::HashSet::new();
        // Process each join
        for join in request_body.get_joins() {
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

            // Handle fields for this join tables from "from"
            if request_body.get_pluck_object().contains_key(from_alias)
                && !join.nested
                && !added_entity_selection.contains(from_alias)
            {
                join_selections.push(Self::construct_pluck_with_object_for_main(
                    request_body,
                    from_alias,
                    timezone,
                    &get_field,
                    &get_field_with_parse_as,
                ));
                added_entity_selection.insert(from_alias.to_string());
            }

            // Handle fields for this join tables from "to"
            if let Some(fields) = request_body.get_pluck_object().get(to_alias) {
                let target_table = &join.field_relation.to.entity;

                // Find previous join in chain if exists
                let previous_join = request_body.get_joins().iter().find(|j| {
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
                });

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

    /// Helper method to build the final join selection
    fn build_join_selection<T: QueryFilter>(
        request_body: &T,
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
            let prev_join = previous_join.unwrap();
            let prev_join_to_alias = prev_join
                .field_relation
                .to
                .alias
                .as_deref()
                .unwrap_or(&prev_join.field_relation.to.entity);

            format!(
                "COALESCE( ( SELECT JSONB_AGG(elem {}) FROM (SELECT JSONB_BUILD_OBJECT({}) AS elem FROM {} {} LEFT JOIN {} {} ON {} WHERE {}) sub ), '[]' ) AS {}",
                order_by_clause,
                field_pairs.join(", "),
                prev_join.field_relation.to.entity,
                prev_join_to_alias,
                target_table,
                to_alias,
                Self::build_join_condition_for_alias(request_body, "", to_alias, join, Some(join)),
                combined_where,
                to_alias
            )
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
        if is_nested {
            if let Some(prev_join) = previous_join {
                // Use the previous join's alias if available
                let prev_from_alias = prev_join
                    .field_relation
                    .from
                    .alias
                    .as_deref()
                    .unwrap_or(&prev_join.field_relation.from.entity);
                let prev_from_field = &prev_join.field_relation.from.field;
                let prev_to_alias = prev_join
                    .field_relation
                    .to
                    .alias
                    .as_deref()
                    .unwrap_or(&prev_join.field_relation.to.entity);
                let prev_to_field = &prev_join.field_relation.to.field;
                return format!(
                    "\"{}\".\"{}\" = \"{}\".\"{}\"",
                    prev_from_alias, prev_from_field, prev_to_alias, prev_to_field
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
}
