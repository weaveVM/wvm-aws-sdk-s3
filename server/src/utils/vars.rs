use std::sync::LazyLock;

pub static AUTH_HOST: LazyLock<String> = LazyLock::new(|| {
    std::env::var("AUTH_API").unwrap_or_else(|_| "https://load-auth-vskt.shuttle.app".to_string())
});
