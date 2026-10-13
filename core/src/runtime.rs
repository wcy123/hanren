//! Inference runtime (like llama.cpp context and inference loop)

use crate::Result;

pub struct Runtime {
    // TODO: inference orchestration
}

impl Runtime {
    pub fn new() -> Self {
        Runtime {}
    }

    pub fn generate(&self, _prompt: &str) -> Result<String> {
        // TODO: inference loop
        todo!("Inference not implemented")
    }
}

impl Default for Runtime {
    fn default() -> Self {
        Self::new()
    }
}
