use planetscale_driver::Database;
use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Default, Serialize, Deserialize, Database)]
    pub struct Account {
        pub account_name: String,
    }