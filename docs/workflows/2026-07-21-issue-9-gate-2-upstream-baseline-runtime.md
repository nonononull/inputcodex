# Runtime Workflow：Issue #9 Gate 2 上游基线

workflow_id: 2026-07-21-issue-9-gate-2-upstream-baseline
schema_version: inputcodex.runtime-workflow.v1
tracking_issue_ref: https://github.com/nonononull/inputcodex/issues/9
branch_ref: pending:issue-9-execution-branch
decision_status: transition-approved-import-scope-pending
current_phase: planning-only
external_agos_status: not-needed-project-native-control-plane

## 状态图

```text
release-verified
  -> license-and-source-reviewed
  -> source-lock-designed
  -> purity-check-designed
  -> owner-scope-approval
  -> upstream-sync-branch
  -> snapshot-verify
  -> upstream-sync-pr
```

## Phase 1：release-verified

当前冻结：`v1.2.41` / `3dafffcafb2566a1e8bce4b35671656d6adb3eda`。如最新正式 Release 或标签提交变化，停止并重新建立决策证据。

## Phase 2：license-and-source-reviewed

记录 GNU AGPLv3、来源仓库、精确提交和必须保留的声明；不复制任何产品运行代码。

## Phase 3：source-lock-designed

设计 Release、提交、仓库、许可证、树哈希、文件哈希、生成时间和验证工具版本字段。格式获批前不创建文件。

## Phase 4：purity-check-designed

验证快照只存在于 `upstream/`，不被 Cargo、Iced、产品运行面或发布资产引用；排除广告、推广、隐蔽遥测和不允许进入最终运行面的内容。

## Phase 5：owner-scope-approval

必须获得项目所有者对允许写入路径、验证命令、许可证处理和回滚方案的明确批准；本阶段之前只允许文档和 GitHub Issue 证据。

## Phase 6：upstream-sync-branch / snapshot-verify

后续执行必须从独立分支开始，staged diff 只能包含 `upstream/`、`source-lock.json` 和同步报告；不得混入产品代码或 Workflow。

## Phase 7：upstream-sync-pr

同步 PR 必须关联 Issue `#9`，完成 Review 根因闭环、CI/验证证据、Squash Merge 和分支清理。功能实现另建 Issue/PR。

## 停止条件

发现副作用、错误语义、许可证不确定、来源变化、构建引用快照或需要 AGOS 改动时，停止当前 Runtime 并建立对应决策或例外 Issue。
