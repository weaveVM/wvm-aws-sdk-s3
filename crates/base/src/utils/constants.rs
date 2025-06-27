use std::sync::LazyLock;

pub const WVM_RPC_URL: &str = "https://alphanet.load.network";

pub static UPLOADER: LazyLock<String> = LazyLock::new(|| {
    std::env::var("UPLOADER_API").unwrap_or_else(|_| "https://uploader.load.network".to_string())
});
