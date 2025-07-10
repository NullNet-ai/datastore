use regex::Regex;
use crate::db::create_connection;
use tokio_postgres::Client;

pub struct FunctionValidator;

impl FunctionValidator {
    pub fn new() -> Self {
        Self
    }

    pub fn has_create_function_statement(&self, function_string: &str) -> bool {
        let re = Regex::new(r"CREATE\s+OR\s+REPLACE\s+FUNCTION").unwrap();
        re.is_match(function_string)
    }

    pub fn has_return_type(&self, function_string: &str) -> bool {
        let re = Regex::new(r"RETURNS\s+\w+").unwrap();
        re.is_match(function_string)
    }

    pub fn has_language_declaration(&self, function_string: &str) -> bool {
        let re = Regex::new(r"LANGUAGE\s+\w+").unwrap();
        re.is_match(function_string)
    }

    pub fn has_balanced_parentheses(&self, function_string: &str) -> bool {
        let mut count = 0;
        for char in function_string.chars() {
            match char {
                '(' => count += 1,
                ')' => count -= 1,
                _ => {}
            }
            if count < 0 {
                return false;
            }
        }
        count == 0
    }

    pub fn has_balanced_quotes(&self, function_string: &str) -> bool {
        let mut in_single_quote = false;
        let mut in_double_quote = false;
        let mut escaped = false;

        for char in function_string.chars() {
            if escaped {
                escaped = false;
                continue;
            }

            match char {
                '\\' => escaped = true,
                '\'' if !in_double_quote => in_single_quote = !in_single_quote,
                '"' if !in_single_quote => in_double_quote = !in_double_quote,
                _ => {}
            }
        }

        !in_single_quote && !in_double_quote
    }

    pub fn has_dangerous_commands(&self, function_string: &str) -> bool {
        let dangerous_patterns = [
            r"DROP\s+TABLE",
            r"DROP\s+DATABASE",
            r"DELETE\s+FROM",
            r"TRUNCATE",
            r"ALTER\s+TABLE",
            r"GRANT\s+ALL",
            r"REVOKE\s+ALL",
        ];

        for pattern in &dangerous_patterns {
            let re = Regex::new(pattern).unwrap();
            if re.is_match(function_string) {
                return true;
            }
        }
        false
    }

    pub fn validate_channel_function_format(&self, function_string: &str) -> Result<(), String> {
        let function_name = self.extract_function_name(function_string)?;
        let channel_name = self.extract_channel_name(function_string)?;
        let json_args = self.extract_json_args(function_string)?;

        if function_name != channel_name {
            return Err("Function name must match channel name".to_string());
        }

        if !self.validate_json_args(&json_args)? {
            return Err("JSON args must include 'type' and 'channel'".to_string());
        }

        Ok(())
    }

    pub fn extract_function_name(&self, function_string: &str) -> Result<String, String> {
        let re = Regex::new(r"CREATE\s+OR\s+REPLACE\s+FUNCTION\s+([a-zA-Z0-9_]+)")
            .map_err(|e| format!("Regex error: {}", e))?;

        if let Some(captures) = re.captures(function_string) {
            if let Some(function_name) = captures.get(1) {
                return Ok(function_name.as_str().to_string());
            }
        }

        Err("Could not extract function name".to_string())
    }

    pub fn extract_channel_name(&self, function_string: &str) -> Result<String, String> {
        let re = Regex::new(r"channel\s+text\s*:=\s*'([a-zA-Z0-9_]+)'\s*;")
            .map_err(|e| format!("Regex error: {}", e))?;

        if let Some(captures) = re.captures(function_string) {
            if let Some(channel_name) = captures.get(1) {
                return Ok(channel_name.as_str().to_string());
            }
        }

        Err("Could not extract channel name".to_string())
    }

    pub fn extract_json_args(&self, function_string: &str) -> Result<String, String> {
        let re = Regex::new(r"SELECT\s+json_build_object\s*\(([\s\S]+?)\)::text")
            .map_err(|e| format!("Regex error: {}", e))?;

        if let Some(captures) = re.captures(function_string) {
            if let Some(json_args) = captures.get(1) {
                return Ok(json_args.as_str().to_string());
            }
        }

        Err("Could not extract JSON args".to_string())
    }

    pub fn validate_json_args(&self, json_args: &str) -> Result<bool, String> {
        let re = Regex::new(r#"['\"']type['\"']\s*,\s*channel"#)
            .map_err(|e| format!("Regex error: {}", e))?;
        Ok(re.is_match(json_args))
    }

    // Test function syntax by creating it in a transaction and rolling back
    pub async fn test_function_syntax(&self, function_string: &str) -> Result<bool, String> {
        let mut client = create_connection().await
            .map_err(|e| format!("Database connection error: {}", e))?;

        // Begin transaction
        let tx = client.transaction().await
            .map_err(|e| format!("Transaction error: {}", e))?;

        // Try to create the function
        let result = tx.execute(function_string, &[]).await;

        // Always rollback - we just want to test syntax
        tx.rollback().await
            .map_err(|e| format!("Rollback error: {}", e))?;

        match result {
            Ok(_) => Ok(true),
            Err(e) => Err(format!("SQL syntax error: {}", e)),
        }
    }

    // Comprehensive validation method that runs all checks
    pub async fn validate_function(&self, function_string: &str) -> Result<(), String> {
        // Basic syntax checks
        if !self.has_create_function_statement(function_string) {
            return Err("Missing CREATE OR REPLACE FUNCTION statement".to_string());
        }

        if !self.has_return_type(function_string) {
            return Err("Missing RETURNS clause".to_string());
        }

        if !self.has_language_declaration(function_string) {
            return Err("Missing LANGUAGE declaration".to_string());
        }

        if !self.has_balanced_parentheses(function_string) {
            return Err("Unbalanced parentheses".to_string());
        }

        if !self.has_balanced_quotes(function_string) {
            return Err("Unbalanced quotes".to_string());
        }

        // Security checks
        if self.has_dangerous_commands(function_string) {
            return Err("Function contains dangerous SQL commands like DROP, DELETE, etc. Please contact the system administrator for assistance.".to_string());
        }

        // Channel function format validation
        self.validate_channel_function_format(function_string)?;

        // Test actual SQL syntax
        self.test_function_syntax(function_string).await?;

        Ok(())
    }
}