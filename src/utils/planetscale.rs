use crate::s3::bucket::Bucket;
use crate::s3::object::Object;
use crate::utils::env_utils::get_env_var;
use anyhow::Error;
use planetscale_driver::{query, PSConnection};

async fn ps_client() -> Result<PSConnection, Error> {
    let host = get_env_var("DATABASE_HOST")?;
    let username = get_env_var("DATABASE_USERNAME")?;
    let password = get_env_var("DATABASE_PASSWORD")?;

    let conn: PSConnection = PSConnection::new(&host, &username, &password);

    Ok(conn)
}

pub async fn list_buckets(account_id: u64) -> Result<Vec<Bucket>, Error> {
    let conn = ps_client().await?;

    let query_str = format!(
        "SELECT id, bucket_name, created_at FROM bucket_index WHERE account_id = {}",
        account_id
    );

    let buckets: Vec<Bucket> = query(&query_str).fetch_all(&conn).await?;

    Ok(buckets)
}

pub async fn create_bucket(
    account_id: u64,
    bucket_name: &str,
    tx_hash: &str,
    block_number: u64,
) -> Result<(), Error> {
    let conn = ps_client().await?;

    let query_str = format!(
        "INSERT INTO bucket_index(account_id, bucket_name, tx_hash, block_number) VALUES({}, \"{}\", \"{}\", {})",
        account_id, bucket_name, tx_hash, block_number
    );

    query(&query_str).execute(&conn).await?;

    Ok(())
}

pub async fn delete_bucket(account_id: u64, bucket_name: &str) -> Result<(), Error> {
    let conn = ps_client().await?;

    let query_str = format!(
        "DELETE FROM bucket_index WHERE account_id = {} AND bucket_name = \"{}\"",
        account_id, bucket_name
    );

    query(&query_str).execute(&conn).await?;

    Ok(())
}

pub async fn list_objects(bucket_id: u64) -> Result<Vec<Object>, Error> {
    let conn = ps_client().await?;

    let query_str = format!(
        "SELECT id, object_key, tx_hash, block_number, size_bytes, metadata FROM object_index WHERE bucket_id = {} AND is_deleted = false",
        bucket_id
    );

    let objects: Vec<Object> = query(&query_str).fetch_all(&conn).await?;

    Ok(objects)
}

pub async fn put_object(
    bucket_id: u64,
    object_key: &str,
    tx_hash: &str,
    block_number: u64,
    size_bytes: u64,
    metadata: &str,
) -> Result<(), Error> {
    let conn = ps_client().await?;

    let query_str = format!(
        "INSERT INTO object_index(bucket_id, object_key, tx_hash, block_number, size_bytes, metadata) VALUES({}, \"{}\", \"{}\", {}, {}, \"{}\")",
        bucket_id, object_key, tx_hash, block_number, size_bytes, metadata
    );

    query(&query_str).execute(&conn).await?;

    Ok(())
}

pub async fn get_object(bucket_id: u64, object_key: &str) -> Result<Object, Error> {
    let conn = ps_client().await?;

    let query_str = format!(
        "SELECT id, tx_hash, block_number, size_bytes, metadata FROM object_index WHERE bucket_id = {} AND object_key = \"{}\" AND is_deleted = false",
        bucket_id, object_key
    );

    let object: Object = query(&query_str).fetch_one(&conn).await?;

    Ok(object)
}

pub async fn delete_object(bucket_id: u64, object_key: &str) -> Result<(), Error> {
    let conn = ps_client().await?;

    let query_str = format!(
        "UPDATE object_index SET is_deleted = true WHERE bucket_id = {} AND object_key = \"{}\"",
        bucket_id, object_key
    );

    query(&query_str).execute(&conn).await?;

    Ok(())
}
