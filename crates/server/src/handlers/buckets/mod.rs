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
async fn put_bucket<'a>(
    service: Data<Arc<WvmS3Services<'a>>>,
    info: web::Path<PutBucket>,
    req: HttpRequest,
) -> Result<Json<Vec<String>>> {
    let bucket_name = &info.bucket;
    let res = service
        .bucket_service
        .s3_client
        .create_bucket()
        .bucket(bucket_name);
    Ok(Json(vec![]))
}
