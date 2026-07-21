# Session Plan：Issue #9 Gate 2 上游基线

schema_version: inputcodex.session-plan.v1
task_id: 2026-07-21-issue-9-gate-2-upstream-baseline
work_class: standard
task_status: planning-only
task_summary: 规划上游 v1.2.41 完整只读快照的来源锁定、许可证和纯净性验证。
project_root: C:/Users/dashuai/Documents/inputcodex
tracking_issue_ref: https://github.com/nonononull/inputcodex/issues/9
branch_ref: pending:issue-9-execution-branch
baseline_ref: c74b66422ba47f96bd3eb2b2385cdfb90541808e
decision_status: transition-approved-import-scope-pending
approved_decision_ref: user-message:authorize-pr7-merge-close-issue1-switch-gate2
session_plan_ref: docs/plans/sessions/2026-07-21-issue-9-gate-2-upstream-baseline.md
implementation_plan_ref: docs/plans/2026-07-21-issue-9-gate-2-upstream-baseline.md
runtime_workflow_ref: docs/workflows/2026-07-21-issue-9-gate-2-upstream-baseline-runtime.md
mutation_intent: 目前只进行规划、来源核验和验证设计；未经新批准不修改 upstream/ 或创建 source-lock.json。
executor_enforcement: 任何快照写入必须使用独立 upstream-sync Issue/PR，且 staged diff 只能包含允许路径。
external_agos_policy: available-optional-unavailable-bypass-no-optimization
external_agos_execution: not-needed-project-native-control-plane

## 硬约束

- 功能真源是上游最新正式 Release `v1.2.41`，不是上游 `main`。
- 快照只用于审计、映射和重构追踪，不参与产品构建。
- Tauri/React UI、注入脚本和远程推荐列表不得进入最终运行面。
- 许可证、来源提交和保留声明必须随快照记录。
- 快照同步和功能重构必须使用不同 Issue/PR。

## 开工门禁

- [ ] 项目所有者批准允许写入路径。
- [ ] `source-lock.json` 格式与哈希算法完成评审。
- [ ] 快照纯净性验证命令可复现。
- [ ] Windows 与 macOS 验证边界明确。
- [ ] 不需要通过修改 AGOS 才能继续。

## 交付证据

tracking_issue_ref: https://github.com/nonononull/inputcodex/issues/9
review_ref: pending:issue-9-owner-decision
pr_ref: none:execution-not-started
ci_ref: not-configured:gate-2-workflows-0
merge_ref: none:execution-not-started
