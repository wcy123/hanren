# HANREN

**Honed Adaptive Neural Runtime ENgine**

A standalone GGML runtime for LLM inference, targeting Llama 3.1 and Qwen 3.6 MoE models.

Inspired by [llama.cpp](https://github.com/ggml-org/llama.cpp) architecture.

## Architecture

Following llama.cpp's three-layer design:

```
┌─────────────────────────────────────────────────────────┐
│              User Tools (examples/)                     │
│              • hanren-cli (like llama-cli)              │
└─────────────────────────────────────────────────────────┘
                            ↓
┌─────────────────────────────────────────────────────────┐
│              Core Library (hanren/)                     │
│              • Model loading (GGUF)                     │
│              • Inference runtime                        │
│              • KV cache management                      │
│              (like llama.cpp/src/, llama.h)             │
└─────────────────────────────────────────────────────────┘
                            ↓
┌─────────────────────────────────────────────────────────┐
│         Computation Layer (ggml/)                       │
│         • Tensor operations                             │
│         • Backend implementations                       │
│           - CPU (SIMD)                                  │
│           - GPU (HIP-IR) - future                       │
│         (like llama.cpp/ggml/)                          │
└─────────────────────────────────────────────────────────┘
```

## Project Structure

```
hanren/
├── ggml/              # Tensor library (like llama.cpp/ggml/)
│   ├── src/
│   │   ├── backend.rs    # Backend trait + CPU impl
│   │   ├── tensor.rs     # Tensor types
│   │   ├── gguf.rs       # GGUF parser
│   │   └── quants.rs     # Quantization
│   └── Cargo.toml
│
├── hanren/            # LLM library (like llama.cpp/src/)
│   ├── src/
│   │   ├── model.rs      # Model loading
│   │   ├── runtime.rs    # Inference loop
│   │   └── kv_cache.rs   # KV cache
│   └── Cargo.toml
│
├── common/            # Common utilities (like llama.cpp/common/)
│   └── Cargo.toml
│
├── examples/
│   └── cli/           # CLI tool (like llama-cli)
│       └── Cargo.toml
│
└── Cargo.toml         # Workspace root
```

## Comparison with llama.cpp

| llama.cpp | HANREN | Purpose |
|-----------|--------|---------|
| `ggml/` | `ggml/` | Tensor library |
| `include/llama.h`, `src/` | `hanren/` | LLM library |
| `common/` | `common/` | Common utilities |
| `llama-cli` | `hanren-cli` | CLI tool |
| `llama-server` | (future) | Server |

## Features

- **llama.cpp-inspired architecture**: Clear separation between tensor ops (ggml/) and LLM logic (hanren/)
- **CPU Backend First**: Working baseline using SIMD (AVX2/NEON)
- **GGUF Support**: Load quantized models (Q4_0, Q4_K, Q8_0)
- **Layer-aware KV Cache**: Smart memory management across VRAM/RAM/NVMe tiers
- **Back-to-back Testing**: CPU results as golden reference for GPU correctness
- **HIP-IR Backend**: Future GPU acceleration via AMD's compiler IR

## Roadmap

### Phase 1: GGML Foundation (Current)
- [ ] Tensor abstractions (ggml/src/tensor.rs)
- [ ] Backend trait (ggml/src/backend.rs)
- [ ] CPU operations (matmul, RoPE, RMS norm, etc.)
- [ ] GGUF parser (ggml/src/gguf.rs)

### Phase 2: HANREN Library
- [ ] Model loading (hanren/src/model.rs)
- [ ] Llama 3.1 architecture
- [ ] Qwen 3.6 MoE architecture
- [ ] Basic inference loop (hanren/src/runtime.rs)

### Phase 3: KV Cache
- [ ] Layer-aware cache management
- [ ] Memory tier scheduling (VRAM → RAM → NVMe)
- [ ] Prefill/Decode optimization

### Phase 4: GPU Backend
- [ ] HIP-IR code generation (ggml/src/backend/gpu.rs)
- [ ] GPU kernel optimization
- [ ] Back-to-back verification against CPU

## Development Principles

1. **Follow llama.cpp architecture**: Clear layer separation (ggml → hanren → examples)
2. **Always Have a Working Revision**: CPU backend is the baseline, GPU is the acceleration
3. **Evidence-Based**: No speculation, verify through testing
4. **Rust-First**: Type safety, zero-cost abstractions, fearless concurrency

## Building

```bash
# Build all crates
cargo build --release

# Run CLI
cargo run --bin hanren

# Test
cargo test --workspace
```

## License

MIT License - Copyright (c) 2026 Wang Chunye

## References

- [llama.cpp](https://github.com/ggml-org/llama.cpp) - The reference implementation
- [GGML](https://github.com/ggml-org/ggml) - Tensor library
