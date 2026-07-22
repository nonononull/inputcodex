# Issue #17：Gate 3 纯 Rust Workspace 骨架规划报告

report_status: pr-open-owner-merge-authorization-pending
tracking_issue_ref: https://github.com/nonononull/inputcodex/issues/17
branch_ref: codex/issue-17-gate-3-rust-workspace-plan
baseline_ref: 113476fb96623452f9a69526edabc73a57d812a1
pr_ref: https://github.com/nonononull/inputcodex/pull/18
ci_ref: not-triggered:docs-only-path-filter
review_ref: https://github.com/nonononull/inputcodex/pull/18#issuecomment-5042127837
merge_ref: pending

## 一、批准范围

- 项目所有者已批准进入 Gate 3 规划阶段，决策引用为 `user-message:approve-gate-3-planning-2026-07-22`。
- 本任务只建立规划控制面，不创建 Cargo、Rust、Iced、产品 Workflow、UI 或功能实现。
- 规划 PR 合并后，Workspace 与首版三平台 CI 仍需新的实现 Issue 和明确批准。

## 二、Fresh 基线

- `main` 基线：`113476fb96623452f9a69526edabc73a57d812a1`，与远端一致。
- 上游正式 Release：`v1.2.41`；标签提交：`3dafffcafb2566a1e8bce4b35671656d6adb3eda`；上游 `main`：`6fa0a57decbb3382771a981247e6922799e97f5d`。
- Ruleset `19395456`：active、无 bypass、required approvals `0`、必须解决 Review 对话、Squash-only。
- Gate 2 已完成：PR `#15` Squash Merge，Issue `#14` 关闭，两次真实运行成功，机器状态 Issue `#16` 唯一且告警数量为 `0`。

## 三、方案结论

- 采用“合同先行，规划与实现拆分”，拒绝直接创建完整 Workspace 和只写概念 ADR。
- 后续 Workspace 固定为桌面 app 加六个分层 crate；Iced 只允许直接存在于 presentation crate。
- Windows 与 macOS 共享 application 用例和稳定错误语义；条件编译限制在 platform 或极薄启动适配层。
- 后续实现使用本地轻量验证和标准 GitHub-hosted runners 的 Linux/Windows/macOS 全量验证；首版无 Cache，不上传整个 `target/`。
- Gate 3 只建立最小生命周期、加载状态和可诊断合同，不迁移任何上游功能，也不确立 UI 设计系统。

## 四、版本与知识证据

- Rust 后续实现候选为精确版本 `1.97.1`；Iced 候选为 `0.14.0`，MSRV `1.88`、MIT、未撤回，实施前必须 Fresh 复核。
- 本地 GBrain 查询返回 `No results`，没有用空结果补写架构结论。
- AGOS 默认入口以 report-only 运行，返回 `needs-input/unregistered`；按项目既有规则记录并绕过，没有修改或优化外部 AGOS。
- 仓库无 `.codegraph/`，未初始化索引；本任务为文档规划，不需要代码调用图。

## 五、当前边界

- 当前允许路径共 `11` 条，范围哈希为 `sha256:0c4fc5017aed0b5883b5cb597b2afc2680646479de32916cc4e720aff67dfd05`。
- 本地 Fresh 验证已通过：变更路径 `11`、越界路径 `0`、产品 Cargo 文件 `0`、产品 Rust 文件 `0`、Workflow 文件 `1` 且仅为既有 `upstream-watch.yml`，`git diff --check` 通过。
- PR `#18` 已创建，初始远端 Head 为 `1ecc4426df33ad728b44ef0bbacc928d18b4bcf9`，状态为 OPEN、非 Draft、`MERGEABLE/CLEAN`；远端文件列表与允许路径完全一致，Review 对话 `0`，自动合并关闭。
- 现有 `Upstream Watch` Workflow 的 `pull_request.paths` 只监听监控脚本、Workflow 与 `upstream/source-lock.json`；本 PR 只改文档，因此没有 Actions 运行或 Check。该事实记录为 `not-triggered:docs-only-path-filter`，不表述为 CI 通过。
- 当前尚未获得项目所有者对 PR `#18` 的 Squash Merge 授权；最终 Head、Fresh Review、Ruleset 与上游基线将回写到同一 Review 锚点，`merge_ref` 保持 `pending`。
