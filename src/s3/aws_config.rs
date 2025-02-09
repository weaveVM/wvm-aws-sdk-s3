use crate::utils::constants::WVM_RPC_URL;
use crate::utils::env_utils::get_env_var;
use crate::utils::wvm::derive_compressed_pubkey;
use anyhow::Error;

#[derive(Debug, Default, Clone)]
pub struct Config {
    pub private_key: String,
    pub wvm_rpc_url: String,
    pub account_name: String,
    pub secret_access_key: String,
    pub account_id: Option<u64>,
}

impl Config {
    pub async fn load_from_env() -> Result<Self, Error> {
        let private_key = get_env_var("WVM_AWS_S3_PK")?;
        let secret_access_key = get_env_var("SECRET_ACCESS_KEY")?;
        let address = derive_compressed_pubkey(&private_key)?;
        Ok(Self {
            private_key,
            secret_access_key,
            wvm_rpc_url: WVM_RPC_URL.to_string(),
            account_name: address,
            account_id: None,
        })
    }
}
