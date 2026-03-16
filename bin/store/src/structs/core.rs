use crate::database::schema::forbidden_tables;
use crate::database::schema::verify::field_type_in_table;
use crate::providers::operations::sync::hlc::mutable_timestamp::MutableTimestamp;
use actix_web::{HttpResponse, ResponseError};
use chrono::Utc;
use diesel::sql_types::Text;
use diesel::AsExpression;
use merkle::MerkleTree;
use serde::de::Error as _;
use serde::{Deserialize, Deserializer, Serialize};
use serde_json::{json, Value};
use std::collections::{BTreeMap, HashMap};
use ulid::Ulid;
use uuid::Uuid;
#[derive(Debug)]
/// Configuration structure for command-line arguments
pub struct CommandArgs {
    pub cleanup: bool,
    pub init_db: bool,
    pub generate_proto: bool,
    pub generate_grpc: bool,
    pub generate_table_enum: bool,
    pub create_schema: bool,
}

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
    pub advance_filters: Vec<FilterCriteria>,
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

/// Ensure the string has an RFC3339 timezone so chrono accepts it.
/// - "2026-02-27T10:30:00" or "2025-12-01 23:06:21.25" -> append "+00:00"
/// - "...+00" or "...-05" -> append ":00" so offset is full (+00:00, -05:00)
fn ensure_rfc3339_has_timezone(s: &str) -> String {
    let s = s.trim();
    if s.ends_with('Z') {
        return s.to_string();
    }
    // Already has full offset (+HH:MM or -HH:MM)
    if s.contains('+') || s.rfind('-').map_or(false, |i| i > 10) {
        // Normalize short offset e.g. "+00" -> "+00:00"
        if s.len() >= 3 {
            let rest = &s[s.len() - 3..];
            if (rest.starts_with('+') || rest.starts_with('-'))
                && rest[1..].chars().all(|c| c.is_ascii_digit())
            {
                return format!("{}:00", s);
            }
        }
        return s.to_string();
    }
    // No timezone: append +00:00 (and ensure space-separated has T for RFC3339)
    let with_t = if s.contains(' ') && !s.contains('T') {
        s.replace(' ', "T")
    } else {
        s.to_string()
    };
    format!("{}+00:00", with_t)
}

impl RequestBody {
    // Process record with common fields and return a Value directly
    pub fn process_record(
        &mut self,
        operation: &str,
        auth: &Auth,
        is_root_account: bool,
        table: &str,
    ) {
        // // Add common fields to the record
        self.add_common_fields(operation, auth, is_root_account, table);

        // Normalize all timestamp/timestamptz fields to RFC3339 so model deserialization and sync succeed
        self.normalize_timestamp_fields(table);

        if let Some(timestamp) = self.record.get_mut("timestamp") {
            if let Some(ts_str) = timestamp.as_str() {
                // If timestamp is RFC3339 compliant, convert to UTC and format without timezone
                if let Ok(parsed_ts) = chrono::DateTime::parse_from_rfc3339(ts_str) {
                    let utc_ts = parsed_ts.naive_utc();
                    let formatted = utc_ts.format("%Y-%m-%dT%H:%M:%S%.6f").to_string();
                    *timestamp = json!(formatted);
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

    /// Ensure every timestamp/timestamptz field in the record has an RFC3339 timezone so chrono
    /// deserialization and sync message parsing succeed (e.g. "2026-02-27T10:30:00" -> "...+00:00").
    fn normalize_timestamp_fields(&mut self, table: &str) {
        let obj = match self.record.as_object_mut() {
            Some(o) => o,
            None => return,
        };
        let keys: Vec<String> = obj.keys().cloned().collect();
        for key in keys {
            if let Some(info) = field_type_in_table(table, &key) {
                if info.field_type != "timestamp" {
                    continue;
                }
            } else {
                continue;
            }
            if let Some(Value::String(s)) = obj.get(&key) {
                let normalized = ensure_rfc3339_has_timezone(s);
                if normalized != *s {
                    obj.insert(key, Value::String(normalized));
                }
            }
        }
    }

    /// True if record has no id or id is null/empty/whitespace — then a new ULID will be assigned.
    /// Pass `null` or `""` (or omit `id`) to get an auto-generated id.
    fn id_absent_or_empty(record: &Value) -> bool {
        match record.get("id") {
            None => true,
            Some(v) if v.is_null() => true,
            Some(v) => v.as_str().map_or(true, |s| s.trim().is_empty()),
        }
    }

    // Helper method to add common fields
    fn add_common_fields(
        &mut self,
        operation: &str,
        auth: &Auth,
        is_root_account: bool,
        table: &str,
    ) {
        // When running migrations we must not touch system fields at all.
        // MIGRATION_MODE=true (or "1") disables automatic system field assignment.
        if std::env::var("MIGRATION_MODE")
            .ok()
            .map(|v| {
                let v = v.trim();
                v.eq_ignore_ascii_case("true") || v == "1"
            })
            .unwrap_or(false)
        {
            return;
        }

        // Get current time for timestamps
        let now = Utc::now();
        let date_str = now.format("%Y-%m-%d").to_string();
        let time_str = now.format("%H:%M:%S").to_string();

        // Set common fields

        match operation {
            "create" => {
                if !self
                    .record
                    .as_object()
                    .map_or(false, |obj| obj.contains_key("status"))
                {
                    self.record["status"] = json!("Active");
                }
                self.record["created_date"] = json!(date_str);
                self.record["created_time"] = json!(time_str);
                self.record["updated_date"] = json!(date_str);
                self.record["updated_time"] = json!(time_str);
                self.record["version"] = json!(1);
                self.record["tombstone"] = json!(0);
                // Only set organization_id if it doesn't already exist in the record
                if !self
                    .record
                    .get("organization_id")
                    .is_some_and(|v| !v.is_null())
                {
                    self.record["organization_id"] = json!(auth.organization_id);
                }
                self.record["created_by"] = json!(auth.responsible_account);
            }
            "update" => {
                self.record["updated_date"] = json!(date_str);
                self.record["updated_time"] = json!(time_str);
                self.record["updated_by"] = json!(auth.responsible_account);
                self.record["version"] = json!(0); // ! don't change this, it gets discarded in the end, it's just a placeholder

                // Add organization_id if conditions are met
                if !self
                    .record
                    .get("organization_id")
                    .is_some_and(|v| !v.is_null())
                    && !is_root_account
                    && !forbidden_tables::is_forbidden_table(table)
                {
                    self.record["organization_id"] = json!(auth.organization_id);
                }
            }
            "delete" => {
                self.record["updated_date"] = json!(date_str);
                self.record["updated_time"] = json!(time_str);
                // Set status to "Archived" only if not provided in the request body
                if self.record["status"].is_null()
                    || !self
                        .record
                        .as_object()
                        .map_or(true, |obj| obj.contains_key("status"))
                {
                    self.record["status"] = json!("Deleted");
                }
                self.record["updated_by"] = json!(auth.responsible_account);
                self.record["deleted_by"] = json!(auth.responsible_account);
                self.record["tombstone"] = json!(1);
            }
            _ => {
                // Handle other operations if needed
            }
        }

        // Assign new ULID when id is missing, null, or empty/whitespace (pass null or "" to mean "generate id")
        if operation == "create" && Self::id_absent_or_empty(&self.record) {
            self.record["id"] = json!(Ulid::new().to_string());
        }
    }
}

#[derive(Deserialize, Debug)]
pub struct QueryParams {
    #[serde(default = "default_pluck")]
    pub pluck: String,
}

fn default_pluck() -> String {
    "id".to_string()
}

#[derive(Clone)]
#[allow(dead_code)]
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

fn default_group_by_fields() -> Vec<String> {
    Vec::new()
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GroupBy {
    #[serde(default = "default_group_by_fields")]
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

    #[serde(
        default = "default_group_by",
        deserialize_with = "deserialize_group_by_option"
    )]
    pub group_by: Option<GroupBy>,

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

    pub timezone: Option<String>,

    #[serde(default = "default_time_format")]
    pub time_format: String,

    #[serde(default)]
    pub is_partitioned_table: Option<bool>,
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

    #[serde(default = "default_time_format")]
    pub time_format: String,
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
    /// Casts each field to text before COALESCE to avoid "invalid input syntax for type X" when
    /// null values are coalesced with '' (e.g. timestamptz/interval columns).
    pub fn to_sql_expression(&self) -> String {
        let joined_fields = self
            .fields
            .iter()
            .map(|field| {
                format!(
                    "COALESCE(\"joined_{}\".\"{}\"::text, '')",
                    self.entity, field
                )
            })
            .collect::<Vec<_>>()
            .join(&format!(" || '{}' || ", self.separator));
        format!("({}) AS \"{}\"", joined_fields, self.field_name)
    }

    /// Generates the expression for GROUP BY (no AS alias), using the given table reference.
    /// Casts each field to text before COALESCE to avoid "invalid input syntax for type X" when
    /// null values are coalesced with '' (e.g. timestamptz/interval columns).
    pub fn to_group_by_expression(&self, table_ref: &str) -> String {
        let parts = self
            .fields
            .iter()
            .map(|field| format!("COALESCE(\"{}\".\"{}\"::text, '')", table_ref, field))
            .collect::<Vec<_>>()
            .join(&format!(" || '{}' || ", self.separator));
        format!("({})", parts)
    }

    pub fn parse_main_concatenations(
        concatenate_fields: &[ConcatenateField],
        table_name: &str,
        mut plucked_fields: HashMap<String, serde_json::Value>,
    ) -> Result<Option<HashMap<String, serde_json::Value>>, actix_web::Error> {
        use crate::database::schema::verify;
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

fn default_group_by() -> Option<GroupBy> {
    Some(GroupBy {
        fields: Vec::new(),
        has_count: false,
    })
}

/// Deserialize Option<GroupBy> so that null or empty object `{}` becomes Some(GroupBy::default()).
fn deserialize_group_by_option<'de, D>(deserializer: D) -> Result<Option<GroupBy>, D::Error>
where
    D: Deserializer<'de>,
{
    let opt = Option::<Value>::deserialize(deserializer)?;
    match opt {
        None => Ok(default_group_by()),
        Some(Value::Object(ref map)) if map.is_empty() => Ok(default_group_by()),
        Some(Value::Object(ref map)) if map.get("fields").is_none() => Ok(default_group_by()),
        Some(v) => {
            let g: GroupBy = serde_json::from_value(v).map_err(D::Error::custom)?;
            Ok(Some(g))
        }
    }
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

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ETimeFormats {
    #[serde(rename = "HH24:MI:SS")]
    HH24MISS,
    #[serde(rename = "HH24:MI")]
    HH24MI,
    #[serde(rename = "HH12:MI")]
    HH12MI,
    #[serde(rename = "HH12:MI AM")]
    HH12MIAM,
    #[serde(rename = "HH12:MI:SS AM")]
    HH12MISSAM,
    #[serde(rename = "HH12:MI:SS")]
    HH12MISS,
}

impl ToString for ETimeFormats {
    fn to_string(&self) -> String {
        match self {
            ETimeFormats::HH24MISS => "HH24:MI:SS".to_string(),
            ETimeFormats::HH24MI => "HH24:MI".to_string(),
            ETimeFormats::HH12MI => "HH12:MI".to_string(),
            ETimeFormats::HH12MIAM => "HH12:MI AM".to_string(),
            ETimeFormats::HH12MISSAM => "HH12:MI:SS AM".to_string(),
            ETimeFormats::HH12MISS => "HH12:MI:SS".to_string(),
        }
    }
}

impl Default for ETimeFormats {
    fn default() -> Self {
        ETimeFormats::HH24MI
    }
}

fn default_time_format() -> String {
    ETimeFormats::default().to_string()
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
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub filters: Vec<FilterCriteria>,
}

//advance filters
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum FilterCriteria {
    #[serde(rename = "criteria")]
    Criteria {
        field: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        entity: Option<String>,
        operator: FilterOperator,
        values: Vec<serde_json::Value>,
        #[serde(default = "default_case_sensitive")]
        case_sensitive: Option<bool>,
        #[serde(default = "default_parse_as")]
        parse_as: String,
        #[serde(default)]
        match_pattern: Option<MatchPattern>,
        is_search: Option<bool>,
        #[serde(default)]
        has_group_count: Option<bool>,
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

// #[derive(Deserialize)]
// pub struct UploadFile {
//     pub file: String,
// }

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SearchSuggestionParams {
    #[serde(default)]
    pub advance_filters: Vec<FilterCriteria>,

    #[serde(default)]
    pub concatenate_fields: Vec<ConcatenateField>,

    #[serde(default = "default_date_format")]
    pub date_format: String,

    #[serde(default)]
    pub group_advance_filters: Vec<GroupAdvanceFilter>,

    #[serde(default)]
    pub joins: Vec<Join>,

    #[serde(default = "default_limit")]
    pub limit: usize,

    #[serde(default = "default_offset")]
    pub offset: usize,

    #[serde(default)]
    pub pluck_object: BTreeMap<String, Vec<String>>,

    pub timezone: Option<String>,

    #[serde(default = "default_time_format")]
    pub time_format: String,
}

#[cfg(test)]
mod tests {
    use super::{Auth, RequestBody};
    use serde_json::json;
    use std::env;

    fn test_auth() -> Auth {
        Auth {
            organization_id: "01ARZ3NDEKTSV4RRFFQ69G5FAV".to_string(),
            responsible_account: "01ARZ3NDEKTSV4RRFFQ69G5FAV".to_string(),
            sensitivity_level: 1000,
            role_name: "super_admin".to_string(),
            account_organization_id: "01ARZ3NDEKTSV4RRFFQ69G5FAV".to_string(),
            role_id: "super_admin".to_string(),
            is_root_account: true,
            account_id: "admin@example.com".to_string(),
        }
    }

    #[test]
    fn process_record_create_assigns_id_when_id_missing() {
        // Ensure MIGRATION_MODE does not disable id generation
        env::remove_var("MIGRATION_MODE");

        let mut body = RequestBody {
            record: json!({ "name": "Test" }),
        };
        body.process_record("create", &test_auth(), true, "contacts");
        let id = body.record.get("id").and_then(|v| v.as_str()).unwrap();
        assert!(!id.is_empty(), "id should be assigned");
        assert!(
            ulid::Ulid::from_string(id).is_ok(),
            "id should be a valid ULID"
        );
    }

    #[test]
    fn process_record_create_assigns_id_when_id_null() {
        env::remove_var("MIGRATION_MODE");

        let mut body = RequestBody {
            record: json!({ "id": null, "name": "Test" }),
        };
        body.process_record("create", &test_auth(), true, "contacts");
        let id = body.record.get("id").and_then(|v| v.as_str()).unwrap();
        assert!(!id.is_empty(), "id should be assigned");
        assert!(
            ulid::Ulid::from_string(id).is_ok(),
            "id should be a valid ULID"
        );
    }

    #[test]
    fn process_record_create_assigns_id_when_id_empty_string() {
        env::remove_var("MIGRATION_MODE");

        let mut body = RequestBody {
            record: json!({ "id": "", "name": "Test" }),
        };
        body.process_record("create", &test_auth(), true, "contacts");
        let id = body.record.get("id").and_then(|v| v.as_str()).unwrap();
        assert!(!id.is_empty(), "id should be assigned (was empty string)");
        assert!(
            ulid::Ulid::from_string(id).is_ok(),
            "id should be a valid ULID"
        );
    }

    #[test]
    fn process_record_create_assigns_id_when_id_whitespace_only() {
        env::remove_var("MIGRATION_MODE");

        let mut body = RequestBody {
            record: json!({ "id": "   ", "name": "Test" }),
        };
        body.process_record("create", &test_auth(), true, "contacts");
        let id = body.record.get("id").and_then(|v| v.as_str()).unwrap();
        assert!(
            !id.trim().is_empty(),
            "id should be assigned (was whitespace)"
        );
        assert!(
            ulid::Ulid::from_string(id).is_ok(),
            "id should be a valid ULID"
        );
    }

    #[test]
    fn process_record_create_preserves_id_when_non_empty() {
        env::remove_var("MIGRATION_MODE");

        let existing_id = "01ARZ3NDEKTSV4RRFFQ69G5FAV";
        let mut body = RequestBody {
            record: json!({ "id": existing_id, "name": "Test" }),
        };
        body.process_record("create", &test_auth(), true, "contacts");
        let id = body.record.get("id").and_then(|v| v.as_str()).unwrap();
        assert_eq!(id, existing_id, "existing id should be preserved");
    }

    #[test]
    #[ignore]
    fn add_common_fields_skips_system_fields_when_migration_mode_enabled() {
        env::set_var("MIGRATION_MODE", "true");

        let mut body = RequestBody {
            record: json!({ "name": "Test" }),
        };
        let auth = test_auth();

        body.process_record("create", &auth, false, "contacts");
        let obj = body.record.as_object().expect("record should be an object");

        // System fields should not be injected when MIGRATION_MODE is enabled
        assert!(obj.get("status").is_none());
        assert!(obj.get("created_date").is_none());
        assert!(obj.get("created_time").is_none());
        assert!(obj.get("updated_date").is_none());
        assert!(obj.get("updated_time").is_none());
        assert!(obj.get("version").is_none());
        assert!(obj.get("tombstone").is_none());
        assert!(obj.get("organization_id").is_none());
        assert!(obj.get("created_by").is_none());
        assert!(obj.get("updated_by").is_none());
        assert!(obj.get("deleted_by").is_none());

        // Original fields should be preserved
        assert_eq!(obj.get("name").unwrap(), "Test");

        // Clean up for other tests
        env::remove_var("MIGRATION_MODE");
    }
}
