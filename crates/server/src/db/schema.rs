// schema.rs

diesel::table! {
    use diesel::sql_types::*;
    // For Postgres, Diesel provides Timestamp, Int4, Varchar, Bool, etc.
    // For JSONB, you need the "postgres" feature of Diesel enabled.

    // 1) The `accounts` table
    accounts (id) {
        id -> Int4,
        account_id -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        is_active -> Bool,
    }
}

diesel::table! {
    // 2) The `access_keys` table
    access_keys (id) {
        id -> Int4,
        owner_id -> Int4,         // References accounts.id
        access_key -> Varchar,
        created_at -> Timestamp,
    }
}

diesel::table! {
    // 3) The `bucket_index` table
    bucket_index (id) {
        id -> Int4,
        account_id -> Int4,       // References accounts.id
        bucket_name -> Varchar,
        block_number -> Int4,
        created_at -> Timestamp,
    }
}

diesel::table! {
    // 4) The `object_index` table
    object_index (id) {
        id -> Int4,
        bucket_id -> Int4,        // References bucket_index.id
        object_key -> Varchar,
        block_number -> Int4,
        tx_hash -> Varchar,
        size_bytes -> Int4,
        created_at -> Timestamp,
        last_modified -> Timestamp,
        is_deleted -> Bool,
        // For JSONB columns in Diesel, ensure you have
        // the "serde_json" or "postgres" feature to map to/from JSONB.
        metadata -> diesel::pg::sql_types::Jsonb,
    }
}

diesel::table! {
    // 5) The `files` table
    files (id) {
        id -> Int4,
        created_at -> Timestamp,
        cid -> Varchar,
        size -> Int4,
        envelope_id -> Varchar,
        name -> Varchar,
        req_id -> Varchar,
    }
}

// -- If you want to specify the relationships (joinable!):
diesel::joinable!(access_keys -> accounts (owner_id));
diesel::joinable!(bucket_index -> accounts (account_id));
diesel::joinable!(object_index -> bucket_index (bucket_id));

// -- Let Diesel know all these tables can appear together in queries:
diesel::allow_tables_to_appear_in_same_query!(
    accounts,
    access_keys,
    bucket_index,
    object_index,
    files,
);
