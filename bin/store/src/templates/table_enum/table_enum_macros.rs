#[macro_export]
macro_rules! generate_hypertable_timestamp_match {
    ($self:expr, $conn:expr, $id:expr, $($table:ident),*) => {
        paste::paste! {
            match $self {
                $(
                    Table::$table => {
                        let result = schema::[<$table:snake:lower>]::dsl::[<$table:snake:lower>]
                            .filter(schema::[<$table:snake:lower>]::id.eq($id))
                            .select(schema::[<$table:snake:lower>]::hypertable_timestamp)
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

#[macro_export]
macro_rules! generate_insert_record_match {
    ($self:expr, $auth:expr, $conn:expr, $record:expr, $request:expr, $($table:ident, $model:ty),*) => {
        paste::paste! {
            {
                let mut request = $request.into_inner();
                request.process_record("create", $auth);
                // ! needs refactoring for hypertable_timestamp

                match $self {
                    $(
                        Table::$table => {
                            let mut value: $model = serde_json::from_value($record)
                                .map_err(|e| DieselError::DeserializationError(Box::new(e)))?;

                            // Set hypertable_timestamp if the model has a timestamp field
                            // if field_exists_in_table(stringify!([<$table:lower>]), "hypertable_timestamp") {
                            //     value.hypertable_timestamp = Some(value.timestamp.to_string());
                            // }

                            diesel::insert_into(schema::[<$table:snake:lower>]::dsl::[<$table:snake:lower>]::table())
                                .values(value.clone())
                                .execute($conn)
                                .await?;

                            Ok(serde_json::to_string(&value).unwrap_or_else(|_| "{}".to_string()))
                        },
                    )*
                    _ => {
                        log::error!(
                            "Inserting record for table {:?} is not implemented",
                            $self
                        );
                        Err(DieselError::RollbackTransaction)
                    }
                }
            }
        }
    }
}

#[macro_export]
macro_rules! generate_get_by_id_match {
    ($self:expr, $conn:expr, $id:expr, $($table:ident, $model:ty),*) => {
        paste::paste! {
            match $self {
                $(
                    Table::$table => {
                        let result = schema::[<$table:snake:lower>]::dsl::[<$table:snake:lower>]
                            .filter(schema::[<$table:snake:lower>]::id.eq($id))
                            .filter(schema::[<$table:snake:lower>]::tombstone.eq(0))
                            .select(schema::[<$table:snake:lower>]::all_columns)
                            .first::<$model>($conn)
                            .await
                            .optional()?;

                        Ok(result.map(|record| serde_json::to_value(record).unwrap_or_default()))
                    },
                )*
                _ => {
                    log::error!(
                        "Getting record by id for table {:?} is not implemented",
                        $self
                    );
                    Err(DieselError::RollbackTransaction)
                }
            }
        }
    }
}

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
                            let value: $model = serde_json::from_value($record).map_err(|e| {
                                log::error!("Deserialization error: {:?}", e);
                                DieselError::DeserializationError(Box::new(e))
                            })?;

                            if has_version {
                                diesel::insert_into(schema::[<$table:snake:lower>]::dsl::[<$table:snake:lower>]::table())
                                    .values(value.clone())
                                    .on_conflict((schema::[<$table:snake:lower>]::id))
                                    .do_update()
                                    .set(schema::[<$table:snake:lower>]::version.eq(schema::[<$table:snake:lower>]::version + 1))
                                    .execute($conn)
                                    .await
                                    .map(|_| ())
                            } else if (has_status){
                                diesel::insert_into(schema::[<$table:snake:lower>]::dsl::[<$table:snake:lower>]::table())
                                .values(value.clone())
                                .on_conflict((schema::[<$table:snake:lower>]::id))
                                .do_update()
                                .set((
                                schema::[<$table:snake:lower>]::previous_status.eq(schema::[<$table:snake:lower>]::status),
                                schema::[<$table:snake:lower>]::status.eq(value.status.clone()),
                                ))
                                .execute($conn)
                                .await
                                .map(|_| ())
                            } else {
                                diesel::insert_into(schema::[<$table:snake:lower>]::dsl::[<$table:snake:lower>]::table())
                                    .values(value.clone())
                                    .on_conflict((schema::[<$table:snake:lower>]::id))
                                    .do_update()
                                    .set(value)
                                    .execute($conn)
                                    .await
                                    .map(|_| ())
                            }
                        },
                    )*
                    _ => {
                        log::error!(
                            "Upserting record with id for table {:?} is not implemented",
                            $self
                        );
                        Err(DieselError::RollbackTransaction)
                    }
                }
            }
        }
    }
}
#[macro_export]
macro_rules! generate_upsert_record_with_timestamp_match {
    ($self:expr, $conn:expr, $record:expr, $($table:ident, $model:ty),*) => {
        paste::paste! {
            {
                let has_version = $record.get("version").is_some();
                let has_status = $record.get("status").is_some();
                match $self {
                    $(
                        Table::$table => {
                            let value: $model = serde_json::from_value($record).map_err(|e| {
                                log::error!("Deserialization error: {:?}", e);
                                log::error!("Failed to deserialize record:");
                                DieselError::DeserializationError(Box::new(e))
                            })?;

                            if has_version {
                                diesel::insert_into(schema::[<$table:snake:lower>]::dsl::[<$table:snake:lower>]::table())
                                    .values(value.clone())
                                    .on_conflict((schema::[<$table:snake:lower>]::id, schema::[<$table:snake:lower>]::timestamp))
                                    .do_update()
                                    .set(schema::[<$table:snake:lower>]::version.eq(schema::[<$table:snake:lower>]::version + 1))
                                    .execute($conn)
                                    .await
                                    .map(|_| ())
                            }else if (has_status){
                                diesel::insert_into(schema::[<$table:snake:lower>]::dsl::[<$table:snake:lower>]::table())
                                .values(value.clone())
                                .on_conflict((schema::[<$table:snake:lower>]::id, schema::[<$table:snake:lower>]::timestamp))
                                .do_update()
                                .set((
                                schema::[<$table:snake:lower>]::previous_status.eq(schema::[<$table:snake:lower>]::status),
                                schema::[<$table:snake:lower>]::status.eq(value.status.clone()),
                                ))
                                .execute($conn)
                                .await
                                .map(|_| ())
                            } else {
                                diesel::insert_into(schema::[<$table:snake:lower>]::dsl::[<$table:snake:lower>]::table())
                                    .values(value.clone())
                                    .on_conflict((schema::[<$table:snake:lower>]::id, schema::[<$table:snake:lower>]::timestamp))
                                    .do_update()
                                    .set(value)
                                    .execute($conn)
                                    .await
                                    .map(|_| ())
                            }
                        },
                    )*
                    _ => {
                        log::error!(
                            "Upserting record with id and timestamp for table {:?} is not implemented",
                            $self
                        );
                        Err(DieselError::RollbackTransaction)
                    }
                }
            }
        }
    }
}
