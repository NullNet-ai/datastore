select contacts.status, COUNT(*) from "contacts" left join
            LATERAL (
              SELECT "joined_contact_emails"."id", "joined_contact_emails"."email"
              from "contact_emails" "joined_contact_emails" where ("joined_contact_emails"."tombstone" = 0 and "joined_contact_emails"."organization_id" is not null and "joined_contact_emails"."organization_id" = '01JBHKXHYSKPP247HZZWHA3JCT') AND "contacts"."id" = "joined_contact_emails"."contact_id"


            ) AS "contact_emails"
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
           on TRUE where ("contacts"."tombstone" = 0 and "contacts"."organization_id" is not null and "contacts"."organization_id" = '01JBHKXHYSKPP247HZZWHA3JCT' and "contacts"."status" ilike 'Active%')
--------------------------------------------------------------------
select contacts.first_name, COUNT(*) from "contacts" left join
            LATERAL (
              SELECT "joined_contact_emails"."id", "joined_contact_emails"."email"
              from "contact_emails" "joined_contact_emails" where ("joined_contact_emails"."tombstone" = 0 and "joined_contact_emails"."organization_id" is not null and "joined_contact_emails"."organization_id" = '01JBHKXHYSKPP247HZZWHA3JCT') AND "contacts"."id" = "joined_contact_emails"."contact_id"


            ) AS "contact_emails"
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
           on TRUE where ("contacts"."tombstone" = 0 and "contacts"."organization_id" is not null and "contacts"."organization_id" = '01JBHKXHYSKPP247HZZWHA3JCT' and "contacts"."first_name" ilike 'Active%')
--------------------------------------------------------------------
SELECT JSON_BUILD_OBJECT('contacts', (SELECT JSON_BUILD_OBJECT(
                'status', (select JSON_OBJECT_AGG(COALESCE(status::TEXT, 'null'), count) from (select contacts.status, COUNT(*) from "contacts" left join
            LATERAL (
              SELECT "joined_contact_emails"."id", "joined_contact_emails"."email"
              from "contact_emails" "joined_contact_emails" where ("joined_contact_emails"."tombstone" = 0 and "joined_contact_emails"."organization_id" is not null and "joined_contact_emails"."organization_id" = '01JBHKXHYSKPP247HZZWHA3JCT') AND "contacts"."id" = "joined_contact_emails"."contact_id"


            ) AS "contact_emails"
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
           on TRUE where ("contacts"."tombstone" = 0 and "contacts"."organization_id" is not null and "contacts"."organization_id" = '01JBHKXHYSKPP247HZZWHA3JCT' and "contacts"."status" ilike 'Active%') GROUP BY contacts.status ORDER BY
                        CASE
                        WHEN contacts.status = 'Active' THEN 1
                        WHEN contacts.status ILIKE 'Active%' THEN 2
                        ELSE 3
                        END
                        OFFSET 0 LIMIT 100
                        ) AS status where status ilike 'Active%'),
                'first_name_group', (
                  SELECT COALESCE(
                    JSON_OBJECT_AGG('count', count),
                    JSON_BUILD_OBJECT('count', 0)
                  )
                  FROM (
                    select COUNT(*) OVER() from "contacts" left join
            LATERAL (
              SELECT "joined_contact_emails"."id", "joined_contact_emails"."email"
              from "contact_emails" "joined_contact_emails" where ("joined_contact_emails"."tombstone" = 0 and "joined_contact_emails"."organization_id" is not null and "joined_contact_emails"."organization_id" = '01JBHKXHYSKPP247HZZWHA3JCT') AND "contacts"."id" = "joined_contact_emails"."contact_id"


            ) AS "contact_emails"
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
           on TRUE where ("contacts"."tombstone" = 0 and "contacts"."organization_id" is not null and "contacts"."organization_id" = '01JBHKXHYSKPP247HZZWHA3JCT' and "contacts"."first_name" ilike 'Active%')
                    GROUP BY contacts.first_name
                  ) AS first_name_group
                ),
                'first_name', (select JSON_OBJECT_AGG(COALESCE(first_name::TEXT, 'null'), count) from (select contacts.first_name, COUNT(*) from "contacts" left join
            LATERAL (
              SELECT "joined_contact_emails"."id", "joined_contact_emails"."email"
              from "contact_emails" "joined_contact_emails" where ("joined_contact_emails"."tombstone" = 0 and "joined_contact_emails"."organization_id" is not null and "joined_contact_emails"."organization_id" = '01JBHKXHYSKPP247HZZWHA3JCT') AND "contacts"."id" = "joined_contact_emails"."contact_id"


            ) AS "contact_emails"
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
           on TRUE where ("contacts"."tombstone" = 0 and "contacts"."organization_id" is not null and "contacts"."organization_id" = '01JBHKXHYSKPP247HZZWHA3JCT' and "contacts"."first_name" ilike 'Active%') GROUP BY contacts.first_name ORDER BY
                        CASE
                        WHEN contacts.first_name = 'Active' THEN 1
                        WHEN contacts.first_name ILIKE 'Active%' THEN 2
                        ELSE 3
                        END
                        OFFSET 0 LIMIT 100
                        ) AS first_name where first_name ilike 'Active%')))) AS results