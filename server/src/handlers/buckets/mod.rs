use crate::services::wvm_s3_services::WvmS3Services;
use crate::utils::auth::{ensure_permission, extract_req_user};
use crate::utils::object::{extract_metadata, find_key_in_metadata, retrieve_object_bytes};
use crate::utils::time::to_rfc_7231_datetime;
use actix_web::error::{ErrorBadRequest, ErrorNotFound, ErrorUnauthorized, HttpError};
use actix_web::http::header::HeaderMap;
use actix_web::http::StatusCode;
use actix_web::web::{Bytes, ServiceConfig};
use actix_web::{
    delete, get, post, put, web,
    web::{Data, Json, Query},
    HttpRequest, HttpResponse, Result,
};
use base::error::s3_load_errors::S3LoadErrors;
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

#[get("/abcd")]
async fn abcd() -> &'static str {
    "Hello World!"
}

#[put("/{bucket}")]
async fn create_bucket<'a>(
    service: Data<Arc<WvmS3Services<'a>>>,
    info: web::Path<BucketInfo>,
    req: HttpRequest,
) -> Result<HttpResponse> {
    let (auth, permissions) = extract_req_user(&req)?;
    let bucket_name = &info.bucket;
    ensure_permission(permissions.bucket_can_create(bucket_name))?;

    let res = service
        .bucket_service
        .s3_client
        .create_bucket()
        .bucket(bucket_name)
        .send(auth.0.owner_id as u64)
        .await;

    if res.is_ok() {
        HttpResponse::Ok()
            .insert_header(("Location", format!("/{}", bucket_name)))
            .await
    } else {
        Err(ErrorUnauthorized(
            S3LoadErrors::BucketNotCreated.to_xml(Some(bucket_name.to_string()), None),
        ))
    }
}

#[delete("/{bucket}")]
async fn delete_bucket<'a>(
    service: Data<Arc<WvmS3Services<'a>>>,
    info: web::Path<BucketInfo>,
    req: HttpRequest,
) -> Result<HttpResponse> {
    let (auth, permissions) = extract_req_user(&req)?;
    let bucket_name = &info.bucket;
    ensure_permission(permissions.bucket_can_delete(bucket_name))?;
    let res = service
        .bucket_service
        .s3_client
        .delete_bucket()
        .bucket(bucket_name)
        .send(auth.0.owner_id as u64)
        .await;

    if let Ok(_) = res {
        HttpResponse::Ok()
            .status(StatusCode::from_u16(204).unwrap())
            .await
    } else {
        Err(ErrorUnauthorized(
            S3LoadErrors::BucketNotDeleted.to_xml(Some(bucket_name.to_string()), None),
        ))
    }
}

#[delete("/{bucket}/{key:.*}")]
async fn delete_object<'a>(
    service: Data<Arc<WvmS3Services<'a>>>,
    info: web::Path<BucketAndObjectInfo>,
    req: HttpRequest,
) -> Result<HttpResponse> {
    let (auth, permissions) = extract_req_user(&req)?;
    let bucket_name = &info.bucket;
    let key_name = &info.key;
    ensure_permission(permissions.bucket_can_write(bucket_name))?;
    let res = service
        .bucket_service
        .s3_client
        .delete_object()
        .bucket(bucket_name)
        .key(key_name)
        .send(auth.0.owner_id as u64)
        .await;

    let mut ok_resp = HttpResponse::Ok();
    let mut res_builder = ok_resp.status(StatusCode::from_u16(204).unwrap());

    if let Ok(_) = res {
        res_builder.await
    } else {
        res_builder
            .insert_header(("Error", "ObjectNotDeleted"))
            .await
    }
}

#[get("/{bucket}/{key:.*}")]
async fn get_object<'a>(
    service: Data<Arc<WvmS3Services<'a>>>,
    info: web::Path<BucketAndObjectInfo>,
    req: HttpRequest,
) -> Result<HttpResponse> {
    let (auth, permissions) = extract_req_user(&req)?;
    let bucket_name = &info.bucket;
    ensure_permission(permissions.bucket_can_write(bucket_name))?;
    let key_name = &info.key;
    println!("Key {} bucket {}", key_name, bucket_name);
    println!("{}", auth.0.owner_id);
    let res = service
        .bucket_service
        .s3_client
        .get_object()
        .bucket(bucket_name)
        .key(key_name)
        .send(auth.0.owner_id as u64)
        .await;

    println!("res {:?}", res);

    if let Ok(obj) = res {
        let metadata = extract_metadata(obj.metadata);
        let content_type = find_key_in_metadata(&metadata, "Content-Type".to_string())
            .unwrap_or_else(|| String::from("application/octet-stream"));

        let tx_hash = obj.tx_hash;
        let body = retrieve_object_bytes(&tx_hash);

        if let Some(body) = body {
            let mut resp = HttpResponse::Ok()
                .insert_header(("Content-Length", obj.size_bytes.to_string()))
                .insert_header(("ETag", tx_hash))
                .insert_header((
                    "Last-Modified",
                    to_rfc_7231_datetime(&obj.created_at)
                        .unwrap_or_else(|| String::from("Thu, 01 Jan 1970 00:00:00 GMT")),
                ))
                .content_type(content_type)
                .body(body);

            return Ok(resp);
        }
    }

    Err(ErrorNotFound(S3LoadErrors::NoSuchObject.to_xml(
        Some(format!("{}/{}", bucket_name, key_name)),
        None,
    )))
}

#[get("/")]
async fn list_buckets<'a>(
    service: Data<Arc<WvmS3Services<'a>>>,
    req: HttpRequest,
) -> Result<Json<Vec<Bucket>>> {
    let (auth, permissions) = extract_req_user(&req)?;
    let res = service
        .bucket_service
        .s3_client
        .list_buckets()
        .send(auth.0.owner_id as u64)
        .await;
    let res = res.map_err(|e| actix_web::error::ErrorNotFound(e))?;
    Ok(Json(res))
}

#[get("/{bucket}")]
async fn list_objects<'a>(
    service: Data<Arc<WvmS3Services<'a>>>,
    info: web::Path<BucketAndObjectInfo>,
    req: HttpRequest,
) -> Result<Json<Vec<Object>>> {
    let (auth, permissions) = extract_req_user(&req)?;
    let bucket_name = &info.bucket;
    ensure_permission(permissions.bucket_can_read(bucket_name))?;
    let key_name = &info.key;
    let res: std::result::Result<Vec<Object>, anyhow::Error> = service
        .bucket_service
        .s3_client
        .list_objects_v2()
        .send(auth.0.owner_id as u64)
        .await;
    let res = res.map_err(|e| {
        ErrorNotFound(S3LoadErrors::NoSuchBucket.to_xml(Some(bucket_name.to_string()), None))
    })?;
    Ok(Json(res))
}

#[put("/{bucket}/{key:.*}")]
async fn put_object<'a>(
    service: Data<Arc<WvmS3Services<'a>>>,
    info: web::Path<BucketAndObjectInfo>,
    body: Bytes,
    req: HttpRequest,
) -> Result<HttpResponse> {
    let headers = req.headers();
    let create_bucket_if_not_exists = headers
        .get("x-amz-meta-create-bucket-if-not-exists")
        .map(|e| e.to_str().unwrap() == "true")
        .unwrap_or(false);

    let is_folder = headers
        .get("x-amz-meta-is-folder")
        .map(|e| e.to_str().unwrap() == "true")
        .unwrap_or(false);

    let uploader_api = headers
        .get("x-amz-meta-uploader-api")
        .map(|e| e.to_str().unwrap())
        .map(|e| e.to_string());

    let (auth, permissions) = extract_req_user(&req)?;
    let bucket_name = &info.bucket;
    ensure_permission(permissions.bucket_can_write(bucket_name))?;
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
        .content_type(content_type)
        .send(
            auth.0.owner_id as u64,
            create_bucket_if_not_exists,
            is_folder,
            uploader_api,
            service.bucket_service.s3_client.clone(),
        )
        .await;

    match res {
        Ok(res) => {
            HttpResponse::Ok()
                .status(StatusCode::from_u16(200).unwrap())
                .insert_header(("ETag", res.tx_hash))
                .await
        }
        Err(e) => {
            eprintln!("Error PUT/OBJ handler. {:?}", e);
            Err(ErrorBadRequest(S3LoadErrors::ObjectNotCreated.to_xml(
                Some(format!("{}/{}", bucket_name, key_name)),
                None,
            )))
        }
    }
}

// App configuration function
pub fn configure_app_s3_endpoints(cfg: &mut ServiceConfig) {
    cfg.service(create_bucket)
        .service(delete_bucket)
        .service(delete_object)
        .service(get_object)
        .service(list_buckets)
        .service(list_objects)
        .service(put_object)
        .service(abcd);
}
