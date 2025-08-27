use log::debug;

pub fn construct_permission_select_where_clause(
    tables: &[String],
    main_fields: &[String],
) -> String {
    let mut with_specific_fields = String::new();

    if !main_fields.is_empty() {
        let fields_quoted: Vec<String> = main_fields
            .iter()
            .map(|field| format!("'{}'", field))
            .collect();

        with_specific_fields = format!("AND fields.name IN ({})", fields_quoted.join(","));

        debug!("main_fields: {}", main_fields.join(","));
    }

    let tables_quoted: Vec<String> = tables.iter().map(|table| format!("'{}'", table)).collect();

    format!(
        "AND ( 
           data_permissions.tombstone = 0 AND entities.name IN ({}) {})",
        tables_quoted.join(","),
        with_specific_fields
    )
}