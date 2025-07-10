use crate::utils::vars::DATA_RETRIEVAL_ENDPOINT;

pub fn extract_metadata(str_metadata: String) -> Vec<(String, String)> {
    serde_json::from_str(&str_metadata).unwrap_or_else(|_| vec![])
}

pub fn find_key_in_metadata(metadata: &Vec<(String, String)>, key: String) -> Option<String> {
    let key = key.to_lowercase();
    metadata
        .iter()
        .find(|(k, _)| &(k.to_lowercase()) == &key)
        .map(|(_, v)| v.clone())
}

pub fn retrieve_object_bytes(tx: &str) -> Option<Vec<u8>> {
    let data = ureq::get(format!("{}/{}", &*DATA_RETRIEVAL_ENDPOINT, tx))
        .call()
        .ok();
    if let Some(res) = data {
        let mut body = res.into_body();
        return body
            .with_config()
            .limit(((1024 * 1024) * 120) as u64)
            .read_to_vec()
            .ok();
    }
    None
}
