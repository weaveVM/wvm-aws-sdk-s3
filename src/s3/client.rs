use crate::s3::aws_config::Config;
use crate::s3::builders::builders::{CreateBucketBuilder, ListBucketBuilder};
use anyhow::Error;

use super::builders::builders::DeleteBucketBuilder;

#[derive(Debug, Clone, Default)]
pub struct Client {
    pub config: Config,
}

impl Client {
    pub fn new(config: &Config) -> Result<Self, Error> {
        Ok(Self {
            config: config.clone(),
        })
    }

    pub fn create_bucket(&self) -> CreateBucketBuilder {
        CreateBucketBuilder {
            config: self.config.clone(),
            bucket_name: String::new(),
        }
    }

    pub fn list_buckets(&self) -> ListBucketBuilder {
        ListBucketBuilder {
            config: self.config.clone(),
            max_keys: None,
        }
    }

    pub fn delete_bucket(&self) -> DeleteBucketBuilder {
        DeleteBucketBuilder {
            config: self.config.clone(),
            bucket_name: String::new(),
        }
    }
}
