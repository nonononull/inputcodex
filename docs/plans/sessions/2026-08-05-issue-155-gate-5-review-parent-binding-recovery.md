# Issue #155 Session Plan：Review ref 与 post-merge 父提交绑定

## Session Metadata

- task_id: issue-155-gate-5-review-parent-binding-recovery
- work_class: standard
- decision_status: approved
- tracking_issue_ref: https://github.com/nonononull/inputcodex/issues/155
- approved_decision_ref: https://github.com/nonononull/inputcodex/issues/140#issuecomment-5182811601
- planning_freeze_ref: https://github.com/nonononull/inputcodex/issues/155#issuecomment-5182853973
- selected_business_path: gate-5/review-parent-binding-recovery
- execution_profile: project-native-v1
- execution_contract: agos.execution-contract.v1
- command_source: build.md
- baseline_ref: main@f3e7d6f873f59399e71b602e1a9fbdee71760d64
- branch_ref: codex/issue-155-gate-5-review-parent-binding-recovery
- scope_count: 8
- scope_hash: sha256:16fed02331530651a28e824c1ff1382478511945b4efc6892f4da7b23914247e
- mutation_intent: governance-only
- allowed_operations: eight-path scripts/tests/docs, local verification, normal commit/push, non-Draft PR, review, exact Squash
- executor_enforcement: single-writer / exact-scope / review-hard-stop

## Evidence Lanes

- test: `81 pass / 4 fail` RED；最终 `CI_CONTRACT_GREEN passed=87`
- build: PowerShell AST、Policy、Repository Policy、Release Audit
- review: 精确 Final Head 独立双复审 pending
- verification: live Issue #155、八路径范围/hash、Git 空白检查
- closeout: PR、Hosted、Squash 与 fresh main pending

## External Controls

`superpowers:*` 当前会话未暴露，使用 `karpathy-guidelines` 与项目原生 TDD。AGOS 仅允许 report-only
Git checkpoint；不可用时按项目规则绕过，不修改 AGOS。#132/#133、#149/#150、#153/#154 全部只读。
