# Root Controller Usage

The Root Controller is a wrapper around the existing Store Controller that adds a type parameter to the routes, enabling DRY (Don't Repeat Yourself) code patterns.

## Route Structure

### Original Store Controller Routes
```
/api/store/{table}                    - POST (create record)
/api/store/{table}/{id}               - GET, PATCH, DELETE
/api/store/{table}/filter             - POST (filter records)
/api/store/upsert/{table}             - POST (upsert record)
/api/store/batch/{table}              - POST, PATCH, DELETE (batch operations)
/api/store/aggregate                  - POST (aggregation filter)
```

### Root Controller Routes (with type parameter)
```
/api/store/{type}/{table}             - POST (create record)
/api/store/{type}/{table}/{id}        - GET, PATCH, DELETE
/api/store/{type}/{table}/filter      - POST (filter records)
/api/store/{type}/upsert/{table}      - POST (upsert record)
/api/store/{type}/batch/{table}       - POST, PATCH, DELETE (batch operations)
/api/store/{type}/aggregate           - POST (aggregation filter)
```

## Type Parameter Values

The `{type}` parameter can be:
- `none` - Indicates the route is accessed through the root controller but behaves like the original controller
- `root` - Indicates the route is accessed through the root controller with root-specific behavior
- Any other string value for custom type handling

## Usage Examples

### Using the Original Controller
```bash
# Create a user record using original controller
POST /api/store/users
{
  "record": {
    "name": "John Doe",
    "email": "john@example.com"
  }
}
```

### Using the Root Controller with 'none' type
```bash
# Create a user record using root controller with 'none' type
POST /api/store/none/users
{
  "record": {
    "name": "John Doe",
    "email": "john@example.com"
  }
}
```

### Using the Root Controller with 'root' type
```bash
# Create a user record using root controller with 'root' type
POST /api/store/root/users
{
  "record": {
    "name": "Admin User",
    "email": "admin@example.com"
  }
}
```

## Helper Functions

The root controller provides helper functions to determine the controller type:

```rust
use crate::controllers::root_controller::{
    get_controller_type, is_root_controller, is_none_controller
};

// Get the controller type from request
let controller_type = get_controller_type(&request);

// Check if request is from root controller
if is_root_controller(&request) {
    // Handle root-specific logic
}

// Check if request is from none/default controller
if is_none_controller(&request) {
    // Handle default logic
}
```

## Implementation Details

- The root controller acts as a wrapper around the existing store controller
- All business logic remains in the original store controller functions
- The type parameter is stored in the request extensions for access in downstream functions
- Both controllers use the same middleware stack (Authentication, SessionMiddleware, ShutdownGuard)
- The implementation follows DRY principles by reusing existing controller functions

## Benefits

1. **DRY Code**: No duplication of business logic
2. **Flexible Routing**: Support for different access patterns
3. **Backward Compatibility**: Original routes continue to work
4. **Type-aware Processing**: Ability to handle different types of requests differently
5. **Easy Extension**: Simple to add new type-specific behaviors