# ⚛️ 原子理论 (Atomic Theory) - 软件工程宣言

## 1. 核心定义
复杂系统是由无数个“原子（Atoms）”自由组合而成的。原子是功能的最小单位，**其内部实现对编程语言透明**。

## 2. 原子五大定律
1.  **单一性 (Single Responsibility)**：一个原子只做一件事。
2.  **契约性 (Strict Contract)**：原子必须有明确定义的输入、输出和元数据描述。
3.  **自证性 (Self-Testing)**：每一个原子必须附带自动生成的测试用例。
4.  **溯源性 (Atomic Traceability)**：每一次演变必须与唯一的 Git Commit ID 绑定。
5.  **语言自治性 (Language Autonomy)**：**[NEW]** AI 可根据任务属性选择最合适的语言编写原子。原子间通过统一的“标准插座”通信。

## 3. 原子提交规范 (Atomic Commit Protocol)
- **单原子原则**：一个 Commit 只能修改一个原子（及其对应的元数据和测试）。严禁混合提交。
- **语义化描述**：提交信息需包含 `[Atom:<id>] <Action>: <Reason>`。
- **元数据同步**：每次提交后，必须将最新的 Commit ID 写入 `ATOMIC_METADATA.json`。

## 4. 原子连接协议 (Atomic Connectors)
为实现多语言编排，原子必须遵循以下连接协议之一：
- **Wasm 协议 (推荐)**：原子被编译为 WebAssembly 模块，通过二进制标准接口调用。
- **Socket 协议**：原子通过本地高速套接字进行 JSON/Protobuf 通信。
- **FFI 协议**：基于 C-ABI 的跨语言直接调用。

## 5. 跨语言编排逻辑 (Cross-Language Orchestration)
- **调度者 (The Host)**：系统的“脊髓”，负责协调数据在异构原子间的流动。
- **标准接口描述**：使用类似 WIT (Wasm Interface Type) 或 IDL 的语言，定义跨语言调用的契约。

## 6. 高级进化能力 (Advanced Capabilities)
### 6.1 原子回放 (Atomic Replay)
AI 必须能够通过 `git show <commit_id>:<path>` 检索原子的任何历史快照。
- **用途**：理解逻辑演进、排查“功能退化”问题。
- **指令**：`Review history of atom_xxx from commit yyy to zzz.`

### 6.2 基因修复 (Gene Repair)
当原子出现 Bug 时，AI 应对照 `latest_commit_id` 和最近的一个“健康” `history_commits` 进行差分分析（Diff Analysis）。
- **逻辑**：寻找破坏原子契约或逻辑完整性的“基因突变”，并进行精准回滚或修复。

## 7. AI 自动开发闭环 (Autonomous Loop)
AI 承担原子的全生命周期管理，不再需要人工介入：
1.  **拆解/识别**：根据需求确定受影响的原子 ID。
2.  **手术实现**：编写原子代码与配套测试。
3.  **自证测试**：执行该原子的独立测试，失败则原地修复。
4.  **原子存档**：
    - `git add <file>`
    - `git commit -m "[Atom:<id>] <description>"`
    - 提取新 Hash，更新 `ATOMIC_METADATA.json` 的 `latest_commit_id` 和 `history_commits`。
5.  **语言选型**：AI 根据元数据中的 `type` 和 `performance_req` 自动选择语言。
6.  **交叉验证**：AI 编写一个语言 A 的原子，同时编写一个语言 B 的测试脚本来验证它，确保契约跨语言有效。

## 8. 进化规则 (Evolutionary Rules)
- **原子裂变**：当原子代码超过 200 行或包含两个以上的逻辑分支时，应考虑裂变。
- **原子杂交**：当多个原子出现重复的逻辑片段时，提取该片段形成新的基础原子。
- **动态生长**：当 LLM 发现高频重复的解析模式时，自动生成正则原子并注册到系统中。

## 9. 原子元数据架构 (Metadata Registry Protocol)
- **根模板 (The Root Template)**: 根目录下的 `ATOMIC_METADATA.json` 仅作为格式模板和标准定义，不记录具体业务原子。
- **本地实现 (Local Implementation)**: 每个子项目或 Demo 目录必须维护一份自己的 `ATOMIC_METADATA.json`，用于记录该作用域内的原子。
- **元数据字段**:
  - `latest_commit_id`: 当前生效的基因版本。
  - `history_commits`: 完整的演化历史哈希链。
  - `orchestrators`: 原子的编排逻辑索引。