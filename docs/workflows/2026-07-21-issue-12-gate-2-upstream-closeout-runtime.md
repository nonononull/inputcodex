# Runtime Workflow：Issue #12 Gate 2 上游基线收口

workflow_id: 2026-07-21-issue-12-gate-2-upstream-closeout
schema_version: inputcodex.runtime-workflow.v1
tracking_issue_ref: https://github.com/nonononull/inputcodex/issues/12
branch_ref: codex/issue-12-gate-2-upstream-closeout
decision_status: execution-approved-closeout-only
current_phase: owner-review-wait
external_agos_status: skipped-project-native-control-plane

## 状态图

```text
merge-evidence-locked
  -> closeout-issue-branch
  -> control-plane-backfill
  -> local-fresh-verification
  -> commit-and-push
  -> closeout-pr
  -> owner-review-wait
```

## Phase 1：merge-evidence-locked

锁定 PR `#11` 的获批 Head `90d35a72cffb4a13c5f7588a147e19cbd75b14c6`、Squash Merge `dde08b725eb2bf4add7fbcfa955f3eaf4eb1bbc6`、父提交 `216d400006ad3f1dd2587ca367abb19d0191949f` 和 tree `d0c90b9bfda70de800788782180080d50d914564`。任一 Fresh 证据不一致时停止。

## Phase 2：closeout-issue-branch

Issue `#12` 与分支 `codex/issue-12-gate-2-upstream-closeout` 必须从已验证 `origin/main` 创建。禁止复用已合并同步分支或直接修改 `main`。

## Phase 3：control-plane-backfill

只更新 Issue `#12` 允许的 README、构建/排错文档、Master Plan、Issue `#9` 历史控制面和本任务 closeout 文档。`upstream/` 与产品运行面必须保持不变。

## Phase 4：local-fresh-verification

执行：

- PR `#11` merge commit 父链、tree、279 路径和 Issue `#9` 关闭验证。
- `upstream/` 相对 `origin/main` 的 0 差异验证。
- source-lock 277 文件、manifest、许可证和提交 mode/blob 快速验证。
- 当前分支、允许路径、Markdown `git diff --check` 与根目录禁止表面验证。

## Phase 5：commit-and-push

精确暂存允许路径，提交主题使用 `docs: 收口 Gate 2 上游基线合并证据`。只允许普通 push；禁止 Force Push。

## Phase 6：closeout-pr

PR `#13` 已关联并关闭 Issue `#12`，声明不修改快照、产品源码、Workflow、Ruleset 或 AGOS；Head 为 `dce2f71e7f237145d7186224b621ec304f9e69f2`，Checks `0`、Review 对话 `0`、自动合并关闭。

## Phase 7：owner-review-wait

PR 保持 OPEN、非 Draft、自动合并关闭。未经项目所有者新的明确 Squash Merge 授权，不执行合并、不进入 Gate 3。

## 停止条件

- 上游最新正式 Release、PR `#11` merge ref、Ruleset 或 `main` 状态变化。
- closeout diff 触及 `upstream/`、产品源码、Workflow、Release、Ruleset 或 AGOS。
- Review 对话未完成根因、处理和 Fresh 验证闭环。
- 需要决定 Gate 3、功能迁移、上游监控实现或新的付费/自托管 CI 资源。
