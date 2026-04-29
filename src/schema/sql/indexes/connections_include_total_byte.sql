CREATE INDEX IF NOT EXISTS connections_device_id_source_ip_timestamp_include_total_byte_idx
  ON connections (device_id, source_ip, timestamp DESC)
  INCLUDE (total_byte);
