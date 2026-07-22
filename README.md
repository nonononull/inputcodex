# inputcodex

`inputcodex` 是面向 Codex 本地增强与管理场景的新项目，当前已完成 Gate 1 治理冻结、Gate 2 上游 `v1.2.41` 审计快照与每 6 小时监控，并已通过 Issue `#19/#22`、PR `#21/#23` 完成 Gate 3 纯 Rust Workspace、五类失败语义、三平台最低冷构建基线和独立 closeout；Issue `#24` / PR `#25` 已完成 Gate 4 规划合同，Issue `#26` / PR `#27` 已完成 source-index、五域功能目录、行为合同与脱敏 fixture 的实现、Review/CI 与 Squash Merge，Issue `#28` / PR `#29` 已完成独立 Closeout。下一项可启动工作是独立性能基线 Issue；基线、预算与优化继续分离，Gate 5 保持锁定。

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
- 通过 PR `#21` Squash Merge Gate 3 七成员 Workspace、首版无缓存三平台 CI、五类失败语义和冷构建最低基线；合并提交为 `0716ec0debcd3e059cc4ca88a072232841ca73b4`，Issue `#19` 已按 `COMPLETED` 关闭。
- PR `#21` 的最终 Head 为 `9a4a4425f2fb0d8235554d3e83577111ae34efcc`；merge/head tree 均为 `4881ce609370f77181d9545474c029ab0c5d4972`，GitHub 签名 `valid`，合并后 `main` 运行 `29919596057` 六 Job 全绿且成功 Artifact 数为 `0`。
- 通过 PR `#23` Squash Merge Gate 3 独立 closeout；合并提交为 `f470c062037042a1f7833a29cdcf216f6c0f5601`，Issue `#22` 已按 `COMPLETED` 关闭，合并后 `main` 运行 `29922385227` 六 Job 全绿且成功 Artifact 数为 `0`。
- Issue `#24` / PR `#25` 已 Squash Merge Gate 4 两阶段规划合同；合并提交为 `431682296f53e86de1184c732b0d4748857c9390`，Issue `#24` 已按 `COMPLETED` 关闭，合并后 `main` 运行 `29926710342` 六 Job 全绿且成功 Artifact 数为 `0`。
- Issue `#26` / PR `#27` 已完成 Gate 4 功能目录执行：最终 Head `1d1bf32cdc4edc45e2d28f1047604222ebdb51e4` 以 Squash 提交 `a9b20f00ae069aedd42c8124d2789b230187258c` 进入 `main`；merge/head tree 均为 `205c24e05e0451a3aa39af4f43f0d9853cc7a6a2`，GitHub 签名 `valid`，PR 与合并后主干 CI 均六 Job 全绿且成功 Artifact 数为 `0`。
- Issue `#28` / PR `#29` 已完成 Gate 4 独立 Closeout：最终 Head `7ee316c6bf4d9ca44f3475283ae1aee9c83f8577` 以单父 Squash 提交 `c07da0cad33e09b5c54e528a8a6728a048c88c0b` 合入 `main`，tree 为 `02ab8a3d8497ebb7b990e4078122b9bf916ef454`，GitHub 签名有效；Issue `#28` 已关闭，合并后主干 CI `29948874307` 六 Job 全部成功且 Artifact 数为 `0`。

当前明确不做：

- 不把已导入的上游审计快照接入产品构建或运行面，也不在非 upstream-sync Issue 中修改快照。
- 不导入半成品参考仓库源码。
- 未经独立性能基线 Issue 的范围冻结与项目所有者批准，不创建 `benchmarks/`、测量脚本、性能预算或优化，也不修改 CI Workflow、发布资产或上游快照。
- 不执行功能迁移、安装包构建、发布或未经项目所有者授权的 PR 合并。
- 不让上游 Tauri/React UI、现有注入脚本和远程推荐列表进入最终运行面。

## 当前 Gate 4 功能目录边界

- Gate 2 的 Issue `#9/#12/#14` 与 PR `#11/#13/#15` 均已完成；上游监控 Workflow 持续运行，Issue `#16` 只由机器维护。
- `upstream/CodexPlusPlus/` 含 `v1.2.41` 的 `277` 个只读审计文件，`upstream/source-lock.json` 记录来源、tree、逐文件 blob/SHA-256、许可证和生成工具证据。
- Issue `#19` / PR `#21` 已将七成员纯 Rust Workspace、Rust `1.97.1` 工具链文件、`Cargo.lock`、最小分层源码与首版无缓存三平台 `CI` Workflow 合入 `main`；仍没有 Release、安装包、签名、更新资产或上游业务功能迁移。
- 当前治理合同为 `30/30`，Workspace 许可证元数据已与根 `LICENSE` 对齐为 `AGPL-3.0-only`；五类受控失败语义已全部完成普通提交 RED→GREEN，最终 PR 运行 `29918843397` 与合并后主干运行 `29919596057` 均六 Job 全绿。
- Linux、Windows、macOS 已各取得 `3/3` 次无缓存成功样本；Job 执行时间中位数分别为 `133`、`212`、`96` 秒，最低基线不包含 Cache、P95 或七天调优结论。
- Gate 3 实现顺序固定为“RED 治理合同 → GREEN 治理脚本 → 七成员 Workspace → 无缓存三平台 CI → 真实失败恢复 → 冷构建基线”。
- Iced 只能直接存在于 presentation crate；最小窗口不建立 UI 设计系统，视觉与交互由 Gemini 实现或审阅。
- Gate 4 规划合同已进入 `main`；Issue `#26` 的 source-index 与五域功能目录 checkpoint `87537e6e4a0e6911dd1427cc23f52dcb805a4679` 已记录 `133` 条入口、`36` 个 feature、`3` 个排除和 `0` 个覆盖缺口。
- Issue `#26` / PR `#27` 已将 `36` 份五域行为合同、`11` 个合成或不可逆脱敏 fixture manifest 与验证器合入 `main`；来源 PR 的 `classify`、`governance`、`linux-quality`、`windows`、`macos`、`required` 均成功，Issue 已关闭，来源功能分支的远端、本地与远端跟踪引用均已清理。
- Gate 4 功能目录与独立 Closeout 已完成；功能目录、性能基线与优化继续保持不同 Issue/PR，Gate 5 仍锁定。
- 最新正式功能真源仍为 `v1.2.41`；上游 `main` 当前变化只进入 Issue `#20` 预警，不自动改变 Gate 4 基线。

## 下一步

1. 创建独立性能基线 Issue，冻结测量对象、固定参考来源/许可证/构建方式、可比环境、范围哈希与项目所有者批准。
2. 在性能基线 Issue 中只建立场景、测量脚本、原始样本、统计摘要与预算候选；不得顺带优化产品。
3. 在性能基线结论与预算候选经批准前，Gate 5 继续锁定；优化和分域迁移各自使用新的独立 Issue/PR。

## 项目文档

- 构建说明：`build.md`
- 排错记录：`err.md`
- 项目总计划：`docs/plans/PROJECT-MASTER-PLAN.md`
- Gate 4 规划任务：`docs/plans/2026-07-22-issue-24-gate-4-feature-performance-plan.md`
- Gate 4 Session Plan：`docs/plans/sessions/2026-07-22-issue-24-gate-4-feature-performance-plan.md`
- Gate 4 Runtime Workflow：`docs/workflows/2026-07-22-issue-24-gate-4-feature-performance-runtime.md`
- Gate 4 初始报告：`docs/reports/issue-24-gate-4-feature-performance-plan.md`
- Issue `#26` 实现计划：`docs/plans/2026-07-22-issue-26-gate-4-feature-catalog-implementation.md`
- Issue `#26` Session Plan：`docs/plans/sessions/2026-07-22-issue-26-gate-4-feature-catalog.md`
- Issue `#26` Runtime Workflow：`docs/workflows/2026-07-22-issue-26-gate-4-feature-catalog-runtime.md`
- Issue `#26` 初始报告：`docs/reports/issue-26-gate-4-feature-catalog.md`
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
