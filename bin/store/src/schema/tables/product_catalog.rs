

use crate::schema::generator::diesel_schema_definition::{
    DieselTableDefinition, types::*
};
use crate::define_table_schema;

/// Product catalog table for an e-commerce system
pub struct ProductCatalogTable;

define_table_schema! {
    table_name: "products",
    fields: {
        id: uuid(), primary_key: true,
        sku: text(),
        name: text(),
        description: nullable(text()),
        category_id: integer(),
        price: DieselType::Numeric,
        currency: DieselType::VarChar(Some(3)), default: "USD",
        stock_quantity: integer(), default: "0",
        is_active: boolean(), default: "true",
        is_featured: nullable(boolean()),
        weight_kg: nullable(DieselType::Float),
        dimensions: nullable(jsonb()),
        tags: nullable(array(text())),
        metadata: nullable(jsonb()), default: "{}",
        image_urls: nullable(array(text())),
        supplier_info: nullable(jsonb()),
        created_at: timestamptz(), default: "CURRENT_TIMESTAMP",
        updated_at: timestamptz(), default: "CURRENT_TIMESTAMP",
        deleted_at: nullable(timestamptz()),
        first_name: nullable(text())
    },
    indexes: {
        idx_products_sku: {
            columns: ["sku"],
            unique: true,
            type: "btree"
        },
        idx_products_category: {
            columns: ["category_id"],
            unique: false,
            type: "btree"
        },
        idx_products_active_featured: {
            columns: ["is_active", "is_featured"],
            unique: false,
            type: "btree"
        },
        idx_products_tags_gin: {
            columns: ["tags"],
            unique: false,
            type: "gin"
        },
        idx_products_metadata_gin: {
            columns: ["metadata"],
            unique: false,
            type: "gin"
        },
        idx_products_price: {
            columns: ["price"],
            unique: false,
            type: "btree"
        }
    },
    foreign_keys: {
        category_id -> "categories"."id",
        on_delete: "RESTRICT",
        on_update: "CASCADE"
    }
}

/// Alternative: Manual implementation for demonstration
pub struct ProductVariantsTable;

impl DieselTableDefinition for ProductVariantsTable {
    fn table_name() -> &'static str {
        "product_variants"
    }
    
    fn field_definitions() -> Vec<crate::schema::generator::diesel_schema_definition::DieselFieldDefinition> {
        use crate::schema::generator::diesel_schema_definition::{DieselFieldDefinition, DieselType};
        
        vec![
            DieselFieldDefinition {
                name: "id".to_string(),
                diesel_type: DieselType::Uuid,
                is_primary_key: true,
                is_nullable: false,
                default_value: Some("gen_random_uuid()".to_string()),
                is_indexed: false,
            },
            DieselFieldDefinition {
                name: "product_id".to_string(),
                diesel_type: DieselType::Uuid,
                is_primary_key: false,
                is_nullable: false,
                default_value: None,
                is_indexed: false,
            },
            DieselFieldDefinition {
                name: "variant_name".to_string(),
                diesel_type: DieselType::Text,
                is_primary_key: false,
                is_nullable: false,
                default_value: None,
                is_indexed: false,
            },
            DieselFieldDefinition {
                name: "variant_value".to_string(),
                diesel_type: DieselType::Text,
                is_primary_key: false,
                is_nullable: false,
                default_value: None,
                is_indexed: false,
            },
            DieselFieldDefinition {
                name: "price_adjustment".to_string(),
                diesel_type: DieselType::Nullable(Box::new(DieselType::Numeric)),
                is_primary_key: false,
                is_nullable: true,
                default_value: Some("0.00".to_string()),
                is_indexed: false,
            },
            DieselFieldDefinition {
                name: "stock_adjustment".to_string(),
                diesel_type: DieselType::Nullable(Box::new(DieselType::Integer)),
                is_primary_key: false,
                is_nullable: true,
                default_value: Some("0".to_string()),
                is_indexed: false,
            },
            DieselFieldDefinition {
                name: "is_default".to_string(),
                diesel_type: DieselType::Bool,
                is_primary_key: false,
                is_nullable: false,
                default_value: Some("false".to_string()),
                is_indexed: false,
            },
            DieselFieldDefinition {
                name: "sort_order".to_string(),
                diesel_type: DieselType::Nullable(Box::new(DieselType::Integer)),
                is_primary_key: false,
                is_nullable: true,
                default_value: Some("0".to_string()),
                is_indexed: false,
            },
            DieselFieldDefinition {
                name: "created_at".to_string(),
                diesel_type: DieselType::Timestamptz,
                is_primary_key: false,
                is_nullable: false,
                default_value: Some("CURRENT_TIMESTAMP".to_string()),
                is_indexed: false,
            },
        ]
    }
    
    fn indexes() -> Vec<crate::schema::generator::diesel_schema_definition::IndexDefinition> {
        use crate::schema::generator::diesel_schema_definition::IndexDefinition;
        
        vec![
            IndexDefinition {
                name: "idx_product_variants_product_id".to_string(),
                columns: vec!["product_id".to_string()],
                is_unique: false,
                index_type: Some("btree".to_string()),
            },
            IndexDefinition {
                name: "idx_product_variants_default".to_string(),
                columns: vec!["product_id".to_string(), "is_default".to_string()],
                is_unique: false,
                index_type: Some("btree".to_string()),
            },
            IndexDefinition {
                name: "idx_product_variants_sort".to_string(),
                columns: vec!["product_id".to_string(), "sort_order".to_string()],
                is_unique: false,
                index_type: Some("btree".to_string()),
            },
        ]
    }
    
    fn foreign_keys() -> Vec<crate::schema::generator::diesel_schema_definition::ForeignKeyDefinition> {
        use crate::schema::generator::diesel_schema_definition::ForeignKeyDefinition;
        
        vec![
            ForeignKeyDefinition {
                column: "product_id".to_string(),
                references_table: "products".to_string(),
                references_column: "id".to_string(),
                on_delete: Some("CASCADE".to_string()),
                on_update: Some("CASCADE".to_string()),
            },
        ]
    }
}