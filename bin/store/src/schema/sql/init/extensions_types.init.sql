DROP TYPE IF EXISTS field_type;
CREATE TYPE field_type AS (
    id TEXT,
   label TEXT,
  name TEXT,
  type TEXT,
  created_by TEXT,
  is_encryptable BOOLEAN
);

DROP TYPE IF EXISTS permission_type;
CREATE TYPE permission_type AS (
    id TEXT, 
    read BOOLEAN, 
    write BOOLEAN, 
    encrypt BOOLEAN, 
    decrypt BOOLEAN, 
    required BOOLEAN, 
    sensitive BOOLEAN, 
    archive BOOLEAN, 
    delete BOOLEAN, 
    created_by TEXT
);
CREATE EXTENSION IF NOT EXISTS "pgcrypto";