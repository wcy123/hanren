//! GGML - Tensor library for machine learning
//!
//! Rust implementation of GGML tensor operations and backends.
//! Modeled after llama.cpp/ggml/.

pub mod backend;
pub mod gguf;
pub mod quants;
pub mod tensor;

pub use backend::Backend;
pub use tensor::{DType, Shape, Tensor};

use std::fmt;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    Io(std::io::Error),
    InvalidFormat(String),
    Backend(String),
    InvalidShape(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Io(e) => write!(f, "I/O error: {}", e),
            Error::InvalidFormat(msg) => write!(f, "Invalid format: {}", msg),
            Error::Backend(msg) => write!(f, "Backend error: {}", msg),
            Error::InvalidShape(msg) => write!(f, "Invalid shape: {}", msg),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Error::Io(e) => Some(e),
            _ => None,
        }
    }
}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Error::Io(e)
    }
}
