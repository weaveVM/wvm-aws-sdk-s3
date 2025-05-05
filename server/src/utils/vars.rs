use std::sync::LazyLock;

pub static AUTH_HOST: LazyLock<String> = LazyLock::new(|| {
    std::env::var("AUTH_API").unwrap_or_else(|_| "https://load-auth-vskt.shuttle.app".to_string())
});

pub static DATA_RETRIEVAL_ENDPOINT: LazyLock<String> = LazyLock::new(|| {
    std::env::var("DATA_RETRIEVAL_ENDPOINT_API")
        .unwrap_or_else(|_| "https://gateway.wvm.network/bundle".to_string())
});
