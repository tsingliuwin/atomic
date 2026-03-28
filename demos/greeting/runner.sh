#!/bin/bash
# demos/greeting/runner.sh
# Orchestrator ID: demo_runner

# 1. 模拟宿主获取输入数据
NAME="AtomicUser"
echo "⚛️ [Orchestrator] Starting Atomic Demo (Greeting)..."

# 2. 这里的逻辑通常会从 ATOMIC_METADATA.json 动态解析原子路径
# 这里为了 Demo 直观，直接调用 Rust 编译环境
ATOM_SRC="atoms/atom_greet_logic.rs"
BIN_NAME="./.tmp/greet_bin"

# 3. 模拟“加载与实例化” (Wasm 环境下为 wasmtime instantiate)
mkdir -p .tmp
rustc "$ATOM_SRC" -o "$BIN_NAME"

# 4. 模拟“连接与路由” (将 NAME 传入原子)
echo "⚛️ [Orchestrator] Calling Atom (atom_greet_logic) with input: '$NAME'..."
RESULT=$($BIN_NAME "$NAME")

# 5. 处理结果
echo "⚛️ [Orchestrator] Result Received: $RESULT"

# 6. 清理现场
rm -rf .tmp
echo "⚛️ [Orchestrator] Task Complete."
