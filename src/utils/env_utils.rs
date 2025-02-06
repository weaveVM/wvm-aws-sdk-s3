use dotenv::dotenv;
use std::env;
use anyhow::Error;

pub fn get_env_key(key: &str) -> Result<String, Error> {
    dotenv().ok();
    match env::var(key) {
        Ok(key) => Ok(key),
        Err(key) => Err(key.into())
    }
}