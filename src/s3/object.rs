use bundler::utils::core::tags::Tag;
use planetscale_driver::Database;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default, Database)]
pub struct Object {
    pub id: u64,
    pub bucket_id: u64,
    pub object_key: String,
    pub tx_hash: String,
    pub block_number: u64,
    pub size_bytes: u64,
    pub created_at: String,
    pub last_modified: String,
    pub is_deleted: bool,
    pub metadata: String,
}

#[derive(Debug, Clone, Default)]
pub struct PutObjectOutput {
    pub tx_hash: String,
    pub tags: Option<Vec<Tag>>,
}

impl PutObjectOutput {
    pub fn from(tx_hash: String, tags: Option<Vec<Tag>>) -> Self {
        Self { tx_hash, tags }
    }
}
