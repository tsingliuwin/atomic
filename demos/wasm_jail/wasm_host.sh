#!/bin/bash
# demos/wasm_jail/wasm_host.sh
# Orchestrator ID: wasm_host_simulator

echo "⚛️ [Wasm Host] Initializing Sandboxed Atomic Execution..."

# 1. 准备输入数据 (20 + 22)
INPUT_DATA="20 22"

# 2. 获取 Wasmtime 路径 (由于安装位置可能在 ~/.wasmtime)
WASMTIME_BIN="$HOME/.wasmtime/bin/wasmtime"
if [ ! -f "$WASMTIME_BIN" ]; then
    WASMTIME_BIN="wasmtime"
fi

# 3. 在沙箱中执行
# 观察点：原子内部没有文件访问权，它只能看到我们通过管道传进去的 stdin。
echo "⚛️ [Wasm Host] Executing atom_math_logic.wasm in jail with inputs: '$INPUT_DATA'..."
RESULT=$(echo "$INPUT_DATA" | "$WASMTIME_BIN" run demos/wasm_jail/atoms/atom_math_logic.wasm)

echo "⚛️ [Wasm Host] Result from Sandbox: $RESULT"
echo "⚛️ [Wasm Host] Sandbox Destroyed and Task Complete."
