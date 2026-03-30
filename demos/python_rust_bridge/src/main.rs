// demos/python_rust_bridge/src/main.rs
use std::process::Command;

/// Orchestrator ID: rust_polyglot_host
/// Description: A Rust-based host that orchestrates a pipeline of Python atoms.
fn main() {
    let raw_input = "  ⚛️ Atomic theory is EVOLVING fast!!!  ";
    println!("⚛️ [Rust Host] Starting evolved orchestration pipeline...");
    println!("⚛️ [Rust Host] Raw Input: '{}'", raw_input);

    // 1. 调用新生的 Utility 原子进行数据清洗
    let cleaner_path = "atoms/atom_data_cleaner.py";
    println!("⚛️ [Rust Host] Step 1: Cleaning data via atom_data_cleaner...");
    let clean_output = Command::new("python3")
        .arg(cleaner_path)
        .arg(raw_input)
        .output()
        .expect("Failed to execute cleaner");
    
    let cleaned_text = String::from_utf8_lossy(&clean_output.stdout).trim().to_string();
    println!("⚛️ [Rust Host] Cleaned Text: '{}'", cleaned_text);

    // 2. 将清洗后的数据传递给 Logic 原子进行分析
    let analyzer_path = "atoms/atom_text_analyzer.py";
    println!("⚛️ [Rust Host] Step 2: Analyzing text via atom_text_analyzer...");
    let analyze_output = Command::new("python3")
        .arg(analyzer_path)
        .arg(&cleaned_text)
        .output()
        .expect("Failed to execute analyzer");

    let result_json = String::from_utf8_lossy(&analyze_output.stdout);
    
    // 3. 输出最终结果
    println!("⚛️ [Rust Host] Final Analysis Result (JSON):");
    println!("   > {}", result_json.trim());
    
    println!("⚛️ [Rust Host] Orchestration Pipeline Complete.");
}
