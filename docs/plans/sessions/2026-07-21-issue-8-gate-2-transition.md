# Session Plan：Issue #8 Gate 1→2 控制面过渡

schema_version: inputcodex.session-plan.v1
task_id: 2026-07-21-issue-8-gate-2-transition
work_class: standard
task_status: transition-merge-authorized
task_summary: 回写 PR #7 合并 closeout，并把项目控制面切换到 Gate 2 活动 Issue #9。
project_root: C:/Users/dashuai/Documents/inputcodex
tracking_issue_ref: https://github.com/nonononull/inputcodex/issues/8
active_gate_issue_ref: https://github.com/nonononull/inputcodex/issues/9
branch_ref: codex/issue-8-gate-2-transition
baseline_ref: c74b66422ba47f96bd3eb2b2385cdfb90541808e
decision_status: approved
approved_decision_ref: user-message:authorize-pr7-merge-close-issue1-switch-gate2
session_plan_ref: docs/plans/sessions/2026-07-21-issue-8-gate-2-transition.md
implementation_plan_ref: docs/plans/2026-07-21-issue-8-gate-2-transition.md
runtime_workflow_ref: docs/workflows/2026-07-21-issue-8-gate-2-transition-runtime.md
scope_hash: sha256:085e4272380212c1c5a99490996d21e814578995db58dccee0ca8249eec9e4d3
mutation_intent: 只更新项目治理文档与任务控制面，不导入快照、不创建产品源码、CI 或发布资产。
executor_enforcement: 只允许在 codex/issue-8-gate-2-transition 分支写入；过渡 PR 使用 Closes #8；禁止 Force Push 与绕过 main Ruleset。
external_agos_policy: available-optional-unavailable-bypass-no-optimization
external_agos_execution: skipped-by-project-native-boundary
transition_pr_ref: https://github.com/nonononull/inputcodex/pull/10

## 批准决策

- 允许 Squash Merge PR `#7`。
- 允许在 PR `#7` 合并后关闭筹备 Issue `#1`。
- 允许建立 Gate 2 控制面和开放的 upstream-sync Issue `#9`。
- 不等同于批准导入上游源码；Issue `#9` 仍需要新的执行范围确认。

## 允许写入

- `README.md`
- `build.md`
- `docs/plans/PROJECT-MASTER-PLAN.md`
- `docs/plans/2026-07-21-architecture-governance.md`
- `docs/plans/2026-07-21-issue-8-gate-2-transition.md`
- `docs/plans/2026-07-21-issue-9-gate-2-upstream-baseline.md`
- `docs/plans/sessions/2026-07-21-issue-8-gate-2-transition.md`
- `docs/plans/sessions/2026-07-21-issue-9-gate-2-upstream-baseline.md`
- `docs/workflows/2026-07-21-issue-8-gate-2-transition-runtime.md`
- `docs/workflows/2026-07-21-issue-9-gate-2-upstream-baseline-runtime.md`
- `docs/reports/issue-6-gate-1-finalization-closeout.md`
- `err.md`

## 禁止写入

- `upstream/`、`source-lock.json`、Cargo、Rust、TypeScript、JavaScript 业务代码和 WebView。
- `.github/workflows/`、Release、安装包和更新资产。
- GitHub Ruleset、`main` 历史和外部 AGOS。

## 关键检查点

- `startup-baseline`：本地与 GitHub `main` 为 `c74b6642...`，签名 payload 可重建。
- `after-docs`：控制面指向 Issue `#9`，Gate 2 尚未导入源码。
- `pre-pr`：Issue Forms YAML、禁止表面、Ruleset、上游 Release 和 Git diff 全部通过。
- `handoff`：过渡 PR 非 Draft、`Closes #8`、Review 对话闭环且不自动合并。
- `post-merge`：Issue `#8` 关闭，Issue `#9` 保持 OPEN，Master Plan active gate 为 Gate 2。

## 异常处理

- 先查 `err.md`，重复的 Git API、签名提交和 PowerShell 问题复用已有根因。
- 上游 Release 或提交变化时停止写入并重新决策。
- 任何要求修改 `upstream/` 的动作都转入 Issue `#9` 新 Session Plan，不在本过渡 PR 执行。

## 交付证据

tracking_issue_ref: https://github.com/nonononull/inputcodex/issues/8
active_gate_issue_ref: https://github.com/nonononull/inputcodex/issues/9
review_ref: owner-authorization:user-message-switch-gate2
pr_ref: https://github.com/nonononull/inputcodex/pull/10
ci_ref: not-configured:gate-2-transition-checks-0
merge_ref: authorized:squash-pr-10
