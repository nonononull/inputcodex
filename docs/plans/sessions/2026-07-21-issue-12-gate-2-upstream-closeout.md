# Session Plan：Issue #12 Gate 2 上游基线收口

schema_version: inputcodex.session-plan.v1
task_id: 2026-07-21-issue-12-gate-2-upstream-closeout
work_class: standard
task_status: execution-approved
task_summary: 回写 PR #11 合并证据、当前验证入口和 Gate 2 最新控制面。
project_root: C:/Users/dashuai/Documents/inputcodex
tracking_issue_ref: https://github.com/nonononull/inputcodex/issues/12
branch_ref: codex/issue-12-gate-2-upstream-closeout
baseline_ref: dde08b725eb2bf4add7fbcfa955f3eaf4eb1bbc6
decision_status: execution-approved-closeout-only
approved_decision_ref: user-message:authorize-squash-merge-pr-11-2026-07-21, user-message:create-independent-closeout-issue-pr
session_plan_ref: docs/plans/sessions/2026-07-21-issue-12-gate-2-upstream-closeout.md
implementation_plan_ref: docs/plans/2026-07-21-issue-12-gate-2-upstream-closeout.md
runtime_workflow_ref: docs/workflows/2026-07-21-issue-12-gate-2-upstream-closeout-runtime.md
mutation_intent: 仅更新批准的项目控制文档与 closeout 报告，不修改 upstream、产品源码、Workflow、Ruleset 或 AGOS。
executor_enforcement: staged diff 必须完全属于 Issue #12 允许路径；upstream/ 相对 origin/main 必须为 0 差异。
external_agos_policy: available-optional-unavailable-bypass-no-optimization
external_agos_execution: skipped-not-a-project-gate-no-cross-repo-mutation

## 硬约束

- 软件名、纯 Rust/Iced、无广告、性能优先和双平台一致约束不变。
- 上游 `v1.2.41` 快照只作审计输入，不参与产品构建或最终运行面。
- 本任务不修改快照，不进入 Gate 3，不创建 CI 或发布资产。
- PR 合并方式只能是 Squash Merge；本 Session 不包含 closeout PR 的合并授权。
- Review 反馈必须确定根因、处理并 Fresh 验证后才能闭环。

## 开工门禁

- [x] PR `#11` 已按明确授权 Squash Merge。
- [x] Issue `#9` 已自动关闭。
- [x] `main` 指向 `dde08b725eb2bf4add7fbcfa955f3eaf4eb1bbc6`。
- [x] Issue `#12` 与独立分支已建立。
- [x] AGOS 不构成本任务门禁，也不在本仓修改。

## 交付证据

tracking_issue_ref: https://github.com/nonononull/inputcodex/issues/12
source_pr_ref: https://github.com/nonononull/inputcodex/pull/11
source_merge_ref: dde08b725eb2bf4add7fbcfa955f3eaf4eb1bbc6
review_ref: pending:issue-12-owner-review-pr13
pr_ref: https://github.com/nonononull/inputcodex/pull/13
ci_ref: not-configured:gate-2-workflows-0
merge_ref: none:closeout-pr-not-authorized
