use alloy::primitives::{Address, B256};
use alloy::signers::local::PrivateKeySigner;
use anyhow::Error;
use std::str::FromStr;

pub fn derive_compressed_pubkey(private_key: &str) -> Result<String, Error> {
    let signer_address = PrivateKeySigner::from_bytes(&B256::from_str(private_key)?)?.address();
    let address = Address::to_string(&signer_address);

    Ok(address)
}
