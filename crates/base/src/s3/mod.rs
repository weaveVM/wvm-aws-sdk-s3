use crate::s3::aws_config::Config;
use std::cell::OnceCell;
use std::sync::{Arc, OnceLock};

pub mod account;
pub mod aws_config;
pub mod bucket;
pub mod builders;
pub mod client;
pub mod object;

pub static S3_CONFIG: OnceLock<Arc<Config>> = OnceLock::new();
