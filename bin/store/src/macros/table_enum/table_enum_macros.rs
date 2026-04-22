#[allow(warnings)]
#[macro_export]
macro_rules! generate_hypertable_timestamp_match {
    ($self:expr, $conn:expr, $id:expr, $($table:ident),*) => {
        paste::paste! {
            match $self {
                $(
                    Table::$table => {
                        let result = crate::generated::schema::[<$table:snake:lower>]::dsl::[<$table:snake:lower>]
                            .filter(crate::generated::schema::[<$table:snake:lower>]::id.eq($id))
                            .select(crate::generated::schema::[<$table:snake:lower>]::hypertable_timestamp)
                            .first::<Option<String>>($conn)
                            .await;

                        result
                    },
                )*
                _ => {
                    log::error!(
                        "Getting hypertable_timestamp for table {:?} is not implemented",
                        $self
                    );
                    Err(DieselError::RollbackTransaction)
                }
            }
        }
    }
}
#[allow(warnings)]
#[allow(unreachable_patterns)]
#[macro_export]
macro_rules! generate_insert_record_match {
    ($self:expr, $auth:expr, $conn:expr, $record:expr, $request:expr, $($table:ident, $model:ty),*) => {
        paste::paste! {
            {
                let mut request = $request.into_inner();
                // ! needs refactoring for hypertable_timestamp

                match $self {
                    $(
                        Table::$table => {
                            request.process_record("create", $auth, false, stringify!([<$table:snake:lower>]));
                            let value: $model = serde_json::from_value($record.clone()).map_err(|e| {
                                log::error!("Failed to deserialize record for table {}: {}", stringify!($table), serde_json::to_string_pretty(&$record).unwrap_or_else(|_| "<invalid json>".to_string()));
                                DieselError::DeserializationError(Box::new(e))
                            })?;

                            // Set hypertable_timestamp if the model has a timestamp field
                            // if field_exists_in_table(stringify!([<$table:lower>]), "hypertable_timestamp") {
                            //     value.hypertable_timestamp = Some(value.timestamp.to_string());
                            // }

                            diesel::insert_into(crate::generated::schema::[<$table:snake:lower>]::dsl::[<$table:snake:lower>]::table())
                                .values(value.clone())
                                .execute($conn)
                                .await?;

                            Ok(serde_json::to_string(&value).unwrap_or_else(|_| "{}".to_string()))
                        },
                    )*
                }
            }
        }
    }
}

#[allow(warnings)]
#[allow(unreachable_patterns)]
#[macro_export]
macro_rules! generate_get_by_id_match {
    ($self:expr, $conn:expr, $id:expr, $is_root_account:expr, $organization_id:expr, $($table:ident, $model:ty),*) => {
        paste::paste! {
            match $self {
                $(
                    Table::$table => {
                        let migration_mode = std::env::var("MIGRATION_MODE")
                            .ok()
                            .map(|v| v.trim().eq_ignore_ascii_case("true") || v.trim() == "1")
                            .unwrap_or(false);
                        let table_name = stringify!([<$table:snake:lower>]);
                        let id_escaped = $id.replace("'", "''");
                        let mut clauses: Vec<String> = vec![format!("id = '{}'", id_escaped)];
                        if !migration_mode {
                            clauses.push("tombstone = 0".to_string());
                            if !$is_root_account {
                                if let Some(org_id) = $organization_id.clone() {
                                    let org_escaped = org_id.replace("'", "''");
                                    clauses.push("organization_id IS NOT NULL".to_string());
                                    clauses.push(format!("organization_id = '{}'", org_escaped));
                                }
                            }
                        }
                        let where_sql = clauses.join(" AND ");
                        let select_sql = format!(
                            "SELECT row_to_json(t) FROM (SELECT * FROM {} WHERE {} LIMIT 1) t",
                            table_name, where_sql
                        );
                        let rows = diesel::dsl::sql_query(select_sql)
                            .load::<crate::providers::queries::find::queries::DynamicResult>($conn)
                            .await?;
                        let value = rows
                            .get(0)
                            .and_then(|r| r.row_to_json.clone());
                        Ok(value)
                    },
                )*
            }
        }
    }
}

#[allow(warnings)]
#[allow(unreachable_patterns)]
#[macro_export]
macro_rules! generate_upsert_record_match {
    ($self:expr, $conn:expr, $record:expr, $($table:ident, $model:ty),*) => {
        paste::paste! {
            {
                let has_version = $record.get("version").is_some();
                let has_status = $record.get("status").is_some();

                match $self {
                    $(
                        Table::$table => {
                            let value: $model = serde_json::from_value($record.clone()).map_err(|e| {
                                log::error!("Failed to deserialize record for table {}: {}", stringify!($table), serde_json::to_string_pretty(&$record).unwrap_or_else(|_| "<invalid json>".to_string()));
                                DieselError::DeserializationError(Box::new(e))
                            })?;

                            if has_version {
                                diesel::insert_into(crate::generated::schema::[<$table:snake:lower>]::dsl::[<$table:snake:lower>]::table())
                                    .values(value.clone())
                                    .on_conflict((crate::generated::schema::[<$table:snake:lower>]::id))
                                    .do_update()
                                    .set(crate::generated::schema::[<$table:snake:lower>]::version.eq(crate::generated::schema::[<$table:snake:lower>]::version + 1))
                                    .execute($conn)
                                    .await
                                    .map(|_| ())
                            } else if (has_status){
                                diesel::insert_into(crate::generated::schema::[<$table:snake:lower>]::dsl::[<$table:snake:lower>]::table())
                                .values(value.clone())
                                .on_conflict((crate::generated::schema::[<$table:snake:lower>]::id))
                                .do_update()
                                .set((
                                crate::generated::schema::[<$table:snake:lower>]::previous_status.eq(crate::generated::schema::[<$table:snake:lower>]::status),
                                crate::generated::schema::[<$table:snake:lower>]::status.eq(value.status.clone()),
                                ))
                                .execute($conn)
                                .await
                                .map(|_| ())
                            } else {
                                diesel::insert_into(crate::generated::schema::[<$table:snake:lower>]::dsl::[<$table:snake:lower>]::table())
                                    .values(value.clone())
                                    .on_conflict((crate::generated::schema::[<$table:snake:lower>]::id))
                                    .do_update()
                                    .set(value)
                                    .execute($conn)
                                    .await
                                    .map(|_| ())
                            }
                        },
                    )*
                }
            }
        }
    }
}

/// Migration-mode upsert: always does .set(value) on conflict, no version/status special branches.
#[macro_export]
#[allow(warnings)]
#[allow(unreachable_patterns)]
macro_rules! generate_upsert_record_migration_match {
    ($self:expr, $conn:expr, $record:expr, $($table:ident, $model:ty),*) => {
        paste::paste! {
            {
                match $self {
                    $(
                        Table::$table => {
                            let value: $model = serde_json::from_value($record.clone()).map_err(|e| {
                                log::error!("Failed to deserialize record for table {}: {}", stringify!($table), serde_json::to_string_pretty(&$record).unwrap_or_else(|_| "<invalid json>".to_string()));
                                DieselError::DeserializationError(Box::new(e))
                            })?;

                            diesel::insert_into(crate::generated::schema::[<$table:snake:lower>]::dsl::[<$table:snake:lower>]::table())
                                .values(value.clone())
                                .on_conflict((crate::generated::schema::[<$table:snake:lower>]::id))
                                .do_update()
                                .set(value)
                                .execute($conn)
                                .await
                                .map(|_| ())
                        },
                    )*
                }
            }
        }
    }
}

/// Migration-mode upsert with timestamp conflict: always does .set(value), no version/status branches.
#[macro_export]
#[allow(warnings)]
#[allow(unreachable_patterns)]
macro_rules! generate_upsert_record_migration_with_timestamp_match {
    ($self:expr, $conn:expr, $record:expr, $($table:ident, $model:ty),*) => {
        paste::paste! {
            {
                match $self {
                    $(
                        Table::$table => {
                            let value: $model = serde_json::from_value($record.clone()).map_err(|e| {
                                log::error!("Failed to deserialize record for table {}: {}", stringify!($table), serde_json::to_string_pretty(&$record).unwrap_or_else(|_| "<invalid json>".to_string()));
                                DieselError::DeserializationError(Box::new(e))
                            })?;

                            diesel::insert_into(crate::generated::schema::[<$table:snake:lower>]::dsl::[<$table:snake:lower>]::table())
                                .values(value.clone())
                                .on_conflict((crate::generated::schema::[<$table:snake:lower>]::id, crate::generated::schema::[<$table:snake:lower>]::timestamp))
                                .do_update()
                                .set(value)
                                .execute($conn)
                                .await
                                .map(|_| ())
                        },
                    )*
                }
            }
        }
    }
}

#[macro_export]
#[allow(warnings)]
#[allow(unreachable_patterns)]
macro_rules! generate_upsert_record_with_timestamp_match {
    ($self:expr, $conn:expr, $record:expr, $($table:ident, $model:ty),*) => {
        paste::paste! {
            {
                let has_version = $record.get("version").is_some();
                let has_status = $record.get("status").is_some();
                match $self {
                    $(
                        Table::$table => {
                            let value: $model = serde_json::from_value($record.clone()).map_err(|e| {
                                log::error!("Deserialization error: {:?}", e);
                                log::error!("Failed to deserialize record for table {}: {}", stringify!($table), serde_json::to_string_pretty(&$record).unwrap_or_else(|_| "<invalid json>".to_string()));
                                DieselError::DeserializationError(Box::new(e))
                            })?;

                            if has_version {
                                diesel::insert_into(crate::generated::schema::[<$table:snake:lower>]::dsl::[<$table:snake:lower>]::table())
                                    .values(value.clone())
                                    .on_conflict((crate::generated::schema::[<$table:snake:lower>]::id, crate::generated::schema::[<$table:snake:lower>]::timestamp))
                                    .do_update()
                                    .set(crate::generated::schema::[<$table:snake:lower>]::version.eq(crate::generated::schema::[<$table:snake:lower>]::version + 1))
                                    .execute($conn)
                                    .await
                                    .map(|_| ())
                            }else if (has_status){
                                diesel::insert_into(crate::generated::schema::[<$table:snake:lower>]::dsl::[<$table:snake:lower>]::table())
                                .values(value.clone())
                                .on_conflict((crate::generated::schema::[<$table:snake:lower>]::id, crate::generated::schema::[<$table:snake:lower>]::timestamp))
                                .do_update()
                                .set((
                                crate::generated::schema::[<$table:snake:lower>]::previous_status.eq(crate::generated::schema::[<$table:snake:lower>]::status),
                                crate::generated::schema::[<$table:snake:lower>]::status.eq(value.status.clone()),
                                ))
                                .execute($conn)
                                .await
                                .map(|_| ())
                            } else {
                                diesel::insert_into(crate::generated::schema::[<$table:snake:lower>]::dsl::[<$table:snake:lower>]::table())
                                    .values(value.clone())
                                    .on_conflict((crate::generated::schema::[<$table:snake:lower>]::id, crate::generated::schema::[<$table:snake:lower>]::timestamp))
                                    .do_update()
                                    .set(value)
                                    .execute($conn)
                                    .await
                                    .map(|_| ())
                            }
                        },
                    )*
                }
            }
        }
    }
}
