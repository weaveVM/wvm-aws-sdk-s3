use crate::error::ErrorXmlFactory;
use crate::services::wvm_s3_services::WvmS3Services;
use crate::utils::auth::CurrentUser;
use actix_web::body::MessageBody;
use actix_web::dev::{ServiceRequest, ServiceResponse};
use actix_web::error::ErrorUnauthorized;
use actix_web::middleware::Next;
use actix_web::web::Data;
use actix_web::{Error, HttpMessage, HttpRequest};
use std::sync::Arc;

pub async fn auth_middleware(
    req: ServiceRequest,
    next: Next<impl MessageBody>,
) -> Result<ServiceResponse<impl MessageBody>, Error> {
    let service = req.app_data::<Data<Arc<WvmS3Services>>>().unwrap();
    let is_valid_key = check_user_auth(&req, &service).await.unwrap_or(false);
    if !is_valid_key {
        return Err(ErrorUnauthorized(ErrorXmlFactory::unauthorized().build()));
    }
    // Pre-processing

    // Call the next service in the middleware chain.
    let res = next.call(req).await?;

    // Post-processing
    Ok(res)
}

pub fn extract_aws_access_key(auth_header: &str) -> Result<String, actix_web::Error> {
    // Find `Credential=...`
    let credential = auth_header
        .split(' ')
        .find(|part| part.trim().starts_with("Credential="))
        .and_then(|part| {
            part.trim()
                .strip_prefix("Credential=")
                .and_then(|cred| cred.split('/').next()) // only keep access key
        })
        .ok_or_else(|| {
            ErrorUnauthorized(
                ErrorXmlFactory::unauthorized()
                    .message("Credential attribute is not present")
                    .build(),
            )
        })?;

    Ok(credential.to_string())
}

async fn check_user_auth<'a>(
    req: &ServiceRequest,
    service: &Data<Arc<WvmS3Services<'a>>>,
) -> Result<bool, Error> {
    let token = req
        .headers()
        .get("authorization")
        .and_then(|header| header.to_str().ok())
        .ok_or_else(|| ErrorUnauthorized(ErrorXmlFactory::unauthorized().build()))?;

    // We don't need bearer tokens here.
    let token = token.replace("Bearer ", "");
    let token = extract_aws_access_key(&token)?;

    let access_key = service.auth_service.verify_access(token.clone()).await;

    if let Some(access_key) = access_key {
        req.extensions_mut().insert(CurrentUser(access_key));
        Ok(true)
    } else {
        Ok(false)
    }
}
