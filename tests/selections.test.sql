SELECT    "contacts"."id",
          "contacts"."first_name",
          "contacts"."last_name",
          (Coalesce(To_char("contacts"."created_date"::DATE, 'mm/dd/YYYY'), '')
                    || ' '
                    || Coalesce("contacts"."created_time", '')) AS created_date_time,
          (Coalesce(To_char("contacts"."updated_date"::DATE, 'mm/dd/YYYY'), '')
                    || ' '
                    || Coalesce("contacts"."updated_time", '')) AS updated_date_time,
          "contacts"."code",
          Coalesce(
                     (
                     SELECT Jsonb_agg(elem)
                     FROM   (
                                   SELECT Jsonb_build_object('id', "created_by_account_organizations"."id", 'contact_id', "created_by_account_organizations"."contact_id") AS elem
                                   FROM   "account_organizations" "created_by_account_organizations"
                                   WHERE  (
                                                 "created_by_account_organizations"."tombstone" = 0
                                          AND    "created_by_account_organizations"."organization_id" IS NOT NULL
                                          AND    "created_by_account_organizations"."organization_id" = '01JBHKXHYSKPP247HZZWHA3JCT'
                                          AND    "contacts"."created_by" = "created_by_account_organizations"."id") ) sub ), '[]') AS "created_by_account_organizations",
          Coalesce(
                     (
                     SELECT Jsonb_agg(elem)
                     FROM   (
                                      SELECT    Jsonb_build_object('id', "created_by"."id", 'first_name', "created_by"."first_name", 'last_name', "created_by"."last_name", 'full_name', (Coalesce("created_by"."first_name", '')
                                                          || ' '
                                                          || Coalesce("created_by"."last_name", ''))) AS elem
                                      FROM      "account_organizations" "created_by_account_organizations"
                                      left join "contacts" "created_by"
                                      ON        "created_by"."id" = "created_by_account_organizations"."contact_id"
                                      WHERE     (
                                                          "created_by"."tombstone" = 0
                                                AND       "created_by"."organization_id" IS NOT NULL
                                                AND       "created_by"."organization_id" = '01JBHKXHYSKPP247HZZWHA3JCT'
                                                AND       "created_by_account_organizations"."tombstone" = 0
                                                AND       "created_by_account_organizations"."organization_id" IS NOT NULL
                                                AND       "created_by_account_organizations"."organization_id" = '01JBHKXHYSKPP247HZZWHA3JCT'
                                                AND       "contacts"."created_by" = "created_by_account_organizations"."id") ) sub ), '[]') AS "created_by",
          Coalesce(
                     (
                     SELECT Jsonb_agg(elem)
                     FROM   (
                                   SELECT Jsonb_build_object('id', "updated_by_account_organizations"."id", 'contact_id', "updated_by_account_organizations"."contact_id") AS elem
                                   FROM   "account_organizations" "updated_by_account_organizations"
                                   WHERE  (
                                                 "updated_by_account_organizations"."tombstone" = 0
                                          AND    "updated_by_account_organizations"."organization_id" IS NOT NULL
                                          AND    "updated_by_account_organizations"."organization_id" = '01JBHKXHYSKPP247HZZWHA3JCT'
                                          AND    "contacts"."updated_by" = "updated_by_account_organizations"."id") ) sub ), '[]') AS "updated_by_account_organizations",
          Coalesce(
                     (
                     SELECT Jsonb_agg(elem)
                     FROM   (
                                      SELECT    Jsonb_build_object('id', "updated_by"."id", 'first_name', "updated_by"."first_name", 'last_name', "updated_by"."last_name", 'full_name', (Coalesce("updated_by"."first_name", '')
                                                          || ' '
                                                          || Coalesce("updated_by"."last_name", ''))) AS elem
                                      FROM      "account_organizations" "updated_by_account_organizations"
                                      left join "contacts" "updated_by"
                                      ON        "updated_by"."id" = "updated_by_account_organizations"."contact_id"
                                      WHERE     (
                                                          "updated_by"."tombstone" = 0
                                                AND       "updated_by"."organization_id" IS NOT NULL
                                                AND       "updated_by"."organization_id" = '01JBHKXHYSKPP247HZZWHA3JCT'
                                                AND       "updated_by_account_organizations"."tombstone" = 0
                                                AND       "updated_by_account_organizations"."organization_id" IS NOT NULL
                                                AND       "updated_by_account_organizations"."organization_id" = '01JBHKXHYSKPP247HZZWHA3JCT'
                                                AND       "contacts"."updated_by" = "updated_by_account_organizations"."id") ) sub ), '[]') AS "updated_by"
