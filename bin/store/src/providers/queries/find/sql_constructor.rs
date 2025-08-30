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
use pluralizer::pluralize;
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
        let plural_form = pluralize(entity, 2, false);
        if plural_form == entity {
            plural_form.to_string()
        } else {
            format!("{}s", entity)
        }
    }

    pub fn with_organization_id(mut self, organization_id: String) -> Self {
        self.organization_id = Some(organization_id);
        self
    }

    pub fn construct(&mut self) -> Result<String, String> {
        let body_timezone = self.request_body.get_timezone().unwrap_or("Asia/Manila");
        let timezone = self
            .timezone
            .as_deref()
            .unwrap_or(body_timezone)
            .to_string();
        self.timezone = Some(timezone);

        let mut sql = String::from("SELECT ");

        // Use the new constructor modules with proper parameters
        sql.push_str(&SelectionsConstructor::construct_selections(
            &self.request_body,
            &self.table,
            self.timezone.as_deref(),
            |entity| self.normalize_entity_name(entity),
            |table, field, format_str, main_table, timezone, with_alias| {
                Self::get_field(table, field, format_str, main_table, timezone, with_alias)
            },
            |table, field, format_str, parse_as, main_table, timezone, with_alias| {
                Self::get_field_with_parse_as(
                    table, field, format_str, parse_as, main_table, timezone, with_alias,
                )
            },
            |table_alias| self.build_system_where_clause(table_alias),
            |filters| self.build_infix_expression(filters),
        ));

        sql.push_str(" FROM ");
        sql.push_str(&self.table);

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
        )?);

        let group_by_constructor = GroupByConstructor::new(
            &self.table,
            self.timezone.as_deref(),
            self.request_body.get_date_format(),
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
        );
        sql.push_str(&order_by_constructor.construct_order_by());

        let offset_constructor = OffsetConstructor::new(self.request_body.clone());
        sql.push_str(&offset_constructor.construct_offset());

        let limit_constructor = LimitConstructor::new(self.request_body.clone());
        sql.push_str(&limit_constructor.construct_limit());
        Ok(sql)
    }

    pub fn get_field(
        table: &str,
        field: &str,
        format_str: &str,
        main_table: &str,
        timezone: Option<&str>,
        with_alias: bool,
    ) -> String {
        Self::get_field_with_parse_as(
            table, field, format_str, None, main_table, timezone, with_alias,
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
    ) -> String {
        // TODO: apply permissions
        let base_field = if field.ends_with("_date") {
            Self::date_format_wrapper(table, field, Some(format_str), timezone, with_alias)
        } else if field.ends_with("_time") {
            Self::time_format_wrapper(
                &format!("\"{}\".\"{}\"", table, field),
                timezone,
                main_table,
                with_alias,
            )
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
        timezone: Option<&str>,
        with_alias: bool,
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
                                    with_alias
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
        )
    }

    fn date_format_wrapper(
        table: &str,
        field: &str,
        format_str: Option<&str>,
        timezone: Option<&str>,
        with_alias: bool,
    ) -> String {
        let field_prefix = field.strip_suffix("_date").unwrap_or(field);
        let time_field = format!("{}_time", field_prefix);
        let formatted_field = format!(
            "\"{}\".\"{}\"::TIMESTAMP + \"{}\".\"{}\"::INTERVAL",
            table, field, table, time_field
        );
        let target_timezone = timezone.unwrap_or("Asia/Manila");
        let server_timezone = std::env::var("TZ").unwrap_or_else(|_| "UTC".to_string());

        let timezone_query = format!(
            "AT TIME ZONE '{}' AT TIME ZONE '{}'",
            server_timezone, target_timezone
        );
        let field_with_timezone = format!("(({}) {})", formatted_field, timezone_query);
        let format = format_str.unwrap_or("mm/dd/YYYY");
        let alias = if with_alias {
            format!(" AS \"{}\"", field)
        } else {
            "".to_string()
        };
        format!(
            "COALESCE(TO_CHAR({}::DATE, '{}'), ''){}",
            field_with_timezone, format, alias
        )
    }
    fn time_format_wrapper(
        field: &str,
        timezone: Option<&str>,
        main_table: &str,
        with_alias: bool,
    ) -> String {
        // Convert from stored timezone to target timezone
        // PostgreSQL AT TIME ZONE converts from the specified timezone to UTC, then to local
        let field_parts: Vec<&str> = field.split('.').collect();
        let table_name = field_parts[0].replace("\"", "");
        let partial_field_name = field_parts[1].replace("\"", "");
        let cloned_partial_field_name = partial_field_name.clone();
        let field_prefix = cloned_partial_field_name
            .strip_suffix("_time")
            .unwrap_or(field);
        let field_name = if field_parts.len() == 2 {
            if table_name != main_table {
                format!("{}_{}", table_name, partial_field_name)
            } else {
                partial_field_name
            }
        } else {
            field.to_string()
        };

        let date_field = format!("\"{}\".\"{}_date\"", table_name, field_prefix);
        let formatted_field = format!("{}::TIMESTAMP + {}::INTERVAL", date_field, field);
        let target_timezone = timezone.unwrap_or("Asia/Manila");
        let server_timezone = std::env::var("TZ").unwrap_or_else(|_| "UTC".to_string());
        let timezone_query = format!(
            "AT TIME ZONE '{}' AT TIME ZONE '{}'",
            server_timezone, target_timezone
        );
        let field_with_timezone = format!("(({}) {})", formatted_field, timezone_query);
        let alias = if with_alias {
            format!(" AS {}", field_name)
        } else {
            "".to_string()
        };
        format!("({})::time::text{}", field_with_timezone, alias)
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
    pub fn build_system_where_clause(&self, table_alias: &str) -> Result<String, String> {
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
                let field_name = self.get_field_with_concatenation(
                    &normalized_entity,
                    field,
                    self.request_body.get_date_format(),
                    Some(parse_as),
                    self.timezone.as_deref(),
                    false,
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
                    let field_name = self.get_field_with_concatenation(
                        &normalized_entity,
                        field,
                        self.request_body.get_date_format(),
                        Some(parse_as),
                        self.timezone.as_deref(),
                        false,
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

        let mut where_conditions = vec![self
            .build_system_where_clause(&lateral_alias)
            .unwrap_or_else(|_| format!("({}.tombstone = 0)", lateral_alias))];

        // Add filters from the 'to' RelationEndpoint if they exist
        if !join.field_relation.to.filters.is_empty() {
            match self.build_infix_expression(&join.field_relation.to.filters) {
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
            let joins = self.request_body.get_joins();
            let mut found_alias = None;
            for j in joins {
                if let Some(to_alias) = &j.field_relation.to.alias {
                    if to_alias == from_entity {
                        found_alias = Some(to_alias.as_str());
                        break;
                    }
                }
            }
            found_alias.unwrap_or(if is_self_join {
                &self.table
            } else {
                from_entity
            })
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
        let (_table_name, field_name, field_with_table) =
            // Check if field_name contains complex expressions (like COALESCE)
            if field_name.contains("COALESCE") || field_name.contains("(") {
                // This is already a complex expression, use it as-is
                let extracted_field_name = if let Some(start) = field_name.rfind("AS ") {
                    // Extract alias if present (e.g., "COALESCE(...) AS full_name" -> "full_name")
                    field_name[start + 3..].trim().replace("\"", "")
                } else {
                    // Try to extract a meaningful name from the expression
                    field_name.replace("\"", "")
                };
                (String::new(), extracted_field_name, field_name.to_string())
            } else {
                // Handle simple field names with or without table prefix
                let mut parts = field_name.split(".");
                if let Some(first_part) = parts.next() {
                    if let Some(second_part) = parts.next() {
                        // Two parts: table.field
                        let table_name = first_part.replace("\"", "");
                        let field_name = second_part.replace("\"", "");
                        let field_with_table = format!("{}.{}", table_name, field_name);
                        (table_name, field_name, field_with_table)
                    } else {
                        // One part: just field_name - check if it's a concatenated field
                        let field_name = first_part.replace("\"", "");
                        // Check if this is a concatenated field that should be handled specially
                        let is_concatenated_field = self.request_body.get_concatenate_fields()
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
                                true
                            );
                            (String::new(), field_name, field_with_table)
                        } else {
                            // For regular fields without table prefix, assume main table
                            let field_with_table = format!("{}.{}", &self.table, field_name);
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
                    return format!(
                        "{}::text {} '%{}%'",
                        field_with_table,
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
                    return format!(
                        "{}::text {} '%{}%'",
                        field_with_table,
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
                    return format!("{}::text {} {}", field_with_table, like_op, pattern);
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
        if self.request_body.get_offset() > 0 {
            format!(" OFFSET {}", self.request_body.get_offset())
        } else {
            String::from("")
        }
    }
    pub fn construct_limit(&self) -> String {
        if self.request_body.get_limit() > 0 {
            format!(" LIMIT {}", self.request_body.get_limit())
        } else {
            String::from("LIMIT 10")
        }
    }
}
