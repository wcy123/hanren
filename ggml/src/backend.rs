//! Backend trait and CPU implementation (like ggml/src/ggml-backend.cpp)

use crate::{tensor::Tensor, Result};

/// Backend trait for tensor operations
/// (Similar to ggml_backend_t in llama.cpp)
pub trait Backend: Send + Sync {
    fn name(&self) -> &str;

    // Core operations
    fn matmul(&self, a: &Tensor, b: &Tensor, out: &mut Tensor) -> Result<()>;
    fn add(&self, a: &Tensor, b: &Tensor, out: &mut Tensor) -> Result<()>;
    fn mul(&self, a: &Tensor, b: &Tensor, out: &mut Tensor) -> Result<()>;
    fn rms_norm(&self, x: &Tensor, weight: &Tensor, eps: f32, out: &mut Tensor) -> Result<()>;
    fn rope(
        &self,
        x: &Tensor,
        freqs_cos: &Tensor,
        freqs_sin: &Tensor,
        out: &mut Tensor,
    ) -> Result<()>;
    fn softmax(&self, x: &Tensor, dim: usize, out: &mut Tensor) -> Result<()>;
    fn silu(&self, x: &Tensor, out: &mut Tensor) -> Result<()>;
}

/// CPU backend (like ggml/src/ggml-cpu/)
pub mod cpu {
    use super::*;
    use crate::Error;

    pub struct CpuBackend;

    impl CpuBackend {
        pub fn new() -> Self {
            CpuBackend
        }
    }

    impl Default for CpuBackend {
        fn default() -> Self {
            Self::new()
        }
    }

    impl Backend for CpuBackend {
        fn name(&self) -> &str {
            "cpu"
        }

        fn matmul(&self, _a: &Tensor, _b: &Tensor, _out: &mut Tensor) -> Result<()> {
            Err(Error::Backend("matmul not implemented yet".to_string()))
        }

        fn add(&self, _a: &Tensor, _b: &Tensor, _out: &mut Tensor) -> Result<()> {
            Err(Error::Backend("add not implemented yet".to_string()))
        }

        fn mul(&self, _a: &Tensor, _b: &Tensor, _out: &mut Tensor) -> Result<()> {
            Err(Error::Backend("mul not implemented yet".to_string()))
        }

        fn rms_norm(&self, _x: &Tensor, _weight: &Tensor, _eps: f32, _out: &mut Tensor) -> Result<()> {
            Err(Error::Backend("rms_norm not implemented yet".to_string()))
        }

        fn rope(
            &self,
            _x: &Tensor,
            _freqs_cos: &Tensor,
            _freqs_sin: &Tensor,
            _out: &mut Tensor,
        ) -> Result<()> {
            Err(Error::Backend("rope not implemented yet".to_string()))
        }

        fn softmax(&self, _x: &Tensor, _dim: usize, _out: &mut Tensor) -> Result<()> {
            Err(Error::Backend("softmax not implemented yet".to_string()))
        }

        fn silu(&self, _x: &Tensor, _out: &mut Tensor) -> Result<()> {
            Err(Error::Backend("silu not implemented yet".to_string()))
        }
    }
}

// Future: GPU backend (like ggml/src/ggml-cuda/, ggml-hip/, etc.)
// pub mod gpu { ... }
