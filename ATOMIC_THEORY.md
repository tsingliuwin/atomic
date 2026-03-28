# ⚛️ 原子理论 (Atomic Theory) - AI Native 软件工程宣言

## 1. 核心愿景：代码即意图 (Code as Intent)
原子理论不再视软件为死板的逻辑堆砌，而是视其为一种**活的、可演化的生物系统**。在 AI Native 时代，人类程序员的角色从“手工艺人”转变为“系统意志的播种者”。
- **机器首选 (Machine-First)**：代码的第一读者和操作者是 AI，人类可读性是契约完备后的副产品。
- **意图驱动 (Intent-Driven)**：软件开发的本质是人类意图向机器契约的精准转化。

## 2. 原子五大定律 (The DNA Laws)
1.  **单一性 (Single Responsibility)**：一个原子只执行一个原子级的逻辑。裂变是进化的常态。
2.  **契约性 (Strict Contract)**：原子必须有明确定义的输入、输出和元数据描述（AMS）。
3.  **自证性 (Self-Testing)**：原子必须具备自我修复的判准逻辑，测试用例即原子的生存环境。
4.  **溯源性 (Atomic Traceability)**：将 Git Commit 视为**基因序列**。每一次提交都是一次精准的基因突变，可回溯、可重放。
5.  **语言自治性 (Language Autonomy)**：打破语言霸权。AI 根据效率原则自由选择最适合的“表达方式”（Rust, Python, Wasm 等）。

## 3. 代码生物学：系统的自我演化 (Code Biology)
系统通过以下机制对抗**熵增 (Entropy)**，维持生命力：
- **基因突变 (Mutations)**：每一次原子优化都是一次尝试性的进化。
- **自然选择 (Selection)**：通过自动化测试的原子获得“生存权”并进入主干基因库。
- **原子裂变 (Fission)**：当原子过于臃肿（超过 200 行或逻辑分支过多）时，自动触发裂变。
- **原子杂交 (Hybridization)**：提取跨原子的重复片段，形成更强大的基础“干细胞”原子。

## 4. 跨语言编排：标准插座协议 (Atomic Connectors)
为实现“全语言自治”，系统建立统一的生命支持系统（The Host）：
- **Wasm 协议 (核心)**：利用 WebAssembly 作为异构原子的通用二进制载体。
- **Socket/StdIO 协议**：为无法 Wasm 化的原子提供流式通信链路。
- **FFI 契约**：执行跨语言的直接内存交互。

## 5. AI 自动开发闭环 (The Autonomous Loop)
AI 承担原子的全生命周期管理，实现“无人驾驶”式开发：
1.  **意图拆解**：根据用户需求，在原子索引图中定位或规划受影响的基因片段。
2.  **手术实现**：在沙箱中进行原子的编写或修改。
3.  **交叉自证**：编写异构语言的测试脚本，验证契约的跨语言有效性。
4.  **基因归档**：执行原子提交协议，同步更新 `ATOMIC_METADATA.json` 的哈希链。

## 6. 高级进化能力：基因修复 (Gene Repair)
当系统检测到功能退化时，AI 启动**差分进化分析**：
- **逻辑对比**：对比 `latest_commit_id` 与历史“健康”快照的逻辑差异。
- **精准修复**：识别并修复导致契约断裂的恶意突变。

## 7. 原子元数据架构 (Metadata Registry Protocol)
- **根模板 (The Root Template)**: 定义系统的“物种标准”。
- **本地实现 (Local Implementation)**: 记录特定子系统的“个体特征”。

## 8. 原子元数据规格书 (Atom Metadata Specification - AMS)
为确保 AI 能够精准解析与执行，原子元数据必须严格遵循以下规格定义：

### 8.1 核心身份 (Core Identity)
- **`id` (String)**: 必须以 `atom_` 开头，采用小写蛇形命名法。
- **`type` (Enum)**: `LOGIC`, `IO`, `MODEL`, `UI`, `UTILITY`。

### 8.2 实现与定位 (Implementation & Location)
- **`language` (String)**: 编写该原子所选用的编程语言（e.g., `rust`, `python`）。
- **`runtime` (String)**: 该原子运行所需的具体环境（e.g., `native`, `python3.11`, `wasmtime`）。
- **`path` (Path)**: 原子源文件相对于本地元数据文件的物理路径。
- **`fn_name` (String)**: 该原子的逻辑入口点。

### 8.3 契约定义 (Contract Definitions)
- **`inputs` / `outputs` (Array<String>)**: 
  - **格式**: `Type:Name` (e.g., `String:user_id`)。
  - **支持类型**: `String`, `Number`, `Boolean`, `JSON`, `Binary`, `WasmPtr`。

### 8.4 基因溯源 (Traceability & Genealogy)
- **`latest_commit_id` (Hash)**: 对应源文件最后一次逻辑变更的完整 Git 哈希。
- **`history_commits` (Array<Hash>)**: 按时间倒序排列的历史快照链。

## 9. 原子编排规格书 (Orchestrator Specification)
- **`composes` (Array<ID>)**: 声明编排器所调用的原子 ID 列表。
- **`language`**: 编排器本身的实现语言（通常为 `shell`, `rust` 或 `python`）。
