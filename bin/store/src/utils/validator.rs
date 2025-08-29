use crate::structs::core::ApiResponse;
use log::debug;

pub fn error_check_permission(account_id: &str, role: Option<&str>) -> Option<ApiResponse> {
    debug!(
        "Checking permissions for account_id: {}, role: {}",
        account_id,
        role.unwrap_or("None")
    );

    if role.is_none() || account_id.is_empty() {
        return Some(ApiResponse {
            success: false,
            message: format!(
                "Access denied: Although your role is {} ({}), you do not have the necessary permissions to access this resource.",
                role.unwrap_or("Unknown"),
                account_id
            ),
            count: 0,
            data: vec![],
        });
    }

    if let Some(role_value) = role {
        log::debug!(
            "As a Role {} ({}) has the necessary permissions to access this resource.",
            role_value,
            account_id
        );
    }
    None
}
