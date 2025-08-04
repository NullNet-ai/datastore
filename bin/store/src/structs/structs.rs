use crate::sync::hlc::mutable_timestamp::MutableTimestamp;
use crate::schema::forbidden_tables;
use actix_web::{HttpResponse, ResponseError};
use chrono::Utc;
use diesel::sql_types::Text;
use diesel::AsExpression;
use merkle::MerkleTree;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::collections::HashMap;
use ulid::Ulid;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MatchPattern {
    /// Exact match - value as provided
    Exact,
    /// Prefix match - value%
    Prefix,
    /// Suffix match - %value
    Suffix,
    /// Contains match - %value%
    Contains,
    /// Custom pattern - use value as-is (allows manual % and _ placement)
    Custom,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ApiResponse {
    pub success: bool,
    pub message: String,
    pub count: i32,
    pub data: Vec<Value>,
}

impl ResponseError for ApiResponse {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::BadRequest().json(self) // Or Unauthorized, depending on context
    }
}

impl std::fmt::Display for ApiResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}: {}",
            if self.success { "Success" } else { "Error" },
            self.message
        )
    }
}

#[derive(Deserialize, Serialize, Clone)]
#[serde(transparent)]
pub struct RequestBody {
    pub record: Value,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct UpsertRequestBody {
    pub data: Value,
    pub conflict_columns: Vec<String>,
}

#[derive(Deserialize)]
pub struct BatchUpdateBody {
    pub advance_filters: Vec<Value>,
    pub updates: RequestBody,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct SwitchAccountRequest {
    pub data: SwitchAccountData,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct SwitchAccountData {
    pub token: String,
    pub organization_id: String,
}

#[derive(Debug)]
pub struct SqlUpdate {
    pub sql: String,
    pub params: Vec<serde_json::Value>,
}

impl RequestBody {
    // Process record with common fields and return a Value directly
    pub fn process_record(&mut self, operation: &str, auth: &Auth, is_root_account: bool, table: &str) {
        // // Add common fields to the record
        self.add_common_fields(operation, auth, is_root_account, table);

        if let Some(timestamp) = self.record.get_mut("timestamp") {
            if let Some(ts_str) = timestamp.as_str() {
                // Keep timestamp as-is if it's already properly formatted
                if chrono::DateTime::parse_from_rfc3339(ts_str).is_ok() {
                    // Timestamp is already valid, keep it
                    return;
                }
                // Remove any trailing Z, +00:00, etc. only if needed
                let cleaned_ts =
                    if ts_str.contains('T') && (ts_str.contains('Z') || ts_str.contains('+')) {
                        // Extract just the part before Z or +
                        let parts: Vec<&str> = ts_str.split(|c| c == 'Z' || c == '+').collect();
                        parts[0].to_string()
                    } else {
                        ts_str.to_string()
                    };
                *timestamp = json!(cleaned_ts);
            }
        } else {
            // Generate timestamp without timezone to match the cleaning logic
            let now = chrono::Utc::now();
            let formatted_timestamp = now.format("%Y-%m-%dT%H:%M:%S%.6f").to_string();
            self.record["timestamp"] = json!(formatted_timestamp);
        }
    }

    // Helper method to add common fields
    fn add_common_fields(&mut self, operation: &str, auth: &Auth, is_root_account: bool, table: &str) {
        // Get current time for timestamps
        let now = Utc::now();
        let date_str = now.format("%Y-%m-%d").to_string();
        let time_str = now.format("%H:%M:%S").to_string();

        // Set common fields

        match operation {
            "create" => {
                self.record["status"] = json!("Active");
                self.record["created_date"] = json!(date_str);
                self.record["created_time"] = json!(time_str);
                self.record["updated_date"] = json!(date_str);
                self.record["updated_time"] = json!(time_str);
                self.record["version"] = json!(1);
                self.record["tombstone"] = json!(0);
                self.record["organization_id"] = json!(auth.organization_id);
                self.record["created_by"] = json!(auth.responsible_account);
            }
            "update" => {
                self.record["updated_date"] = json!(date_str);
                self.record["updated_time"] = json!(time_str);
                self.record["updated_by"] = json!(auth.responsible_account);
                self.record["version"] = json!(0); // ! don't change this, it gets discarded in the end, it's just a placeholder
                
                // Add organization_id if conditions are met
                if !self.record.get("organization_id").is_some_and(|v| !v.is_null()) 
                    && !is_root_account 
                    && !forbidden_tables::is_forbidden_table(table) {
                    self.record["organization_id"] = json!(auth.organization_id);
                }
            }
            "delete" => {
                self.record["updated_date"] = json!(date_str);
                self.record["updated_time"] = json!(time_str);
                self.record["status"] = json!("Draft");
                self.record["updated_by"] = json!(auth.responsible_account);
                self.record["deleted_by"] = json!(auth.responsible_account);
                self.record["tombstone"] = json!(1);
            }
            _ => {
                // Handle other operations if needed
            }
        }

        if (operation == "create")
            && (!self.record.get("id").is_some()
                || self.record["id"].is_null()
                || self.record["id"]
                    .as_str()
                    .map_or(true, |s| s.trim().is_empty()))
        {
            self.record["id"] = json!(Ulid::new().to_string());
        }
    }
}

#[derive(Deserialize)]
pub struct QueryParams {
    #[serde(default = "default_pluck")]
    pub pluck: String,
}

fn default_pluck() -> String {
    "id".to_string()
}

#[derive(Clone)]
pub struct Clock {
    pub timestamp: MutableTimestamp,
    pub merkle: MerkleTree,
}

#[derive(Debug, AsExpression)]
#[diesel(sql_type = diesel::sql_types::Array<diesel::sql_types::Text>)]
pub enum ColumnValue {
    String(String),
    Array(Vec<String>),
    Integer(i32),
    Float(f64),
    Timestamp(chrono::DateTime<chrono::FixedOffset>),
    Boolean(bool),
    /// JSON/JSONB fields - preserves structured data
    Json(serde_json::Value),
    /// UUID fields
    Uuid(String),
    /// Binary data (bytea)
    Binary(Vec<u8>),
    /// Network addresses (inet, cidr)
    Network(String),
    /// Numeric/decimal with arbitrary precision
    Numeric(String),
    None,
}

#[derive(Debug, AsExpression)]
#[diesel(sql_type = Text)]
pub enum Id {
    Text(String),
    Uuid(Uuid),
    Ulid(Ulid),
}

impl Id {
    pub fn as_expression(
        &self,
    ) -> Box<dyn diesel::expression::Expression<SqlType = diesel::sql_types::Text>> {
        match self {
            Id::Text(text) => Box::new(diesel::dsl::sql::<diesel::sql_types::Text>(&format!(
                "'{}'",
                text
            ))),
            Id::Uuid(uuid) => Box::new(diesel::dsl::sql::<diesel::sql_types::Text>(&format!(
                "'{}'",
                uuid.to_string()
            ))),
            Id::Ulid(ulid) => Box::new(diesel::dsl::sql::<diesel::sql_types::Text>(&format!(
                "'{}'",
                ulid.to_string()
            ))),
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            Id::Text(text) => text.clone(),
            Id::Uuid(uuid) => uuid.to_string(),
            Id::Ulid(ulid) => ulid.to_string(),
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Endpoint {
    pub url: String,
    pub username: Option<String>,
    pub password: Option<String>,
    pub id: Option<String>,
    pub group_id: Option<String>,
    pub status: Option<String>,
}

#[derive(Clone, Debug)]
#[allow(warnings)]
pub struct Auth {
    pub organization_id: String,
    pub responsible_account: String,
    pub sensitivity_level: u32,
    pub role_name: String,
    pub account_organization_id: String,
    pub role_id: String,
    pub is_root_account: bool,
    pub account_id: String,
}

// get by filter
#[derive(Clone, Debug)]
#[allow(warnings)]

pub struct ParsedConcatenatedFields {
    pub fields: HashMap<String, Vec<String>>,
    pub expressions: HashMap<String, Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GroupBy {
    pub fields: Vec<String>,
    #[serde(default)]
    pub has_count: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GetByFilter {
    #[serde(default = "default_pluck_vec")]
    pub pluck: Vec<String>,

    #[serde(default)]
    pub pluck_object: HashMap<String, Vec<String>>,

    #[serde(default)]
    pub pluck_group_object: HashMap<String, Vec<String>>,

    #[serde(default)]
    pub advance_filters: Vec<FilterCriteria>,

    #[serde(default)]
    pub group_advance_filters: Vec<GroupAdvanceFilter>,

    #[serde(default)]
    pub joins: Vec<Join>,

    #[serde(default)]
    pub group_by: GroupBy,

    #[serde(default)]
    pub concatenate_fields: Vec<ConcatenateField>,

    #[serde(default)]
    pub multiple_sort: Vec<SortOption>,

    #[serde(default = "default_date_format")]
    pub date_format: String,

    #[serde(default = "default_order_by")]
    pub order_by: String,

    #[serde(default = "default_order_direction")]
    pub order_direction: String,

    #[serde(default)]
    pub is_case_sensitive_sorting: Option<bool>,

    #[serde(default = "default_offset")]
    pub offset: usize,

    #[serde(default = "default_limit")]
    pub limit: usize,

    pub distinct_by: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AggregationFilter {
    pub entity: String,

    pub aggregations: Vec<Aggregation>,

    #[serde(default)]
    pub advance_filters: Vec<FilterCriteria>,

    #[serde(default)]
    pub joins: Vec<Join>,

    #[serde(default = "default_limit")]
    pub limit: usize,

    pub bucket_size: Option<String>,

    pub timezone: Option<String>,

    pub order: Option<AggregationOrder>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Aggregation {
    pub aggregation: AggregationType,
    pub aggregate_on: String,
    pub bucket_name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum AggregationType {
    Sum,
    Avg,
    Count,
    Min,
    Max,
    StdDev,
    Variance,
    #[serde(rename = "ARRAY_AGG")]
    ArrayAgg,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AggregationOrder {
    pub order_by: String,
    pub order_direction: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConcatenateField {
    pub fields: Vec<String>,
    pub field_name: String,
    pub separator: String,
    pub entity: String,
    pub aliased_entity: Option<String>,
}
#[allow(warnings)]
impl ConcatenateField {
    /// Generates the SQL expression for this concatenate field.
    pub fn to_sql_expression(&self) -> String {
        let joined_fields = self
            .fields
            .iter()
            .map(|field| format!("COALESCE(\"joined_{}\".\"{}\", '')", self.entity, field))
            .collect::<Vec<_>>()
            .join(&format!(" || '{}' || ", self.separator));
        format!("({}) AS \"{}\"", joined_fields, self.field_name)
    }

    pub fn parse_main_concatenations(
        concatenate_fields: &[ConcatenateField],
        table_name: &str,
        mut plucked_fields: HashMap<String, serde_json::Value>,
    ) -> Result<Option<HashMap<String, serde_json::Value>>, actix_web::Error> {
        use crate::schema::verify;
        use actix_web::error::ErrorBadRequest;

        for field in concatenate_fields {
            if field.entity != table_name {
                continue;
            }

            // Check if all fields exist and are valid string types
            for f in &field.fields {
                let field_type_info =
                    verify::field_type_in_table(table_name, f).ok_or_else(|| {
                        ErrorBadRequest(format!(
                            "Field \"{}\" doesn't exist in the schema of {}",
                            f, table_name
                        ))
                    })?;

                let field_lower = f.to_lowercase();
                if field_type_info.field_type != "text"
                    || field_lower.ends_with("date")
                    || field_lower.contains("id")
                {
                    return Err(ErrorBadRequest(format!(
                        "Concatenated fields must be of type string. Verify the fields in {}",
                        table_name
                    )));
                }
            }

            let sql_expression = field.to_sql_expression();
            plucked_fields.insert(
                field.field_name.clone(),
                serde_json::Value::String(sql_expression),
            );
        }

        if plucked_fields.is_empty() {
            Ok(None)
        } else {
            Ok(Some(plucked_fields))
        }
    }

    pub fn parse_concatenate_fields(
        concatenate_fields: &[ConcatenateField],
        table: String,
    ) -> ParsedConcatenatedFields {
        let (fields, expressions) = concatenate_fields.iter().fold(
            (HashMap::new(), HashMap::new()),
            |(mut fields, mut expressions), field| {
                // Initialize vectors for this entity if they don't exist
                expressions
                    .entry(field.entity.clone())
                    .or_insert_with(Vec::new);
                fields.entry(field.entity.clone()).or_insert_with(Vec::new);

                // Build the concatenated SQL expression using the existing to_sql_expression method
                let mut concatenated_field = field.to_sql_expression();
                if field.entity == table {
                    //remove joined_ prefix from concatenated_field
                    concatenated_field = concatenated_field.replace("joined_", "");
                }

                // Store the expression
                expressions
                    .get_mut(&field.entity)
                    .unwrap()
                    .push(concatenated_field);

                // Store the field name in the fields HashMap if not already present
                if !fields
                    .get(&field.entity)
                    .unwrap()
                    .contains(&field.field_name)
                {
                    fields
                        .get_mut(&field.entity)
                        .unwrap()
                        .push(field.field_name.clone());
                }

                (fields, expressions)
            },
        );

        ParsedConcatenatedFields {
            fields,
            expressions,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SortOption {
    pub by_field: String,
    pub by_direction: String,
    #[serde(default)]
    pub is_case_sensitive_sorting: Option<bool>,
}

fn default_date_format() -> String {
    "mm/dd/YYYY".to_string()
}

fn default_limit() -> usize {
    10
}

fn default_order_direction() -> String {
    "asc".to_string()
}

fn default_order_by() -> String {
    "id".to_string()
}

fn default_offset() -> usize {
    0
}

fn default_pluck_vec() -> Vec<String> {
    vec!["id".to_string()]
}

fn default_group_operator() -> LogicalOperator {
    LogicalOperator::Or
}

fn default_case_sensitive() -> Option<bool> {
    Some(false)
}

fn default_parse_as() -> String {
    String::new()
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Join {
    pub r#type: String, // use r#type because `type` is a Rust keyword
    pub field_relation: FieldRelation,
    #[serde(default)]
    pub nested: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FieldRelation {
    pub to: RelationEndpoint,
    pub from: RelationEndpoint,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RelationEndpoint {
    pub entity: String,
    pub field: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alias: Option<String>,
    pub order_direction: Option<String>,
    pub order_by: Option<String>,
    pub limit: Option<usize>,
    pub offset: Option<usize>,
}

//advance filters
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum FilterCriteria {
    #[serde(rename = "criteria")]
    Criteria {
        field: String,
        entity: String,
        operator: FilterOperator,
        values: Vec<serde_json::Value>,
        #[serde(default = "default_case_sensitive")]
        case_sensitive: Option<bool>,
        #[serde(default = "default_parse_as")]
        parse_as: String,
        #[serde(default)]
        match_pattern: Option<MatchPattern>,
    },
    #[serde(rename = "operator")]
    LogicalOperator { operator: LogicalOperator },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum LogicalOperator {
    And,
    Or,
}
impl ToString for LogicalOperator {
    fn to_string(&self) -> String {
        match self {
            LogicalOperator::And => "and".to_string(),
            LogicalOperator::Or => "or".to_string(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum GroupAdvanceFilter {
    #[serde(rename = "criteria")]
    Criteria {
        #[serde(default = "default_group_operator")]
        operator: LogicalOperator,
        filters: Vec<FilterCriteria>,
    },
    #[serde(rename = "operator")]
    Operator {
        operator: LogicalOperator,
        #[serde(default)]
        filters: Vec<FilterCriteria>,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FilterOperator {
    #[serde(rename = "equal")]
    Equal,
    #[serde(rename = "not_equal")]
    NotEqual,
    #[serde(rename = "greater_than")]
    GreaterThan,
    #[serde(rename = "greater_than_or_equal")]
    GreaterThanOrEqual,
    #[serde(rename = "less_than")]
    LessThan,
    #[serde(rename = "less_than_or_equal")]
    LessThanOrEqual,
    #[serde(rename = "is_null")]
    IsNull,
    #[serde(rename = "is_not_null")]
    IsNotNull,
    #[serde(rename = "contains")]
    Contains,
    #[serde(rename = "not_contains")]
    NotContains,
    #[serde(rename = "is_between")]
    IsBetween,
    #[serde(rename = "is_not_between")]
    IsNotBetween,
    #[serde(rename = "is_empty")]
    IsEmpty,
    #[serde(rename = "is_not_empty")]
    IsNotEmpty,
    #[serde(rename = "like")]
    Like,
    #[serde(rename = "has_no_value")]
    HasNoValue,
}
