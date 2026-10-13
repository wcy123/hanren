//! HANREN - LLM inference library
//!
//! High-level LLM inference API (like llama.cpp/include/llama.h)

pub mod kv_cache;
pub mod model;
pub mod runtime;

pub use ggml;
pub use kv_cache::KvCache;
pub use model::Model;
pub use runtime::Runtime;

use std::fmt;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    Ggml(ggml::Error),
    ModelLoad(String),
    Inference(String),
    InvalidConfig(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Ggml(e) => write!(f, "GGML error: {}", e),
            Error::ModelLoad(msg) => write!(f, "Model load: {}", msg),
            Error::Inference(msg) => write!(f, "Inference: {}", msg),
            Error::InvalidConfig(msg) => write!(f, "Config: {}", msg),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Error::Ggml(e) => Some(e),
            _ => None,
        }
    }
}

impl From<ggml::Error> for Error {
    fn from(e: ggml::Error) -> Self {
        Error::Ggml(e)
    }
}

pub const VERSION: &str = env!("CARGO_PKG_VERSION");
