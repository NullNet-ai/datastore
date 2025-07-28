// use serde::{Serialize, Deserialize};
// use serde_json::Value;
use crate::structs::structs::{ApiResponse, GetByFilter, FilterCriteria};
use crate::schema::verify::field_exists_in_table;

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
    // Made this a static method since it doesn't need self
    // pub fn get_keys_from_object<T: Serialize>(obj: &T) -> Vec<String> {
    //     let value = serde_json::to_value(obj).unwrap();
    //     if let Value::Object(map) = value {
    //         map.keys().cloned().collect()
    //     } else {
    //         Vec::new()
    //     }
    // }

    pub fn exec(&self) -> ApiResponse {
        let validation_checks = vec![
            "table",
            "pluck", 
            "pluck_object",
            "advance_filters:group_advance_filters",
            "advance_filters",
            "group_advance_filters",
            "concatenated_fields",
            "joins",
            "order_by_format",
            "multiple_sort",
            "limit_offset"
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
                 "joins" => self.validate_joins(),
                 "order_by_format" => self.validate_order_by_format(),
                 "multiple_sort" => self.validate_multiple_sort(),
                 "limit_offset" => self.validate_limit_offset(),
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
        for concatenate_field in &self.request_body.concatenate_fields {
            if concatenate_field.fields.is_empty()
                || concatenate_field.field_name.is_empty()
                || concatenate_field.entity.is_empty()
            {
                return ApiResponse {
                    success: false,
                    message:
                        "Each concatenated field must have non-empty fields, field_name and entity"
                            .to_string(),
                    count: 0,
                    data: vec![],
                };
            }
        }

        return ApiResponse {
            success: true,
            message: "Successfully validated concatenated_fields".to_string(),
            count: 0,
            data: vec![],
        };
    }

    pub fn validate_pluck_object(&self) -> ApiResponse {
        // Validate pluck_object fields exist in schema
        for (entity, fields) in &self.request_body.pluck_object {
            for field in fields {
                if !field_exists_in_table(entity, field) {
                    return ApiResponse {
                        success: false,
                        message: format!(
                            "Field '{}' does not exist in entity '{}'",
                            field, entity
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
        for join in &self.request_body.joins {
            // Validate join type
            let join_type = join.r#type.to_uppercase();
            if join_type != "LEFT" && join_type != "SELF" {
                return ApiResponse {
                    success: false,
                    message: format!(
                        "Invalid join type: '{}'. Supported types are: LEFT, SELF",
                        join.r#type
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

            if !field_exists_in_table(from_entity, from_field) {
                return ApiResponse {
                    success: false,
                    message: format!(
                        "Join from field '{}' does not exist in entity '{}'",
                        from_field, from_entity
                    ),
                    count: 0,
                    data: vec![],
                };
            }

            if !field_exists_in_table(to_entity, to_field) {
                return ApiResponse {
                    success: false,
                    message: format!(
                        "Join to field '{}' does not exist in entity '{}'",
                        to_field, to_entity
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
        // Validate order_by format (should be "entity.field")
        if !self.request_body.order_by.is_empty() {
            let by_entity_field: Vec<&str> = self.request_body.order_by.split('.').collect();
            
            if by_entity_field.len() < 2 {
                return ApiResponse {
                    success: false,
                    message: format!(
                        "Invalid order_by format: '{}'. It should be separated by dot like 'table.field'",
                        self.request_body.order_by
                    ),
                    count: 0,
                    data: vec![],
                };
            }

            let sort_entity = by_entity_field[0];
            let sort_field = by_entity_field[1];

            // Validate field exists in schema
            if !field_exists_in_table(sort_entity, sort_field) {
                return ApiResponse {
                    success: false,
                    message: format!(
                        "Order by field '{}' does not exist in entity '{}'",
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

    pub fn validate_multiple_sort(&self) -> ApiResponse {
        for sort_option in &self.request_body.multiple_sort {
            // Validate sort field format
            let by_entity_field: Vec<&str> = sort_option.by_field.split('.').collect();
            
            if by_entity_field.len() < 2 {
                return ApiResponse {
                    success: false,
                    message: format!(
                        "Invalid sort field format: '{}'. It should be separated by dot like 'table.field'",
                        sort_option.by_field
                    ),
                    count: 0,
                    data: vec![],
                };
            }

            let sort_entity = by_entity_field[0];
            let sort_field = by_entity_field[1];

            // Validate field exists in schema
            if !field_exists_in_table(sort_entity, sort_field) {
                return ApiResponse {
                    success: false,
                    message: format!(
                        "Sort field '{}' does not exist in entity '{}'",
                        sort_field, sort_entity
                    ),
                    count: 0,
                    data: vec![],
                };
            }

            // Validate direction
            let direction = sort_option.by_direction.to_lowercase();
            if direction != "asc" && direction != "desc" && direction != "ascending" && direction != "descending" {
                return ApiResponse {
                    success: false,
                    message: format!(
                        "Invalid sort direction: '{}'. Valid values are: asc, desc, ascending, descending",
                        sort_option.by_direction
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
         for filter in &self.request_body.advance_filters {
             match filter {
                 FilterCriteria::Criteria { field, entity, operator, values, .. } => {
                     // Validate field exists in schema
                     if !field_exists_in_table(entity, field) {
                         return ApiResponse {
                             success: false,
                             message: format!(
                                 "Filter field '{}' does not exist in entity '{}'",
                                 field, entity
                             ),
                             count: 0,
                             data: vec![],
                         };
                     }

                     // Validate values are not empty for most operators
                     if values.is_empty() && !matches!(operator, crate::structs::structs::FilterOperator::IsNull | crate::structs::structs::FilterOperator::IsNotNull | crate::structs::structs::FilterOperator::IsEmpty | crate::structs::structs::FilterOperator::IsNotEmpty | crate::structs::structs::FilterOperator::HasNoValue) {
                         return ApiResponse {
                             success: false,
                             message: format!(
                                 "Filter values cannot be empty for operator '{:?}' on field '{}'",
                                 operator, field
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
         for group_filter in &self.request_body.group_advance_filters {
             let filters = match group_filter {
                 crate::structs::structs::GroupAdvanceFilter::Criteria { filters, .. } => filters,
                 crate::structs::structs::GroupAdvanceFilter::Operator { filters, .. } => filters,
             };

             for filter in filters {
                 match filter {
                     FilterCriteria::Criteria { field, entity, operator, values, .. } => {
                         // Validate field exists in schema
                         if !field_exists_in_table(entity, field) {
                             return ApiResponse {
                                 success: false,
                                 message: format!(
                                     "Group filter field '{}' does not exist in entity '{}'",
                                     field, entity
                                 ),
                                 count: 0,
                                 data: vec![],
                             };
                         }

                         // Validate values are not empty for most operators
                         if values.is_empty() && !matches!(operator, crate::structs::structs::FilterOperator::IsNull | crate::structs::structs::FilterOperator::IsNotNull | crate::structs::structs::FilterOperator::IsEmpty | crate::structs::structs::FilterOperator::IsNotEmpty | crate::structs::structs::FilterOperator::HasNoValue) {
                             return ApiResponse {
                                 success: false,
                                 message: format!(
                                     "Group filter values cannot be empty for operator '{:?}' on field '{}'",
                                     operator, field
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
