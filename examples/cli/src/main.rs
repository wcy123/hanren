//! HANREN CLI (like llama-cli)

use ggml::backend::cpu::CpuBackend;

fn main() {
    env_logger::init();

    println!("🗡️  HANREN v{}", hanren::VERSION);
    println!("   Honed Adaptive Neural Runtime ENgine");
    println!();

    let backend = CpuBackend::new();
    println!("Backend: {}", backend.name());
    println!();

    println!("Architecture (following llama.cpp):");
    println!("  ggml/         - Tensor library (like llama.cpp/ggml/)");
    println!("  hanren/       - LLM library (like llama.cpp/src/)");
    println!("  common/       - Common utilities");
    println!("  examples/cli/ - This CLI tool (like llama-cli)");
    println!();

    println!("Next steps:");
    println!("  1. Implement CPU operations in ggml/");
    println!("  2. Add GGUF parser in ggml/");
    println!("  3. Build inference runtime in hanren/");
    println!("  4. GPU backend (HIP-IR) in ggml/");
}
