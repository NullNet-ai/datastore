DROP SCHEMA IF EXISTS diesel CASCADE;

DO $$ 
DECLARE
    r RECORD;
BEGIN
    -- Drop all materialized views in public schema
    FOR r IN (SELECT matviewname FROM pg_matviews WHERE schemaname = 'public')
    LOOP
        EXECUTE 'DROP MATERIALIZED VIEW IF EXISTS public.' || quote_ident(r.matviewname) || ' CASCADE';
    END LOOP;

    -- Drop all procedures and functions in public schema
    FOR r IN (
        SELECT n.nspname AS schema_name,
               p.proname AS routine_name,
               pg_get_function_identity_arguments(p.oid) AS args,
               p.prokind
        FROM pg_proc p
        JOIN pg_namespace n ON n.oid = p.pronamespace
        WHERE n.nspname = 'public'
    )
    LOOP
        IF r.prokind = 'p' THEN
            EXECUTE 'DROP PROCEDURE IF EXISTS ' || quote_ident(r.schema_name) || '.' || quote_ident(r.routine_name) || '(' || r.args || ') CASCADE';
        ELSE
            EXECUTE 'DROP FUNCTION IF EXISTS ' || quote_ident(r.schema_name) || '.' || quote_ident(r.routine_name) || '(' || r.args || ') CASCADE';
        END IF;
    END LOOP;

    -- Drop triggers that may exist on views in public schema
    FOR r IN (
        SELECT c.relname AS viewname, t.tgname
        FROM pg_trigger t
        JOIN pg_class c ON c.oid = t.tgrelid
        JOIN pg_namespace n ON n.oid = c.relnamespace
        WHERE n.nspname = 'public' AND c.relkind = 'v'
    )
    LOOP
        EXECUTE 'DROP TRIGGER IF EXISTS ' || quote_ident(r.tgname) || ' ON public.' || quote_ident(r.viewname) || ' CASCADE';
    END LOOP;

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
