use crate::actix_web_service::CustomShuttleActixWeb;
use crate::services::wvm_s3_services::WvmS3Services;
use actix_web::get;
use actix_web::web::{Data, ServiceConfig};
use base::s3::aws_config::Config;
use base::s3::client::Client;
use base::utils::wvm::derive_compressed_pubkey;
use planetscale::PlanetScaleDriver;
use std::sync::Arc;

mod actix_web_service;
mod handlers;
mod middleware;
mod services;

#[get("/")]
async fn hello_world() -> &'static str {
    "Hello World!"
}

async fn get_services(secrets: shuttle_runtime::SecretStore) -> Arc<WvmS3Services> {
    let driver = Arc::new(PlanetScaleDriver::from(&secrets));
    let private_key = secrets.get("WVM_AWS_S3_PK").unwrap();
    let address = derive_compressed_pubkey(&private_key).unwrap();
    let secret_access_key = secrets.get("SECRET_ACCESS_KEY").unwrap();

    let s3_client = Client::new(Some(&Config {
        private_key,
        wvm_rpc_url: secrets.get("SECRET_ACCESS_KEY").unwrap(),
        account_name: address,
        secret_access_key,
        account_id: None,
        db_driver: driver.clone(),
    }))
    .unwrap();

    Arc::new(WvmS3Services::new(driver))
}

#[shuttle_runtime::main]
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
