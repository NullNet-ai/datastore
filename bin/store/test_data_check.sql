-- Test query with relaxed filters to check for data
SELECT "organizations"."id", "organizations"."code", "organizations"."name", "organizations"."categories", "organizations"."status"
FROM organizations 
WHERE (organizations.tombstone = 0) 
  -- Remove status filter to see all statuses
  -- Remove categories filter to see all categories
ORDER BY LOWER(organizations.name) ASC NULLS FIRST 
LIMIT 10;