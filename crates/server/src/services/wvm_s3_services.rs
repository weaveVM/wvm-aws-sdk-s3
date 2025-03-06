use planetscale::PlanetScaleDriver;
use std::sync::Arc;

pub struct WvmS3Services {
    pub db_driver: Arc<PlanetScaleDriver>,
}

impl WvmS3Services {
    pub fn new(db_driver: Arc<PlanetScaleDriver>) -> Self {
        Self { db_driver }
    }
}
