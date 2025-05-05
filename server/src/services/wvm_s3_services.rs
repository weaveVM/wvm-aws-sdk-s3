use crate::services::auth_service::AuthService;
use crate::services::buckets::bucket_service::WvmBucketService;
use planetscale::PlanetScaleDriver;
use std::sync::Arc;

pub struct WvmS3Services<'a> {
    pub db_driver: Arc<PlanetScaleDriver>,
    pub bucket_service: Arc<WvmBucketService<'a>>,
    pub auth_service: Arc<AuthService>,
}

impl<'a> WvmS3Services<'a> {
    pub fn new(
        db_driver: Arc<PlanetScaleDriver>,
        bucket_service: Arc<WvmBucketService<'a>>,
        auth_service: Arc<AuthService>,
    ) -> Self {
        Self {
            db_driver,
            bucket_service,
            auth_service,
        }
    }
}
