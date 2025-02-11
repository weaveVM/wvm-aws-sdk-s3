use crate::s3::bucket::{Bucket, CreateBucketOutput};
use crate::s3::builders::builders::CreateBucketBuilder;
use crate::utils::planetscale::{ps_create_bucket, ps_get_account_id, ps_get_account_name};
use crate::utils::wvm::get_transaction;
use crate::utils::wvm_bundler::post_data_to_bundler;
use anyhow::Error;
use bundler::utils::core::tags::Tag;

impl CreateBucketBuilder {
    pub fn bucket(mut self, bucket_name: &str) -> Self {
        self.bucket_name = bucket_name.to_string();
        self
    }

    pub async fn send(self) -> Result<CreateBucketOutput, Error> {
        let account_name = self.config.account_name.clone();
        let mut config = self.config;
        config.account_id = Some(ps_get_account_id(account_name.clone()).await?);

        let bucket_tx = create_bucket(account_name, self.bucket_name.clone()).await?;

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

async fn create_bucket(account_name: String, bucket_name: String) -> Result<String, Error> {
    let bucket_data = bucket_name.as_bytes().to_vec();
    let bucket_tags = vec![Tag::new("owner".to_string(), account_name)];
    let envelope = post_data_to_bundler(bucket_data, Some(bucket_tags)).await?;
    Ok(envelope)
}
