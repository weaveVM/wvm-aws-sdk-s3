use crate::services::db_service::DbService;
use std::sync::Arc;

pub struct WipfsServices {
    pub db_service: Arc<DbService>,
}

impl WipfsServices {
    pub fn new(db_service: Arc<DbService>) -> Self {
        Self { db_service }
    }
}
