//! KV cache management (layer-aware, multi-tier)

pub struct KvCache {
    // TODO: VRAM → RAM → NVMe tiering
}

impl KvCache {
    pub fn new() -> Self {
        KvCache {}
    }
}

impl Default for KvCache {
    fn default() -> Self {
        Self::new()
    }
}
