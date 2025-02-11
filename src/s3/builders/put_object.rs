use anyhow::Error;
use bundler::utils::core::tags::Tag;

use crate::s3::builders::builders::PutObjectBuilder;
use crate::s3::object::PutObjectOutput;
use crate::utils::planetscale::{ps_get_account_id, ps_get_bucket, ps_put_object};
use crate::utils::wvm::get_transaction;
use crate::utils::wvm_bundler::post_data_to_bundler;
use tokio::time::{sleep, Duration};

impl PutObjectBuilder {
    pub fn bucket(mut self, bucket_name: &str) -> Self {
        self.bucket_name = bucket_name.to_string();
        self
    }

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

    pub async fn send(mut self) -> Result<PutObjectOutput, Error> {
        self.config.account_id = Some(ps_get_account_id(self.config.account_name.clone()).await?);
        let account_id = self.config.account_id.unwrap();
        let wvm_tx =
            post_data_to_bundler(self.clone().data, Some(self.clone().wvm_bundler_tags)).await?;
        // sleep 1s for tx inclusion
        sleep(Duration::from_secs(1)).await;

        let block = get_transaction(wvm_tx.clone()).await?;
        let bucket = ps_get_bucket(account_id, &self.bucket_name).await?;

        if let Some(block) = block {
            let _bucket = ps_put_object(
                bucket.id.parse::<u64>()?,
                &self.key,
                &wvm_tx,
                block.block_number.unwrap_or_default(),
                self.clone().data.len() as u64,
                &serde_json::to_string(&self.metadata)?,
            )
            .await?;
        }

        let output = PutObjectOutput::from(wvm_tx, Some(self.wvm_bundler_tags));
        Ok(output)
    }
}
