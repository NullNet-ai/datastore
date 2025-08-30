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

            for join in request_body.get_joins() {
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
            build_system_where_clause,
            build_infix_expression,
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
            build_system_where_clause,
            build_infix_expression,
        )
    }

    /// Builds a JOIN LATERAL clause (generic implementation)
    fn build_join_lateral<T: QueryFilter>(
        request_body: &T,
        table: &str,
        join: &Join,
        is_self_join: bool,
        build_system_where_clause: &impl Fn(&str) -> Result<String, String>,
        build_infix_expression: &impl Fn(&[FilterCriteria]) -> Result<String, String>,
    ) -> String {
        let to_entity = if is_self_join {
            table
        } else {
            &join.field_relation.to.entity
        };

        let to_alias = if is_self_join {
            join.field_relation.to.alias.as_deref().unwrap_or(table)
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
            .unwrap_or_else(|_| format!("({}.tombstone = 0)", lateral_alias))];

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
            return format!(
                "LEFT JOIN LATERAL (SELECT {} FROM \"{}\" \"{}\" WHERE {} AND \"{}\".\"{}\" = \"{}\".\"{}\" ) AS \"{}\" ON TRUE",
                selected_fields,
                to_entity, lateral_alias,
                combined_where,
                lateral_alias, to_field, from_entity, from_field,
                to_alias
            );
        }

        // Determine the correct from table reference for the join condition
        let from_table_ref = if let Some(alias) = &join.field_relation.from.alias {
            alias.as_str()
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
            found_alias.unwrap_or(if is_self_join { table } else { from_entity })
        };

        format!(
            "LEFT JOIN LATERAL (SELECT {} FROM \"{}\" \"{}\" WHERE {} AND \"{}\".\"{}\" = \"{}\".\"{}\" ) AS \"{}\" ON TRUE",
            selected_fields,
            to_entity, lateral_alias,
            combined_where,
            lateral_alias, to_field, from_table_ref, from_field,
            to_alias
        )
    }
}
