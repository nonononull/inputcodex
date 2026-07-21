# Issue #8：Gate 1→2 控制面过渡计划

## 基本信息

- 日期：2026-07-21。
- 跟踪 Issue：`https://github.com/nonononull/inputcodex/issues/8`。
- 执行分支：`codex/issue-8-gate-2-transition`。
- 基线提交：`c74b66422ba47f96bd3eb2b2385cdfb90541808e`。
- 范围哈希：`sha256:085e4272380212c1c5a99490996d21e814578995db58dccee0ca8249eec9e4d3`。
- 活动 Gate 2 Issue：`https://github.com/nonononull/inputcodex/issues/9`。
- 决策状态：项目所有者已授权 PR `#7` 合并、Issue `#1` 关闭并切换 Gate 2。

## 目标

把 PR `#7` 的最终合并事实、Issue `#1/#6` 关闭事实和 Gate 2 的来源锁定边界写回项目控制面，并让 Master Plan 指向持续开放的 Issue `#9`。

## 权威输入

- PR `#7`：`MERGED`，合并时间 `2026-07-21T16:21:01Z`。
- PR `#7` 合并提交：`c74b66422ba47f96bd3eb2b2385cdfb90541808e`。
- PR `#7` 父提交：`b7404b0c63f2d2ba65474c077182c42a01cc9a64`。
- PR `#7` tree：`00f0f7fe0e408a1e6f218ee8e1be0d8442ed1e65`。
- PR `#7` Review 对话与 Checks：均为 `0`。
- Issue `#1`：`CLOSED`，关闭原因为 `COMPLETED`。
- Issue `#6`：`CLOSED`，由 PR `#7` 自动关闭。
- 上游最新正式 Release：`v1.2.41`，提交 `3dafffcafb2566a1e8bce4b35671656d6adb3eda`。

## 允许操作

- 更新 README、Master Plan、架构治理和 `build.md` 的 Gate 状态。
- 新增本计划、Session Plan、Runtime Workflow 和 PR `#7` closeout 报告。
- 新增 Issue `#9` 的 Gate 2 计划、Session Plan 和 Runtime Workflow。
- 在 GitHub 创建包含 `Closes #8` 的文档过渡 PR。
- 按项目所有者本次授权 Squash Merge 过渡 PR，并清理其已合并分支。

## 禁止操作

- 不导入上游或半成品源码，不创建 `upstream/` 内容或 `source-lock.json`。
- 不创建 Cargo Workspace、Rust/Iced 代码、UI 或 `.github/workflows/`。
- 不创建 Release、安装包、签名或更新资产。
- 不修改 Ruleset、required checks、仓库级合并开关或 `main` 历史。
- 不修改、修复或优化 AGOS。
- 不把 Gate 2 规划误写成已经完成快照导入。

## 执行批次

1. 核验 PR `#7` 签名 Squash、Issue 关闭、`main` 和上游 Release。
2. 从 PR `#7` 合并提交创建本分支。
3. 更新项目入口、Master Plan、架构治理和 Gate 2 文档。
4. 运行本地文档、模板、GitHub 和禁止表面验证。
5. 创建 `Closes #8` 的非 Draft 过渡 PR，回写真实 URL。
6. 按本次明确授权 Squash Merge 过渡 PR，验证 Gate 2 活动态。

## 完成标准

- Issue `#8` 通过独立 PR 完成并关闭。
- Master Plan active gate 为 Gate 2，active task 为开放 Issue `#9`。
- README、架构治理、`build.md` 和 Gate 2 计划明确“尚未导入源码”。
- PR `#7` 合并、Issue `#1/#6` 关闭和旧分支清理证据可重复核验。
- 仓库仍无 Rust 源码、Cargo、Workflow 和 Release。
- Gate 2 快照导入仍等待 Issue `#9` 的 Session Plan 与允许写入范围批准。
