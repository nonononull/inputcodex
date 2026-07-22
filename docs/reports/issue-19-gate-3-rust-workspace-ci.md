# Issue #19：Gate 3 纯 Rust Workspace 与首版三平台 CI 报告

report_status: control-plane-active-implementation-not-started
tracking_issue_ref: https://github.com/nonononull/inputcodex/issues/19
branch_ref: codex/issue-19-gate-3-rust-workspace-ci
baseline_ref: 477d110a9b284e127af365f5278901bcfa69e093
session_plan_ref: docs/plans/sessions/2026-07-22-issue-19-gate-3-rust-workspace-ci.md
runtime_workflow_ref: docs/workflows/2026-07-22-issue-19-gate-3-rust-workspace-ci-runtime.md
scope_hash: sha256:2e101627480012d57d6d0472a08cfbe03fc401f6ac74ef3ae1e6a42929ed61ba
pr_ref: pending
ci_ref: pending
review_ref: pending
merge_ref: pending

## 一、批准范围

- 项目所有者已批准创建七成员纯 Rust Workspace、最小分层骨架、治理脚本与首版无缓存三平台 CI。
- 批准引用为 `user-message:approve-gate-3-implementation-2026-07-22`；该批准不包含最终 PR Squash Merge。
- 本任务不迁移上游业务功能，不实现数据库、网络、安装、更新、注入、远程推荐、广告、推广、遥测或发布流程。
- UI 只允许最小生命周期集成，不建立设计系统；视觉与交互由 Gemini 实现或审阅。

## 二、Fresh 基线

- PR `#18` 已 Squash Merge，Issue `#17` 已关闭；当前 `main` 基线为 `477d110a9b284e127af365f5278901bcfa69e093`。
- 上游正式 Release 为 `v1.2.41`，tag SHA 为 `3dafffcafb2566a1e8bce4b35671656d6adb3eda`；上游 `main` 的二维码图片变化对本任务无物质影响。
- Rust 为 `1.97.1 (8bab26f4f 2026-07-14)`；Iced 为 `0.14.0`、MIT、MSRV `1.88`、checksum `000e01026c93ba643f8357a3db3ada0e6555265a377f6f9291c472f6dd701fb3`、未撤回。
- Ruleset `19395456` 保持 active、无 bypass、required approvals `0`、解决 Review 对话和 Squash-only。
- 本地 GBrain 查询无结果；AGOS report-only 返回 `needs-input/unregistered`，已按项目合同绕过且未修改外部仓库。

## 三、当前 checkpoint

- 已创建公开 Issue `#19` 与分支 `codex/issue-19-gate-3-rust-workspace-ci`。
- Issue 正文已固定 23 条允许路径模式、RED/GREEN 合同、三平台 CI、停止条件、回滚和独立合并授权边界。
- Session Plan 与 Runtime Workflow 已落盘；当前 checkpoint 只修改治理文档。
- 当前仍不存在产品 `Cargo.toml`、`Cargo.lock`、`rust-toolchain.toml`、`.rs`、Iced 或 `.github/workflows/ci.yml`。

## 四、下一合法批次

1. 先提交并普通 push 当前控制面 checkpoint，在 Issue `#19` 回写 commit 和文档路径。
2. 创建 `scripts/ci/Test-CiScripts.ps1`，取得“治理入口尚不存在”的可信 RED 证据。
3. 未取得 RED checkpoint 前不得创建 Cargo Workspace。

## 五、收口边界

- PR、CI、Review 和 merge 字段保持 `pending`，不得提前宣称通过。
- 最终 PR 必须包含 `Closes #19`，所有适用 Job 成功、Review 对话根因闭环后，再等待项目所有者新的 Squash Merge 授权。
