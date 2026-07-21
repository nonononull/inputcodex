# Runtime Workflow：Issue #6 Gate 1 仓库治理基线最终收口

workflow_id: 2026-07-21-issue-6-gate-1-finalization
schema_version: inputcodex.runtime-workflow.v1
tracking_issue_ref: https://github.com/nonononull/inputcodex/issues/6
branch_ref: codex/issue-6-gate-1-finalization
pr_ref: https://github.com/nonononull/inputcodex/pull/7
decision_status: approved
current_phase: owner-review-wait
external_agos_status: skipped-by-project-boundary

## 状态图

```text
baseline-verified
  -> issue-created
  -> branch-created
  -> governance-files
  -> github-metadata
  -> fresh-verification
  -> commit-and-push
  -> pr-open
  -> owner-review-wait
```

## Phase 1：baseline-verified

输入：干净 `main`、本地/远端 HEAD、GitHub Issue/PR、Ruleset、上游 Release。

通过条件：

- `main` 为 `b7404b0c63f2d2ba65474c077182c42a01cc9a64`。
- Issue `#4` 为 `CLOSED`，PR `#5` 为 `MERGED`。
- PR `#5` merge tree 与最终 Head tree 一致，且合并提交只有一个父提交。
- Ruleset `19395456` 保持批准参数。
- 上游最新正式 Release 为 `v1.2.41`，提交为 `3dafffcafb2566a1e8bce4b35671656d6adb3eda`。

失败处理：重新抓取事实；事实发生变化时停止 mutation，不按缓存结论继续。

## Phase 2：issue-created / branch-created

通过条件：

- 跟踪 Issue 为 `#6`。
- 当前分支为 `codex/issue-6-gate-1-finalization`。
- 分支从批准基线创建，未直接修改 `main`。

## Phase 3：governance-files

允许写入：当前计划、Session Plan、Runtime Workflow、README、Master Plan、架构治理、`build.md`、closeout 报告、Issue Forms 与 PR 模板。

禁止写入：源码、Cargo、Actions、Release、Ruleset 和外部 AGOS。

通过条件：

- 所有当前状态均指向 Issue `#6` 和当前分支。
- Issue `#4` / PR `#5` 只作为已完成历史证据。
- 八类 Issue Forms 和 PR 模板具备必填追踪、范围、验证与决策字段。
- PR 模板明确要求 Review 根因、处理和验证证据闭环。

## Phase 4：github-metadata

操作：创建批准标签、给 Issue `#6` 打标签、清理 PR `#5` 已合并旧分支。

通过条件：

- 默认九个标签仍存在。
- 新增八个 `type:*`、六个 `gate:*`、两个 `platform:*` 与两个 `status:*` 标签。
- Issue `#6` 同时拥有 `type:architecture` 与 `gate:1`。
- 远端、本地和 `origin/*` 均不再保留 `codex/issue-4-gate-1-closeout`。

## Phase 5：fresh-verification

必须执行 `build.md` 中的：

- 本地文件、YAML 与禁用表面验证。
- GitHub Issue、PR、Ruleset、标签与空 Workflow/Release 验证。
- PR `#5` Squash tree 与旧分支清理验证。
- `git diff --check`、分支和 staged diff 检查。

失败处理：先查 `err.md`，确定根因，只修复本 Issue 范围内问题，随后从失败命令 Fresh 重跑。

## Phase 6：commit-and-push

约束：

- 精确暂存本 Issue 文件。
- 提交主题使用 `docs: 完成 Gate 1 仓库治理收口`。
- 只执行普通 push，禁止 Force Push。
- Git transport 失败时按 `err.md` 区分网络路径，不覆盖远端历史。

## Phase 7：pr-open

通过条件：

- PR 基线为 `main`，Head 为当前分支，非 Draft。
- PR 正文包含 `Closes #6`，并关联 Issue `#1`。
- PR 明确声明无源码、无 Cargo、无 Actions、无 Release、无 Ruleset 变更。
- Issue `#1` 获得 PR URL 追踪评论，但保持开放。
- Review 对话未解决数为 `0`；后续任何对话都必须根因闭环。

## Phase 8：owner-review-wait

当前证据：PR `#7` 为 `OPEN`、非 Draft、`CLEAN`，Checks、Reviews、Review 对话与未解决数均为 `0`，未启用自动合并；本地、远端跟踪与 PR Head 在首次推送时一致为 `df3795d03027ddfff512a911bf8493b63b603275`。

停止条件：PR 已开放且远端证据一致。未经项目所有者再次明确授权，不执行合并、关闭 Issue `#1` 或进入 Gate 2。

## 中断包

```text
branch: codex/issue-6-gate-1-finalization
tracking_issue: https://github.com/nonononull/inputcodex/issues/6
last_verified_command: <填入最后一次通过命令>
pending_step: <填入下一步>
git_status: <填入当前状态>
external_agos: skipped; not a project gate
```
