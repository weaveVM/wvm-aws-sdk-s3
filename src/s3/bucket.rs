use crate::utils::wvm_bundler::post_data_to_bundler;
use anyhow::Error;
use bundler::utils::core::tags::Tag;
use planetscale_driver::Database;
use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default, Database)]
pub struct Bucket {
    pub id: String,
    pub account_id: String,
    pub bucket_name: String,
    pub tx_hash: String,
    pub block_number: String,
    pub created_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CreateBucketOutput {
    pub location: String,
    pub bucket_tx: String,
}

impl CreateBucketOutput {
    pub fn from(bucket_tx: String) -> Self {
        Self {
            location: "wvm-ledger".to_string(),
            bucket_tx,
        }
    }
}

impl Bucket {
    pub async fn create_bucket(account_name: String, bucket_name: String) -> Result<String, Error> {
        let bucket_data = bucket_name.as_bytes().to_vec();
        let bucket_tags = vec![Tag::new("owner".to_string(), account_name)];
        let envelope = post_data_to_bundler(bucket_data, Some(bucket_tags)).await?;
        Ok(envelope)
    }
}
