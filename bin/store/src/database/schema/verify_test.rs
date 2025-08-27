#[cfg(test)]
mod tests {
    use crate::database::schema::verify::{field_exists_in_table, get_table_fields};
    #[test]
    fn test_field_exists() {
        // // Test with existing table and field
        // assert!(field_exists_in_table("contacts", "name"));

        // // Test with existing table but non-existing field
        // assert!(!field_exists_in_table("items", "nonexistent_field"));

        // // Test with non-existing table
        // assert!(!field_exists_in_table("nonexistent_table", "name"));
    }

    #[test]
    fn test_get_table_fields() {
        // Test with existing table
        if let Some(fields) = get_table_fields("items") {
            assert!(!fields.is_empty());
            // Check that common fields exist
            assert!(fields.contains(&"id".to_string()));
        }

        // Test with non-existing table
        assert!(get_table_fields("nonexistent_table").is_none());
    }
}
