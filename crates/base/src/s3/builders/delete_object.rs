use crate::s3::aws_config::Config;
use crate::s3::object::DeleteObjectOutput;
use crate::utils::planetscale::{ps_delete_object, ps_get_account_id, ps_get_bucket};
use anyhow::Error;

#[derive(Debug, Clone, Default)]
pub struct DeleteObjectBuilder<'a> {
    pub config: &'a Config,
    pub bucket_name: String,
    pub key: String,
}

impl<'a> DeleteObjectBuilder<'a> {
    pub fn bucket(mut self, bucket_name: &str) -> Self {
        self.bucket_name = bucket_name.to_string();
        self
    }

    pub fn key(mut self, key: &str) -> Self {
        self.key = key.to_string();
        self
    }

    pub async fn send(mut self) -> Result<DeleteObjectOutput, Error> {
        let account_id = self.config.account_id.unwrap();
        let bucket = ps_get_bucket(account_id, &self.bucket_name).await?;
        let _deleted_object = ps_delete_object(bucket.id.parse::<u64>()?, &self.key).await?;
        let output = DeleteObjectOutput::from(true, self.key);
        Ok(output)
    }
}
