# Runtime Workflow：Issue #9 Gate 2 上游基线

workflow_id: 2026-07-21-issue-9-gate-2-upstream-baseline
schema_version: inputcodex.runtime-workflow.v1
tracking_issue_ref: https://github.com/nonononull/inputcodex/issues/9
branch_ref: codex/issue-9-upstream-sync
decision_status: completed-squash-merged
current_phase: complete
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
  -> owner-review
  -> squash-merged
```

## Phase 1：release-verified

已冻结并 Fresh 复核：`v1.2.41` / `3dafffcafb2566a1e8bce4b35671656d6adb3eda` / tree `22e3a9c8ad15e18b972eae44a892b7980dca5ec2`。

## Phase 2：license-and-source-reviewed

已记录 GNU AGPLv3、来源仓库、精确提交和 `7` 份许可证/声明；快照不接入产品运行面。

## Phase 3：source-lock-designed

已生成 `upstream/source-lock.json`，记录 Release、提交、tree、归档、逐文件 blob/SHA-256、许可证和生成工具版本。

## Phase 4：purity-check-designed

已验证快照只存在于 `upstream/`，仓库根没有 Cargo、Iced、产品运行面、Workflow 或发布资产；广告、推广、遥测及其他不允许内容只能作为审计输入。

## Phase 5：owner-scope-approval

项目所有者已批准允许写入路径、验证命令、许可证处理和回滚边界，决策引用为 `user-message:approve-issue-9-upstream-sync-2026-07-21`。

## Phase 6：upstream-sync-branch / snapshot-verify

执行使用独立分支 `codex/issue-9-upstream-sync`；PR 共 `279` 条路径，其中 `278` 条位于 `upstream/`，`1` 条为同步报告，未混入产品代码或 Workflow。

## Phase 7：upstream-sync-pr

同步 PR `#11` 已关联并关闭 Issue `#9`，完成 Review 根因闭环和 Fresh 验证后，于 `2026-07-21T19:01:02Z` Squash Merge。功能实现仍必须另建 Issue/PR。

## 完成证据

- 获批 Head：`90d35a72cffb4a13c5f7588a147e19cbd75b14c6`。
- Squash Merge：`dde08b725eb2bf4add7fbcfa955f3eaf4eb1bbc6`。
- Merge tree 与获批 Head tree：`d0c90b9bfda70de800788782180080d50d914564`，完全一致。
- 快照：`277` 个文件、`70` 个目录、`24,175,877` 字节、manifest SHA-256 `3c9b16802f49a1bcb56fda9630d97edc52c918c30d1924145244d9239801d3d4`。
- Checks：`0`，只表示项目尚未配置 CI；Review 对话：`0`。
- 控制面收口：Issue `#12`，不得修改已合并快照。

## 停止条件

发现副作用、错误语义、许可证不确定、来源变化、构建引用快照或需要 AGOS 改动时，停止当前 Runtime 并建立对应决策或例外 Issue。
