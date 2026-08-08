# Issue #176 Session Plan：source-lock 与 Release 身份完整性

## Session Metadata

- task_id: issue-176-source-lock-release-integrity
- work_class: standard
- decision_status: approved
- tracking_issue_ref: https://github.com/nonononull/inputcodex/issues/176
- approved_decision_ref: https://github.com/nonononull/inputcodex/issues/140#issuecomment-5226190361
- planning_freeze_ref: https://github.com/nonononull/inputcodex/issues/176#issuecomment-5226216846
- selected_business_path: gate-5/v1.2.45-closure/source-lock-release-integrity
- execution_profile: project-native-v1
- command_source: build.md
- baseline_ref: main@5a7465252b56f7e90673e72d3e02881ac9238141
- branch_ref: codex/issue-176-source-lock-release-integrity
- scope_count: 11
- scope_hash: sha256:973cd1dd29f70ab6cf3f8bd2c82f61c1673adf09eec7f15db3c76dbd93e97001
- mutation_intent: governance-only
- allowed_operations: exact eleven-path implementation, local verification, normal commit/push, non-Draft PR, review, exact Squash
- executor_enforcement: one-writer-serial-six-batch-review-hard-stop

## Bootstrap

使用 `karpathy-guidelines` 与项目原生流程。AGOS ReportOnly 返回 `v1-state-unavailable`，已按项目规则记录并绕过；
仓库没有 `.codegraph/`，未初始化索引。本 session 不修改 AGOS，也不复用 #175 分支或 worktree。

## Evidence Lanes

- test: PowerShell 严格 JSON、快照、许可证和远端 evidence 负例；Rust source-lock 结构负例
- build: PowerShell AST、`cargo fmt --check`；本机缺少 MSVC `link.exe`，Parity 定向运行交给 exact-head Hosted CI
- verification: CI 合同 `99/99`；离线/live Release Audit、自治策略与仓库政策通过；scope hash 与 Git 空白检查精确
- review: 双独立 Final Head 复审，当前 pending
- closeout: PR、Squash、fresh main 双 Workflow，当前 pending

## Ownership

当前唯一 writer 为本协调线程。历史失败分支、#132 与 #175 仅作只读证据；不得清理、重置或复用。
