CREATE TABLE stream_queue_items (
    id TEXT PRIMARY KEY,
    queue_name TEXT NOT NULL REFERENCES stream_queue(name) ON DELETE CASCADE,
    content JSONB NOT NULL,
    timestamp TIMESTAMPTZ DEFAULT NOW()
);

CREATE INDEX idx_stream_queue_items_queue_name ON stream_queue_items(queue_name);
CREATE INDEX idx_stream_queue_items_timestamp ON stream_queue_items(timestamp);