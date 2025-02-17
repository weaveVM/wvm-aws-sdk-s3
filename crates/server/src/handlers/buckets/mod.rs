use actix_web::{
    delete, get, post, put,
    web::{Data, Json, Query},
    HttpRequest, HttpResponse, Result,
};
use std::sync::Arc;

// #[put("/{bucket}")]
async fn put_bucket(service: Data<Arc<WipfsServices>>, req: HttpRequest) {}
