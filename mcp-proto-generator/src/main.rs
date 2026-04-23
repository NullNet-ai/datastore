use convert_case::{Case, Casing};
use regex::Regex;
use serde_json::{json, Value};
use std::io::{self, BufRead, Write};

#[derive(Debug, Clone)]
struct Field {
    name: String,
    proto_type: String,
    is_optional: bool,
    is_array: bool,
}

#[derive(Debug, Clone)]
struct Table {
    name: String,
    fields: Vec<Field>,
}

struct MCPServer {
    name: String,
    version: String,
}

impl MCPServer {
    fn new() -> Self {
        Self {
            name: "proto-generator".to_string(),
            version: "1.0.0".to_string(),
        }
    }

    fn handle_request(&self, request: Value) -> Option<Value> {
        let method = request["method"].as_str().unwrap_or("");
        let id = request["id"].clone();

        match method {
            "initialize" => Some(self.handle_initialize(id)),
            "tools/list" => Some(self.handle_tools_list(id)),
            "tools/call" => Some(self.handle_tools_call(id, &request["params"])),
            "notifications/initialized" => {
                // This is a notification, no response needed
                None
            }
            _ => Some(json!({
                "jsonrpc": "2.0",
                "id": id,
                "error": {
                    "code": -32601,
                    "message": "Method not found"
                }
            })),
        }
    }

    fn handle_initialize(&self, id: Value) -> Value {
        json!({
            "jsonrpc": "2.0",
            "id": id,
            "result": {
                "protocolVersion": "2024-11-05",
                "capabilities": {
                    "tools": {}
                },
                "serverInfo": {
                    "name": self.name,
                    "version": self.version
                }
            }
        })
    }

    fn handle_tools_list(&self, id: Value) -> Value {
        json!({
            "jsonrpc": "2.0",
            "id": id,
            "result": {
                "tools": [
                    {
                        "name": "generate_proto_from_schema",
                        "description": "Generate Protocol Buffer (.proto) files from Diesel schema content",
                        "inputSchema": {
                            "type": "object",
                            "properties": {
                                "schema_content": {
                                    "type": "string",
                                    "description": "The Diesel schema content (table! macro definitions)"
                                },
                                "package_name": {
                                    "type": "string",
                                    "description": "Package name for the proto file (optional, defaults to 'generated')",
                                    "default": "generated"
                                }
                            },
                            "required": ["schema_content"]
                        }
                    }
                ]
            }
        })
    }

    fn handle_tools_call(&self, id: Value, params: &Value) -> Value {
        let tool_name = params["name"].as_str().unwrap_or("");
        let arguments = &params["arguments"];

        match tool_name {
            "generate_proto_from_schema" => self.generate_proto_from_schema(id, arguments),
            _ => json!({
                "jsonrpc": "2.0",
                "id": id,
                "error": {
                    "code": -32602,
                    "message": "Invalid tool name"
                }
            }),
        }
    }

    fn generate_proto_from_schema(&self, id: Value, arguments: &Value) -> Value {
        let schema_content = match arguments["schema_content"].as_str() {
            Some(content) => content,
            None => {
                return json!({
                    "jsonrpc": "2.0",
                    "id": id,
                    "error": {
                        "code": -32602,
                        "message": "Missing required parameter: schema_content"
                    }
                });
            }
        };

        let package_name = arguments["package_name"].as_str().unwrap_or("generated");

        match self.parse_and_generate_proto(schema_content, package_name) {
            Ok(proto_content) => {
                json!({
                    "jsonrpc": "2.0",
                    "id": id,
                    "result": {
                        "content": [
                            {
                                "type": "text",
                                "text": proto_content
                            }
                        ]
                    }
                })
            }
            Err(e) => {
                json!({
                    "jsonrpc": "2.0",
                    "id": id,
                    "error": {
                        "code": -32603,
                        "message": format!("Failed to generate proto: {}", e)
                    }
                })
            }
        }
    }

    fn parse_and_generate_proto(
        &self,
        schema_content: &str,
        package_name: &str,
    ) -> Result<String, String> {
        let tables = self.parse_tables(schema_content)?;
        if tables.is_empty() {
            return Err("No tables found in schema".to_string());
        }
        Ok(self.generate_unified_proto(&tables, package_name))
    }

    fn parse_tables(&self, schema: &str) -> Result<Vec<Table>, String> {
        let mut tables = Vec::new();

        // Regex to match Diesel table! macro definitions
        let table_regex =
            Regex::new(r"table!\s*\{\s*([a-zA-Z_][a-zA-Z0-9_]*)\s*\([^)]*\)\s*\{([^}]+)\}\s*\}")
                .map_err(|e| format!("Regex error: {}", e))?;

        for cap in table_regex.captures_iter(schema) {
            let table_name = cap[1].to_string();
            let columns_str = &cap[2];

            let fields = self.parse_diesel_columns(columns_str)?;

            tables.push(Table {
                name: table_name,
                fields,
            });
        }

        Ok(tables)
    }

    fn parse_diesel_columns(&self, columns_str: &str) -> Result<Vec<Field>, String> {
        let mut fields = Vec::new();

        // Split by commas and parse Diesel column definitions
        let lines: Vec<&str> = columns_str.split(',').collect();

        for line in lines {
            let line = line.trim();
            if line.is_empty() {
                continue;
            }

            // Parse Diesel column definition: column_name -> Type,
            // Example: id -> Integer,
            // Example: name -> Nullable<Text>,
            let parts: Vec<&str> = line.split("->").collect();
            if parts.len() != 2 {
                continue;
            }

            let column_name = parts[0].trim().to_string();
            let column_type = parts[1].trim().trim_end_matches(',').to_string();

            let is_optional = column_type.starts_with("Nullable<");
            let is_array = column_type.contains("Array<");

            // Extract the inner type if it's wrapped in Nullable<> or Array<>
            let inner_type = if is_optional {
                column_type
                    .strip_prefix("Nullable<")
                    .and_then(|s| s.strip_suffix(">"))
                    .unwrap_or(&column_type)
            } else if is_array {
                column_type
                    .strip_prefix("Array<")
                    .and_then(|s| s.strip_suffix(">"))
                    .unwrap_or(&column_type)
            } else {
                &column_type
            };

            let proto_type = self.diesel_type_to_proto_type(inner_type);

            fields.push(Field {
                name: column_name,
                proto_type,
                is_optional,
                is_array,
            });
        }

        Ok(fields)
    }

    fn diesel_type_to_proto_type(&self, diesel_type: &str) -> String {
        match diesel_type {
            "Integer" => "int32".to_string(),
            "BigInt" => "int64".to_string(),
            "SmallInt" => "int32".to_string(),
            "Float" => "float".to_string(),
            "Double" => "double".to_string(),
            "Numeric" => "double".to_string(),
            "Bool" => "bool".to_string(),
            "Text" => "string".to_string(),
            "VarChar" => "string".to_string(),
            "Char" => "string".to_string(),
            "Uuid" => "string".to_string(),
            "Timestamp" => "Timestamp".to_string(),
            "Timestamptz" => "Timestamp".to_string(),
            "Date" => "string".to_string(),
            "Time" => "string".to_string(),
            "Timetz" => "string".to_string(),
            "Json" => "string".to_string(),
            "Jsonb" => "string".to_string(),
            "Binary" => "bytes".to_string(),
            "Bytea" => "bytes".to_string(),
            _ => "string".to_string(),
        }
    }

    fn to_singular(&self, word: &str) -> String {
        if word.ends_with("ies") {
            format!("{}y", &word[..word.len() - 3])
        } else if word.ends_with("s") && !word.ends_with("ss") {
            word[..word.len() - 1].to_string()
        } else {
            word.to_string()
        }
    }

    fn generate_unified_proto(&self, tables: &[Table], package_name: &str) -> String {
        let mut proto = String::new();

        // Header
        proto.push_str("syntax = \"proto3\";\n\n");
        proto.push_str("// Generated automatically from database schema\n");
        proto.push_str("// Do not edit manually\n\n");
        proto.push_str(&format!("package {};\n\n", package_name));

        // Timestamp definition
        proto.push_str("// Standard timestamp representation\n");
        proto.push_str("message Timestamp {\n");
        proto.push_str("  int64 seconds = 1; // Seconds since epoch\n");
        proto.push_str("  int32 nanos = 2;   // Nanoseconds offset\n");
        proto.push_str("}\n\n");

        // Common structures for Create requests
        proto.push_str("// Common parameter structure for Create requests\n");
        proto.push_str("message CreateParams {\n");
        proto.push_str("  string table = 1; // Table name\n");
        proto.push_str("}\n\n");

        proto.push_str("// Common query structure for Create requests\n");
        proto.push_str("message CreateQuery {\n");
        proto.push_str("  string pluck = 1; // Field to pluck (e.g., \"id\")\n");
        proto.push_str("  string durability = 2; // Durability level (e.g., \"soft\")\n");
        proto.push_str("}\n\n");

        // Common structure for AdvanceFilter
        proto.push_str("// Common parameter structure for AdvanceFilter requests\n");
        proto.push_str("message AdvanceFilter {\n");
        proto.push_str("  string entity = 1; // Table name\n");
        proto.push_str("  string type = 2; // Filter type criteria or operator\n");
        proto.push_str("  string field = 3; // Column name\n");
        proto.push_str("  string operator = 4; // Equal, not equal etc\n");
        proto.push_str("  string values = 5; // JSON string of values\n");
        proto.push_str("}\n\n");

        // Add other common structures
        proto.push_str("// Common parameter structure for Update requests\n");
        proto.push_str("message UpdateParams {\n");
        proto.push_str("  string id = 1; // Record ID\n");
        proto.push_str("  string table = 2; // Table name\n");
        proto.push_str("}\n\n");

        proto.push_str("// Common query structure for Update requests\n");
        proto.push_str("message UpdateQuery {\n");
        proto.push_str("  string pluck = 1; // Field to pluck (e.g., \"id,code\")\n");
        proto.push_str("}\n\n");

        proto.push_str("// Common query structure for Delete requests\n");
        proto.push_str("message DeleteQuery {\n");
        proto.push_str("  string is_permanent = 1; // Whether to perform permanent deletion\n");
        proto.push_str("}\n\n");

        proto.push_str("// Common parameter structure for Delete requests\n");
        proto.push_str("message DeleteParams {\n");
        proto.push_str("  string id = 1; // Record ID\n");
        proto.push_str("  string table = 2; // Table name\n");
        proto.push_str("}\n\n");

        // Generate all data messages first
        for table in tables {
            proto.push_str(&format!(
                "// {} entity definition\n",
                table.name.to_case(Case::Pascal)
            ));
            proto.push_str(&format!(
                "message {} {{\n",
                table.name.to_case(Case::Pascal)
            ));

            // Fields with comments
            for (i, field) in table.fields.iter().enumerate() {
                let field_number = i + 1;
                let type_prefix = if field.is_optional { "optional " } else { "" };
                let type_prefix = if field.is_array {
                    "repeated "
                } else {
                    type_prefix
                };

                proto.push_str(&format!(
                    "  {}{} {} = {};\n",
                    type_prefix,
                    field.proto_type,
                    field.name.to_case(Case::Snake),
                    field_number
                ));
            }
            proto.push_str("}\n\n");
        }

        // Generate request/response messages for CRUD operations
        for table in tables {
            let pascal_name = table.name.to_case(Case::Pascal);
            let snake_name = table.name.to_case(Case::Snake);
            let singular_name = self.to_singular(&table.name);

            // Create operation
            proto.push_str(&format!("// Create {} request\n", pascal_name));
            proto.push_str(&format!("message Create{}Request {{\n", pascal_name));
            proto.push_str(&format!("  {} {} = 1;\n", pascal_name, snake_name));
            proto.push_str("  CreateParams params = 2;\n");
            proto.push_str("  CreateQuery query = 3;\n");
            proto.push_str("}\n\n");

            // Create response
            proto.push_str(&format!("// Create {} response\n", pascal_name));
            proto.push_str(&format!("message Create{}Response {{\n", pascal_name));
            proto.push_str("  bool success = 1;\n");
            proto.push_str("  int32 count = 2;\n");
            proto.push_str("  string message = 3;\n");
            proto.push_str(&format!("  {} data = 4;\n", pascal_name));
            proto.push_str("}\n\n");

            // Get operation
            proto.push_str(&format!("// Get {} request\n", pascal_name));
            proto.push_str(&format!("message Get{}Request {{\n", pascal_name));
            proto.push_str("  string id = 1;\n");
            proto.push_str("}\n\n");

            proto.push_str(&format!("// Get {} response\n", pascal_name));
            proto.push_str(&format!("message Get{}Response {{\n", pascal_name));
            proto.push_str("  bool success = 1;\n");
            proto.push_str("  string message = 2;\n");
            proto.push_str(&format!("  {} data = 3;\n", pascal_name));
            proto.push_str("}\n\n");

            // Update operation
            proto.push_str(&format!("// Update {} request\n", pascal_name));
            proto.push_str(&format!("message Update{}Request {{\n", pascal_name));
            proto.push_str(&format!("  {} {} = 1;\n", pascal_name, singular_name));
            proto.push_str("  UpdateParams params = 2;\n");
            proto.push_str("  UpdateQuery query = 3;\n");
            proto.push_str("}\n\n");

            proto.push_str(&format!("// Update {} response\n", pascal_name));
            proto.push_str(&format!("message Update{}Response {{\n", pascal_name));
            proto.push_str("  bool success = 1;\n");
            proto.push_str("  int32 count = 2;\n");
            proto.push_str("  string message = 3;\n");
            proto.push_str(&format!("  {} data = 4;\n", pascal_name));
            proto.push_str("}\n\n");

            // Delete operation
            proto.push_str(&format!("// Delete {} request\n", pascal_name));
            proto.push_str(&format!("message Delete{}Request {{\n", pascal_name));
            proto.push_str("  DeleteParams params = 1;\n");
            proto.push_str("  DeleteQuery query = 2;\n");
            proto.push_str("}\n\n");

            proto.push_str(&format!("// Delete {} response\n", pascal_name));
            proto.push_str(&format!("message Delete{}Response {{\n", pascal_name));
            proto.push_str("  bool success = 1;\n");
            proto.push_str("  string message = 2;\n");
            proto.push_str("  int32 count = 3;\n");
            proto.push_str(&format!("  {} data = 4;\n", pascal_name));
            proto.push_str("}\n\n");
        }

        // Generate unified service with full CRUD operations
        proto.push_str("// Store service definition with CRUD operations\n");
        proto.push_str(&format!(
            "service {}Service {{\n",
            package_name.to_case(Case::Pascal)
        ));

        for table in tables {
            let pascal_name = table.name.to_case(Case::Pascal);

            // Create
            proto.push_str(&format!("  // Create a new {}\n", pascal_name));
            proto.push_str(&format!(
                "  rpc Create{}(Create{}Request) returns (Create{}Response);\n\n",
                pascal_name, pascal_name, pascal_name
            ));

            // Get
            proto.push_str(&format!("  // Get a {} by ID\n", pascal_name));
            proto.push_str(&format!(
                "  rpc Get{}(Get{}Request) returns (Get{}Response);\n\n",
                pascal_name, pascal_name, pascal_name
            ));

            // Update
            proto.push_str(&format!("  // Update an existing {}\n", pascal_name));
            proto.push_str(&format!(
                "  rpc Update{}(Update{}Request) returns (Update{}Response);\n\n",
                pascal_name, pascal_name, pascal_name
            ));

            // Delete
            proto.push_str(&format!("  // Delete a {} by ID\n", pascal_name));
            proto.push_str(&format!(
                "  rpc Delete{}(Delete{}Request) returns (Delete{}Response);\n\n",
                pascal_name, pascal_name, pascal_name
            ));
        }

        proto.push_str("}\n");

        proto
    }
}

fn main() -> io::Result<()> {
    let server = MCPServer::new();
    let stdin = io::stdin();
    let mut stdout = io::stdout();

    for line in stdin.lock().lines() {
        let line = line?;
        if line.trim().is_empty() {
            continue;
        }

        match serde_json::from_str::<Value>(&line) {
            Ok(request) => {
                if let Some(response) = server.handle_request(request) {
                    let response_str = serde_json::to_string(&response).unwrap();
                    writeln!(stdout, "{}", response_str)?;
                    stdout.flush()?;
                }
                // If None is returned (notification), no response is sent
            }
            Err(e) => {
                let error_response = json!({
                    "jsonrpc": "2.0",
                    "id": null,
                    "error": {
                        "code": -32700,
                        "message": format!("Parse error: {}", e)
                    }
                });
                let response_str = serde_json::to_string(&error_response).unwrap();
                writeln!(stdout, "{}", response_str)?;
                stdout.flush()?;
            }
        }
    }

    Ok(())
}
