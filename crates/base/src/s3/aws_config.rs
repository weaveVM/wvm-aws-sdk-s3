use crate::s3::client::Client;
use crate::s3::S3_CONFIG;
use crate::utils::constants::WVM_RPC_URL;
use crate::utils::env_utils::get_env_var;
use crate::utils::wvm::derive_compressed_pubkey;
use anyhow::Error;
use planetscale::PlanetScaleDriver;
use std::sync::Arc;

#[derive(Debug, Default, Clone)]
pub struct Config {
    pub private_key: String,
    pub wvm_rpc_url: String,
    pub account_name: String,
    pub secret_access_key: String,
    pub account_id: Option<u64>,
    pub db_driver: Arc<PlanetScaleDriver>,
}

impl<'a> Default for &'a Config {
    fn default() -> &'a Config {
        S3_CONFIG.get().unwrap()
    }
}

impl Config {
    pub fn load(aws_s3_pk: Option<String>, access_key: Option<String>) -> Result<Arc<Self>, Error> {
        let is_initialized = S3_CONFIG.get();

        if let Some(conf) = is_initialized {
            Ok(conf.clone())
        } else {
            let private_key = get_env_var("WVM_AWS_S3_PK").unwrap_or_else(|_| aws_s3_pk.unwrap());
            let secret_access_key =
                get_env_var("SECRET_ACCESS_KEY").unwrap_or_else(|_| access_key.unwrap());
            let address = derive_compressed_pubkey(&private_key)?;
            let conf = Self {
                private_key,
                secret_access_key,
                wvm_rpc_url: WVM_RPC_URL.to_string(),
                account_name: address,
                account_id: None,
                db_driver: Arc::new(PlanetScaleDriver {
                    host: "".to_string(),
                    username: "".to_string(),
                    password: "".to_string(),
                }),
            };

            let s3_conf = S3_CONFIG.get_or_init(|| Arc::new(conf));
            Ok(s3_conf.clone())
        }
    }
}
