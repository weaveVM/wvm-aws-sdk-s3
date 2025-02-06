use anyhow::Error;

use crate::s3::aws_config::Config;

pub struct Client {
    pub config: Config,
}

impl Client {
    pub fn new(config: Config) -> Result<Self, Error> {
        Ok(Self { config })
    }
}
