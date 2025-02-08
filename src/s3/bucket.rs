use crate::utils::wvm_bundler::post_data_to_bundler;
use crate::utils::wvm::get_transaction;
use anyhow::Error;
use bundler::utils::core::tags::Tag;
use planetscale_driver::Database;
use serde::{Deserialize, Serialize};
use serde_json::{Value, to_vec};
use crate::utils::planetscale::{get_account_name, ps_create_bucket};
#[derive(Debug, Clone, Serialize, Deserialize, Default, Database)]
pub struct Bucket {
    pub id: u64,
    pub account_id: u64,
    pub account_name: String,
    pub bucket_name: String,
    pub tx_hash: String,
    pub block_number: u64,
    pub created_at: String
}

impl Bucket {
    pub async fn create_bucket(account_id: u64, bucket_name: String) -> Result<(), Error> {
        let account_name = get_account_name(account_id).await?;
        let bucket_data = bucket_name.as_bytes().to_vec();
        let bucket_tags = vec![Tag::new("owner".to_string(), account_name)];
        let envelope = post_data_to_bundler(bucket_data, Some(bucket_tags)).await?;
        let block = get_transaction(envelope.clone()).await?;

        if let Some(block) = block {
            let _bucket = ps_create_bucket(account_id, &bucket_name, &envelope, block.block_number.unwrap_or_default()).await?;
        }

        Ok(())
    }
}