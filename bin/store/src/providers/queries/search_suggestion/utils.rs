use crate::providers::queries::search_suggestion::structs::{
    AliasedJoinedEntity, ConcatenatedExpressions, FieldExpression, FieldFiltersResult,
    FormatFilterResponse,
};
use crate::structs::structs::{ConcatenateField, FilterCriteria, MatchPattern};
use pluralizer::pluralize;
use serde_json::Value;
use std::collections::{HashMap, HashSet};
use std::env;

fn get_default_search_pattern() -> MatchPattern {
    match env::var("DEFAULT_SEARCH_PATTERN").as_deref() {
        Ok("exact") => MatchPattern::Exact,
        Ok("prefix") => MatchPattern::Prefix,
        Ok("suffix") => MatchPattern::Suffix,
        Ok("contains") => MatchPattern::Contains,
        Ok("custom") => MatchPattern::Custom,
        _ => MatchPattern::Contains, // fallback default
    }
}

pub fn format_filters(
    filters: Vec<FilterCriteria>,
    aliased_joined_entities: Option<&[AliasedJoinedEntity]>,
    table: &str,
    mut filtered_fields: Value,
    mut search_term: String,
) -> FormatFilterResponse {
    let mut formatted_filters = Vec::new();

    for filter in filters {
        match filter {
            FilterCriteria::Criteria {
                field,
                entity,
                operator,
                values,
                case_sensitive,
                parse_as,
                match_pattern,
                is_search,
                has_group_count,
            } => {
                let is_search = is_search.unwrap_or(false);
                let final_match_pattern = match_pattern.unwrap_or(get_default_search_pattern());

                // Set search_term if this is a search criteria
                if is_search {
                    search_term = values
                        .first()
                        .and_then(|v| v.as_str())
                        .unwrap_or("")
                        .to_string();
                }

                // Determine filtered_entity
                let mut filtered_entity = match entity {
                    Some(ref e) if !e.is_empty() => e.clone(),
                    _ => table.to_string(),
                };

                // Check if entity is aliased
                let is_aliased = aliased_joined_entities
                    .map(|entities| {
                        entities
                            .iter()
                            .any(|aliased| aliased.alias == filtered_entity)
                    })
                    .unwrap_or(false);

                // If not aliased, pluralize the entity
                if !is_aliased {
                    filtered_entity = pluralize(&filtered_entity, 2, false);
                }

                // Update filtered_fields if this is a search criteria
                if is_search {
                    filtered_fields = match filtered_fields {
                        Value::Object(mut obj) => {
                            let entry = obj
                                .entry(filtered_entity.clone())
                                .or_insert(Value::Array(vec![]));
                            if let Value::Array(arr) = entry {
                                // Convert to set for uniqueness
                                let mut unique_fields: HashSet<String> = arr
                                    .iter()
                                    .filter_map(|v| v.as_str().map(String::from))
                                    .collect();
                                unique_fields.insert(field.clone());
                                *arr = unique_fields.into_iter().map(Value::String).collect();
                            }
                            Value::Object(obj)
                        }
                        _ => filtered_fields,
                    };
                }

                let criteria = FilterCriteria::Criteria {
                    field,
                    entity: Some(filtered_entity),
                    operator,
                    values,
                    case_sensitive,
                    parse_as,
                    match_pattern: Some(final_match_pattern),
                    is_search: Some(is_search),
                    has_group_count,
                };

                formatted_filters.push(serde_json::to_value(criteria).unwrap_or(Value::Null));
            }
            FilterCriteria::LogicalOperator { operator } => {
                let logical_op = FilterCriteria::LogicalOperator { operator };
                formatted_filters.push(serde_json::to_value(logical_op).unwrap_or(Value::Null));
            }
        }
    }

    FormatFilterResponse {
        formatted_filters,
        search_term,
        filtered_fields,
    }
}

pub fn get_field_filters(
    filters: Vec<FilterCriteria>,
    field: &str,
    entity: &str,
    search_term: &str,
) -> FieldFiltersResult {
    let mut all_field_filters = Vec::new();
    let mut field_filter = None;

    for (index, filter) in filters.iter().enumerate() {
        let filter_value = match filter {
            FilterCriteria::Criteria { values, .. } => Value::Array(values.clone()),
            _ => Value::Array(vec![]),
        };
        let search_term_value = Value::Array(vec![Value::String(search_term.to_string())]);

        // Check if current filter is an operator and follows criteria pattern
        let is_operator_after_criteria = matches!(filter, FilterCriteria::LogicalOperator { .. })
            && all_field_filters
                .last()
                .map(|last| matches!(last, FilterCriteria::Criteria { .. }))
                .unwrap_or(false);

        let next_is_non_search_criteria = filters.get(index + 1)
            .map(|next| {
                matches!(next, FilterCriteria::Criteria { is_search, .. } if !is_search.unwrap_or(false))
            })
            .unwrap_or(false);

        let last_is_non_search_criteria = all_field_filters.last()
            .map(|last| {
                matches!(last, FilterCriteria::Criteria { is_search, .. } if !is_search.unwrap_or(false))
            })
            .unwrap_or(false);

        if is_operator_after_criteria
            && (next_is_non_search_criteria || last_is_non_search_criteria)
        {
            all_field_filters.push(filter.clone());
        } else if let FilterCriteria::Criteria {
            entity: ref e,
            field: ref f,
            is_search,
            ..
        } = filter
        {
            let is_search = is_search.unwrap_or(false);
            if e.as_deref() == Some(entity) && f == field && is_search {
                field_filter = Some(filter.clone());
                all_field_filters.push(filter.clone());
            } else if filter_value != search_term_value && !is_search {
                all_field_filters.push(filter.clone());
            }
        }
    }

    FieldFiltersResult {
        all_field_filters,
        field_filter,
    }
}

pub fn generate_concatenated_expressions(
    concatenate_fields: Vec<ConcatenateField>,
    date_format: Option<&str>,
) -> ConcatenatedExpressions {
    let default_date_format = "mm/dd/YYYY";
    let date_fmt = date_format.unwrap_or(default_date_format);

    concatenate_fields
        .into_iter()
        .fold(HashMap::new(), |mut acc, field| {
            let entity = field
                .aliased_entity
                .unwrap_or_else(|| pluralize(&field.entity, 2, false));

            let concatenated_expression = format!(
                "({})",
                field
                    .fields
                    .iter()
                    .map(|f| {
                        if f.ends_with("_date") {
                            format!(
                                "COALESCE(to_char(\"{}\".\"{}\"::date, '{}'), '')",
                                entity, f, date_fmt
                            )
                        } else {
                            format!("COALESCE(\"{}\".\"{}\" , '')", entity, f)
                        }
                    })
                    .collect::<Vec<_>>()
                    .join(&format!(" || '{}' || ", field.separator))
            );

            let field_expr = FieldExpression {
                expression: concatenated_expression,
                fields: field.fields.clone(),
            };

            acc.entry(entity)
                .or_insert_with(HashMap::new)
                .insert(field.field_name, field_expr);

            acc
        })
}
