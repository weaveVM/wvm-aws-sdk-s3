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

pub type BucketTx = String;

impl From<BucketTx> for CreateBucketOutput {
    fn from(value: BucketTx) -> Self {
        Self {
            location: "wvm-ledger".to_string(),
            bucket_tx: value,
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct DeleteBucketOutput {
    pub bucket_name: String,
    pub account_name: String,
}

impl From<(String, String)> for DeleteBucketOutput {
    fn from(value: (String, String)) -> Self {
        Self {
            bucket_name: value.0,
            account_name: value.1,
        }
    }
}
