# Session Plan：Issue #22 Gate 3 合并 closeout

schema_version: inputcodex.session-plan.v1
task_id: 2026-07-22-issue-22-gate-3-closeout
work_class: major
task_status: pr-review-ready-owner-merge-authorized
task_summary: 使用独立 Issue/PR 持久化 PR #21 与 Issue #19 的 Gate 3 合并证据，修正陈旧控制面，不修改产品、CI、upstream、Ruleset 或 AGOS。
project_root: C:/Users/dashuai/Documents/inputcodex
tracking_issue_ref: https://github.com/nonononull/inputcodex/issues/22
source_issue_ref: https://github.com/nonononull/inputcodex/issues/19
source_pr_ref: https://github.com/nonononull/inputcodex/pull/21
branch_ref: codex/issue-22-gate-3-closeout
baseline_ref: 0716ec0debcd3e059cc4ca88a072232841ca73b4
decision_status: approved-closeout-and-squash-merge-authorized
approved_decision_ref: user-message:create-gate3-closeout-through-squash-merge-2026-07-22
owner_merge_authorization_ref: user-message:create-gate3-closeout-through-squash-merge-2026-07-22
session_plan_ref: docs/plans/sessions/2026-07-22-issue-22-gate-3-closeout.md
task_plan_ref: docs/plans/2026-07-22-issue-22-gate-3-closeout.md
runtime_workflow_ref: docs/workflows/2026-07-22-issue-22-gate-3-closeout-runtime.md
report_ref: docs/reports/issue-22-gate-3-closeout.md
scope_hash: sha256:16760a8ce385b171b007451a43a3acb604a7b8ffc06b098b5482b8d803115ec8
mutation_intent: 只回写 Gate 3 已完成合并的治理、计划、报告、构建和排错证据；不改变产品行为、CI 合同或上游快照。
executor_enforcement: 每批次核对分支、14 条允许路径、受保护表面、Git 状态和 Fresh GitHub 证据；禁止直接写 main、Force Push 或管理员绕过。
delivery_contract: inputcodex.issue-pr-merge.v1
external_agos_policy: optional-bypassed-no-optimization
closeout_pr_ref: https://github.com/nonononull/inputcodex/pull/23
review_ref: github-pr-23-review-threads-0
ci_ref: https://github.com/nonononull/inputcodex/actions/runs/29921450017
merge_ref: pending-closeout-pr

## 一、批准决策

- 项目所有者明确要求完整执行“Issue → Session Plan → Runtime Workflow → 回写 Master Plan/合并证据 → PR → Review/CI → Squash Merge”。
- 当前消息同时构成本 Issue 范围内的方案批准和最终 Squash Merge 授权；若范围或 Head 漂移，授权自动失效。
- 采用独立 closeout PR；拒绝直接修改 `main`，拒绝只留 GitHub 评论。

## 二、Fresh 基线

- 本地与 `origin/main` 均为 `0716ec0debcd3e059cc4ca88a072232841ca73b4`，工作树干净。
- PR `#21` 为 `MERGED`，合并时间 `2026-07-22T12:25:59Z`，最终 Head `9a4a4425f2fb0d8235554d3e83577111ae34efcc`。
- Squash commit 只有一个父提交 `477d110a9b284e127af365f5278901bcfa69e093`；merge/head tree 均为 `4881ce609370f77181d9545474c029ab0c5d4972`，GitHub 签名 `valid`。
- Issue `#19` 于 `2026-07-22T12:26:00Z` 以 `COMPLETED` 关闭。
- 最终 PR 运行 `29918843397` 与合并后 `main` 运行 `29919596057` 均为六 Job 全绿，成功 Artifact 数均为 `0`。
- PR `#21` Review 对话为 `0`；Ruleset `19395456` active、无 bypass、审批数 `0`、必须解决对话、Squash-only。
- 具备合并权限的人类维护者仍只有 `nonononull`。

## 三、范围合同

```yaml
allowed_paths:
  - AGENTS.md
  - README.md
  - build.md
  - err.md
  - docs/plans/PROJECT-MASTER-PLAN.md
  - docs/plans/2026-07-21-rust-ci-offload-implementation-plan.md
  - docs/plans/sessions/2026-07-22-issue-19-gate-3-rust-workspace-ci.md
  - docs/workflows/2026-07-22-issue-19-gate-3-rust-workspace-ci-runtime.md
  - docs/reports/issue-19-gate-3-rust-workspace-ci.md
  - docs/reports/rust-ci-cold-baseline.md
  - docs/plans/2026-07-22-issue-22-gate-3-closeout.md
  - docs/plans/sessions/2026-07-22-issue-22-gate-3-closeout.md
  - docs/workflows/2026-07-22-issue-22-gate-3-closeout-runtime.md
  - docs/reports/issue-22-gate-3-closeout.md
allowed_path_count: 14
scope_hash: sha256:16760a8ce385b171b007451a43a3acb604a7b8ffc06b098b5482b8d803115ec8
```

## 四、允许操作

- 修正 Gate 3 状态、合并引用、Issue/PR/CI/Review/Ruleset 与签名证据。
- 修正 `AGENTS.md` 中“尚未导入应用源码”的陈旧边界，但继续锁定 Gate 4。
- 新增 closeout 计划、Session Plan、Runtime Workflow 与报告。
- 在 `err.md` 记录本轮真实网络和 CLI 参数错误。
- 运行本地轻量验证、GitHub Actions、Review/Fresh 门禁和已授权 Squash Merge。

## 五、禁止操作

- 修改任何 Cargo、Rust 产品、测试、`scripts/ci/`、Workflow、upstream、Ruleset、AGOS 或发布表面。
- 实现 Gate 4、Cache、性能优化、UI、功能迁移或一致性差异。
- Force Push、删除 `main`、管理员绕过、Merge Commit 或 Rebase Merge。

## 六、执行批次

1. **startup-baseline**：Fresh 验证来源 PR/Issue/CI/commit/Ruleset/维护者。
2. **issue-and-branch**：创建 Issue `#22` 与 closeout 分支。
3. **control-plane-writeback**：创建四份新控制面并更新十份既有文件。
4. **local-verification**：验证 14 条路径、受保护表面、AST、治理合同、仓库政策和空白。
5. **pr-review-ci**：普通提交/push、创建 PR、回写自引用、等待所有适用检查与 Review 闭环。
6. **authorized-squash-merge**：用已记录授权锁定最终 Head 执行 Squash Merge。
7. **post-merge-verification**：验证 Issue 关闭、单父/tree/签名、主干 CI、分支删除和本地 `main`。

## 七、验证合同

```yaml
local_commands:
  - pwsh -NoProfile -File scripts/ci/Test-CiScripts.ps1
  - pwsh -NoProfile -File scripts/ci/Verify-RepositoryPolicy.ps1 -RepositoryRoot .
  - PowerShell AST parse for four scripts/ci files
  - Issue #22 exact 14-path and protected-surface check
  - git diff --check
remote_evidence:
  - PR #21 merged and checks=6 success
  - Issue #19 CLOSED/COMPLETED
  - merge commit one parent and tree equals final Head
  - GitHub verification valid
  - main run 29919596057 jobs=6 success artifacts=0
  - Ruleset 19395456 unchanged
closeout_pr_gate:
  - all applicable checks successful
  - review threads zero
  - auto merge disabled
  - head matches authorized head
```

## 八、停止条件

- 任何修改超出 14 条路径或触及受保护表面。
- Fresh 证据无法复核、CI 失败、Review 对话未闭环或 Head 漂移。
- 需要修改授权范围、实现 Gate 4 或修改外部 AGOS。

## 九、PR #23 Review/Fresh checkpoint

- 候选 Head `956214529c5ea5fce9e70f70a6907d1d147fb2e3` 的运行 `29921450017` 成功；`classify`、`governance`、`required` 成功，Linux/Windows/macOS 按文档-only 合同合法跳过，成功 Artifact 数为 `0`。
- PR 为 OPEN、非 Draft、Merge State `CLEAN`、自动合并关闭；Review 对话为 `0`。
- Ruleset `19395456` active、无 bypass、审批数 `0`、必须解决对话、Squash-only；具备合并权限的人类维护者仍只有 `nonononull`。
- 变更路径恰好为 14 条，工作树干净；当前只剩最终状态封口提交、最终 Head CI 与已授权 Squash Merge。
