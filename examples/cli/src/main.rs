//! HANREN CLI (like llama-cli)

use hanren_ops::backend::cpu::CpuBackend;
use hanren_ops::Backend;

fn main() {
    env_logger::init();

    println!("🗡️  HANREN v{}", hanren::VERSION);
    println!("   Honed Adaptive Neural Runtime ENgine");
    println!();

    let backend = CpuBackend::new();
    println!("Backend: {}", backend.name());
    println!();

    println!("Architecture (following llama.cpp):");
    println!("  ops/          - Tensor library (like llama.cpp/ggml/)");
    println!("  hanren/       - LLM library (like llama.cpp/src/)");
    println!("  common/       - Common utilities");
    println!("  examples/cli/ - This CLI tool (like llama-cli)");
    println!();

    println!("Next steps:");
    println!("  1. Implement CPU operations in ops/");
    println!("  2. Add GGUF parser in ops/");
    println!("  3. Build inference runtime in hanren/");
    println!("  4. GPU backend (HIP-IR) in ops/");
}
