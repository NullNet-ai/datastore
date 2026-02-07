use crate::utils::structs::{FilterCriteria, FilterOperator};

#[derive(Debug)]
pub struct SqlFilter {
    pub sql: String,
    pub params: Vec<serde_json::Value>,
}

pub fn build_sql_filter(filters: &[FilterCriteria]) -> Result<SqlFilter, Box<dyn std::error::Error>> {
    let mut sql_parts = Vec::new();
    let mut params = Vec::new();
    let mut param_index = 1;

    let mut last_logical = None;

    sql_parts.push("(".to_string());
    // ! refactor to IS NOT NULL
    sql_parts.push("organization_id IS NOT NULL".to_string());
    sql_parts.push("AND".to_string());
    sql_parts.push("tombstone = 0".to_string());
    sql_parts.push(")".to_string());

    if !filters.is_empty() {
        sql_parts.push("AND".to_string());
        sql_parts.push("(".to_string());

        let mut first_filter = true;
        for filter in filters {
            if !first_filter {
                // Don't add logical operator before the first filter
                match filter {
                    FilterCriteria::LogicalOperator { operator } => {
                        // Only AND/OR supported as logical operators
                        match operator {
                            FilterOperator::And => sql_parts.push("AND".to_string()),
                            FilterOperator::Or => sql_parts.push("OR".to_string()),
                            _ => {}
                        }
                        last_logical = Some(operator.clone());
                        continue;
                    }
                    _ => {
                        // If no explicit logical operator is provided between criteria,
                        // default to AND
                        if last_logical.is_none() {
                            sql_parts.push("AND".to_string());
                        }
                    }
                }
            }

            match filter {
                FilterCriteria::Criteria {
                    field,
                    operator,
                    values,
                } => {
                    match criteria_to_sql(field, operator, values, param_index) {
                        Ok((sql, mut vals, next_param_index)) => {
                            sql_parts.push(sql);
                            params.append(&mut vals);
                            param_index = next_param_index;
                            first_filter = false;
                            last_logical = None;
                        }
                        Err(e) => {
                            log::error!("Failed to build SQL for criteria: {}", e);
                            return Err(e);
                        }
                    }
                }
                FilterCriteria::LogicalOperator { .. } => {
                    // Already handled above
                    if first_filter {
                        // Skip logical operator if it's the first item
                        continue;
                    }
                }
            }
        }

        sql_parts.push(")".to_string());
    }

    Ok(SqlFilter {
        sql: sql_parts.join(" "),
        params,
    })
}

fn criteria_to_sql(
    field: &str,
    operator: &FilterOperator,
    values: &[serde_json::Value],
    param_index: usize,
) -> Result<(String, Vec<serde_json::Value>, usize), Box<dyn std::error::Error>> {
    use FilterOperator::*;
    let mut params = Vec::new();
    let sql = match operator {
        Equal => {
            params.push(values[0].clone());
            format!("{} = ${}", field, param_index)
        }
        NotEqual => {
            params.push(values[0].clone());
            format!("{} <> ${}", field, param_index)
        }
        GreaterThan => {
            params.push(values[0].clone());
            format!("{} > ${}", field, param_index)
        }
        GreaterThanOrEqual => {
            params.push(values[0].clone());
            format!("{} >= ${}", field, param_index)
        }
        LessThan => {
            params.push(values[0].clone());
            format!("{} < ${}", field, param_index)
        }
        LessThanOrEqual => {
            params.push(values[0].clone());
            format!("{} <= ${}", field, param_index)
        }
        IsNull => format!("{} IS NULL", field),
        IsNotNull => format!("{} IS NOT NULL", field),
        Contains => {
            let array_value = serde_json::Value::Array(values.to_vec());
            params.push(array_value);
            format!("{} && ${}", field, param_index)
        }
        NotContains => {
            params.push(values[0].clone());
            format!("{} <> ALL(${})", field, param_index)
        }
        IsBetween => {
            params.push(values[0].clone());
            params.push(values[1].clone());
            format!(
                "{} BETWEEN ${} AND ${}",
                field,
                param_index,
                param_index + 1
            )
        }
        IsNotBetween => {
            params.push(values[0].clone());
            params.push(values[1].clone());
            format!(
                "{} NOT BETWEEN ${} AND ${}",
                field,
                param_index,
                param_index + 1
            )
        }
        IsEmpty => format!("{} = ''", field),
        IsNotEmpty => format!("{} <> ''", field),
        Like => {
            params.push(values[0].clone());
            format!("{} ILIKE ${}", field, param_index)
        }
        _ => {
            log::error!("Unsupported operator in criteria_to_sql: {:?}", operator);
            return Err(format!("Unsupported operator: {:?}", operator).into());
        }
    };
    let params_len = params.len();
    Ok((sql, params, param_index + params_len))
}
