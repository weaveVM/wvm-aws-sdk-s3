use crate::s3::account::{AccountId, AccountName};
use crate::s3::bucket::Bucket;
use crate::s3::object::Object;
use anyhow::{anyhow, Error};
use planetscale_driver::{query, PSConnection};
use std::sync::Arc;

pub async fn ps_list_buckets(
    conn: PSConnection,
    account_id: u64,
    max_keys: Option<i32>,
) -> Result<Vec<Bucket>, Error> {
    let mut query_str = format!(
        "SELECT id, account_id, bucket_name, tx_hash, block_number, created_at FROM bucket_index WHERE account_id = {}",
        account_id
    );

    if let Some(max_keys) = max_keys {
        query_str.push_str(&format!(" LIMIT {}", max_keys));
    }

    let buckets = query(&query_str).fetch_all(&conn).await?;

    Ok(buckets)
}

pub async fn ps_create_bucket(
    conn: PSConnection,
    account_id: u64,
    bucket_name: &str,
    tx_hash: &str,
    block_number: u64,
) -> Result<(), Error> {
    let query_str = format!(
        "INSERT INTO bucket_index(account_id, bucket_name, tx_hash, block_number) VALUES({}, \"{}\", \"{}\", {})",
        account_id, bucket_name, tx_hash, block_number
    );

    query(&query_str).execute(&conn).await?;

    Ok(())
}

pub async fn ps_delete_bucket(
    conn: PSConnection,
    account_id: u64,
    bucket_name: &str,
) -> Result<(), Error> {
    let query_str = format!(
        "DELETE FROM bucket_index WHERE account_id = {} AND bucket_name = \"{}\"",
        account_id, bucket_name
    );

    query(&query_str).execute(&conn).await?;

    Ok(())
}

pub async fn ps_list_objects(
    conn: PSConnection,
    bucket_id: u64,
    max_keys: Option<i32>,
) -> Result<Vec<Object>, Error> {
    let mut query_str = format!(
        "SELECT * FROM object_index WHERE bucket_id = {} AND is_deleted = false",
        bucket_id
    );

    if let Some(max_keys) = max_keys {
        query_str.push_str(&format!(" LIMIT {}", max_keys));
    }

    let objects: Vec<Object> = query(&query_str).fetch_all(&conn).await?;

    Ok(objects)
}

pub async fn ps_put_object(
    conn: PSConnection,
    bucket_id: u64,
    object_key: &str,
    tx_hash: &str,
    block_number: u64,
    size_bytes: u64,
    is_folder: bool,
    metadata: &str,
) -> Result<(), Error> {
    let query_str = format!(
        "INSERT INTO object_index(bucket_id, object_key, full_path, tx_hash, block_number, size_bytes, is_folder, metadata)
     SELECT {}, \"{}\", CONCAT(b.bucket_name, '/', \"{}\"), \"{}\", {}, {}, {}, JSON_OBJECT('metadata', CAST('{}' AS JSON))
     FROM bucket_index b
     WHERE b.id = {};",
        bucket_id, object_key, object_key, tx_hash, block_number, size_bytes, is_folder, metadata, bucket_id
    );

    query(&query_str).execute(&conn).await?;

    Ok(())
}

pub async fn ps_get_object(
    conn: PSConnection,
    bucket_id: u64,
    object_key: &str,
) -> Result<Object, Error> {
    let query_str = format!(
        "SELECT * FROM object_index WHERE bucket_id = {} AND object_key = \"{}\" AND is_deleted = false",
        bucket_id, object_key
    );

    let object: Object = query(&query_str).fetch_one(&conn).await?;

    Ok(object)
}

pub async fn ps_delete_object(
    conn: PSConnection,
    bucket_id: u64,
    object_key: &str,
) -> Result<(), Error> {
    let query_str = format!(
        "UPDATE object_index SET is_deleted = true WHERE bucket_id = {} AND object_key = \"{}\"",
        bucket_id, object_key
    );

    query(&query_str).execute(&conn).await?;

    Ok(())
}

pub async fn ps_get_account_name(conn: PSConnection, account_id: u64) -> Result<String, Error> {
    let query_str = format!(
        "SELECT account_name FROM accounts WHERE id = {}",
        account_id
    );

    let account: AccountName = query(&query_str).fetch_one(&conn).await?;

    Ok(account.account_name)
}

pub async fn ps_get_account_id(conn: PSConnection, account_name: String) -> Result<u64, Error> {
    let query_str = format!(
        "SELECT id FROM accounts WHERE account_name = \"{}\"",
        account_name
    );

    let account: AccountId = query(&query_str).fetch_one(&conn).await?;

    Ok(account.account_id)
}

pub async fn ps_get_bucket(
    conn: PSConnection,
    account_id: u64,
    bucket_name: &str,
) -> Result<Bucket, Error> {
    let query_str = format!(
        "SELECT * FROM bucket_index WHERE account_id = {} AND bucket_name = \"{}\"",
        account_id, bucket_name
    );

    let bucket: Bucket = query(&query_str).fetch_one(&conn).await?;

    Ok(bucket)
}

pub async fn ps_get_file_system_structure(
    conn: PSConnection,
    bucket_name: &str,
    potential_folder: Option<String>,
    account_id: u64,
    folder_only: bool,
) -> Result<Vec<Object>, Error> {
    let _bucket = ps_get_bucket(conn.clone(), account_id, bucket_name)
        .await
        .map_err(|_| anyhow!("Bucket does not exist or is not owned by current user"))?;

    let lookup_folder = match potential_folder {
        Some(folder) => format!("{}/{}", bucket_name, folder),
        None => bucket_name.to_string(),
    };

    let query_str = if folder_only {
        format!(
            "SELECT * FROM object_index WHERE is_folder = 1 AND full_path LIKE CONCAT('{}', '/%')",
            lookup_folder
        )
    } else {
        format!(
            "SELECT * FROM object_index WHERE full_path LIKE CONCAT('{}', '/%')",
            lookup_folder
        )
    };

    let folder: Vec<Object> = query(&query_str).fetch_all(&conn).await?;
    Ok(folder)
}
