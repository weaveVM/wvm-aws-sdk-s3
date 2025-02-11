use crate::s3::aws_config::Config;

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
