# Issue #159 Runtime Workflow：SHA 绝对终止恢复

## Runtime Metadata

- tracking_issue_ref: https://github.com/nonononull/inputcodex/issues/159
- session_plan_ref: docs/plans/sessions/2026-08-05-issue-159-gate-5-sha-terminal-anchor-recovery.md
- approved_decision_ref: https://github.com/nonononull/inputcodex/issues/140#issuecomment-5187810181
- planning_freeze_ref: https://github.com/nonononull/inputcodex/issues/159#issuecomment-5187842988
- baseline_ref: main@f3e7d6f873f59399e71b602e1a9fbdee71760d64
- branch_ref: codex/issue-159-gate-5-sha-terminal-anchor-recovery
- runtime_state: local-verification-green

## Node Order

```text
startup baseline
  -> Issue and eight-path Planning Freeze
  -> nine-finding production RED
  -> three SHA absolute-anchor GREEN
  -> local exact-scope verification
  -> non-Draft PR
  -> two independent Final Head reviews
  -> Hosted CI / Performance / Artifact gates
  -> exact Head Squash
  -> fresh main verification
  -> reopen #140 / await-owner-decision
```

## Stop Gates

- 实际路径不等于冻结八路径。
- policy、产品、Parity、Cargo、Workflow、Runner、Ruleset、Release/upstream/UI/AGOS 出现修改。
- writer 超过一个，或 main/Release 基线漂移。
- 独立复审出现任一 Critical/Important。

命中任一停止门时关闭当前 Issue/PR 为 `DO_NOT_MERGE`，重新打开 `#140`；不得在当前 PR 继续修复。

## Current Node

生产 RED、夹具纠正与最小 GREEN 已完成，完整治理合同为 `93/93`。当前只允许完成八路径本地门禁、普通提交/push、non-Draft PR 和 Final Head 交付。
