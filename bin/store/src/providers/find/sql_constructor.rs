use crate::structs::structs::{
    ConcatenateField, FilterCriteria, FilterOperator, GetByFilter, GroupAdvanceFilter, GroupBy,
    Join, LogicalOperator, MatchPattern, SortOption,
};
use std::collections::HashMap;
// Trait to define common interface for both GetByFilter and AggregationFilter
pub trait QueryFilter {
    fn get_advance_filters(&self) -> &[FilterCriteria];
    fn get_joins(&self) -> &[Join];
    fn get_limit(&self) -> usize;
    fn get_date_format(&self) -> &str;

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
    fn get_group_advance_filters(&self) -> &[crate::structs::structs::GroupAdvanceFilter] {
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
}

// Implement QueryFilter for GetByFilter
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

    fn get_group_advance_filters(&self) -> &[crate::structs::structs::GroupAdvanceFilter] {
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
}

#[derive(Debug, Clone)]
enum Token {
    Condition(String),
    And,
    Or,
}

pub struct SQLConstructor<T: QueryFilter> {
    pub request_body: T,
    pub table: String,
    pub organization_id: Option<String>,
    pub is_root: bool,
}

impl<T: QueryFilter> SQLConstructor<T> {
    pub fn new(request_body: T, table: String, is_root: bool) -> Self {
        Self {
            request_body,
            table,
            organization_id: None,
            is_root,
        }
    }

    pub fn with_organization_id(mut self, organization_id: String) -> Self {
        self.organization_id = Some(organization_id);
        self
    }

    pub fn construct(&mut self) -> Result<String, String> {
        let mut sql = String::from("SELECT ");
        sql.push_str(&self.construct_selections());
        sql.push_str(" FROM ");
        sql.push_str(&self.table);
        sql.push_str(&self.construct_joins());
        sql.push_str(&self.construct_where_clauses()?);
        sql.push_str(&self.construct_group_by());
        sql.push_str(&self.construct_order_by());
        sql.push_str(&self.construct_offset());
        sql.push_str(&self.construct_limit());
        dbg!(&sql);
        Ok(sql)
    }

    fn get_field(table: &str, field: &str, format_str: &str) -> String {
        Self::get_field_with_parse_as(table, field, format_str, None)
    }

    fn get_field_with_parse_as(
        table: &str,
        field: &str,
        format_str: &str,
        parse_as: Option<&str>,
    ) -> String {
        // TODO: apply permissions
        let base_field = if field.ends_with("_date") {
            Self::date_format_wrapper(table, field, Some(format_str))
        } else if field.ends_with("_time") {
            Self::time_format_wrapper(&format!("\"{}\".\"{}\"", table, field), None)
        } else {
            format!("\"{}\".\"{}\"", table, field)
        };
        // Apply parse_as type casting if provided and not empty
        if let Some(cast_type) = parse_as {
            if !cast_type.is_empty() {
                format!("{}::{}", base_field, cast_type)
            } else {
                base_field
            }
        } else {
            base_field
        }
    }

    fn get_field_with_concatenation(
        &self,
        table: &str,
        field: &str,
        format_str: &str,
        parse_as: Option<&str>,
    ) -> String {
        // Check if this field is defined as a concatenated field
        for concat_field in self.request_body.get_concatenate_fields() {
            // Match by field name and entity/aliased_entity
            if concat_field.field_name == field {
                let target_table = if let Some(aliased_entity) = &concat_field.aliased_entity {
                    aliased_entity.as_str()
                } else {
                    concat_field.entity.as_str()
                };

                // Check if the table matches
                if target_table == table {
                    // Generate concatenated expression
                    let concatenated_expression = concat_field
                        .fields
                        .iter()
                        .map(|f| Self::get_field_with_parse_as(table, f, format_str, None))
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
        Self::get_field_with_parse_as(table, field, format_str, parse_as)
    }

    fn date_format_wrapper(table: &str, field: &str, format_str: Option<&str>) -> String {
        let format = format_str.unwrap_or("mm/dd/YYYY");
        format!(
            "Coalesce(TO_CHAR(\"{}\".\"{}\"::DATE, '{}'), '')",
            table, field, format
        )
    }
    fn time_format_wrapper(field: &str, timezone: Option<&str>) -> String {
        // Convert from stored timezone to target timezone
        // PostgreSQL AT TIME ZONE converts from the specified timezone to UTC, then to local
        let target_timezone = timezone.unwrap_or("Asia/Manila");
        let timezone_query = format!(" AT TIME ZONE '{}'", target_timezone);
        format!("({}::time {})::time", field, timezone_query)
    }
    fn construct_selections(&self) -> String {
        let mut selections = Vec::new();

        // Check if there are existing join selections that would handle concatenated fields
        let has_join_selections = !self.request_body.get_joins().is_empty()
            && !self.request_body.get_pluck_object().is_empty()
            && self
                .request_body
                .get_pluck_object()
                .iter()
                .any(|(_, fields)| !fields.is_empty());

        // Handle concatenated fields for main table and joins
        if !self.request_body.get_concatenate_fields().is_empty() {
            for field in self.request_body.get_concatenate_fields() {
                let entity_alias = field.aliased_entity.as_ref().unwrap_or(&field.entity);

                // Check if this concatenated field is for the main table
                let is_main_table = field.entity == self.table
                    || field.aliased_entity.as_ref() == Some(&self.table);

                // Check if this concatenated field is for a joined table that has pluck_object
                let is_joined_table_with_pluck = has_join_selections
                    && self
                        .request_body
                        .get_pluck_object()
                        .contains_key(entity_alias)
                    && !is_main_table;

                // Only create selection for main table concatenated fields
                // Join selections will handle concatenated fields for joined tables
                if is_main_table && !is_joined_table_with_pluck {
                    let table_name = if let Some(aliased_entity) = &field.aliased_entity {
                        aliased_entity
                    } else {
                        field.entity.as_str()
                    };

                    let concatenated_expression = field
                        .fields
                        .iter()
                        .map(|f| {
                            Self::get_field(table_name, f, self.request_body.get_date_format())
                        })
                        .collect::<Vec<_>>()
                        .join(&format!(" || '{}' || ", field.separator));

                    let alias = &field.field_name;

                    selections.push(format!("({}) AS \"{}\"", concatenated_expression, alias));
                }
            }
        }

        if let Some(distinct_by) = self.request_body.get_distinct_by() {
            if distinct_by.is_empty() {
                // Skip if distinct_by is empty string
            } else if distinct_by == "id" {
                return format!("DISTINCT \"{}\".\"{}\"", self.table, distinct_by);
            } else {
                let parts: Vec<&str> = distinct_by.split('.').collect();
                return if parts.len() == 2 {
                    format!("DISTINCT \"{}\".\"{}\"", parts[0], parts[1])
                } else {
                    format!("DISTINCT \"{}\".\"{}\"", self.table, distinct_by)
                };
            }
        }

        let group_by = self.request_body.get_group_by();

        // If group_by is present and not empty, replace all selections with group by aggregations
        if let Some(group_by) = group_by {
            if !group_by.fields.is_empty() {
                return self.construct_group_by_selections();
            }
        }

        // Add join selections from pluck_object if joins are present
        if !self.request_body.get_joins().is_empty()
            && !self.request_body.get_pluck_object().is_empty()
            && self
                .request_body
                .get_pluck_object()
                .iter()
                .any(|(_, fields)| !fields.is_empty())
        {
            let join_selections = self.construct_join_selections();
            if !join_selections.is_empty() {
                selections.extend(join_selections);
            }
        }
        // set pluck as selections
        if !self.request_body.get_pluck().is_empty() {
            let pluck = self.construct_pluck();
            if !pluck.is_empty() {
                selections.push(pluck);
            }
        }
        // set pluck group object as selections
        if !self.request_body.get_pluck_group_object().is_empty() {
            let pluck_group_object = self.construct_pluck_group_object();
            if !pluck_group_object.is_empty() {
                selections.push(pluck_group_object);
            }
        }
        dbg!(format!("selections: {:?}", selections));
        if selections.is_empty() {
            "id".to_string()
        } else {
            selections.join(", ")
        }
    }

    fn construct_group_by_selections(&self) -> String {
        let group_by = self.request_body.get_group_by();
        let mut selections = Vec::new();

        if let Some(group_by) = group_by {
            // Add count aggregation if has_count is true
            if group_by.has_count {
                selections.push("COUNT(*) AS count".to_string());
                selections.push("COUNT(*) OVER () AS total_group_count".to_string());
            }

            // Add unique fields from group_by.fields
            for field in &group_by.fields {
                let field_selection =
                    Self::get_field(&self.table, field, self.request_body.get_date_format());
                selections.push(field_selection);
            }
        }

        if selections.is_empty() {
            "id".to_string()
        } else {
            selections.join(", ")
        }
    }

    fn construct_pluck(&self) -> String {
        let mut pluck_fields = Vec::new();

        // Add regular pluck fields
        for field in self.request_body.get_pluck() {
            pluck_fields.push(Self::get_field(
                &self.table,
                field,
                self.request_body.get_date_format(),
            ));
        }

        // Add concatenated fields that match the main table
        if !self.request_body.get_concatenate_fields().is_empty() {
            for field in self.request_body.get_concatenate_fields() {
                if field.aliased_entity.as_ref() == Some(&self.table) || field.entity == self.table
                {
                    let table_name = if let Some(aliased_entity) = &field.aliased_entity {
                        aliased_entity
                    } else {
                        field.entity.as_str()
                    };
                    let concatenated_expression = field
                        .fields
                        .iter()
                        .map(|f| {
                            Self::get_field(table_name, f, self.request_body.get_date_format())
                        })
                        .collect::<Vec<_>>()
                        .join(&format!(" || '{}' || ", field.separator));

                    // Create unique alias by combining table_name and field_name to avoid duplicates
                    let unique_alias = if let Some(aliased_entity) = &field.aliased_entity {
                        format!("{}_{}", aliased_entity, field.field_name)
                    } else {
                        field.field_name.clone()
                    };

                    pluck_fields.push(format!("({}) AS {}", concatenated_expression, unique_alias));
                }
            }
        }

        if pluck_fields.is_empty() {
            String::new()
        } else {
            pluck_fields.join(", ")
        }
    }
    fn construct_pluck_group_object(&self) -> String {
        let mut selections = Vec::new();
        // Check if pluck_group_object exists and is not empty
        // Iterate over each key-value pair in pluck_group_object
        for (table_alias, fields) in self.request_body.get_pluck_group_object() {
            // For each field in the fields vector, create a JSONB_AGG statement
            for field in fields {
                let selection = format!(
                    "JSONB_AGG(\"{}\".\"{}\") AS \"{}_{}\"",
                    table_alias, field, table_alias, field
                );
                selections.push(selection);
            }

            // Add concatenated fields that match this table alias
            if !self.request_body.get_concatenate_fields().is_empty() {
                for field in self.request_body.get_concatenate_fields() {
                    if field.aliased_entity.as_ref() == Some(table_alias)
                        || field.entity == *table_alias
                    {
                        let table_name = if let Some(aliased_entity) = &field.aliased_entity {
                            aliased_entity
                        } else {
                            field.entity.as_str()
                        };
                        let concatenated_expression = field
                            .fields
                            .iter()
                            .map(|f| {
                                Self::get_field(table_name, f, self.request_body.get_date_format())
                            })
                            .collect::<Vec<_>>()
                            .join(&format!(" || '{}' || ", field.separator));
                        let selection = format!(
                            "JSONB_AGG({}) AS \"{}_{}\"",
                            concatenated_expression, table_alias, field.field_name
                        );
                        selections.push(selection);
                    }
                }
            }
        }

        // Join all selections with commas, or return empty string if no selections
        if selections.is_empty() {
            String::new()
        } else {
            selections.join(", ")
        }
    }
    fn construct_join_selections(&self) -> Vec<String> {
        let mut join_selections = Vec::new();
        if let Some(fields) = self.request_body.get_pluck_object().get(&self.table) {
            // Get concatenated field names to filter out
            let concatenated_field_names: Vec<String> = self
                .request_body
                .get_concatenate_fields()
                .iter()
                .map(|f| f.field_name.clone())
                .collect();

            join_selections.extend(
                fields
                    .iter()
                    .filter(|field| *field != "id" && !concatenated_field_names.contains(field))
                    .map(|field| {
                        Self::get_field(&self.table, field, self.request_body.get_date_format())
                    }),
            );
        }
        // Only construct selections if joins are present
        if self.request_body.get_joins().is_empty() {
            return join_selections;
        }
        // Iterate through each alias in pluck_object
        for join in self.request_body.get_joins() {
            let to_alias = join.field_relation.to.alias.as_deref().unwrap_or(&join.field_relation.to.entity);
            if let Some(fields) = self.request_body.get_pluck_object().get(to_alias) {
                let target_table = &join.field_relation.to.entity;
                // Fixed: Look for a join whose target entity matches this join's source entity
                    let previous_join = self.request_body.get_joins().iter().find(|j| {
                        // Get the target reference (prioritize alias over entity)
                        let j_to_reference = j.field_relation.to.alias.as_deref().unwrap_or(&j.field_relation.to.entity);
                        // Get the current join's from reference (prioritize alias over entity)
                        let current_from_reference = join.field_relation.from.alias.as_deref().unwrap_or(&join.field_relation.from.entity);
                        
                        // Find a join where the target reference matches this join's from reference
                        j_to_reference == current_from_reference
                    });
                    let join_condition =
                        self.build_join_condition_for_alias(to_alias, join, previous_join);

                    // Build JSONB_BUILD_OBJECT field pairs
                    let mut field_pairs: Vec<String> = fields
                        .iter()
                        .map(|field| {
                            format!(
                                "'{}', {}",
                                field,
                                Self::get_field(
                                    to_alias,
                                    field,
                                    self.request_body.get_date_format()
                                )
                            )
                        })
                        .collect();

                    if !self.request_body.get_concatenate_fields().is_empty() {
                        self.request_body
                            .get_concatenate_fields()
                            .iter()
                            .for_each(|field| {
                                // Check if this concatenate field matches the current alias (either by entity or aliased_entity)
                                if field.aliased_entity.as_deref() == Some(to_alias)
                                    || field.entity == *to_alias
                                {
                                    let table_name =
                                        if let Some(aliased_entity) = &field.aliased_entity {
                                            aliased_entity
                                        } else {
                                            field.entity.as_str()
                                        };
                                    let concatenated_expression = field
                                        .fields
                                        .iter()
                                        .map(|f| {
                                            Self::get_field(
                                                table_name,
                                                f,
                                                self.request_body.get_date_format(),
                                            )
                                        })
                                        .collect::<Vec<_>>()
                                        .join(&format!(" || '{}' || ", field.separator));
                                    field_pairs.push(format!(
                                        "'{}', ({})",
                                        field.field_name, concatenated_expression
                                    ));
                                }
                            });
                    }

                    let standard_where = match self.build_system_where_clause(to_alias) {
                        Ok(clause) => clause,
                        Err(_) => format!("({}.tombstone = 0)", to_alias),
                    };

                    let order_by_clause = self.build_jsonb_agg_order_by(to_alias);
                    let mut selection = format!(
                        "COALESCE((SELECT JSONB_AGG(JSONB_BUILD_OBJECT({}){}) FROM \"{}\" \"{}\" WHERE {} AND {}), '[]') AS \"{}\"",
                        field_pairs.join(", "),
                        order_by_clause,
                        target_table,
                        to_alias,
                        standard_where,
                        join_condition,
                        to_alias
                    );

                    if join.nested {
                        let prev_join_to_alias = previous_join
                            .unwrap()
                            .field_relation
                            .to
                            .alias
                            .as_deref()
                            .unwrap_or(&previous_join.unwrap().field_relation.to.entity);
                        selection = format!(
                        "COALESCE((SELECT JSONB_AGG(JSONB_BUILD_OBJECT({}){}) FROM \"{}\" \"{}\" LEFT JOIN \"{}\" \"{}\" ON {} WHERE {} AND {}), '[]') AS \"{}\"",
                        // JSON_BUILD OBJECT
                        field_pairs.join(", "),
                        // ORDER BY
                        order_by_clause,
                        // FROM
                        previous_join.unwrap().field_relation.to.entity,
                        prev_join_to_alias,
                        // LEFT JOIN
                        target_table,
                        to_alias,
                        // ON
                        self.build_join_condition_for_alias(to_alias, join, Some(join)),
                        // WHERE
                        standard_where,
                        // ADDITIONAL WHERE
                        join_condition,
                        // selection alias
                        to_alias
                    );
                    }

                    join_selections.push(selection);
                } else {
                    // Handle case where no fields are specified for this alias
                    join_selections.push(format!("\"{}\".\"id\"", to_alias));
                }
        }

        join_selections
    }

    /// Builds ORDER BY clause for JSONB_AGG based on multiple_sort or fallback to order_by/order_direction
    fn build_jsonb_agg_order_by(&self, table_alias: &str) -> String {
        // Check if multiple_sort is available and not empty
        if !self.request_body.get_multiple_sort().is_empty() {
            let sort_clauses: Vec<String> = self
                .request_body
                .get_multiple_sort()
                .iter()
                .map(|sort_option| {
                    let field_parts: Vec<&str> = sort_option.by_field.split('.').collect();
                    let (sort_table_alias, field_name) = if field_parts.len() == 2 {
                        (field_parts[0], field_parts[1])
                    } else {
                        (table_alias, sort_option.by_field.as_str())
                    };

                    let field_expression = Self::get_field(
                        sort_table_alias,
                        field_name,
                        self.request_body.get_date_format(),
                    );

                    // Handle case sensitivity
                    let final_field = if sort_option.is_case_sensitive_sorting.unwrap_or(false) {
                        field_expression
                    } else {
                        format!("LOWER({})", field_expression)
                    };

                    format!(
                        "{} {}",
                        final_field,
                        sort_option.by_direction.to_uppercase()
                    )
                })
                .collect();

            format!(" ORDER BY {}", sort_clauses.join(", "))
        }
        // Fallback to single field sorting using trait methods
        else if !self.request_body.get_order_by().is_empty() {
            // Skip ORDER BY if using default values (id field with asc direction)
            let order_by = self.request_body.get_order_by();
            let order_direction = self.request_body.get_order_direction();

            if order_by == "id" && order_direction == "asc" {
                return String::new();
            }

            let field_expression =
                Self::get_field(table_alias, order_by, self.request_body.get_date_format());

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
            String::new()
        }
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
        format!(
            "\"{}\".\"{}\" = \"{}\".\"{}\"",
            self.table, from_field, alias, to_field
        )
    }

    pub fn construct_joins(&self) -> String {
        if self.request_body.get_joins().is_empty() {
            String::from("")
        } else {
            let mut join_clauses = Vec::new();

            for join in self.request_body.get_joins() {
                match join.r#type.as_str() {
                    "left" => {
                        let join_clause = self.build_left_join_lateral(join);
                        join_clauses.push(join_clause);
                    }
                    "self" => {
                        let join_clause = self.build_self_join_lateral(join);
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
    /// Constructs the standard WHERE clause pattern used across queries
    fn build_system_where_clause(&self, table_alias: &str) -> Result<String, String> {
        // For root access, only check tombstone
        if self.is_root {
            return Ok(format!("({}.tombstone = 0)", table_alias));
        }

        // For non-root access, check organization constraints
        let organization_id = match &self.organization_id {
            Some(id) => format!("'{}'", id),
            None => return Err("Organization ID is required".to_string()),
        };

        Ok(format!(
            "({}.tombstone = 0 AND {}.organization_id IS NOT NULL AND {}.organization_id = {})",
            table_alias, table_alias, table_alias, organization_id
        ))
    }

    pub fn construct_where_clauses(&self) -> Result<String, String> {
        // Use the reusable standard WHERE clause pattern
        let mut base_where = format!(" WHERE {}", self.build_system_where_clause(&self.table)?);

        // Prioritize advance_filters over group_advance_filters
        if !self.request_body.get_advance_filters().is_empty() {
            let expression =
                self.build_infix_expression(self.request_body.get_advance_filters())?;
            if !expression.is_empty() {
                base_where.push_str(" AND ");
                base_where.push_str(&expression);
            }
        } else if !self.request_body.get_group_advance_filters().is_empty() {
            let group_expression = self.build_group_advance_filters_expression(
                self.request_body.get_group_advance_filters(),
            )?;
            if !group_expression.is_empty() {
                base_where.push_str(" AND ");
                base_where.push_str(&group_expression);
            }
        }

        Ok(base_where)
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
    fn build_infix_expression(&self, filters: &[FilterCriteria]) -> Result<String, String> {
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
            } = &filters[0]
            {
                let field_name = self.get_field_with_concatenation(
                    entity.as_deref().unwrap_or(&self.table),
                    field,
                    self.request_body.get_date_format(),
                    Some(parse_as),
                );
                return Ok(self.format_condition_with_case_sensitivity_and_pattern(
                    &field_name,
                    operator,
                    values,
                    *case_sensitive,
                    match_pattern.as_ref(),
                ));
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
                } => {
                    let field_name = self.get_field_with_concatenation(
                        entity.as_deref().unwrap_or(&self.table),
                        field,
                        self.request_body.get_date_format(),
                        Some(parse_as),
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

    fn build_left_join_lateral(&self, join: &Join) -> String {
        self.build_join_lateral(join, false)
    }

    fn build_self_join_lateral(&self, join: &Join) -> String {
        self.build_join_lateral(join, true)
    }

    fn build_join_lateral(&self, join: &Join, is_self_join: bool) -> String {
        let to_entity = if is_self_join {
            &self.table
        } else {
            &join.field_relation.to.entity
        };

        let to_alias = if is_self_join {
            join.field_relation
                .to
                .alias
                .as_deref()
                .unwrap_or(&self.table)
        } else {
            join.field_relation.to.alias.as_deref().unwrap_or(to_entity)
        };

        let to_field = &join.field_relation.to.field;
        let from_entity = if is_self_join {
            &self.table
        } else {
            &join.field_relation.from.entity
        };
        let from_field = &join.field_relation.from.field;
        let is_nested = join.nested;

        // Build the lateral subquery alias
        let lateral_alias = format!("joined_{}", to_alias);

        // Build dynamic field selection based on pluck_object
        let selected_fields =
            if let Some(fields) = self.request_body.get_pluck_object().get(to_alias) {
                fields
                    .iter()
                    .map(|field| format!("\"{}\".\"{}\"", lateral_alias, field))
                    .collect::<Vec<_>>()
                    .join(", ")
            } else {
                // Default fallback fields if no pluck_object configuration found
                format!("\"{}\".\"id\"", lateral_alias)
            };

        let standard_where = self
            .build_system_where_clause(&lateral_alias)
            .unwrap_or_else(|_| format!("({}.tombstone = 0)", lateral_alias));

        if is_nested {
            return format!(
                "LEFT JOIN LATERAL (SELECT {} FROM \"{}\" \"{}\" WHERE {} AND \"{}\".\"{}\" = \"{}\".\"{}\" ) AS \"{}\" ON TRUE",
                selected_fields,
                to_entity, lateral_alias,
                standard_where,
                lateral_alias, to_field, from_entity, from_field,
                to_alias
            );
        }

        let from_table_ref = if is_self_join {
            &self.table
        } else {
            &self.table
        };

        format!(
            "LEFT JOIN LATERAL (SELECT {} FROM \"{}\" \"{}\" WHERE {} AND \"{}\".\"{}\" = \"{}\".\"{}\" ) AS \"{}\" ON TRUE",
            selected_fields,
            to_entity, lateral_alias,
            standard_where,
            from_table_ref, from_field, lateral_alias, to_field,
            to_alias
        )
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
        let values_str = values
            .iter()
            .map(|v| match v {
                serde_json::Value::String(s) => format!("'{}'", s.replace("'", "''")), // Escape single quotes
                serde_json::Value::Number(n) => n.to_string(),
                serde_json::Value::Bool(b) => b.to_string(),
                serde_json::Value::Null => "NULL".to_string(),
                _ => format!("'{}'", v.to_string().replace("'", "''")),
            })
            .collect::<Vec<_>>();

        match operator {
            FilterOperator::Equal => {
                if values_str.len() == 1 {
                    format!("{} = {}", field_name, values_str[0])
                } else {
                    format!("{} IN ({})", field_name, values_str.join(", "))
                }
            }
            FilterOperator::NotEqual => {
                if values_str.len() == 1 {
                    format!("{} != {}", field_name, values_str[0])
                } else {
                    // Use AND for each item: field != value1 AND field != value2 AND ...
                    let conditions: Vec<String> = values_str
                        .iter()
                        .map(|value| format!("{} != {}", field_name, value))
                        .collect();
                    format!("({})", conditions.join(" AND "))
                }
            }
            FilterOperator::GreaterThan => format!("{} > {}", field_name, values_str[0]),
            FilterOperator::GreaterThanOrEqual => format!("{} >= {}", field_name, values_str[0]),
            FilterOperator::LessThan => format!("{} < {}", field_name, values_str[0]),
            FilterOperator::LessThanOrEqual => format!("{} <= {}", field_name, values_str[0]),
            FilterOperator::IsNull => format!("{} IS NULL", field_name),
            FilterOperator::IsNotNull => format!("{} IS NOT NULL", field_name),
            FilterOperator::Contains => {
                let like_op = if case_sensitive.unwrap_or(true) {
                    "LIKE"
                } else {
                    "ILIKE"
                };
                format!(
                    "{} {} '%{}%'",
                    field_name,
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
                format!(
                    "{} {} '%{}%'",
                    field_name,
                    like_op,
                    values_str[0].trim_matches('\'')
                )
            }
            FilterOperator::IsBetween => {
                if values_str.len() >= 2 {
                    format!(
                        "{} BETWEEN {} AND {}",
                        field_name, values_str[0], values_str[1]
                    )
                } else {
                    format!("{} = {}", field_name, values_str[0])
                }
            }
            FilterOperator::IsNotBetween => {
                if values_str.len() >= 2 {
                    format!(
                        "{} NOT BETWEEN {} AND {}",
                        field_name, values_str[0], values_str[1]
                    )
                } else {
                    format!("{} != {}", field_name, values_str[0])
                }
            }
            FilterOperator::IsEmpty => format!("{} = ''", field_name),
            FilterOperator::IsNotEmpty => format!("{} != ''", field_name),
            FilterOperator::Like => {
                let like_op = if case_sensitive.unwrap_or(true) {
                    "LIKE"
                } else {
                    "ILIKE"
                };
                let pattern = self.build_like_pattern(&values_str[0], match_pattern);
                format!("{} {} {}", field_name, like_op, pattern)
            }
            FilterOperator::HasNoValue => {
                // Check if field is an array by looking for array indicators
                let is_array_field = field_name.contains("[]")
                    || field_name.ends_with("_array")
                    || field_name.ends_with("s");

                if is_array_field {
                    // For array fields: check if array length is null or 0
                    format!(
                        "(ARRAY_LENGTH({}, 1) IS NULL OR ARRAY_LENGTH({}, 1) = 0 OR {} IS NULL)",
                        field_name, field_name, field_name
                    )
                } else {
                    // For regular fields: check if empty string or null
                    format!("({} = '' OR {} IS NULL)", field_name, field_name)
                }
            }
        }
    }

    fn build_like_pattern(&self, value: &str, match_pattern: Option<&MatchPattern>) -> String {
        let clean_value = value.trim_matches('\'');

        match match_pattern {
            Some(MatchPattern::Exact) => format!("'{}'", clean_value),
            Some(MatchPattern::Prefix) => format!("'{}%'", clean_value),
            Some(MatchPattern::Suffix) => format!("'%{}'", clean_value),
            Some(MatchPattern::Contains) => format!("'%{}%'", clean_value),
            Some(MatchPattern::Custom) => value.to_string(), // Use original value for custom patterns
            None => value.to_string(), // Use original value if no pattern specified
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
    fn build_group_advance_filters_expression(
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

    fn construct_order_by(&self) -> String {
        if let Some(distinct_by) = self.request_body.get_distinct_by() {
            if distinct_by.is_empty() {
                return String::new();
            }

            let fields: Vec<String> = distinct_by
                .split(',')
                .map(|field| {
                    let parts: Vec<&str> = field.trim().split('.').collect();
                    if parts.len() == 2 {
                        format!("\"{}\".\"{}\"", parts[0], parts[1])
                    } else {
                        format!("\"{}\".\"{}\"", self.table, field.trim())
                    }
                })
                .collect();
            return format!(" ORDER BY {}", fields.join(", "));
        }
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
                    );

                    // Handle case sensitivity
                    let final_field = if sort_option.is_case_sensitive_sorting.unwrap_or(false) {
                        field_expression
                    } else {
                        format!("LOWER({})", field_expression)
                    };

                    format!(
                        "{} {}",
                        final_field,
                        sort_option.by_direction.to_uppercase()
                    )
                })
                .collect();

            format!(" ORDER BY {}", sort_clauses.join(", "))
        }
        // Fallback to single field sorting using trait methods
        else if !self.request_body.get_order_by().is_empty() {
            // Skip ORDER BY if using default values (id field with asc direction)
            let order_by = self.request_body.get_order_by();
            let order_direction = self.request_body.get_order_direction();

            if order_by == "id" && order_direction == "asc" {
                return String::from("");
            }

            let field_expression =
                Self::get_field(&self.table, order_by, self.request_body.get_date_format());

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
    fn construct_group_by(&self) -> String {
        let group_by = self.request_body.get_group_by();
        if let Some(group_by) = group_by {
            if !group_by.fields.is_empty() {
                // Get all fields from the group_by fields and create GROUP BY clause
                let group_fields: Vec<String> = group_by
                    .fields
                    .iter()
                    .map(|field| {
                        Self::get_field(&self.table, field, self.request_body.get_date_format())
                    })
                    .collect();

                return format!(" GROUP BY {}", group_fields.join(", "));
            }
        }
        String::from("")
    }
    fn construct_offset(&self) -> String {
        if self.request_body.get_offset() > 0 {
            format!(" OFFSET {}", self.request_body.get_offset())
        } else {
            String::from("")
        }
    }
    fn construct_limit(&self) -> String {
        if self.request_body.get_limit() > 0 {
            format!(" LIMIT {}", self.request_body.get_limit())
        } else {
            String::from("LIMIT 10")
        }
    }
}
