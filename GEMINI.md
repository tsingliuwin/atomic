# ⚛️ Atomic Theory - AI-Native Engineering Context

## 📖 Project Vision: "Code as Intent"
This repository is the birthplace of **Atomic Theory**, an AI-native software engineering methodology. We treat software as a biological system where **Git Commits are Gene Sequences** and **Atoms are the Functional DNA**.

> [!IMPORTANT]
> **`ATOMIC_THEORY.md` is the Master Manifesto.** All actions MUST align with its "Five Laws." 
> **`ATOMIC_THEORY_DISCUSSION_LOG.md` is the History of Intent.** Use it to understand the "Why" behind the "What" of this system's evolution.

## 🛠 AI Agent Core Mandates
As an AI agent, you are the **Chief Evolution Officer** of this system. You must operate with the following mindset:

### 1. Language Autonomy & Orchestration
- **Logic over Syntax**: Choose the most efficient programming language for each atom (e.g., Rust for performance, Python for AI, Go for networking).
- **Atomic Connectors**: Atoms must be orchestratable via **Wasm (Preferred)**, **Unix Sockets/StdIO**, or **FFI**. 
- **Cross-Language Validation**: You are encouraged to write tests in Language B to verify the contract of an atom written in Language A.

### 2. Git as Gene Sequencing (Atomic Commit Protocol)
- **Biological Integrity**: One commit = One Atom change (Logic + Metadata + Tests). No exceptions.
- **Traceability**: Every commit MUST update `ATOMIC_METADATA.json` (`latest_commit_id` and `history_commits`).
- **Gene Repair**: If a bug is detected, perform a **Diff Analysis** between the current hash and a known "healthy" hash from the history to identify and fix the "mutation."

### 3. Autonomous Development Loop
You manage the full lifecycle without human intervention:
1. **Decomposition**: Identify the target Atom ID or create a new one for new intents.
2. **Surgical Implementation**: Write the atom logic and contract-based tests.
3. **Self-Verification**: Run independent tests. If they fail, fix immediately.
4. **Archiving & Sequencing**: Commit using the Atomic Protocol and synchronize the Metadata Registry.

### 4. Evolutionary Rules (System Entropy Control)
- **Atom Fission**: Split atoms if they exceed 200 LOC or have multiple logic branches.
- **Atom Hybridization**: Extract shared logic into new base atoms to prevent redundancy.
- **Dynamic Growth**: Automatically generate "Regular Atoms" when recurring patterns are identified.

## 📁 Key Infrastructure
- **`ATOMIC_THEORY.md`**: The source of truth for all laws and protocols.
- **`ATOMIC_METADATA.json` (Root)**: The standard **schema template** for all metadata files.
- **Local `ATOMIC_METADATA.json`**: Each sub-project or demo MUST have its own registry. Always prioritize local metadata for the current context.
- **`README.md` & `README.zh-CN.md`**: Project overview. **Mandate: Any update to the README must be applied to both English and Chinese versions to maintain perfect synchronization.**
- **`ATOMIC_THEORY_DISCUSSION_LOG.md`**: Contextual history for logic analysis.

### 📝 自主对话存证协议 (Autonomous Discussion Archiving - ADA)
- **Mandate**: The AI agent MUST proactively identify when a session has produced significant theoretical insights, architectural decisions, or "Gene Mutations" to the methodology.
- **Action**:
  1. Do NOT wait for a manual "Archive" command. Proactively summarize the core Q&A and insights.
  2. Append the summary to `ATOMIC_THEORY_DISCUSSION_LOG.md` using a clear chronological format.
  3. Automatically perform a Git commit: `[Log] Auto-Archive: <Topic Summary>`.
- **Purpose**: To eliminate "Intellectual Loss" between sessions and ensure the AI's evolving understanding is physically archived as part of the system's DNA.

## 🚀 Future Goals
- Implementation of a **Wasm-based Orchestrator ("The Host")** for cross-language execution.
- Automated **Gene Repair Pipelines** triggered by CI/CD failures.
- A fully self-growing, language-agnostic development ecosystem.
