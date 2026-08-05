# Issue #159 Session Plan：SHA 绝对终止恢复

## Session Metadata

- task_id: issue-159-gate-5-sha-terminal-anchor-recovery
- work_class: standard
- decision_status: approved
- tracking_issue_ref: https://github.com/nonononull/inputcodex/issues/159
- approved_decision_ref: https://github.com/nonononull/inputcodex/issues/140#issuecomment-5187810181
- planning_freeze_ref: https://github.com/nonononull/inputcodex/issues/159#issuecomment-5187842988
- selected_business_path: gate-5/sha-terminal-anchor-recovery
- execution_profile: project-native-v1
- execution_contract: agos.execution-contract.v1
- command_source: build.md
- baseline_ref: main@f3e7d6f873f59399e71b602e1a9fbdee71760d64
- branch_ref: codex/issue-159-gate-5-sha-terminal-anchor-recovery
- scope_count: 8
- scope_hash: sha256:f56d3b5f45794641ddb1186b485bdc23c9c583068d15dbae61ce8f958225bddf
- mutation_intent: governance-only
- allowed_operations: eight-path docs/scripts/tests, local verification, normal commit/push, non-Draft PR, review, exact Squash
- executor_enforcement: single-writer-exact-scope-review-hard-stop

## Bootstrap

`superpowers:*` 当前会话未暴露，使用 `karpathy-guidelines` 与项目原生 TDD。仓库没有 `.codegraph/`，未初始化索引。AGOS default entry 的只读调用返回 `blocked / v1-state-unavailable`；按 inputcodex 项目规则记录并绕过，未修改 AGOS。私有 agent tool stack 声明路径不存在，同样作为外部 bootstrap 缺口绕过。

## Evidence Lanes

- test: 九个精确 RED；GREEN 为 `CI_CONTRACT_GREEN passed=93`
- build: 两份 PowerShell AST 零错误；其余治理门 pending fresh final run
- review: Final Head 双路独立只读复审 pending
- verification: Issue #159、Planning Freeze、fresh main 与单 writer 已锁定
- closeout: PR、Hosted CI、Squash 与 fresh main pending

## Ownership

当前唯一 writer 为父线程 `6564a52`。`#132/#133`、`#149/#150`、`#153/#154`、`#155/#156` 与 `#157/#158` 只允许只读引用；任何并发 writer 或范围外修改立即停止。
