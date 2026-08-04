# Issue #155 Runtime Workflow：Review ref 与 post-merge 父提交绑定

## Runtime Metadata

- tracking_issue_ref: https://github.com/nonononull/inputcodex/issues/155
- session_plan_ref: docs/plans/sessions/2026-08-05-issue-155-gate-5-review-parent-binding-recovery.md
- approved_decision_ref: https://github.com/nonononull/inputcodex/issues/140#issuecomment-5182811601
- planning_freeze_ref: https://github.com/nonononull/inputcodex/issues/155#issuecomment-5182853973
- baseline_ref: main@f3e7d6f873f59399e71b602e1a9fbdee71760d64
- branch_ref: codex/issue-155-gate-5-review-parent-binding-recovery
- candidate_scope: 8
- candidate_scope_hash: sha256:16fed02331530651a28e824c1ff1382478511945b4efc6892f4da7b23914247e
- runtime_state: local-verification-green-delivery-pending

## Node Order

```text
fresh main
  -> Issue #155 and eight-path Planning Freeze
  -> production helper/gate RED
  -> exact PR review ref and parent SHA GREEN
  -> local exact-scope verification
  -> normal commit/push and non-Draft PR
  -> independent Final Head review
  -> Hosted CI / Performance / Artifact gates
  -> exact Head Squash and fresh-main verification
  -> reopen #140 / await-owner-decision
```

## Stop Gates

- 实际路径不等于冻结八路径，或出现第二 writer。
- policy、产品、Parity、Cargo、Workflow/Runner/Ruleset、Release/upstream/UI/AGOS 漂移。
- Release/main 基线漂移，或独立复审任一 Critical/Important。

命中任一停止门时关闭当前交付为 `DO_NOT_MERGE`，不得在同一 PR 继续修复。
