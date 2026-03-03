# Counter Service

gRPC service that generates unique codes per database and entity (table) using **Redis** and **deadpool-redis**. Used by the store when `CODE_SERVICE_GRPC_URL` is set; otherwise the store uses its local DB counters.

## What it does

- **GetCode(database, table)** — Returns the next unique code for that (database, entity). Uses Redis `INCR` atomically so no two callers get the same number. Config (prefix, default_code, digits_number) must be set via InitCounters first.
- **InitCounters(database, list of CounterConfig)** — Sets or updates config (prefix, default_code, digits_number) per entity. If the counter key does not exist, it is set to 0 so the first GetCode returns the first code. **Re-initialization does not reset existing counters**; only config is updated and missing counters are seeded.
- **HTTP POST /migrate** — Replaces the whole counter record in Redis for a given (database, entity). Send a JSON body with `database`, `entity`, `prefix`, `default_code`, `digits_number`, and `counter`; both the config hash and the counter value are overwritten. Useful for migrations or fixing state.
- **HTTP GET /counters** — Lists all counters with full details: database, entity, prefix, default_code, digits_number, and current counter value.
- **HTTP GET /counters/:database/:entity** — Returns one counter record from Redis: config (prefix, default_code, digits_number) and current counter value. Use to query a specific counter.

Codes are formatted as `prefix + (zero-padded counter or default_code + counter)` depending on `digits_number`.

## Redis

- **Standard Redis** (no modules required). Commands used: `INCR`, `HMGET`/`HSET`, `SET`, `EXISTS`.
- Any Redis 5+ (e.g. 6 or 7) is fine. Run locally (`brew install redis` / `redis-server`), via Docker (`docker run -d -p 6379:6379 redis:7-alpine`), or use a managed Redis and set `REDIS_URL` accordingly.

## gRPC API

- **GetCode** — `GetCodeRequest`: `database`, `table`. Response: `code` (string).
- **InitCounters** — `InitCountersRequest`: `database`, `counters` (repeated `CounterConfig`: entity, prefix, default_code, digits_number). Response: success, message.

Proto: `proto/code_service.proto`. Generated code: `src/generated/code_service.rs` (run `cargo build` to generate).

## Environment

Copy `sample-env.txt` to `.env` and adjust. The binary loads `.env` from the **crate directory** first (`bin/counter-service/.env`), then from the current working directory.

| Variable | Description | Default |
|----------|-------------|---------|
| `REDIS_URL` | Redis connection URL | `redis://127.0.0.1:6379` |
| `CODE_SERVICE_GRPC_LISTEN` | gRPC listen address (host:port) | `0.0.0.0:50051` |
| `CODE_SERVICE_HTTP_LISTEN` | HTTP listen address for /migrate (host:port) | `0.0.0.0:8080` |
| `RUST_LOG` | Log level | `info` |

## Build and run

From workspace root:

```bash
# Build
cargo build -p counter-service

# Run (uses bin/counter-service/.env if present)
cargo run -p counter-service
```

Or from `bin/counter-service`:

```bash
cargo run
```

## Makefile targets (workspace root)

- **`make redis-flush`** — Flush Redis at `REDIS_URL` (from env or `.env`).
- **`make counter-service`** — Start the counter-service binary.
- **`make counter-service-test`** — Run unit tests (no Redis).
- **`make counter-service-test-integration`** — Run integration tests (requires Redis; runs `redis-flush` first).
- **`make counter-service-test-all`** — Flush Redis, then run all tests.

## Re-initialization

- **Config** (prefix, default_code, digits_number) is overwritten on every InitCounters call.
- **Counter value** is only set to 0 when the counter key does **not** exist. If it already exists, it is left unchanged. So re-running the initializer does **not** reset existing counters.

To reset a counter manually (e.g. for a test): delete the Redis key (e.g. `code:counter:<database>:<entity>`) or use `make redis-flush` to clear all counter data. You can also **POST /migrate** with the desired full record (see below).

## Migration endpoint (HTTP)

**POST /migrate** — Replace the whole counter record for one (database, entity). Body (JSON):

```json
{
  "database": "connectivo",
  "entity": "organizations",
  "prefix": "OR",
  "default_code": 1000,
  "digits_number": 6,
  "counter": 42
}
```

All fields are required. This overwrites both the config hash and the counter value in Redis. Example:

```bash
curl -X POST http://localhost:8080/migrate -H "Content-Type: application/json" -d '{"database":"connectivo","entity":"organizations","prefix":"OR","default_code":1000,"digits_number":6,"counter":42}'
```

## Query counters (HTTP) — Postman / curl

Base URL: same as HTTP server (e.g. `http://localhost:8080`). No auth.

| Method | Path | Description |
|--------|------|-------------|
| GET | `/counters` | List all counter keys (database, entity) in Redis |
| GET | `/counters/:database/:entity` | Get one counter: config + current value |

**List all counters**

```bash
curl http://localhost:8080/counters
```

Response (each counter includes config and current count):

```json
{
  "counters": [
    {
      "database": "connectivo",
      "entity": "organizations",
      "prefix": "OR",
      "default_code": 1000,
      "digits_number": 6,
      "counter": 42
    },
    {
      "database": "connectivo",
      "entity": "invoices",
      "prefix": "INV",
      "default_code": 0,
      "digits_number": 6,
      "counter": 105
    }
  ]
}
```

**Get one counter**

```bash
curl http://localhost:8080/counters/connectivo/organizations
```

Response (200):

```json
{
  "database": "connectivo",
  "entity": "organizations",
  "prefix": "OR",
  "default_code": 1000,
  "digits_number": 6,
  "counter": 42
}
```

If the counter is missing, response is 404 with `{ "error": "Counter not found", "database": "...", "entity": "..." }`.

## Store integration

- When **`CODE_SERVICE_GRPC_URL`** is set in the store’s env, the store calls this service for `generate_code` and for code prefix initialization. Database name is derived from **`DATABASE_URL`** (e.g. `connectivo` from `postgres://.../connectivo`).
- When `CODE_SERVICE_GRPC_URL` is not set, the store uses its local DB counters (unchanged behavior).
- The store depends on the `counter-service` crate for the gRPC client and generated types; no proto file is required in the store. Store and counter-service can be deployed separately; the store binary contains the client and only needs the service URL at runtime.
