use crate::s3::aws_config::Config;
use crate::s3::builders::create_bucket::CreateBucketBuilder;
use crate::s3::builders::delete_bucket::DeleteBucketBuilder;
use crate::s3::builders::delete_object::DeleteObjectBuilder;
use crate::s3::builders::get_object::GetObjectBuilder;
use crate::s3::builders::list_buckets::ListBucketBuilder;
use crate::s3::builders::list_objects::ListObjectsBuilder;
use crate::s3::builders::put_object::PutObjectBuilder;
use anyhow::Error;

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
            ..Default::default()
        }
    }

    pub fn list_buckets(&self) -> ListBucketBuilder {
        ListBucketBuilder {
            config: self.config.clone(),
            ..Default::default()
        }
    }

    pub fn delete_bucket(&self) -> DeleteBucketBuilder {
        DeleteBucketBuilder {
            config: self.config.clone(),
            ..Default::default()
        }
    }

    pub fn put_object(&self) -> PutObjectBuilder {
        PutObjectBuilder {
            config: self.config.clone(),
            ..Default::default()
        }
    }

    pub fn get_object(&self) -> GetObjectBuilder {
        GetObjectBuilder {
            config: self.config.clone(),
            ..Default::default()
        }
    }

    pub fn list_objects_v2(&self) -> ListObjectsBuilder {
        ListObjectsBuilder {
            config: self.config.clone(),
            ..Default::default()
        }
    }

    pub fn delete_object(&self) -> DeleteObjectBuilder {
        DeleteObjectBuilder {
            config: self.config.clone(),
            ..Default::default()
        }
    }
}
