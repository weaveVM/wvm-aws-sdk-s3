use crate::s3::aws_config::Config;
use crate::s3::bucket::{Bucket, CreateBucketOutput};
use crate::s3::builders::RequireBucket;
use crate::utils::planetscale::{ps_create_bucket, ps_get_account_id, ps_get_account_name};
use crate::utils::wvm::get_transaction;
use crate::utils::wvm_bundler::post_data_to_bundler;
use anyhow::Error;
use bundler::utils::core::tags::Tag;
use macros::weavevm;
use tokio::time::{sleep, Duration};

#[weavevm(require_bucket)]
#[derive(Debug, Clone, Default)]
pub struct CreateBucketBuilder<'a> {
    pub config: &'a Config,
}

impl<'a> CreateBucketBuilder<'a> {
    pub async fn send(self) -> Result<CreateBucketOutput, Error> {
        let account_name = self.config.account_name.clone();
        let mut config = self.config;

        let bucket_tx = create_bucket(account_name, self.bucket_name.clone()).await?;
        // sleep 1s for tx inclusion on WeaveVM block
        sleep(Duration::from_secs(1)).await;

        let block = get_transaction(bucket_tx.clone()).await?;

        if let Some(block) = block {
            let db_conn = config.db_driver.get_conn();

            let _bucket = ps_create_bucket(
                db_conn,
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
