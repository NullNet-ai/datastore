# MCP Proto Generator

A standalone Model Context Protocol (MCP) server that generates Protocol Buffer (.proto) files from Diesel schema content. This allows Claude to generate protobuf definitions directly from Diesel table! macro definitions.

## Features

- **Standalone MCP Server**: Works independently of any existing project
- **Diesel to Proto Conversion**: Converts Diesel table! macro definitions to Protocol Buffer definitions
- **CRUD Operations**: Generates complete CRUD request/response messages
- **Type Mapping**: Intelligent mapping from Diesel types to protobuf types
- **Claude Integration**: Direct integration with Claude Desktop

## Installation

**Install from Crates.io (Recommended):**
```bash
cargo install mcp-diesel-proto-generator
```

The binary will be installed to `~/.cargo/bin/mcp-diesel-proto-generator`.

**Build from Source:**
```bash
git clone <repository-url>
cd mcp-diesel-proto-generator
cargo build --release
```

## Claude Desktop Configuration

Add the following to your Claude Desktop configuration file:

**macOS:** `~/Library/Application Support/Claude/claude_desktop_config.json`
**Windows:** `%APPDATA%/Claude/claude_desktop_config.json`

```json
{
  "mcpServers": {
    "proto-generator": {
      "command": "/Users/yourusername/.cargo/bin/mcp-diesel-proto-generator",
      "args": []
    }
  }
}
```

**Important:** Replace `/Users/yourusername` with your actual home directory path, or use the full path where the binary is installed.

## Usage with Claude

Once configured, you can use the following commands in Claude:

### Generate Proto from Schema Content

```
Generate a proto file from this Diesel schema:

table! {
    users (id) {
        id -> Integer,
        email -> Text,
        name -> Nullable<Text>,
        created_at -> Timestamp,
        is_active -> Bool,
    }
}

table! {
    posts (id) {
        id -> Integer,
        user_id -> Integer,
        title -> Text,
        content -> Nullable<Text>,
        published_at -> Nullable<Timestamp>,
    }
}
```

### Custom Package Name

```
Generate a proto file with package name "blog" from this schema:

[your schema here]
```

## Supported Diesel Types

The MCP server maps Diesel types to protobuf types as follows:

| Diesel Type | Protobuf Type |
|-------------|---------------|
| Integer | int32 |
| BigInt | int64 |
| SmallInt | int32 |
| Float | float |
| Double, Numeric | double |
| Bool | bool |
| Text, VarChar, Char | string |
| Uuid | string |
| Timestamp, Timestamptz | Timestamp |
| Date, Time, Timetz | string |
| Json, Jsonb | string |
| Binary, Bytea | bytes |
| Nullable<Type> | optional Type |
| Array<Type> | repeated Type |

## Generated Proto Structure

The generated proto files include:

1. **Entity Messages**: One message per table with all fields
2. **CRUD Request/Response Messages**: Complete CRUD operations for each entity
3. **Common Structures**: Shared parameter and query structures
4. **Service Definition**: gRPC service with all CRUD operations
5. **Timestamp Message**: Custom timestamp representation

## Example Output

For a `users` table, the generator creates:

- `Users` message (entity)
- `CreateUsersRequest/Response`
- `GetUsersRequest/Response` 
- `UpdateUsersRequest/Response`
- `DeleteUsersRequest/Response`
- Service methods for all operations

## Troubleshooting

### MCP Server Not Found
- Ensure the binary path in `claude_desktop_config.json` is absolute and correct
- If installed via `cargo install`, the binary should be at `~/.cargo/bin/mcp-diesel-proto-generator`
- Restart Claude Desktop after configuration changes

### Schema Parsing Issues
- Ensure your Diesel schema is valid
- The parser expects standard `table!` macro definitions
- Make sure column definitions follow the format: `column_name -> Type,`

### Permission Issues
- Ensure the binary has execute permissions: `chmod +x target/release/mcp-proto-generator`

## Example Usage

You can test the MCP server with the provided example schema:

```bash
cat example_schema.rs | ./target/release/mcp-proto-generator
```

## Development

To modify or extend the MCP server:

1. Clone the repository and edit `src/main.rs`
2. Rebuild with `cargo build --release`
3. Update your Claude Desktop configuration to point to the new binary
4. Restart Claude Desktop to pick up changes

## License

This project is provided as-is for educational and development purposes.