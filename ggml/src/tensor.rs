//! Tensor types and operations (like ggml/include/ggml.h)

use crate::{Error, Result};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DType {
    F32,
    F16,
    Q4_0,   // 4-bit quantized, block size 32
    Q4_K,   // 4-bit quantized, k-quants
    Q8_0,   // 8-bit quantized
}

impl DType {
    pub fn size_bytes(&self) -> usize {
        match self {
            DType::F32 => 4,
            DType::F16 => 2,
            DType::Q4_0 => 0,  // Block-based
            DType::Q4_K => 0,  // Block-based
            DType::Q8_0 => 1,
        }
    }

    pub fn is_quantized(&self) -> bool {
        matches!(self, DType::Q4_0 | DType::Q4_K | DType::Q8_0)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Shape(Vec<usize>);

impl Shape {
    pub fn new(dims: &[usize]) -> Self {
        Shape(dims.to_vec())
    }

    pub fn dims(&self) -> &[usize] {
        &self.0
    }

    pub fn rank(&self) -> usize {
        self.0.len()
    }

    pub fn elem_count(&self) -> usize {
        self.0.iter().product()
    }
}

pub struct Tensor {
    shape: Shape,
    dtype: DType,
    data: Vec<u8>,
}

impl Tensor {
    pub fn new(shape: Shape, dtype: DType) -> Result<Self> {
        let byte_count = if dtype.is_quantized() {
            return Err(Error::Backend(
                "Cannot create quantized tensor without data".to_string(),
            ));
        } else {
            shape.elem_count() * dtype.size_bytes()
        };

        Ok(Tensor {
            shape,
            dtype,
            data: vec![0; byte_count],
        })
    }

    pub fn from_raw(shape: Shape, dtype: DType, data: Vec<u8>) -> Result<Self> {
        Ok(Tensor { shape, dtype, data })
    }

    pub fn shape(&self) -> &Shape {
        &self.shape
    }

    pub fn dtype(&self) -> DType {
        self.dtype
    }

    pub fn data(&self) -> &[u8] {
        &self.data
    }

    pub fn data_mut(&mut self) -> &mut [u8] {
        &mut self.data
    }
}
