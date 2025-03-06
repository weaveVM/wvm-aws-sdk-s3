use anyhow::Error;
use base::s3::bucket::CreateBucketOutput;
use base::s3::client::Client;
use planetscale::PlanetScaleDriver;
use std::sync::Arc;

pub struct WvmBucketService<'a> {
    pub db_service: Arc<PlanetScaleDriver>,
    pub s3_client: Arc<Client<'a>>,
}

impl<'a> WvmBucketService<'a> {
    pub async fn create_bucket(&self, bucket_name: String) -> Result<CreateBucketOutput, Error> {
        self.s3_client
            .create_bucket()
            .bucket(&bucket_name)
            .send()
            .await
    }
}
