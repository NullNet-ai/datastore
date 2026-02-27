select "organizations"."id", "organizations"."code", "organizations"."name", "organizations"."categories", "organizations"."district_id", "organizations"."department_id", "organizations"."city", "organizations"."county", "organizations"."state", "organizations"."school_identifier", "organizations"."district_identifier", "organizations"."status", "organizations"."superintendent_id", "organizations"."principal_id", 
to_char((("organizations"."created_date"::timestamp))::date, 'mm/dd/YYYY')
             AS "created_date", 
to_char((("organizations"."created_time"::interval))::time, 'HH24:MI')::text
             AS "created_time", COALESCE(
              (
                SELECT JSONB_AGG(elem)
                FROM (
                  SELECT
                    JSONB_BUILD_OBJECT('id', "created_by"."id", 'first_name', "created_by"."first_name", 'last_name', "created_by"."last_name", 'full_name', (COALESCE("created_by"."first_name", '') || ' ' || COALESCE("created_by"."last_name", ''))) AS elem
                  FROM "account_organizations" "created_by_account_organizations"
                  LEFT JOIN "contacts" "created_by" ON "created_by"."id" = "created_by_account_organizations"."contact_id"
                  WHERE ("created_by"."tombstone" = 0 AND "created_by"."organization_id" IS NOT NULL AND "created_by"."organization_id" = '01JBHKXHYSKPP247HZZWHA3JCT' AND "created_by_account_organizations"."tombstone" = 0 AND "created_by_account_organizations"."organization_id" IS NOT NULL AND "created_by_account_organizations"."organization_id" = '01JBHKXHYSKPP247HZZWHA3JCT' AND "organizations"."created_by" = "created_by_account_organizations"."id")
                ) sub
            ), '[]') as "created_by", to_char((("organizations"."updated_date"::timestamp))::date, 'mm/dd/YYYY')
             AS "updated_date", to_char((("organizations"."updated_time"::interval))::time, 'HH24:MI')::text
             AS "updated_time", 
             COALESCE(
              (
                SELECT JSONB_AGG(elem)
                FROM (
                  SELECT
                    JSONB_BUILD_OBJECT('id', "updated_by"."id", 'first_name', "updated_by"."first_name", 'last_name', "updated_by"."last_name", 'full_name', (COALESCE("updated_by"."first_name", '') || ' ' || COALESCE("updated_by"."last_name", ''))) AS elem
                  FROM "account_organizations" "updated_by_account_organizations"
                  LEFT JOIN "contacts" "updated_by" ON "updated_by"."id" = "updated_by_account_organizations"."contact_id"
                  WHERE ("updated_by"."tombstone" = 0 AND "updated_by"."organization_id" IS NOT NULL AND "updated_by"."organization_id" = '01JBHKXHYSKPP247HZZWHA3JCT' AND "updated_by_account_organizations"."tombstone" = 0 AND "updated_by_account_organizations"."organization_id" IS NOT NULL AND "updated_by_account_organizations"."organization_id" = '01JBHKXHYSKPP247HZZWHA3JCT' AND "organizations"."updated_by" = "updated_by_account_organizations"."id")
                ) sub
            ), '[]') as "updated_by", (COALESCE(to_char((("organizations"."created_date"::timestamp))::date, 'mm/dd/YYYY'), '') || ' ' || COALESCE(to_char((("organizations"."created_time"::interval))::time, 'HH24:MI')::text, '')) AS created_date_time, (COALESCE(to_char((("organizations"."updated_date"::timestamp))::date, 'mm/dd/YYYY'), '') || ' ' || COALESCE(to_char((("organizations"."updated_time"::interval))::time, 'HH24:MI')::text, '')) AS updated_date_time, COALESCE(
              (
                SELECT JSONB_AGG(elem)
                FROM (
                  SELECT
                    JSONB_BUILD_OBJECT('id', "created_by_account_organizations"."id", 'contact_id', "created_by_account_organizations"."contact_id") AS elem
                  FROM "account_organizations" "created_by_account_organizations"

                  WHERE ("created_by_account_organizations"."tombstone" = 0 AND "created_by_account_organizations"."organization_id" IS NOT NULL AND "created_by_account_organizations"."organization_id" = '01JBHKXHYSKPP247HZZWHA3JCT' AND "organizations"."created_by" = "created_by_account_organizations"."id")
                ) sub
            ), '[]') as "created_by_account_organizations", COALESCE(
              (
                SELECT JSONB_AGG(elem)
                FROM (
                  SELECT
                    JSONB_BUILD_OBJECT('id', "updated_by_account_organizations"."id", 'contact_id', "updated_by_account_organizations"."contact_id") AS elem
                  FROM "account_organizations" "updated_by_account_organizations"

                  WHERE ("updated_by_account_organizations"."tombstone" = 0 AND "updated_by_account_organizations"."organization_id" IS NOT NULL AND "updated_by_account_organizations"."organization_id" = '01JBHKXHYSKPP247HZZWHA3JCT' AND "organizations"."updated_by" = "updated_by_account_organizations"."id")
                ) sub
            ), '[]') as "updated_by_account_organizations", COALESCE(
              (
                SELECT JSONB_AGG(elem)
                FROM (
                  SELECT
                    JSONB_BUILD_OBJECT('id', "district_orgs"."id", 'code', "district_orgs"."code", 'name', "district_orgs"."name", 'categories', "district_orgs"."categories", 'district_id', "district_orgs"."district_id", 'department_id', "district_orgs"."department_id", 'city', "district_orgs"."city", 'county', "district_orgs"."county", 'state', "district_orgs"."state", 'school_identifier', "district_orgs"."school_identifier", 'district_identifier', "district_orgs"."district_identifier", 'status', "district_orgs"."status", 'created_date', to_char((("district_orgs"."created_date"::timestamp))::date, 'mm/dd/YYYY'), 'created_time', to_char((("district_orgs"."created_time"::interval))::time, 'HH24:MI')::text, 'created_by', "district_orgs"."created_by", 'updated_date', to_char((("district_orgs"."updated_date"::timestamp))::date, 'mm/dd/YYYY'), 'updated_time', to_char((("district_orgs"."updated_time"::interval))::time, 'HH24:MI')::text, 'updated_by', "district_orgs"."updated_by", 'superintendent_id', "district_orgs"."superintendent_id", 'principal_id', "district_orgs"."principal_id") AS elem
                  FROM "organizations" "district_orgs"

                  WHERE ("district_orgs"."tombstone" = 0 AND "district_orgs"."organization_id" IS NOT NULL AND "district_orgs"."organization_id" = '01JBHKXHYSKPP247HZZWHA3JCT' AND "organizations"."district_id" = "district_orgs"."id")
                ) sub
            ), '[]') as "district_orgs", COALESCE(
              (
                SELECT JSONB_AGG(elem)
                FROM (
                  SELECT
                    JSONB_BUILD_OBJECT('first_name', "district_superintendent"."first_name", 'code', "district_superintendent"."code", 'last_name', "district_superintendent"."last_name", 'username', "district_superintendent"."username", 'id', "district_superintendent"."id", 'full_name', (COALESCE("district_superintendent"."first_name", '') || ' ' || COALESCE("district_superintendent"."last_name", ''))) AS elem
                  FROM "organizations" "district_orgs"
                  LEFT JOIN "contacts" "district_superintendent" ON "district_superintendent"."id" = "district_orgs"."superintendent_id"
                  WHERE ("district_superintendent"."tombstone" = 0 AND "district_superintendent"."organization_id" IS NOT NULL AND "district_superintendent"."organization_id" = '01JBHKXHYSKPP247HZZWHA3JCT' AND "district_orgs"."tombstone" = 0 AND "district_orgs"."organization_id" IS NOT NULL AND "district_orgs"."organization_id" = '01JBHKXHYSKPP247HZZWHA3JCT' AND "organizations"."district_id" = "district_orgs"."id")
                ) sub
            ), '[]') as "district_superintendent", COALESCE(
              (
                SELECT JSONB_AGG(elem)
                FROM (
                  SELECT
                    JSONB_BUILD_OBJECT('first_name', "superintendent"."first_name", 'code', "superintendent"."code", 'last_name', "superintendent"."last_name", 'username', "superintendent"."username", 'id', "superintendent"."id", 'full_name', (COALESCE("superintendent"."first_name", '') || ' ' || COALESCE("superintendent"."last_name", ''))) AS elem
                  FROM "contacts" "superintendent"

                  WHERE ("superintendent"."tombstone" = 0 AND "superintendent"."organization_id" IS NOT NULL AND "superintendent"."organization_id" = '01JBHKXHYSKPP247HZZWHA3JCT' AND "organizations"."superintendent_id" = "superintendent"."id")
                ) sub
            ), '[]') as "superintendent", COALESCE(
              (
                SELECT JSONB_AGG(elem)
                FROM (
                  SELECT
                    JSONB_BUILD_OBJECT('first_name', "principal"."first_name", 'code', "principal"."code", 'last_name', "principal"."last_name", 'username', "principal"."username", 'id', "principal"."id", 'full_name', (COALESCE("principal"."first_name", '') || ' ' || COALESCE("principal"."last_name", ''))) AS elem
                  FROM "contacts" "principal"

                  WHERE ("principal"."tombstone" = 0 AND "principal"."organization_id" IS NOT NULL AND "principal"."organization_id" = '01JBHKXHYSKPP247HZZWHA3JCT' AND "organizations"."principal_id" = "principal"."id")
                ) sub
            ), '[]') as "principal" from "organizations" left join 
                LATERAL (
                  SELECT "created_by_account_organizations"."id", "created_by_account_organizations"."contact_id", "created_by_account_organizations"."tombstone", "created_by_account_organizations"."organization_id"
                  from "account_organizations" "created_by_account_organizations" where ("created_by_account_organizations"."tombstone" = 0 and "created_by_account_organizations"."organization_id" is not null and "created_by_account_organizations"."organization_id" = '01JBHKXHYSKPP247HZZWHA3JCT') AND "organizations"."created_by" = "created_by_account_organizations"."id"



                ) AS "created_by_account_organizations"
               on TRUE left join 
                LATERAL (
                  SELECT "created_by"."id", "created_by"."first_name", "created_by"."last_name", "created_by"."tombstone", "created_by"."organization_id"
                  from "contacts" "created_by" where ("created_by"."tombstone" = 0 and "created_by"."organization_id" is not null and "created_by"."organization_id" = '01JBHKXHYSKPP247HZZWHA3JCT' and "created_by"."id" = "created_by_account_organizations"."contact_id") 



                ) AS "created_by"
               on TRUE left join 
                LATERAL (
                  SELECT "updated_by_account_organizations"."id", "updated_by_account_organizations"."contact_id", "updated_by_account_organizations"."tombstone", "updated_by_account_organizations"."organization_id"
                  from "account_organizations" "updated_by_account_organizations" where ("updated_by_account_organizations"."tombstone" = 0 and "updated_by_account_organizations"."organization_id" is not null and "updated_by_account_organizations"."organization_id" = '01JBHKXHYSKPP247HZZWHA3JCT') AND "organizations"."updated_by" = "updated_by_account_organizations"."id"



                ) AS "updated_by_account_organizations"
               on TRUE left join 
                LATERAL (
                  SELECT "updated_by"."id", "updated_by"."first_name", "updated_by"."last_name", "updated_by"."tombstone", "updated_by"."organization_id"
                  from "contacts" "updated_by" where ("updated_by"."tombstone" = 0 and "updated_by"."organization_id" is not null and "updated_by"."organization_id" = '01JBHKXHYSKPP247HZZWHA3JCT' and "updated_by"."id" = "updated_by_account_organizations"."contact_id") 



                ) AS "updated_by"
               on TRUE left join 
                LATERAL (
                  SELECT "district_orgs"."id", "district_orgs"."code", "district_orgs"."name", "district_orgs"."categories", "district_orgs"."district_id", "district_orgs"."department_id", "district_orgs"."city", "district_orgs"."county", "district_orgs"."state", "district_orgs"."school_identifier", "district_orgs"."district_identifier", "district_orgs"."status", "district_orgs"."created_date", "district_orgs"."created_time", "district_orgs"."created_by", "district_orgs"."updated_date", "district_orgs"."updated_time", "district_orgs"."updated_by", "district_orgs"."superintendent_id", "district_orgs"."principal_id", "district_orgs"."tombstone", "district_orgs"."organization_id"
                  from "organizations" "district_orgs" where ("district_orgs"."tombstone" = 0 and "district_orgs"."organization_id" is not null and "district_orgs"."organization_id" = '01JBHKXHYSKPP247HZZWHA3JCT') AND "organizations"."district_id" = "district_orgs"."id"



                ) AS "district_orgs"
               on TRUE left join 
                LATERAL (
                  SELECT "district_superintendent"."first_name", "district_superintendent"."code", "district_superintendent"."last_name", "district_superintendent"."username", "district_superintendent"."id", "district_superintendent"."tombstone", "district_superintendent"."organization_id"
                  from "contacts" "district_superintendent" where ("district_superintendent"."tombstone" = 0 and "district_superintendent"."organization_id" is not null and "district_superintendent"."organization_id" = '01JBHKXHYSKPP247HZZWHA3JCT' and "district_superintendent"."id" = "district_orgs"."superintendent_id") 



                ) AS "district_superintendent"
               on TRUE left join 
                LATERAL (
                  SELECT "superintendent"."first_name", "superintendent"."code", "superintendent"."last_name", "superintendent"."username", "superintendent"."id", "superintendent"."tombstone", "superintendent"."organization_id"
                  from "contacts" "superintendent" where ("superintendent"."tombstone" = 0 and "superintendent"."organization_id" is not null and "superintendent"."organization_id" = '01JBHKXHYSKPP247HZZWHA3JCT') AND "organizations"."superintendent_id" = "superintendent"."id"



                ) AS "superintendent"
               on TRUE left join 
                LATERAL (
                  SELECT "principal"."first_name", "principal"."code", "principal"."last_name", "principal"."username", "principal"."id", "principal"."tombstone", "principal"."organization_id"
                  from "contacts" "principal" where ("principal"."tombstone" = 0 and "principal"."organization_id" is not null and "principal"."organization_id" = '01JBHKXHYSKPP247HZZWHA3JCT') AND "organizations"."principal_id" = "principal"."id"



                ) AS "principal"
               on TRUE where ("organizations"."tombstone" = $1 and "organizations"."organization_id" is not null and "organizations"."organization_id" = $2) group by "organizations"."id" order by lower("organizations"."name") ASC NULLS FIRST limit $3