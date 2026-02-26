use crate::structs::core::{ConcatenateField, GroupBy, SortOption};
use crate::utils::helpers::{date_format_wrapper, time_format_wrapper, timestamp_format_wrapper};

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
                            let t = Self::dequote_ident(field_parts[0]);
                            let f = Self::dequote_ident(field_parts[1]);
                            format!("\"{}\".\"{}\"", t, f)
                        } else {
                            let f = Self::dequote_ident(field);
                            format!("\"{}\".\"{}\"", self.table, f)
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
                    let (mut table_alias, mut field_name) = if field_parts.len() == 2 {
                        (field_parts[0].to_string(), field_parts[1].to_string())
                    } else {
                        (self.table.clone(), sort_option.by_field.clone())
                    };
                    // Support inputs like "\"table\".\"column\"" by dequoting
                    table_alias = Self::dequote_ident(&table_alias);
                    field_name = Self::dequote_ident(&field_name);

                    let field_expression =
                        self.get_field_expression_for_sort(&table_alias, &field_name);

                    // Handle case sensitivity
                    let final_field = if sort_option.is_case_sensitive_sorting.unwrap_or(false) {
                        field_expression
                    } else {
                        format!("LOWER({})", field_expression)
                    };

                    // When GROUP BY is present, ORDER BY must be in GROUP BY or wrapped in an aggregate
                    let group_by = self.request_body.get_group_by();
                    let field_in_group_by = group_by.map_or(false, |g| {
                        !g.fields.is_empty()
                            && g.fields.iter().any(|group_field| {
                                let group_parts: Vec<&str> =
                                    group_field.trim().split('.').collect();
                                let group_table_name = Self::dequote_ident(
                                    &self.normalize_entity_name(group_parts[0]),
                                );
                                let table_alias = Self::dequote_ident(&table_alias);
                                if group_parts.len() > 1 {
                                    let g_field = Self::dequote_ident(group_parts[1]);
                                    g_field == field_name && group_table_name == table_alias
                                } else {
                                    let g_field = Self::dequote_ident(group_parts[0]);
                                    g_field == field_name
                                }
                            })
                    });

                    let nulls_clause = if sort_option.by_direction.eq_ignore_ascii_case("asc")
                        || sort_option.by_direction.eq_ignore_ascii_case("ascending")
                    {
                        "NULLS FIRST"
                    } else {
                        "NULLS LAST"
                    };
                    if field_in_group_by {
                        format!(
                            "{} {} {}",
                            final_field,
                            sort_option.by_direction.to_uppercase(),
                            nulls_clause
                        )
                    } else if group_by.map_or(false, |g| !g.fields.is_empty()) {
                        let agg = if sort_option.by_direction.eq_ignore_ascii_case("ASC") {
                            "MIN"
                        } else {
                            "MAX"
                        };
                        format!(
                            "{}({}) {} {}",
                            agg,
                            final_field,
                            sort_option.by_direction.to_uppercase(),
                            nulls_clause
                        )
                    } else {
                        format!(
                            "{} {} {}",
                            final_field,
                            sort_option.by_direction.to_uppercase(),
                            nulls_clause
                        )
                    }
                })
                .filter(|clause| !clause.is_empty()) // Filter out empty clauses
                .collect();

            // Deduplicate identical ORDER BY clauses while preserving order
            if !sort_clauses.is_empty() {
                let mut seen = std::collections::HashSet::<String>::new();
                let mut deduped: Vec<String> = Vec::with_capacity(sort_clauses.len());
                for clause in sort_clauses {
                    let key = clause.trim().to_lowercase();
                    if seen.insert(key) {
                        deduped.push(clause);
                    }
                }
                if !deduped.is_empty() {
                    return format!(" ORDER BY {}", deduped.join(", "));
                }
            }

            String::from("")
        }
        // Fallback to single field sorting using trait methods
        else if !self.request_body.get_order_by().is_empty() {
            let order_by = self.request_body.get_order_by();
            let order_direction = self.request_body.get_order_direction();
            let (mut table_alias, mut field_name) = {
                let parts: Vec<&str> = order_by.split('.').collect();
                if parts.len() == 2 {
                    (parts[0].to_string(), parts[1].to_string())
                } else {
                    (self.table.clone(), order_by.to_string())
                }
            };
            table_alias = Self::dequote_ident(&table_alias);
            field_name = Self::dequote_ident(&field_name);

            let field_expression = self.get_field_expression_for_sort(&table_alias, &field_name);

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

            // When GROUP BY is present, ORDER BY columns must be in GROUP BY or wrapped in an aggregate
            let nulls_clause = if order_direction.eq_ignore_ascii_case("asc")
                || order_direction.eq_ignore_ascii_case("ascending")
            {
                "NULLS FIRST"
            } else {
                "NULLS LAST"
            };
            let order_clause = if self.order_by_field_in_group_by(order_by) {
                format!(
                    "{} {} {}",
                    final_field,
                    order_direction.to_uppercase(),
                    nulls_clause
                )
            } else if self
                .request_body
                .get_group_by()
                .map_or(false, |g| !g.fields.is_empty())
            {
                let agg = if order_direction.eq_ignore_ascii_case("ASC") {
                    "MIN"
                } else {
                    "MAX"
                };
                format!(
                    "{}({}) {} {}",
                    agg,
                    final_field,
                    order_direction.to_uppercase(),
                    nulls_clause
                )
            } else {
                format!(
                    "{} {} {}",
                    final_field,
                    order_direction.to_uppercase(),
                    nulls_clause
                )
            };

            format!(" ORDER BY {}", order_clause)
        } else {
            String::from("")
        }
    }

    /// Returns true if the order_by field is in group_by.fields (so no aggregate needed in ORDER BY).
    fn order_by_field_in_group_by(&self, order_by_field: &str) -> bool {
        let group_by = match self.request_body.get_group_by() {
            Some(g) if !g.fields.is_empty() => g,
            _ => return false,
        };
        let order_parts: Vec<&str> = order_by_field.split('.').collect();
        let (order_entity, order_field) = if order_parts.len() == 2 {
            (
                Self::dequote_ident(order_parts[0]),
                Self::dequote_ident(order_parts[1]),
            )
        } else {
            (self.table.clone(), Self::dequote_ident(order_by_field))
        };
        group_by.fields.iter().any(|gf| {
            let g_parts: Vec<&str> = gf.trim().split('.').collect();
            if g_parts.len() == 2 {
                let g_entity = Self::dequote_ident(&self.normalize_entity_name(g_parts[0]));
                let g_field = Self::dequote_ident(g_parts[1]);
                g_field == order_field && g_entity == order_entity
            } else {
                let g_field = Self::dequote_ident(g_parts[0]);
                g_field == order_field
            }
        })
    }

    /// Normalizes entity name by converting to lowercase and pluralizing
    fn normalize_entity_name(&self, entity: &str) -> String {
        entity.to_string()
    }

    /// Resolves the sort field to a SQL expression, handling concatenated datetime fields.
    /// For concatenated fields that combine *_date and *_time, uses expression::timestamp for proper sorting.
    fn get_field_expression_for_sort(&self, table_alias: &str, field_name: &str) -> String {
        let concatenate_fields = self.request_body.get_concatenate_fields();
        let normalized_entity = if table_alias == "self" {
            self.table.as_str()
        } else {
            table_alias
        };

        if let Some(concat_field) = concatenate_fields.iter().find(|cf| {
            cf.field_name == field_name
                && (cf.entity == table_alias
                    || cf.entity == normalized_entity
                    || cf
                        .aliased_entity
                        .as_deref()
                        .map_or(false, |a| a == table_alias || a == normalized_entity))
        }) {
            let sort_expr = concat_field.to_group_by_expression(normalized_entity);
            // If concatenated field combines _date and _time, cast to timestamp for proper datetime sorting
            let is_concatenated_datetime = concat_field.fields.iter().any(|f| f.ends_with("_date"))
                && concat_field.fields.iter().any(|f| f.ends_with("_time"));
            if is_concatenated_datetime {
                format!("({})::timestamp", sort_expr)
            } else {
                sort_expr
            }
        } else {
            Self::get_field(
                table_alias,
                field_name,
                self.request_body.get_date_format(),
                &self.table,
                self.timezone.as_deref(),
                false,
                self.time_format.as_str(),
            )
        }
    }

    fn dequote_ident(s: &str) -> String {
        s.replace('"', "")
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
        let base_field = format!("\"{}\".\"{}\"", table, field);

        let formatted_field = if field.eq_ignore_ascii_case("timestamp") {
            timestamp_format_wrapper(table, field, format_str, time_format, timezone, with_alias)
        } else if format_str.contains("%Y")
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
                format!("{} AS \"{}\"", base_field, field)
            } else {
                base_field.clone()
            }
        };

        if let Some(cast_type) = parse_as {
            if with_alias {
                format!("CAST({} AS {}) AS \"{}\"", &base_field, cast_type, field)
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

    /// Returns concatenate_fields for resolving concatenated datetime sort fields.
    /// Default returns empty; QueryFilter implementors override to provide the real value.
    fn get_concatenate_fields(&self) -> &[ConcatenateField] {
        &[]
    }
}
