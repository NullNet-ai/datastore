DROP SCHEMA IF EXISTS diesel CASCADE;

DO $$ 
DECLARE
    r RECORD;
BEGIN
    -- Disable triggers temporarily
    EXECUTE 'SET session_replication_role = replica';
    
    -- Loop through all tables in the public schema and drop them
    FOR r IN (SELECT tablename FROM pg_tables WHERE schemaname = 'public') 
    LOOP
        EXECUTE 'DROP TABLE IF EXISTS public.' || quote_ident(r.tablename) || ' CASCADE';
    END LOOP;
    
    -- Re-enable triggers
    EXECUTE 'SET session_replication_role = DEFAULT';
END $$;

-- Reset TimescaleDB so chunk catalog is cleared (avoids "cache lookup failed for relation _hyper_*_chunk"
-- after cleanup when migrations recreate hypertables).
DROP EXTENSION IF EXISTS timescaledb CASCADE;
CREATE EXTENSION IF NOT EXISTS timescaledb;