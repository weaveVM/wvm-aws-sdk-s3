use planetscale_driver::Database;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default, Database)]
pub struct Bucket {
    pub id: u64,
    pub account_id: u64,
    pub bucket_name: String,
    pub tx_hash: String,
    pub block_number: u64,
    pub created_at: String,
    pub mutable_settings: String,
}
