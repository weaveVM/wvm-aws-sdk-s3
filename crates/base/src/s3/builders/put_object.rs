use anyhow::{anyhow, Error};
use bundler::utils::core::tags::Tag;
use std::sync::Arc;

use crate::s3::aws_config::Config;
use crate::s3::bucket::Bucket;
use crate::s3::builders::RequireBucket;
use crate::s3::client::Client;
use crate::s3::object::PutObjectOutput;
use crate::utils::planetscale::{ps_get_account_id, ps_get_bucket, ps_put_object};
use crate::utils::wvm::get_transaction;
use crate::utils::wvm_bundler::post_data_to_bundler;
use macros::weavevm;
use tokio::time::{sleep, Duration};

#[weavevm(require_bucket)]
#[derive(Debug, Clone, Default)]
pub struct PutObjectBuilder<'a> {
    pub config: &'a Config,
    pub key: String,
    pub data: Vec<u8>,
    pub metadata: Vec<(String, String)>,
    pub wvm_bundler_tags: Vec<Tag>,
}

impl<'a> PutObjectBuilder<'a> {
    pub fn key(mut self, key: &str) -> Self {
        self.key = key.to_string();
        self
    }

    pub fn body(mut self, data: Vec<u8>) -> Self {
        self.data = data;
        self
    }

    pub fn content_type(mut self, mime: &str) -> Self {
        let content_type = if mime.trim().is_empty() {
            "application/octet-stream"
        } else {
            mime
        };

        self.metadata
            .push(("Content-Type".to_string(), content_type.to_string()));

        self.wvm_bundler_tags.push(Tag::new(
            "Content-Type".to_string(),
            content_type.to_string(),
        ));

        self
    }

    pub fn metadata(mut self, tag1: &str, tag2: &str) -> Self {
        let tag1 = tag1.to_string();
        let tag2 = tag2.to_string();
        self.metadata.push((tag1.clone(), tag2.clone()));
        self.wvm_bundler_tags.push(Tag::new(tag1, tag2));
        self
    }

    async fn find_bucket(&self, account_id: u64) -> Result<Bucket, Error> {
        ps_get_bucket(
            self.config.db_driver.clone().get_conn(),
            account_id,
            &self.bucket_name,
        )
        .await
    }

    pub async fn send(
        mut self,
        account_id: u64,
        create_bucket_if_not_exists: bool,
        s3_service: Arc<Client<'a>>,
    ) -> Result<PutObjectOutput, Error> {
        let wvm_tx =
            post_data_to_bundler(self.clone().data, Some(self.clone().wvm_bundler_tags)).await?;
        // sleep 1s for tx inclusion
        sleep(Duration::from_secs(1)).await;

        let bucket = match self.find_bucket(account_id).await {
            Ok(bucket) => Ok(bucket),
            Err(e) => {
                let e_str = e.to_string();
                let no_results = e_str.contains("No results found");
                if no_results && create_bucket_if_not_exists {
                    let _crb = s3_service
                        .create_bucket()
                        .bucket(&self.bucket_name)
                        .send(account_id)
                        .await;
                    Ok(self.find_bucket(account_id).await?)
                } else {
                    eprintln!("Error finding bucket {}", e.to_string());
                    Err(anyhow!("{}", e.to_string()))
                }
            }
        };

        println!("{:?}", bucket);

        let bucket = bucket?;

        let db_conn = self.config.db_driver.get_conn();
        let _bucket = ps_put_object(
            db_conn,
            bucket.id.parse::<u64>()?,
            &self.key,
            &wvm_tx,
            0,
            self.clone().data.len() as u64,
            &serde_json::to_string(&self.metadata)?,
        )
        .await?;

        let output = PutObjectOutput::from(wvm_tx, Some(self.wvm_bundler_tags));
        Ok(output)
    }
}
