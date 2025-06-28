use log::debug;

pub fn construct_permission_select_where_clause(
    tables: &[String],
    main_fields: Option<&[String]>,
) -> String {
    let mut with_specific_fields = String::new();

    if let Some(fields) = main_fields {
        if !fields.is_empty() {
            let fields_quoted: Vec<String> =
                fields.iter().map(|field| format!("'{}'", field)).collect();

            with_specific_fields = format!("AND fields.name IN ({})", fields_quoted.join(","));
        }
    }

    if let Some(fields) = main_fields {
        if !fields.is_empty() {
            debug!("main_fields: {}", fields.join(","));
        }
    }

    let tables_quoted: Vec<String> = tables.iter().map(|table| format!("'{}'", table)).collect();

    format!(
        "AND ( 
           data_permissions.tombstone = 0 AND entities.name IN ({}) {})",
        tables_quoted.join(","),
        with_specific_fields
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_construct_permission_select_where_clause_with_fields() {
        let tables = vec!["users".to_string(), "accounts".to_string()];
        let main_fields = vec!["name".to_string(), "email".to_string()];

        let result = construct_permission_select_where_clause(&tables, Some(&main_fields));

        assert!(result.contains("entities.name IN ('users','accounts')"));
        assert!(result.contains("fields.name IN ('name','email')"));
    }

    #[test]
    fn test_construct_permission_select_where_clause_without_fields() {
        let tables = vec!["users".to_string()];

        let result = construct_permission_select_where_clause(&tables, None);

        assert!(result.contains("entities.name IN ('users')"));
        assert!(!result.contains("fields.name IN"));
    }
}
