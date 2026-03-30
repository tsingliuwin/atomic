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

## 4. Wasm —— 通用连接器 (The Universal Connector)
在原子理论中，WebAssembly (Wasm) 被选为实现“语言自治”的核心连接协议。

### 4.1 什么是 Wasm？
Wasm 是一种高性能的、跨语言的、安全的二进制指令格式。如果说 Docker 是打包整台电脑，Wasm 就是打包一个**“乐高零件”**。

### 4.2 为什么原子理论必须依赖 Wasm？
1.  **极致隔离 (Sandboxing)**：每个原子运行在受限的沙箱内。即使某个原子代码存在 Bug（如内存溢出），也无法干扰宿主系统或其他原子。这为 AI 自动生成的代码提供了天然的安全屏障。
2.  **原生性能 (Near-Native Performance)**：与传统的虚拟机或容器不同，Wasm 的执行速度极度接近原生代码，确保了高频原子调用的性能损耗最小。
3.  **全语言自治 (Polyglot Support)**：无论原子是用 Rust、C++、Go、Python 还是 AssemblyScript 编写，最终都编译为标准的 `.wasm` 文件，实现了真正的“逻辑无关语法”。
4.  **轻量与极速启动 (Lightweight)**：Wasm 模块的启动仅需微秒级，这使得系统可以根据负载实时加载和卸载原子。

### 4.3 编排器的核心：标准插座
通过 Wasm，原子理论抹平了不同编程语言之间的内存鸿沟。编排器只需通过 **WIT (Wasm Interface Type)** 契约即可实现数据的无缝调度。

## 5. 跨语言编排：标准插座协议 (Atomic Connectors)
为实现“全语言自治”，系统建立统一的生命支持系统（The Host）：
- **Wasm 协议 (核心)**：利用 WebAssembly 作为异构原子的通用二进制载体。
- **Socket/StdIO 协议**：为无法 Wasm 化的原子提供流式通信链路。
- **FFI 契约**：执行跨语言的直接内存交互。

## 6. AI 自动开发闭环 (The Autonomous Loop)
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
编排器（Orchestrator / The Host）是系统的“神经中枢”，负责将离散的原子串联成复杂的业务流：

### 9.1 核心职责 (The Host Responsibilities)
- **生命周期管理**: 负责原子的加载、实例化（Instantiation）与销毁。
- **安全隔离**: 利用 Wasm 沙箱确保原子间的内存隔离，防止单个原子的 Bug 摧毁整个系统。
- **路由与链路 (The Linker)**: 根据业务逻辑，将“原子 A 的输出”导向“原子 B 的输入”。
- **性能调度**: 最小化异构原子间的数据传递开销，利用共享内存或高效指针传递。

### 9.2 连接机制 (Connecting Mechanisms)
- **接口定义 (WIT - Wasm Interface Type)**: 所有的异构原子必须通过标准的 IDL (如 WIT) 定义其导出函数与导入依赖，确保 AI 能够自动化生成桥接代码。
- **内存映射**: 编排器负责在宿主环境与 Wasm 沙箱之间进行复杂对象（如 String, JSON）的内存映射与拷贝。

### 9.3 编排器定义 (Registry Fields)
- **`id` (String)**: 编排器的唯一标识。
- **`composes` (Array<ID>)**: 声明该编排器所调用的原子 ID 列表及其依赖顺序。
- **`language` (String)**: 编排器的实现语言。**强烈建议使用 Rust** 作为宿主语言，以获得极致的并发控制与 Wasm 运行性能。
- **`description` (String)**: 描述该编排器实现的完整业务链路逻辑。


## 11. 原子自增长协议 (Atomic Self-Growth Protocol)
原子自增长（Autonomous Propagation）是系统从“被动进化”转向“主动扩张”的关键标志。AI 不仅是原子的维护者，更是新物种的“育种师”。

### 11.1 触发机制 (Growth Triggers)
- **高频模式识别**: 当 AI 发现超过 3 个原子中存在重复的逻辑片段时，必须自动提取该片段，生化为一个新的 `UTILITY` 或 `LOGIC` 原子。
- **意图缺口补偿**: 在编排新任务时，若发现现有原子库无法完全覆盖需求，AI 应主动“生长”出缺失的原子片段，并将其注册到系统中。
- **复杂度裂变驱动**: 当原子因进化而突破复杂度阈值时，裂变产生的碎片应被评估为独立的“子原子”，实现功能的分离与复用。

### 11.2 生长流程 (Propagation Workflow)
1. **识别与提取**: 定位冗余或缺失的逻辑。
2. **基因合成**: 编写新原子的代码、测试用例及其 AMS 元数据。
3. **生存选择 (Natural Selection)**: 新生原子必须通过独立的自证性测试（Self-Testing）。未通过测试的“畸变”原子将被系统自动丢弃。
4. **动态注册**: 将通过筛选的新原子写入本地 `ATOMIC_METADATA.json`，并建立其与父原子的溯源关系。

### 11.3 语义稳定性 (Semantic Stabilization)
- **命名建议**: AI 必须为新生原子提供具有高检索价值的 `id` 和 `description`。
- **知识沉淀**: 新生原子的出现应反馈给全局索引，提升系统处理同类意图的未来效率。

### 11.4 跨语言杂交 (Cross-Language Hybridization)
系统鼓励将一种语言的成熟逻辑，通过“自增长”机制翻译并生成另一种语言的等效原子，以优化特定运行环境下的执行性能。

## 12. 原子理论 2.0：轻量化演进准则 (Lightweight Evolution)
为了平衡工程严谨性与开发灵活性，系统引入 2.0 演进准则，旨在实现“严契约，轻结构”。

### 12.1 元数据副作用化 (Metadata as a Side-Effect)
- **核心理念**: `ATOMIC_METADATA.json` 不再是手动维护的源文件，而是代码逻辑的“副作用”产物。
- **文档即契约 (Doc-as-Contract)**: AI 在编写原子时，直接在代码注释中嵌入 AMS 规格。系统通过自动化工具扫描代码，动态生成并同步元数据。这消除了手动维护哈希和契约的沉重负担。

### 12.2 原子成熟度模型 (Atomic Maturity Model)
- **逻辑孵化区 (Incubation Zone)**: 新功能允许以“普通代码块”形式存在，不强制立即原子化。
- **原子化手术 (Atomization)**: 当代码满足以下任一条件时，触发强制原子化：
  1. 被跨模块调用超过 3 次。
  2. 逻辑分支或代码行数超过进化阈值（200 行）。
  3. 需要进行跨语言（如编译为 Wasm）编排。

### 12.3 透明编排与动态胶水 (Transparent Orchestration)
- **瞬时胶水 (Transient Glue)**: 对于临时的、非核心的业务流，AI 自动生成瞬时“胶水代码”完成调用，无需预先定义正式的 `Orchestrator`。
- **神经通路固化**: 只有当业务流稳定且具有高频复用价值时，才将其正式注册为编排器规格，作为系统的“长期记忆”。

### 12.4 原子簇与逻辑重力 (Atomic Clusters & Gravity)
- **物理聚集**: 逻辑相关度极高的原子可以物理上共存于同一目录（原子簇），以保持文件系统的整洁。
- **逻辑隔离**: 物理聚集不代表契约耦合。原子簇内的每个单元仍必须遵循独立的 AMS 契约和自证性测试。
