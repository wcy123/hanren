//! HANREN - LLM inference library
//!
//! High-level LLM inference API (like llama.cpp/include/llama.h)

pub mod kv_cache;
pub mod model;
pub mod runtime;

pub use hanren_ops as ops;
pub use kv_cache::KvCache;
pub use model::Model;
pub use runtime::Runtime;

use std::fmt;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    Ops(hanren_ops::Error),
    ModelLoad(String),
    Inference(String),
    InvalidConfig(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Ops(e) => write!(f, "Ops error: {}", e),
            Error::ModelLoad(msg) => write!(f, "Model load: {}", msg),
            Error::Inference(msg) => write!(f, "Inference: {}", msg),
            Error::InvalidConfig(msg) => write!(f, "Config: {}", msg),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Error::Ops(e) => Some(e),
            _ => None,
        }
    }
}

impl From<hanren_ops::Error> for Error {
    fn from(e: hanren_ops::Error) -> Self {
        Error::Ops(e)
    }
}

pub const VERSION: &str = env!("CARGO_PKG_VERSION");
