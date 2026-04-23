SELECT 
  'procedure' AS object_type
  ,
  n.nspname AS schema_name
  ,
  p.proname AS object_name
  ,
  NULL AS schedule,
  NULL AS extra_info
FROM pg_proc p
JOIN pg_namespace n ON p.pronamespace = n.oid
WHERE p.prokind = 'p' AND p.proname LIKE '%udp_refresh%'

UNION ALL

SELECT
  'materialized_view' AS object_type
  ,
  schemaname AS schema_name
  ,
  matviewname AS object_name
  ,
  NULL AS schedule,
  definition AS extra_info
FROM pg_matviews WHERE matviewname LIKE '%mv_%'

UNION ALL

SELECT
  'cron_job' AS object_type
  ,
  'cron' AS schema_name
  ,
  jobid::text AS object_name
  ,
  schedule,
  command AS extra_info
FROM cron.job LIMIT 100;


# get extensions
SELECT current_setting('shared_preload_libraries'); should include pg_cron

# job schedule
SELECT jobid, jobname, schedule, command, active FROM cron.job ORDER BY jobid DESC

# check database name
SHOW cron.database_name

# Recent runs
SELECT jobid, status, return_message, start_time, end_time FROM cron.job_run_details ORDER BY start_time DESC LIMIT 10;


# Confirm unique index exists
SELECT i.relname AS index_name, ix.indisunique, pg_get_indexdef(ix.indexrelid)
FROM pg_index ix
JOIN pg_class i ON i.oid = ix.indexrelid
JOIN pg_class t ON t.oid = ix.indrelid
JOIN pg_namespace n ON n.oid = t.relnamespace
WHERE n.nspname = 'public'
  AND t.relname = 'mv_contacts_by_status_70759285e43a19e0';