use crate::s3::bucket::Bucket;
use crate::s3::builders::builders::ListBucketBuilder;
use crate::utils::planetscale::{get_account_id, ps_list_buckets};
use anyhow::Error;

impl ListBucketBuilder {
    pub fn max_keys(mut self, input: i32) -> Self {
        self.max_keys = Some(input);
        self
    }

    pub async fn send(&mut self) -> Result<Vec<Bucket>, Error> {
        self.config.account_id = Some(get_account_id(self.config.account_name.clone()).await?);
        let account_id = self.config.account_id.unwrap();
        let buckets = ps_list_buckets(account_id, self.max_keys).await?;
        Ok(buckets)
    }
}
