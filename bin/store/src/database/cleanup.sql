DROP SCHEMA IF EXISTS diesel CASCADE;

DO $$ 
DECLARE
    r RECORD;
BEGIN
    IF to_regclass('cron.job') IS NOT NULL THEN
        FOR r IN (SELECT jobid FROM cron.job WHERE jobname LIKE 'refresh\_%' ESCAPE '\')
        LOOP
            PERFORM cron.unschedule(r.jobid);
        END LOOP;
        DELETE FROM cron.job_run_details WHERE jobid NOT IN (SELECT jobid FROM cron.job);
    END IF;

    -- Drop only materialized views with mv_ prefix in public schema
    FOR r IN (
        SELECT matviewname
        FROM pg_matviews
        WHERE schemaname = 'public'
          AND matviewname LIKE 'mv\_%' ESCAPE '\'
    )
    LOOP
        EXECUTE 'DROP MATERIALIZED VIEW IF EXISTS public.' || quote_ident(r.matviewname) || ' CASCADE';
    END LOOP;

    -- Drop procedures with udp_ prefix in public schema
    FOR r IN (
        SELECT n.nspname AS schema_name,
               p.proname AS routine_name,
               pg_get_function_identity_arguments(p.oid) AS args
        FROM pg_proc p
        JOIN pg_namespace n ON n.oid = p.pronamespace
        WHERE n.nspname = 'public'
          AND p.prokind = 'p'
          AND p.proname LIKE 'udp\_%' ESCAPE '\'
    )
    LOOP
        EXECUTE 'DROP PROCEDURE IF EXISTS ' || quote_ident(r.schema_name) || '.' || quote_ident(r.routine_name) || '(' || r.args || ') CASCADE';
    END LOOP;

    -- Drop remaining non-extension-owned functions in public schema
    FOR r IN (
        SELECT n.nspname AS schema_name,
               p.proname AS routine_name,
               pg_get_function_identity_arguments(p.oid) AS args
        FROM pg_proc p
        JOIN pg_namespace n ON n.oid = p.pronamespace
        WHERE n.nspname = 'public'
          AND p.prokind <> 'p'
          AND NOT EXISTS (
              SELECT 1
              FROM pg_depend d
              JOIN pg_extension e ON e.oid = d.refobjid
              WHERE d.objid = p.oid
                AND d.deptype = 'e'
          )
    )
    LOOP
        EXECUTE 'DROP FUNCTION IF EXISTS ' || quote_ident(r.schema_name) || '.' || quote_ident(r.routine_name) || '(' || r.args || ') CASCADE';
    END LOOP;

    -- Drop user-defined triggers that may exist on views in public schema
    FOR r IN (
        SELECT c.relname AS viewname, t.tgname
        FROM pg_trigger t
        JOIN pg_class c ON c.oid = t.tgrelid
        JOIN pg_namespace n ON n.oid = c.relnamespace
        WHERE n.nspname = 'public'
          AND c.relkind = 'v'
          AND t.tgisinternal = false
    )
    LOOP
        EXECUTE 'DROP TRIGGER IF EXISTS ' || quote_ident(r.tgname) || ' ON public.' || quote_ident(r.viewname) || ' CASCADE';
    END LOOP;

    -- Drop user-defined triggers that may exist on tables in public schema
    FOR r IN (
        SELECT c.relname AS tablename, t.tgname
        FROM pg_trigger t
        JOIN pg_class c ON c.oid = t.tgrelid
        JOIN pg_namespace n ON n.oid = c.relnamespace
        WHERE n.nspname = 'public'
          AND c.relkind = 'r'
          AND t.tgisinternal = false   -- skip system/internal triggers
          AND t.tgconstraint = 0       -- skip constraint/FK-backed triggers (RI_ConstraintTrigger_*)
    )
    LOOP
        EXECUTE 'DROP TRIGGER IF EXISTS ' || quote_ident(r.tgname) || ' ON public.' || quote_ident(r.tablename) || ' CASCADE';
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
