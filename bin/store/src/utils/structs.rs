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


impl FilterCriteria {
        pub fn build_from_conflict_columns(conflict_columns: Vec<String>, data: &serde_json::Value) -> Result<Vec<FilterCriteria>, String> {
            // First validate that all conflict columns exist in data
            let missing_columns: Vec<String> = conflict_columns
                .iter()
                .filter(|col| data.get(col).is_none())
                .cloned()
                .collect();
                
            if !missing_columns.is_empty() {
                return Err(format!(
                    "Missing required conflict columns in data: {}",
                    missing_columns.join(", ")
                ));
            }
            
            let mut filters = Vec::new();
            
            // Build criteria for each column
            for (index, column) in conflict_columns.iter().enumerate() {
                // Add AND operator between criteria (except for the first one)
                if index > 0 {
                    filters.push(FilterCriteria::LogicalOperator {
                        operator: FilterOperator::And
                    });
                }
                
                // We can safely unwrap here because we already validated the existence
                let value = data.get(column).unwrap();
                filters.push(FilterCriteria::Criteria {
                    field: column.to_string(),
                    operator: FilterOperator::Equal,
                    values: vec![value.clone()]
                });
            }
            
            Ok(filters)
        }
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
