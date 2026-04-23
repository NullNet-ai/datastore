use serde::Serialize;
use serde_json::Value;
use std::collections::HashMap;

use crate::structs::core::FilterCriteria;

pub struct SearchSuggestionCache {}

#[derive(Debug, Clone)]
pub struct AliasedJoinedEntity {
    #[allow(dead_code)]
    pub to_entity: String,
    pub alias: String,
}
#[derive(Debug)]
pub struct FormatFilterResponse {
    pub formatted_filters: Vec<Value>,
    pub search_term: String,
    pub filtered_fields: Value,
}

#[derive(Debug)]
pub struct FieldFiltersResult {
    pub all_field_filters: Vec<FilterCriteria>,
    pub field_filter: Option<FilterCriteria>,
}

#[derive(Debug, Clone, Serialize)]
pub struct FieldExpression {
    pub expression: String,
    pub fields: Vec<String>,
}

pub type ConcatenatedExpressions = HashMap<String, HashMap<String, FieldExpression>>;
