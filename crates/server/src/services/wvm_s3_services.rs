use crate::services::db_service::DbService;
use std::sync::Arc;

pub struct WvmS3Services {
    pub db_service: Arc<DbService>,
}

impl WvmS3Services {
    pub fn new(db_service: Arc<DbService>) -> Self {
        Self { db_service }
    }
}
