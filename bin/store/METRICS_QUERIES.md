# Prometheus Query Cookbook (bin/store)

Scrape target: `store:9000`

## RED Metrics

- Requests/sec (by route and method)

```
sum by (http_route, http_request_method)(
  rate(http_server_request_duration_count[1m])
)
```

- Error rate (5xx / all, by route and method)

```
(
  sum by (http_route, http_request_method)(
    rate(http_server_request_duration_count{http_response_status_code=~"5.."}[5m])
  )
)
/
(
  sum by (http_route, http_request_method)(
    rate(http_server_request_duration_count[5m])
  )
)
```

- Latency p50/p95/p99 (by route and method)

```
histogram_quantile(
  0.5,
  sum by (le, http_route, http_request_method)(
    rate(http_server_request_duration_bucket[5m])
  )
)
```

```
histogram_quantile(
  0.95,
  sum by (le, http_route, http_request_method)(
    rate(http_server_request_duration_bucket[5m])
  )
)
```

```
histogram_quantile(
  0.99,
  sum by (le, http_route, http_request_method)(
    rate(http_server_request_duration_bucket[5m])
  )
)
```

- Active requests (current)

```
sum(http_server_active_requests)
```

## System Metrics

- CPU usage (process-level, per-second rate)

```
rate(process_cpu_seconds_total[1m])
```

- Memory (RSS bytes)

```
process_resident_memory_bytes
```

- Open file descriptors

```
process_open_fds
```

- Max file descriptors

```
process_max_fds
```

## Tokio Runtime

- Global queue depth (tasks waiting)

```
tokio_global_queue_depth
```

- Workers count

```
tokio_num_workers
```

- Alive tasks

```
tokio_num_alive_tasks
```

## Database Metrics

- DB connection errors/sec

```
rate(db_connection_errors_total[5m])
```

- Pool size

```
db_pool_size
```

- Active connections

```
db_pool_active_connections
```

- Idle connections

```
db_pool_idle_connections
```

- Pool utilization ratio

```
db_pool_active_connections / db_pool_size
```

## Response Size

- Average response size (bytes, by route/method)

```
(
  sum by (http_route, http_request_method)(
    rate(http_server_response_body_size_sum[5m])
  )
)
/
(
  sum by (http_route, http_request_method)(
    rate(http_server_response_body_size_count[5m])
  )
)
```

## Useful Variants

- Top erroring routes (5xx rate, top 10)

```
topk(10,
  sum by (http_route)(
    rate(http_server_request_duration_count{http_response_status_code=~"5.."}[5m])
  )
)
```

- Requests/sec per status code

```
sum by (http_response_status_code)(
  rate(http_server_request_duration_count[1m])
)
```

- Slowest routes (p95 latency, top 10)

```
topk(10,
  histogram_quantile(
    0.95,
    sum by (le, http_route)(
      rate(http_server_request_duration_bucket[5m])
    )
  )
)
```

```
{
  "api": {
      "throughput": "sum(rate(http_server_request_duration_count[1m]))",
      "errors": "(sum(rate(http_server_request_duration_count{http_response_status_code=~\"5..\"}[5m])))/(sum(rate(http_server_request_duration_count[5m])))"
  },
  "system": {
      "cpu_usage": "rate(process_cpu_seconds_total[1m])",
      "memory_usage": "process_resident_memory_bytes"
  }
}
```
