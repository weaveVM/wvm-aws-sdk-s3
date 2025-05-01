use crate::utils::auth::{get_internal_call, AccessKey};
use crate::utils::vars::AUTH_HOST;

pub struct AuthService;

impl AuthService {
    pub fn new() -> Self {
        Self
    }

    pub async fn verify_access(&self, authorization_header_key: String) -> Option<AccessKey> {
        let url = format!(
            "{}/internal/verify/{}",
            &*AUTH_HOST, authorization_header_key
        );

        get_internal_call(url).ok()
    }
}
