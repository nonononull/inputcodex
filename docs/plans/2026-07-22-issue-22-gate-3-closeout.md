# Issue #22：Gate 3 合并 closeout 计划

schema_version: inputcodex.task-plan.v1
task_status: control-plane-implementation-in-progress
tracking_issue_ref: https://github.com/nonononull/inputcodex/issues/22
source_issue_ref: https://github.com/nonononull/inputcodex/issues/19
source_pr_ref: https://github.com/nonononull/inputcodex/pull/21
branch_ref: codex/issue-22-gate-3-closeout
baseline_ref: 0716ec0debcd3e059cc4ca88a072232841ca73b4
approved_decision_ref: user-message:create-gate3-closeout-through-squash-merge-2026-07-22
owner_merge_authorization_ref: user-message:create-gate3-closeout-through-squash-merge-2026-07-22
scope_hash: sha256:16760a8ce385b171b007451a43a3acb604a7b8ffc06b098b5482b8d803115ec8
allowed_path_count: 14

## 一、背景

- PR `#21` 已于 `2026-07-22T12:25:59Z` 由 `nonononull` Squash Merge，Issue `#19` 已于 `2026-07-22T12:26:00Z` 按 `COMPLETED` 关闭。
- Gate 3 七成员纯 Rust Workspace、首版无缓存三平台 CI、五类失败语义和三平台各 `3/3` 次冷构建最低基线已经进入 `main`。
- 仓库内仍存在 `merge_ref: pending`、等待授权、Draft PR 和“尚未导入应用源码”等陈旧控制面；GitHub 评论不能替代仓库单一真源。
- 本任务使用独立 Issue/PR 收口，禁止直接写 `main`，不进入 Gate 4。

## 二、方案决策

1. 直接修改 `main`：违反 Issue/PR 治理，拒绝。
2. 只保留 GitHub 评论：无法修正仓库内陈旧状态，拒绝。
3. 独立 closeout Issue/PR：最小、可审计、可回滚，采用。

项目所有者已明确授权从创建 Issue 到最终 Squash Merge 的完整交付链；该授权只覆盖本计划冻结的 14 条路径。

## 三、目标

1. 持久化 PR `#21` 的最终 Head、merge commit、唯一父提交、tree 等价、合并时间和有效 GitHub 签名。
2. 持久化 Issue `#19` 的 `COMPLETED` 关闭、最终 PR CI、合并后 `main` CI、Review、Ruleset 与远端分支删除证据。
3. 修正 `AGENTS.md`、`README.md`、`build.md`、`err.md` 与 Master Plan 的 Gate 3 状态。
4. 封存 Issue `#19` 的 Session Plan、Runtime Workflow、实施报告和冷构建报告。
5. 建立本 closeout 的计划、Session Plan、Runtime Workflow 与报告。
6. 记录本轮 HTTPS fetch 和 `gh pr edit` 多行参数错误的根因与恢复方式。

## 四、允许路径

1. `AGENTS.md`
2. `README.md`
3. `build.md`
4. `err.md`
5. `docs/plans/PROJECT-MASTER-PLAN.md`
6. `docs/plans/2026-07-21-rust-ci-offload-implementation-plan.md`
7. `docs/plans/sessions/2026-07-22-issue-19-gate-3-rust-workspace-ci.md`
8. `docs/workflows/2026-07-22-issue-19-gate-3-rust-workspace-ci-runtime.md`
9. `docs/reports/issue-19-gate-3-rust-workspace-ci.md`
10. `docs/reports/rust-ci-cold-baseline.md`
11. `docs/plans/2026-07-22-issue-22-gate-3-closeout.md`
12. `docs/plans/sessions/2026-07-22-issue-22-gate-3-closeout.md`
13. `docs/workflows/2026-07-22-issue-22-gate-3-closeout-runtime.md`
14. `docs/reports/issue-22-gate-3-closeout.md`

## 五、禁止范围

- 不修改 Cargo、Rust 产品源码、测试、`scripts/ci/` 或 `.github/workflows/`。
- 不修改 `upstream/`、`source-lock.json`、Issue `#16/#20`、Ruleset、Release、签名、安装包或更新资产。
- 不实现 Gate 4 功能目录、业务功能、Cache、性能优化、UI 或一致性差异。
- 不修改或优化 AGOS。
- 不使用 Force Push、管理员绕过、Merge Commit 或 Rebase Merge。

## 六、执行步骤

1. Fresh 核对 `main`、PR `#21`、Issue `#19`、运行 `29919596057`、Ruleset 和维护者数量。
2. 创建 Issue `#22` 与 `codex/issue-22-gate-3-closeout` 分支。
3. 创建本计划、Session Plan、Runtime Workflow 与 closeout 报告。
4. 更新十份既有控制面，只修改 14 条允许路径。
5. 运行路径、内容、AST、治理合同、仓库政策和空白验证。
6. 普通提交并 push，创建关联 PR，回写 PR/CI/Review 引用。
7. 所有适用检查成功、Review 对话为 `0`、自动合并关闭后执行 Fresh 门禁。
8. 使用已记录的项目所有者授权执行 Squash Merge，并验证 Issue 关闭、提交结构、tree、签名、主干 CI 和远端分支删除。

## 七、验收标准

- PR `#21` merge commit 为 `0716ec0debcd3e059cc4ca88a072232841ca73b4`，唯一父提交为 `477d110a9b284e127af365f5278901bcfa69e093`。
- Merge tree 与最终 PR Head `9a4a4425f2fb0d8235554d3e83577111ae34efcc` 的 tree 均为 `4881ce609370f77181d9545474c029ab0c5d4972`；签名为 `valid`。
- Issue `#19` 为 `CLOSED / COMPLETED`；合并后 `main` 运行 `29919596057` 六 Job 全绿、成功 Artifact 数为 `0`。
- 只修改 14 个批准路径；产品源码、CI、`upstream/`、Ruleset 和 AGOS 为零差异。
- `CI_CONTRACT_GREEN passed=30`、仓库政策 `ok=true / violation_count=0`、PowerShell AST 和 `git diff --check` 通过。
- closeout PR 的所有适用检查成功、Review 对话为 `0`、自动合并关闭。
- 最终仅使用 Squash Merge，禁止任何绕过或历史改写。

## 八、停止条件

- 需要扩展 14 条允许路径或修改任何产品/CI/上游/Ruleset/AGOS 表面。
- Fresh 证据与 GitHub 状态不一致，或 merge/tree/签名/Issue/CI 任一结论无法复核。
- Review 对话出现且尚未完成根因、处理与验证闭环。
- 最终 Head 漂移、适用检查失败或授权范围不再覆盖当前差异。
