# inputcodex

`inputcodex` 是面向 Codex 本地增强与管理场景的新项目，当前处于重构筹备阶段。

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

两份参考项目均采用 GNU AGPLv3。当前仓库同样采用 GNU AGPLv3，但尚未导入任何参考项目源码。

## 当前阶段

截至 2026 年 7 月 21 日，已完成：

- 建立本地 Git 仓库与 GitHub 公开仓库。
- 固定项目名称、无广告原则和性能优先目标。
- 建立项目级构建、排错、计划与执行约束文档。
- 通过 Issue `#2` 批准纯 Rust/Iced、完整上游快照、自主发布线和 Issue/PR 治理方案。
- 通过 PR `#3` Squash Merge 重构与发布治理单一真源、项目语境、ADR、Session Plan、Runtime Workflow、`main-protection` Ruleset 和 Rust CI 云端卸载计划。
- PR `#3` 的合并提交为 `0e11375997ff10fdc0c233b31c8468af2d9a4f44`；Issue `#2` 已关闭，旧功能分支已删除。
- 通过 Issue `#4` 的独立分支准备回写最终 closeout 证据和最新 Master Plan。

当前明确不做：

- 不导入上游或半成品源码。
- 不创建 Rust/Iced 工程或 GitHub Actions。
- 不执行功能迁移、安装包构建、发布或 PR 合并。
- 不让上游 Tauri/React UI、现有注入脚本和远程推荐列表进入最终运行面。

## 下一步

1. 完成 Issue `#4` closeout PR 的项目所有者 Review 与 Squash Merge；该 PR 未经再次授权不得自动合并。
2. 通过独立 Issue/PR 补齐 Issue/PR 模板和标签；`main` 分支保护已经落地并完成合并后复核。
3. Gate 1 完成后，新建 `upstream-sync` Issue 导入 `v1.2.41` 完整快照并建立每 6 小时上游监控。
4. 后续再分别建立 Rust/Iced 骨架、功能矩阵和性能基线 Issue；任何源码实现都需要新的 Session Plan 与批准。

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
- 当前 closeout 计划：`docs/plans/2026-07-21-issue-4-gate-1-closeout.md`
- 当前会话计划：`docs/plans/sessions/2026-07-21-issue-4-gate-1-closeout.md`
- 当前运行工作流：`docs/workflows/2026-07-21-issue-4-gate-1-closeout-runtime.md`
- 架构决策：`docs/adr/`

## 许可证

本项目采用 GNU Affero General Public License v3.0，详见 `LICENSE`。
