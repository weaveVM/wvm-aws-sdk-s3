use crate::s3::aws_config::Config;
use crate::s3::bucket::{Bucket, CreateBucketOutput};
use crate::utils::planetscale::{get_account_id, ps_create_bucket};
use crate::utils::wvm::get_transaction;
use anyhow::{Error, Ok};

#[derive(Debug, Clone, Default)]
pub struct Client {
    pub config: Config,
    pub temporal_placeholder: String,
}

impl Client {
    pub fn new(config: &Config) -> Result<Self, Error> {
        Ok(Self {
            config: config.clone(),
            ..Default::default()
        })
    }

    pub fn create_bucket(self) -> Self {
        self
    }

    pub fn bucket(mut self, bucket_name: &str) -> Self {
        self.temporal_placeholder = bucket_name.to_string();
        self
    }

    pub async fn send(mut self) -> Result<CreateBucketOutput, Error> {
        let account_name = self.config.account_name.clone();
        self.config.account_id = Some(get_account_id(account_name.clone()).await?);
        let bucket_tx =
            Bucket::create_bucket(account_name, self.temporal_placeholder.clone()).await?;

        let block = get_transaction(bucket_tx.clone()).await?;

        if let Some(block) = block {
            let _bucket = ps_create_bucket(
                self.config.account_id.unwrap(),
                &self.temporal_placeholder,
                &bucket_tx,
                block.block_number.unwrap_or_default(),
            )
            .await?;
        }

        self.temporal_placeholder.clear();
        let output = CreateBucketOutput::from(bucket_tx);

        Ok(output)
    }
}
