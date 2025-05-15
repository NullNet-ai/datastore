use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum FilterCriteria {
    #[serde(rename = "criteria")]
    Criteria {
        field: String,
        operator: FilterOperator,
        values: Vec<serde_json::Value>,
    },
    #[serde(rename = "operator")]
    LogicalOperator { operator: FilterOperator },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FilterOperator {
    #[serde(rename = "equal")]
    Equal,
    #[serde(rename = "not_equal")]
    NotEqual,
    #[serde(rename = "greater_than")]
    GreaterThan,
    #[serde(rename = "greater_than_or_equal")]
    GreaterThanOrEqual,
    #[serde(rename = "less_than")]
    LessThan,
    #[serde(rename = "less_than_or_equal")]
    LessThanOrEqual,
    #[serde(rename = "is_null")]
    IsNull,
    #[serde(rename = "is_not_null")]
    IsNotNull,
    #[serde(rename = "contains")]
    Contains,
    #[serde(rename = "not_contains")]
    NotContains,
    #[serde(rename = "is_between")]
    IsBetween,
    #[serde(rename = "is_not_between")]
    IsNotBetween,
    #[serde(rename = "is_empty")]
    IsEmpty,
    #[serde(rename = "is_not_empty")]
    IsNotEmpty,
    #[serde(rename = "and")]
    And,
    #[serde(rename = "or")]
    Or,
    #[serde(rename = "like")]
    Like,
}
