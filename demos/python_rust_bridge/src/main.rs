// demos/python_rust_bridge/src/main.rs
use std::process::Command;

/// Orchestrator ID: rust_polyglot_host
/// Description: A Rust-based host that orchestrates cross-language atoms.
fn main() {
    let input = "Atomic theory is evolving fast!";
    println!("⚛️ [Rust Host] Starting orchestration...");
    println!("⚛️ [Rust Host] Input Text: '{}'", input);

    // 逻辑上这里应该从 ATOMIC_METADATA.json 动态读取，
    // 此处直接引用以直观展示跨语言调用逻辑。
    let atom_path = "atoms/atom_text_analyzer.py";

    println!("⚛️ [Rust Host] Calling Python Atom (atom_text_analyzer)...");
    
    // 模拟连接协议：通过 StdIO 传递数据并获取 JSON 结果
    let output = Command::new("python3")
        .arg(atom_path)
        .arg(input)
        .output()
        .expect("Failed to execute atom");

    if !output.status.success() {
        eprintln!("Error: {}", String::from_utf8_lossy(&output.stderr));
        return;
    }

    let result_json = String::from_utf8_lossy(&output.stdout);
    
    // 获取 Python 处理后的 JSON 数据并展示
    println!("⚛️ [Rust Host] Result Mapping (Python JSON -> Rust String):");
    println!("   > {}", result_json.trim());
    
    println!("⚛️ [Rust Host] Orchestration Task Complete.");
}
