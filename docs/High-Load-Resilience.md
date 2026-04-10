# High‑Load Resilience Guide

This guide explains how the system behaves under heavy request pressure, how backpressure is applied, and which environment variables to tune for stable high‑throughput operation.

## What Happens Under Pressure
- Sync server backpressure:
  - Incoming sync requests that produce large outgoing batches are enqueued to a bounded background insert worker. The queue capacity is controlled by `INSERT_QUEUE_CAPACITY`.
  - When the queue is full, the server immediately returns `503` with `{ "status": "error", "message": "server_busy" }`. Clients should retry with jitter. This prevents unbounded thread creation and protects Postgres.
- Database connection pool caps:
  - Both services use a capped Postgres connection pool. When all connections are in use, new DB work waits instead of spawning more connections.
- Actix workers and keep‑alive:
  - The HTTP server concurrency is the Actix worker count. If `WORKERS` is unset, the default equals the number of logical CPU cores. HTTP keep‑alive is controlled by `KEEP_ALIVE_SECS`.
- Health classification during spikes:
  - If DB health checks time out, the store reports a “Degraded” status (not “Unhealthy”), so instances keep serving while signaling reduced capacity.

## Key Environment Variables
- HTTP/Runtime
  - `WORKERS` – Optional integer worker count. Leave unset to use the default (logical cores). Set explicitly for deterministic tuning.
  - `KEEP_ALIVE_SECS` – Integer seconds for HTTP keep‑alive (default `30`).
- Database
  - `DATABASE_POOL_SIZE` – Max DB pool size (store and sync server). Start around `20–64` depending on DB resources.
  - `DATABASE_URL` – Postgres connection string.
- Backpressure (sync server)
  - `INSERT_QUEUE_CAPACITY` – Bounded queue length for background inserts. Increase to buffer larger bursts. Default `100` for dev; consider `200–1000` in production.
- Logging
  - `RUST_LOG` – Use `info` or `warn` in production to reduce overhead.

## Recommended Production Baselines
- Actix workers: leave `WORKERS` unset, or set ≈ number of CPU cores for CPU‑bound, up to 2× cores for I/O‑bound workloads. Validate with latency SLOs.
- Keep‑alive: `KEEP_ALIVE_SECS=30` to reuse connections through load balancers.
- DB pool: size `DATABASE_POOL_SIZE` to match Postgres `max_connections` and the number of app instances. Ensure pgbouncer (transaction mode) is used if you need very high client concurrency.
- Backpressure queue: start with `INSERT_QUEUE_CAPACITY=200` and tune after observing 503 rates and DB load.
- Logging: `RUST_LOG=info` to minimize I/O overhead under load.

## Observability
- Metrics endpoint:
  - Store exposes Prometheus metrics at `/api/metrics`.
  - Scrape core histograms and counters (HTTP latency/throughput, DB pool gauges).
- Health endpoints:
  - `/api/health` for basic health.
  - Timeouts in DB checks are reported as “Degraded” to avoid unnecessary instance rotation during transient spikes.

## Load Test Checklist
1. Set environment:
   - `KEEP_ALIVE_SECS=30`
   - `WORKERS=` (unset or a chosen value)
   - `DATABASE_POOL_SIZE` sized to your DB
   - `INSERT_QUEUE_CAPACITY=200` (or higher for bursty sync)
2. Start services (e.g., `make store`) and confirm:
   - Health at `/api/health`
   - Metrics at `/api/metrics`
3. Run a load generator against representative endpoints (read and write).
4. Watch:
   - HTTP p95/p99 latency and error rates
   - DB pool gauges (active/idle/size)
   - Rate of `503 server_busy` responses (indicates queue saturation)
5. Tune:
   - Increase `DATABASE_POOL_SIZE` if connections are the bottleneck (ensure DB headroom).
   - Increase `INSERT_QUEUE_CAPACITY` to buffer bursts if 503s are frequent but DB has headroom.
   - Adjust `WORKERS` for better CPU utilization or to reduce context switching.

## Failure Modes and Responses
- Queue saturation:
  - Symptom: `503` with `server_busy`.
  - Action: Increase `INSERT_QUEUE_CAPACITY`, scale instances, and verify DB capacity.
- DB health timeouts:
  - Symptom: Health moves to Degraded; requests may slow but service continues.
  - Action: Increase pool size, reduce per‑request DB load, scale out, and confirm Postgres tuning.
- Worker starvation:
  - Symptom: Rising request latency with low CPU utilization.
  - Action: Increase `WORKERS` or reduce handler blocking; confirm any synchronous work is offloaded.

## Quick Reference: Example Production Env
```
HOST=0.0.0.0
PORT=5005
RUST_LOG=info
DEBUG=false
TZ=UTC
KEEP_ALIVE_SECS=30
WORKERS=

DATABASE_URL=postgres://USER:PASS@DB_HOST:5432/DB_NAME
DATABASE_POOL_SIZE=50

INSERT_QUEUE_CAPACITY=200

REDIS_CONNECTION=redis://redis:6379/0
CACHE_TTL=3600
CACHE_TYPE=redis

STORAGE_ENDPOINT=https://your-minio-or-s3-endpoint
STORAGE_ACCESS_KEY=...
STORAGE_SECRET_KEY=...
STORAGE_BUCKET_NAME=your-bucket
STORAGE_REGION=us-east-1
STORAGE_DISABLE_SSL_VERIFICATION=false
```

Use these as a baseline and tune per environment. Always validate with load tests and production metrics.

