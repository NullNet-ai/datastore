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

-- Keep extension setup aligned with bin/store/docker/init/001-enable-timescaledb.sql
CREATE EXTENSION IF NOT EXISTS timescaledb;
CREATE EXTENSION IF NOT EXISTS pg_cron;
