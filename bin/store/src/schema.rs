diesel::table! {
    items (id) {
        id -> Int4,
        name -> Varchar,
        description -> Nullable<Text>,
    }
}
