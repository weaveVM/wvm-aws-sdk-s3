use crate::s3::aws_config::Config;
use crate::s3::builders::RequireBucket;
use crate::s3::object::Object;
use crate::utils::planetscale::{ps_get_account_id, ps_get_bucket, ps_list_objects};
use anyhow::Error;
use macros::weavevm;

#[weavevm(require_bucket)]
#[derive(Debug, Clone, Default)]
pub struct ListObjectsBuilder<'a> {
    pub config: &'a Config,
    pub max_keys: Option<i32>,
}

impl<'a> ListObjectsBuilder<'a> {
    pub fn max_keys(mut self, input: i32) -> Self {
        self.max_keys = Some(input);
        self
    }

    pub async fn send(mut self) -> Result<Vec<Object>, Error> {
        let account_id = self.config.account_id.unwrap();
        let db_conn = self.config.db_driver.get_conn();
        let bucket = ps_get_bucket(db_conn.clone(), account_id, &self.bucket_name).await?;
        let objects = ps_list_objects(db_conn, bucket.id.parse::<u64>()?, self.max_keys).await?;
        Ok(objects)
    }
}
