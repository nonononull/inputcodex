# Issue #145 Runtime Workflow：Gate 5 副作用准入矩阵

## Runtime Metadata

- task_id: issue-145-gate-5-side-effect-admission-matrix
- tracking_issue_ref: https://github.com/nonononull/inputcodex/issues/145
- session_plan_ref: docs/plans/sessions/2026-08-04-issue-145-gate-5-side-effect-admission-matrix.md
- approved_decision_ref: https://github.com/nonononull/inputcodex/issues/140#issuecomment-5168939607
- baseline_ref: main@42c73f401e7a758cdc5eca374613625dad46340b
- branch_ref: codex/issue-145-gate-5-side-effect-admission-matrix
- candidate_scope: 20
- candidate_scope_hash: sha256:b7955bb33dac2a5f58990dfbe2aff22cc6145a2b60e601e6de255bc0a8f4360f
- runtime_state: correction-local-verification-green

## Node Order

```text
startup baseline
  -> owner decision and tracking Issue
  -> twenty-path Planning Freeze
  -> control-plane RED/GREEN
  -> admission schema and matrix RED/GREEN
  -> repository validation integration
  -> local exact-scope closeout
  -> non-Draft PR and independent Final Head review
  -> hosted CI / Performance / Artifact gates
  -> exact Head Squash and main verification
  -> reopen #140 / await-owner-decision / STOP
```

## Allowed Operations

二十路径内的治理脚本、Parity admission 事实层、测试、文档、本地轻量验证、named Git checkpoint、普通 commit/push、non-Draft PR、只读复审和精确 Squash。

## Forbidden Operations

产品 Rust、Cargo 依赖、Workflow/Runner/Ruleset、Release/upstream/UI/AGOS、任何实际副作用执行、任何 Parity disposition/计数变化、任意路径/secret/SQLite/进程/网络、`unsafe`/FFI/VFS、auto-merge、force push，以及任何 #132/#133 写操作。

## Resume Algorithm

恢复时先运行 worktree `ReportOnly` 并核对 HEAD、merge-base、clean、#145、Release Audit 和 20 路 scope。dirty tree 只恢复当前 TDD 节点；范围、授权、Release、第二 writer 或目录计数漂移立即 hard-stop。完成或 hard-stop 后关闭 #145、重开 #140，并禁止自动选择产品候选。

## Current Node

三轮 Final Head 复审问题均已完成 RED/GREEN：22-feature owner/blocker oracle、model-catalog credential profile、#145 复合 terminal、PowerShell 原始 JSON 类型边界与 PR marker 均已纳入修正；Workflow/owner/base/issue number 身份、Workflow PR 关联、merged PR base、post-merge review evidence 与多 Issue hard-stop 也使用原始类型和对称终态门。完整治理合同为 `92/92`，Parity admission 为 `6/6`，目录为 `33/33`；当前只允许普通 correction commit/push 和新 Head 只读复审。未经新 Head 独立复审和 Hosted 双 Workflow，不得创建完成结论或合并。
