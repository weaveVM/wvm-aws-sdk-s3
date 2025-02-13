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
    pub async fn send(&mut self) -> Result<Vec<Bucket>, Error> {
        let account_id = self.config.account_id.unwrap();
        let buckets = ps_list_buckets(account_id, self.max_keys).await?;
        Ok(buckets)
    }
}
