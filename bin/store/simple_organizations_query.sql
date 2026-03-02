SELECT "organizations"."id", "organizations"."code", "organizations"."name", "organizations"."categories", "organizations"."district_id", "organizations"."department_id", "organizations"."city", "organizations"."county", "organizations"."state", "organizations"."school_identifier", "organizations"."district_identifier", "organizations"."status", "organizations"."superintendent_id", "organizations"."principal_id"
FROM organizations 
WHERE (organizations.tombstone = 0) 
  AND organizations.status IN ('Active', 'Draft') 
  AND (organizations.categories::text NOT ILIKE '%Personal%' AND organizations.categories::text NOT ILIKE '%Root%' AND organizations.categories::text NOT ILIKE '%Team%') 
ORDER BY LOWER(organizations.name) ASC NULLS FIRST 
LIMIT 100;