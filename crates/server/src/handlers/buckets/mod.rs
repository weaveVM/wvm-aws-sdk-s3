use crate::services::wvm_s3_services::WvmS3Services;
use actix_web::http::header::HeaderMap;
use actix_web::http::StatusCode;
use actix_web::web::Bytes;
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
async fn create_bucket<'a>(
    service: Data<Arc<WvmS3Services<'a>>>,
    info: web::Path<BucketInfo>,
    req: HttpRequest,
) -> Result<HttpResponse> {
    let bucket_name = &info.bucket;
    let res = service
        .bucket_service
        .s3_client
        .create_bucket()
        .bucket(bucket_name)
        .send()
        .await;

    if res.is_ok() {
        HttpResponse::Ok()
            .insert_header(("Location", format!("/{}", bucket_name)))
            .await
    } else {
        HttpResponse::InternalServerError().await
    }
}

#[delete("/{bucket}")]
async fn delete_bucket<'a>(
    service: Data<Arc<WvmS3Services<'a>>>,
    info: web::Path<BucketInfo>,
    req: HttpRequest,
) -> Result<HttpResponse> {
    let bucket_name = &info.bucket;
    let res = service
        .bucket_service
        .s3_client
        .delete_bucket()
        .bucket(bucket_name)
        .send()
        .await;

    if res.is_ok() {
        HttpResponse::Ok()
            .status(StatusCode::from_u16(204).unwrap())
            .await
    } else {
        HttpResponse::InternalServerError().await
    }
}

#[delete("/{bucket}/{key}")]
async fn delete_object<'a>(
    service: Data<Arc<WvmS3Services<'a>>>,
    info: web::Path<BucketAndObjectInfo>,
    req: HttpRequest,
) -> Result<HttpResponse> {
    let bucket_name = &info.bucket;
    let key_name = &info.key;
    let res = service
        .bucket_service
        .s3_client
        .delete_object()
        .bucket(bucket_name)
        .key(key_name)
        .send()
        .await;

    if res.is_ok() {
        HttpResponse::Ok()
            .status(StatusCode::from_u16(204).unwrap())
            .await
    } else {
        HttpResponse::InternalServerError().await
    }
}

#[get("/{bucket}/{key}")]
async fn get_object<'a>(
    service: Data<Arc<WvmS3Services<'a>>>,
    info: web::Path<BucketAndObjectInfo>,
    req: HttpRequest,
) -> Result<HttpResponse> {
    let bucket_name = &info.bucket;
    let key_name = &info.key;
    let res = service
        .bucket_service
        .s3_client
        .get_object()
        .bucket(bucket_name)
        .key(key_name)
        .send()
        .await;

    if let Ok(obj) = res {
        Ok(HttpResponse::Ok().json(obj))
    } else {
        HttpResponse::InternalServerError().await
    }
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
    body: Bytes,
    req: HttpRequest,
) -> Result<Json<Vec<u8>>> {
    let bucket_name = &info.bucket;
    let key_name = &info.key;
    let content_type = req
        .headers()
        .get("Content-Type")
        .map(|e| e.to_str().unwrap())
        .unwrap_or("");
    let res = service
        .bucket_service
        .s3_client
        .put_object()
        .bucket(bucket_name)
        .key(key_name)
        .body(body.to_vec())
        .content_type(content_type);
    Ok(Json(vec![]))
}
