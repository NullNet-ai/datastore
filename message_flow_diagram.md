# Message Stream Flow Diagram

## Complete Message Flow: From Client Authentication to Message Delivery

```mermaid
flowchart TD
    %% Client Authentication Phase
    A[Client Connects to Socket.IO] --> B{JWT Token Provided?}
    B -->|No| C[Emit auth_error]
    B -->|Yes| D[Extract JWT from query/headers]
    D --> E[Verify JWT Token]
    E -->|Invalid| C
    E -->|Valid| F[Extract Claims: organization_id, account_id]
    F --> G[Generate Client ID]
    G --> H[Store ClientData in Socket Extensions]
    H --> I[Join Organization Room: org_{organization_id}]
    I --> J[Register Client in AUTHENTICATED_CLIENTS]
    J --> K[Emit auth_success with client_id]
    K --> L[Client Successfully Authenticated]
    
    %% PostgreSQL Event Handling Phase
    M[PostgreSQL NOTIFY Event] --> N[PgListenerService Receives Notification]
    N --> O[Parse Notification Payload]
    O --> P[Create Message Object]
    P --> Q[Send to Main Token Bucket]
    Q --> R[Main Routing Task Processes Message]
    
    %% Message Routing and Channel Creation
    R --> S[Extract event_name from Message]
    S --> T[Extract organization_id from Message]
    T --> U{Organization Has Authenticated Clients?}
    U -->|No| V[Discard Message - Log Info]
    U -->|Yes| W{Channel Exists?}
    W -->|Yes| X[Get Existing Token Bucket]
    W -->|No| Y[Create New Token Bucket]
    Y --> Z[Register with Broker]
    Z --> AA[Register with Global Registry]
    AA --> AB[Add Channel to Organization]
    AB --> X
    
    %% Token Bucket Processing
    X --> AC{Token Bucket Has Capacity?}
    AC -->|No| AD[Save Message to Database Queue]
    AC -->|Yes| AE[Add Message to Token Bucket Buffer]
    AE --> AF[Decrement Available Tokens]
    AF --> AG[Broadcast to Socket.IO Clients]
    
    %% Message Broadcasting
    AG --> AH[broadcast_to_channel Function]
    AH --> AI[Send to Organization Room: org_{organization_id}]
    AI --> AJ[Use event_name as Socket.IO Event]
    AJ --> AK[All Authenticated Clients in Org Receive Message]
    
    %% Queue Processing (Backpressure Recovery)
    AD --> AL[Message Stored in stream_queue_items Table]
    AL --> AM[Token Bucket Drain Event Triggered]
    AM --> AN[flush_channel_queue Called]
    AN --> AO[Retrieve Queued Messages from DB]
    AO --> AP{Token Bucket Has Capacity?}
    AP -->|No| AQ[Stop Processing - Avoid Infinite Loop]
    AP -->|Yes| AR[Process Queued Message]
    AR --> AS[Add to Token Bucket]
    AS --> AT[Broadcast to Clients]
    AT --> AU[Delete from Queue]
    AU --> AV{More Queued Messages?}
    AV -->|Yes| AP
    AV -->|No| AW[Queue Flush Complete]
    
    %% Message Emission from Token Bucket
    AE --> AX[Message Processing Loop]
    AX --> AY[emit_message from Token Bucket]
    AY --> AZ{Message Available?}
    AZ -->|Yes| BA[Increment Available Tokens]
    BA --> BB[Extract organization_id]
    BB --> BC[Broadcast to Socket.IO]
    BC --> AZ
    AZ -->|No| BD[Wait for Next Iteration]
    BD --> AX
    
    %% Client Disconnect Handling
    AK --> BE[Client May Disconnect]
    BE --> BF[handle_client_disconnect Called]
    BF --> BG[Remove from AUTHENTICATED_CLIENTS]
    BG --> BH[Remove from Organization Tracking]
    BH --> BI{Last Client in Organization?}
    BI -->|Yes| BJ[Future Messages Discarded]
    BI -->|No| BK[Other Clients Continue Receiving]
    
    %% Dashboard Integration
    L --> BL[Dashboard Can Query Client Status]
    X --> BM[Dashboard Can Query Bucket Status]
    AG --> BN[Dashboard Receives System Metrics]
    
    %% Error Scenarios
    C --> BO[Connection Rejected]
    V --> BP[Message Lost - No Clients]
    AQ --> BQ[Messages Remain Queued]
    
    %% Styling
    classDef authPhase fill:#e1f5fe
    classDef pgPhase fill:#f3e5f5
    classDef routingPhase fill:#e8f5e8
    classDef bucketPhase fill:#fff3e0
    classDef broadcastPhase fill:#fce4ec
    classDef queuePhase fill:#f1f8e9
    classDef errorPhase fill:#ffebee
    
    class A,B,C,D,E,F,G,H,I,J,K,L authPhase
    class M,N,O,P,Q,R pgPhase
    class S,T,U,V,W,X,Y,Z,AA,AB routingPhase
    class AC,AD,AE,AF,AX,AY,AZ,BA,BD bucketPhase
    class AG,AH,AI,AJ,AK,BC broadcastPhase
    class AL,AM,AN,AO,AP,AQ,AR,AS,AT,AU,AV,AW queuePhase
    class C,V,BO,BP,BQ errorPhase
```

## Key Components and Their Roles

### 1. **Client Authentication (gateway.rs)**
- **JWT Verification**: Validates tokens using `verify_token` function
- **Client Registration**: Stores authenticated clients in `AUTHENTICATED_CLIENTS`
- **Room Management**: Joins clients to organization-specific Socket.IO rooms
- **Connection Tracking**: Maintains `ClientData` with organization and account info

### 2. **PostgreSQL Event Handling (pg_listener_service.rs)**
- **Channel Subscription**: Listens to PostgreSQL NOTIFY events
- **Message Parsing**: Converts notifications to `Message` objects
- **Main Stream**: Routes all events through a central token bucket
- **Auto-Reconnection**: Handles database connection failures

### 3. **Message Routing (streaming_service.rs)**
- **Dynamic Channel Creation**: Creates token buckets for new channels on-demand
- **Organization Filtering**: Only processes messages for orgs with authenticated clients
- **Channel Association**: Links channels to specific organizations
- **Broker Integration**: Registers channels with the message broker

### 4. **Token Bucket Rate Limiting (token_bucket.rs)**
- **Capacity Management**: Controls message flow with configurable limits
- **Backpressure Handling**: Queues messages when capacity is exceeded
- **Buffer Management**: Maintains FIFO message ordering
- **Drain Notifications**: Triggers queue processing when capacity becomes available

### 5. **Message Broadcasting (gateway.rs)**
- **Organization Rooms**: Uses Socket.IO rooms for efficient message delivery
- **Event Naming**: Uses channel names as Socket.IO event names
- **Selective Delivery**: Only sends to authenticated clients in the target organization

### 6. **Queue Management (stream_queue_service.rs)**
- **Database Persistence**: Stores overflow messages in PostgreSQL
- **Batch Processing**: Retrieves and processes queued messages in batches
- **Cleanup**: Removes successfully processed messages from queue
- **Backpressure Recovery**: Automatically processes queue when capacity returns

## Critical Decision Points

1. **Authentication Check**: Messages are only processed if the target organization has authenticated clients
2. **Capacity Check**: Messages are either processed immediately or queued based on token bucket capacity
3. **Channel Existence**: New channels are created dynamically when first message arrives
4. **Queue Processing**: Queued messages are processed when token bucket capacity becomes available
5. **Error Handling**: Invalid tokens, missing organizations, or capacity issues are handled gracefully

## Performance Optimizations

- **Lazy Channel Creation**: Channels are only created when needed
- **Organization-based Filtering**: Messages for organizations without clients are discarded early
- **Efficient Broadcasting**: Uses Socket.IO rooms to avoid iterating through all clients
- **Batch Queue Processing**: Processes multiple queued messages at once
- **Token Bucket Drain Events**: Minimizes polling by using event-driven queue processing

## Error Scenarios and Handling

1. **Invalid JWT**: Client connection rejected with `auth_error`
2. **No Authenticated Clients**: Messages discarded with info logging
3. **Backpressure**: Messages queued in database for later processing
4. **Database Errors**: Logged and handled gracefully without crashing
5. **Client Disconnection**: Automatic cleanup of tracking data