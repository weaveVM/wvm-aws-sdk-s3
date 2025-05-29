use crate::error::s3_load_errors::S3LoadErrors;
use crate::s3::aws_config::Config;
use crate::s3::bucket::{Bucket, CreateBucketOutput};
use crate::s3::builders::RequireBucket;
use crate::utils::planetscale::ps_get_bucket;
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
    pub async fn send(self, account_id: u64) -> Result<CreateBucketOutput, S3LoadErrors> {
        let account_name = self.config.account_name.clone();
        let mut config = self.config;
        let db_conn = config.db_driver.get_conn();
        let bucket_name = &self.bucket_name;

        let curr_bucket = ps_get_bucket(db_conn.clone(), account_id, bucket_name).await;

        // Bucket does not exist and it can be created
        if let Ok(bucket) = curr_bucket {
            if bucket.account_id == account_id.to_string() {
                Err(S3LoadErrors::BucketAlreadyOwnedByYou)
            } else {
                Err(S3LoadErrors::BucketAlreadyExists)
            }
        } else {
            let bucket_tx = create_bucket(account_name, self.bucket_name.clone())
                .await
                .map_err(|_| S3LoadErrors::BucketNotCreated)?;
            // sleep 1s for tx inclusion on WeaveVM block
            sleep(Duration::from_secs(1)).await;

            let _bucket = ps_create_bucket(db_conn, account_id, &bucket_name, &bucket_tx, 0)
                .await
                .map_err(|_| S3LoadErrors::BucketNotCreated)?;

            let output = CreateBucketOutput::from(bucket_tx);
            Ok(output)
        }
    }
}

async fn create_bucket(account_name: String, bucket_name: String) -> Result<String, Error> {
    let bucket_data = bucket_name.as_bytes().to_vec();
    let bucket_tags = vec![Tag::new("owner".to_string(), account_name)];
    let envelope = post_data_to_bundler(bucket_data, Some(bucket_tags)).await?;
    Ok(envelope)
}
