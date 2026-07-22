# Issue #22 Runtime Workflow：Gate 3 合并 closeout

workflow_status: pr-review-ready-owner-merge-authorized
tracking_issue_ref: https://github.com/nonononull/inputcodex/issues/22
session_plan_ref: docs/plans/sessions/2026-07-22-issue-22-gate-3-closeout.md
task_plan_ref: docs/plans/2026-07-22-issue-22-gate-3-closeout.md
approved_decision_ref: user-message:create-gate3-closeout-through-squash-merge-2026-07-22
owner_merge_authorization_ref: user-message:create-gate3-closeout-through-squash-merge-2026-07-22
closeout_pr_ref: https://github.com/nonononull/inputcodex/pull/23
scope_hash: sha256:16760a8ce385b171b007451a43a3acb604a7b8ffc06b098b5482b8d803115ec8

## 当前 checkpoint

- Phase 0 已完成：`main`、PR `#21`、Issue `#19`、运行 `29919596057`、Ruleset 与维护者数量 Fresh 一致。
- Phase 1 已完成：Issue `#22` 与 `codex/issue-22-gate-3-closeout` 已建立。
- Phase 2/3 已完成：14 条批准路径已回写，本地 AST、`30/30` 治理合同、仓库政策和空白门禁通过。
- PR `#23` 已创建，当前进入 Phase 4：回写自引用并等待最终 Head 的 Review/CI。
- 候选 Head 运行 `29921450017` 已成功；Review 对话 `0`、Merge State `CLEAN`、自动合并关闭，当前执行最终状态封口提交。

## Phase 0：startup-baseline

1. 核对本地/远端 `main` 为 `0716ec0debcd3e059cc4ca88a072232841ca73b4`，工作树干净。
2. 核对 PR `#21`、Issue `#19`、最终 PR CI、合并后 main CI、Review、Ruleset 和维护者数量。
3. 核对 merge commit 单父、tree 与最终 Head 等价、GitHub 签名 `valid`。
4. 任何不一致先停止并查根因，不根据旧报告猜测。

## Phase 1：issue-and-branch

1. 创建独立 Issue，写明背景、14 条允许路径、禁止范围、验收标准和完整授权引用。
2. 从已验证 `main` 创建 `codex/issue-22-gate-3-closeout`。
3. 禁止在创建 Issue 前修改仓库文件。

## Phase 2：control-plane-writeback

1. 创建 closeout 任务计划、Session Plan、Runtime Workflow 与报告。
2. 更新 `AGENTS.md`、README、build/err、Master Plan 与 Issue `#19` 历史控制面。
3. 所有状态必须区分“来源 Gate 3 已完成”和“closeout PR 尚在交付”。
4. 不修改产品、CI、upstream、Ruleset 或 AGOS。

## Phase 3：local-verification

1. 核对变更路径恰好等于 14 条批准路径。
2. 核对 Cargo、apps、crates、scripts/ci、Workflow、upstream 与 AGOS 为零差异。
3. 解析四份 PowerShell 脚本 AST，运行 `CI_CONTRACT_GREEN passed=30` 与真实仓库政策。
4. 扫描 pending/Draft/Issue #19 OPEN 等陈旧状态，只允许 closeout PR 自身的动态字段保持 pending。
5. 运行 `git diff --check`，形成普通提交。

## Phase 4：pr-review-ci

1. 普通 push 并创建关联 `Closes #22` 的非 Draft PR。
2. 取得真实 PR 编号后回写 Master Plan、Session Plan、Runtime Workflow 与报告，再普通提交/push。
3. 等待所有适用检查完成；文档-only 分类允许三平台 Job 按合同跳过，但 `classify`、`governance` 与 `required` 必须成功。
4. Fresh 核对 Head、允许路径、自动合并、Ruleset、维护者数量和 Review 对话。
5. 每条 Review 对话必须完成根因、处理和验证闭环。

## Phase 5：authorized-squash-merge

1. 确认当前差异仍完全属于已授权的 14 条路径。
2. 在 PR/Issue 留存 `user-message:create-gate3-closeout-through-squash-merge-2026-07-22` 决策证据。
3. 使用 `--match-head-commit` 锁定最终 Head，只执行 Squash Merge，不使用管理员绕过。
4. 删除远端功能分支；永久禁止 Force Push 和删除 `main`。

## Phase 6：post-merge-verification

1. 验证 PR 为 MERGED、Issue `#22` 为 CLOSED/COMPLETED。
2. 验证 closeout merge commit 单父、tree 与最终 Head 等价、签名 `valid`。
3. 等待合并后 main CI 的所有适用检查成功，成功 Artifact 继续为 `0`。
4. 验证远端功能分支删除、本地 `main` 快进且工作树干净。
5. 在已关闭 Issue 回写最终 closeout 评论；不直接修改 `main` 形成递归 closeout。

## 停止条件

- 范围、Head、Ruleset、维护者数量或授权发生物质漂移。
- 适用 CI 失败、Review 对话未闭环或自动合并被启用。
- 需要触及产品、CI、upstream、Gate 4、Ruleset 或 AGOS。
