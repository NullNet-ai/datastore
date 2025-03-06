#[macro_export]
macro_rules! system_fields {
    () => {
        id -> Text,
        categories -> Nullable<Array<Nullable<Text>>>,
        code -> Nullable<Text>,
        tombstone -> Nullable<Int4>,
        status -> Nullable<Text>,
        previous_status -> Nullable<Text>,
        version -> Nullable<Int4>,
        created_date -> Nullable<Text>,
        created_time -> Nullable<Text>,
        updated_date -> Nullable<Text>,
        updated_time -> Nullable<Text>,
        organization_id -> Nullable<Text>,
        created_by -> Nullable<Text>,
        updated_by -> Nullable<Text>,
        deleted_by -> Nullable<Text>,
        requested_by -> Nullable<Text>,
        tags -> Nullable<Array<Nullable<Text>>>,
    };
}