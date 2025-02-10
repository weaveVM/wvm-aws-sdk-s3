pub mod s3;
pub mod utils;
use anyhow::Error;

use crate::s3::aws_config::Config;
use crate::s3::bucket::Bucket;
use crate::s3::client::Client;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let aws_config = Config::load_from_env().await?;
    let client = Client::new(&aws_config)?;

    let bucket_name = "aloha";
    // let bucket = client.create_bucket().bucket(bucket_name).send().await?;
    let buckets = client.list_buckets().send().await?;
    println!("{:?}", buckets);

    Ok(())
}
