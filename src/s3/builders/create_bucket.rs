use crate::s3::bucket::{Bucket, CreateBucketOutput};
use crate::s3::builders::builders::CreateBucketBuilder;
use crate::utils::planetscale::{get_account_id, get_account_name, ps_create_bucket};
use crate::utils::wvm::get_transaction;
use anyhow::Error;

impl CreateBucketBuilder {
    pub fn bucket(mut self, bucket_name: &str) -> Self {
        self.bucket_name = bucket_name.to_string();
        self
    }

    pub async fn send(self) -> Result<CreateBucketOutput, Error> {
        let account_name = self.config.account_name.clone();
        let mut config = self.config;
        config.account_id = Some(get_account_id(account_name.clone()).await?);

        let bucket_tx = Bucket::create_bucket(account_name, self.bucket_name.clone()).await?;

        let block = get_transaction(bucket_tx.clone()).await?;

        if let Some(block) = block {
            let _bucket = ps_create_bucket(
                config.account_id.unwrap(),
                &self.bucket_name,
                &bucket_tx,
                block.block_number.unwrap_or_default(),
            )
            .await?;
        }

        let output = CreateBucketOutput::from(bucket_tx);
        Ok(output)
    }
}
