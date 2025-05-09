use crate::actix_web_service::CustomShuttleActixWeb;
use crate::handlers::buckets::configure_app_s3_endpoints;
use crate::services::auth_service::AuthService;
use crate::services::buckets::bucket_service::WvmBucketService;
use crate::services::wvm_s3_services::WvmS3Services;
use actix_web::get;
use actix_web::web::{Data, ServiceConfig};
use base::s3::aws_config::Config;
use base::s3::client::Client;
use base::utils::constants::WVM_RPC_URL;
use base::utils::wvm::derive_compressed_pubkey;
use planetscale::PlanetScaleDriver;
use shuttle_runtime::main;
use std::cell::OnceCell;
use std::sync::{Arc, OnceLock};

mod actix_web_service;
mod error;
mod handlers;
mod middleware;
mod services;
mod utils;

#[get("/")]
async fn hello_world() -> &'static str {
    "Hello World!"
}

static S3_CLIENT_CONFIG: OnceLock<Config> = OnceLock::new();

async fn get_services(secrets: shuttle_runtime::SecretStore) -> Arc<WvmS3Services<'static>> {
    let driver = Arc::new(PlanetScaleDriver::new(
        secrets.get("DATABASE_HOST").unwrap(),
        secrets.get("DATABASE_USERNAME").unwrap(),
        secrets.get("DATABASE_PASSWORD").unwrap(),
    ));

    let _ = S3_CLIENT_CONFIG.get_or_init(|| {
        let private_key = secrets.get("WVM_AWS_S3_PK").unwrap();
        let address = derive_compressed_pubkey(&private_key).unwrap();
        let secret_access_key = secrets.get("SECRET_ACCESS_KEY").unwrap();

        Config {
            private_key,
            wvm_rpc_url: WVM_RPC_URL.to_string(),
            account_name: address,
            secret_access_key,
            account_id: None,
            db_driver: driver.clone(),
        }
    });

    let s3_client = Client::new(Some(S3_CLIENT_CONFIG.get().unwrap())).unwrap();

    let s3_client = Arc::new(s3_client);

    let bucket_service = Arc::new(WvmBucketService {
        db_service: driver.clone(),
        s3_client: s3_client.clone(),
    });

    let auth_service = Arc::new(AuthService::new());

    Arc::new(WvmS3Services::new(driver, bucket_service, auth_service))
}

fn configure_env_vars(secrets: &shuttle_runtime::SecretStore) {
    unsafe {
        std::env::set_var(
            "API_INTERNAL_KEY",
            secrets.get("API_INTERNAL_KEY").unwrap_or("".to_string()),
        );

        std::env::set_var(
            "LOAD0_API_KEY",
            secrets.get("LOAD0_API_KEY").unwrap_or("".to_string()),
        );
    }
}

#[main]
async fn main(
    #[shuttle_runtime::Secrets] secrets: shuttle_runtime::SecretStore,
) -> CustomShuttleActixWeb<impl FnOnce(&mut ServiceConfig) + Send + Clone + 'static> {
    let aws_config = Config::load(
        secrets.get("WVM_AWS_S3_PK"),
        secrets.get("SECRET_ACCESS_KEY"),
    )
    .unwrap();

    configure_env_vars(&secrets);

    let service_box = get_services(secrets.clone()).await;
    let config = move |cfg: &mut ServiceConfig| {
        cfg.app_data(Data::new(service_box));
        cfg.service(hello_world);
        configure_app_s3_endpoints(cfg);
    };

    Ok(config.into())
}
