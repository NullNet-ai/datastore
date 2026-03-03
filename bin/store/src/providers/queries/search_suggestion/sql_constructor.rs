use serde_json::Value;
use std::collections::BTreeMap;

use crate::providers::queries::find::sql_constructor::QueryFilter;
use crate::providers::queries::find::sql_constructor::SQLConstructor as FindSQLConstructor;
use crate::providers::queries::search_suggestion::{
    structs::{ConcatenatedExpressions, FieldFiltersResult},
    utils::get_field_filters,
};
use crate::structs::core::{
    ConcatenateField, FilterCriteria, GroupAdvanceFilter, Join, SearchSuggestionParams,
};
use crate::utils::helpers::{date_format_wrapper, time_format_wrapper, timestamp_format_wrapper};

pub trait QuerySearchSuggestion {
    fn get_pluck_object(&self) -> &BTreeMap<String, Vec<String>> {
        use std::collections::BTreeMap;
        static EMPTY: std::sync::LazyLock<BTreeMap<String, Vec<String>>> =
            std::sync::LazyLock::new(|| BTreeMap::new());
        &EMPTY
    }
}

impl QuerySearchSuggestion for SearchSuggestionParams {
    fn get_pluck_object(&self) -> &BTreeMap<String, Vec<String>> {
        &self.pluck_object
    }
}

impl QueryFilter for SearchSuggestionParams {
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
        &self.date_format
    }
    fn get_offset(&self) -> usize {
        self.offset
    }

    fn get_group_advance_filters(&self) -> &[crate::structs::core::GroupAdvanceFilter] {
        &self.group_advance_filters
    }

    fn get_concatenate_fields(&self) -> &[ConcatenateField] {
        &self.concatenate_fields
    }

    fn get_timezone(&self) -> Option<&str> {
        self.timezone.as_deref()
    }

    fn get_time_format(&self) -> &str {
        &self.time_format
    }
}

pub struct SQLConstructor<T: QuerySearchSuggestion + QueryFilter + Clone> {
    sql_constructor: FindSQLConstructor<T>,
    advance_filters: Option<Vec<FilterCriteria>>,
    group_advance_filters: Option<Vec<Value>>,
}

impl<T: QuerySearchSuggestion + QueryFilter + Clone> QuerySearchSuggestion for SQLConstructor<T> {
    fn get_pluck_object(&self) -> &BTreeMap<String, Vec<String>> {
        <T as QuerySearchSuggestion>::get_pluck_object(&self.sql_constructor.request_body)
    }
}

impl<T: QuerySearchSuggestion + QueryFilter + Clone> SQLConstructor<T> {
    pub fn new(request_body: T, table: String, is_root: bool, timezone: Option<String>) -> Self {
        Self {
            sql_constructor: FindSQLConstructor::new(request_body, table, is_root, timezone),
            advance_filters: None,
            group_advance_filters: None,
        }
    }

    pub fn with_organization_id(mut self, organization_id: String) -> Self {
        self.sql_constructor = self.sql_constructor.with_organization_id(organization_id);
        self
    }

    pub fn construct(
        &mut self,
        filtered_fields: &Value,
        advance_filters: &Vec<Value>,
        group_advance_filters: &Vec<Value>,
        search_term: &str,
        concatenated_expressions: &ConcatenatedExpressions,
    ) -> Result<String, String> {
        let mut sql = String::new();

        // Extract all values needed from immutable borrows before any mutable borrow
        let date_format_str = self
            .sql_constructor
            .request_body
            .get_date_format()
            .to_string();
        let time_format_str = self
            .sql_constructor
            .request_body
            .get_time_format()
            .to_string();

        // Handle timezone calculation
        let timezone_option = {
            let body_timezone = self.sql_constructor.request_body.get_timezone();
            let header_timezone = self.sql_constructor.timezone.as_deref();

            match (header_timezone, body_timezone) {
                (Some(tz), _) => Some(tz.to_string()),
                (None, Some(tz)) => Some(tz.to_string()),
                (None, None) => None,
            }
        };

        // construct per field (value and group) query
        let result = self.construct_field_query(
            filtered_fields,
            advance_filters,
            group_advance_filters,
            search_term,
            concatenated_expressions,
            Some(&date_format_str),
            time_format_str.as_str(),
            timezone_option.as_deref(),
        )?;
        let (field_query, field_query_names) = result;
        sql.push_str(&field_query);
        // construct union clause
        let union_clause = self.construct_union_clause(&field_query_names);
        sql.push_str(&union_clause);
        // construct key score clause
        let key_score_clause = self.construct_key_score_clause(search_term);
        sql.push_str(&key_score_clause);
        // construct entity score clause
        let entity_score_clause = self.construct_entity_score_clause();
        sql.push_str(&entity_score_clause);

        // construct final query string
        let json_result_query = self.construct_json_result_query();
        sql.push_str(&json_result_query);
        Ok(sql)
    }

    fn construct_field_query(
        &mut self,
        filtered_fields: &Value,
        advance_filters: &Vec<Value>,
        group_advance_filters: &Vec<Value>,
        search_term: &str,
        concatenated_expressions: &ConcatenatedExpressions,
        date_format: Option<&str>,
        time_format: &str,
        timezone: Option<&str>,
    ) -> Result<(String, Vec<String>), String> {
        let mut sql = String::new();
        let mut field_query_names: Vec<String> = Vec::new();
        let json_build_object_query = filtered_fields
            .as_object()
            .ok_or_else(|| String::from("filtered_fields must be an object"))?
            .iter()
            .fold(String::new(), |acc, (entity, fields)| {
                let field_object_agg =
                    fields
                        .as_array()
                        .unwrap_or(&Vec::new())
                        .iter()
                        .filter_map(|field| field.as_str())
                        .map(|field| {
                            let mut per_field_query = String::new();
                            let mut entity_field = format!("{}.{}", entity, field);

                            let mut field_filter = None::<FilterCriteria>;
                            let mut all_field_filters: Vec<FilterCriteria> = Vec::new();
                            let mut all_field_group_filters: Vec<Value> = Vec::new();

                            if !group_advance_filters.is_empty() {
                                for grouped_filters in group_advance_filters {
                                    if let Some(filters_obj) = grouped_filters.get("filters") {
                                        if let Ok(filters) =
                                            serde_json::from_value::<Vec<FilterCriteria>>(
                                                filters_obj.clone(),
                                            )
                                        {
                                            let FieldFiltersResult {
                                                all_field_filters: group_all_field_filters,
                                                field_filter: group_field_filter,
                                            } = get_field_filters(
                                                filters,
                                                field,
                                                entity,
                                                search_term,
                                            );

                                            // Update field_filter if it's empty and group_field_filter has content
                                            if field_filter.is_none() {
                                                field_filter = group_field_filter;
                                            }

                                            // Create a new group filter with updated filters
                                            let mut new_group = grouped_filters.clone();
                                            new_group["filters"] =
                                                serde_json::to_value(group_all_field_filters)
                                                    .unwrap_or(Value::Array(vec![]));
                                            all_field_group_filters.push(new_group);
                                        }
                                    }
                                }
                            } else {
                                let filters: Vec<FilterCriteria> = advance_filters
                                    .iter()
                                    .filter_map(|v| serde_json::from_value(v.clone()).ok())
                                    .collect();

                                let FieldFiltersResult {
                                    all_field_filters: _all_field_filters,
                                    field_filter: _field_filter,
                                } = get_field_filters(filters, field, entity, search_term);

                                all_field_filters = _all_field_filters;
                                field_filter = _field_filter;
                            }

                            let (values, parse_as, has_group_count) = match field_filter {
                                Some(FilterCriteria::Criteria {
                                    values,
                                    parse_as,
                                    has_group_count,
                                    ..
                                }) => (values, parse_as, has_group_count),
                                _ => (Vec::new(), String::new(), None),
                            };

                            if let Some(field_expr) = concatenated_expressions
                                .get(entity)
                                .and_then(|fields| fields.get(field))
                            {
                                entity_field = format!("{}", field_expr.expression)
                            } else if field.ends_with("_date") {
                                entity_field = format!(
                                    "{}",
                                    date_format_wrapper(
                                        entity.as_str(),
                                        field,
                                        date_format,
                                        timezone,
                                        false
                                    )
                                );
                            } else if field.ends_with("_time") {
                                entity_field = format!(
                                    "{}",
                                    time_format_wrapper(
                                        entity.as_str(),
                                        field,
                                        timezone,
                                        entity.as_str(),
                                        false,
                                        time_format
                                    )
                                );
                            } else if field.eq_ignore_ascii_case("timestamp") {
                                let date_fmt = date_format.unwrap_or("YYYY-mm-dd");
                                entity_field = format!(
                                    "{}",
                                    timestamp_format_wrapper(
                                        entity.as_str(),
                                        field,
                                        date_fmt,
                                        time_format,
                                        timezone,
                                        false,
                                    )
                                );
                            }

                            let values_flat = values
                                .iter()
                                .map(|v| v.to_string())
                                .collect::<Vec<String>>()
                                .join(",")
                                .replace('\'', "\"");
                            let parse_string = if parse_as == "text" { "::text" } else { "" };
                            let has_group_count = has_group_count.unwrap_or(false);
                            if has_group_count {
                                let query_field_name = format!("{}_{}_group", entity, field);
                                let field_group_query = self.construct_per_field_query(
                                    entity,
                                    field,
                                    entity_field.as_str(),
                                    parse_string,
                                    values_flat.as_str(),
                                    has_group_count,
                                    &all_field_filters,
                                    &all_field_group_filters,
                                    concatenated_expressions,
                                    parse_as.as_str(),
                                );
                                per_field_query.push_str(&format!(
                                    "{} AS ({}), ",
                                    query_field_name, field_group_query
                                ));
                                field_query_names.push(query_field_name);
                            }

                            let query_field_name = format!("{}_{}_values", entity, field);
                            field_query_names.push(query_field_name.clone());
                            let field_query = self.construct_per_field_query(
                                entity,
                                field,
                                entity_field.as_str(),
                                parse_string,
                                values_flat.as_str(),
                                false,
                                &all_field_filters,
                                &all_field_group_filters,
                                concatenated_expressions,
                                parse_as.as_str(),
                            );
                            per_field_query
                                .push_str(&format!("{} AS ({})", query_field_name, field_query));

                            per_field_query
                        })
                        .collect::<Vec<String>>()
                        .join(", ");

                format!(
                    "{}{}{}",
                    acc,
                    if acc.is_empty() { "WITH " } else { ", " },
                    field_object_agg
                )
            });

        sql.push_str(&format!("{}, ", json_build_object_query));
        if sql.is_empty() {
            Err(String::from("No query was constructed"))
        } else {
            Ok((sql, field_query_names))
        }
    }

    fn construct_union_clause(&self, field_query_names: &Vec<String>) -> String {
        let mut queries = Vec::new();
        for name in field_query_names {
            queries.push(format!("SELECT * FROM {}", name));
        }
        format!(
            "all_values AS ({}), ",
            queries
                .iter()
                .map(|s| s.as_str())
                .collect::<Vec<&str>>()
                .join(" UNION ALL ")
        )
    }

    fn construct_key_score_clause(&self, search_term: &str) -> String {
        let query = format!("SELECT
            entity_type,
            key,
            MAX(match_score) AS best_score,
            SUM(CASE WHEN match_score = 100 THEN count ELSE 0 END) AS exact_count,
            SUM(CASE WHEN match_score >= 70 AND match_score < 100 THEN count ELSE 0 END) AS prefix_count,
            SUM(CASE WHEN match_score >= 50 AND match_score < 70 THEN count ELSE 0 END) AS partial_count,
            JSON_OBJECT_AGG(
              value, 
              count 
              ORDER BY 
                CASE
                  WHEN LOWER(value) = LOWER('{}') THEN 100
                  WHEN LOWER(value) LIKE LOWER('{} %') THEN 90
                  WHEN LOWER(value) LIKE LOWER('% {}') THEN 85
                  WHEN LOWER(value) LIKE LOWER('% {} %') THEN 80
                  WHEN LOWER(value) LIKE LOWER('{}%') THEN 70
                  WHEN LOWER(value) LIKE LOWER('%{}') THEN 60
                  WHEN LOWER(value) LIKE LOWER('%{}%') THEN 50
                  ELSE 0
                END DESC,
                value ASC
            ) AS value_json
        FROM all_values
        GROUP BY entity_type, key",
        search_term,
        search_term,
        search_term,
        search_term,
        search_term,
        search_term,
        search_term,
    );
        format!("key_scores AS ({}), ", query)
    }

    fn construct_entity_score_clause(&self) -> String {
        format!("entity_scores AS (SELECT 
            entity_type,
            MAX(best_score)::integer as max_score,
            SUM(exact_count * 100 + prefix_count * 70 + partial_count * 50)::integer as total_weighted_score,
            SUM(CASE WHEN best_score >= 70 THEN 1 ELSE 0 END)::integer as high_score_count,
            JSON_OBJECT_AGG(key, value_json ORDER BY best_score DESC) AS entity_data
        FROM key_scores
        GROUP BY entity_type) ")
    }

    fn construct_json_result_query(&self) -> String {
        format!(
            "SELECT JSON_BUILD_OBJECT(
            'data', JSON_OBJECT_AGG(
                entity_type,
                (
                SELECT JSON_OBJECT_AGG(
                    key, value_json
                    ORDER BY best_score DESC, key
                )
                FROM key_scores ks
                WHERE ks.entity_type = entity_scores.entity_type
                    AND ks.value_json IS NOT NULL
                )
                ORDER BY 
                max_score DESC,          
                total_weighted_score DESC, 
                high_score_count DESC,   
                entity_type               
            )
        ) AS results
        FROM entity_scores"
        )
    }

    fn construct_per_field_query(
        &mut self,
        entity: &str,
        field: &str,
        entity_field: &str,
        parse_string: &str,
        values_flat: &str,
        is_group_count: bool,
        all_field_filters: &Vec<FilterCriteria>,
        all_field_group_filters: &Vec<Value>,
        concatenated_expressions: &ConcatenatedExpressions,
        parse_as: &str,
    ) -> String {
        let mut field_query = if is_group_count {
            self.construct_field_group_selections(entity, field)
        } else {
            self.construct_field_value_selections(
                entity,
                field,
                entity_field,
                parse_string,
                values_flat,
            )
        };

        // Construct joins
        let joins_sql_string = &self.construct_joins();
        field_query.push_str(joins_sql_string);
        // Construct where clauses
        let _ =
            &self.set_advance_filters(all_field_filters.clone(), all_field_group_filters.clone());
        let where_clauses = &self
            .construct_where_clauses(concatenated_expressions)
            .unwrap_or_else(|_| String::new());
        field_query.push_str(where_clauses);

        if is_group_count {
            return field_query;
        } else {
            // Construct group by
            let group_by =
                self.construct_group_by(&entity, &field, concatenated_expressions, Some(parse_as));
            field_query.push_str(&group_by);
            // Construct offset
            let offset = self.sql_constructor.construct_offset();
            field_query.push_str(&offset);
            // Construct limit
            let limit = self.sql_constructor.construct_limit();
            field_query.push_str(&limit);

            field_query
        }
    }

    fn construct_field_value_selections(
        &self,
        entity: &str,
        field: &str,
        entity_field: &str,
        parse_string: &str,
        value: &str,
    ) -> String {
        let clean_value = value.trim_matches('\"');
        format!(
            "SELECT '{}' AS key, {}{} AS value, COUNT(*) AS count,
            CASE
                WHEN LOWER({}{}) = LOWER('{}') THEN 100  
                WHEN LOWER({}{}) LIKE LOWER('{} %') THEN 90
                WHEN LOWER({}{}) LIKE LOWER('% {}') THEN 85
                WHEN LOWER({}{}) LIKE LOWER('% {} %') THEN 80  
                WHEN LOWER({}{}) LIKE LOWER('{}%') THEN 70  
                WHEN LOWER({}{}) LIKE LOWER('%{}') THEN 60  
                WHEN LOWER({}{}) LIKE LOWER('%{}%') THEN 50  
                ELSE 0
            END AS match_score,
            '{}' AS entity_type
            FROM {}
            ",
            field,
            entity_field,
            parse_string,
            entity_field,
            parse_string,
            clean_value,
            entity_field,
            parse_string,
            clean_value,
            entity_field,
            parse_string,
            clean_value,
            entity_field,
            parse_string,
            clean_value,
            entity_field,
            parse_string,
            clean_value,
            entity_field,
            parse_string,
            clean_value,
            entity_field,
            parse_string,
            clean_value,
            entity,
            &self.sql_constructor.table
        )
    }

    fn construct_field_group_selections(&self, entity: &str, field: &str) -> String {
        format!("SELECT '{}_group' AS key, 'count' AS value, COUNT(*) AS count, 0 AS match_score, '{}' AS entity_type FROM {}", field, entity, &self.sql_constructor.table )
    }

    fn construct_joins(&self) -> String {
        if self.sql_constructor.request_body.get_joins().is_empty() {
            String::from("")
        } else {
            let mut join_clauses = Vec::new();

            for join in self.sql_constructor.request_body.get_joins() {
                match join.r#type.as_str() {
                    "left" => {
                        let join_clause = self.build_join_lateral(join, false);
                        join_clauses.push(join_clause);
                    }
                    "self" => {
                        let join_clause = self.build_join_lateral(join, true);
                        join_clauses.push(join_clause);
                    }
                    _ => {
                        // Unsupported join type, skip or log warning
                        log::warn!("Unsupported join type: {}", join.r#type);
                    }
                }
            }

            if join_clauses.is_empty() {
                String::from("")
            } else {
                format!(" {}", join_clauses.join(" "))
            }
        }
    }

    fn build_join_lateral(&self, join: &Join, is_self_join: bool) -> String {
        let to_entity = if is_self_join {
            &self.sql_constructor.table
        } else {
            &join.field_relation.to.entity
        };

        let to_alias = if is_self_join {
            if let Some(from_alias) = &join.field_relation.from.alias {
                from_alias.as_str()
            } else {
                join.field_relation
                    .to
                    .alias
                    .as_deref()
                    .unwrap_or(&self.sql_constructor.table)
            }
        } else {
            join.field_relation.to.alias.as_deref().unwrap_or(to_entity)
        };

        let to_field = &join.field_relation.to.field;
        let from_entity = if is_self_join {
            &self.sql_constructor.table
        } else {
            &join.field_relation.from.entity
        };
        let from_field = &join.field_relation.from.field;
        let is_nested = join.nested;

        // Build the lateral subquery alias
        let lateral_alias = format!("joined_{}", to_alias);

        // Build dynamic field selection based on pluck_object
        let selected_fields = if let Some(fields) = self.get_pluck_object().get(to_alias) {
            fields
                .iter()
                .map(|field| format!("\"{}\".\"{}\"", lateral_alias, field))
                .collect::<Vec<_>>()
                .join(", ")
        } else {
            // Default fallback fields if no pluck_object configuration found
            format!("\"{}\".\"id\"", lateral_alias)
        };

        let standard_where = self
            .sql_constructor
            .build_system_where_clause(&lateral_alias)
            .unwrap_or_else(|_| format!("({}.tombstone = 0)", lateral_alias));

        if is_nested {
            return format!(
                "LEFT JOIN LATERAL (SELECT {} FROM \"{}\" \"{}\" WHERE {} AND \"{}\".\"{}\" = \"{}\".\"{}\" ) AS \"{}\" ON TRUE",
                selected_fields,
                to_entity, lateral_alias,
                standard_where,
                lateral_alias, to_field, from_entity, from_field,
                to_alias
            );
        }

        let from_table_ref = if is_self_join {
            &self.sql_constructor.table
        } else {
            &self.sql_constructor.table
        };

        format!(
            "LEFT JOIN LATERAL (SELECT {} FROM \"{}\" \"{}\" WHERE {} AND \"{}\".\"{}\" = \"{}\".\"{}\" ) AS \"{}\" ON TRUE",
            selected_fields,
            to_entity, lateral_alias,
            standard_where,
            from_table_ref, from_field, lateral_alias, to_field,
            to_alias
        )
    }

    fn construct_where_clauses(
        &mut self,
        _concatenated_expressions: &ConcatenatedExpressions,
    ) -> Result<String, String> {
        let mut base_where = format!(
            " WHERE {}",
            self.sql_constructor
                .build_system_where_clause(&self.sql_constructor.table)?
        );

        // Prioritize advance_filters over group_advance_filters
        if !self
            .advance_filters
            .as_ref()
            .map_or(true, |filters| filters.is_empty())
        {
            let expression = self
                .sql_constructor
                .build_infix_expression(self.advance_filters.as_deref().unwrap_or_default())?;
            if !expression.is_empty() {
                base_where.push_str(" AND ");
                base_where.push_str(&expression);
            }
        } else if !self
            .group_advance_filters
            .as_ref()
            .map_or(true, |filters| filters.is_empty())
        {
            let group_filters: Vec<GroupAdvanceFilter> = self
                .group_advance_filters
                .as_ref()
                .map_or(Vec::new(), |filters| {
                    filters
                        .iter()
                        .filter_map(|value| match serde_json::from_value(value.clone()) {
                            Ok(filter) => Some(filter),
                            Err(_) => None,
                        })
                        .collect()
                });
            if !self
                .sql_constructor
                .build_group_advance_filters_expression(&group_filters)
                .map_or(true, |expr| expr.is_empty())
            {
                if let Ok(group_expression) = self
                    .sql_constructor
                    .build_group_advance_filters_expression(&group_filters)
                {
                    base_where.push_str(" AND ");
                    base_where.push_str(&group_expression);
                }
            }
        }

        Ok(base_where)
    }

    fn set_advance_filters(
        &mut self,
        advance_filters: Vec<FilterCriteria>,
        group_advance_filters: Vec<Value>,
    ) {
        self.advance_filters = Some(advance_filters);
        self.group_advance_filters = Some(group_advance_filters);
    }

    fn construct_group_by(
        &self,
        entity: &str,
        field: &str,
        concatenated_expressions: &ConcatenatedExpressions,
        parse_as: Option<&str>,
    ) -> String {
        if let Some(field_expr) = concatenated_expressions
            .get(entity)
            .and_then(|fields| fields.get(field))
        {
            format!(" GROUP BY {}", field_expr.expression.clone())
        } else if !entity.is_empty() && !field.is_empty() {
            let field = FindSQLConstructor::<SearchSuggestionParams>::get_field(
                entity,
                field,
                self.sql_constructor.request_body.get_date_format(),
                &self.sql_constructor.table.as_str(),
                self.sql_constructor.request_body.get_timezone(),
                false,
                self.sql_constructor.request_body.get_time_format(),
                parse_as,
            );
            format!(" GROUP BY {}", field)
        } else {
            String::from("")
        }
    }
}
