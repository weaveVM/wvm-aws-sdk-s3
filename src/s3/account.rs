use planetscale_driver::Database;
use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Default, Serialize, Deserialize, Database)]
pub struct Account {
    pub account_name: String,
}
