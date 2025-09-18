use crate::structs::core::{GroupBy, SortOption};
use crate::utils::helpers::{date_format_wrapper, time_format_wrapper};

pub struct OrderByConstructor<T> {
    pub request_body: T,
    pub table: String,
    pub timezone: Option<String>,
    pub time_format: String,
}

impl<T> OrderByConstructor<T>
where
    T: OrderByQueryFilter,
{
    pub fn new(
        request_body: T,
        table: String,
        timezone: Option<String>,
        time_format: String,
    ) -> Self {
        Self {
            request_body,
            table,
            timezone,
            time_format,
        }
    }

    /// Constructs the ORDER BY clause for SQL queries
    pub fn construct_order_by(&self) -> String {
        if let Some(distinct_by) = self.request_body.get_distinct_by() {
            if !distinct_by.is_empty() {
                let distinct_fields: Vec<String> = distinct_by
                    .split(',')
                    .map(|field| {
                        let field = field.trim();
                        let field_parts: Vec<&str> = field.split('.').collect();
                        if field_parts.len() == 2 {
                            format!("{}.{}", field_parts[0], field_parts[1])
                        } else {
                            format!("{}.{}", self.table, field)
                        }
                    })
                    .collect();
                return format!(" ORDER BY {}", distinct_fields.join(", "));
            }
        }
        self.get_proper_order(&self.table)
    }

    /// Gets the proper ORDER BY clause based on multiple_sort or single field sorting
    fn get_proper_order(&self, _table: &str) -> String {
        // Check if multiple_sort is available and not empty
        if !self.request_body.get_multiple_sort().is_empty() {
            let sort_clauses: Vec<String> = self
                .request_body
                .get_multiple_sort()
                .iter()
                .map(|sort_option| {
                    let field_parts: Vec<&str> = sort_option.by_field.split('.').collect();
                    let (table_alias, field_name) = if field_parts.len() == 2 {
                        (field_parts[0], field_parts[1])
                    } else {
                        (self.table.as_str(), sort_option.by_field.as_str())
                    };

                    let field_expression = Self::get_field(
                        table_alias,
                        field_name,
                        self.request_body.get_date_format(),
                        &self.table,
                        self.timezone.as_deref(),
                        false,
                        self.time_format.as_str(),
                    );

                    // Handle case sensitivity
                    let final_field = if sort_option.is_case_sensitive_sorting.unwrap_or(false) {
                        field_expression
                    } else {
                        format!("LOWER({})", field_expression)
                    };

                    // Check if field exists in group_by and use proper formatting
                    if let Some(group_by) = &self.request_body.get_group_by() {
                        let field_in_group_by = group_by.fields.iter().any(|group_field| {
                            let group_parts: Vec<&str> = group_field.trim().split('.').collect();
                            let group_table_name = self.normalize_entity_name(group_parts[0]);

                            if group_parts.len() > 1 {
                                // Handle entity.field format in group_by
                                group_parts[1] == field_name
                                    && (group_table_name == table_alias
                                        || group_parts[0] == table_alias)
                            } else {
                                // Handle single field format in group_by
                                group_parts[0] == field_name
                            }
                        });
                        // If field is in group_by, use the field expression without direction
                        if field_in_group_by {
                            return final_field;
                        }
                    }

                    format!(
                        "{} {}",
                        final_field,
                        sort_option.by_direction.to_uppercase()
                    )
                })
                .filter(|clause| !clause.is_empty()) // Filter out empty clauses
                .collect();

            if !sort_clauses.is_empty() {
                return format!(" ORDER BY {}", sort_clauses.join(", "));
            }

            String::from("")
        }
        // Fallback to single field sorting using trait methods
        else if !self.request_body.get_order_by().is_empty() {
            let order_by = self.request_body.get_order_by();
            let order_direction = self.request_body.get_order_direction();

            let field_expression = Self::get_field(
                _table,
                order_by,
                self.request_body.get_date_format(),
                self.table.as_str(),
                self.timezone.as_deref(),
                false,
                self.time_format.as_str(),
            );

            // Handle case sensitivity
            let final_field = if self
                .request_body
                .get_is_case_sensitive_sorting()
                .unwrap_or(false)
            {
                field_expression
            } else {
                format!("LOWER({})", field_expression)
            };

            format!(
                " ORDER BY {} {}",
                final_field,
                order_direction.to_uppercase()
            )
        } else {
            String::from("")
        }
    }

    /// Normalizes entity name by converting to lowercase and pluralizing
    fn normalize_entity_name(&self, entity: &str) -> String {
        entity.to_string()
    }

    /// Gets field with proper formatting and date/time handling
    pub fn get_field(
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
            None,
            main_table,
            timezone,
            with_alias,
            time_format,
        )
    }

    /// Gets field with parse_as option for type casting
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
        let base_field = format!("{}.{}", table, field);

        let formatted_field = if format_str.contains("%Y")
            || format_str.contains("%m")
            || format_str.contains("%d")
        {
            Self::date_format_wrapper(table, field, Some(format_str), timezone, with_alias)
        } else if format_str.contains("%H")
            || format_str.contains("%i")
            || format_str.contains("%s")
        {
            Self::time_format_wrapper(table, field, timezone, main_table, with_alias, time_format)
        } else {
            if with_alias {
                format!("{} AS {}", base_field, field)
            } else {
                base_field.clone()
            }
        };

        if let Some(cast_type) = parse_as {
            if with_alias {
                format!("CAST({} AS {}) AS {}", &base_field, cast_type, field)
            } else {
                format!("CAST({} AS {})", formatted_field, cast_type)
            }
        } else {
            formatted_field
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

/// Trait defining the required methods for ORDER BY construction
pub trait OrderByQueryFilter {
    fn get_multiple_sort(&self) -> &[SortOption];
    fn get_order_by(&self) -> &str;
    fn get_order_direction(&self) -> &str;
    fn get_is_case_sensitive_sorting(&self) -> Option<bool>;
    fn get_group_by(&self) -> Option<&GroupBy>;
    fn get_distinct_by(&self) -> Option<&str>;
    fn get_date_format(&self) -> &str;
}
