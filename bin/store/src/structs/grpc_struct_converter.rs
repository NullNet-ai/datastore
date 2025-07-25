use crate::generated::store;
use crate::structs::structs::{
    Aggregation, AggregationOrder, AggregationType, FieldRelation, FilterCriteria, FilterOperator,
    Join, LogicalOperator, MatchPattern, RelationEndpoint,
};

/// Convert protobuf FilterCriteria to internal FilterCriteria struct
pub fn convert_filter_criteria(proto_filter: &store::FilterCriteria) -> Option<FilterCriteria> {
    use store::filter_criteria::FilterType;

    match &proto_filter.filter_type {
        Some(FilterType::Criteria(criteria)) => {
            let operator = match criteria.operator {
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

            let match_pattern = criteria.match_pattern.and_then(|mp| match mp {
                0 => Some(MatchPattern::Exact),
                1 => Some(MatchPattern::Prefix),
                2 => Some(MatchPattern::Suffix),
                3 => Some(MatchPattern::Contains),
                4 => Some(MatchPattern::Custom),
                _ => None,
            });

            let values: Vec<serde_json::Value> = criteria
                .values
                .iter()
                .filter_map(|v| serde_json::from_str(v).ok())
                .collect();

            Some(FilterCriteria::Criteria {
                field: criteria.field.clone(),
                entity: criteria.entity.clone(),
                operator,
                values,
                case_sensitive: criteria.case_sensitive,
                parse_as: criteria.parse_as.clone().unwrap_or_default(),
                match_pattern,
            })
        }
        Some(FilterType::LogicalOperator(logical_op)) => {
            let operator = match logical_op.operator {
                0 => LogicalOperator::And,
                1 => LogicalOperator::Or,
                _ => LogicalOperator::And, // Default fallback
            };
            Some(FilterCriteria::LogicalOperator { operator })
        }
        None => None,
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
                })
                .unwrap_or(RelationEndpoint {
                    entity: String::new(),
                    field: String::new(),
                    alias: None,
                    order_direction: None,
                    order_by: None,
                    limit: None,
                    offset: None,
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
                })
                .unwrap_or(RelationEndpoint {
                    entity: String::new(),
                    field: String::new(),
                    alias: None,
                    order_direction: None,
                    order_by: None,
                    limit: None,
                    offset: None,
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
            },
            from: RelationEndpoint {
                entity: String::new(),
                field: String::new(),
                alias: None,
                order_direction: None,
                order_by: None,
                limit: None,
                offset: None,
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
