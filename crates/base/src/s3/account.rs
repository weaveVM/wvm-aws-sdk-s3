use planetscale_driver::Database;
use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Default, Serialize, Deserialize, Database)]
pub struct AccountName {
    pub account_name: String,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, Database)]
pub struct AccountId {
    pub account_id: u64,
}
