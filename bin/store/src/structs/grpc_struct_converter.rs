use crate::generated::store;
use crate::structs::core::{
    Aggregation, AggregationOrder, AggregationType, FieldRelation, FilterCriteria, FilterOperator,
    Join, LogicalOperator, MatchPattern, RelationEndpoint,
};

/// Convert protobuf FilterCriteria to internal FilterCriteria struct
pub fn convert_filter_criteria(proto_filter: &store::FilterCriteria) -> Option<FilterCriteria> {
    // Debug log to see what we're receiving
    log::info!("Converting filter criteria: {:?}", proto_filter);

    match proto_filter.r#type.as_str() {
        "criteria" => {
            let operator = match proto_filter.operator.unwrap_or(0) {
                0 => FilterOperator::Equal,
                1 => FilterOperator::NotEqual,
                2 => FilterOperator::GreaterThan,
                3 => FilterOperator::GreaterThanOrEqual,
                4 => FilterOperator::LessThan,
                5 => FilterOperator::LessThanOrEqual,
                6 => FilterOperator::IsNull,
                7 => FilterOperator::IsNotNull,
                8 => FilterOperator::Contains,
                9 => FilterOperator::NotContains,
                10 => FilterOperator::Like,
                11 => FilterOperator::IsBetween,
                12 => FilterOperator::IsNotBetween,
                13 => FilterOperator::IsEmpty,
                14 => FilterOperator::IsNotEmpty,
                15 => FilterOperator::HasNoValue,
                _ => FilterOperator::Equal, // Default fallback
            };

            let match_pattern = proto_filter.match_pattern.and_then(|mp| match mp {
                0 => Some(MatchPattern::Exact),
                1 => Some(MatchPattern::Prefix),
                2 => Some(MatchPattern::Suffix),
                3 => Some(MatchPattern::Contains),
                4 => Some(MatchPattern::Custom),
                _ => None,
            });

            let values: Vec<serde_json::Value> = proto_filter
                .values
                .iter()
                .map(|v| {
                    log::info!("Processing value: {}", v);
                    // First try to parse as JSON, if that fails treat as a plain string
                    let parsed = serde_json::from_str(v)
                        .unwrap_or_else(|_| serde_json::Value::String(v.clone()));
                    log::info!("Parsed value: {:?}", parsed);
                    parsed
                })
                .collect();

            Some(FilterCriteria::Criteria {
                field: proto_filter.field.clone().unwrap_or_default(),
                entity: proto_filter.entity.clone(),
                operator,
                values,
                case_sensitive: Some(proto_filter.case_sensitive.unwrap_or(false)),
                parse_as: proto_filter.parse_as.clone().unwrap_or_default(),
                match_pattern,
                is_search: proto_filter.is_search,
                has_group_count: proto_filter.has_group_count,
            })
        }
        "operator" => {
            let operator_value = proto_filter.operator.unwrap_or(16); // Default to 'and'
            let operator = match operator_value {
                16 => LogicalOperator::And, // 'and' in FilterOperator enum
                17 => LogicalOperator::Or,  // 'or' in FilterOperator enum
                _ => LogicalOperator::And,  // Default fallback
            };
            Some(FilterCriteria::LogicalOperator { operator })
        }
        _ => {
            log::warn!("FilterCriteria received with invalid type: '{}'. Expected 'criteria' or 'operator'.", proto_filter.r#type);
            log::warn!("Expected structure: {{ type: 'criteria', field: 'field_name', entity: 'entity_name', operator: 0, values: ['value'] }}");
            None
        }
    }
}

/// Convert protobuf Join to internal Join struct
pub fn convert_join(proto_join: &store::Join) -> Option<Join> {
    let field_relation = proto_join
        .field_relation
        .as_ref()
        .map(|fr| {
            let to = fr
                .to
                .as_ref()
                .map(|to_endpoint| RelationEndpoint {
                    entity: to_endpoint.entity.clone(),
                    field: to_endpoint.field.clone(),
                    alias: to_endpoint.alias.clone(),
                    order_direction: to_endpoint.order_direction.clone(),
                    order_by: to_endpoint.order_by.clone(),
                    limit: to_endpoint.limit.map(|l| l as usize),
                    offset: to_endpoint.offset.map(|o| o as usize),
                    filters: to_endpoint
                        .filters
                        .iter()
                        .filter_map(convert_filter_criteria)
                        .collect(),
                })
                .unwrap_or(RelationEndpoint {
                    entity: String::new(),
                    field: String::new(),
                    alias: None,
                    order_direction: None,
                    order_by: None,
                    limit: None,
                    offset: None,
                    filters: Vec::new(),
                });

            let from = fr
                .from
                .as_ref()
                .map(|from_endpoint| RelationEndpoint {
                    entity: from_endpoint.entity.clone(),
                    field: from_endpoint.field.clone(),
                    alias: from_endpoint.alias.clone(),
                    order_direction: from_endpoint.order_direction.clone(),
                    order_by: from_endpoint.order_by.clone(),
                    limit: from_endpoint.limit.map(|l| l as usize),
                    offset: from_endpoint.offset.map(|o| o as usize),
                    filters: from_endpoint
                        .filters
                        .iter()
                        .filter_map(convert_filter_criteria)
                        .collect(),
                })
                .unwrap_or(RelationEndpoint {
                    entity: String::new(),
                    field: String::new(),
                    alias: None,
                    order_direction: None,
                    order_by: None,
                    limit: None,
                    offset: None,
                    filters: Vec::new(),
                });

            FieldRelation { to, from }
        })
        .unwrap_or(FieldRelation {
            to: RelationEndpoint {
                entity: String::new(),
                field: String::new(),
                alias: None,
                order_direction: None,
                order_by: None,
                limit: None,
                offset: None,
                filters: Vec::new(),
            },
            from: RelationEndpoint {
                entity: String::new(),
                field: String::new(),
                alias: None,
                order_direction: None,
                order_by: None,
                limit: None,
                offset: None,
                filters: Vec::new(),
            },
        });

    Some(Join {
        r#type: proto_join.r#type.clone(),
        field_relation,
        nested: proto_join.nested.unwrap_or(false),
    })
}

/// Convert protobuf Aggregation to internal Aggregation struct
pub fn convert_aggregation(proto_agg: &store::Aggregation) -> Aggregation {
    let aggregation = match proto_agg.aggregation {
        0 => AggregationType::Sum,
        1 => AggregationType::Avg,
        2 => AggregationType::Count,
        3 => AggregationType::Min,
        4 => AggregationType::Max,
        5 => AggregationType::StdDev,
        6 => AggregationType::Variance,
        7 => AggregationType::ArrayAgg,
        _ => AggregationType::Count, // Default fallback
    };

    Aggregation {
        aggregation,
        aggregate_on: proto_agg.aggregate_on.clone(),
        bucket_name: proto_agg.bucket_name.clone(),
    }
}

/// Convert protobuf AggregationOrder to internal AggregationOrder struct
pub fn convert_aggregation_order(proto_order: &store::AggregationOrder) -> AggregationOrder {
    AggregationOrder {
        order_by: proto_order.order_by.clone(),
        order_direction: proto_order.order_direction.clone(),
    }
}
