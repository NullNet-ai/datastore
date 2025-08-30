select "contacts"."id", "contacts"."code", "contacts"."categories", "contacts"."organization_id", "contacts"."first_name", "contacts"."middle_name", "contacts"."last_name", "contacts"."status", to_char((("contacts"."created_date"::timestamp + "contacts"."created_time"::interval))::date, 'mm/dd/YYYY')
             AS "created_date", to_char((("contacts"."updated_date"::timestamp + "contacts"."updated_time"::interval))::date, 'mm/dd/YYYY')
             AS "updated_date", (("contacts"."created_date"::timestamp + "contacts"."created_time"::interval))::time::text
             AS "created_time", (("contacts"."updated_date"::timestamp + "contacts"."updated_time"::interval))::time::text
             AS "updated_time", COALESCE(
              (
                SELECT JSONB_AGG(elem)
                FROM (
                  SELECT
                    JSONB_BUILD_OBJECT('id', "created_by"."id", 'first_name', "created_by"."first_name", 'last_name', "created_by"."last_name", 'full_name', (COALESCE("created_by"."first_name", '') || ' ' || COALESCE("created_by"."last_name", ''))) AS elem
                  FROM "account_organizations" "created_by_account_organizations"
                  LEFT JOIN "contacts" "created_by" ON "created_by"."id" = "created_by_account_organizations"."contact_id"
                  WHERE ("created_by"."tombstone" = 0 AND "created_by"."organization_id" IS NOT NULL AND "created_by"."organization_id" = '01JBHKXHYSKPP247HZZWHA3JCT' AND "created_by_account_organizations"."tombstone" = 0 AND "created_by_account_organizations"."organization_id" IS NOT NULL AND "created_by_account_organizations"."organization_id" = '01JBHKXHYSKPP247HZZWHA3JCT' AND "contacts"."created_by" = "created_by_account_organizations"."id")
                ) sub
            ), '[]') as "created_by", COALESCE(
              (
                SELECT JSONB_AGG(elem)
                FROM (
                  SELECT
                    JSONB_BUILD_OBJECT('id', "updated_by"."id", 'first_name', "updated_by"."first_name", 'last_name', "updated_by"."last_name", 'full_name', (COALESCE("updated_by"."first_name", '') || ' ' || COALESCE("updated_by"."last_name", ''))) AS elem
                  FROM "account_organizations" "updated_by_account_organizations"
                  LEFT JOIN "contacts" "updated_by" ON "updated_by"."id" = "updated_by_account_organizations"."contact_id"
                  WHERE ("updated_by"."tombstone" = 0 AND "updated_by"."organization_id" IS NOT NULL AND "updated_by"."organization_id" = '01JBHKXHYSKPP247HZZWHA3JCT' AND "updated_by_account_organizations"."tombstone" = 0 AND "updated_by_account_organizations"."organization_id" IS NOT NULL AND "updated_by_account_organizations"."organization_id" = '01JBHKXHYSKPP247HZZWHA3JCT' AND "contacts"."updated_by" = "updated_by_account_organizations"."id")
                ) sub
            ), '[]') as "updated_by", "contacts"."previous_status", (COALESCE(to_char((("contacts"."created_date"::timestamp + "contacts"."created_time"::interval))::date, 'mm/dd/YYYY'), '') || ' ' || COALESCE((("contacts"."created_date"::timestamp + "contacts"."created_time"::interval))::time::text, '')) AS created_date_time, (COALESCE(to_char((("contacts"."updated_date"::timestamp + "contacts"."updated_time"::interval))::date, 'mm/dd/YYYY'), '') || ' ' || COALESCE((("contacts"."updated_date"::timestamp + "contacts"."updated_time"::interval))::time::text, '')) AS updated_date_time, COALESCE(
              (
                SELECT JSONB_AGG(elem)
                FROM (
                  SELECT
                    JSONB_BUILD_OBJECT('email', "contact_emails"."email", 'is_primary', "contact_emails"."is_primary", 'id', "contact_emails"."id") AS elem
                  FROM "contact_emails"

                  WHERE ("contact_emails"."tombstone" = 0 AND "contact_emails"."organization_id" IS NOT NULL AND "contact_emails"."organization_id" = '01JBHKXHYSKPP247HZZWHA3JCT' AND "contacts"."id" = "contact_emails"."contact_id")
                ) sub
            ), '[]') as "contact_emails", COALESCE(
              (
                SELECT JSONB_AGG(elem)
                FROM (
                  SELECT
                    JSONB_BUILD_OBJECT('phone_number_raw', "contact_phone_numbers"."phone_number_raw", 'id', "contact_phone_numbers"."id") AS elem
                  FROM "contact_phone_numbers"

                  WHERE ("contact_phone_numbers"."tombstone" = 0 AND "contact_phone_numbers"."organization_id" IS NOT NULL AND "contact_phone_numbers"."organization_id" = '01JBHKXHYSKPP247HZZWHA3JCT' AND "contacts"."id" = "contact_phone_numbers"."contact_id")
                ) sub
            ), '[]') as "contact_phone_numbers", COALESCE(
              (
                SELECT JSONB_AGG(elem)
                FROM (
                  SELECT
                    JSONB_BUILD_OBJECT('id', "created_by_account_organizations"."id", 'contact_id', "created_by_account_organizations"."contact_id") AS elem
                  FROM "account_organizations" "created_by_account_organizations"

                  WHERE ("created_by_account_organizations"."tombstone" = 0 AND "created_by_account_organizations"."organization_id" IS NOT NULL AND "created_by_account_organizations"."organization_id" = '01JBHKXHYSKPP247HZZWHA3JCT' AND "contacts"."created_by" = "created_by_account_organizations"."id")
                ) sub
            ), '[]') as "created_by_account_organizations", COALESCE(
              (
                SELECT JSONB_AGG(elem)
                FROM (
                  SELECT
                    JSONB_BUILD_OBJECT('id', "updated_by_account_organizations"."id", 'contact_id', "updated_by_account_organizations"."contact_id") AS elem
                  FROM "account_organizations" "updated_by_account_organizations"

                  WHERE ("updated_by_account_organizations"."tombstone" = 0 AND "updated_by_account_organizations"."organization_id" IS NOT NULL AND "updated_by_account_organizations"."organization_id" = '01JBHKXHYSKPP247HZZWHA3JCT' AND "contacts"."updated_by" = "updated_by_account_organizations"."id")
                ) sub
            ), '[]') as "updated_by_account_organizations", JSONB_AGG("contact_phone_numbers"."phone_number_raw") as "contact_phone_numbers_phone_number_raws", JSONB_AGG("contact_emails"."email") as "contact_emails_emails", JSONB_AGG("contact_emails"."is_primary") as "contact_emails_is_primaries" from "contacts" left join 
            LATERAL (
              SELECT "joined_contact_emails"."email", "joined_contact_emails"."is_primary", "joined_contact_emails"."id"
              from "contact_emails" "joined_contact_emails" where ("joined_contact_emails"."tombstone" = 0 and "joined_contact_emails"."organization_id" is not null and "joined_contact_emails"."organization_id" = '01JBHKXHYSKPP247HZZWHA3JCT') AND "contacts"."id" = "joined_contact_emails"."contact_id"


            ) AS "contact_emails"
           on TRUE left join 
            LATERAL (
              SELECT "joined_contact_phone_numbers"."phone_number_raw", "joined_contact_phone_numbers"."id"
              from "contact_phone_numbers" "joined_contact_phone_numbers" where ("joined_contact_phone_numbers"."tombstone" = 0 and "joined_contact_phone_numbers"."organization_id" is not null and "joined_contact_phone_numbers"."organization_id" = '01JBHKXHYSKPP247HZZWHA3JCT') AND "contacts"."id" = "joined_contact_phone_numbers"."contact_id"


            ) AS "contact_phone_numbers"
           on TRUE left join 
            LATERAL (
              SELECT "joined_created_by_account_organizations"."id", "joined_created_by_account_organizations"."contact_id"
              from "account_organizations" "joined_created_by_account_organizations" where ("joined_created_by_account_organizations"."tombstone" = 0 and "joined_created_by_account_organizations"."organization_id" is not null and "joined_created_by_account_organizations"."organization_id" = '01JBHKXHYSKPP247HZZWHA3JCT') AND "contacts"."created_by" = "joined_created_by_account_organizations"."id"


            ) AS "created_by_account_organizations"
           on TRUE left join 
            LATERAL (
              SELECT "joined_created_by"."id", "joined_created_by"."first_name", "joined_created_by"."last_name"
              from "contacts" "joined_created_by" where ("joined_created_by"."tombstone" = 0 and "joined_created_by"."organization_id" is not null and "joined_created_by"."organization_id" = '01JBHKXHYSKPP247HZZWHA3JCT' and "joined_created_by"."id" = "created_by_account_organizations"."contact_id") 


            ) AS "created_by"
           on TRUE left join 
            LATERAL (
              SELECT "joined_updated_by_account_organizations"."id", "joined_updated_by_account_organizations"."contact_id"
              from "account_organizations" "joined_updated_by_account_organizations" where ("joined_updated_by_account_organizations"."tombstone" = 0 and "joined_updated_by_account_organizations"."organization_id" is not null and "joined_updated_by_account_organizations"."organization_id" = '01JBHKXHYSKPP247HZZWHA3JCT') AND "contacts"."updated_by" = "joined_updated_by_account_organizations"."id"


            ) AS "updated_by_account_organizations"
           on TRUE left join 
            LATERAL (
              SELECT "joined_updated_by"."id", "joined_updated_by"."first_name", "joined_updated_by"."last_name"
              from "contacts" "joined_updated_by" where ("joined_updated_by"."tombstone" = 0 and "joined_updated_by"."organization_id" is not null and "joined_updated_by"."organization_id" = '01JBHKXHYSKPP247HZZWHA3JCT' and "joined_updated_by"."id" = "updated_by_account_organizations"."contact_id") 


            ) AS "updated_by"
           on TRUE where ("contacts"."tombstone" = $1 and "contacts"."organization_id" is not null and "contacts"."organization_id" = $2 and ((COALESCE(to_char((("contacts"."created_date"::timestamp + "contacts"."created_time"::interval))::date, 'mm/dd/YYYY'), '') || ' ' || COALESCE((("contacts"."created_date"::timestamp + "contacts"."created_time"::interval))::time::text, '')) ilike $3 and ("contacts"."status" = $4 or "contacts"."status" = $5))) group by "contacts"."id" order by lower("contacts"."status") asc limit $6