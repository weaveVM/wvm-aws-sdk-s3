use std::sync::Arc;
use actix_web::{get, HttpRequest, HttpResponse, web};
use actix_web::error::ErrorNotFound;
use actix_web::web::{Data, Json, ServiceConfig};
use serde::{Deserialize, Serialize};
use base::s3::object::Object;
use base::utils::planetscale::ps_get_file_system_structure;
use crate::handlers::buckets::BucketAndObjectInfo;
use crate::services::wvm_s3_services::WvmS3Services;
use crate::utils::auth::extract_req_user;

#[derive(Deserialize, Serialize)]
pub struct FsRequest {
    bucket: String,
    folder: Option<String>,
}

//#[get("/fs/{bucket}/{folder:.*}")]
pub async fn get_fs<'a>(
    service: Data<Arc<WvmS3Services<'a>>>,
    info: web::Path<FsRequest>,
    req: HttpRequest,
) -> actix_web::Result<Json<Vec<Object>>> {
    let load_only_folders = req
        .headers()
        .get("X-Load-Only-Folders")
        .and_then(|v| v.to_str().ok())   // turn HeaderValue → &str, ignore UTF-8 errors
        .unwrap_or("false");             // default when header missing or invalid
    let load_only_folders = load_only_folders.to_lowercase() == "true";

    let auth = extract_req_user(&req)?;
    let bucket_name = &info.bucket;

    let objects = ps_get_file_system_structure(service.db_driver.get_conn(), bucket_name, info.folder.clone(), auth.0.owner_id as u64, load_only_folders).await.map_err(|e| {
        ErrorNotFound(e)
    })?;
    Ok(Json(objects))
}

// App configuration function
pub fn configure_fs_endpoints(cfg: &mut ServiceConfig) {
    //cfg.service(get_fs);
}
