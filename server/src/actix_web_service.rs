use crate::middleware::auth_middleware;
use actix_web::middleware::NormalizePath;
use async_trait::async_trait;
use std::net::SocketAddr;

/// A wrapper type for a closure that returns an [actix_web::web::ServiceConfig] so we can implement
/// [shuttle_runtime::Service] for it.
#[derive(Clone)]
pub struct CustomActixWebService<F>(pub F);

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
                .configure(self.0.clone())
                .wrap(actix_web::middleware::from_fn(auth_middleware))
                .wrap(NormalizePath::trim())
        })
        .workers(worker_count)
        .bind(addr)?
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
