use crate::services::wvm_s3_services::WvmS3Services;
use actix_web::{
    delete, get, post, put, web,
    web::{Data, Json, Query},
    HttpRequest, HttpResponse, Result,
};
use base::s3::bucket::Bucket;
use base::s3::object::Object;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Deserialize, Serialize)]
pub struct BucketInfo {
    bucket: String,
}

#[derive(Deserialize, Serialize)]
pub struct BucketAndObjectInfo {
    bucket: String,
    key: String,
}

#[put("/{bucket}")]
async fn put_bucket<'a>(
    service: Data<Arc<WvmS3Services<'a>>>,
    info: web::Path<BucketInfo>,
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

#[delete("/{bucket}")]
async fn delete_bucket<'a>(
    service: Data<Arc<WvmS3Services<'a>>>,
    info: web::Path<BucketInfo>,
    req: HttpRequest,
) -> Result<Json<Vec<String>>> {
    let bucket_name = &info.bucket;
    let res = service
        .bucket_service
        .s3_client
        .delete_bucket()
        .bucket(bucket_name);
    Ok(Json(vec![]))
}

#[delete("/{bucket}/{key}")]
async fn delete_object<'a>(
    service: Data<Arc<WvmS3Services<'a>>>,
    info: web::Path<BucketAndObjectInfo>,
    req: HttpRequest,
) -> Result<Json<Vec<String>>> {
    let bucket_name = &info.bucket;
    let key_name = &info.key;
    let res = service
        .bucket_service
        .s3_client
        .delete_object()
        .bucket(bucket_name)
        .key(key_name);
    Ok(Json(vec![]))
}

#[get("/{bucket}/{key}")]
async fn get_object<'a>(
    service: Data<Arc<WvmS3Services<'a>>>,
    info: web::Path<BucketAndObjectInfo>,
    req: HttpRequest,
) -> Result<Json<Vec<String>>> {
    let bucket_name = &info.bucket;
    let key_name = &info.key;
    let res = service
        .bucket_service
        .s3_client
        .get_object()
        .bucket(bucket_name)
        .key(key_name);
    Ok(Json(vec![]))
}

#[get("/")]
async fn list_buckets<'a>(
    service: Data<Arc<WvmS3Services<'a>>>,
    req: HttpRequest,
) -> Result<Json<Vec<Bucket>>> {
    let res = service.bucket_service.s3_client.list_buckets().send().await;
    let res = res.map_err(|e| actix_web::error::ErrorNotFound(e))?;
    Ok(Json(res))
}

#[get("/{bucket}")]
async fn list_objects<'a>(
    service: Data<Arc<WvmS3Services<'a>>>,
    info: web::Path<BucketAndObjectInfo>,
    req: HttpRequest,
) -> Result<Json<Vec<Object>>> {
    let bucket_name = &info.bucket;
    let key_name = &info.key;
    let res: std::result::Result<Vec<Object>, anyhow::Error> = service
        .bucket_service
        .s3_client
        .list_objects_v2()
        .send()
        .await;
    let res = res.map_err(|e| actix_web::error::ErrorNotFound(e))?;
    Ok(Json(res))
}

#[put("/{bucket}/{key}")]
async fn put_object<'a>(
    service: Data<Arc<WvmS3Services<'a>>>,
    info: web::Path<BucketAndObjectInfo>,
    req: HttpRequest,
) -> Result<Json<Vec<u8>>> {
    let bucket_name = &info.bucket;
    let key_name = &info.key;
    let res = service
        .bucket_service
        .s3_client
        .get_object()
        .bucket(bucket_name)
        .key(key_name);
    Ok(Json(vec![]))
}
