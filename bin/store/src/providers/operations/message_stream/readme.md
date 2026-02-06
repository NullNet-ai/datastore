
# Message Stream System

The Message Stream System is a high-performance, real-time message delivery system built on top of PostgreSQL NOTIFY/LISTEN and Socket.IO. It provides reliable message routing with backpressure handling, rate limiting, and fairness guarantees.

## Architecture Overview

The system consists of several interconnected components:

1. **PostgreSQL Notification System**: Receives NOTIFY events from the database
2. **PgListenerService**: Listens to PostgreSQL notifications and routes them to the main stream
3. **MessageStreamingService**: Central orchestrator that manages message routing and processing
4. **TokenBucket System**: Provides rate limiting and backpressure management per channel
5. **SharedStreamingState**: Manages global state for channels, organizations, and processing queues
6. **StreamQueueService**: Handles database persistence for queued messages
7. **Gateway**: Manages Socket.IO connections and client authentication

## Architecture Components

### Core Services
1. **PgListenerService** - PostgreSQL notification listener
2. **MessageStreamingService** - Central message router and processor
3. **TokenBucket** - Rate limiting and message buffering
4. **StreamQueueService** - Message persistence during backpressure
5. **FlushConnectionLimiter** - Database connection optimization
6. **Gateway** - Socket.IO client management and authentication
7. **SharedState** - Global state management and coordination

## Complete Message Lifecycle

### Phase 1: PostgreSQL Notification Reception

**File:** `pg_listener_service.rs`

#### 1.1 Service Initialization
```
PgListenerService::initialize() → 
  ├── Creates singleton instance with TokenBucket::new_without_consumer()
  ├── Establishes PostgreSQL connection with tokio_postgres
  ├── Spawns connection management task
  ├── Spawns periodic channel refresh task (30s intervals)
  └── Spawns drain listener task for backpressure recovery
```

#### 1.2 Channel Discovery and Subscription
```
refresh_channels() →
  ├── Queries postgres_channels table for active channels
  ├── Executes LISTEN commands for new channels
  ├── Updates subscribed_channels HashSet
  └── Logs subscription status
```

#### 1.3 Notification Processing
```
process_notification(notification) →
  ├── Parses JSON payload into Message struct
  ├── Pushes message to main_stream.buffer (VecDeque)
  └── Calls notify_message_available.notify_one()
```

#### 1.4 Connection Management
- **Automatic Reconnection:** 5-second retry on connection failures
- **Channel Persistence:** Maintains subscription state across reconnects
- **Error Handling:** Graceful degradation with comprehensive logging

### Phase 2: Message Routing and Organization Filtering

**File:** `streaming_service.rs`

#### 2.1 Routing Task Initialization
```
start_routing_task(main_stream) →
  ├── Spawns dedicated tokio task
  ├── Enters infinite loop waiting for message notifications
  ├── Uses notify_message_available.notified().await
  └── Processes all available messages in buffer
```

#### 2.2 Message Processing Loop
```
while let Some(message) = main_stream.buffer.pop_front() →
  ├── Parses JSON to extract event_name and organization_id
  ├── Validates required fields (logs errors for missing data)
  ├── Checks if organization has authenticated clients
  ├── If clients exist: calls handle_message()
  └── If no clients: discards message (logs info)
```

#### 2.3 Organization Client Verification
```
get_organization_clients(org_id) →
  ├── Queries global ORGANIZATION_CLIENTS registry
  ├── Returns HashMap<SocketId, SocketRef> if clients exist
  └── Returns None if no authenticated clients
```

### Phase 3: Channel Management and Rate Limiting

**File:** `streaming_service.rs` → `handle_message()`

#### 3.1 Channel State Verification
```
handle_message(channel_name, org_id, message) →
  ├── Checks shared_state.is_flushing(channel_name)
  ├── Checks shared_state.is_backpressured(channel_name)
  ├── If flushing: queues message via queue_service
  └── If backpressured: queues message and returns
```

#### 3.2 Channel Creation and Retrieval
```
get_or_create_channel() →
  ├── Attempts shared_state.get_channel(channel_name)
  ├── If exists: returns existing TokenBucket
  ├── If not exists: creates new TokenBucket with default capacity
  ├── Registers channel with organization mapping
  ├── Starts drain listener for backpressure recovery
  └── Returns Arc<TokenBucket>
```

#### 3.3 Rate Limiting and Message Processing
```
bucket.receive_message(message) →
  ├── Acquires token lock (async mutex)
  ├── If tokens > 0: decrements token, stores message in buffer
  ├── Calls notify_message_available.notify_one()
  ├── Returns true if tokens remain (capacity available)
  └── Returns false if tokens = 0 (backpressured)
```

### Phase 4: Token Bucket System

**File:** `token_bucket.rs`

#### 4.1 Token Bucket Architecture
```
TokenBucket {
  name: String,                    // Channel identifier
  capacity: Mutex<usize>,          // Maximum tokens
  tokens: Mutex<usize>,            // Current available tokens
  buffer: Mutex<VecDeque<Message>>, // Message queue
  notify_drain: Arc<Notify>,       // Backpressure recovery signal
  notify_message_available: Arc<Notify>, // New message signal
  consumer_started: Mutex<bool>    // Consumer task state
}
```

#### 4.2 Sequential Consumer Task
```
start_sequential_consumer() →
  ├── Spawns single tokio task per bucket
  ├── Waits for notify_message_available signals
  ├── Processes messages one-by-one with rate limiting
  ├── Calls emit_message() → transmit_to_channel()
  ├── Adds configurable delay between messages
  └── Triggers drain notifications on capacity recovery
```

#### 4.3 Message Emission and Token Recovery
```
emit_message() →
  ├── Pops message from front of buffer
  ├── Increments token count (capacity recovery)
  ├── Detects backpressure recovery (was 0, now > 0)
  ├── Triggers drain() if buffer empty or capacity full
  └── Returns message for transmission
```

#### 4.4 Drain Event Handling
```
drain() →
  ├── Calls notify_drain.notify_waiters()
  ├── Signals all waiting drain listeners
  ├── Triggers queue processing for backpressured channels
  └── Enables channel state transitions
```

### Phase 5: Message Broadcasting

**File:** `token_bucket.rs` → `transmit_to_channel()`

#### 5.1 Organization ID Extraction
```
transmit_to_channel(message) →
  ├── Parses message JSON to extract organization_id
  ├── Validates organization_id field exists
  ├── Extracts event_name for Socket.IO event type
  └── Calls gateway::broadcast_to_channel()
```

#### 5.2 Socket.IO Broadcasting
**File:** `gateway.rs`

```
broadcast_to_channel(channel, org_id, notification) →
  ├── Constructs organization room: "org_{organization_id}"
  ├── Uses event_name as Socket.IO event name
  ├── Calls io.to(room).emit(event_name, notification)
  └── Delivers to all authenticated clients in organization
```

### Phase 6: Backpressure and Queue Management

**File:** `streaming_service.rs`

#### 6.1 Drain Listener System
```
start_drain_listener(channel_name, bucket) →
  ├── Spawns dedicated task per channel
  ├── Waits for bucket.on_drain() notifications
  ├── Removes channel from backpressured state
  ├── Checks for queued messages in database
  ├── If messages exist: marks as flushing, starts processing
  └── If no messages: ensures channel ready for direct processing
```

#### 6.2 Queue Processing with Connection Optimization
```
process_queued_messages(channel_name) →
  ├── Acquires FlushConnectionLimiter permit
  ├── Gets shared database connection
  ├── Processes messages in batches (500 per batch)
  ├── Calls bucket.receive_message() for each message
  ├── Tracks consumed message IDs for deletion
  ├── Deletes processed messages in single operation
  ├── Handles backpressure during processing
  └── Queues channel for fairness if more messages exist
```

#### 6.3 Fairness Queue System
```
start_processing_queue_handler() →
  ├── Spawns global fairness coordinator task
  ├── Dequeues channels from processing queue
  ├── Verifies channel still in flushing state
  ├── Processes one batch per channel per turn
  └── Ensures fair processing across all channels
```

### Phase 7: Database Queue Persistence

**File:** `stream_queue_service.rs`

#### 7.1 Message Queuing
```
queue_message_with_conn(conn, channel_name, message) →
  ├── Inserts into stream_queue_items table
  ├── Stores: channel_name, content, created_at, organization_id
  ├── Uses provided connection for efficiency
  └── Returns queue item ID
```

#### 7.2 Batch Dequeuing
```
dequeue_batch_from_channel_with_conn(conn, channel, limit) →
  ├── Queries oldest messages for channel (ORDER BY created_at)
  ├── Limits result set to batch size
  ├── Returns Vec<QueueItem> with id, content, metadata
  └── Maintains message ordering
```

#### 7.3 Cleanup Operations
```
delete_processed_items_with_conn(conn, item_ids) →
  ├── Deletes multiple items in single SQL operation
  ├── Uses IN clause for efficiency
  ├── Returns count of deleted items
  └── Prevents queue table growth
```

### Phase 8: Connection Management

**File:** `flush_connection_limiter.rs`

#### 8.1 Connection Limiting
```
FlushConnectionLimiter {
  semaphore: Arc<Semaphore>,  // Controls concurrent connections
  pool: Arc<Pool>,            // Database connection pool
  max_connections: usize      // Configurable limit (default: 10)
}
```

#### 8.2 Connection Acquisition
```
acquire_flush_connection() →
  ├── Acquires semaphore permit (blocks if at limit)
  ├── Gets connection from pool
  ├── Returns (Permit, Connection) tuple
  └── Auto-releases on drop (RAII)
```

### Phase 9: Client Authentication and Management

**File:** `gateway.rs`

#### 9.1 Socket.IO Authentication
```
auth_middleware(socket, auth_data) →
  ├── Extracts JWT token from auth data
  ├── Validates token signature and expiration
  ├── Extracts organization_id from token claims
  ├── Joins client to "org_{organization_id}" room
  ├── Registers client in ORGANIZATION_CLIENTS
  └── Sets up disconnect handler for cleanup
```

#### 9.2 Client Registry Management
```
ORGANIZATION_CLIENTS: OnceCell<RwLock<HashMap<String, HashMap<SocketId, SocketRef>>>>

add_client_to_organization(org_id, socket_id, socket_ref) →
  ├── Acquires write lock on global registry
  ├── Creates organization entry if not exists
  ├── Adds socket to organization's client map
  └── Enables message delivery to organization

remove_client_from_organization(org_id, socket_id) →
  ├── Acquires write lock on global registry
  ├── Removes socket from organization's client map
  ├── Cleans up empty organization entries
  └── Prevents message delivery to disconnected clients
```

### Phase 10: State Management and Coordination

**File:** `shared_state.rs`

#### 10.1 Channel State Tracking
```
SharedState {
  channels: RwLock<HashMap<String, Arc<TokenBucket>>>,
  channel_organizations: RwLock<HashMap<String, String>>,
  flushing_channels: RwLock<HashSet<String>>,
  backpressured_channels: RwLock<HashSet<String>>,
  processing_queue: Mutex<VecDeque<String>>
}
```

#### 10.2 State Transitions
```
Channel States:
  Normal → Backpressured (when tokens = 0)
  Backpressured → Flushing (when drain occurs + queued messages)
  Flushing → Normal (when queue empty)
  Flushing → Backpressured (if backpressure during flush)
```

## Performance Optimizations

### 1. Connection Reuse
- All queue operations use `_with_conn` variants
- Single connection per batch processing operation
- FlushConnectionLimiter prevents pool exhaustion

### 2. Batch Processing
- Messages processed in batches of 500
- Single database delete operation per batch
- Fairness queue ensures no channel starvation

### 3. Memory Management
- Efficient VecDeque for message buffering
- Automatic cleanup of processed queue items
- Optimized state management with RwLock

### 4. Async Coordination
- Tokio Notify for efficient task coordination
- RwLock for concurrent read access to shared state
- Semaphore-based connection limiting

## Configuration Parameters

### Environment Variables
```
JWT_SECRET: Socket.IO authentication secret
POSTGRES_USER: Database username (default: admin)
POSTGRES_PASSWORD: Database password (default: admin)
POSTGRES_DB: Database name (default: nullnet)
POSTGRES_HOST: Database host (default: localhost)
POSTGRES_PORT: Database port (default: 5432)
```

### System Defaults
```
Token Bucket Capacity: 1000 tokens per channel
Batch Size: 500 messages per processing batch
Max Flush Connections: 10 concurrent database connections
Channel Refresh Interval: 30 seconds
Reconnection Delay: 5 seconds
Processing Queue Management: Fair channel rotation
Rate Limiting Delay: Configurable per bucket
```

## Monitoring and Observability

### Socket.IO Events
```
getBucketStatus: Token utilization and capacity
getClientStatus: Connected clients per organization
getSystemMetrics: Message rates and system health
getCurrentHighWaterMark: Channel capacity settings
updateHighWaterMark: Dynamic capacity adjustment
```

### Logging Levels
```
ERROR: Connection failures, parsing errors, critical issues
WARN: Missing organization_id, no clients for organization
INFO: Service initialization, channel creation, state changes
DEBUG: Message processing, duplicate detection, detailed flow
```

## Error Handling and Recovery

### 1. PostgreSQL Connection Issues
- Automatic reconnection with exponential backoff
- Channel subscription persistence across reconnects
- Graceful degradation during connection loss

### 2. Message Processing Errors
- Invalid JSON payloads logged and discarded
- Missing required fields handled gracefully
- Failed message routing logged with context

### 3. Database Queue Issues
- Connection pool exhaustion prevented by limiter
- Failed queue operations logged with retry capability
- Transactional consistency for batch operations

### 4. Socket.IO Client Issues
- Automatic client cleanup on disconnect
- Invalid authentication handled with rejection
- Organization isolation maintained

## Security Considerations

### 1. Authentication
- JWT-based client authentication
- Organization-level access control
- Token validation on every connection

### 2. Message Isolation
- Messages only delivered to authorized organizations
- No cross-organization message leakage
- Client registry isolation

### 3. Resource Protection
- Rate limiting prevents client overload
- Connection limiting prevents database exhaustion
- Memory limits prevent cache overflow

This architecture provides a robust, scalable, and secure message delivery system capable of handling high-throughput scenarios while maintaining message ordering, delivery guarantees, and system stability.