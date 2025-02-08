use crate::utils::constants::WVM_RPC_URL;
use alloy::hex::FromHex;
use alloy::providers::{ProviderBuilder, Provider};
use alloy::primitives::{Address, B256, TxHash};
use alloy::rpc::types::Transaction;
use alloy::signers::local::PrivateKeySigner;
use anyhow::{Error, Ok};
use std::str::FromStr;


pub fn derive_compressed_pubkey(private_key: &str) -> Result<String, Error> {
    let signer_address = PrivateKeySigner::from_bytes(&B256::from_str(private_key)?)?.address();
    let address = Address::to_string(&signer_address);

    Ok(address)
}

pub async fn get_transaction(txid: String) -> Result<Option<Transaction>, Error> {
let rpc_url = WVM_RPC_URL.parse()?;
let provider = ProviderBuilder::new().on_http(rpc_url);
let tx = provider.get_transaction_by_hash(TxHash::from_str(&txid)?).await?;
Ok(tx)
}