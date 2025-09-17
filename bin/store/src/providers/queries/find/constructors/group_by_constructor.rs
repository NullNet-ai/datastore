use crate::database::schema::hypertables::is_hypertable;
use crate::structs::core::{ConcatenateField, GroupBy, Join};
use crate::utils::helpers::{date_format_wrapper, time_format_wrapper};
use std::collections::HashMap;

pub struct GroupByConstructor<'a> {
    pub table: &'a str,
    pub timezone: Option<&'a str>,
    pub date_format: &'a str,
    pub time_format: &'a str,
}

impl<'a> GroupByConstructor<'a> {
    pub fn new(
        table: &'a str,
        timezone: Option<&'a str>,
        date_format: &'a str,
        time_format: &'a str,
    ) -> Self {
        Self {
            table,
            timezone,
            date_format,
            time_format,
        }
    }

    pub fn construct_group_by(
        &self,
        group_by: Option<&GroupBy>,
        pluck: &[String],
        pluck_object: &HashMap<String, Vec<String>>,
        pluck_group_object: &HashMap<String, Vec<String>>,
        concatenate_fields: &[ConcatenateField],
        joins: &[Join],
    ) -> String {
        if let Some(group_by) = group_by {
            if !group_by.fields.is_empty() {
                // Get all fields from the group_by fields and create GROUP BY clause
                let mut group_fields: Vec<String> = group_by
                    .fields
                    .iter()
                    .map(|field| {
                        let parts: Vec<&str> = field.trim().split('.').collect();
                        if parts.len() == 2 {
                            // Handle entity.field pattern like "contacts.code"
                            let entity = parts[0]; // index 0 is the entity
                            let field_name = parts[1]; // index 1 is the actual field
                            let normalized_entity = self.normalize_entity_name(entity);

                            // Use the normalized entity as the table reference
                            Self::get_field(
                                &normalized_entity,
                                field_name,
                                self.date_format,
                                self.table,
                                self.timezone,
                                false, // GROUP BY cannot have aliases
                                self.time_format,
                            )
                        } else {
                            // Handle single field without entity prefix
                            Self::get_field(
                                self.table,
                                field,
                                self.date_format,
                                self.table,
                                self.timezone,
                                false, // GROUP BY cannot have aliases
                                self.time_format,
                            )
                        }
                    })
                    .collect();

                if is_hypertable(self.table) {
                    group_fields.push("timestamp".to_string());
                }
                return format!(" GROUP BY {}", group_fields.join(", "));
            } else if group_by.has_count {
                // When has_count is true but no specific fields are provided,
                // we need to group by all non-aggregated columns to satisfy PostgreSQL requirements
                let mut group_fields: Vec<String> = Vec::new();

                // Add main table ID field
                group_fields.push(format!("{}.id", self.table));

                // Add pluck fields from main table
                for field in pluck {
                    if field != "id" {
                        // Avoid duplicating id
                        group_fields.push(Self::get_field(
                            self.table,
                            field,
                            self.date_format,
                            self.table,
                            self.timezone,
                            false,
                            self.time_format,
                        ));
                    }
                }

                // Add pluck_object fields
                for (entity, fields) in pluck_object {
                    let normalized_entity = self.normalize_entity_name(entity);
                    for field in fields {
                        if field != "id" || entity != self.table {
                            // Avoid duplicating main table id
                            group_fields.push(Self::get_field(
                                &normalized_entity,
                                field,
                                self.date_format,
                                self.table,
                                self.timezone,
                                false,
                                self.time_format,
                            ));
                        }
                    }
                }

                // Add pluck_group_object fields
                for (entity, fields) in pluck_group_object {
                    let normalized_entity = self.normalize_entity_name(entity);
                    for field in fields {
                        if field != "id" || entity != self.table {
                            // Avoid duplicating main table id
                            group_fields.push(Self::get_field(
                                &normalized_entity,
                                field,
                                self.date_format,
                                self.table,
                                self.timezone,
                                false,
                                self.time_format,
                            ));
                        }
                    }
                }

                // Add concatenated fields - these are computed fields that appear in SELECT
                // and need to be included in GROUP BY when using aggregates
                for concat_field in concatenate_fields {
                    let entity_name = if let Some(aliased_entity) = &concat_field.aliased_entity {
                        // For alias entities, use the original alias name without normalization
                        aliased_entity.clone()
                    } else {
                        // Check if this entity has a corresponding JOIN with an alias
                        let join_alias = joins
                            .iter()
                            .find(|join| {
                                // Match by entity name or normalized entity name
                                join.field_relation.to.entity == concat_field.entity
                                    || self.normalize_entity_name(&join.field_relation.to.entity)
                                        == concat_field.entity
                                    // Also match by alias if it exists
                                    || join
                                        .field_relation
                                        .to
                                        .alias
                                        .as_ref()
                                        .map_or(false, |alias| alias == &concat_field.entity)
                            })
                            .and_then(|join| join.field_relation.to.alias.as_ref())
                            .cloned();

                        join_alias
                            .unwrap_or_else(|| self.normalize_entity_name(&concat_field.entity))
                    };

                    // Add the individual fields that make up the concatenated field
                    for field in &concat_field.fields {
                        group_fields.push(Self::get_field(
                            &entity_name,
                            field,
                            self.date_format,
                            self.table,
                            self.timezone,
                            false,
                            self.time_format,
                        ));
                    }
                }

                if is_hypertable(self.table) {
                    group_fields.push("timestamp".to_string());
                }

                if !group_fields.is_empty() {
                    return format!(" GROUP BY {}", group_fields.join(", "));
                }
            }
        }
        String::from("")
    }

    fn normalize_entity_name(&self, entity: &str) -> String {
        if entity == "self" {
            self.table.to_string()
        } else {
            entity.to_string()
        }
    }

    fn get_field(
        table: &str,
        field: &str,
        format_str: &str,
        main_table: &str,
        timezone: Option<&str>,
        with_alias: bool,
        time_format: &str,
    ) -> String {
        Self::get_field_with_parse_as(
            table,
            field,
            format_str,
            None, // No parse_as for GROUP BY fields
            main_table,
            timezone,
            with_alias,
            time_format,
        )
    }

    fn get_field_with_parse_as(
        table: &str,
        field: &str,
        format_str: &str,
        parse_as: Option<&str>,
        main_table: &str,
        timezone: Option<&str>,
        with_alias: bool,
        time_format: &str,
    ) -> String {
        match parse_as {
            Some("date") => {
                Self::date_format_wrapper(table, field, Some(format_str), timezone, with_alias)
            }
            Some("time") => Self::time_format_wrapper(
                table,
                field,
                timezone,
                main_table,
                with_alias,
                time_format,
            ),
            Some("text") => {
                let field_expr = format!("\"{}\".\"{}\"::text", table, field);
                if with_alias {
                    format!("{} AS {}", field_expr, field)
                } else {
                    field_expr
                }
            }
            _ => {
                let field_expr = format!("\"{}\".\"{}\"", table, field);
                if with_alias {
                    format!("{} AS {}", field_expr, field)
                } else {
                    field_expr
                }
            }
        }
    }

    fn date_format_wrapper(
        table: &str,
        field: &str,
        format_str: Option<&str>,
        timezone: Option<&str>,
        with_alias: bool,
    ) -> String {
        date_format_wrapper(table, field, format_str, timezone, with_alias)
    }

    fn time_format_wrapper(
        table: &str,
        field: &str,
        timezone: Option<&str>,
        main_table: &str,
        with_alias: bool,
        time_format: &str,
    ) -> String {
        time_format_wrapper(table, field, timezone, main_table, with_alias, time_format)
    }
}
