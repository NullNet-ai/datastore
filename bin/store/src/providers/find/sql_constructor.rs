use crate::structs::structs::{GetByFilter, FilterCriteria, LogicalOperator, FilterOperator};

#[derive(Debug, Clone)]
enum Token {
    Condition(String),
    And,
    Or,
}

pub struct SQLConstructor {
    request_body: GetByFilter,
    table: String,
}

impl SQLConstructor {
    pub fn new(request_body: GetByFilter, table: String) -> Self {
        Self {
            request_body,
            table,
        }
    }

    pub fn construct(&self) -> String {
        let mut sql = String::from("SELECT ");

        // TODO: group by selections
        sql.push_str(&self.construct_selections());

        sql.push_str(" FROM ");
        sql.push_str(&self.table);
        // TODO: set join selections
        sql.push_str(&self.construct_joins());
        // TODO: set Where Clauses
        sql.push_str(&self.construct_where_clauses());
        // TODO: set Group By
        sql.push_str(&self.construct_group_by());
        // TODO: set Order By
        sql.push_str(&self.construct_order_by());
        // TODO: set Offset
        sql.push_str(&self.construct_offset());
        // TODO: set Limit
        sql.push_str(&self.construct_limit());

        sql
    }

    fn get_field(table: &str, field: &str) -> String {
        format!("{}.{}", table, field)
    }
    fn construct_selections(&self) -> String {
        let mut selections = String::from("");
        // set pluck as selections
        if !self.request_body.pluck.is_empty() {
            for (i, field) in self.request_body.pluck.iter().enumerate() {
                if i > 0 {
                    selections.push_str(", ");
                }
                selections.push_str(&Self::get_field(&self.table, field));
            }
        }
        // TODO: set concatenated fields
        selections
    }
    fn construct_joins(&self) -> String {
        if self.request_body.joins.is_empty() {
            String::from("")
        } else {
            // TODO: implement join construction for Vec<Join>
            String::from("")
        }
    }
    fn construct_where_clauses(&self) -> String {
        if !self.request_body.advance_filters.is_empty() {
            let expression = self.build_infix_expression(&self.request_body.advance_filters);
            if expression.is_empty() {
                String::from("")
            } else {
                format!(" WHERE {}", expression)
            }
        } else {
            String::from("")
        }
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
    fn build_infix_expression(&self, filters: &[FilterCriteria]) -> String {
        if filters.is_empty() {
            return String::new();
        }
        
        // Handle single criteria case
        if filters.len() == 1 {
            if let FilterCriteria::Criteria { field, entity, operator, values } = &filters[0] {
                let field_name = Self::get_field(entity, field);
                return self.format_condition(&field_name, operator, values);
            }
            return String::new();
        }
        
        // Parse the filter array into tokens
        let tokens = self.parse_filter_tokens(filters);
        if tokens.is_empty() {
            return String::new();
        }
        
        // Build expression with proper precedence (AND > OR)
        self.build_expression_with_precedence(&tokens)
    }
    
    fn parse_filter_tokens(&self, filters: &[FilterCriteria]) -> Vec<Token> {
        let mut tokens = Vec::new();
        let mut i = 0;
        
        while i < filters.len() {
            match &filters[i] {
                FilterCriteria::Criteria { field, entity, operator, values } => {
                    let field_name = Self::get_field(entity, field);
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
            return Vec::new();
        }
        
        tokens
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
            _ => format!("{} = {}", field_name, values_str[0]) // Default fallback
        }
    }
    fn construct_order_by(&self) -> String {
        if !self.request_body.order_by.is_empty() {
            format!(" ORDER BY {} {}", Self::get_field(&self.table, "id"), self.request_body.order_direction)
        } else {
            String::from("")
        }
    }
    fn construct_group_by(&self) -> String {
        if !self.request_body.group_by.is_empty() {
            if let Some(first_key) = self.request_body.group_by.keys().next() {
                format!(" GROUP BY {}", Self::get_field(&self.table, first_key))
            } else {
                String::from("")
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
