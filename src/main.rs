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
    
    // create bucket
    let bucket = client.create_bucket().bucket(bucket_name).send().await?;
    println!("{:?}", bucket);
    
    // list buckets
    // let buckets = client.list_buckets().max_keys(1).send().await?;
    // println!("{:?}", buckets);

    // delete bucket
    // let deleted_bucket = client.delete_bucket().bucket(bucket_name).send().await?;
    // println!("{:?}", deleted_bucket);

    Ok(())
}
