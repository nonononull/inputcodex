# Runtime Workflow：Issue #8 Gate 1→2 控制面过渡

workflow_id: 2026-07-21-issue-8-gate-2-transition
schema_version: inputcodex.runtime-workflow.v1
tracking_issue_ref: https://github.com/nonononull/inputcodex/issues/8
active_gate_issue_ref: https://github.com/nonononull/inputcodex/issues/9
branch_ref: codex/issue-8-gate-2-transition
decision_status: approved
current_phase: gate-2-transition
external_agos_status: skipped-by-project-native-boundary

## 状态图

```text
merge-evidence-verified
  -> issue1-closed
  -> gate2-issues-created
  -> control-plane-updated
  -> transition-pr-open
  -> authorized-squash
  -> gate2-active
```

## Phase 1：merge-evidence-verified

- PR `#7` 为 `MERGED`，合并提交为 `c74b66422ba47f96bd3eb2b2385cdfb90541808e`。
- 合并提交的 GitHub 签名验证为 `valid`。
- parent 为 `b7404b0c63f2d2ba65474c077182c42a01cc9a64`，tree 为 `00f0f7fe0e408a1e6f218ee8e1be0d8442ed1e65`。
- Review 对话、未解决对话和 Checks 均为 `0`。

## Phase 2：issue1-closed / gate2-issues-created

- Issue `#1` 以 `completed` 关闭并回写 PR `#7` 证据。
- Issue `#8` 负责过渡 closeout。
- Issue `#9` 保持 OPEN，作为 Gate 2 活动 upstream-sync 任务。

## Phase 3：control-plane-updated

允许修改 README、Master Plan、架构治理、`build.md` 和任务文档；禁止修改产品运行面。

通过条件：

- `active_gate` 为 Gate 2。
- `active_task` 指向 Issue `#9`。
- 文档明确尚未导入源码、尚未创建 Cargo、Workflow 或 Release。
- `v1.2.41` 与提交 `3dafffcafb2566a1e8bce4b35671656d6adb3eda` 未变化。

## Phase 4：transition-pr-open

- PR 必须从 `codex/issue-8-gate-2-transition` 指向 `main`。
- PR 正文必须包含 `Closes #8`。
- PR 必须为非 Draft，Review 对话必须完成根因、处理和验证闭环。
- 不启用自动合并。

## Phase 5：authorized-squash / gate2-active

按项目所有者本次授权执行 Squash Merge。合并后验证：

- Issue `#8` 为 CLOSED。
- Issue `#9` 仍为 OPEN。
- `main` 包含过渡控制面，且没有源码、Cargo、Workflow 或 Release。
- Master Plan 指向 Gate 2 规划与来源锁定。

## 停止条件

需要导入快照、修改 `upstream/`、创建 `source-lock.json`、建立监控 Workflow 或进入 Gate 3 时，停止本 Runtime，转入 Issue `#9` 新 Session Plan。
