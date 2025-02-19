use crate::actix_web_service::CustomShuttleActixWeb;
use crate::services::db_service::DbService;
use crate::services::wvm_s3_services::WvmS3Services;
use actix_web::get;
use actix_web::web::{Data, ServiceConfig};
use std::sync::Arc;

mod actix_web_service;
mod db;
mod handlers;
mod middleware;
mod services;

#[get("/")]
async fn hello_world() -> &'static str {
    "Hello World!"
}

async fn get_services(secrets: shuttle_runtime::SecretStore) -> Arc<WvmS3Services> {
    let db_service: Arc<DbService> = Arc::new(DbService::new(secrets.get("PG_URL").unwrap()).await);

    Arc::new(WvmS3Services::new(db_service))
}

// #[shuttle_runtime::main]
async fn main(
    #[shuttle_runtime::Secrets] secrets: shuttle_runtime::SecretStore,
) -> CustomShuttleActixWeb<impl FnOnce(&mut ServiceConfig) + Send + Clone + 'static> {
    let service_box = get_services(secrets).await;
    let config = move |cfg: &mut ServiceConfig| {
        cfg.app_data(Data::new(service_box));
        cfg.service(hello_world);
        //configure_app(cfg);
    };

    Ok(config.into())
}
