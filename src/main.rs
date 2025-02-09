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

    let bucket_name = "hello-world-from-wvm";
    let bucket = client.create_bucket().bucket(bucket_name).send().await?;
    println!("{:?}", bucket);

    Ok(())
}
