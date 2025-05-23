use crate::s3::aws_config::Config;
use crate::s3::bucket::Bucket;
use crate::utils::planetscale::{ps_get_account_id, ps_list_buckets};
use anyhow::Error;

#[derive(Debug, Clone, Default)]
pub struct ListBucketBuilder<'a> {
    pub config: &'a Config,
    pub max_keys: Option<i32>,
}

impl<'a> ListBucketBuilder<'a> {
    pub async fn send(&mut self, account_id: u64) -> Result<Vec<Bucket>, Error> {
        let db_conn = self.config.db_driver.get_conn();
        let buckets = ps_list_buckets(db_conn, account_id, self.max_keys).await?;
        Ok(buckets)
    }
}
