use crate::actix_web_service::CustomShuttleActixWeb;
use actix_web::web::ServiceConfig;

mod actix_web_service;
mod handlers;
mod middleware;
mod services;

// #[shuttle_runtime::main]
async fn main(
    #[shuttle_runtime::Secrets] secrets: shuttle_runtime::SecretStore,
) -> CustomShuttleActixWeb<impl FnOnce(&mut ServiceConfig) + Send + Clone + 'static> {
    let service_box = get_services(secrets).await;
    let config = move |cfg: &mut ServiceConfig| {
        cfg.app_data(Data::new(service_box));
        cfg.service(hello_world);
        configure_app(cfg);
    };

    Ok(config.into())
}
