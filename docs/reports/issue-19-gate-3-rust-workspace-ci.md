# Issue #19：Gate 3 纯 Rust Workspace 与首版三平台 CI 报告

report_status: governance-green-verified-awaiting-checkpoint-push
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

## 三、控制面与 RED checkpoint

- 已创建公开 Issue `#19` 与分支 `codex/issue-19-gate-3-rust-workspace-ci`。
- Issue 正文已固定 23 条允许路径模式、RED/GREEN 合同、三平台 CI、停止条件、回滚和独立合并授权边界。
- Session Plan 与 Runtime Workflow 已落盘，控制面提交 `03b68584add4e43291818376a2a85a696ea1e688` 已与远端分支精确一致。
- `scripts/ci/Test-CiScripts.ps1` 已预置空 diff、文档、重型路径、删除、重命名、非法路径、Iced 越层、Workspace 越界、脚本语言、WebView/Tauri、广告/遥测、更新源和依赖方向夹具。
- PowerShell AST 解析为 `0` 个错误；实际 RED 执行退出码为 `10`，稳定标记 `CI_CONTRACT_RED_MISSING_IMPLEMENTATION` 恰好出现一次。
- RED 根因是 `scripts/ci/Classify-Changes.ps1` 与 `scripts/ci/Verify-RepositoryPolicy.ps1` 尚不存在，不是路径、拼写、解析或夹具错误。
- RED 提交 `67fe99457e1aa2717cc29c70d51114028d68dafd`、tree `21bc79da1e402c4d7266d4d8b54af6866a685463` 已通过普通 fast-forward push，并回写 Issue 评论 `5043557146`。

## 四、治理 GREEN 证据

- 路径分类器输出确定 JSON，区分空 diff、纯文档和重型路径，并审计删除、重命名的新旧路径。
- 仓库政策验证器固定七成员显式 Workspace、包名与依赖方向；Iced 只能直接存在于 presentation，并拒绝生产 TypeScript/JavaScript、Tauri/WebView、广告/遥测和外部更新源。
- 初始 GREEN 自测暴露测试夹具的零结果数组形状和空集合参数绑定问题；均确定根因并修复测试，不降低实现门禁。
- 安全复核先新增 TOML 表形式的 Tauri 别名与逆向依赖 RED，再修复依赖解析器，防止通过 `[dependencies.alias]` 绕过政策。
- 最终合同测试 `23/23` 通过，三份 PowerShell 脚本 AST 均为 `0` 个错误，测试退出码为 `0`。
- 当前仍不存在产品 `Cargo.toml`、`Cargo.lock`、`rust-toolchain.toml`、`.rs`、Iced 或 `.github/workflows/ci.yml`。

## 五、下一合法批次

1. 提交并普通 push 治理 GREEN checkpoint，在 Issue `#19` 回写 `23/23`、commit、失败恢复和安全复核证据。
2. 按 Runtime Workflow 的 Phase 4 顺序创建七成员最小 Workspace、加载状态语义、平台 unsupported 合同与 Iced 展示层隔离。
3. Workspace checkpoint 完成并回写 Issue 前不得创建 `.github/workflows/ci.yml`。

## 六、收口边界

- PR、CI、Review 和 merge 字段保持 `pending`，不得提前宣称通过。
- 最终 PR 必须包含 `Closes #19`，所有适用 Job 成功、Review 对话根因闭环后，再等待项目所有者新的 Squash Merge 授权。
