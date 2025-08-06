//! Comprehensive example showcasing all available Diesel types
//! This demonstrates the full range of types supported by the schema generator

use crate::schema::generator::diesel_schema_definition::{
    DieselTableDefinition, types::*
};
use crate::define_table_schema;

pub struct ComprehensiveExampleTable;

// Showcase of all available types
define_table_schema! {
    table_name: "comprehensive_examples",
    fields: {
        // Primary key
        id: integer(), primary_key: true,
        
        // Integer types
        small_number: nullable(smallint()),
        regular_number: nullable_integer(),
        big_number: nullable_bigint(),
        
        // Text types
        unlimited_text: nullable_text(),
        limited_varchar: text(),
        unlimited_varchar: text(),
        fixed_char: char(10),
        
        // Floating point types
        price: nullable(float()),
        precise_price: nullable(double()),
        currency_amount: nullable(numeric()),
        
        // Boolean types
        is_active: boolean(), default: "true",
        is_verified: nullable_boolean(),
        
        // Date and time types
        birth_date: nullable(date()),
        meeting_time: nullable(time()),
        created_timestamp: nullable_timestamp(),
        updated_timestamptz: nullable_timestamptz(),
        
        // JSON types
        simple_json: nullable(json()),
        structured_data: nullable_jsonb(), default: "{}",
        
        // Network types
        ip_address: nullable_inet(),
        network_cidr: nullable(cidr()),
        device_mac: nullable(macaddr()),
        
        // Other types
        file_data: nullable(binary()),
        unique_id: nullable_uuid(),
        
        // Array types
        tags: nullable_text_array(),
        scores: nullable_integer_array(),
        custom_array: nullable(array(text())),
        
        // Indexed fields
        search_key: text(), indexed: true,
        category_id: nullable_integer(), indexed: true,
        
        // Timestamps
        created_at: timestamptz(), default: "CURRENT_TIMESTAMP",
        updated_at: timestamptz(), default: "CURRENT_TIMESTAMP"
    },
    indexes: {
        idx_comprehensive_search: {
            columns: ["search_key", "category_id"],
            unique: false,
            type: "btree"
        },
        idx_comprehensive_unique_id: {
            columns: ["unique_id"],
            unique: true,
            type: "btree"
        },
        idx_comprehensive_tags: {
            columns: ["tags"],
            unique: false,
            type: "gin"
        }
    },
    foreign_keys: {
        fk_comprehensive_category_id: {
            columns: ["category_id"],
            foreign_table: "categories",
            foreign_columns: ["id"],
            on_delete: "SET NULL",
            on_update: "no action"
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::schema::generator::diesel_schema_definition::DieselType;

    #[test]
    fn test_comprehensive_table_definition() {
        let table_name = ComprehensiveExampleTable::table_name();
        assert_eq!(table_name, "comprehensive_examples");
        
        let fields = ComprehensiveExampleTable::field_definitions();
        assert!(!fields.is_empty());
        
        // Test some specific field types
        let id_field = fields.iter().find(|f| f.name == "id").unwrap();
        assert!(matches!(id_field.diesel_type, DieselType::Integer));
        assert!(id_field.is_primary_key);
        
        let varchar_field = fields.iter().find(|f| f.name == "limited_varchar").unwrap();
        assert!(matches!(varchar_field.diesel_type, DieselType::Text));
        
        let array_field = fields.iter().find(|f| f.name == "tags").unwrap();
        assert!(matches!(array_field.diesel_type, DieselType::Nullable(_)));
        
        let indexes = ComprehensiveExampleTable::indexes();
        assert_eq!(indexes.len(), 3);
        
        let foreign_keys = ComprehensiveExampleTable::foreign_keys();
        assert_eq!(foreign_keys.len(), 1);
        assert_eq!(foreign_keys[0].column, "category_id");
        assert_eq!(foreign_keys[0].references_table, "categories");
        assert_eq!(foreign_keys[0].references_column, "id");
    }
    
    #[test]
    fn test_type_helper_functions() {
        // Test integer types
        assert!(matches!(smallint(), DieselType::SmallInt));
        assert!(matches!(integer(), DieselType::Integer));
        assert!(matches!(bigint(), DieselType::BigInt));
        
        // Test text types
        assert!(matches!(text(), DieselType::Text));
        assert!(matches!(char(10), DieselType::Char(10)));
        
        // Test floating point types
        assert!(matches!(float(), DieselType::Float));
        assert!(matches!(double(), DieselType::Double));
        assert!(matches!(numeric(), DieselType::Numeric));
        
        // Test boolean type
        assert!(matches!(boolean(), DieselType::Bool));
        
        // Test date/time types
        assert!(matches!(date(), DieselType::Date));
        assert!(matches!(time(), DieselType::Time));
        assert!(matches!(timestamp(), DieselType::Timestamp));
        assert!(matches!(timestamptz(), DieselType::Timestamptz));
        
        // Test JSON types
        assert!(matches!(json(), DieselType::Json));
        assert!(matches!(jsonb(), DieselType::Jsonb));
        
        // Test network types
        assert!(matches!(inet(), DieselType::Inet));
        assert!(matches!(cidr(), DieselType::Cidr));
        assert!(matches!(macaddr(), DieselType::MacAddr));
        
        // Test other types
        assert!(matches!(binary(), DieselType::Binary));
        assert!(matches!(uuid(), DieselType::Uuid));
        
        // Test wrapper functions
        assert!(matches!(nullable(text()), DieselType::Nullable(_)));
        assert!(matches!(array(text()), DieselType::Array(_)));
        assert!(matches!(nullable_text(), DieselType::Nullable(_)));
        assert!(matches!(text_array(), DieselType::Array(_)));
        assert!(matches!(nullable_text_array(), DieselType::Nullable(_)));
    }
}