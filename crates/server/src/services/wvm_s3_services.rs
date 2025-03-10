use crate::services::buckets::bucket_service::WvmBucketService;
use planetscale::PlanetScaleDriver;
use std::sync::Arc;

pub struct WvmS3Services<'a> {
    pub db_driver: Arc<PlanetScaleDriver>,
    pub bucket_service: Arc<WvmBucketService<'a>>,
}

impl<'a> WvmS3Services<'a> {
    pub fn new(
        db_driver: Arc<PlanetScaleDriver>,
        bucket_service: Arc<WvmBucketService<'a>>,
    ) -> Self {
        Self {
            db_driver,
            bucket_service,
        }
    }
}
