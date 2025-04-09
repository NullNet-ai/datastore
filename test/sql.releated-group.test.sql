select 
-- selections
"contacts"."last_name", 
"contacts"."code", 
"contacts"."categories", 
"contacts"."organization_id", 
"contacts"."first_name", 
"contacts"."middle_name", 
"contacts"."status", 
"contacts"."created_time", 
"contacts"."updated_time", 
"contacts"."updated_by", 
"contacts"."previous_status",
to_char("contacts"."created_date"::timestamp, 'mm-dd-YYYY') AS "created_date", 
to_char("contacts"."updated_date"::timestamp, 'mm-dd-YYYY') AS "updated_date", 
-- aggregations
COALESCE(
JSONB_AGG( DISTINCT
    JSONB_BUILD_OBJECT('id', "created_by"."id", 'account_id', "created_by"."account_id")
) FILTER (WHERE "created_by"."id" IS NOT NULL),
'[]'
) as "created_by",
COUNT(*), 
COUNT(*) OVER (), 
MIN("contacts"."id")
-- tables
from "contacts" 
-- join tables
left join LATERAL
(
    SELECT "joined_created_by"."id", "joined_created_by"."account_id"
    from "organization_accounts" "joined_created_by" 
    where (
        "joined_created_by"."tombstone" = 0 
        and "joined_created_by"."organization_id" is not null 
        and "joined_created_by"."organization_id" = 'ee1b9a50-51ec-4ecf-bcc2-8f9511f9feb8'
    ) AND "contacts"."created_by" = "joined_created_by"."id"
) AS "created_by" on TRUE 

where ("contacts"."tombstone" = 0 
       and "contacts"."organization_id" is not null 
       and "contacts"."organization_id" = 'ee1b9a50-51ec-4ecf-bcc2-8f9511f9feb8'
      ) 
      
group by (
  "contacts"."last_name",
  "contacts"."code", 
  "contacts"."categories", 
  "contacts"."organization_id",
  "contacts"."first_name", 
  "contacts"."middle_name", 
  "contacts"."status", 
  "contacts"."created_time", 
  "contacts"."updated_time", 
  "contacts"."updated_by", 
  "contacts"."previous_status",
  "created_date",
  "updated_date"
) 
order by MIN("contacts"."id") asc 
limit 500