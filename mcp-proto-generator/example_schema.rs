// Example Diesel schema for testing the MCP proto generator
// You can use this schema to test the proto generation

use diesel::table;

table! {
    users (id) {
        id -> Integer,
        email -> Text,
        username -> Text,
        password_hash -> Text,
        first_name -> Nullable<Text>,
        last_name -> Nullable<Text>,
        avatar_url -> Nullable<Text>,
        is_active -> Bool,
        is_verified -> Bool,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    organizations (id) {
        id -> Integer,
        name -> Text,
        slug -> Text,
        description -> Nullable<Text>,
        website_url -> Nullable<Text>,
        logo_url -> Nullable<Text>,
        is_active -> Bool,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    organization_members (id) {
        id -> Integer,
        user_id -> Integer,
        organization_id -> Integer,
        role -> Text,
        joined_at -> Timestamp,
    }
}

table! {
    projects (id) {
        id -> Integer,
        organization_id -> Integer,
        name -> Text,
        description -> Nullable<Text>,
        repository_url -> Nullable<Text>,
        is_public -> Bool,
        is_archived -> Bool,
        created_by -> Integer,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    tasks (id) {
        id -> Integer,
        project_id -> Integer,
        title -> Text,
        description -> Nullable<Text>,
        status -> Text,
        priority -> Text,
        assignee_id -> Nullable<Integer>,
        reporter_id -> Integer,
        due_date -> Nullable<Timestamp>,
        completed_at -> Nullable<Timestamp>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    comments (id) {
        id -> Integer,
        task_id -> Integer,
        author_id -> Integer,
        content -> Text,
        is_edited -> Bool,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}