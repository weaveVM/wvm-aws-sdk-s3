use crate::s3::aws_config::Config;
use bundler::utils::core::tags::Tag;

#[derive(Debug, Clone, Default)]
pub struct CreateBucketBuilder {
    pub config: Config,
    pub bucket_name: String,
}

#[derive(Debug, Clone, Default)]
pub struct ListBucketBuilder {
    pub config: Config,
    pub max_keys: Option<i32>,
}

#[derive(Debug, Clone, Default)]
pub struct DeleteBucketBuilder {
    pub config: Config,
    pub bucket_name: String,
}

#[derive(Debug, Clone, Default)]
pub struct PutObjectBuilder {
    pub config: Config,
    pub bucket_name: String,
    pub key: String,
    pub data: Vec<u8>,
    pub metadata: Vec<(String, String)>,
    pub wvm_bundler_tags: Vec<Tag>,
}

#[derive(Debug, Clone, Default)]
pub struct GetObjectBuilder {
    pub config: Config,
    pub bucket_name: String,
    pub key: String,
}

#[derive(Debug, Clone, Default)]
pub struct ListObjectsBuilder {
    pub config: Config,
    pub bucket_name: String,
    pub max_keys: Option<i32>,
}

#[derive(Debug, Clone, Default)]
pub struct DeleteObjectBuilder {
    pub config: Config,
    pub bucket_name: String,
    pub key: String,
}
