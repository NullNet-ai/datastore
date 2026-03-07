# Changelog

All notable changes to the CRDT Server project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## 0.1.1

### Author
Kashan

### Added
  - ***`position` BIGSERIAL column on `crdt_client_messages`***:
    - Migration `2026-03-07-000000_add_position_to_crdt_client_messages` — adds a `BIGSERIAL position` column that auto-increments on every insert, guaranteeing strict insertion order regardless of `record_id` (ULID) sort order.
    - Dropped `idx_crdt_client_messages_client_id_record_id`; replaced with `idx_crdt_client_messages_client_id_position ON (client_id, position)`. The new composite index satisfies all three hot query patterns: `WHERE client_id = ? ORDER BY position` (get_chunk), `WHERE client_id = ? COUNT(*)` (get_chunk_status), and `DELETE WHERE client_id = ?` (delete_chunk) without a heap sort step.
    - `schema/core.rs` — added `position -> BigInt` to the `crdt_client_messages` table macro.
    - `models/crdt_client_message.rs` — split into `NewCrdtClientMessage` (for inserts, no `position`) and `CrdtClientMessage` (for queries, includes `position`).

  - ***Indexes on `crdt_messages` (initial migration)***:
    - `idx_crdt_messages_group_id_timestamp_client_id ON crdt_messages (group_id, timestamp ASC, client_id)` — covering index for the hot sync query path: `WHERE group_id = ? AND timestamp > ? AND client_id != ? ORDER BY timestamp ASC`. Column order places the equality filter (`group_id`) first, the range+sort column (`timestamp`) second, and the inequality filter (`client_id`) last so PostgreSQL can satisfy the `WHERE` and `ORDER BY` entirely from the index with no additional heap sort.
    - `idx_crdt_messages_group_id_client_id ON crdt_messages (group_id, client_id)` — covering index for the bootstrap path where `timestamp > ''` (client merkle is empty). Keeps the bootstrap scan efficient independently of the diff path index.
    - `idx_crdt_client_messages_client_id_record_id ON crdt_client_messages (client_id, record_id)` — initial index on the client message buffer (later replaced by `idx_crdt_client_messages_client_id_position` in the position migration above).

  - ***`GET /app/sync/chunk/status` endpoint***:
    - Returns the current row count in `crdt_client_messages` for a given `client_id`. Used by the client to poll whether background inserts have completed before fetching chunks.

### Changed
  - ***Server sync handler — immediate response with background inserts***:
    - `controllers/main_controllers.rs` — when `new_messages.len() >= outgoing_limit`, the handler now serializes all messages to `NewCrdtClientMessage` structs in memory (fast, no DB I/O), responds immediately with `{ incomplete: 1, total: N }`, and offloads the actual DB writes to a `std::thread::spawn` background thread. The client polls `/chunk/status` while inserts proceed.
    - Background thread inserts messages into `crdt_client_messages` in batches of 10,000 using `diesel::insert_into(...).values(chunk).on_conflict(record_id).do_nothing()`.
    - Values are deserialized on the server before storage (using `deserialize_value`) so clients receive clean JSON (`0`, `"2026-02-22"`, etc.) rather than the internal serialized form (`N:0`, `S:2026-02-22`).

  - ***`GET /app/sync/chunk` — order by `position` instead of `record_id`***:
    - Chunk pages are now fetched `ORDER BY position ASC`, which preserves strict insertion order (matching the original `timestamp.asc()` order from `crdt_messages`). The previous `record_id` (ULID) ordering was effectively random within the same millisecond, causing FK violations when dependent records (e.g. `organizations` before `account_organizations`) arrived out of order.

  - ***DB connection pool***:
    - `db/db.rs` — pool `max_size` raised from 10 to 50. Configurable via `DB_POOL_SIZE` environment variable (e.g. `DB_POOL_SIZE=100`).
