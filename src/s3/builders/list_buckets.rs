use crate::s3::aws_config::Config;
use crate::s3::bucket::Bucket;
use crate::utils::planetscale::{ps_get_account_id, ps_list_buckets};
use anyhow::Error;

#[derive(Debug, Clone, Default)]
pub struct ListBucketBuilder {
    pub config: Config,
    pub max_keys: Option<i32>,
}

impl ListBucketBuilder {
    pub fn max_keys(mut self, input: i32) -> Self {
        self.max_keys = Some(input);
        self
    }

    pub async fn send(&mut self) -> Result<Vec<Bucket>, Error> {
        self.config.account_id = Some(ps_get_account_id(self.config.account_name.clone()).await?);
        let account_id = self.config.account_id.unwrap();
        let buckets = ps_list_buckets(account_id, self.max_keys).await?;
        Ok(buckets)
    }
}
