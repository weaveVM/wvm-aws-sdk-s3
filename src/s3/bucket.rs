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

#[derive(Debug, Clone, Default)]
pub struct DeleteBucketOutput {
    pub bucket_name: String,
    pub account_name: String,
}

impl DeleteBucketOutput {
    pub fn from(bucket_name: String, account_name: String) -> Self {
        Self {
            bucket_name,
            account_name,
        }
    }
}
