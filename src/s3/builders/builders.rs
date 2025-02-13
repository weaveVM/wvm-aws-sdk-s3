use crate::s3::aws_config::Config;

#[derive(Debug, Clone, Default)]
pub struct DeleteObjectBuilder {
    pub config: Config,
    pub bucket_name: String,
    pub key: String,
}
