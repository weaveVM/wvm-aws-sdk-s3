use crate::permission_container::PermissionContainer;
use actix_web::error::ErrorUnauthorized;
use actix_web::{Error, HttpMessage, HttpRequest};
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AccessKey {
    pub id: i64,
    pub owner_id: i64,
    pub access_key: String,
    pub created_at: String,
    pub is_active: bool,
    pub metadata: KeyMetadata,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Account {
    pub id: i64,
    pub account_name: String,
    pub created_at: String,
    pub updated_at: String,
    pub is_active: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct KeyMetadata {
    pub permissions: Option<Vec<String>>,
}

#[derive(Clone)]
pub struct CurrentUser(pub AccessKey);

pub fn get_internal_call<T: Serialize + DeserializeOwned>(url: String) -> anyhow::Result<T> {
    let api_internal_key = std::env::var("API_INTERNAL_KEY").unwrap_or("".to_string());

    let req = ureq::get(&url).header("X-Load-Auth-Token", api_internal_key);

    let response = req
        .call()
        .map_err(|e| anyhow::anyhow!("HTTP request failed: {}", e))?;

    response
        .into_body()
        .read_json::<T>()
        .map_err(|e| anyhow::anyhow!("Failed to parse JSON: {}", e))
}

pub fn extract_req_user(
    req: &HttpRequest,
) -> actix_web::Result<(CurrentUser, PermissionContainer), Error> {
    let extensions = req.extensions();

    let auth = extensions
        .get::<CurrentUser>()
        .map(|e| e.clone())
        .ok_or_else(|| ErrorUnauthorized("Invalid Credentials"))?;

    let permissions = extensions
        .get::<PermissionContainer>()
        .map(|e| e.clone())
        .ok_or_else(|| ErrorUnauthorized("Invalid Credentials"))?;

    Ok((auth, permissions))
}

pub fn ensure_permission(data: bool) -> Result<(), Error> {
    if !data {
        Err(ErrorUnauthorized("Invalid Permissions"))
    } else {
        Ok(())
    }
}
