# Sync (CRDT sync with server)

This module syncs CRDT messages between the **store** (client) and the **server** (`bin/server`). The store sends local changes and its Merkle tree; the server applies messages, computes a diff, and returns messages the client is missing. A new store can **bootstrap** by pulling all messages from the server first so it does not overwrite server state.

---

## Table of contents

1. [Overview](#overview)
2. [Why a new client must fetch from the server first (bootstrap)](#why-bootstrap)
3. [How a new client catches up to the server](#how-new-client-catches-up)
4. [How CRDT messages are stored (background queue → DB)](#how-crdt-messages-stored)
5. [How the sync queue works (outgoing messages to server)](#how-sync-queue-works)
6. [How Merkle is saved](#how-merkle-saved)
7. [How chunking works](#how-chunking-works)
8. [Sync endpoints: what to insert](#sync-endpoints-insert)
9. [Env vars](#env-vars)
10. [Main modules](#main-modules)
11. [Server side (bin/server)](#server-side)
12. [store-clean-setup](#store-clean-setup)
13. [Related](#related)

---

<a id="overview"></a>
## Overview

**Code:** [sync_service.rs](sync_service.rs) (bg_sync, sync), [transport_driver.rs](transport/transport_driver.rs)

- **Store** keeps a Merkle tree and HLC clock per group and syncs with one or more **sync endpoints** (from the `sync_endpoints` table).
- **Normal sync**: Store POSTs to `{endpoint}/app/sync` with `group_id`, `client_id`, `messages` (outgoing), and `merkle`. Server applies messages, diffs client merkle vs server merkle, and returns `new_messages` (and `incomplete` + chunk API when there are many).
- **Bootstrap sync**: When `SYNC_BOOTSTRAP_URL` is set, the store runs one bootstrap at the first `bg_sync`: it sends **empty messages** and **empty merkle** so the server returns **all** messages. The store applies them and updates its merkle from the server.

---

<a id="why-bootstrap"></a>
## Why a new client must fetch from the server first (bootstrap)

**Code:** [sync_service.rs](sync_service.rs) (`bootstrap_sync_once`, `get_bootstrap_opts_from_env`), server: `bin/server/src/controllers/main_controllers.rs` (bootstrap path when client merkle empty)

If a **new** or **reset** store (empty or small Merkle tree) syncs by **sending** its state first:

1. The client sends its merkle (e.g. empty or a few leaves) and its outgoing messages.
2. The server **applies** the client’s messages and updates its own merkle.
3. The server then **diffs** client merkle vs server merkle. If the client’s merkle is empty or very small, the server may conclude “client has nothing” and return a huge set of messages — but in some flows the server had already **replaced** or **merged** its idea of the client’s state with the small one the client sent. So the server’s merkle for that group can effectively become the **client’s** merkle (e.g. 16 leaves after pruning), and the server’s existing 14k messages are no longer reflected in that tree. The server then returns **no** messages (diff = 0), and the new client never receives the existing data.

To avoid that, a **new client must fetch from the server first** (bootstrap):

- The client sends **empty merkle** and **no messages**.
- The server treats this as **bootstrap** and returns **all** messages for the group (from timestamp `""`).
- The client applies those messages and adopts the server’s merkle from the response.
- After that, normal two-way sync can continue without overwriting the server’s existing state.

So: **bootstrap = “pull everything from server first”** so the new client gets the true server state (e.g. 14k messages) instead of the server adopting the client’s empty tree and returning nothing.

---

<a id="how-new-client-catches-up"></a>
## How a new client catches up to the server

**Code:** [sync_service.rs](sync_service.rs) (`bootstrap_sync_once`, `sync`, `receive_messages`), [transport_driver.rs](transport/transport_driver.rs) (chunk fetch)

Two mechanisms:

1. **Bootstrap (one-time, when configured)**  
   - On first `bg_sync`, if `SYNC_BOOTSTRAP_URL` is set, the store calls the server with **empty merkle** and **empty messages**.  
   - Server returns **all** messages for the group (and uses chunking if needed).  
   - Client applies them via `receive_messages` and updates its merkle from the response (`commit_tree`).  
   - Runs **once per process** (`BOOTSTRAP_DONE`).

2. **Normal catch-up (every sync)**  
   - On each sync, the client sends its current **merkle**.  
   - Server diffs it against the server merkle and returns only messages the client is missing.  
   - Client applies those and updates its merkle from the response.  
   - If the response is large, the server sets `incomplete=1` and the client fetches the rest via the **chunk API** (see [How chunking works](#how-chunking-works)).

So a new client either does a one-time full pull (bootstrap) or, without bootstrap, catches up over time via the normal diff + chunk flow.

---

<a id="how-crdt-messages-stored"></a>
## How CRDT messages are stored (background queue → DB)

**Code:** [message_manager.rs](message_manager.rs) (channel, batching, `write_batch_to_db`), [message_service.rs](message_service.rs) (`insert_messages_batch`), [sync_service.rs](sync_service.rs) (`apply_messages`, `send_messages`)

Outgoing CRDT messages (from local inserts/updates/deletes) are not written to the DB synchronously in the request path. They go through a **background pipeline**:

1. **Creation**  
   When a record is created/updated/deleted, the store builds one or more `CrdtMessageModel` values and calls `send_messages` (which applies them and then feeds them to the MessageManager).

2. **MessageManager (in-memory channel + batched write)**  
   - A single **unbounded channel** receives `CrdtMessageModel` messages.  
   - A dedicated task reads from the channel and batches them (up to `BATCH_SIZE` 1000, or `MIN_FLUSH_SIZE` 300, or after `BATCH_FLUSH_TIMEOUT` 2 ms with no new message).  
   - Each batch is sent to a **writer task** over a bounded channel (cap 2).  
   - The writer task calls `insert_messages_batch` to insert the batch into the **`crdt_messages`** table.  
   - If the insert fails, the batch is pushed to `FAILED_BATCH_QUEUE` and retried later (e.g. on shutdown or periodically).

3. **HLC and Merkle**  
   As part of apply, each message gets a timestamp via `HlcService::insert_timestamp`, which updates the Merkle tree (add leaf, prune). So **Merkle is updated in memory** when messages are applied; persistence of Merkle is separate (see [How Merkle is saved](#how-merkle-is-saved)).

4. **Sync queue (outgoing to server)**  
   In addition, a **pack** (the serialized messages plus `since` / `transaction_id`) is **enqueued** in the **sync queue** (`QueueService`, `queues` / `queue_items` tables) so that `bg_sync` can later dequeue and POST them to each sync endpoint. So: **storage in `crdt_messages`** is via MessageManager; **sending to the server** is via the sync queue and `process_queue` / `sync()`.

Summary: **CRDT messages** are stored in **`crdt_messages`** via the **MessageManager** background batching; **outgoing sync** is driven by the **sync queue** (queue_items) and `bg_sync` / `process_queue`.

---

<a id="how-sync-queue-works"></a>
## How the sync queue works (outgoing messages to server)

**Code:** [queue_service.rs](transactions/queue_service.rs) (`enqueue`, `dequeue_batch`, `ack_batch`), [sync_service.rs](sync_service.rs) (`process_queue`, `bg_sync`)

- **Tables**: `queues` (e.g. one row per logical queue, with `name`, `size`, `count`) and `queue_items` (each item has `queue_id`, `order`, `value` JSON).
- **Enqueue**: When messages are applied (after local create/update/delete), a **pack** is enqueued: `{ "messages": [...], "since": ..., "transaction_id": ... }`. `QueueService::enqueue` increments the queue’s `size`, inserts a new `queue_items` row with the next `order`, and stores the pack in `value`.
- **bg_sync**  
  - If **queue size is 0**: it runs `sync(Vec::new(), None, ...)` per endpoint (sync with no outgoing messages, just to pull server diff).  
  - If **queue size > 0**: it runs `process_queue`, which:  
    - Dequeues a **batch** of packs (`dequeue_batch`, up to `SYNC_QUEUE_BATCH_SIZE` default 20).  
    - Merges all `messages` from the packs and uses the first pack’s `since` / `transaction_id`.  
    - Calls `sync(messages, since, transaction_id, endpoint, ...)` for **each** endpoint (round-robin over endpoints).  
    - On **success** for all endpoints, **acks** the batch (`ack_batch` advances `count` so those items are considered consumed).  
    - On **failure**, does not ack; the same items will be dequeued again on the next run.
- **Scheduling**: After each `bg_sync` run, the next run is scheduled: if the queue still has work, use `SYNC_BUSY_INTERVAL_MS` (e.g. 2s); otherwise `SYNC_TIMER_MS` (e.g. 30s).

So the **sync queue** is the **outgoing work queue**: it holds packs of messages that must be sent to every sync endpoint; `process_queue` drains it and acks only when sync succeeds for all endpoints.

---

<a id="how-merkle-saved"></a>
## How Merkle is saved

**Code:** [merkle_manager.rs](merkles/merkle_manager.rs) (`save_to_db`, `start_periodic_save`, `load_trees_from_db`), [merkle_service.rs](merkles/merkle_service.rs), [hlc_service.rs](hlc/hlc_service.rs) (`set_clock`, `insert_timestamp`), store init: `store/src/initializers/system_initialization/background_services_init.rs` (periodic save task)

- **In memory**: The **MerkleManager** holds a map `group_id -> (MerkleTree, timestamp)` inside an `Arc<RwLock<...>>`. Every time the store applies a message or receives a sync response, it updates this in-memory tree (via `HlcService::insert_timestamp`, `commit_tree`, etc.).
- **Load on startup**: When background services start, **MerkleManager** loads all trees from the DB (`load_trees_from_db` → `MerkleService::get_merkles_by_group_id` for each group) into the map.
- **Periodic save**: A background task started in **background_services_init** runs every **`MERKLE_SAVE_INTERVAL`** ms (default 30 seconds). Each tick it calls `MerkleManager::save_to_db()`, which writes every `(group_id, merkle, timestamp)` from the map to the **`crdt_merkles`** table (via `MerkleService::set_merkles_by_group_id`).
- So Merkle is **always up to date in memory** during sync; **persistence** is eventually consistent every 30s (or whatever you set).

---

<a id="how-chunking-works"></a>
## How chunking works

**Code:** [transport_driver.rs](transport/transport_driver.rs) (incomplete check, GET/DELETE chunk loop), server: `bin/server/src/controllers/main_controllers.rs` (store in crdt_client_messages, return incomplete=1)

When the server has **many** messages to send (e.g. catch-up or bootstrap), it does **not** put them all in the sync response body. Instead it:

1. **Server**  
   - Computes `new_messages` (e.g. from merkle diff or bootstrap).  
   - If `new_messages.len() >= outgoing_limit` (e.g. 1):  
     - Stores those messages in **`crdt_client_messages`** keyed by `client_id`.  
     - Returns **`incomplete: 1`** and **`messages: []`** (or a small set). The client is expected to fetch the rest via the chunk API.  
   - **Chunk API**: `GET /app/sync/chunk?client_id=...&start=...&limit=...` returns a slice of messages from the server’s buffer for that client. `DELETE /app/sync/chunk?client_id=...` clears the buffer when the client has finished fetching.

2. **Client (transport)**  
   - After the initial POST to `/app/sync`, the client checks the response for **`incomplete`** (number or bool).  
   - If **incomplete**:  
     - It collects the initial `messages` (if any), then in a **loop**:  
       - `GET {url}/app/sync/chunk?client_id=...&start={start}&limit={CHUNK_LIMIT}` (default 100).  
       - Appends the returned messages (from `data.messages`) to the list, advancing `start` by the chunk length.  
       - Stops when a chunk is empty.  
     - Then calls **`DELETE .../app/sync/chunk?client_id=...`** to clear the server buffer.  
     - Replaces `result["messages"]` with the full concatenated list so the rest of the sync pipeline (e.g. `receive_messages`) sees one big list.  
   - Chunk fetches are **retried** up to 10 times per chunk on failure.

So **chunking** = server stores the large reply in `crdt_client_messages`, returns `incomplete=1`, and the client pulls pages via `GET /app/sync/chunk` and then deletes the buffer.

---

<a id="sync-endpoints-insert"></a>
## Sync endpoints: what to insert

**Code:** [sync_endpoints_service.rs](sync_endpoints_service.rs) (`get_sync_endpoints`), [sync_endpoint_model.rs](../../../generated/models/sync_endpoint_model.rs), schema: `sync_endpoints` in [schema.rs](../../../generated/schema.rs)

Sync runs only when there is at least one **Active** row in **`sync_endpoints`**. The store uses **`url`**, **`username`**, and **`password`** for HTTP Basic Auth when calling `{url}/app/sync` and `{url}/app/sync/chunk`.

**Table: `sync_endpoints`**

| Column     | Type   | Description |
|-----------|--------|-------------|
| `id`      | Text   | Primary key (e.g. unique string). |
| `name`    | Text   | Human-readable name. |
| `url`     | Text   | Base URL of the server (e.g. `http://localhost:3002`). No trailing slash. |
| `group_id`| Text   | CRDT group id this endpoint syncs for (often same as store’s `GROUP_ID`). |
| `username`| Text   | Basic-auth username. |
| `password`| Text   | Basic-auth password. |
| `status`  | Text   | Only rows with `status = 'Active'` are used. |

**Example (SQL):**

```sql
INSERT INTO sync_endpoints (id, name, url, group_id, username, password, status)
VALUES (
  'id-1',
  'my-server',
  'http://localhost:3002',
  '01JBHKXHYSKPP247HZZWHA3JBT',
  'admin@example.com',
  'your-password',
  'Active'
)
ON CONFLICT (id) DO UPDATE SET
  name = EXCLUDED.name,
  url = EXCLUDED.url,
  group_id = EXCLUDED.group_id,
  username = EXCLUDED.username,
  password = EXCLUDED.password,
  status = EXCLUDED.status;
```

**Example (store API):**  
`POST /api/sync_endpoints` with body like:

```json
{
  "endpoint": {
    "id": "id-1",
    "name": "my-server",
    "url": "http://localhost:3002",
    "group_id": "01JBHKXHYSKPP247HZZWHA3JBT",
    "username": "admin@example.com",
    "password": "your-password",
    "status": "Active"
  }
}
```

**Bootstrap** is separate: it uses **env** vars `SYNC_BOOTSTRAP_URL`, `SYNC_BOOTSTRAP_USERNAME`, `SYNC_BOOTSTRAP_PASSWORD` (see [Env vars](#env-vars)). You can point these at the same server as one of your `sync_endpoints` so a new store first pulls from that server, then continues normal sync using the endpoints from the table.

---

<a id="env-vars"></a>
## Env vars

**Code:** [config/core.rs](../../../config/core.rs) (sync_timer_ms, merkle_save_interval, etc.), [.env](../../../../.env) (example)

| Variable | Default | Description |
|----------|---------|-------------|
| `SYNC_ENABLED` | `false` | Set `true` to enable sync. |
| `SYNC_TIMER_MS` | `30000` | Idle interval between sync runs (ms). |
| `SYNC_BUSY_INTERVAL_MS` | `2000` | Interval when queue has work. |
| `GROUP_ID` | (see code) | CRDT group id for sync and bootstrap. |
| `MERKLE_SAVE_INTERVAL` | `30000` | How often to persist merkle to DB (ms). |
| `CHUNK_LIMIT` | `100` | Chunk size for `GET /app/sync/chunk`. |
| `SYNC_BOOTSTRAP_URL` | — | If set, bootstrap runs once at first bg_sync (pull all from server). |
| `SYNC_BOOTSTRAP_USERNAME` | — | Basic-auth username for bootstrap. |
| `SYNC_BOOTSTRAP_PASSWORD` | — | Basic-auth password for bootstrap. |

---

<a id="main-modules"></a>
## Main modules

**Code:** [mod.rs](mod.rs), [sync_service.rs](sync_service.rs), [message_manager.rs](message_manager.rs), [queue_service.rs](transactions/queue_service.rs), [hlc_service.rs](hlc/hlc_service.rs), [merkle_manager.rs](merkles/merkle_manager.rs), [transport_driver.rs](transport/transport_driver.rs), [sync_endpoints_service.rs](sync_endpoints_service.rs)

| Module | Role |
|--------|------|
| `sync_service` | `bg_sync`, `sync()`, `bootstrap_sync_once`, queue/schedule, receive and apply. |
| `transport::transport_driver` | HTTP POST to `/app/sync`, chunk fetch and delete, `PostOpts`. |
| `message_manager` | In-memory channel + batched write of CRDT messages to `crdt_messages`. |
| `transactions::queue_service` | Sync queue: enqueue/dequeue/ack packs for outgoing sync. |
| `hlc::hlc_service` | Clock (HLC) and Merkle read/write, `commit_tree`, `recv`, `insert_timestamp`. |
| `merkles::merkle_manager` | In-memory Merkle trees, load from DB, periodic save to DB. |
| `sync_endpoints_service` | Load Active sync endpoints from DB (`url`, `username`, `password`). |

---

<a id="server-side"></a>
## Server side (bin/server)

**Code:** `bin/server/src/controllers/main_controllers.rs` (POST /app/sync, bootstrap, chunk storage), `bin/server/src/sync/crdt/crdt_service.rs` (add_messages, get_merkle, get_all_messages_from_timestamp)

- **POST /app/sync** — Applies client messages, updates server merkle, diffs client merkle vs server merkle, returns `new_messages` (and `incomplete` when response is chunked). If client merkle is empty/`{}`, server treats as **bootstrap** and returns all messages for the group.
- **GET /app/sync/chunk** — Returns a slice of messages from the server’s buffer for `client_id` (`start`, `limit`).
- **DELETE /app/sync/chunk** — Clears the server’s buffer for `client_id` after the client has finished fetching.

---

<a id="store-clean-setup"></a>
## store-clean-setup

**Code:** repo root [Makefile](../../../../../../Makefile) (`store-clean-setup` target), store [Makefile.toml](../../../Makefile.toml) (clean-setup task)

- `make store-clean-setup` runs `cargo make clean-setup` (store with `--cleanup --init-db`).
- The Makefile uses **expect** (or `timeout`) and waits for the line **"Store is running"**. Timeouts were increased to **600 seconds** so long bootstrap sync or DB init can finish before the process is stopped.

---

<a id="related"></a>
## Related

- **Message stream**: Real-time delivery (PostgreSQL NOTIFY + Socket.IO) is documented in `message_stream/readme.md`.
- **Merkle tree**: `libs/merkle` — leaves are timestamps; server and store prune to a fixed number of leaves (`prune_to_level_4`).
