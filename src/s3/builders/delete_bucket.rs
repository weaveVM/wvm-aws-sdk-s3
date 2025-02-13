use crate::s3::aws_config::Config;
use crate::s3::bucket::DeleteBucketOutput;
use crate::utils::planetscale::{ps_delete_bucket, ps_get_account_id};
use anyhow::Error;

#[derive(Debug, Clone, Default)]
pub struct DeleteBucketBuilder {
    pub config: Config,
    pub bucket_name: String,
}

impl DeleteBucketBuilder {
    pub fn bucket(mut self, bucket_name: &str) -> Self {
        self.bucket_name = bucket_name.to_string();
        self
    }

    pub async fn send(mut self) -> Result<DeleteBucketOutput, Error> {
        self.config.account_id = Some(ps_get_account_id(self.config.account_name.clone()).await?);
        let account_id = self.config.account_id.unwrap();
        let _deleted_bucket = ps_delete_bucket(account_id, &self.bucket_name).await?;
        let output = DeleteBucketOutput::from((self.bucket_name, self.config.account_name));
        Ok(output)
    }
}
