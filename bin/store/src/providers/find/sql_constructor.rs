use crate::{structs::structs::{FilterCriteria, FilterOperator, GetByFilter, Join, LogicalOperator}};

#[derive(Debug, Clone)]
enum Token {
    Condition(String),
    And,
    Or,
}

pub struct SQLConstructor {
    request_body: GetByFilter,
    table: String,
    organization_id: Option<String>,
}

impl SQLConstructor {
    pub fn new(request_body: GetByFilter, table: String) -> Self {
        Self {
            request_body,
            table,
            organization_id: None,
        }
    }
    
    pub fn with_organization_id(mut self, organization_id: String) -> Self {
        self.organization_id = Some(organization_id);
        self
    }

    pub fn construct(&self) -> Result<String, String> {
        let mut sql = String::from("SELECT ");
        // TODO: suggestions for adding correct parameters
        // TODO: group by selections
            // TODO: concantenated fields
            // TODO: pluck_group_object need to fix by jean
        sql.push_str(&self.construct_selections());

        sql.push_str(" FROM ");
        sql.push_str(&self.table);
        // TODO: set join selections
            // TODO: concantenated fields
        sql.push_str(&self.construct_joins());
        // TODO: set Where Clauses
            // TODO: concantenated fields
        sql.push_str(&self.construct_where_clauses()?);
        // TODO: set Group By
            // TODO: concantenated fields
        sql.push_str(&self.construct_group_by());
        // TODO: set Order By
        // TODO: multiple sort
            // TODO: concantenated fields
        sql.push_str(&self.construct_order_by());
        sql.push_str(&self.construct_offset());
        sql.push_str(&self.construct_limit());
        dbg!(&sql);
        Ok(sql)
    }

    fn get_field(table: &str, field: &str, format_str: &str) -> String {
        // TODO: apply permissions
        // TODO: apply concantenated fields
        if field.contains("_date") {
            Self::date_format_wrapper(table, field, Some(format_str))
        } else {
            format!("\"{}\".\"{}\"", table, field)
        }
    }
    fn date_format_wrapper(table: &str, field: &str, format_str: Option<&str>) -> String {
        let format = format_str.unwrap_or("mm/dd/YYYY");
        format!("Coalesce(TO_CHAR(\"{}\".\"{}\"::DATE, '{}'), '')", table, field, format)
    }
    fn construct_selections(&self) -> String {
        let mut selections = Vec::new();
        
        // Add join selections from pluck_object if joins are present
        if !self.request_body.joins.is_empty() && !self.request_body.pluck_object.is_empty() && self.request_body.pluck_object.iter().any(|(_, fields)| !fields.is_empty()) {
            let join_selections = self.construct_join_selections();
            if !join_selections.is_empty() {
                selections.extend(join_selections);
            }
        }
        // set pluck as selections
        else if !self.request_body.pluck.is_empty() {
            let pluck = self.construct_pluck();
            if !pluck.is_empty() {
                selections.push(pluck);
            }
        }
        // set pluck group object as selections
        if !self.request_body.pluck_group_object.is_empty() {
            let pluck_group_object = self.construct_pluck_group_object();
            if !pluck_group_object.is_empty() {
                selections.push(pluck_group_object);
            }
        }

        if selections.is_empty() {
            "*".to_string()
        } else {
            selections.join(", ")
        }
    }

    fn construct_pluck(&self) -> String {
        let mut pluck = String::new();
        for field in &self.request_body.pluck {
            pluck.push_str(&Self::get_field(&self.table, field, &self.request_body.date_format));
            pluck.push_str(", ");
        }
        pluck.trim_end_matches(", ").to_string()
    }
    fn construct_pluck_group_object(&self) -> String {
        let mut selections = Vec::new();
        // Check if pluck_group_object exists and is not empty
        // Iterate over each key-value pair in pluck_group_object
        for (table_alias, fields) in &self.request_body.pluck_group_object {
            // For each field in the fields vector, create a JSONB_AGG statement
            for field in fields {
                let selection = format!(
                    "JSONB_AGG(\"{}\".\"{}\") AS \"{}_{}\"",
                    table_alias,
                    field,
                    table_alias,
                    field
                );
                selections.push(selection);
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
        
        // Only construct selections if joins are present
        if self.request_body.joins.is_empty() {
            return join_selections;
        }
        
        // Get organization_id for WHERE clause
        let organization_id = self.organization_id.as_ref()
            .map(|id| format!("'{}'", id))
            .unwrap_or_else(|| "NULL".to_string());
        
        // Iterate through each alias in pluck_object
        for (alias, fields) in &self.request_body.pluck_object {
            // Find the corresponding join to get table information
            if let Some(join) = self.find_join_by_alias(alias) {
                let target_table = &join.field_relation.to.entity;
                let join_condition = self.build_join_condition_for_alias(alias, join);
                
                // Build JSONB_BUILD_OBJECT field pairs
                let mut field_pairs: Vec<String> = fields.iter()
                    .map(|field| format!("'{}', {}", field, Self::get_field(alias, field, &self.request_body.date_format)))
                    .collect();

                if !self.request_body.concatenate_fields.is_empty() {
                    self.request_body.concatenate_fields.iter().for_each(|field| {
                        let concatenated_expression = field.fields.iter()
                              .map(|f| Self::get_field(&field.entity, f, &self.request_body.date_format))
                              .collect::<Vec<_>>()
                              .join(&format!(" || '{}' || ", field.separator));
                        field_pairs.push(format!("'{}', ({})", field.field_name, concatenated_expression));
                    });
                }
                
                let selection = format!(
                    "COALESCE((SELECT JSONB_AGG(JSONB_BUILD_OBJECT({})) FROM \"{}\" \"{}\" WHERE (\"{}\".\"tombstone\" = 0 AND \"{}\".\"organization_id\" IS NOT NULL AND \"{}\".\"organization_id\" = {}) AND {}), '[]') AS \"{}\"",
                    field_pairs.join(", "),
                    target_table,
                    alias,
                    alias,
                    alias,
                    alias,
                    organization_id,
                    join_condition,
                    alias
                );
                
                join_selections.push(selection);
            } else {
                fields.iter().for_each(|field| {
                    join_selections.push(format!("\"{}\".\"{}\"", alias, field));
                });
            }
        }
        
        join_selections
    }
    
    fn find_join_by_alias(&self, alias: &str) -> Option<&Join> {
        self.request_body.joins.iter()
            .find(|join| join.field_relation.to.alias.as_deref() == Some(alias))
    }
    
    fn build_join_condition_for_alias(&self, alias: &str, join: &Join) -> String {
        let from_field = &join.field_relation.from.field;
        let to_field = &join.field_relation.to.field;
        format!("\"{}\".\"{}\" = \"{}\".\"{}\"" , self.table, from_field, alias, to_field)
    }
    
    fn construct_joins(&self) -> String {
        if self.request_body.joins.is_empty() {
            String::from("")
        } else {
            let mut join_clauses = Vec::new();
            
            for join in &self.request_body.joins {
                match join.r#type.as_str() {
                    "left" => {
                        let join_clause = self.build_left_join_lateral(join);
                        join_clauses.push(join_clause);
                    }
                    "self" => {
                        // Handle self joins if needed
                        // TODO: Implement self join logic
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
    fn construct_where_clauses(&self) -> Result<String, String> {
        // add default constraints here
        let mut base_where = format!(" WHERE (tombstone = 0 AND organization_id = '{}')", 
            self.organization_id.as_ref().ok_or("Organization ID is required")?);

        if !self.request_body.advance_filters.is_empty() {
            let expression = self.build_infix_expression(&self.request_body.advance_filters)?;
            if !expression.is_empty() {
                base_where.push_str(" AND ");
                base_where.push_str(&expression);
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
             return Err("Invalid filter sequence: first filter must be a criteria, not an operator".to_string());
         }
         
         // Handle single criteria case
        if filters.len() == 1 {
            if let FilterCriteria::Criteria { field, entity, operator, values } = &filters[0] {
                let field_name = Self::get_field(entity, field, &self.request_body.date_format);
                return Ok(self.format_condition(&field_name, operator, values));
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
                FilterCriteria::Criteria { field, entity, operator, values } => {
                    let field_name = Self::get_field(entity, field, &self.request_body.date_format);
                    let condition = self.format_condition(&field_name, operator, values);
                    tokens.push(Token::Condition(condition));
                    i += 1;
                },
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
                (0, Token::Condition(_)) => {}, // Even indices should be conditions
                (1, Token::And | Token::Or) => {}, // Odd indices should be operators
                _ => return false,
            }
        }
        
        // Must end with a condition (even number of tokens)
        tokens.len() % 2 == 1
    }

    fn build_left_join_lateral(&self, join: &Join) -> String {
        let to_entity = &join.field_relation.to.entity;
        let to_alias = join.field_relation.to.alias.as_deref().unwrap_or("");
        let to_field = &join.field_relation.to.field;
        // TODO: revisit this
        // let from_entity = &join.field_relation.from.entity;
        let from_field = &join.field_relation.from.field;
        // TODO: Add nested join logic after jean fix the issue from Typescript datastore
        
        // Build the lateral subquery alias
        let lateral_alias = format!("joined_{}", to_alias);
        
        // Use organization_id from the constructor if available, otherwise use a placeholder
        let organization_id = match &self.organization_id {
            Some(id) => format!("'{}'", id),
            None => "".to_string(),
        };
        
        // Build dynamic field selection based on pluck_object
        let selected_fields = if let Some(fields) = self.request_body.pluck_object.get(to_alias) {
            fields.iter()
                .map(|field| format!("\"{}\".\"{}\"" , lateral_alias, field))
                .collect::<Vec<_>>()
                .join(", ")
        } else {
            // Default fallback fields if no pluck_object configuration found
            format!("\"{}\".\"id\"", lateral_alias)
        };
        
        format!(
            "LEFT JOIN LATERAL (SELECT {} FROM \"{}\" \"{}\" WHERE (\"{}\".tombstone = 0 AND \"{}\".organization_id IS NOT NULL AND \"{}\".organization_id = {}) AND \"{}\".\"{}\" = \"{}\".\"{}\" ) AS \"{}\" ON TRUE",
            selected_fields,
            to_entity, lateral_alias,
            lateral_alias, lateral_alias, lateral_alias, organization_id,
            self.table, from_field, lateral_alias, to_field,
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
                },
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
    fn format_condition(&self, field_name: &str, operator: &FilterOperator, values: &[serde_json::Value]) -> String {
        let values_str = values.iter()
            .map(|v| match v {
                serde_json::Value::String(s) => format!("'{}'", s.replace("'", "''")), // Escape single quotes
                serde_json::Value::Number(n) => n.to_string(),
                serde_json::Value::Bool(b) => b.to_string(),
                serde_json::Value::Null => "NULL".to_string(),
                _ => format!("'{}'", v.to_string().replace("'", "''"))
            })
            .collect::<Vec<_>>();
            
        match operator {
            FilterOperator::Equal => {
                if values_str.len() == 1 {
                    format!("{} = {}", field_name, values_str[0])
                } else {
                    format!("{} IN ({})", field_name, values_str.join(", "))
                }
            },
            FilterOperator::NotEqual => {
                if values_str.len() == 1 {
                    format!("{} != {}", field_name, values_str[0])
                } else {
                    format!("{} NOT IN ({})", field_name, values_str.join(", "))
                }
            },
            FilterOperator::GreaterThan => format!("{} > {}", field_name, values_str[0]),
            FilterOperator::GreaterThanOrEqual => format!("{} >= {}", field_name, values_str[0]),
            FilterOperator::LessThan => format!("{} < {}", field_name, values_str[0]),
            FilterOperator::LessThanOrEqual => format!("{} <= {}", field_name, values_str[0]),
            FilterOperator::IsNull => format!("{} IS NULL", field_name),
            FilterOperator::IsNotNull => format!("{} IS NOT NULL", field_name),
            FilterOperator::Contains => format!("{} LIKE '%{}%'", field_name, values_str[0].trim_matches('\'')),
            FilterOperator::NotContains => format!("{} NOT LIKE '%{}%'", field_name, values_str[0].trim_matches('\'')),
            FilterOperator::IsBetween => {
                if values_str.len() >= 2 {
                    format!("{} BETWEEN {} AND {}", field_name, values_str[0], values_str[1])
                } else {
                    format!("{} = {}", field_name, values_str[0])
                }
            },
            FilterOperator::IsNotBetween => {
                if values_str.len() >= 2 {
                    format!("{} NOT BETWEEN {} AND {}", field_name, values_str[0], values_str[1])
                } else {
                    format!("{} != {}", field_name, values_str[0])
                }
            },
            FilterOperator::IsEmpty => format!("{} = ''", field_name),
            FilterOperator::IsNotEmpty => format!("{} != ''", field_name),
            FilterOperator::Like => format!("{} LIKE {}", field_name, values_str[0]),
        }
    }
    fn construct_order_by(&self) -> String {
        // Check if multiple_sort is available and not empty
        if !self.request_body.multiple_sort.is_empty() {
            let sort_clauses: Vec<String> = self.request_body.multiple_sort
                .iter()
                .map(|sort_option| {
                    let field_parts: Vec<&str> = sort_option.by_field.split('.').collect();
                    let (table_alias, field_name) = if field_parts.len() == 2 {
                        (field_parts[0], field_parts[1])
                    } else {
                        (self.table.as_str(), sort_option.by_field.as_str())
                    };
                    
                    let field_expression = Self::get_field(table_alias, field_name, &self.request_body.date_format);
                    
                    // Handle case sensitivity
                    let final_field = if !sort_option.is_case_sensitive_sorting {
                        format!("LOWER({})", field_expression)
                    } else {
                        field_expression
                    };
                    
                    format!("{} {}", final_field, sort_option.by_direction.to_uppercase())
                })
                .collect();
            
            format!(" ORDER BY {}", sort_clauses.join(", "))
        }
        // Fallback to single field sorting if multiple_sort is empty
        else if !self.request_body.order_by.is_empty() {
            format!(" ORDER BY {} {}", Self::get_field(&self.table, "id", &self.request_body.date_format), self.request_body.order_direction)
        } else {
            String::from("")
        }
    }
    fn construct_group_by(&self) -> String {
        if !self.request_body.pluck_group_object.is_empty() {
            if !self.request_body.group_by.is_empty() {
                if let Some(first_key) = self.request_body.group_by.keys().next() {
                    format!(" GROUP BY {}", Self::get_field(&self.table, first_key, &self.request_body.date_format))
                } else {
                    String::from("")
                }
            } else {
                format!(" GROUP BY {}", Self::get_field(&self.table, "id", &self.request_body.date_format))
            }
        } else {
            String::from("")
        }
    }
    fn construct_offset(&self) -> String {
        if self.request_body.offset > 0 {
            format!(" OFFSET {}", self.request_body.offset)
        } else {
            String::from("")
        }
    }
    fn construct_limit(&self) -> String {
        if self.request_body.limit > 0 {
            format!(" LIMIT {}", self.request_body.limit)
        } else {
            String::from("")
        }
    }
}
