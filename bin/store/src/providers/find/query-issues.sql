SELECT    "contacts"."code", 
          "contacts"."categories", 
          "contacts"."organization_id", 
          "contacts"."first_name", 
          "contacts"."middle_name", 
          "contacts"."last_name", 
          "contacts"."status", 
          COALESCE(To_char("contacts"."created_date"::date, 'mm/dd/YYYY'), ''), 
          COALESCE(To_char("contacts"."updated_date"::date, 'mm/dd/YYYY'), ''), 
          ("contacts"."created_time" AT TIME ZONE 'Asia/Manila')::time, 
          ("contacts"."updated_time" AT TIME ZONE 'Asia/Manila')::time, 
          "contacts"."created_by", 
          "contacts"."updated_by", 
          "contacts"."previous_status", 
          COALESCE( 
                    ( 
                    SELECT   jsonb_agg(jsonb_build_object('id', "created_by_account_organizations"."id", 'contact_id', "created_by_account_organizations"."contact_id") ORDER BY lower("created_by_account_organizations"."status") ASC)
                    FROM     "account_organizations" "created_by_account_organizations" 
                    WHERE    ( 
                                      created_by_account_organizations.tombstone = 0 
                             AND      created_by_account_organizations.organization_id IS NOT NULL
                             AND      created_by_account_organizations.organization_id = '01JBHKXHYSKPP247HZZWHA3JCT')
                    AND      "contacts"."created_by" = "created_by_account_organizations"."id"), '[]') AS "created_by_account_organizations",
          COALESCE( 
                    ( 
                    SELECT    jsonb_agg(jsonb_build_object('id', "created_by"."id", 'first_name', "created_by"."first_name", 'last_name', "created_by"."last_name", 'full_name', ("created_by"."first_name"
                                        || ' ' 
                                        || "created_by"."last_name")) ORDER BY lower("created_by"."status") ASC)
                    FROM      "account_organizations" "created_by_account_organizations" 
                    LEFT JOIN "contacts" "created_by" 
                    ON        "created_by_account_organizations"."contact_id" = "created_by"."id" 
                    WHERE     ( 
                                        created_by.tombstone = 0 
                              AND       created_by.organization_id IS NOT NULL 
                              AND       created_by.organization_id = '01JBHKXHYSKPP247HZZWHA3JCT')
                    AND       "contacts"."created_by" = "created_by_account_organizations"."id"), '[]') AS "created_by",
          COALESCE( 
                    ( 
                    SELECT   jsonb_agg(jsonb_build_object('id', "updated_by_account_organizations"."id", 'contact_id', "updated_by_account_organizations"."contact_id") ORDER BY lower("updated_by_account_organizations"."status") ASC)
                    FROM     "account_organizations" "updated_by_account_organizations" 
                    WHERE    ( 
                                      updated_by_account_organizations.tombstone = 0 
                             AND      updated_by_account_organizations.organization_id IS NOT NULL
                             AND      updated_by_account_organizations.organization_id = '01JBHKXHYSKPP247HZZWHA3JCT')
                    AND      "contacts"."updated_by" = "updated_by_account_organizations"."id"), '[]') AS "updated_by_account_organizations",
          COALESCE( 
                    ( 
                    SELECT    jsonb_agg(jsonb_build_object('id', "updated_by"."id", 'first_name', "updated_by"."first_name", 'last_name', "updated_by"."last_name", 'full_name', ("updated_by"."first_name"
                                        || ' ' 
                                        || "updated_by"."last_name")) ORDER BY lower("updated_by"."status") ASC)
                    FROM      "account_organizations" "updated_by_account_organizations" 
                    LEFT JOIN "contacts" "updated_by" 
                    ON        "updated_by_account_organizations"."contact_id" = "updated_by"."id" 
                    WHERE     ( 
                                        updated_by.tombstone = 0 
                              AND       updated_by.organization_id IS NOT NULL 
                              AND       updated_by.organization_id = '01JBHKXHYSKPP247HZZWHA3JCT')
                    AND       "contacts"."updated_by" = "updated_by_account_organizations"."id"), '[]') AS "updated_by"
FROM      contacts 
LEFT JOIN lateral 
          ( 
                 SELECT "joined_created_by_account_organizations"."id", 
                        "joined_created_by_account_organizations"."contact_id" 
                 FROM   "account_organizations" "joined_created_by_account_organizations" 
                 WHERE  ( 
                               joined_created_by_account_organizations.tombstone = 0 
                        AND    joined_created_by_account_organizations.organization_id IS NOT NULL
                        AND    joined_created_by_account_organizations.organization_id = '01JBHKXHYSKPP247HZZWHA3JCT')
                 AND    "contacts"."created_by" = "joined_created_by_account_organizations"."id" ) AS "created_by_account_organizations"
ON        true 
LEFT JOIN lateral 
          ( 
                 SELECT "joined_created_by"."id", 
                        "joined_created_by"."first_name", 
                        "joined_created_by"."last_name" 
                 FROM   "contacts" "joined_created_by" 
                 WHERE  ( 
                               joined_created_by.tombstone = 0 
                        AND    joined_created_by.organization_id IS NOT NULL 
                        AND    joined_created_by.organization_id = '01JBHKXHYSKPP247HZZWHA3JCT') 
                 AND    "joined_created_by"."id" = "created_by_account_organizations"."contact_id" ) AS "created_by"
ON        true 
LEFT JOIN lateral 
          ( 
                 SELECT "joined_updated_by_account_organizations"."id", 
                        "joined_updated_by_account_organizations"."contact_id" 
                 FROM   "account_organizations" "joined_updated_by_account_organizations" 
                 WHERE  ( 
                               joined_updated_by_account_organizations.tombstone = 0 
                        AND    joined_updated_by_account_organizations.organization_id IS NOT NULL
                        AND    joined_updated_by_account_organizations.organization_id = '01JBHKXHYSKPP247HZZWHA3JCT')
                 AND    "contacts"."updated_by" = "joined_updated_by_account_organizations"."id" ) AS "updated_by_account_organizations"
ON        true 
LEFT JOIN lateral 
          ( 
                 SELECT "joined_updated_by"."id", 
                        "joined_updated_by"."first_name", 
                        "joined_updated_by"."last_name" 
                 FROM   "contacts" "joined_updated_by" 
                 WHERE  ( 
                               joined_updated_by.tombstone = 0 
                        AND    joined_updated_by.organization_id IS NOT NULL 
                        AND    joined_updated_by.organization_id = '01JBHKXHYSKPP247HZZWHA3JCT') 
                 AND    "joined_updated_by"."id" = "updated_by_account_organizations"."contact_id" ) AS "updated_by"
ON        true 
WHERE     ( 
                    contacts.tombstone = 0 
          AND       contacts.organization_id IS NOT NULL 
          AND       contacts.organization_id = '01JBHKXHYSKPP247HZZWHA3JCT') 
AND       "created_by"."full_name" = 'Super Admin' 
AND       "contacts"."status" = 'Active' 
ORDER BY  lower("contacts"."status") ASC limit 100