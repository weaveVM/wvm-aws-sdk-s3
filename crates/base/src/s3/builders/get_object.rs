use crate::s3::aws_config::Config;
use crate::s3::builders::RequireBucket;
use crate::s3::object::Object;
use crate::utils::planetscale::{ps_get_account_id, ps_get_bucket, ps_get_object};
use anyhow::Error;
use macros::weavevm;

#[derive(Debug, Clone, Default)]
#[weavevm(require_bucket)]
pub struct GetObjectBuilder<'a> {
    pub config: &'a Config,
    pub bucket_name: String,
    pub key: String,
}

impl<'a> GetObjectBuilder<'a> {
    pub fn key(mut self, key: &str) -> Self {
        self.key = key.to_string();
        self
    }

    pub async fn send(mut self) -> Result<Object, Error> {
        let account_id = self.config.account_id.unwrap();
        let bucket = ps_get_bucket(account_id, &self.bucket_name).await?;
        let object = ps_get_object(bucket.id.parse::<u64>()?, &self.key).await?;
        Ok(object)
    }
}
