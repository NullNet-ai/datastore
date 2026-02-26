use crate::utils::helpers::{date_format_wrapper, time_format_wrapper, timestamp_format_wrapper};
use crate::{
    providers::queries::find::constructors::{
        group_by_constructor::GroupByConstructor,
        joins_constructor::JoinsConstructor,
        limit_constructor::{LimitConstructor, LimitQueryFilter},
        offset_constructor::{OffsetConstructor, OffsetQueryFilter},
        order_by_constructor::{OrderByConstructor, OrderByQueryFilter},
        selections_constructor::SelectionsConstructor,
        where_constructor::WhereConstructor,
    },
    structs::core::{
        ConcatenateField, FilterCriteria, FilterOperator, GetByFilter, GroupAdvanceFilter, GroupBy,
        Join, LogicalOperator, MatchPattern, SortOption,
    },
};
use serde_json::Value;

use crate::config::core::EnvConfig;
use std::collections::HashMap;
// Trait to define common interface for both GetByFilter and AggregationFilter
pub trait QueryFilter {
    fn get_advance_filters(&self) -> &[FilterCriteria];
    fn get_joins(&self) -> &[Join];
    fn get_limit(&self) -> usize;
    fn get_date_format(&self) -> &str;
    fn get_time_format(&self) -> &str;

    // Optional methods with default implementations
    fn get_pluck(&self) -> &[String] {
        &[]
    }
    fn get_pluck_object(&self) -> &HashMap<String, Vec<String>> {
        use std::collections::HashMap;
        static EMPTY: std::sync::LazyLock<HashMap<String, Vec<String>>> =
            std::sync::LazyLock::new(|| HashMap::new());
        &EMPTY
    }
    fn get_pluck_group_object(&self) -> &HashMap<String, Vec<String>> {
        use std::collections::HashMap;
        static EMPTY: std::sync::LazyLock<HashMap<String, Vec<String>>> =
            std::sync::LazyLock::new(|| HashMap::new());
        &EMPTY
    }
    fn get_group_advance_filters(&self) -> &[crate::structs::core::GroupAdvanceFilter] {
        &[]
    }
    fn get_concatenate_fields(&self) -> &[ConcatenateField] {
        &[]
    }
    fn get_order_by(&self) -> &str {
        "id"
    }
    fn get_order_direction(&self) -> &str {
        "asc"
    }
    fn get_offset(&self) -> usize {
        0
    }
    fn get_multiple_sort(&self) -> &[SortOption] {
        &[]
    }
    fn get_group_by(&self) -> Option<&GroupBy> {
        None
    }
    fn get_distinct_by(&self) -> Option<&str> {
        None
    }
    fn get_is_case_sensitive_sorting(&self) -> Option<bool> {
        None
    }
    fn get_timezone(&self) -> Option<&str> {
        Some("Asia/Manila")
    }
}

// Implement QueryFilter for GetByFilter
// Implement the required traits for QueryFilter
impl<T: QueryFilter> LimitQueryFilter for T {
    fn get_limit(&self) -> usize {
        QueryFilter::get_limit(self)
    }
}

impl<T: QueryFilter> OffsetQueryFilter for T {
    fn get_offset(&self) -> usize {
        QueryFilter::get_offset(self)
    }
}

impl<T: QueryFilter> OrderByQueryFilter for T {
    fn get_multiple_sort(&self) -> &[SortOption] {
        QueryFilter::get_multiple_sort(self)
    }

    fn get_order_by(&self) -> &str {
        QueryFilter::get_order_by(self)
    }

    fn get_order_direction(&self) -> &str {
        QueryFilter::get_order_direction(self)
    }

    fn get_is_case_sensitive_sorting(&self) -> Option<bool> {
        QueryFilter::get_is_case_sensitive_sorting(self)
    }

    fn get_group_by(&self) -> Option<&GroupBy> {
        QueryFilter::get_group_by(self)
    }

    fn get_distinct_by(&self) -> Option<&str> {
        QueryFilter::get_distinct_by(self)
    }

    fn get_date_format(&self) -> &str {
        QueryFilter::get_date_format(self)
    }

    fn get_concatenate_fields(&self) -> &[ConcatenateField] {
        QueryFilter::get_concatenate_fields(self)
    }
}

impl QueryFilter for GetByFilter {
    fn get_advance_filters(&self) -> &[FilterCriteria] {
        &self.advance_filters
    }

    fn get_joins(&self) -> &[Join] {
        &self.joins
    }

    fn get_limit(&self) -> usize {
        self.limit
    }

    fn get_date_format(&self) -> &str {
        &self.date_format
    }

    fn get_pluck(&self) -> &[String] {
        &self.pluck
    }

    fn get_pluck_object(&self) -> &HashMap<String, Vec<String>> {
        &self.pluck_object
    }

    fn get_pluck_group_object(&self) -> &HashMap<String, Vec<String>> {
        &self.pluck_group_object
    }

    fn get_group_advance_filters(&self) -> &[crate::structs::core::GroupAdvanceFilter] {
        &self.group_advance_filters
    }

    fn get_concatenate_fields(&self) -> &[ConcatenateField] {
        &self.concatenate_fields
    }

    fn get_order_by(&self) -> &str {
        &self.order_by
    }

    fn get_order_direction(&self) -> &str {
        &self.order_direction
    }

    fn get_offset(&self) -> usize {
        self.offset
    }

    fn get_multiple_sort(&self) -> &[SortOption] {
        &self.multiple_sort
    }

    fn get_group_by(&self) -> Option<&GroupBy> {
        self.group_by.as_ref()
    }

    fn get_is_case_sensitive_sorting(&self) -> Option<bool> {
        self.is_case_sensitive_sorting
    }

    fn get_distinct_by(&self) -> Option<&str> {
        self.distinct_by.as_deref()
    }

    fn get_timezone(&self) -> Option<&str> {
        self.timezone.as_deref()
    }

    fn get_time_format(&self) -> &str {
        &self.time_format
    }
}

#[derive(Debug, Clone)]
enum Token {
    Condition(String),
    And,
    Or,
}

pub struct SQLConstructor<T: QueryFilter + Clone> {
    pub request_body: T,
    pub table: String,
    pub organization_id: Option<String>,
    pub is_root: bool,
    pub timezone: Option<String>,
}
// TODO - remove this after testing
#[allow(warnings)]
impl<T: QueryFilter + Clone> SQLConstructor<T> {
    pub fn new(request_body: T, table: String, is_root: bool, timezone: Option<String>) -> Self {
        Self {
            request_body,
            table,
            organization_id: None,
            is_root,
            timezone,
        }
    }

    /// Helper function to convert entity names from singular to plural form
    /// If the entity is already plural, adds 's' to it
    fn normalize_entity_name(&self, entity: &str) -> String {
        entity.to_string()
    }

    pub fn with_organization_id(mut self, organization_id: String) -> Self {
        self.organization_id = Some(organization_id);
        self
    }

    pub fn construct(&mut self) -> Result<String, String> {
        let body_timezone = self.request_body.get_timezone();
        // Prefer body timezone over header for consistency across find, count, aggregation, and search suggestion
        let timezone = match (self.timezone.as_deref(), body_timezone) {
            (_, Some(tz)) => Some(tz.to_string()), // Body takes precedence
            (Some(tz), None) => Some(tz.to_string()), // Header fallback
            (None, None) => None,
        };

        self.timezone = timezone;
        let time_format = self.request_body.get_time_format();

        let mut sql = String::from("SELECT ");

        // Use the new constructor modules with proper parameters
        sql.push_str(&SelectionsConstructor::construct_selections(
            &self.request_body,
            &self.table,
            self.timezone.as_deref(),
            |entity| self.normalize_entity_name(entity),
            |table, field, format_str, main_table, timezone, with_alias| {
                Self::get_field(
                    table,
                    field,
                    format_str,
                    main_table,
                    timezone,
                    with_alias,
                    time_format,
                    None,
                )
            },
            |table, field, format_str, parse_as, main_table, timezone, with_alias| {
                Self::get_field_with_parse_as(
                    table,
                    field,
                    format_str,
                    parse_as,
                    main_table,
                    timezone,
                    with_alias,
                    time_format,
                )
            },
            |table_alias| self.build_system_where_clause(table_alias),
            |filters| self.build_infix_expression(filters),
        ));

        let ids_selections = self.construct_pluck_group_ids_selections();
        if !ids_selections.is_empty() {
            if !sql.ends_with("SELECT ") {
                sql.push_str(", ");
            }
            sql.push_str(&ids_selections.join(", "));
        }

        sql.push_str(&format!(" FROM \"{}\" \"{}\"", self.table, self.table));

        sql.push_str(&JoinsConstructor::construct_joins(
            &self.request_body,
            &self.table,
            |table_alias| self.build_system_where_clause(table_alias),
            |filters| self.build_infix_expression(filters),
        ));

        let where_constructor = WhereConstructor::new(
            &self.table,
            self.organization_id.as_deref(),
            self.is_root,
            self.timezone.as_deref(),
        );
        sql.push_str(&where_constructor.construct_where_clauses(
            self.request_body.get_advance_filters(),
            self.request_body.get_group_advance_filters(),
            self.request_body.get_concatenate_fields(),
            self.request_body.get_date_format(),
            self.request_body.get_time_format(),
        )?);

        let group_by_constructor = GroupByConstructor::new(
            &self.table,
            self.timezone.as_deref(),
            self.request_body.get_date_format(),
            self.request_body.get_time_format(),
        );

        sql.push_str(&group_by_constructor.construct_group_by(
            self.request_body.get_group_by(),
            self.request_body.get_pluck(),
            self.request_body.get_pluck_object(),
            self.request_body.get_pluck_group_object(),
            self.request_body.get_concatenate_fields(),
            self.request_body.get_joins(),
        ));

        let order_by_constructor = OrderByConstructor::new(
            self.request_body.clone(),
            self.table.clone(),
            self.timezone.clone(),
            self.request_body.get_time_format().to_string(),
        );
        sql.push_str(&order_by_constructor.construct_order_by());

        let offset_constructor = OffsetConstructor::new(self.request_body.clone());
        sql.push_str(&offset_constructor.construct_offset());

        let limit_constructor = LimitConstructor::new(self.request_body.clone());
        sql.push_str(&limit_constructor.construct_limit());
        Ok(sql)
    }

    fn construct_pluck_group_ids_selections(&self) -> Vec<String> {
        let mut selections = Vec::new();
        let pluck_group_object = self.request_body.get_pluck_group_object();
        if pluck_group_object.is_empty() {
            return selections;
        }
        let joins = self.request_body.get_joins();
        for (alias_key, _fields) in pluck_group_object.iter() {
            if let Some((idx, join)) = self.find_join_by_alias(alias_key) {
                let to_alias = join
                    .field_relation
                    .to
                    .alias
                    .as_deref()
                    .unwrap_or(&join.field_relation.to.entity);
                let target_table = &join.field_relation.to.entity;
                let previous_join = self.find_previous_join(idx, join);
                let selection = if !join.nested {
                    let mut where_conditions = Vec::new();
                    let standard_where = match self.build_system_where_clause(to_alias) {
                        Ok(clause) => clause,
                        Err(_) => format!("(\"{}\".\"tombstone\" = 0)", to_alias),
                    };
                    where_conditions.push(standard_where);
                    let join_condition =
                        self.build_join_condition_for_alias(to_alias, join, previous_join);
                    where_conditions.push(join_condition);
                    if !join.field_relation.to.filters.is_empty() {
                        if let Ok(filter_expression) =
                            self.build_infix_expression(&join.field_relation.to.filters)
                        {
                            if !filter_expression.is_empty() {
                                where_conditions.push(filter_expression);
                            }
                        }
                    }
                    let combined_where = where_conditions.join(" AND ");
                    format!(
                        "COALESCE( ( SELECT JSONB_AGG(\"{}\".\"id\") FROM \"{}\" \"{}\" WHERE {} ), '[]' ) AS \"{}_ids\"",
                        to_alias, target_table, to_alias, combined_where, to_alias
                    )
                } else if let Some(prev_join) = previous_join {
                    let (prev_alias, prev_table) = if prev_join.r#type == "self" {
                        (
                            prev_join
                                .field_relation
                                .from
                                .alias
                                .as_deref()
                                .unwrap_or(&prev_join.field_relation.from.entity),
                            prev_join.field_relation.from.entity.as_str(),
                        )
                    } else {
                        (
                            prev_join
                                .field_relation
                                .to
                                .alias
                                .as_deref()
                                .unwrap_or(&prev_join.field_relation.to.entity),
                            prev_join.field_relation.to.entity.as_str(),
                        )
                    };
                    let parent_where = match self.build_system_where_clause(prev_alias) {
                        Ok(clause) => clause,
                        Err(_) => format!("(\"{}\".\"tombstone\" = 0)", prev_alias),
                    };
                    let child_where = match self.build_system_where_clause(to_alias) {
                        Ok(clause) => clause,
                        Err(_) => format!("(\"{}\".\"tombstone\" = 0)", to_alias),
                    };
                    let main_to_parent_correlation = format!(
                        "\"{}\".\"{}\" = \"{}\".\"{}\"",
                        self.table,
                        prev_join.field_relation.from.field,
                        prev_alias,
                        prev_join.field_relation.to.field
                    );
                    let on_condition = format!(
                        "\"{}\".\"{}\" = \"{}\".\"{}\"",
                        to_alias,
                        join.field_relation.to.field,
                        prev_alias,
                        join.field_relation.from.field
                    );
                    let mut where_conditions =
                        vec![child_where, parent_where, main_to_parent_correlation];
                    if !join.field_relation.to.filters.is_empty() {
                        if let Ok(filter_expression) =
                            self.build_infix_expression(&join.field_relation.to.filters)
                        {
                            if !filter_expression.is_empty() {
                                where_conditions.push(filter_expression);
                            }
                        }
                    }
                    let combined_where = where_conditions.join(" AND ");
                    format!(
                        "COALESCE( ( SELECT JSONB_AGG(\"{}\".\"id\") FROM \"{}\" \"{}\" LEFT JOIN \"{}\" \"{}\" ON {} WHERE {} ), '[]' ) AS \"{}_ids\"",
                        to_alias, prev_table, prev_alias, target_table, to_alias, on_condition, combined_where, to_alias
                    )
                } else {
                    let mut where_conditions = Vec::new();
                    let standard_where = match self.build_system_where_clause(to_alias) {
                        Ok(clause) => clause,
                        Err(_) => format!("(\"{}\".\"tombstone\" = 0)", to_alias),
                    };
                    where_conditions.push(standard_where);
                    let join_condition = self.build_join_condition_for_alias(to_alias, join, None);
                    where_conditions.push(join_condition);
                    let combined_where = where_conditions.join(" AND ");
                    format!(
                        "COALESCE( ( SELECT JSONB_AGG(\"{}\".\"id\") FROM \"{}\" \"{}\" WHERE {} ), '[]' ) AS \"{}_ids\"",
                        to_alias, target_table, to_alias, combined_where, to_alias
                    )
                };
                selections.push(selection);
            } else {
                let _ = joins;
            }
        }
        selections
    }

    fn find_join_by_alias<'a>(&'a self, alias: &str) -> Option<(usize, &'a Join)> {
        self.request_body
            .get_joins()
            .iter()
            .enumerate()
            .find(|(_, j)| {
                j.field_relation
                    .to
                    .alias
                    .as_deref()
                    .map(|a| a == alias)
                    .unwrap_or_else(|| j.field_relation.to.entity.as_str() == alias)
            })
    }

    fn find_previous_join<'a>(&'a self, idx: usize, join: &'a Join) -> Option<&'a Join> {
        if join.nested && idx > 0 {
            return self.request_body.get_joins().get(idx - 1);
        }
        let current_from_ref = join
            .field_relation
            .from
            .alias
            .as_deref()
            .unwrap_or(&join.field_relation.from.entity);
        self.request_body.get_joins().iter().find(|j| {
            let j_to_ref = j
                .field_relation
                .to
                .alias
                .as_deref()
                .unwrap_or(&j.field_relation.to.entity);
            j_to_ref == current_from_ref
        })
    }

    fn build_join_condition_for_alias(
        &self,
        alias: &str,
        join: &Join,
        previous_join: Option<&Join>,
    ) -> String {
        let is_nested = join.nested;
        let from_field = &join.field_relation.from.field;
        let to_field = &join.field_relation.to.field;
        if is_nested {
            if let Some(prev_join) = previous_join {
                let prev_to_alias = if prev_join.r#type == "self" {
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
                return format!(
                    "\"{}\".\"{}\" = \"{}\".\"{}\"",
                    alias, to_field, prev_to_alias, from_field
                );
            }
        }
        let from_table_ref = if let Some(from_alias) = &join.field_relation.from.alias {
            from_alias.as_str()
        } else {
            let from_entity = &join.field_relation.from.entity;
            let matching_alias = self.request_body.get_joins().iter().find_map(|j| {
                if let Some(alias) = &j.field_relation.to.alias {
                    if alias == from_entity {
                        return Some(alias.as_str());
                    }
                }
                None
            });
            matching_alias.unwrap_or_else(|| {
                if from_entity == &self.table {
                    self.table.as_str()
                } else {
                    from_entity
                }
            })
        };
        format!(
            "\"{}\".\"{}\" = \"{}\".\"{}\"",
            from_table_ref, from_field, alias, to_field
        )
    }

    /// Builds a COUNT(DISTINCT id) query using the same filter parsing as construct().
    /// Reuses WhereConstructor and JoinsConstructor; no GROUP BY, ORDER BY, OFFSET, LIMIT.
    pub fn construct_count(&mut self) -> Result<String, String> {
        let body_timezone = self.request_body.get_timezone();
        let timezone = match (self.timezone.as_deref(), body_timezone) {
            (_, Some(tz)) => Some(tz.to_string()),
            (Some(tz), None) => Some(tz.to_string()),
            (None, None) => None,
        };
        self.timezone = timezone;

        let id_expr = format!("\"{}\".\"id\"", self.table);
        let mut sql = format!("SELECT COUNT(DISTINCT {}) FROM ", id_expr);
        sql.push_str(&format!("\"{}\" \"{}\"", self.table, self.table));

        sql.push_str(&JoinsConstructor::construct_joins(
            &self.request_body,
            &self.table,
            |table_alias| self.build_system_where_clause(table_alias),
            |filters| self.build_infix_expression(filters),
        ));

        let where_constructor = WhereConstructor::new(
            &self.table,
            self.organization_id.as_deref(),
            self.is_root,
            self.timezone.as_deref(),
        );
        sql.push_str(&where_constructor.construct_where_clauses(
            self.request_body.get_advance_filters(),
            self.request_body.get_group_advance_filters(),
            self.request_body.get_concatenate_fields(),
            self.request_body.get_date_format(),
            self.request_body.get_time_format(),
        )?);

        Ok(sql)
    }

    pub fn get_field(
        table: &str,
        field: &str,
        format_str: &str,
        main_table: &str,
        timezone: Option<&str>,
        with_alias: bool,
        time_format: &str,
        parse_as: Option<&str>,
    ) -> String {
        Self::get_field_with_parse_as(
            table,
            field,
            format_str,
            parse_as,
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
                &format!("\"{}\".\"{}\"", table, field),
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

    /// Resolves parse_as for filter context. When parse_as is "text", the schema field is only
    /// cast to ::TEXT when values[0] is a string OR the operator is HasNoValue/IsNotEmpty
    /// (operators that don't use values). Otherwise, parse_as "text" is not applied.
    fn resolve_parse_as_for_filter(
        parse_as: &str,
        operator: &FilterOperator,
        values: &[Value],
    ) -> Option<String> {
        if parse_as != "text" {
            return Some(parse_as.to_string());
        }
        let with_default_parse_op = matches!(
            operator,
            FilterOperator::HasNoValue | FilterOperator::IsNotEmpty
        );
        let is_first_value_string = values.first().map_or(false, |v| v.is_string());
        if is_first_value_string || with_default_parse_op {
            Some("text".to_string())
        } else {
            None
        }
    }

    fn get_field_with_concatenation(
        &self,
        table: &str,
        field: &str,
        format_str: &str,
        parse_as: Option<&str>,
        timezone: Option<&str>,
        with_alias: bool,
        time_format: &str,
    ) -> String {
        // Check if this field is defined as a concatenated field
        for concat_field in self.request_body.get_concatenate_fields() {
            // Match by field name and entity/aliased_entity
            if concat_field.field_name == field {
                // Priority: aliased_entity takes precedence over entity
                let target_table = concat_field
                    .aliased_entity
                    .as_deref()
                    .unwrap_or(&concat_field.entity);

                // Check if the table matches (with normalization)
                let normalized_entity = self.normalize_entity_name(&concat_field.entity);
                let normalized_target_table = if concat_field.aliased_entity.is_some() {
                    target_table
                } else {
                    &normalized_entity
                };
                if target_table == table || normalized_target_table == table {
                    // Use the normalized table name for COALESCE expression generation
                    let table_for_coalesce = if concat_field.aliased_entity.is_some() {
                        target_table
                    } else {
                        &normalized_entity
                    };

                    // Generate concatenated expression
                    let concatenated_expression = concat_field
                        .fields
                        .iter()
                        .map(|f| {
                            format!(
                                "COALESCE({}, '')",
                                Self::get_field_with_parse_as(
                                    table_for_coalesce,
                                    f,
                                    format_str,
                                    None,
                                    self.table.as_str(),
                                    timezone,
                                    with_alias,
                                    time_format
                                )
                            )
                        })
                        .collect::<Vec<_>>()
                        .join(&format!(" || '{}' || ", concat_field.separator));

                    // Apply parse_as type casting if provided and not empty
                    let base_expression = format!("({})", concatenated_expression);
                    return if let Some(cast_type) = parse_as {
                        if !cast_type.is_empty() {
                            format!("{}::{}", base_expression, cast_type)
                        } else {
                            base_expression
                        }
                    } else {
                        base_expression
                    };
                }
            }
        }

        // Fall back to regular field handling if not a concatenated field
        Self::get_field_with_parse_as(
            table,
            field,
            format_str,
            parse_as,
            &self.table,
            timezone,
            with_alias,
            time_format,
        )
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
        // Convert from stored timezone to target timezone
        // PostgreSQL AT TIME ZONE converts from the specified timezone to UTC, then to local
    }

    pub fn construct_joins(&self) -> String {
        JoinsConstructor::construct_joins(
            &self.request_body,
            &self.table,
            |table_alias| self.build_system_where_clause(table_alias),
            |filters| self.build_infix_expression(filters),
        )
    }
    /// Constructs the standard WHERE clause pattern used across queries
    pub fn build_system_where_clause(&self, table_alias: &str) -> Result<String, String> {
        // For root access, only check tombstone
        if self.is_root {
            return Ok(format!("(\"{}\".\"tombstone\" = 0)", table_alias));
        }

        // For non-root access, check organization constraints
        let organization_id = match &self.organization_id {
            Some(id) => format!("'{}'", id),
            None => {
                let default_id = EnvConfig::default().default_organization_id;
                format!("'{}'", default_id)
            }
        };

        Ok(format!(
            "(\"{}\".\"tombstone\" = 0 AND \"{}\".\"organization_id\" IS NOT NULL AND \"{}\".\"organization_id\" = {})",
            table_alias, table_alias, table_alias, organization_id
        ))
    }

    pub fn construct_where_clauses(&self) -> Result<String, String> {
        let where_constructor = WhereConstructor::new(
            &self.table,
            self.organization_id.as_deref(),
            self.is_root,
            self.timezone.as_deref(),
        );
        where_constructor.construct_where_clauses(
            self.request_body.get_advance_filters(),
            self.request_body.get_group_advance_filters(),
            self.request_body.get_concatenate_fields(),
            self.request_body.get_date_format(),
            self.request_body.get_time_format(),
        )
    }

    /// Builds an infix expression with proper order of operations using infix notation
    ///
    /// Order of precedence (highest to lowest):
    /// 1. Parentheses (for grouping OR expressions)
    /// 2. AND operators (higher precedence)
    /// 3. OR operators (lower precedence)
    ///
    /// Examples:
    /// - `A AND B OR C AND D` becomes `(A AND B) OR (C AND D)`
    /// - `A OR B AND C` becomes `A OR (B AND C)`
    /// - Single conditions and pure AND chains remain unchanged
    ///
    /// The implementation ensures that:
    /// - Filter arrays follow the pattern: [criteria, operator, criteria, operator, ...]
    /// - AND operations are grouped together naturally
    /// - OR operations split the expression into separate groups with parentheses when needed
    pub fn build_infix_expression(&self, filters: &[FilterCriteria]) -> Result<String, String> {
        if filters.is_empty() {
            return Ok(String::new());
        }

        // Ensure first filter is always a criteria
        if !matches!(filters[0], FilterCriteria::Criteria { .. }) {
            return Err(
                "Invalid filter sequence: first filter must be a criteria, not an operator"
                    .to_string(),
            );
        }

        // Handle single criteria case
        if filters.len() == 1 {
            if let FilterCriteria::Criteria {
                field,
                entity,
                operator,
                values,
                case_sensitive,
                parse_as,
                match_pattern,
                ..
            } = &filters[0]
            {
                let normalized_entity = entity
                    .as_ref()
                    .map(|e| self.normalize_entity_name(e))
                    .unwrap_or_else(|| self.table.clone());
                let effective_parse_as =
                    Self::resolve_parse_as_for_filter(parse_as, operator, values);
                let field_name = self.get_field_with_concatenation(
                    &normalized_entity,
                    field,
                    self.request_body.get_date_format(),
                    effective_parse_as.as_deref(),
                    self.timezone.as_deref(),
                    false,
                    self.request_body.get_time_format(),
                );
                let final_statement = self.format_condition_with_case_sensitivity_and_pattern(
                    &field_name,
                    operator,
                    values,
                    *case_sensitive,
                    match_pattern.as_ref(),
                );
                return Ok(final_statement);
            }
            return Err("Invalid filter: single filter must be a criteria".to_string());
        }

        // Parse the filter array into tokens
        let tokens = self.parse_filter_tokens(filters)?;
        if tokens.is_empty() {
            return Err("Failed to parse filter tokens: invalid filter sequence".to_string());
        }

        // Build expression with proper precedence (AND > OR)
        Ok(self.build_expression_with_precedence(&tokens))
    }
    fn parse_filter_tokens(&self, filters: &[FilterCriteria]) -> Result<Vec<Token>, String> {
        let mut tokens = Vec::new();
        let mut i = 0;

        while i < filters.len() {
            match &filters[i] {
                FilterCriteria::Criteria {
                    field,
                    entity,
                    operator,
                    values,
                    case_sensitive,
                    parse_as,
                    match_pattern,
                    ..
                } => {
                    let normalized_entity = entity
                        .as_ref()
                        .map(|e| self.normalize_entity_name(e))
                        .unwrap_or_else(|| self.table.clone());
                    let effective_parse_as =
                        Self::resolve_parse_as_for_filter(parse_as, operator, values);
                    let field_name = self.get_field_with_concatenation(
                        &normalized_entity,
                        field,
                        self.request_body.get_date_format(),
                        effective_parse_as.as_deref(),
                        self.timezone.as_deref(),
                        false,
                        self.request_body.get_time_format(),
                    );
                    let condition = self.format_condition_with_case_sensitivity_and_pattern(
                        &field_name,
                        operator,
                        values,
                        *case_sensitive,
                        match_pattern.as_ref(),
                    );
                    tokens.push(Token::Condition(condition));
                    i += 1;
                }
                FilterCriteria::LogicalOperator { operator } => {
                    match operator {
                        LogicalOperator::And => tokens.push(Token::And),
                        LogicalOperator::Or => tokens.push(Token::Or),
                    }
                    i += 1;
                }
            }
        }

        // Validate token sequence (should be: condition, operator, condition, operator, ...)
        if !self.is_valid_token_sequence(&tokens) {
            return Err("Invalid filter sequence: filters must follow the pattern [criteria, operator, criteria, operator, ...] and start/end with criteria".to_string());
        }

        Ok(tokens)
    }
    fn is_valid_token_sequence(&self, tokens: &[Token]) -> bool {
        if tokens.is_empty() {
            return false;
        }

        // Must start with a condition
        if !matches!(tokens[0], Token::Condition(_)) {
            return false;
        }

        // Check alternating pattern: condition, operator, condition, operator, ...
        for (i, token) in tokens.iter().enumerate() {
            match (i % 2, token) {
                (0, Token::Condition(_)) => {}    // Even indices should be conditions
                (1, Token::And | Token::Or) => {} // Odd indices should be operators
                _ => return false,
            }
        }

        // Must end with a condition (even number of tokens)
        tokens.len() % 2 == 1
    }
    fn build_expression_with_precedence(&self, tokens: &[Token]) -> String {
        if tokens.is_empty() {
            return String::new();
        }

        if tokens.len() == 1 {
            if let Token::Condition(condition) = &tokens[0] {
                return condition.clone();
            }
            return String::new();
        }

        // Split by OR operators (lowest precedence)
        let or_groups = self.split_by_or(tokens);

        if or_groups.len() == 1 {
            // No OR operators, just handle AND operations
            self.build_and_expression(&or_groups[0])
        } else {
            // Multiple OR groups, join them with OR and wrap in parentheses if needed
            let or_expressions: Vec<String> = or_groups
                .iter()
                .map(|group| {
                    let expr = self.build_and_expression(group);
                    if group.len() > 1 {
                        format!("({})", expr)
                    } else {
                        expr
                    }
                })
                .collect();

            or_expressions.join(" OR ")
        }
    }
    fn split_by_or(&self, tokens: &[Token]) -> Vec<Vec<Token>> {
        let mut groups = Vec::new();
        let mut current_group = Vec::new();

        for token in tokens {
            match token {
                Token::Or => {
                    if !current_group.is_empty() {
                        groups.push(current_group);
                        current_group = Vec::new();
                    }
                }
                _ => current_group.push(token.clone()),
            }
        }

        if !current_group.is_empty() {
            groups.push(current_group);
        }

        groups
    }
    fn build_and_expression(&self, tokens: &[Token]) -> String {
        let conditions: Vec<String> = tokens
            .iter()
            .filter_map(|token| {
                if let Token::Condition(condition) = token {
                    Some(condition.clone())
                } else {
                    None
                }
            })
            .collect();

        conditions.join(" AND ")
    }

    fn format_condition_with_case_sensitivity_and_pattern(
        &self,
        field_name: &str,
        operator: &FilterOperator,
        values: &[serde_json::Value],
        case_sensitive: Option<bool>,
        match_pattern: Option<&MatchPattern>,
    ) -> String {
        let (_table_name, field_name, field_with_table) = if field_name.contains("COALESCE")
            || field_name.contains('(')
            || field_name.contains("::")
        {
            let extracted_field_name = if let Some(start) = field_name.rfind("AS ") {
                field_name[start + 3..].trim().replace("\"", "")
            } else if field_name.contains("::") {
                let base = field_name.splitn(2, "::").next().unwrap_or(field_name);
                let field_part = base
                    .rsplit('.')
                    .next()
                    .unwrap_or(base)
                    .trim()
                    .trim_matches('"')
                    .to_string();
                field_part
            } else {
                field_name.replace('"', "")
            };
            (String::new(), extracted_field_name, field_name.to_string())
        } else {
            // Handle simple field names with or without table prefix
            let mut parts = field_name.split(".");
            if let Some(first_part) = parts.next() {
                if let Some(second_part) = parts.next() {
                    // Two parts: table.field
                    let table_name = first_part.replace('"', "");
                    let field_name = second_part.replace('"', "");
                    let field_with_table = format!("\"{}\".\"{}\"", table_name, field_name);
                    (table_name, field_name, field_with_table)
                } else {
                    // One part: just field_name - check if it's a concatenated field
                    let field_name = first_part.replace('"', "");
                    // Check if this is a concatenated field that should be handled specially
                    let is_concatenated_field = self
                        .request_body
                        .get_concatenate_fields()
                        .iter()
                        .any(|concat_field| concat_field.field_name == field_name);
                    if is_concatenated_field {
                        // For concatenated fields, generate the full concatenated expression
                        let field_with_table = self.get_field_with_concatenation(
                            &self.table,
                            &field_name,
                            self.request_body.get_date_format(),
                            None,
                            self.timezone.as_deref(),
                            true,
                            self.request_body.get_time_format(),
                        );
                        (String::new(), field_name, field_with_table)
                    } else {
                        // For regular fields without table prefix, assume main table
                        let field_with_table = format!("\"{}\".\"{}\"", &self.table, field_name);
                        (self.table.clone(), field_name, field_with_table)
                    }
                }
            } else {
                // No parts (shouldn't happen, but handle gracefully)
                (String::new(), String::new(), String::new())
            }
        };
        let plural_form = pluralizer::pluralize(&field_name, 2, false);
        let is_plural = plural_form == field_name;
        let values_str = values
            .iter()
            .map(|v| match v {
                serde_json::Value::String(s) => format!("'{}'", s.replace("'", "''")), // Escape single quotes
                serde_json::Value::Number(n) => n.to_string(),
                serde_json::Value::Bool(b) => b.to_string(),
                serde_json::Value::Null => "NULL".to_string(),
                _ => format!("'{}'", v.to_string().trim().replace("'", "''")),
            })
            .collect::<Vec<_>>();

        match operator {
            FilterOperator::Equal => {
                if values_str.len() == 1 {
                    format!("{} = {}", field_with_table, values_str[0])
                } else {
                    format!("{} IN ({})", field_with_table, values_str.join(", "))
                }
            }
            FilterOperator::NotEqual => {
                if values_str.len() == 1 {
                    format!("{} != {}", field_with_table, values_str[0])
                } else {
                    // Use AND for each item: field != value1 AND field != value2 AND ...
                    let conditions: Vec<String> = values_str
                        .iter()
                        .map(|value| format!("{} != {}", field_with_table, value))
                        .collect();
                    format!("({})", conditions.join(" AND "))
                }
            }
            FilterOperator::GreaterThan => format!("{} > {}", field_with_table, values_str[0]),
            FilterOperator::GreaterThanOrEqual => {
                format!("{} >= {}", field_with_table, values_str[0])
            }
            FilterOperator::LessThan => format!("{} < {}", field_with_table, values_str[0]),
            FilterOperator::LessThanOrEqual => format!("{} <= {}", field_with_table, values_str[0]),
            FilterOperator::IsNull => format!("{} IS NULL", field_with_table),
            FilterOperator::IsNotNull => format!("{} IS NOT NULL", field_with_table),
            FilterOperator::Contains => {
                let like_op = if case_sensitive.unwrap_or(true) {
                    "LIKE"
                } else {
                    "ILIKE"
                };

                if is_plural {
                    let expr = if field_with_table.contains("::text") {
                        field_with_table.clone()
                    } else {
                        format!("{}::text", field_with_table)
                    };
                    return format!(
                        "{} {} '%{}%'",
                        expr,
                        like_op,
                        values_str[0].trim_matches('\'')
                    );
                }

                format!(
                    "{} {} '%{}%'",
                    field_with_table,
                    like_op,
                    values_str[0].trim_matches('\'')
                )
            }
            FilterOperator::NotContains => {
                let like_op = if case_sensitive.unwrap_or(true) {
                    "NOT LIKE"
                } else {
                    "NOT ILIKE"
                };

                if is_plural {
                    let expr = if field_with_table.contains("::text") {
                        field_with_table.clone()
                    } else {
                        format!("{}::text", field_with_table)
                    };
                    return format!(
                        "{} {} '%{}%'",
                        expr,
                        like_op,
                        values_str[0].trim_matches('\'')
                    );
                }

                format!(
                    "{} {} '%{}%'",
                    field_with_table,
                    like_op,
                    values_str[0].trim_matches('\'')
                )
            }
            FilterOperator::IsBetween => {
                if values_str.len() >= 2 {
                    format!(
                        "{} BETWEEN {} AND {}",
                        field_with_table, values_str[0], values_str[1]
                    )
                } else {
                    format!("{} = {}", field_with_table, values_str[0])
                }
            }
            FilterOperator::IsNotBetween => {
                if values_str.len() >= 2 {
                    format!(
                        "{} NOT BETWEEN {} AND {}",
                        field_with_table, values_str[0], values_str[1]
                    )
                } else {
                    format!("{} != {}", field_with_table, values_str[0])
                }
            }
            FilterOperator::IsEmpty => format!("{} = ''", field_with_table),
            FilterOperator::IsNotEmpty => format!("{} != ''", field_with_table),
            FilterOperator::Like => {
                let like_op = if case_sensitive.unwrap_or(true) {
                    "LIKE"
                } else {
                    "ILIKE"
                };
                let pattern = self.build_like_pattern(&values_str[0], match_pattern);
                if is_plural {
                    let expr = if field_with_table.contains("::text") {
                        field_with_table.clone()
                    } else {
                        format!("{}::text", field_with_table)
                    };
                    return format!("{} {} {}", expr, like_op, pattern);
                }
                format!("{} {} {}", field_with_table, like_op, pattern)
            }
            FilterOperator::HasNoValue => {
                // Check if field is an array by looking for array indicators
                let is_array_field = field_with_table.contains("[]")
                    || field_with_table.ends_with("_array")
                    || field_with_table.ends_with("s");

                if is_array_field {
                    // For array fields: check if array length is null or 0
                    format!(
                        "(ARRAY_LENGTH({}, 1) IS NULL OR ARRAY_LENGTH({}, 1) = 0 OR {} IS NULL)",
                        field_with_table, field_with_table, field_with_table
                    )
                } else {
                    // For regular fields: check if empty string or null
                    format!(
                        "({} = '' OR {} IS NULL)",
                        field_with_table, field_with_table
                    )
                }
            }
        }
    }

    fn build_like_pattern(&self, value: &str, match_pattern: Option<&MatchPattern>) -> String {
        let clean_value = value.trim_matches('\'');

        match match_pattern.unwrap_or(&MatchPattern::Contains) {
            MatchPattern::Exact => format!("'{}'", clean_value),
            MatchPattern::Prefix => format!("'{}%'", clean_value),
            MatchPattern::Suffix => format!("'%{}'", clean_value),
            MatchPattern::Contains => format!("'%{}%'", clean_value),
            MatchPattern::Custom => value.to_string(), // Use original value for custom patterns
        }
    }

    /// Helper method to extract operator from GroupAdvanceFilter enum
    fn get_group_operator<'a>(&self, group_filter: &'a GroupAdvanceFilter) -> &'a LogicalOperator {
        match group_filter {
            GroupAdvanceFilter::Criteria { operator, .. } => operator,
            GroupAdvanceFilter::Operator { operator, .. } => operator,
        }
    }

    /// Helper method to extract filters from GroupAdvanceFilter enum
    fn get_group_filters<'a>(
        &self,
        group_filter: &'a GroupAdvanceFilter,
    ) -> &'a Vec<FilterCriteria> {
        match group_filter {
            GroupAdvanceFilter::Criteria { filters, .. } => filters,
            GroupAdvanceFilter::Operator { filters, .. } => filters,
        }
    }

    /// Builds SQL expression for group advance filters
    /// Handles GroupAdvanceFilter enum which can be either "criteria" or "operator" type
    /// Each group can contain multiple FilterCriteria that are processed as a unit
    /// For "operator" type groups, filters can be empty array
    pub fn build_group_advance_filters_expression(
        &self,
        group_filters: &[GroupAdvanceFilter],
    ) -> Result<String, String> {
        if group_filters.is_empty() {
            return Ok(String::new());
        }

        if group_filters.len() == 1 {
            // Single group case - just process the filters within the group
            let group_filter = &group_filters[0];
            let filters = self.get_group_filters(group_filter);
            if !filters.is_empty() {
                let group_expression = self.build_infix_expression(filters)?;
                if !group_expression.is_empty() {
                    return Ok(format!("({})", group_expression));
                }
            }
            return Ok(String::new());
        }

        // Multiple groups case - build infix expression with group operators
        let mut tokens = Vec::new();

        for (i, group_filter) in group_filters.iter().enumerate() {
            let filters = self.get_group_filters(group_filter);
            let operator = self.get_group_operator(group_filter);

            // Process the filters within this group (skip if empty for operator type)
            if !filters.is_empty() {
                let group_expression = self.build_infix_expression(filters)?;
                if !group_expression.is_empty() {
                    // Wrap each group in parentheses for proper precedence
                    tokens.push(Token::Condition(format!("({})", group_expression)));
                }
            }

            // Add operator token if not the last group
            if i < group_filters.len() - 1 {
                match operator {
                    LogicalOperator::And => tokens.push(Token::And),
                    LogicalOperator::Or => tokens.push(Token::Or),
                }
            }
        }

        if tokens.is_empty() {
            return Ok(String::new());
        }

        // Use the same infix expression building logic as advance filters
        Ok(self.build_expression_with_precedence(&tokens))
    }
    pub fn construct_offset(&self) -> String {
        let offset_constructor = OffsetConstructor::new(self.request_body.clone());
        offset_constructor.construct_offset()
    }
    pub fn construct_limit(&self) -> String {
        let limit_constructor = LimitConstructor::new(self.request_body.clone());
        limit_constructor.construct_limit()
    }
}
