# Session Plan：Issue #9 Gate 2 上游基线

schema_version: inputcodex.session-plan.v1
task_id: 2026-07-21-issue-9-gate-2-upstream-baseline
work_class: standard
task_status: completed-squash-merged
task_summary: 已完成上游 v1.2.41 完整只读快照的来源锁定、许可证、纯净性验证和 Squash Merge。
project_root: C:/Users/dashuai/Documents/inputcodex
tracking_issue_ref: https://github.com/nonononull/inputcodex/issues/9
branch_ref: codex/issue-9-upstream-sync
baseline_ref: 216d400006ad3f1dd2587ca367abb19d0191949f
decision_status: completed-squash-merged
approved_decision_ref: user-message:approve-issue-9-upstream-sync-2026-07-21, user-message:authorize-squash-merge-pr-11-2026-07-21
session_plan_ref: docs/plans/sessions/2026-07-21-issue-9-gate-2-upstream-baseline.md
implementation_plan_ref: docs/plans/2026-07-21-issue-9-gate-2-upstream-baseline.md
runtime_workflow_ref: docs/workflows/2026-07-21-issue-9-gate-2-upstream-baseline-runtime.md
mutation_intent: 已按批准范围写入 upstream/CodexPlusPlus、upstream/source-lock.json 与同步报告；未创建产品源码或 Workflow。
executor_enforcement: PR #11 的 279 条路径均位于 upstream/ 或同步报告，Squash tree 与获批 Head tree 完全一致。
external_agos_policy: available-optional-unavailable-bypass-no-optimization
external_agos_execution: not-needed-project-native-control-plane

## 硬约束

- 功能真源是上游最新正式 Release `v1.2.41`，不是上游 `main`。
- 快照只用于审计、映射和重构追踪，不参与产品构建。
- Tauri/React UI、注入脚本和远程推荐列表不得进入最终运行面。
- 许可证、来源提交和保留声明必须随快照记录。
- 快照同步和功能重构必须使用不同 Issue/PR。

## 开工门禁

- [x] 项目所有者批准允许写入路径。
- [x] `source-lock.json` 格式与哈希算法完成评审。
- [x] 快照纯净性验证命令可复现。
- [x] Windows 与 macOS 验证边界明确。
- [x] 不需要通过修改 AGOS 即完成交付。

## 交付证据

tracking_issue_ref: https://github.com/nonononull/inputcodex/issues/9
review_ref: https://github.com/nonononull/inputcodex/pull/11#issuecomment-5037841114
owner_merge_authorization_ref: https://github.com/nonononull/inputcodex/pull/11#issuecomment-5037940742
pr_ref: https://github.com/nonononull/inputcodex/pull/11
ci_ref: not-configured:gate-2-workflows-0
merge_ref: dde08b725eb2bf4add7fbcfa955f3eaf4eb1bbc6
closeout_issue_ref: https://github.com/nonononull/inputcodex/issues/12
