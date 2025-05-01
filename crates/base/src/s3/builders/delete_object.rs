use crate::s3::aws_config::Config;
use crate::s3::builders::RequireBucket;
use crate::s3::object::DeleteObjectOutput;
use crate::utils::planetscale::{ps_delete_object, ps_get_account_id, ps_get_bucket};
use anyhow::Error;
use macros::weavevm;

#[weavevm(require_bucket)]
#[derive(Debug, Clone, Default)]
pub struct DeleteObjectBuilder<'a> {
    pub config: &'a Config,
    pub key: String,
}

impl<'a> DeleteObjectBuilder<'a> {
    pub fn key(mut self, key: &str) -> Self {
        self.key = key.to_string();
        self
    }

    pub async fn send(mut self, account_id: u64) -> Result<DeleteObjectOutput, Error> {
        let db_conn = self.config.db_driver.get_conn();
        let bucket = ps_get_bucket(db_conn.clone(), account_id, &self.bucket_name).await?;
        let _deleted_object =
            ps_delete_object(db_conn, bucket.id.parse::<u64>()?, &self.key).await?;
        let output = DeleteObjectOutput::from(true, self.key);
        Ok(output)
    }
}
