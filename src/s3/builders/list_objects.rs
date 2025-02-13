use crate::s3::aws_config::Config;
use crate::s3::object::Object;
use crate::utils::planetscale::{ps_get_account_id, ps_get_bucket, ps_list_objects};
use anyhow::Error;

#[derive(Debug, Clone, Default)]
pub struct ListObjectsBuilder {
    pub config: Config,
    pub bucket_name: String,
    pub max_keys: Option<i32>,
}

impl ListObjectsBuilder {
    pub fn bucket(mut self, bucket_name: &str) -> Self {
        self.bucket_name = bucket_name.to_string();
        self
    }

    pub fn max_keys(mut self, input: i32) -> Self {
        self.max_keys = Some(input);
        self
    }

    pub async fn send(mut self) -> Result<Vec<Object>, Error> {
        self.config.account_id = Some(ps_get_account_id(self.config.account_name.clone()).await?);
        let account_id = self.config.account_id.unwrap();
        let bucket = ps_get_bucket(account_id, &self.bucket_name).await?;
        let objects = ps_list_objects(bucket.id.parse::<u64>()?, self.max_keys).await?;
        Ok(objects)
    }
}
