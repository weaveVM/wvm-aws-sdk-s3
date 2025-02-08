pub mod s3;
pub mod utils;
use crate::s3::bucket::Bucket;

#[tokio::main]
async fn main() {
    println!("Hello, world!");
    let _bucket = Bucket::create_bucket(1, "hello_wvm".to_string())
        .await
        .unwrap();
}
