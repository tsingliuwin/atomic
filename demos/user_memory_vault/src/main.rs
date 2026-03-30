// demos/user_memory_vault/src/main.rs
use std::process::Command;
use std::fs;

/// Orchestrator ID: rust_vault_orchestrator
/// Description: A high-performance Rust host that processes thousands of user memory blocks through an atomic pipeline.
fn main() {
    let raw_data_path = "data/sample_users.json";
    println!("⚛️ [Vault Host] Initializing User Memory Vault...");

    // 1. 模拟读取百万级用户数据（此处用一个样本 JSON）
    let users_json = r#"[
        {"uid": "user_001", "content": "I love coding in Rust and building AI with Atomic Theory!"},
        {"uid": "user_002", "content": "Traveling the world and investing in the stock market."},
        {"uid": "user_003", "content": "Atomic systems are evolving so fast, it's powerful."}
    ]"#;
    
    let users: serde_json::Value = serde_json::from_str(users_json).unwrap();
    let users_list = users.as_array().unwrap();

    println!("⚛️ [Vault Host] Processing {} User Records in parallel...", users_list.len());

    // 2. 模拟原子流水线调度
    for user in users_list {
        let uid = user["uid"].as_str().unwrap();
        let content = user["content"].as_str().unwrap();

        println!("--- [Processing: {}] ---", uid);

        // Step A: Ingest
        let ingest_out = Command::new("python3")
            .arg("atoms/atom_user_ingestor.py")
            .arg(content)
            .output().unwrap();
        let normalized = String::from_utf8_lossy(&ingest_out.stdout);

        // Step B: Extract Traits
        let trait_out = Command::new("python3")
            .arg("atoms/atom_trait_extractor.py")
            .arg(normalized.trim())
            .output().unwrap();
        let traits = String::from_utf8_lossy(&trait_out.stdout);

        // Step C: Synthesize Persona
        let persona_out = Command::new("python3")
            .arg("atoms/atom_persona_synthesizer.py")
            .arg(normalized.trim())
            .arg(traits.trim())
            .output().unwrap();
        let persona = String::from_utf8_lossy(&persona_out.stdout);

        // 3. 输出最终结果
        println!("⚛️ [Vault Host] Persona Synthesized for {}:", uid);
        println!("   > {}", persona.trim());
    }

    println!("⚛️ [Vault Host] All User Personas Synthesized successfully.");
}
