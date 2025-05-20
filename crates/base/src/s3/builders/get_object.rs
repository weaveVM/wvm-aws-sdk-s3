use crate::error::s3_load_errors::S3LoadErrors;
use crate::s3::aws_config::Config;
use crate::s3::builders::RequireBucket;
use crate::s3::object::Object;
use crate::utils::planetscale::{ps_get_account_id, ps_get_bucket, ps_get_object};
use anyhow::Error;
use macros::weavevm;

#[weavevm(require_bucket)]
#[derive(Debug, Clone, Default)]
pub struct GetObjectBuilder<'a> {
    pub config: &'a Config,
    pub key: String,
}

impl<'a> GetObjectBuilder<'a> {
    pub fn key(mut self, key: &str) -> Self {
        self.key = key.to_string();
        self
    }

    pub async fn send(mut self, account_id: u64) -> Result<Object, S3LoadErrors> {
        let db_conn = self.config.db_driver.get_conn();
        let bucket = ps_get_bucket(db_conn.clone(), account_id, &self.bucket_name)
            .await
            .map_err(|_| S3LoadErrors::NoSuchObject)?;
        let bucket_id = bucket
            .id
            .parse::<u64>()
            .map_err(|_| S3LoadErrors::ObjectNotDeleted)?;
        let object = ps_get_object(db_conn, bucket_id, &self.key)
            .await
            .map_err(|_| S3LoadErrors::NoSuchObject)?;
        Ok(object)
    }
}
