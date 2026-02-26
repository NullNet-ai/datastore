use crate::database::schema::hypertables::is_hypertable;
use crate::structs::core::{ConcatenateField, GroupBy, Join};
use crate::utils::helpers::{date_format_wrapper, time_format_wrapper, timestamp_format_wrapper};
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

    /// Resolve a concatenated field to its GROUP BY expression if it exists.
    fn get_concatenated_group_by_expression(
        concatenate_fields: &[ConcatenateField],
        entity: &str,
        field_name: &str,
        main_table: &str,
    ) -> Option<String> {
        let normalized_entity = if entity == "self" {
            main_table.to_string()
        } else {
            entity.to_string()
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
            .map(|cf| cf.to_group_by_expression(&normalized_entity))
    }

    pub fn construct_group_by(
        &self,
        group_by: Option<&GroupBy>,
        _pluck: &[String],
        _pluck_object: &HashMap<String, Vec<String>>,
        _pluck_group_object: &HashMap<String, Vec<String>>,
        concatenate_fields: &[ConcatenateField],
        _joins: &[Join],
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
                            // Handle entity.field pattern (e.g. "contacts.code" or "samples.full_name" concatenated)
                            let entity = parts[0];
                            let field_name = parts[1];
                            let normalized_entity = self.normalize_entity_name(entity);

                            if let Some(expr) = Self::get_concatenated_group_by_expression(
                                concatenate_fields,
                                entity,
                                field_name,
                                self.table,
                            ) {
                                return expr;
                            }
                            Self::get_field(
                                &normalized_entity,
                                field_name,
                                self.date_format,
                                self.table,
                                self.timezone,
                                false,
                                self.time_format,
                            )
                        } else {
                            // Handle single field without entity prefix (defaults to main table)
                            let field_name = parts[0];
                            if let Some(expr) = Self::get_concatenated_group_by_expression(
                                concatenate_fields,
                                self.table,
                                field_name,
                                self.table,
                            ) {
                                return expr;
                            }
                            Self::get_field(
                                self.table,
                                field,
                                self.date_format,
                                self.table,
                                self.timezone,
                                false,
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
                // default to grouping by main table id only (e.g. GROUP BY "samples"."id").
                let mut group_fields: Vec<String> = vec![format!("\"{}\".\"id\"", self.table)];
                if is_hypertable(self.table) {
                    group_fields.push("timestamp".to_string());
                }
                return format!(" GROUP BY {}", group_fields.join(", "));
            }
        }

        let mut group_fields: Vec<String> = vec![format!("\"{}\".\"id\"", self.table)];
        if is_hypertable(self.table) {
            group_fields.push("timestamp".to_string());
        }
        format!(" GROUP BY {}", group_fields.join(", "))
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
            Some("timestamp") => timestamp_format_wrapper(
                table,
                field,
                format_str,
                time_format,
                timezone,
                with_alias,
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
                let field_expr = if field.ends_with("_date") {
                    Self::date_format_wrapper(table, field, Some(format_str), timezone, with_alias)
                } else if field.ends_with("_time") {
                    Self::time_format_wrapper(
                        table,
                        field,
                        timezone,
                        main_table,
                        with_alias,
                        time_format,
                    )
                } else if field.eq_ignore_ascii_case("timestamp") {
                    timestamp_format_wrapper(
                        table,
                        field,
                        format_str,
                        time_format,
                        timezone,
                        with_alias,
                    )
                } else {
                    let table_field = format!("\"{}\".\"{}\"", table, field);
                    if with_alias {
                        format!("{} AS {}", table_field, field)
                    } else {
                        table_field
                    }
                };
                field_expr
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
