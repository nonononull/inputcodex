# Issue #165 Runtime Workflow：副作用准入矩阵 successor v4

## Runtime Metadata

- tracking_issue_ref: https://github.com/nonononull/inputcodex/issues/165
- session_plan_ref: docs/plans/sessions/2026-08-05-issue-165-gate-5-side-effect-admission-matrix-successor-v4.md
- approved_decision_ref: https://github.com/nonononull/inputcodex/issues/140#issuecomment-5193014446
- planning_freeze_ref: https://github.com/nonononull/inputcodex/issues/165#issuecomment-5193029156
- baseline_ref: main@5a7465252b56f7e90673e72d3e02881ac9238141
- branch_ref: codex/issue-165-gate-5-side-effect-admission-matrix-successor-v4
- runtime_state: local-green-commit-pending
- limits: 1 repository PR / 0 product deliveries / 1 writer

## Node Order

```text
startup baseline
  -> Issue and twenty-path Planning Freeze
  -> three successor terminal-state RED contracts
  -> admission model and repository RED
  -> strict 83-source matrix GREEN
  -> consumed tranche and successor terminal routing GREEN
  -> local exact-scope verification
  -> non-Draft PR
  -> independent Final Head reviews
  -> Hosted CI / Performance / Artifact gates
  -> exact Head Squash
  -> fresh main verification
  -> close #165 / reopen #140 / await-owner-decision
```

## Stop Gates

- 实际路径不等于冻结 20 路。
- 产品、Parity disposition/count、Cargo、Workflow、Runner、Ruleset、Release/upstream、UI/AGOS 出现修改。
- writer 超过一个，或 main/Release 基线漂移。
- 复用 predecessor 分支，或独立复审出现任一 Critical/Important。

命中任一停止门时关闭当前 Issue/PR 为 `DO_NOT_MERGE`，重开 `#140`；不得在当前 PR 继续修复。

## Current Node

fresh 基线、Issue、owner 决策、20 路 Planning Freeze、三类可信 RED 与全部本地 GREEN 已锁定。
当前节点是普通提交、push 和唯一 non-Draft PR；禁止 force push 与 GitHub 原生 auto-merge。
Final Head 复审出现任一 Critical/Important 时立即 hard stop，禁止在同一 PR 修补。
