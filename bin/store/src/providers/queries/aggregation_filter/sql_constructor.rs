use crate::{
    providers::queries::find::{sql_constructor::QueryFilter, SQLConstructor},
    structs::core::{Aggregation, AggregationFilter, AggregationOrder, FilterCriteria, Join},
    structs::grpc_struct_converter::{
        convert_aggregation, convert_aggregation_order, convert_filter_criteria, convert_join,
    },
};

// Trait specifically for aggregation query filters
pub trait AggregationQueryFilter {
    #[allow(dead_code)]
    fn get_advance_filters(&self) -> &[FilterCriteria] {
        &[]
    }
    #[allow(dead_code)]
    fn get_joins(&self) -> &[Join] {
        &[]
    }
    fn get_limit(&self) -> usize;
    #[allow(dead_code)]
    fn get_date_format(&self) -> &str {
        "mm/dd/YYYY"
    }
    fn get_aggregations(&self) -> &[Aggregation];
    fn get_bucket_size(&self) -> Option<&str>;
    fn get_timezone(&self) -> Option<&str>;
    fn get_aggregation_order(&self) -> Option<&AggregationOrder>;
    fn get_entity(&self) -> Option<&str>;
    #[allow(dead_code)]
    fn get_order_by(&self) -> &str {
        "id"
    }
    #[allow(dead_code)]
    fn get_order_direction(&self) -> &str {
        "asc"
    }
}

pub struct AggregationSQLConstructor<T: QueryFilter + Clone> {
    sql_constructor: SQLConstructor<T>,
}

impl<T> AggregationSQLConstructor<T>
where
    T: AggregationQueryFilter + QueryFilter + Clone,
{
    pub fn new(request_body: T, table: String, is_root: bool, timezone: Option<String>) -> Self {
        Self {
            sql_constructor: SQLConstructor::new(request_body, table, is_root, timezone),
        }
    }

    pub fn with_organization_id(mut self, organization_id: String) -> Self {
        self.sql_constructor = self.sql_constructor.with_organization_id(organization_id);
        self
    }

    pub fn construct_aggregation(&mut self) -> Result<String, String> {
        // Validate required parameters for aggregation
        let aggregations =
            AggregationQueryFilter::get_aggregations(&self.sql_constructor.request_body);
        let bucket_size =
            AggregationQueryFilter::get_bucket_size(&self.sql_constructor.request_body);
        let entity = AggregationQueryFilter::get_entity(&self.sql_constructor.request_body);

        if aggregations.is_empty() {
            return Err("Missing required parameter: aggregations cannot be empty".to_string());
        }

        if bucket_size.is_none() {
            return Err("Missing required parameter: bucket_size".to_string());
        }

        if entity.is_none() || entity.map_or(true, |e| e.is_empty()) {
            return Err("Missing required parameter: entity".to_string());
        }

        let bucket_size = bucket_size.unwrap();
        let entity = entity.unwrap();
        let timezone = AggregationQueryFilter::get_timezone(&self.sql_constructor.request_body)
            .unwrap_or("UTC");

        // Generate the SELECT clause with time bucket and aggregations
        let mut sql = String::from("SELECT ");

        // Add time bucket
        sql.push_str(&format!(
            "time_bucket('{}', {}.timestamp AT TIME ZONE '{}') AS bucket",
            bucket_size, entity, timezone
        ));

        // Add aggregation clauses
        for aggregation in aggregations {
            sql.push_str(", ");
            let agg_type = match aggregation.aggregation {
                crate::structs::core::AggregationType::Sum => "SUM",
                crate::structs::core::AggregationType::Count => "COUNT",
                crate::structs::core::AggregationType::Avg => "AVG",
                crate::structs::core::AggregationType::Min => "MIN",
                crate::structs::core::AggregationType::Max => "MAX",
                crate::structs::core::AggregationType::StdDev => "STDDEV",
                crate::structs::core::AggregationType::Variance => "VARIANCE",
                crate::structs::core::AggregationType::ArrayAgg => "ARRAY_AGG",
            };

            sql.push_str(&format!(
                "{}({}.{}) AS {}",
                agg_type, entity, aggregation.aggregate_on, aggregation.bucket_name
            ));
        }

        // Add FROM clause
        sql.push_str(&format!(" FROM {}", self.sql_constructor.table));

        // Add joins if any
        sql.push_str(&self.sql_constructor.construct_joins());

        // Add WHERE clauses
        sql.push_str(&self.sql_constructor.construct_where_clauses()?);

        // Add GROUP BY clause
        sql.push_str(" GROUP BY bucket");

        // Add ORDER BY clause
        if let Some(order) =
            AggregationQueryFilter::get_aggregation_order(&self.sql_constructor.request_body)
        {
            let order_direction = order.order_direction.to_uppercase();
            sql.push_str(&format!(" ORDER BY {} {}", order.order_by, order_direction));
        }

        // Add LIMIT clause
        if AggregationQueryFilter::get_limit(&self.sql_constructor.request_body) > 0 {
            sql.push_str(&format!(
                " LIMIT {}",
                AggregationQueryFilter::get_limit(&self.sql_constructor.request_body)
            ));
        }

        Ok(sql)
    }
}

// Implement QueryFilter for AggregationFilter
impl QueryFilter for AggregationFilter {
    fn get_advance_filters(&self) -> &[FilterCriteria] {
        &self.advance_filters
    }

    fn get_joins(&self) -> &[Join] {
        &self.joins
    }

    fn get_limit(&self) -> usize {
        self.limit
    }

    fn get_date_format(&self) -> &str {
        "mm/dd/YYYY" // Default format for aggregation queries
    }

    fn get_order_by(&self) -> &str {
        self.order
            .as_ref()
            .map(|o| o.order_by.as_str())
            .unwrap_or("id")
    }

    fn get_order_direction(&self) -> &str {
        self.order
            .as_ref()
            .map(|o| o.order_direction.as_str())
            .unwrap_or("asc")
    }
}

// Implement AggregationQueryFilter for AggregationFilter
impl AggregationQueryFilter for AggregationFilter {
    fn get_advance_filters(&self) -> &[FilterCriteria] {
        &self.advance_filters
    }

    fn get_joins(&self) -> &[Join] {
        &self.joins
    }

    fn get_limit(&self) -> usize {
        self.limit
    }

    fn get_date_format(&self) -> &str {
        "mm/dd/YYYY" // Default format for aggregation queries
    }

    fn get_aggregations(&self) -> &[crate::structs::core::Aggregation] {
        &self.aggregations
    }

    fn get_bucket_size(&self) -> Option<&str> {
        self.bucket_size.as_deref()
    }

    fn get_timezone(&self) -> Option<&str> {
        self.timezone.as_deref()
    }

    fn get_aggregation_order(&self) -> Option<&AggregationOrder> {
        self.order.as_ref()
    }

    fn get_entity(&self) -> Option<&str> {
        Some(&self.entity)
    }

    fn get_order_by(&self) -> &str {
        self.order
            .as_ref()
            .map(|o| o.order_by.as_str())
            .unwrap_or("id")
    }

    fn get_order_direction(&self) -> &str {
        self.order
            .as_ref()
            .map(|o| o.order_direction.as_str())
            .unwrap_or("asc")
    }
}

#[derive(Clone)]
pub struct AggregationFilterWrapper {
    pub request: crate::generated::store::AggregationFilterRequest,
    pub converted_filters: Vec<FilterCriteria>,
    pub converted_joins: Vec<Join>,
    pub converted_aggregations: Vec<Aggregation>,
    pub converted_order: Option<AggregationOrder>,
}

impl AggregationFilterWrapper {
    pub fn new(request: crate::generated::store::AggregationFilterRequest) -> Self {
        // Extract data from the body field
        let body = request.body.as_ref();

        let converted_filters: Vec<_> = body
            .map(|b| {
                b.advance_filters
                    .iter()
                    .filter_map(convert_filter_criteria)
                    .collect()
            })
            .unwrap_or_default();

        let converted_joins: Vec<_> = body
            .map(|b| b.joins.iter().filter_map(convert_join).collect())
            .unwrap_or_default();

        let converted_aggregations: Vec<_> = body
            .map(|b| b.aggregations.iter().map(convert_aggregation).collect())
            .unwrap_or_default();

        let converted_order = body.and_then(|b| b.order.as_ref().map(convert_aggregation_order));

        Self {
            request,
            converted_filters,
            converted_joins,
            converted_aggregations,
            converted_order,
        }
    }
}

// Implement AggregationQueryFilter for AggregationFilterWrapper
impl AggregationQueryFilter for AggregationFilterWrapper {
    fn get_advance_filters(&self) -> &[FilterCriteria] {
        &self.converted_filters
    }

    fn get_joins(&self) -> &[Join] {
        &self.converted_joins
    }

    fn get_limit(&self) -> usize {
        self.request
            .body
            .as_ref()
            .and_then(|b| b.limit)
            .unwrap_or(100) as usize
    }

    fn get_date_format(&self) -> &str {
        "mm/dd/YYYY"
    }

    fn get_aggregations(&self) -> &[Aggregation] {
        &self.converted_aggregations
    }

    fn get_bucket_size(&self) -> Option<&str> {
        self.request
            .body
            .as_ref()
            .and_then(|b| b.bucket_size.as_deref())
    }

    fn get_timezone(&self) -> Option<&str> {
        self.request
            .body
            .as_ref()
            .and_then(|b| b.timezone.as_deref())
    }

    fn get_aggregation_order(&self) -> Option<&AggregationOrder> {
        self.converted_order.as_ref()
    }

    fn get_entity(&self) -> Option<&str> {
        self.request.body.as_ref().map(|b| b.entity.as_str())
    }

    fn get_order_by(&self) -> &str {
        self.request
            .body
            .as_ref()
            .and_then(|b| b.order.as_ref())
            .map(|o| o.order_by.as_str())
            .unwrap_or("id")
    }

    fn get_order_direction(&self) -> &str {
        self.request
            .body
            .as_ref()
            .and_then(|b| b.order.as_ref())
            .map(|o| o.order_direction.as_str())
            .unwrap_or("asc")
    }
}

impl QueryFilter for AggregationFilterWrapper {
    fn get_advance_filters(&self) -> &[FilterCriteria] {
        &self.converted_filters
    }

    fn get_joins(&self) -> &[Join] {
        &self.converted_joins
    }

    fn get_limit(&self) -> usize {
        self.request
            .body
            .as_ref()
            .and_then(|b| b.limit)
            .unwrap_or(100) as usize
    }

    fn get_date_format(&self) -> &str {
        "mm/dd/YYYY"
    }

    fn get_order_by(&self) -> &str {
        self.request
            .body
            .as_ref()
            .and_then(|b| b.order.as_ref())
            .map(|o| o.order_by.as_str())
            .unwrap_or("id")
    }

    fn get_order_direction(&self) -> &str {
        self.request
            .body
            .as_ref()
            .and_then(|b| b.order.as_ref())
            .map(|o| o.order_direction.as_str())
            .unwrap_or("asc")
    }
}
