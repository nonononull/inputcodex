# inputcodex

`inputcodex` 是面向 Codex 本地增强与管理场景的新项目，当前已完成 Gate 1 治理冻结、Gate 2 上游 `v1.2.41` 审计快照与每 6 小时监控，并已通过 Issue `#17` / PR `#18` 冻结 Gate 3 合同；Issue `#19` 已完成纯 Rust Workspace、五类失败语义与三平台最低冷构建基线，Draft PR `#21` 正在进入 Review/CI 收口。

## 项目目标

- 软件名称统一为 `inputcodex`。
- 产品与代码中不引入广告、推广位或广告 SDK。
- 优先解决卡顿、功能加载异常和模块职责混乱问题。
- 已批准采用纯 Rust、Iced 展示层和分层核心重新实现，不照搬上游或半成品架构。
- Windows 与 macOS 从首版起保持功能一致。
- 使用纯 Rust 与 Iced 构建桌面产品，不使用 TypeScript、JavaScript 业务代码或 WebView。
- 上游 Tauri/React 管理界面、现有注入脚本和远程推荐列表只进入快照审计，不直接进入新架构或最终运行面。

## 参考项目

- 上游项目：`BigPizzaV3/CodexPlusPlus`
- 半成品参考：`zsr131550/CodexPlusPlus`

两份参考项目均采用 GNU AGPLv3。当前仓库同样采用 GNU AGPLv3；`BigPizzaV3/CodexPlusPlus` 正式 Release `v1.2.41` 已以完整只读审计快照导入 `upstream/CodexPlusPlus/`，半成品参考仓库尚未导入。

## 当前阶段

截至 2026 年 7 月 22 日，已完成：

- 建立本地 Git 仓库与 GitHub 公开仓库。
- 固定项目名称、无广告原则和性能优先目标。
- 建立项目级构建、排错、计划与执行约束文档。
- 通过 Issue `#2` 批准纯 Rust/Iced、完整上游快照、自主发布线和 Issue/PR 治理方案。
- 通过 PR `#3` Squash Merge 重构与发布治理单一真源、项目语境、ADR、Session Plan、Runtime Workflow、`main-protection` Ruleset 和 Rust CI 云端卸载计划。
- PR `#3` 的合并提交为 `0e11375997ff10fdc0c233b31c8468af2d9a4f44`；Issue `#2` 已关闭，旧功能分支已删除。
- 通过 PR `#5` Squash Merge Issue `#2` / PR `#3` 的最终 closeout 证据；合并提交为 `b7404b0c63f2d2ba65474c077182c42a01cc9a64`，Issue `#4` 已关闭。
- PR `#5` 的合并提交只有一个父提交，merge tree 与最终 PR Head tree 均为 `af186e05673b441a936199e55c7d632cd06ea929`；Review 对话与 Checks 数量均为 `0`。
- 通过 PR `#7` Squash Merge Issue Forms、PR 模板、项目标签与 Gate 1 最终控制面；合并提交为 `c74b66422ba47f96bd3eb2b2385cdfb90541808e`，Issue `#6` 已关闭。
- 筹备 Issue `#1` 已回写 Gate 1 完成证据并以 `completed` 关闭。
- 通过 PR `#10` Squash Merge Gate 1→2 控制面过渡；合并提交为 `216d400006ad3f1dd2587ca367abb19d0191949f`，Issue `#8` 已关闭。
- 通过 PR `#11` Squash Merge 上游 `v1.2.41` 完整只读审计快照、`source-lock.json` 与同步报告；合并提交为 `dde08b725eb2bf4add7fbcfa955f3eaf4eb1bbc6`，Issue `#9` 已关闭。
- 通过 PR `#13` Squash Merge Gate 2 上游基线 closeout；合并提交为 `5e64015075ddf2adef4bf685f50977b47b7f72e7`，Issue `#12` 已关闭。
- 通过 PR `#15` Squash Merge 定时/手动上游监控、只读 PR 验证和幂等 Issue 状态机；合并提交为 `113476fb96623452f9a69526edabc73a57d812a1`，Issue `#14` 已关闭。
- `main` 上两次真实运行 `29890586102` 与 `29890641799` 均成功；唯一机器状态为 Issue `#16`，重复告警数量为 `0`。
- 通过 PR `#18` Squash Merge Gate 3 Workspace、Iced 隔离、双平台抽象、性能诊断和三平台 CI 合同；合并提交为 `477d110a9b284e127af365f5278901bcfa69e093`，Issue `#17` 已关闭。
- 已创建实现 Issue `#19` 与分支 `codex/issue-19-gate-3-rust-workspace-ci`，项目所有者已批准并完成 RED/GREEN 治理合同、Workspace、首版 CI、失败语义与最低冷构建基线；最终 Squash Merge 仍需新的明确授权。

当前明确不做：

- 不把已导入的上游审计快照接入产品构建或运行面，也不在非 upstream-sync Issue 中修改快照。
- 不导入半成品参考仓库源码。
- 不在取得可信 RED 治理证据前创建 Cargo Workspace，也不在 Issue `#19` 中创建发布 Workflow、安装包、签名或更新资产。
- 不执行功能迁移、安装包构建、发布或未经项目所有者授权的 PR 合并。
- 不让上游 Tauri/React UI、现有注入脚本和远程推荐列表进入最终运行面。

## 当前 Gate 3 实现边界

- Gate 2 的 Issue `#9/#12/#14` 与 PR `#11/#13/#15` 均已完成；上游监控 Workflow 持续运行，Issue `#16` 只由机器维护。
- `upstream/CodexPlusPlus/` 含 `v1.2.41` 的 `277` 个只读审计文件，`upstream/source-lock.json` 记录来源、tree、逐文件 blob/SHA-256、许可证和生成工具证据。
- Issue `#19` 已创建七成员纯 Rust Workspace、Rust `1.97.1` 工具链文件、`Cargo.lock`、最小分层源码与首版无缓存三平台 `CI` Workflow；仍没有 Release、安装包、签名、更新资产或上游业务功能迁移。
- 当前治理合同为 `30/30`，Workspace 许可证元数据已与根 `LICENSE` 对齐为 `AGPL-3.0-only`；五类受控失败语义已全部完成普通提交 RED→GREEN，最新修复运行 `29917649550` 六 Job 全绿且成功 Artifact 数为 `0`。
- Linux、Windows、macOS 已各取得 `3/3` 次无缓存成功样本；Job 执行时间中位数分别为 `133`、`212`、`96` 秒，最低基线不包含 Cache、P95 或七天调优结论。
- Gate 3 实现顺序固定为“RED 治理合同 → GREEN 治理脚本 → 七成员 Workspace → 无缓存三平台 CI → 真实失败恢复 → 冷构建基线”。
- Iced 只能直接存在于 presentation crate；最小窗口不建立 UI 设计系统，视觉与交互由 Gemini 实现或审阅。

## 下一步

1. 提交并推送 Gate 3 最终控制面 checkpoint，不修改产品语义、Ruleset、`upstream/` 或外部 AGOS。
2. 等待最终 PR Head 的六个 CI Job 成功，Fresh 复核允许路径、自动合并、Ruleset 与 Review 对话数量后把 Draft PR `#21` 转为 Ready for review。
3. 停在等待项目所有者新的明确 Squash Merge 授权；未授权前不合并 PR、不关闭 Issue `#19`。

## 项目文档

- 构建说明：`build.md`
- 排错记录：`err.md`
- 项目总计划：`docs/plans/PROJECT-MASTER-PLAN.md`
- 本次筹备计划：`docs/plans/2026-07-21-bootstrap.md`
- 筹备会话计划：`docs/plans/sessions/2026-07-21-inputcodex-bootstrap.md`
- 筹备运行工作流：`docs/workflows/2026-07-21-inputcodex-bootstrap-runtime.md`
- 项目术语：`CONTEXT.md`
- 重构与发布治理总方案：`docs/plans/2026-07-21-architecture-governance.md`
- Issue `#2` closeout 报告：`docs/reports/issue-2-architecture-governance-closeout.md`
- Issue `#4` closeout 报告：`docs/reports/issue-4-gate-1-closeout.md`
- Issue `#6` closeout 报告：`docs/reports/issue-6-gate-1-finalization-closeout.md`
- Gate 1→2 过渡计划：`docs/plans/2026-07-21-issue-8-gate-2-transition.md`
- Gate 2 活动计划：`docs/plans/2026-07-21-issue-9-gate-2-upstream-baseline.md`
- Gate 2 Session Plan：`docs/plans/sessions/2026-07-21-issue-9-gate-2-upstream-baseline.md`
- Gate 2 Runtime Workflow：`docs/workflows/2026-07-21-issue-9-gate-2-upstream-baseline-runtime.md`
- 上游 `v1.2.41` 同步报告：`docs/reports/2026-07-21-upstream-v1.2.41-sync.md`
- Gate 2 closeout 计划：`docs/plans/2026-07-21-issue-12-gate-2-upstream-closeout.md`
- Gate 2 closeout Session Plan：`docs/plans/sessions/2026-07-21-issue-12-gate-2-upstream-closeout.md`
- Gate 2 closeout Runtime Workflow：`docs/workflows/2026-07-21-issue-12-gate-2-upstream-closeout-runtime.md`
- Gate 2 closeout 报告：`docs/reports/issue-12-gate-2-upstream-closeout.md`
- Issue `#14` 上游监控计划：`docs/plans/2026-07-22-issue-14-gate-2-upstream-watch.md`
- Issue `#14` Session Plan：`docs/plans/sessions/2026-07-22-issue-14-gate-2-upstream-watch.md`
- Issue `#14` Runtime Workflow：`docs/workflows/2026-07-22-issue-14-gate-2-upstream-watch-runtime.md`
- Issue `#14` 交付报告：`docs/reports/issue-14-gate-2-upstream-watch.md`
- 已完成 Issue `#17` Gate 3 规划：`docs/plans/2026-07-22-issue-17-gate-3-rust-workspace-plan.md`
- Issue `#19` Gate 3 实现 Session Plan：`docs/plans/sessions/2026-07-22-issue-19-gate-3-rust-workspace-ci.md`
- Issue `#19` Gate 3 实现 Runtime Workflow：`docs/workflows/2026-07-22-issue-19-gate-3-rust-workspace-ci-runtime.md`
- Issue `#19` Gate 3 实现报告：`docs/reports/issue-19-gate-3-rust-workspace-ci.md`
- Issue `#17` Session Plan：`docs/plans/sessions/2026-07-22-issue-17-gate-3-rust-workspace-plan.md`
- Issue `#17` Runtime Workflow：`docs/workflows/2026-07-22-issue-17-gate-3-rust-workspace-plan-runtime.md`
- Issue `#17` 规划报告：`docs/reports/issue-17-gate-3-rust-workspace-plan.md`
- 架构决策：`docs/adr/`

## 许可证

本项目采用 GNU Affero General Public License v3.0，详见 `LICENSE`。
