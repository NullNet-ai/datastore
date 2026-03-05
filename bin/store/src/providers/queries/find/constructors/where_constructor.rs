use crate::database::schema::verify::field_type_in_table;
use crate::structs::core::{
    FilterCriteria, FilterOperator, GroupAdvanceFilter, LogicalOperator, MatchPattern,
};
use crate::utils::helpers::{date_format_wrapper, time_format_wrapper, timestamp_format_wrapper};
use crate::utils::sql_sanitizer;
use serde_json::Value;

#[derive(Debug, Clone)]
enum Token {
    Condition(String),
    And,
    Or,
}

pub struct WhereConstructor<'a> {
    pub table: &'a str,
    pub organization_id: Option<&'a str>,
    pub is_root: bool,
    pub timezone: Option<&'a str>,
}

impl<'a> WhereConstructor<'a> {
    pub fn new(
        table: &'a str,
        organization_id: Option<&'a str>,
        is_root: bool,
        timezone: Option<&'a str>,
    ) -> Self {
        Self {
            table,
            organization_id,
            is_root,
            timezone,
        }
    }

    pub fn construct_where_clauses(
        &self,
        advance_filters: &[FilterCriteria],
        group_advance_filters: &[GroupAdvanceFilter],
        concatenate_fields: &[crate::structs::core::ConcatenateField],
        date_format: &str,
        time_format: &str,
    ) -> Result<String, String> {
        // Use the reusable standard WHERE clause pattern
        let mut base_where = format!(" WHERE {}", self.build_system_where_clause(self.table)?);

        // Prioritize advance_filters over group_advance_filters
        if !advance_filters.is_empty() {
            let expression = self.build_infix_expression(
                advance_filters,
                concatenate_fields,
                date_format,
                time_format,
            )?;
            if !expression.is_empty() {
                base_where.push_str(" AND (");
                base_where.push_str(&expression);
                base_where.push(')');
            }
        } else if !group_advance_filters.is_empty() {
            let group_expression = self.build_group_advance_filters_expression(
                group_advance_filters,
                concatenate_fields,
                date_format,
                time_format,
            )?;
            if !group_expression.is_empty() {
                base_where.push_str(" AND (");
                base_where.push_str(&group_expression);
                base_where.push(')');
            }
        }

        Ok(base_where)
    }

    pub fn build_system_where_clause(&self, table_alias: &str) -> Result<String, String> {
        // For root access, only check tombstone
        if self.is_root {
            return Ok(format!("(\"{}\".\"tombstone\" = 0)", table_alias));
        }

        // For non-root access, check organization constraints
        let organization_id = match self.organization_id {
            Some(id) => format!("'{}'", id),
            None => return Err("Organization ID is required".to_string()),
        };

        Ok(format!(
            "(\"{}\".\"tombstone\" = 0 AND \"{}\".\"organization_id\" IS NOT NULL AND \"{}\".\"organization_id\" = {})",
            table_alias, table_alias, table_alias, organization_id
        ))
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
    pub fn build_infix_expression(
        &self,
        filters: &[FilterCriteria],
        concatenate_fields: &[crate::structs::core::ConcatenateField],
        date_format: &str,
        time_format: &str,
    ) -> Result<String, String> {
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
                    .unwrap_or_else(|| self.table.to_string());
                let effective_parse_as =
                    Self::resolve_parse_as_for_filter(parse_as, operator, values);
                let field_name = self.get_field_with_concatenation(
                    &normalized_entity,
                    field,
                    date_format,
                    effective_parse_as.as_deref(),
                    self.timezone,
                    false,
                    concatenate_fields,
                    time_format,
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
        let tokens =
            self.parse_filter_tokens(filters, concatenate_fields, date_format, time_format)?;
        if tokens.is_empty() {
            return Err("Failed to parse filter tokens: invalid filter sequence".to_string());
        }

        // Build expression with proper precedence (AND > OR)
        Ok(self.build_expression_with_precedence(&tokens))
    }

    fn parse_filter_tokens(
        &self,
        filters: &[FilterCriteria],
        concatenate_fields: &[crate::structs::core::ConcatenateField],
        date_format: &str,
        time_format: &str,
    ) -> Result<Vec<Token>, String> {
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
                        .unwrap_or_else(|| self.table.to_string());
                    let effective_parse_as =
                        Self::resolve_parse_as_for_filter(parse_as, operator, values);
                    let field_name = self.get_field_with_concatenation(
                        &normalized_entity,
                        field,
                        date_format,
                        effective_parse_as.as_deref(),
                        self.timezone,
                        false,
                        concatenate_fields,
                        time_format,
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

    fn normalize_entity_name(&self, entity: &str) -> String {
        if entity == "self" {
            self.table.to_string()
        } else {
            entity.to_string()
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
        concatenate_fields: &[crate::structs::core::ConcatenateField],
        time_format: &str,
    ) -> String {
        // Check if this field is a concatenated field
        if let Some(concat_field) = concatenate_fields.iter().find(|cf| cf.field_name == field) {
            // Build concatenated field expression
            let concatenated_expr: String = concat_field
                .fields
                .iter()
                .map(|f| {
                    let field_expr = if f.contains("'") {
                        // This is a literal string
                        f.clone()
                    } else {
                        // This is a field reference
                        format!(
                            "COALESCE({}, '')",
                            Self::get_field_with_parse_as(
                                table,
                                f,
                                format_str,
                                parse_as,
                                table,
                                timezone,
                                with_alias,
                                time_format,
                            )
                        )
                    };
                    field_expr
                })
                .collect::<Vec<_>>()
                .join(&format!(" || '{}' || ", concat_field.separator));
            concatenated_expr
        } else {
            // Regular field handling
            Self::get_field_with_parse_as(
                table,
                field,
                format_str,
                parse_as,
                self.table,
                timezone,
                with_alias,
                time_format,
            )
        }
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
                } else if Self::is_timestamp_column(table, field) {
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

    fn is_timestamp_column(table: &str, field: &str) -> bool {
        if let Some(info) = field_type_in_table(table, field) {
            info.field_type == "timestamp"
        } else {
            false
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

    fn format_condition_with_case_sensitivity_and_pattern(
        &self,
        field_name: &str,
        operator: &FilterOperator,
        values: &[serde_json::Value],
        case_sensitive: Option<bool>,
        match_pattern: Option<&MatchPattern>,
    ) -> String {
        let (table_name, field_name, field_with_table) =
                // Check if field_name contains complex expressions (like COALESCE)
                if field_name.contains("COALESCE") || field_name.contains("(") || field_name.contains("::") {
                    if let Some(start) = field_name.rfind("AS ") {
                        let extracted_field_name = field_name[start + 3..].trim().replace("\"", "");
                        (String::new(), extracted_field_name, field_name.to_string())
                    } else if field_name.contains("::") {
                        // Extract base before type cast and parse table/field
                        let base = field_name.splitn(2, "::").next().unwrap_or(field_name);
                        let mut parts = base.split('.');
                        let table_part = parts.next().map(|s| s.trim().trim_matches('"').to_string());
                        let field_part = parts.next().map(|s| s.trim().trim_matches('"').to_string());
                        let tname = table_part.unwrap_or_else(|| String::new());
                        let fname = field_part.unwrap_or_else(|| {
                            base.rsplit('.')
                                .next()
                                .unwrap_or(base)
                                .trim()
                                .trim_matches('"')
                                .to_string()
                        });
                        (tname, fname, field_name.to_string())
                    } else {
                        let extracted_field_name = field_name.replace("\"", "");
                        (String::new(), extracted_field_name, field_name.to_string())
                    }
                } else {
                    // Handle simple field names with or without table prefix
                    let mut parts = field_name.split(".");
                    if let Some(first_part) = parts.next() {
                        if let Some(second_part) = parts.next() {
                            // Two parts: table.field
                            let table_name = first_part.replace("\"", "");
                            let field_name = second_part.replace("\"", "");
                            let field_with_table = format!("\"{}\".\"{}\"", table_name, field_name);
                            (table_name, field_name, field_with_table)
                        } else {
                            // One part: just field_name
                            let field_name = first_part.replace("\"", "");
                            let field_with_table = format!("\"{}\".\"{}\"", self.table, field_name);
                            (self.table.to_string(), field_name, field_with_table)
                        }
                    } else {
                        // No parts (shouldn't happen, but handle gracefully)
                        (String::new(), String::new(), String::new())
                    }
                };

        let plural_form = pluralizer::pluralize(&field_name, 2, false);
        let is_plural = plural_form == field_name;

        // Determine if this operator uses LIKE patterns (needs wildcard escaping)
        let is_like_operator = matches!(
            operator,
            FilterOperator::Contains | FilterOperator::NotContains | FilterOperator::Like
        );

        // Sanitize values using the SQL sanitizer module
        // This protects against:
        // - SQL injection (quote/comment/union attacks)
        // - NULL byte injection
        // - LIKE wildcard injection (when is_like_operator = true)
        // - Control character attacks
        let values_str: Result<Vec<String>, String> = values
            .iter()
            .map(|v| {
                sql_sanitizer::sanitize_value(v, is_like_operator)
                    .map_err(|e| format!("Value sanitization failed: {}", e))
            })
            .collect();

        let values_str = match values_str {
            Ok(vals) => vals,
            Err(e) => {
                log::error!("Failed to sanitize filter values: {}", e);
                return format!("FALSE /* {} */", e); // Return FALSE condition on sanitization failure
            }
        };

        let type_info = if !table_name.is_empty() {
            field_type_in_table(&table_name, &field_name)
        } else {
            None
        };
        let is_array_field = type_info.as_ref().map_or(false, |i| i.is_array);
        let is_json_field = type_info.as_ref().map_or(false, |i| i.is_json);
        let base_field = if field_with_table.contains("::text") {
            field_with_table
                .splitn(2, "::")
                .next()
                .unwrap_or(&field_with_table)
                .to_string()
        } else {
            field_with_table.clone()
        };

        match operator {
            FilterOperator::Equal => {
                if is_array_field && !is_json_field {
                    if values_str.len() == 1 {
                        format!("{} = ANY({})", values_str[0], base_field)
                    } else {
                        let conditions: Vec<String> = values_str
                            .iter()
                            .map(|v| format!("{} = ANY({})", v, base_field))
                            .collect();
                        format!("({})", conditions.join(" OR "))
                    }
                } else if is_json_field {
                    if values_str.len() == 1 {
                        format!("{} @> [{}]::jsonb", base_field, values_str[0])
                    } else {
                        let conditions: Vec<String> = values_str
                            .iter()
                            .map(|v| format!("{} @> [{}]::jsonb", base_field, v))
                            .collect();
                        format!("({})", conditions.join(" OR "))
                    }
                } else {
                    if values_str.len() == 1 {
                        format!("{} = {}", field_with_table, values_str[0])
                    } else {
                        format!("{} IN ({})", field_with_table, values_str.join(", "))
                    }
                }
            }
            FilterOperator::NotEqual => {
                if is_array_field && !is_json_field {
                    if values_str.len() == 1 {
                        format!("NOT ({} = ANY({}))", values_str[0], base_field)
                    } else {
                        let conditions: Vec<String> = values_str
                            .iter()
                            .map(|v| format!("NOT ({} = ANY({}))", v, base_field))
                            .collect();
                        format!("({})", conditions.join(" AND "))
                    }
                } else if is_json_field {
                    if values_str.len() == 1 {
                        format!("NOT ({} @> [{}]::jsonb)", base_field, values_str[0])
                    } else {
                        let conditions: Vec<String> = values_str
                            .iter()
                            .map(|v| format!("NOT ({} @> [{}]::jsonb)", base_field, v))
                            .collect();
                        format!("({})", conditions.join(" AND "))
                    }
                } else {
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

                if values_str.len() > 1 {
                    let conditions: Vec<String> = values_str
                        .iter()
                        .map(|v| {
                            let clean = v.trim_matches('\'');
                            if is_plural {
                                let expr = if field_with_table.contains("::text") {
                                    field_with_table.clone()
                                } else {
                                    format!("{}::text", field_with_table)
                                };
                                format!("{} {} '%{}%'", expr, like_op, clean)
                            } else {
                                format!("{} {} '%{}%'", field_with_table, like_op, clean)
                            }
                        })
                        .collect();
                    return format!("({})", conditions.join(" OR "));
                } else {
                    let clean = values_str[0].trim_matches('\'');
                    if is_plural {
                        let expr = if field_with_table.contains("::text") {
                            field_with_table.clone()
                        } else {
                            format!("{}::text", field_with_table)
                        };
                        return format!("{} {} '%{}%'", expr, like_op, clean);
                    }
                    format!("{} {} '%{}%'", field_with_table, like_op, clean)
                }
            }
            FilterOperator::NotContains => {
                let like_op = if case_sensitive.unwrap_or(true) {
                    "NOT LIKE"
                } else {
                    "NOT ILIKE"
                };

                if values_str.len() > 1 {
                    let conditions: Vec<String> = values_str
                        .iter()
                        .map(|v| {
                            let clean = v.trim_matches('\'');
                            if is_plural {
                                let expr = if field_with_table.contains("::text") {
                                    field_with_table.clone()
                                } else {
                                    format!("{}::text", field_with_table)
                                };
                                format!("{} {} '%{}%'", expr, like_op, clean)
                            } else {
                                format!("{} {} '%{}%'", field_with_table, like_op, clean)
                            }
                        })
                        .collect();
                    return format!("({})", conditions.join(" AND "));
                } else {
                    let clean = values_str[0].trim_matches('\'');
                    if is_plural {
                        let expr = if field_with_table.contains("::text") {
                            field_with_table.clone()
                        } else {
                            format!("{}::text", field_with_table)
                        };
                        return format!("{} {} '%{}%'", expr, like_op, clean);
                    }
                    format!("{} {} '%{}%'", field_with_table, like_op, clean)
                }
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
                if values_str.len() > 1 {
                    let patterns: Vec<String> = values_str
                        .iter()
                        .map(|v| self.build_like_pattern(v, match_pattern))
                        .collect();
                    let conditions: Vec<String> = patterns
                        .iter()
                        .map(|p| {
                            if is_plural {
                                let expr = if field_with_table.contains("::text") {
                                    field_with_table.clone()
                                } else {
                                    format!("{}::text", field_with_table)
                                };
                                format!("{} {} {}", expr, like_op, p)
                            } else {
                                format!("{} {} {}", field_with_table, like_op, p)
                            }
                        })
                        .collect();
                    return format!("({})", conditions.join(" OR "));
                } else {
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
    fn get_group_operator<'b>(&self, group_filter: &'b GroupAdvanceFilter) -> &'b LogicalOperator {
        match group_filter {
            GroupAdvanceFilter::Criteria { operator, .. } => operator,
            GroupAdvanceFilter::Operator { operator, .. } => operator,
        }
    }

    /// Helper method to extract filters from GroupAdvanceFilter enum
    fn get_group_filters<'b>(
        &self,
        group_filter: &'b GroupAdvanceFilter,
    ) -> &'b Vec<FilterCriteria> {
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
        concatenate_fields: &[crate::structs::core::ConcatenateField],
        date_format: &str,
        time_format: &str,
    ) -> Result<String, String> {
        if group_filters.is_empty() {
            return Ok(String::new());
        }

        if group_filters.len() == 1 {
            // Single group case - just process the filters within the group
            let group_filter = &group_filters[0];
            let filters = self.get_group_filters(group_filter);
            if !filters.is_empty() {
                let group_expression = self.build_infix_expression(
                    filters,
                    concatenate_fields,
                    date_format,
                    time_format,
                )?;
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
                let group_expression = self.build_infix_expression(
                    filters,
                    concatenate_fields,
                    date_format,
                    time_format,
                )?;
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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_detect_timestamp_columns_by_type_for_where_constructor() {
        // account_profiles.date_of_birth -> Timestamp, field name is not "timestamp"
        assert!(WhereConstructor::is_timestamp_column(
            "account_profiles",
            "date_of_birth"
        ));

        // account_profiles.timestamp -> Timestamp, classic timestamp column
        assert!(WhereConstructor::is_timestamp_column(
            "account_profiles",
            "timestamp"
        ));

        // accounts.timestamp -> Timestamptz, simplified to "timestamp" in FieldTypeInfo
        assert!(WhereConstructor::is_timestamp_column(
            "accounts",
            "timestamp"
        ));

        // Non-timestamp column should not be treated as timestamp
        assert!(!WhereConstructor::is_timestamp_column(
            "account_profiles",
            "code"
        ));
    }

    #[test]
    fn should_build_equal_for_text_array_membership() {
        let wc = WhereConstructor::new("samples", None, true, None);
        let filters = vec![FilterCriteria::Criteria {
            field: "categories".to_string(),
            entity: Some("samples".to_string()),
            operator: FilterOperator::Equal,
            values: vec![serde_json::json!("Root")],
            case_sensitive: None,
            parse_as: "text".to_string(),
            match_pattern: None,
            is_search: None,
            has_group_count: None,
        }];
        let expr = wc
            .build_infix_expression(&filters, &[], "YYYY-mm-dd", "HH24:MI:SS")
            .expect("build_infix_expression should succeed");
        assert_eq!(expr, "'Root' = ANY(\"samples\".\"categories\")");
    }

    #[test]
    fn should_build_equal_for_jsonb_array_membership() {
        let wc = WhereConstructor::new("organizations", None, true, None);
        let filters = vec![FilterCriteria::Criteria {
            field: "path_level".to_string(),
            entity: Some("organizations".to_string()),
            operator: FilterOperator::Equal,
            values: vec![serde_json::json!("Root")],
            case_sensitive: None,
            parse_as: String::new(),
            match_pattern: None,
            is_search: None,
            has_group_count: None,
        }];
        let expr = wc
            .build_infix_expression(&filters, &[], "YYYY-mm-dd", "HH24:MI:SS")
            .expect("build_infix_expression should succeed");
        assert_eq!(expr, "\"organizations\".\"path_level\" @> ['Root']::jsonb");
    }
}
