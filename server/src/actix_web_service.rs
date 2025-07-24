use crate::handlers::fs::{get_fs, get_fs_bucket};
use crate::middleware::auth_middleware;
use actix_web::middleware::NormalizePath;
use actix_web::{guard, web, HttpResponse};
use async_trait::async_trait;
use std::net::{SocketAddr, SocketAddrV4};
use std::str::FromStr;
use actix_web::web::PayloadConfig;

/// A wrapper type for a closure that returns an [actix_web::web::ServiceConfig] so we can implement
/// [shuttle_runtime::Service] for it.
#[derive(Clone)]
pub struct CustomActixWebService<F>(pub F);

async fn fs_root() -> HttpResponse {
    HttpResponse::Ok().body("FS")
}

#[async_trait]
impl<F> shuttle_runtime::Service for CustomActixWebService<F>
where
    F: FnOnce(&mut actix_web::web::ServiceConfig) + Send + Clone + 'static,
{
    async fn bind(mut self, addr: SocketAddr) -> Result<(), shuttle_runtime::Error> {
        // Start a worker for each cpu, but no more than 4.
        let worker_count = num_cpus::get().min(4);

        let server = actix_web::HttpServer::new(move || {
            actix_web::App::new()
                // 1️⃣  Everything under /fs goes in its own scope, and that
                //     scope is registered BEFORE `.configure(self.0.clone())`.
                .app_data(PayloadConfig::new((1024 * 1024) * 25))
                .service(
                    web::scope("/fs")
                        .route("", web::get().to(fs_root))
                        .route("/{bucket}", web::get().to(get_fs_bucket)) // GET /fs
                        .route("/{bucket}/{folder:.*}", web::get().to(get_fs)), // GET /fs/{bucket}/{…}
                )
                .configure(self.0.clone())
                .wrap(actix_web::middleware::from_fn(auth_middleware))
                .wrap(NormalizePath::trim())
        })
        .workers(worker_count)
        .bind(SocketAddr::from(SocketAddrV4::from_str("0.0.0.0:8000").unwrap()))?
        .run();

        server.await.map_err(shuttle_runtime::CustomError::new)?;

        Ok(())
    }
}

impl<F> From<F> for CustomActixWebService<F>
where
    F: FnOnce(&mut actix_web::web::ServiceConfig) + Send + Clone + 'static,
{
    fn from(service_config: F) -> Self {
        Self(service_config)
    }
}

pub type CustomShuttleActixWeb<F> = Result<CustomActixWebService<F>, shuttle_runtime::Error>;
