CREATE TABLE stream_queue (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL UNIQUE,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    last_accessed TIMESTAMPTZ
);

CREATE INDEX idx_stream_queue_name ON stream_queue(name);
CREATE INDEX idx_stream_queue_created_at ON stream_queue(created_at);
CREATE INDEX idx_stream_queue_last_accessed ON stream_queue(last_accessed);