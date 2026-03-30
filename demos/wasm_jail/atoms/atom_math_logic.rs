// demos/wasm_jail/atoms/atom_math_logic.rs
use std::io::{self, Read};

/// @atomic
/// id: atom_math_logic
/// type: LOGIC
/// language: rust
/// runtime: wasmtime
/// inputs: ["Int:a", "Int:b"]
/// outputs: ["Int:sum"]
/// @atomic

fn main() {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer).unwrap();
    
    // 我们模拟一个简单的求和逻辑
    let nums: Vec<i32> = buffer
        .split_whitespace()
        .map(|s| s.parse().unwrap_or(0))
        .collect();

    if nums.len() >= 2 {
        println!("{}", nums[0] + nums[1]);
    }
}
