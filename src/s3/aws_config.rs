use crate::utils::env_utils::get_env_key;

pub struct Config {
    pub private_key: String
}

impl Config {
    pub fn load_from_env() -> Self {
        let private_key = get_env_key("WVM_PK").unwrap();
        Self {
            private_key
        }
    }
}