# Session Middleware Guide

This guide explains how to use the session middleware for both HTTP and gRPC services in the store application.

## Overview

The session middleware provides a unified session management system that works across both HTTP and gRPC protocols. It includes:

- **Shared Session Core**: Common session management logic
- **HTTP Session Middleware**: Actix-web middleware for HTTP requests
- **gRPC Session Interceptor**: Tonic interceptor for gRPC requests
- **Database Integration**: Persistent session storage using PostgreSQL

## Architecture

### Core Components

1. **SessionManager** (`session_core.rs`): Central session management with configurable settings
2. **SessionMiddleware** (`session_middleware.rs`): HTTP middleware using Actix-web
3. **GrpcSessionInterceptor** (`grpc_session_interceptor.rs`): gRPC interceptor using Tonic
4. **InterceptorChain** (`interceptor_chain.rs`): Chains multiple gRPC interceptors

### Session Flow

#### HTTP Requests
1. Extract session ID from header (`X-Session-ID`) or cookie (`session_id`)
2. Load existing session or create new one
3. Store session in request extensions
4. Process request
5. Save updated session to database
6. Set session cookie in response

#### gRPC Requests
1. Extract session ID from metadata header (`x-session-id`)
2. Load existing session or create new one
3. Store session in request extensions
4. Process request
5. Session is automatically saved after processing

## Configuration

### SessionConfig

```rust
pub struct SessionConfig {
    pub session_header: String,        // Header name for session ID
    pub cookie_name: String,           // Cookie name for HTTP sessions
    pub cookie_max_age: String,        // Cookie expiration time
    pub secret: String,                // Secret for session security
}
```

### Default Configuration

- Session Header: `"X-Session-ID"`
- Cookie Name: `"session_id"`
- Cookie Max Age: `"7d"` (7 days)
- Secret: `"default_secret"` (should be changed in production)

## Usage

### HTTP Middleware Setup

The HTTP middleware is automatically configured in the Actix-web application:

```rust
use crate::middlewares::session_middleware::SessionMiddleware;

App::new()
    .wrap(SessionMiddleware::default())
    // ... other middleware and routes
```

### gRPC Interceptor Setup

The gRPC interceptor is automatically configured in the gRPC server:

```rust
use crate::middlewares::grpc_session_interceptor::GrpcSessionInterceptor;
use crate::middlewares::interceptor_chain::InterceptorChain;

// Create interceptor chain: shutdown -> session -> auth
let shutdown_interceptor = GrpcShutdownInterceptor::new(shutdown_receiver);
let session_interceptor = GrpcSessionInterceptor::new();
let auth_interceptor = GrpcAuthInterceptor::new();

let interceptor_chain = InterceptorChain::new(
    shutdown_interceptor,
    InterceptorChain::new(session_interceptor, auth_interceptor)
);

Server::builder()
    .layer(tower::ServiceBuilder::new().layer_fn(move |service| {
        InterceptorService::new(service, interceptor_chain.clone())
    }))
    // ... add services
```

### Accessing Sessions in Handlers

#### HTTP Handlers

```rust
use actix_web::{web, HttpRequest, HttpResponse};
use crate::auth::structs::Session;

pub async fn my_handler(req: HttpRequest) -> HttpResponse {
    // Get session from request extensions
    if let Some(session) = req.extensions().get::<Session>() {
        println!("Session ID: {}", session.session_id);
        println!("User ID: {:?}", session.user_id);
        
        // Access session data
        if let Some(value) = session.data.get("key") {
            println!("Session data: {}", value);
        }
    }
    
    HttpResponse::Ok().json("Success")
}
```

#### gRPC Handlers

```rust
use tonic::{Request, Response, Status};
use crate::auth::structs::Session;

pub async fn my_grpc_method(
    &self,
    request: Request<MyRequest>,
) -> Result<Response<MyResponse>, Status> {
    // Get session from request extensions
    if let Some(session) = request.extensions().get::<Session>() {
        println!("Session ID: {}", session.session_id);
        println!("User ID: {:?}", session.user_id);
        
        // Access session data
        if let Some(value) = session.data.get("key") {
            println!("Session data: {}", value);
        }
    }
    
    Ok(Response::new(MyResponse {}))
}
```

### Modifying Session Data

#### HTTP

```rust
use actix_web::{web, HttpRequest, HttpResponse};
use crate::auth::structs::Session;

pub async fn update_session(mut req: HttpRequest) -> HttpResponse {
    if let Some(mut session) = req.extensions_mut().remove::<Session>() {
        // Modify session data
        session.data.insert("last_action".to_string(), "updated".to_string());
        session.user_id = Some("user123".to_string());
        
        // Put the modified session back
        req.extensions_mut().insert(session);
    }
    
    HttpResponse::Ok().json("Session updated")
}
```

#### gRPC

```rust
use tonic::{Request, Response, Status};
use crate::auth::structs::Session;

pub async fn update_session_grpc(
    &self,
    mut request: Request<MyRequest>,
) -> Result<Response<MyResponse>, Status> {
    if let Some(mut session) = request.extensions_mut().remove::<Session>() {
        // Modify session data
        session.data.insert("last_action".to_string(), "updated".to_string());
        session.user_id = Some("user123".to_string());
        
        // Put the modified session back
        request.extensions_mut().insert(session);
    }
    
    Ok(Response::new(MyResponse {}))
}
```

## Session Management

### Manual Session Operations

```rust
use crate::middlewares::session_core::SessionManager;

// Create session manager
let session_manager = SessionManager::with_default_config();

// Create new session
let session_id = session_manager.extract_session_id(None, None);
let session = session_manager.get_or_create_session(&session_id).await;

// Load existing session
if let Ok(Some(existing_session)) = session_manager.load_session("session_123").await {
    println!("Loaded session: {}", existing_session.session_id);
}

// Save session
if let Err(e) = session_manager.save_session(&session).await {
    eprintln!("Failed to save session: {:?}", e);
}
```

### Session Cleanup

```rust
use crate::middlewares::session_core::prune_expired_sessions;

// Remove expired sessions (should be called periodically)
if let Err(e) = prune_expired_sessions().await {
    eprintln!("Failed to prune expired sessions: {:?}", e);
}
```

## Client Usage

### HTTP Clients

Include the session ID in requests:

```bash
# Using header
curl -H "X-Session-ID: your_session_id" http://localhost:8080/api/endpoint

# Using cookie
curl -b "session_id=your_session_id" http://localhost:8080/api/endpoint
```

### gRPC Clients

Include the session ID in metadata:

```rust
use tonic::metadata::MetadataValue;
use tonic::Request;

let mut request = Request::new(MyRequest {});
request.metadata_mut().insert(
    "x-session-id",
    MetadataValue::from_str("your_session_id").unwrap()
);

let response = client.my_method(request).await?;
```

## Security Considerations

1. **Session ID Generation**: Uses ULID for cryptographically secure, sortable session IDs
2. **Cookie Security**: HTTP-only, secure, and SameSite cookies for web clients
3. **Session Expiration**: Configurable session timeout with automatic cleanup
4. **Database Storage**: Sessions are stored securely in PostgreSQL

## Troubleshooting

### Common Issues

1. **Session Not Found**: Ensure the session ID is correctly passed in headers/cookies
2. **Database Errors**: Check PostgreSQL connection and session table schema
3. **Compilation Errors**: Ensure all dependencies are properly imported

### Debug Tips

1. Enable debug logging to trace session operations
2. Check database for session records
3. Verify interceptor chain order (shutdown -> session -> auth)

## Examples

See `src/examples/grpc_session_usage.rs` for complete usage examples.