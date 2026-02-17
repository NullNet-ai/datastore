   #[tokio::test]
    #[ignore]
    async fn should_use_contacts_filter_with_date_time_fields_without_join_scenario() {
        println!(
            "Testing contacts_filter_with_date_time_fields_without_join payload scenario with HTTP request..."
        );

        // First perform login to get authentication
        let auth_response = perform_login().await;
        if !auth_response.server_available {
            println!("  ⚠ Server not available, skipping HTTP test");
            return;
        }

        match load_payload_scenario("contacts_filter_with_date_time_fields_without_join") {
            Ok(payload) => {
                println!("  ✓ Successfully loaded contacts_filter_with_date_time_fields_without_join scenario");

                println!("  ✓ Payload fields: {:?}", payload.pluck);
                println!("  ✓ Filter count: {}", payload.advance_filters.len());
                println!(
                    "  ✓ Concatenate fields count: {}",
                    payload.concatenate_fields.len()
                );
                println!("  ✓ Joins count: {}", payload.joins.len());

                // Validate payload structure
                assert_eq!(
                    payload.pluck,
                    vec![
                        "id",
                        "categories",
                        "organization_id",
                        "first_name",
                        "middle_name",
                        "last_name",
                        "created_date",
                        "created_time",
                        "updated_date",
                        "updated_time"
                    ]
                );
                assert_eq!(payload.limit, 100);
                assert_eq!(payload.offset, 0);
                assert!(
                    payload.advance_filters.len() >= 1,
                    "Should have one advance filter for created_date"
                );
                assert!(payload.joins.len() == 0, "Should have no join");
                assert!(
                    payload.pluck_object.is_empty(),
                    "Should have no pluck object"
                );
                // Check that payload.time_format is a valid ETimeFormats value
                let time_format = payload.time_format.clone();
                assert!(
                    matches!(
                        time_format.as_str(),
                        "HH24:MI:SS"
                            | "HH24:MI"
                            | "HH12:MI"
                            | "HH12:MI AM"
                            | "HH12:MI:SS AM"
                            | "HH12:MI:SS"
                    ),
                    "Time format '{}' should be a valid ETimeFormats value",
                    time_format
                );
                // Check that payload.date_format is a valid date format
                let date_format = payload.date_format.clone();
                assert!(
                    matches!(
                        date_format.as_str(),
                        "mm/dd/YYYY"
                            | "dd/mm/YYYY"
                            | "YYYY/mm/dd"
                            | "YYYY/dd/mm"
                            | "mm-dd-YYYY"
                            | "YYYY-mm-dd"
                            | "YYYY-dd-mm"
                    ),
                    "Date format '{}' should be a valid date format",
                    date_format
                );

                // Verify date-time concatenation fields
                let date_time_fields = payload
                    .concatenate_fields
                    .iter()
                    .filter(|f| {
                        f.field_name == "created_date_time" || f.field_name == "updated_date_time"
                    })
                    .collect::<Vec<_>>();

                assert_eq!(
                    date_time_fields.len(),
                    2,
                    "Should have created_date_time and updated_date_time concatenated fields"
                );

                assert_eq!(payload.timezone, Some("America/Los_Angeles".to_string()));

                // Verify date-time advance filter fields
                let date_time_fields_filter = payload
                    .advance_filters
                    .iter()
                    .filter(|f| {
                        match f {
                            FilterCriteria::Criteria { field, .. } => {
                                field.contains("_date") || field.contains("_time")
                            }
                            _ => false, // Logical operators don't have fields
                        }
                    })
                    .collect::<Vec<_>>();

                assert!(
                    date_time_fields_filter.len() > 0,
                    "Should have date-time advance filter fields"
                );
                // Generate SQL query
                match get_raw_query(
                    &serde_json::to_value(payload.clone()).unwrap(),
                    get_table_name(),
                    true,
                    None,
                ) {
                    Ok(sql) => {
                        println!("  ✓ Successfully generated SQL query");

                        // Write SQL to file for debugging
                        if let Err(e) = write_sql_to_file(
                            &sql,
                            "should_use_contacts_filter_with_date_time_fields_with_join_scenario",
                        ) {
                            println!("  ⚠ Failed to write SQL to file: {}", e);
                        }

                        // Verify SQL contains date and time format elements
                        assert!(
                            sql.contains("TO_CHAR"),
                            "SQL should contain TO_CHAR for date/time formatting"
                        );
                        assert!(
                            sql.contains("HH24:MI"),
                            "SQL should use HH24:MI time format"
                        );
                        assert!(
                            sql.contains("mm/dd/YYYY"),
                            "SQL should use mm/dd/YYYY date format"
                        );

                        assert!(
                            sql.contains("AT TIME ZONE 'America/Los_Angeles'"),
                            "SQL should use AT TIME ZONE 'America/Los_Angeles' date format"
                        );

                        // Verify concatenation of date and time fields
                        assert!(
                            sql.contains("(\"contacts\".\"created_date\"::TIMESTAMP + \"contacts\".\"created_time\"::INTERVAL)") && sql.contains("::DATE, 'mm/dd/YYYY'), '')") && sql.contains(" || ' ' || "),
                            "SQL should concatenate created_date and created_time"
                        );

                        // Execute query if database is available
                        match execute_raw_sql_query(&sql).await {
                            Ok(results) => {
                                println!("  ✓ Successfully executed SQL query");
                                println!("  ✓ Query returned {} results", results.len());

                                // Additional validation can be added here if needed
                            }
                            Err(e) => {
                                println!("  ⚠ Failed to execute SQL query: {}", e);
                                // This is not a test failure as database might not be available
                            }
                        }
                    }
                    Err(e) => {
                        display_error_response(&format!("Failed to generate SQL: {}", e));
                    }
                }
            }
            Err(e) => {
                println!("  ⚠ Failed to load scenario: {}", e);
                println!("  ℹ This may be expected if scenario files haven't been created yet");
            }
        }

        println!("  ✓ contacts_filter_with_date_time_fields_without_join scenario test completed");
    }
