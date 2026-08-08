# Issue #176 Runtime Workflow：source-lock 与 Release 身份完整性

## Runtime Metadata

- tracking_issue_ref: https://github.com/nonononull/inputcodex/issues/176
- session_plan_ref: docs/plans/sessions/2026-08-08-issue-176-source-lock-release-integrity.md
- approved_decision_ref: https://github.com/nonononull/inputcodex/issues/140#issuecomment-5226190361
- planning_freeze_ref: https://github.com/nonononull/inputcodex/issues/176#issuecomment-5226216846
- baseline_ref: main@5a7465252b56f7e90673e72d3e02881ac9238141
- branch_ref: codex/issue-176-source-lock-release-integrity
- runtime_state: local-verification-complete-pr-pending

## Node Order

```text
startup baseline
  -> Issue #176 and eleven-path Planning Freeze
  -> strict source-lock and release identity RED/GREEN
  -> local exact-scope verification
  -> non-Draft PR
  -> exact-head Hosted CI
  -> two independent Final Head reviews
  -> exact Head Squash
  -> fresh main verification
  -> batch 2 exact-head checkout successor
```

## Stop Gates

- 实际路径不等于冻结十一路径，或 writer 超过一个。
- `upstream/source-lock.json`、快照、产品、Parity 数据、Cargo、其他 Workflow、Runner、Ruleset、Release、UI 或 AGOS 漂移。
- Final Head 独立复审出现任一 Critical/Important。

命中任一停止门时关闭当前交付为 `DO_NOT_MERGE`，不得在同一 PR 内修补 finding，也不得启动批次 2。

## Current Node

中断恢复后已复锁基线与十一路径冻结合同；严格 JSON、本地 Git 闭包、远端 evidence、archive/license 与
Rust 全结构负例均已实现。Release Audit 定向合同为 `6/6`，真实 `v1.2.44` 离线快照复算通过；本机因
缺少 MSVC `link.exe` 未启动 Rust 定向测试，该运行证据必须由 exact-head Hosted CI 提供。完整 CI 合同
`99/99`、离线/live Release Audit、自治策略、仓库政策、格式、空白与十一路径 scope hash 均已通过；当前
节点为 Final Head 提交、non-Draft PR 与 Hosted CI，交付仍 pending。
