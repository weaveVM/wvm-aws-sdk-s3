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
    pub async fn send(mut self, account_id: u64) -> Result<DeleteBucketOutput, Error> {
        let db_conn = self.config.db_driver.get_conn();
        let _deleted_bucket = ps_delete_bucket(db_conn, account_id, &self.bucket_name).await?;
        let output = DeleteBucketOutput::from((self.bucket_name, account_id));
        Ok(output)
    }
}
