// use serde::{Serialize, Deserialize};
// use serde_json::Value;
use crate::structs::structs::{ApiResponse, GetByFilter};

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
        // let mut keys = Self::get_keys_from_object(&self.request_body);
        // keys.append(&mut vec!["table".to_string()]);
        let required_keys = vec!["table", "pluck", "advance_filters:group_advance_filters"];
        let mut response = ApiResponse {
            success: true,
            message: format!(
                "Successfully validated request body with keys: {}",
                required_keys.join(", ")
            ),
            count: required_keys.len() as i32,
            data: vec![],
        };
        // Iterate through keys and validate each property that are required before proceeding
        for key in &required_keys {
            match *key {
                "table" => {
                    response = self.validate_table();
                    if !response.success {
                        return response;
                    }
                }
                "pluck" => {
                    response = self.validate_pluck();
                    if !response.success {
                        return response;
                    }
                }
                "advance_filters:group_advance_filters" => {
                    response = self.validate_conflicting_filters();
                    if !response.success {
                        return response;
                    }
                }
                // Add more validation cases as needed
                _ => {
                    // Handle unknown or unvalidated keys
                }
            }
        }

        response
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
}
