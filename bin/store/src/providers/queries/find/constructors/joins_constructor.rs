use crate::{
    providers::queries::find::sql_constructor::QueryFilter,
    structs::core::{FilterCriteria, Join},
};

/// Joins constructor module for SQL query building
pub struct JoinsConstructor;

impl JoinsConstructor {
    /// Constructs JOIN clauses for SQL queries
    pub fn construct_joins<T: QueryFilter>(
        request_body: &T,
        table: &str,
        build_system_where_clause: impl Fn(&str) -> Result<String, String>,
        build_infix_expression: impl Fn(&[FilterCriteria]) -> Result<String, String>,
    ) -> String {
        if request_body.get_joins().is_empty() {
            String::from("")
        } else {
            let mut join_clauses = Vec::new();

            let joins = request_body.get_joins();
            for (index, join) in joins.iter().enumerate() {
                match join.r#type.as_str() {
                    "left" => {
                        let join_clause = Self::build_left_join_lateral(
                            request_body,
                            table,
                            join,
                            &build_system_where_clause,
                            &build_infix_expression,
                        );
                        join_clauses.push(join_clause);
                    }
                    "right" => {
                        let join_clause = Self::build_right_join_lateral(
                            request_body,
                            table,
                            join,
                            index,
                            joins,
                            &build_system_where_clause,
                            &build_infix_expression,
                        );
                        join_clauses.push(join_clause);
                    }
                    "inner" => {
                        let join_clause = Self::build_inner_join_lateral(
                            request_body,
                            table,
                            join,
                            &build_system_where_clause,
                            &build_infix_expression,
                        );
                        join_clauses.push(join_clause);
                    }
                    "self" => {
                        let join_clause = Self::build_self_join_lateral(
                            request_body,
                            table,
                            join,
                            &build_system_where_clause,
                            &build_infix_expression,
                        );
                        join_clauses.push(join_clause);
                    }
                    _ => {
                        // Unsupported join type, skip or log warning
                        log::warn!("Unsupported join type: {}", join.r#type);
                    }
                }
            }

            if join_clauses.is_empty() {
                String::from("")
            } else {
                format!(" {}", join_clauses.join(" "))
            }
        }
    }

    /// Builds a LEFT JOIN LATERAL clause
    fn build_left_join_lateral<T: QueryFilter>(
        request_body: &T,
        table: &str,
        join: &Join,
        build_system_where_clause: &impl Fn(&str) -> Result<String, String>,
        build_infix_expression: &impl Fn(&[FilterCriteria]) -> Result<String, String>,
    ) -> String {
        Self::build_join_lateral(
            request_body,
            table,
            join,
            false,
            "LEFT",
            build_system_where_clause,
            build_infix_expression,
        )
    }

    /// Builds an INNER JOIN LATERAL clause (same structure as LEFT: WHERE in subquery, ON TRUE)
    fn build_inner_join_lateral<T: QueryFilter>(
        request_body: &T,
        table: &str,
        join: &Join,
        build_system_where_clause: &impl Fn(&str) -> Result<String, String>,
        build_infix_expression: &impl Fn(&[FilterCriteria]) -> Result<String, String>,
    ) -> String {
        Self::build_join_lateral(
            request_body,
            table,
            join,
            false,
            "INNER",
            build_system_where_clause,
            build_infix_expression,
        )
    }

    /// Builds a RIGHT JOIN LATERAL clause.
    /// Same structure as build_join_lateral (LEFT); only difference: no WHERE in subquery, conditions go in ON.
    fn build_right_join_lateral<T: QueryFilter>(
        request_body: &T,
        _table: &str,
        join: &Join,
        join_index: usize,
        joins: &[Join],
        build_system_where_clause: &impl Fn(&str) -> Result<String, String>,
        build_infix_expression: &impl Fn(&[FilterCriteria]) -> Result<String, String>,
    ) -> String {
        let to_entity = &join.field_relation.to.entity;
        let to_alias = join.field_relation.to.alias.as_deref().unwrap_or(to_entity);
        let to_field = &join.field_relation.to.field;
        let from_entity = &join.field_relation.from.entity;
        let from_field = &join.field_relation.from.field;
        let is_nested = join.nested;

        // Same as LEFT: lateral alias used inside subquery (for RIGHT, result is also exposed as to_alias so ON can reference it)
        let lateral_alias = to_alias;

        // Build dynamic field selection based on pluck_object (same as LEFT)
        // For RIGHT we must include tombstone and organization_id so the ON clause can reference them
        let mut selected_field_names: Vec<&str> =
            if let Some(fields) = request_body.get_pluck_object().get(to_alias) {
                fields.iter().map(|s| s.as_str()).collect()
            } else {
                vec!["id"]
            };
        if !selected_field_names.iter().any(|&f| f == "tombstone") {
            selected_field_names.push("tombstone");
        }
        if !selected_field_names.iter().any(|&f| f == "organization_id") {
            selected_field_names.push("organization_id");
        }
        let selected_fields = selected_field_names
            .iter()
            .map(|field| format!("\"{}\".\"{}\"", lateral_alias, field))
            .collect::<Vec<_>>()
            .join(", ");

        // Same as LEFT: build where conditions (used in ON for RIGHT instead of WHERE)
        let mut where_conditions = vec![build_system_where_clause(lateral_alias)
            .unwrap_or_else(|_| format!("(\"{}\".\"tombstone\" = 0)", lateral_alias))];

        if !join.field_relation.to.filters.is_empty() {
            match build_infix_expression(&join.field_relation.to.filters) {
                Ok(filter_expression) if !filter_expression.is_empty() => {
                    where_conditions.push(filter_expression);
                }
                Err(_) => {}
                _ => {}
            }
        }

        let combined_where = where_conditions.join(" AND ");

        // Same as LEFT: determine the correct from table reference for the join condition
        // Prioritize from.alias over from.entity when creating SQL alias for selections
        let from_table_ref = if let Some(alias) = &join.field_relation.from.alias {
            alias.as_str()
        } else if is_nested && join_index > 0 {
            let prev_join = &joins[join_index - 1];
            prev_join
                .field_relation
                .to
                .alias
                .as_deref()
                .unwrap_or(prev_join.field_relation.to.entity.as_str())
        } else {
            let mut found_alias = None;
            for j in joins {
                if let Some(a) = &j.field_relation.to.alias {
                    if a == from_entity {
                        found_alias = Some(a.as_str());
                        break;
                    }
                }
            }
            found_alias.unwrap_or(from_entity.as_str())
        };

        let join_condition = format!(
            "\"{}\".\"{}\" = \"{}\".\"{}\"",
            lateral_alias, to_field, from_table_ref, from_field
        );

        // RIGHT: no WHERE in subquery; put combined_where + join_condition in ON
        if is_nested {
            let prev_join = &joins[join_index - 1];
            let prev_join_to_alias = prev_join
                .field_relation
                .to
                .alias
                .as_deref()
                .unwrap_or(prev_join.field_relation.to.entity.as_str());
            let nested_cond = format!(
                "\"{}\".\"{}\" = \"{}\".\"{}\"",
                lateral_alias, to_field, prev_join_to_alias, prev_join.field_relation.from.field
            );
            return format!(
                "RIGHT JOIN LATERAL (SELECT {} FROM \"{}\" \"{}\") AS \"{}\" ON ({} AND {})",
                selected_fields,
                to_entity,
                lateral_alias,
                to_alias,
                combined_where,
                format!("{} AND {}", join_condition, nested_cond)
            );
        }

        format!(
            "RIGHT JOIN LATERAL (SELECT {} FROM \"{}\" \"{}\") AS \"{}\" ON ({} AND {})",
            selected_fields, to_entity, lateral_alias, to_alias, combined_where, join_condition
        )
    }

    /// Builds a SELF JOIN LATERAL clause
    fn build_self_join_lateral<T: QueryFilter>(
        request_body: &T,
        table: &str,
        join: &Join,
        build_system_where_clause: &impl Fn(&str) -> Result<String, String>,
        build_infix_expression: &impl Fn(&[FilterCriteria]) -> Result<String, String>,
    ) -> String {
        Self::build_join_lateral(
            request_body,
            table,
            join,
            true,
            "LEFT",
            build_system_where_clause,
            build_infix_expression,
        )
    }

    /// Builds a JOIN LATERAL clause (generic implementation). join_kind is "LEFT" or "INNER".
    fn build_join_lateral<T: QueryFilter>(
        request_body: &T,
        table: &str,
        join: &Join,
        is_self_join: bool,
        join_kind: &str,
        build_system_where_clause: &impl Fn(&str) -> Result<String, String>,
        build_infix_expression: &impl Fn(&[FilterCriteria]) -> Result<String, String>,
    ) -> String {
        let to_entity = if is_self_join {
            table
        } else {
            &join.field_relation.to.entity
        };

        let to_alias = if is_self_join {
            // For self joins, prefer the alias specified on the 'from' side (JSON places alias there)
            if let Some(from_alias) = &join.field_relation.from.alias {
                from_alias.as_str()
            } else {
                join.field_relation.to.alias.as_deref().unwrap_or(table)
            }
        } else {
            join.field_relation.to.alias.as_deref().unwrap_or(to_entity)
        };

        let to_field = &join.field_relation.to.field;
        let from_entity = if is_self_join {
            table
        } else {
            &join.field_relation.from.entity
        };
        let from_field = &join.field_relation.from.field;
        let is_nested = join.nested;

        // Build the lateral subquery alias
        let lateral_alias = format!("joined_{}", to_alias);

        // Build dynamic field selection based on pluck_object
        let selected_fields = if let Some(fields) = request_body.get_pluck_object().get(to_alias) {
            fields
                .iter()
                .map(|field| format!("\"{}\".\"{}\"", lateral_alias, field))
                .collect::<Vec<_>>()
                .join(", ")
        } else {
            // Default fallback fields if no pluck_object configuration found
            format!("\"{}\".\"id\"", lateral_alias)
        };

        let mut where_conditions = vec![build_system_where_clause(&lateral_alias)
            .unwrap_or_else(|_| format!("(\"{}\".\"tombstone\" = 0)", lateral_alias))];

        // Add filters from the 'to' RelationEndpoint if they exist
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

        if is_nested {
            // For nested joins, we need to include an additional join inside the lateral subquery
            // Use the from.alias for the joined table reference
            let nested_join_alias = if let Some(alias) = &join.field_relation.from.alias {
                alias.as_str()
            } else {
                from_entity
            };

            // For nested joins, we need to find the previous join's entity to use as the table name
            // The from.entity in the current join might be referencing the previous join's alias
            let actual_from_entity = {
                let joins = request_body.get_joins();
                let mut found_entity = from_entity;
                let mut current_join_index = None;

                // Find the current join index first
                for (i, j) in joins.iter().enumerate() {
                    if std::ptr::eq(j, join) {
                        current_join_index = Some(i);
                        break;
                    }
                }

                if let Some(current_index) = current_join_index {
                    // Look at the previous join specifically
                    if current_index > 0 {
                        let prev_join = &joins[current_index - 1];

                        // Special handling for self joins
                        if prev_join.r#type == "self" {
                            // For self joins, we need to look at the previous join's to.entity
                            // The from.entity in current join references the alias from the self join
                            found_entity = prev_join.field_relation.to.entity.as_str();
                        } else {
                            // Check if our from.entity matches a previous join's to.alias
                            if prev_join.field_relation.to.alias.as_ref()
                                == Some(&join.field_relation.from.entity)
                            {
                                found_entity = prev_join.field_relation.to.entity.as_str();
                            }
                            // Check if our from.entity matches a previous join's to.entity
                            else if prev_join.field_relation.to.entity
                                == join.field_relation.from.entity
                            {
                                found_entity = prev_join.field_relation.to.entity.as_str();
                            }
                        }
                    }
                }
                found_entity
            };

            return format!(
                "{} JOIN LATERAL (SELECT {} FROM \"{}\" \"{}\" WHERE {} AND \"{}\".\"{}\" = (SELECT DISTINCT \"{}\" FROM \"{}\" WHERE \"{}\" = \"{}\".\"{}\" LIMIT 1)) AS \"{}\" ON TRUE",
                join_kind,
                selected_fields,
                to_entity, lateral_alias,
                combined_where,
                lateral_alias, to_field, from_field, actual_from_entity, from_field, nested_join_alias, from_field,
                to_alias
            );
        }

        // Determine the correct from table reference for the join condition
        // For self joins, always reference the main table (cannot reference the join alias inside its own lateral subquery)
        // For other joins, prioritize from.alias over from.entity when creating SQL alias for selections
        let from_table_ref = if is_self_join {
            table
        } else if let Some(alias) = &join.field_relation.from.alias {
            // Prioritize from.alias over from.entity for selections
            alias.as_str()
        } else if is_nested {
            // For nested joins, we need to look at the previous join's alias
            // Find the index of the current join to get the previous one
            let joins = request_body.get_joins();
            if let Some(current_index) = joins.iter().position(|j| std::ptr::eq(j, join)) {
                if current_index > 0 {
                    let prev_join = &joins[current_index - 1];
                    prev_join
                        .field_relation
                        .to
                        .alias
                        .as_deref()
                        .unwrap_or(prev_join.field_relation.to.entity.as_str())
                } else {
                    // Fallback to the logic for non-nested joins
                    let mut found_alias = None;
                    for j in joins {
                        if let Some(to_alias) = &j.field_relation.to.alias {
                            if to_alias == from_entity {
                                found_alias = Some(to_alias.as_str());
                                break;
                            }
                        }
                    }
                    found_alias.unwrap_or(from_entity)
                }
            } else {
                // Fallback to the logic for non-nested joins
                let mut found_alias = None;
                for j in joins {
                    if let Some(to_alias) = &j.field_relation.to.alias {
                        if to_alias == from_entity {
                            found_alias = Some(to_alias.as_str());
                            break;
                        }
                    }
                }
                found_alias.unwrap_or(from_entity)
            }
        } else {
            // Check if from_entity matches any alias from previous joins
            let joins = request_body.get_joins();
            let mut found_alias = None;
            for j in joins {
                if let Some(to_alias) = &j.field_relation.to.alias {
                    if to_alias == from_entity {
                        found_alias = Some(to_alias.as_str());
                        break;
                    }
                }
            }
            found_alias.unwrap_or(from_entity)
        };

        format!(
            "{} JOIN LATERAL (SELECT {} FROM \"{}\" \"{}\" WHERE {} AND \"{}\".\"{}\" = \"{}\".\"{}\" ) AS \"{}\" ON TRUE",
            join_kind,
            selected_fields,
            to_entity, lateral_alias,
            combined_where,
            lateral_alias, to_field, from_table_ref, from_field,
            to_alias
        )
    }
}
