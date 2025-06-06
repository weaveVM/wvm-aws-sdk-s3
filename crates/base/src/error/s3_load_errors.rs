use crate::error::ErrorXmlFactory;

#[derive(Clone, Debug)]
pub enum S3LoadErrors {
    Unauthorized,
    CredentialNotPresent,
    AuthorizationNotPresent,
    BucketNotDeleted,
    ObjectNotDeleted,
    NoSuchObject,
    BucketNotCreated,
    NoSuchBucket,
    ObjectNotCreated,
    BucketAlreadyExists,
    BucketAlreadyOwnedByYou,
}

impl S3LoadErrors {
    pub fn to_xml(&self, resource: Option<String>, request_id: Option<String>) -> String {
        let (code, message) = match self {
            S3LoadErrors::Unauthorized => ("NoSuchKey", "The specified key does not exist."),
            S3LoadErrors::CredentialNotPresent => {
                ("NoSuchKey", "Credential attribute was not found.")
            }
            S3LoadErrors::AuthorizationNotPresent => {
                ("NoSuchKey", "Authorization header was not found.")
            }
            S3LoadErrors::BucketNotDeleted => (
                "InvalidRequest",
                "Bucket could not be deleted and the reason is ambiguous.",
            ),
            S3LoadErrors::ObjectNotDeleted => (
                "InvalidRequest",
                "Object could not be deleted and the reason is ambiguous.",
            ),
            S3LoadErrors::NoSuchObject => ("NoSuchKey", "The requested object has not been found."),
            S3LoadErrors::BucketNotCreated => ("OperationAborted", "The bucket was not created"),
            S3LoadErrors::NoSuchBucket => ("InvalidBucketName", "The bucket was not found"),
            S3LoadErrors::ObjectNotCreated => (
                "InvalidRequest",
                "Object could not be created and the reason is ambiguous.",
            ),
            S3LoadErrors::BucketAlreadyExists => (
                "BucketAlreadyExists",
                "The requested bucket name is not available. The bucket namespace is shared by all users of the system. Specify a different name and try again."
            ),
            S3LoadErrors::BucketAlreadyOwnedByYou => (
                "BucketAlreadyOwnedByYou",
                "The bucket that you tried to create already exists, and you own it."
            )
        };

        ErrorXmlFactory::new()
            .code(code)
            .message(message)
            .resource(resource.unwrap_or_else(|| String::new()))
            .request_id(request_id.unwrap_or_else(|| String::new()))
            .build()
    }
}
