// use serde::{Serialize, Deserialize};
// use serde_json::Value;
use crate::schema::verify::field_exists_in_table;
use crate::structs::structs::{ApiResponse, FilterCriteria, GetByFilter};

// #[derive(Serialize, Deserialize)]
pub struct Validation<'a, 'b> {
    request_body: &'a GetByFilter,
    table: &'b String,
}

impl<'a, 'b> Validation<'a, 'b> {
    pub fn new(request_body: &'a GetByFilter, table: &'b String) -> Self {
        Self {
            request_body,
            table,
        }
    }

    pub fn exec(&self) -> ApiResponse {
        let validation_checks = vec![
            "table",
            "pluck",
            "pluck_object",
            "advance_filters:group_advance_filters",
            "advance_filters",
            "group_advance_filters",
            "concatenated_fields",
            "group_by",
            "joins",
            "order_by_format",
            "order_direction",
            "date_format",
            "multiple_sort",
            "limit_offset",
            "distinct_by",
        ];

        // Iterate through validation checks
        for check in &validation_checks {
            let response = match *check {
                "table" => self.validate_table(),
                "pluck" => self.validate_pluck(),
                "pluck_object" => self.validate_pluck_object(),
                "advance_filters:group_advance_filters" => self.validate_conflicting_filters(),
                "advance_filters" => self.validate_advance_filters(),
                "group_advance_filters" => self.validate_group_advance_filters(),
                "concatenated_fields" => self.validate_concatenated_fields(),
                "group_by" => self.validate_group_by(),
                "joins" => self.validate_joins(),
                "order_by_format" => self.validate_order_by_format(),
                "order_direction" => self.validate_order_direction(),
                "date_format" => self.validate_date_format(),
                "multiple_sort" => self.validate_multiple_sort(),
                "limit_offset" => self.validate_limit_offset(),
                "distinct_by" => self.validate_distinct_by(),
                _ => continue,
            };

            if !response.success {
                return response;
            }
        }

        ApiResponse {
            success: true,
            message: "All validations passed successfully".to_string(),
            count: 0,
            data: vec![],
        }
    }

    pub fn validate_table(&self) -> ApiResponse {
        if self.table.is_empty() {
            return ApiResponse {
                success: false,
                message: "table is required".to_string(),
                count: 0,
                data: vec![],
            };
        }

        ApiResponse {
            success: true,
            message: "Successfully validated table field".to_string(),
            count: 0,
            data: vec![],
        }
    }
    pub fn validate_distinct_by(&self) -> ApiResponse {
        if let Some(distinct_by) = &self.request_body.distinct_by {
            if distinct_by.is_empty() {
                return ApiResponse {
                    success: true,
                    message: "Successfully validated distinct_by field".to_string(),
                    count: 0,
                    data: vec![],
                };
            }

            if !field_exists_in_table(self.table, distinct_by) {
                return ApiResponse {
                    success: false,
                    message: format!(
                        "distinct_by field {} does not exist in table {}",
                        distinct_by, self.table
                    ),
                    count: 0,
                    data: vec![],
                };
            }
        }

        ApiResponse {
            success: true,
            message: "Successfully validated distinct_by field".to_string(),
            count: 0,
            data: vec![],
        }
    }

    pub fn validate_pluck(&self) -> ApiResponse {
        if self.request_body.pluck.is_empty() {
            return ApiResponse {
                success: false,
                message: "pluck is required".to_string(),
                count: 0,
                data: vec![],
            };
        }

        ApiResponse {
            success: true,
            message: "Successfully validated pluck field".to_string(),
            count: 0,
            data: vec![],
        }
    }

    pub fn validate_conflicting_filters(&self) -> ApiResponse {
        if !self.request_body.advance_filters.is_empty()
            && !self.request_body.group_advance_filters.is_empty()
        {
            return ApiResponse {
                success: false,
                message: "Both advance_filters and group_advance_filters cannot be provided at the same time".to_string(),
                count: 0,
                data: vec![]
            };
        }

        ApiResponse {
            success: true,
            message: "Successfully validated conflicting properties".to_string(),
            count: 0,
            data: vec![],
        }
    }
    pub fn validate_concatenated_fields(&self) -> ApiResponse {
        for (concat_index, concatenate_field) in
            self.request_body.concatenate_fields.iter().enumerate()
        {
            if concatenate_field.fields.is_empty() {
                return ApiResponse {
                    success: false,
                    message: format!(
                        "concatenate_fields[{}] > fields > Fields array cannot be empty",
                        concat_index
                    ),
                    count: 0,
                    data: vec![],
                };
            }

            if concatenate_field.field_name.is_empty() {
                return ApiResponse {
                    success: false,
                    message: format!(
                        "concatenate_fields[{}] > field_name > Field name cannot be empty",
                        concat_index
                    ),
                    count: 0,
                    data: vec![],
                };
            }

            if concatenate_field.entity.is_empty() {
                return ApiResponse {
                    success: false,
                    message: format!(
                        "concatenate_fields[{}] > entity > Entity cannot be empty",
                        concat_index
                    ),
                    count: 0,
                    data: vec![],
                };
            }

            // Validate that all fields exist in the specified entity
            for (field_index, field) in concatenate_field.fields.iter().enumerate() {
                if !field_exists_in_table(&concatenate_field.entity, field) {
                    return ApiResponse {
                        success: false,
                        message: format!(
                            "concatenate_fields[{}] > fields[{}] > Field '{}' does not exist in entity '{}'",
                            concat_index, field_index, field, concatenate_field.entity
                        ),
                        count: 0,
                        data: vec![],
                    };
                }
            }
        }

        return ApiResponse {
            success: true,
            message: "Successfully validated concatenated_fields".to_string(),
            count: 0,
            data: vec![],
        };
    }

    pub fn validate_group_by(&self) -> ApiResponse {
        // If group_by is None, validation passes
        if let Some(group_by) = &self.request_body.group_by {
            // Validate that all fields in group_by exist in the main table
            for (field_index, field) in group_by.fields.iter().enumerate() {
                if !field_exists_in_table(self.table, field) {
                    return ApiResponse {
                        success: false,
                        message: format!(
                            "group_by > fields[{}] > Field '{}' does not exist in table '{}'",
                            field_index, field, self.table
                        ),
                        count: 0,
                        data: vec![],
                    };
                }
            }
        }

        ApiResponse {
            success: true,
            message: "Successfully validated group_by fields".to_string(),
            count: 0,
            data: vec![],
        }
    }

    pub fn validate_pluck_object(&self) -> ApiResponse {
        // If no joins are present, pluck_object should be empty or only reference the main table
        if self.request_body.joins.is_empty() {
            for (entity, _) in &self.request_body.pluck_object {
                if entity != self.table {
                    return ApiResponse {
                        success: false,
                        message: format!(
                            "pluck_object[{}] > Entity '{}' is not valid. Without joins, only the main table '{}' can be referenced",
                            entity, entity, self.table
                        ),
                        count: 0,
                        data: vec![],
                    };
                }
            }
        } else {
            // Collect valid entities from joins (both entity names and aliases)
            let mut valid_entities = std::collections::HashSet::new();

            // Add the main table as a valid entity
            valid_entities.insert(self.table.clone());

            // Add join entities and their aliases
            for join in &self.request_body.joins {
                // Add the target entity
                valid_entities.insert(join.field_relation.to.entity.clone());

                // Add the alias if it exists
                if let Some(alias) = &join.field_relation.to.alias {
                    valid_entities.insert(alias.clone());
                }
            }

            // Validate that pluck_object entities are valid (either join entities or aliases)
            for (entity, _) in &self.request_body.pluck_object {
                if !valid_entities.contains(entity) {
                    return ApiResponse {
                        success: false,
                        message: format!(
                            "pluck_object[{}] > Entity '{}' is not valid. Must be either a joined entity or its alias",
                            entity, entity
                        ),
                        count: 0,
                        data: vec![],
                    };
                }
            }
        }

        // Collect all concatenated field names
        let concatenated_field_names: std::collections::HashSet<_> = self
            .request_body
            .concatenate_fields
            .iter()
            .map(|cf| cf.field_name.as_str())
            .collect();

        // Validate pluck_object fields exist in their respective entities
        for (entity, fields) in &self.request_body.pluck_object {
            // Determine the actual table name to check against
            let table_to_check = if entity == self.table {
                // If entity is the main table, use it directly
                entity.clone()
            } else {
                // Find the join that corresponds to this entity (either by entity name or alias)
                let join_entity = self
                    .request_body
                    .joins
                    .iter()
                    .find(|join| {
                        join.field_relation.to.entity == *entity
                            || join.field_relation.to.alias.as_ref() == Some(entity)
                    })
                    .map(|join| join.field_relation.to.entity.clone());

                match join_entity {
                    Some(table) => table,
                    None => {
                        return ApiResponse {
                            success: false,
                            message: format!(
                                "pluck_object[{}] > Cannot find corresponding join for entity '{}'",
                                entity, entity
                            ),
                            count: 0,
                            data: vec![],
                        };
                    }
                }
            };

            for (field_index, field) in fields.iter().enumerate() {
                // Skip validation if field is a concatenated field name
                if concatenated_field_names.contains(field.as_str()) {
                    continue;
                }

                if !field_exists_in_table(&table_to_check, field) {
                    return ApiResponse {
                        success: false,
                        message: format!(
                            "pluck_object[{}][{}] > Field '{}' does not exist in entity '{}'",
                            entity, field_index, field, table_to_check
                        ),
                        count: 0,
                        data: vec![],
                    };
                }
            }
        }

        ApiResponse {
            success: true,
            message: "Successfully validated pluck_object fields".to_string(),
            count: 0,
            data: vec![],
        }
    }
    pub fn validate_joins(&self) -> ApiResponse {
        for (join_index, join) in self.request_body.joins.iter().enumerate() {
            // Validate join type
            let join_type = join.r#type.to_uppercase();
            if join_type != "LEFT" && join_type != "SELF" {
                return ApiResponse {
                    success: false,
                    message: format!(
                        "joins[{}] > type > Invalid join type: '{}'. Supported types are: LEFT, SELF",
                        join_index, join.r#type
                    ),
                    count: 0,
                    data: vec![],
                };
            }

            // Validate field relations exist
            let from_entity = &join.field_relation.from.entity;
            let from_field = &join.field_relation.from.field;
            let to_entity = &join.field_relation.to.entity;
            let to_field = &join.field_relation.to.field;

            // For nested joins, validate that the from entity matches the previous join's to entity or alias
            if join.nested && join_index > 0 {
                let previous_join = &self.request_body.joins[join_index - 1];
                let expected_from_entity =
                    if let Some(alias) = &previous_join.field_relation.to.alias {
                        alias
                    } else {
                        &previous_join.field_relation.to.entity
                    };

                if from_entity != expected_from_entity {
                    return ApiResponse {
                        success: false,
                        message: format!(
                            "joins[{}] > field_relation > from > entity > Nested join from entity '{}' must match previous join's to entity or alias '{}'",
                            join_index, from_entity, expected_from_entity
                        ),
                        count: 0,
                        data: vec![],
                    };
                }
            } else if join.nested && join_index == 0 {
                return ApiResponse {
                    success: false,
                    message: format!(
                        "joins[{}] > nested > First join cannot be nested",
                        join_index
                    ),
                    count: 0,
                    data: vec![],
                };
            }

            // Determine the actual table to validate the from field against
            let from_table_to_check = if join.nested && join_index > 0 {
                let previous_join = &self.request_body.joins[join_index - 1];
                &previous_join.field_relation.to.entity
            } else {
                from_entity
            };

            if !field_exists_in_table(from_table_to_check, from_field) {
                return ApiResponse {
                    success: false,
                    message: format!(
                        "joins[{}] > field_relation > from > field > Join from field '{}' does not exist in entity '{}'",
                        join_index, from_field, from_table_to_check
                    ),
                    count: 0,
                    data: vec![],
                };
            }

            if !field_exists_in_table(to_entity, to_field) {
                return ApiResponse {
                    success: false,
                    message: format!(
                        "joins[{}] > field_relation > to > field > Join to field '{}' does not exist in entity '{}'",
                        join_index, to_field, to_entity
                    ),
                    count: 0,
                    data: vec![],
                };
            }
        }

        ApiResponse {
            success: true,
            message: "Successfully validated joins".to_string(),
            count: 0,
            data: vec![],
        }
    }

    pub fn validate_order_by_format(&self) -> ApiResponse {
        // Validate order_by format (should be "entity.field" or just "field" for main table)
        if !self.request_body.order_by.is_empty() {
            let by_entity_field: Vec<&str> = self.request_body.order_by.split('.').collect();

            let (sort_entity, sort_field) = if by_entity_field.len() == 1 {
                // If just field name (e.g., "id"), default to main table
                (self.table.as_str(), by_entity_field[0])
            } else if by_entity_field.len() == 2 {
                // If entity.field format
                (by_entity_field[0], by_entity_field[1])
            } else {
                return ApiResponse {
                    success: false,
                    message: format!(
                        "order_by > Invalid order_by format: '{}'. It should be 'field' or 'table.field'",
                        self.request_body.order_by
                    ),
                    count: 0,
                    data: vec![],
                };
            };

            // Validate field exists in schema
            if !field_exists_in_table(sort_entity, sort_field) {
                return ApiResponse {
                    success: false,
                    message: format!(
                        "order_by > Order by field '{}' does not exist in entity '{}'",
                        sort_field, sort_entity
                    ),
                    count: 0,
                    data: vec![],
                };
            }
        }

        ApiResponse {
            success: true,
            message: "Successfully validated order_by format".to_string(),
            count: 0,
            data: vec![],
        }
    }

    pub fn validate_order_direction(&self) -> ApiResponse {
        // Validate order_direction if it's not empty
        if !self.request_body.order_direction.is_empty() {
            let direction_lower = self.request_body.order_direction.to_lowercase();
            if direction_lower != "asc" && direction_lower != "desc" {
                return ApiResponse {
                    success: false,
                    message: format!(
                        "order_direction > Invalid order direction: '{}'. Valid values are: asc, desc",
                        self.request_body.order_direction
                    ),
                    count: 0,
                    data: vec![],
                };
            }
        }

        ApiResponse {
            success: true,
            message: "Successfully validated order_direction".to_string(),
            count: 0,
            data: vec![],
        }
    }

    pub fn validate_date_format(&self) -> ApiResponse {
        // Define allowed date formats
        let allowed_formats = vec![
            "mm/dd/YYYY",
            "dd/mm/YYYY",
            "YYYY/mm/dd",
            "YYYY/dd/mm",
            "mm-dd-YYYY",
            "dd-mm-YYYY",
            "YYYY-mm-dd",
            "YYYY-dd-mm",
        ];

        // Validate date_format if it's not empty
        if !self.request_body.date_format.is_empty() {
            if !allowed_formats.contains(&self.request_body.date_format.as_str()) {
                return ApiResponse {
                    success: false,
                    message: format!(
                        "date_format > Invalid date format: '{}'. Valid formats are: {}",
                        self.request_body.date_format,
                        allowed_formats.join(", ")
                    ),
                    count: 0,
                    data: vec![],
                };
            }
        }

        ApiResponse {
            success: true,
            message: "Successfully validated date_format".to_string(),
            count: 0,
            data: vec![],
        }
    }

    pub fn validate_multiple_sort(&self) -> ApiResponse {
        for (sort_index, sort_option) in self.request_body.multiple_sort.iter().enumerate() {
            // Validate sort field format
            let by_entity_field: Vec<&str> = sort_option.by_field.split('.').collect();

            let (sort_entity, sort_field) = if by_entity_field.len() == 1 {
                // If just field name (e.g., "id"), default to main table
                (self.table.as_str(), by_entity_field[0])
            } else if by_entity_field.len() == 2 {
                // If entity.field format
                (by_entity_field[0], by_entity_field[1])
            } else {
                return ApiResponse {
                    success: false,
                    message: format!(
                        "multiple_sort[{}] > by_field > Invalid sort field format: '{}'. It should be 'field' or 'table.field'",
                        sort_index, sort_option.by_field
                    ),
                    count: 0,
                    data: vec![],
                };
            };

            // Validate field exists in schema
            if !field_exists_in_table(sort_entity, sort_field) {
                return ApiResponse {
                    success: false,
                    message: format!(
                        "multiple_sort[{}] > by_field > Sort field '{}' does not exist in entity '{}'",
                        sort_index, sort_field, sort_entity
                    ),
                    count: 0,
                    data: vec![],
                };
            }

            // Validate direction
            let direction = sort_option.by_direction.to_lowercase();
            if direction != "asc" && direction != "desc" {
                return ApiResponse {
                    success: false,
                    message: format!(
                        "multiple_sort[{}] > by_direction > Invalid sort direction: '{}'. Valid values are: asc, desc",
                        sort_index, sort_option.by_direction
                    ),
                    count: 0,
                    data: vec![],
                };
            }
        }

        ApiResponse {
            success: true,
            message: "Successfully validated multiple_sort".to_string(),
            count: 0,
            data: vec![],
        }
    }

    pub fn validate_limit_offset(&self) -> ApiResponse {
        // Validate limit is reasonable (not too large)
        if self.request_body.limit > 10000 {
            return ApiResponse {
                success: false,
                message: "Limit cannot exceed 10000 records".to_string(),
                count: 0,
                data: vec![],
            };
        }

        // Validate offset is not negative (though usize prevents this, good to be explicit)
        if self.request_body.limit == 0 {
            return ApiResponse {
                success: false,
                message: "Limit must be greater than 0".to_string(),
                count: 0,
                data: vec![],
            };
        }

        ApiResponse {
            success: true,
            message: "Successfully validated limit and offset".to_string(),
            count: 0,
            data: vec![],
        }
    }

    pub fn validate_advance_filters(&self) -> ApiResponse {
        for (filter_index, filter) in self.request_body.advance_filters.iter().enumerate() {
            match filter {
                FilterCriteria::Criteria {
                    field,
                    entity,
                    operator,
                    values,
                    ..
                } => {
                    // Skip validation if entity is None
                    let entity_str = match entity {
                        Some(e) => e,
                        None => continue,
                    };

                    // Check if this field is a concatenated field and skip validation if it is
                    let is_concatenated_field =
                        self.request_body
                            .concatenate_fields
                            .iter()
                            .any(|concat_field| {
                                concat_field.field_name == *field
                                    && (concat_field.entity == *entity_str
                                        || concat_field.aliased_entity.as_ref() == Some(entity_str))
                            });

                    if is_concatenated_field {
                        // Skip validation for concatenated fields as they are virtual fields
                        continue;
                    }

                    // Check if the filtered field exists in JOIN "to" fields, aliases, or entities
                    let field_exists_in_joins = self.request_body.joins.iter().any(|join| {
                        let to_endpoint = &join.field_relation.to;

                        // Check if field matches the "to" field
                        if to_endpoint.field == *field {
                            // Check if entity matches the "to" entity or alias
                            if to_endpoint.entity == *entity_str {
                                return true;
                            }
                            if let Some(alias) = &to_endpoint.alias {
                                if alias == entity_str {
                                    return true;
                                }
                            }
                        }
                        false
                    });

                    if !field_exists_in_joins {
                        return ApiResponse {
                            success: false,
                            message: format!(
                                "advance_filters[{}] > field > Filter field '{}' in entity '{}' conflicts with JOIN 'to' field. Filtered fields cannot reference JOIN 'to' fields, their aliases, or entities",
                                filter_index, field, entity_str
                            ),
                            count: 0,
                            data: vec![],
                        };
                    }

                    // Validate field exists in schema
                    if !field_exists_in_table(entity_str, field) && !field_exists_in_joins {
                        return ApiResponse {
                             success: false,
                             message: format!(
                                 "advance_filters@@@@{}] > field > Filter field '{}' does not exist in entity '{}'",
                                 filter_index, field, entity_str
                             ),
                             count: 0,
                             data: vec![],
                         };
                    }

                    // Validate values are not empty for most operators
                    if values.is_empty()
                        && !matches!(
                            operator,
                            crate::structs::structs::FilterOperator::IsNull
                                | crate::structs::structs::FilterOperator::IsNotNull
                                | crate::structs::structs::FilterOperator::IsEmpty
                                | crate::structs::structs::FilterOperator::IsNotEmpty
                                | crate::structs::structs::FilterOperator::HasNoValue
                        )
                    {
                        return ApiResponse {
                             success: false,
                             message: format!(
                                 "advance_filters[{}] > values > Filter values cannot be empty for operator '{:?}' on field '{}'",
                                 filter_index, operator, field
                             ),
                             count: 0,
                             data: vec![],
                         };
                    }
                }
                FilterCriteria::LogicalOperator { .. } => {
                    // LogicalOperator variant doesn't contain field references to validate
                    continue;
                }
            }
        }

        ApiResponse {
            success: true,
            message: "Successfully validated advance_filters".to_string(),
            count: 0,
            data: vec![],
        }
    }

    pub fn validate_group_advance_filters(&self) -> ApiResponse {
        for (group_index, group_filter) in
            self.request_body.group_advance_filters.iter().enumerate()
        {
            let filters = match group_filter {
                crate::structs::structs::GroupAdvanceFilter::Criteria { filters, .. } => filters,
                crate::structs::structs::GroupAdvanceFilter::Operator { filters, .. } => filters,
            };

            for (filter_index, filter) in filters.iter().enumerate() {
                match filter {
                    FilterCriteria::Criteria {
                        field,
                        entity,
                        operator,
                        values,
                        ..
                    } => {
                        // Skip validation if entity is None
                        let entity_str = match entity {
                            Some(e) => e,
                            None => continue,
                        };

                        // Check if this field is a concatenated field and skip validation if it is
                        let is_concatenated_field = self
                            .request_body
                            .concatenate_fields
                            .iter()
                            .any(|concat_field| {
                                concat_field.field_name == *field
                                    && (concat_field.entity == *entity_str
                                        || concat_field.aliased_entity.as_ref() == Some(entity_str))
                            });

                        if is_concatenated_field {
                            // Skip validation for concatenated fields as they are virtual fields
                            continue;
                        }

                        // Validate field exists in schema
                        if !field_exists_in_table(entity_str, field) {
                            return ApiResponse {
                                 success: false,
                                 message: format!(
                                     "group_advance_filters[{}] > filters[{}] > field > Group filter field '{}' does not exist in entity '{}'",
                                     group_index, filter_index, field, entity_str
                                 ),
                                 count: 0,
                                 data: vec![],
                             };
                        }

                        // Validate values are not empty for most operators
                        if values.is_empty()
                            && !matches!(
                                operator,
                                crate::structs::structs::FilterOperator::IsNull
                                    | crate::structs::structs::FilterOperator::IsNotNull
                                    | crate::structs::structs::FilterOperator::IsEmpty
                                    | crate::structs::structs::FilterOperator::IsNotEmpty
                                    | crate::structs::structs::FilterOperator::HasNoValue
                            )
                        {
                            return ApiResponse {
                                 success: false,
                                 message: format!(
                                     "group_advance_filters[{}] > filters[{}] > values > Group filter values cannot be empty for operator '{:?}' on field '{}'",
                                     group_index, filter_index, operator, field
                                 ),
                                 count: 0,
                                 data: vec![],
                             };
                        }
                    }
                    FilterCriteria::LogicalOperator { .. } => {
                        // LogicalOperator variant doesn't contain field references to validate
                        continue;
                    }
                }
            }
        }

        ApiResponse {
            success: true,
            message: "Successfully validated group_advance_filters".to_string(),
            count: 0,
            data: vec![],
        }
    }
}
