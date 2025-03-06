use crate::services::wvm_s3_services::WvmS3Services;
use actix_web::{
    delete, get, post, put, web,
    web::{Data, Json, Query},
    HttpRequest, HttpResponse, Result,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Deserialize, Serialize)]
pub struct PutBucket {
    bucket: String,
}

#[put("/{bucket}")]
async fn put_bucket(
    service: Data<Arc<WvmS3Services>>,
    info: web::Path<PutBucket>,
    req: HttpRequest,
) -> Result<Json<Vec<String>>> {
    Ok(Json(vec![]))
}
