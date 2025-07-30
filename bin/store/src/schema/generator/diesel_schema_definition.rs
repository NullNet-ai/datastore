//! Improved schema definition using actual Diesel types
//! This replaces the comment-based approach with proper Rust structs

use serde::{Deserialize, Serialize};

/// Trait for defining table schemas using Diesel types
#[allow(dead_code)]
pub trait DieselTableDefinition {
    /// Get the table name
    fn table_name() -> &'static str;
    
    /// Get all field definitions for this table
    fn field_definitions() -> Vec<DieselFieldDefinition>;
    
    /// Get indexes for this table
    fn indexes() -> Vec<IndexDefinition> {
        Vec::new()
    }
    
    /// Get foreign key constraints
    fn foreign_keys() -> Vec<ForeignKeyDefinition> {
        Vec::new()
    }
}

/// Represents a field definition using Diesel's actual types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DieselFieldDefinition {
    pub name: String,
    pub diesel_type: DieselType,
    pub is_primary_key: bool,
    pub is_nullable: bool,
    pub default_value: Option<String>,
    pub is_indexed: bool,
}

/// Enum representing all supported Diesel SQL types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DieselType {
    // Integer types
    SmallInt,
    Integer,
    BigInt,
    
    // Text types
    Text,
    VarChar(Option<u32>),
    Char(u32),
    
    // Floating point
    Float,
    Double,
    Numeric,
    
    // Boolean
    Bool,
    
    // Date/Time types
    Date,
    Time,
    Timestamp,
    Timestamptz,
    
    // JSON types
    Json,
    Jsonb,
    
    // Network types
    Inet,
    Cidr,
    MacAddr,
    
    // Array types
    Array(Box<DieselType>),
    
    // Binary
    Binary,
    
    // UUID
    Uuid,
    
    // Nullable wrapper
    Nullable(Box<DieselType>),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndexDefinition {
    pub name: String,
    pub columns: Vec<String>,
    pub is_unique: bool,
    pub index_type: Option<String>, // btree, hash, gin, gist, etc.
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ForeignKeyDefinition {
    pub column: String,
    pub references_table: String,
    pub references_column: String,
    pub on_delete: Option<String>,
    pub on_update: Option<String>,
}

/// Macro to easily define table schemas
#[macro_export]
macro_rules! define_table_schema {
    (
        table_name: $table_name:literal,
        fields: {
            $(
                $field_name:ident: $field_type:expr
                $(, primary_key: $is_pk:expr)?
                $(, indexed: $is_indexed:expr)?
                $(, default: $default:expr)?
            ),* $(,)?
        }
        $(, indexes: {
            $(
                $index_name:ident: {
                    columns: [$($col:literal),*],
                    unique: $unique:expr
                    $(, type: $index_type:literal)?
                }
            ),* $(,)?
        })?
        $(, foreign_keys: {
            $(
                $fk_column:ident -> $ref_table:literal.$ref_column:literal
                $(, on_delete: $on_delete:literal)?
                $(, on_update: $on_update:literal)?
            ),* $(,)?
        })?
    ) => {
        impl DieselTableDefinition for Self {
            fn table_name() -> &'static str {
                $table_name
            }
            
            fn field_definitions() -> Vec<DieselFieldDefinition> {
                vec![
                    $(
                        DieselFieldDefinition {
                            name: stringify!($field_name).to_string(),
                            diesel_type: $field_type,
                            is_primary_key: $crate::define_table_schema!(@default_pk $($is_pk)?),
                            is_nullable: $crate::define_table_schema!(@is_nullable $field_type),
                            default_value: $crate::define_table_schema!(@default_val $($default)?),
                            is_indexed: $crate::define_table_schema!(@default_indexed $($is_indexed)?),
                        },
                    )*
                ]
            }
            
            $(
                fn indexes() -> Vec<IndexDefinition> {
                    vec![
                        $(
                            IndexDefinition {
                                name: stringify!($index_name).to_string(),
                                columns: vec![$($col.to_string()),*],
                                is_unique: $unique,
                                index_type: $crate::define_table_schema!(@index_type $($index_type)?),
                            },
                        )*
                    ]
                }
            )?
            
            $(
                fn foreign_keys() -> Vec<ForeignKeyDefinition> {
                    vec![
                        $(
                            ForeignKeyDefinition {
                                column: stringify!($fk_column).to_string(),
                                references_table: $ref_table.to_string(),
                                references_column: $ref_column.to_string(),
                                on_delete: $crate::define_table_schema!(@fk_action $($on_delete)?),
                                on_update: $crate::define_table_schema!(@fk_action $($on_update)?),
                            },
                        )*
                    ]
                }
            )?
        }
    };
    
    // Helper macros for default values
    (@default_pk) => { false };
    (@default_pk $val:expr) => { $val };
    
    (@default_indexed) => { false };
    (@default_indexed $val:expr) => { $val };
    
    (@default_val) => { None };
    (@default_val $val:expr) => { Some($val.to_string()) };
    
    (@index_type) => { None };
    (@index_type $val:expr) => { Some($val.to_string()) };
    
    (@fk_action) => { None };
    (@fk_action $val:expr) => { Some($val.to_string()) };
    
    (@is_nullable DieselType::Nullable($_:expr)) => { true };
    (@is_nullable $_:expr) => { false };
}

impl DieselType {
    /// Convert to Diesel schema.rs format
    #[allow(dead_code)]
    pub fn to_diesel_schema_type(&self) -> String {
        match self {
            DieselType::SmallInt => "Int2".to_string(),
            DieselType::Integer => "Int4".to_string(),
            DieselType::BigInt => "Int8".to_string(),
            DieselType::Text => "Text".to_string(),
            DieselType::VarChar(Some(len)) => format!("Varchar<{}>" , len),
            DieselType::VarChar(None) => "Varchar".to_string(),
            DieselType::Char(len) => format!("Char<{}>", len),
            DieselType::Float => "Float4".to_string(),
            DieselType::Double => "Float8".to_string(),
            DieselType::Numeric => "Numeric".to_string(),
            DieselType::Bool => "Bool".to_string(),
            DieselType::Date => "Date".to_string(),
            DieselType::Time => "Time".to_string(),
            DieselType::Timestamp => "Timestamp".to_string(),
            DieselType::Timestamptz => "Timestamptz".to_string(),
            DieselType::Json => "Json".to_string(),
            DieselType::Jsonb => "Jsonb".to_string(),
            DieselType::Inet => "Inet".to_string(),
            DieselType::Cidr => "Cidr".to_string(),
            DieselType::MacAddr => "MacAddr".to_string(),
            DieselType::Array(inner) => format!("Array<{}>", inner.to_diesel_schema_type()),
            DieselType::Binary => "Binary".to_string(),
            DieselType::Uuid => "Uuid".to_string(),
            DieselType::Nullable(inner) => format!("Nullable<{}>", inner.to_diesel_schema_type()),
        }
    }
    
    /// Convert to Rust type for model generation
    #[allow(dead_code)]
    pub fn to_rust_type(&self) -> String {
        match self {
            DieselType::SmallInt => "i16".to_string(),
            DieselType::Integer => "i32".to_string(),
            DieselType::BigInt => "i64".to_string(),
            DieselType::Text | DieselType::VarChar(_) | DieselType::Char(_) => "String".to_string(),
            DieselType::Float => "f32".to_string(),
            DieselType::Double => "f64".to_string(),
            DieselType::Numeric => "bigdecimal::BigDecimal".to_string(),
            DieselType::Bool => "bool".to_string(),
            DieselType::Date => "chrono::NaiveDate".to_string(),
            DieselType::Time => "chrono::NaiveTime".to_string(),
            DieselType::Timestamp => "chrono::NaiveDateTime".to_string(),
            DieselType::Timestamptz => "chrono::DateTime<chrono::Utc>".to_string(),
            DieselType::Json | DieselType::Jsonb => "serde_json::Value".to_string(),
            DieselType::Inet => "std::net::IpAddr".to_string(),
            DieselType::Cidr => "String".to_string(), // No standard Rust type for CIDR
            DieselType::MacAddr => "String".to_string(),
            DieselType::Array(inner) => format!("Vec<{}>", inner.to_rust_type()),
            DieselType::Binary => "Vec<u8>".to_string(),
            DieselType::Uuid => "uuid::Uuid".to_string(),
            DieselType::Nullable(inner) => format!("Option<{}>", inner.to_rust_type()),
        }
    }
    
    /// Get SQL type for migration
    #[allow(dead_code)]
    pub fn to_sql_type(&self) -> String {
        match self {
            DieselType::SmallInt => "SMALLINT".to_string(),
            DieselType::Integer => "INTEGER".to_string(),
            DieselType::BigInt => "BIGINT".to_string(),
            DieselType::Text => "TEXT".to_string(),
            DieselType::VarChar(Some(len)) => format!("VARCHAR({})", len),
            DieselType::VarChar(None) => "VARCHAR".to_string(),
            DieselType::Char(len) => format!("CHAR({})", len),
            DieselType::Float => "REAL".to_string(),
            DieselType::Double => "DOUBLE PRECISION".to_string(),
            DieselType::Numeric => "NUMERIC".to_string(),
            DieselType::Bool => "BOOLEAN".to_string(),
            DieselType::Date => "DATE".to_string(),
            DieselType::Time => "TIME".to_string(),
            DieselType::Timestamp => "TIMESTAMP".to_string(),
            DieselType::Timestamptz => "TIMESTAMPTZ".to_string(),
            DieselType::Json => "JSON".to_string(),
            DieselType::Jsonb => "JSONB".to_string(),
            DieselType::Inet => "INET".to_string(),
            DieselType::Cidr => "CIDR".to_string(),
            DieselType::MacAddr => "MACADDR".to_string(),
            DieselType::Array(inner) => format!("{}[]", inner.to_sql_type()),
            DieselType::Binary => "BYTEA".to_string(),
            DieselType::Uuid => "UUID".to_string(),
            DieselType::Nullable(inner) => inner.to_sql_type(), // Nullability is handled separately
        }
    }
}

/// Helper functions to create common Diesel types
#[allow(dead_code)]
pub mod types {
    use super::DieselType;
    
    // Wrapper functions
    pub fn nullable<T>(inner: T) -> DieselType 
    where 
        T: Into<DieselType>
    {
        DieselType::Nullable(Box::new(inner.into()))
    }
    
    pub fn array<T>(inner: T) -> DieselType 
    where 
        T: Into<DieselType>
    {
        DieselType::Array(Box::new(inner.into()))
    }
    
    // Integer types
    pub fn smallint() -> DieselType { DieselType::SmallInt }
    pub fn integer() -> DieselType { DieselType::Integer }
    pub fn bigint() -> DieselType { DieselType::BigInt }
    
    // Text types
    pub fn text() -> DieselType { DieselType::Text }
    pub fn varchar(length: Option<u32>) -> DieselType { DieselType::VarChar(length) }
    pub fn char(length: u32) -> DieselType { DieselType::Char(length) }
    
    // Floating point types
    pub fn float() -> DieselType { DieselType::Float }
    pub fn double() -> DieselType { DieselType::Double }
    pub fn numeric() -> DieselType { DieselType::Numeric }
    
    // Boolean type
    pub fn boolean() -> DieselType { DieselType::Bool }
    
    // Date and time types
    pub fn date() -> DieselType { DieselType::Date }
    pub fn time() -> DieselType { DieselType::Time }
    pub fn timestamp() -> DieselType { DieselType::Timestamp }
    pub fn timestamptz() -> DieselType { DieselType::Timestamptz }
    
    // JSON types
    pub fn json() -> DieselType { DieselType::Json }
    pub fn jsonb() -> DieselType { DieselType::Jsonb }
    
    // Network types
    pub fn inet() -> DieselType { DieselType::Inet }
    pub fn cidr() -> DieselType { DieselType::Cidr }
    pub fn macaddr() -> DieselType { DieselType::MacAddr }
    
    // Binary type
    pub fn binary() -> DieselType { DieselType::Binary }
    
    // UUID type
    pub fn uuid() -> DieselType { DieselType::Uuid }
    
    // Convenience functions for common nullable types
    pub fn nullable_text() -> DieselType { nullable(text()) }
    pub fn nullable_integer() -> DieselType { nullable(integer()) }
    pub fn nullable_bigint() -> DieselType { nullable(bigint()) }
    pub fn nullable_boolean() -> DieselType { nullable(boolean()) }
    pub fn nullable_timestamp() -> DieselType { nullable(timestamp()) }
    pub fn nullable_timestamptz() -> DieselType { nullable(timestamptz()) }
    pub fn nullable_jsonb() -> DieselType { nullable(jsonb()) }
    pub fn nullable_uuid() -> DieselType { nullable(uuid()) }
    pub fn nullable_inet() -> DieselType { nullable(inet()) }
    
    // Convenience functions for common array types
    pub fn text_array() -> DieselType { array(text()) }
    pub fn integer_array() -> DieselType { array(integer()) }
    pub fn nullable_text_array() -> DieselType { nullable(array(text())) }
    pub fn nullable_integer_array() -> DieselType { nullable(array(integer())) }
}

// Note: From<DieselType> for DieselType is automatically implemented by Rust