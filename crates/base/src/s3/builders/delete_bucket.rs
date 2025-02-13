use crate::s3::aws_config::Config;
use crate::s3::bucket::DeleteBucketOutput;
use crate::s3::builders::RequireBucket;
use crate::utils::planetscale::{ps_delete_bucket, ps_get_account_id};
use anyhow::Error;
use macros::weavevm;

#[weavevm(require_bucket)]
#[derive(Debug, Clone, Default)]
pub struct DeleteBucketBuilder<'a> {
    pub config: &'a Config,
}

impl<'a> DeleteBucketBuilder<'a> {
    pub async fn send(mut self) -> Result<DeleteBucketOutput, Error> {
        let account_id = self.config.account_id.unwrap();
        let _deleted_bucket = ps_delete_bucket(account_id, &self.bucket_name).await?;
        let output = DeleteBucketOutput::from((self.bucket_name, self.config.account_name.clone()));
        Ok(output)
    }
}
