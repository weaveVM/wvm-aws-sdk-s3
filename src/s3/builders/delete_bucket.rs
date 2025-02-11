use crate::s3::bucket::DeleteBucketOutput;
use crate::s3::builders::builders::DeleteBucketBuilder;
use crate::utils::planetscale::{ps_delete_bucket, ps_get_account_id};
use anyhow::Error;

impl DeleteBucketBuilder {
    pub fn bucket(mut self, bucket_name: &str) -> Self {
        self.bucket_name = bucket_name.to_string();
        self
    }

    pub async fn send(mut self) -> Result<DeleteBucketOutput, Error> {
        self.config.account_id = Some(ps_get_account_id(self.config.account_name.clone()).await?);
        let account_id = self.config.account_id.unwrap();
        let _deleted_bucket = ps_delete_bucket(account_id, &self.bucket_name).await?;
        let res = DeleteBucketOutput::from(self.bucket_name, self.config.account_name);
        Ok(res)
    }
}
