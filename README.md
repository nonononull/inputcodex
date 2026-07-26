# inputcodex

`inputcodex` 是面向 Codex 本地增强与管理场景的新项目，当前已完成 Gate 1 治理冻结、Gate 2 上游监控、Gate 3 纯 Rust Workspace，以及 Gate 4 功能目录、`v1.2.42` 缓存与重新审计、双平台性能基线和性能预算 Discovery。Issue `#50` / PR `#51` 已以单父 Squash 提交 `fea8824c652665df710a7e6ef941854060eb6e1f` 进入 `main`，tree 为 `9fb518cda8b35a9388fb9fce0a1ff6ba976d80cb`，GitHub 签名 `valid`；合并后主干 CI Run `30175592979` 七 Job 全绿且 Artifact 为 `0`。Issue `#54` 已完成八次 hosted 复测但 Windows 未取得五个同队列样本，因此当前仍没有预算数值；Issue `#57` 只负责队列异构性 Discovery，Gate 5 继续锁定。

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

两份参考项目均采用 GNU AGPLv3。当前仓库同样采用 GNU AGPLv3；`BigPizzaV3/CodexPlusPlus` 正式 Release `v1.2.42` 已以完整只读审计快照导入 `upstream/CodexPlusPlus/`，半成品参考仓库尚未导入。

## 当前阶段

截至 2026 年 7 月 25 日，已完成：

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
- Issue `#35` / PR `#36` 已将活动 Release 快照与功能目录审计基线解耦；Squash 提交为 `d7438a0f2c43b7fbd2b159b3759aacea4ef1999e`。
- Issue `#34` / PR `#40` 已完成 `v1.2.42` 纯缓存同步；PR `#40` 以 Squash 提交 `353391424db5514d022473ba97f601486a190869` 合并，合并后主干 CI Run `30147841226` 七 Job 全绿。
- Issue `#41` / PR `#42` 已完成 CI 合同解耦；PR `#42` 以 Squash 提交 `8aa1d4c96b0543e766b477b1b8e9652968b55f92` 合并，合并后主干 CI Run `30147071062` 七 Job 全绿。
- Issue `#43` / PR `#44` 已完成缓存与 CI 合同状态收口；PR `#44` 以单父 Squash 提交 `fdb2f98c701800969fc478f95cd2539be598faaa` 合并，合并后主干 CI Run `30152001233` 成功。
- Issue `#38` 二十六路径实施哈希 `sha256:a384353e947bcb9d95b51ac5ccce49ef9558ca34580c130307a64b6d868819af` 已获批准；RED 提交为 `4206ef66076a4c9e9a19ce014a20f78cb3b73163`，最终 Head `d3df8759bdb9c6378497a3a0c8f409c3968f4d4f` 的 `catalog_repository` 为 `12/12`、Release Audit 为 `current`，并通过 PR `#45` Squash Merge 为 `5fd337fb7ceb9b0ef53e2e694cc5ddd81ea0a98c`。
- PR `#45` CI Run `30157623932` 七 Job 全绿且 Artifact 为 `0`；合并后主干 Run `30158058627` 的 Attempt `1/2` 因 GitHub Actions major outage 未创建任何 Job，服务恢复后的同一 Run Attempt `3` 在未修改仓库的情况下七 Job 全绿且 Artifact 为 `0`，Issue `#46` 已按 `COMPLETED` 关闭。
- Issue `#26` / PR `#27` 已完成 Gate 4 功能目录执行：最终 Head `1d1bf32cdc4edc45e2d28f1047604222ebdb51e4` 以 Squash 提交 `a9b20f00ae069aedd42c8124d2789b230187258c` 进入 `main`；merge/head tree 均为 `205c24e05e0451a3aa39af4f43f0d9853cc7a6a2`，GitHub 签名 `valid`，PR 与合并后主干 CI 均六 Job 全绿且成功 Artifact 数为 `0`。
- Issue `#28` / PR `#29` 已完成 Gate 4 独立 Closeout：最终 Head `7ee316c6bf4d9ca44f3475283ae1aee9c83f8577` 以单父 Squash 提交 `c07da0cad33e09b5c54e528a8a6728a048c88c0b` 合入 `main`，tree 为 `02ab8a3d8497ebb7b990e4078122b9bf916ef454`，GitHub 签名有效；Issue `#28` 已关闭，合并后主干 CI `29948874307` 六 Job 全部成功且 Artifact 数为 `0`。

当前明确不做：

- 不把已导入的上游审计快照接入产品构建或运行面，也不在非 upstream-sync Issue 中修改快照。
- 不导入半成品参考仓库源码。
- Issue `#32` 已完成隔离性能基线，但没有批准预算；两平台数据禁止跨平台排名，单次有效 Run 不得被解释为 required budget。
- Issue `#50` / PR `#51` 已在批准九路径内冻结性能预算方法和长期状态；该交付没有填写预算数值、修改性能实现/Workflow、实施优化或解锁 Gate 5。
- 不改写 Issue `#38` 的历史 Plan、Session Plan、Runtime Workflow 或来源提交；其阶段性叙述与最终 GitHub 事实的差异只通过 Issue `#47` Closeout 报告和长期控制面更正收口。
- 不执行功能迁移、安装包构建、发布或未经项目所有者授权的 PR 合并。
- 不让上游 Tauri/React UI、现有注入脚本和远程推荐列表进入最终运行面。

## 当前 Gate 4 功能目录边界

- Gate 2 的 Issue `#9/#12/#14` 与 PR `#11/#13/#15` 均已完成；上游监控 Workflow 持续运行，Issue `#16` 只由机器维护。
- `upstream/CodexPlusPlus/` 含 `v1.2.42` 的 `277` 个只读审计文件，`upstream/source-lock.json` 记录活动来源、tree、逐文件 blob/SHA-256、许可证、生成工具证据，以及已对齐 `v1.2.42` 的功能目录审计基线与 `current` 状态。
- Issue `#19` / PR `#21` 已将七成员纯 Rust Workspace、Rust `1.97.1` 工具链文件、`Cargo.lock`、最小分层源码与首版无缓存三平台 `CI` Workflow 合入 `main`；仍没有 Release、安装包、签名、更新资产或上游业务功能迁移。
- 当前治理合同为 `30/30`，Workspace 许可证元数据已与根 `LICENSE` 对齐为 `AGPL-3.0-only`；五类受控失败语义已全部完成普通提交 RED→GREEN，最终 PR 运行 `29918843397` 与合并后主干运行 `29919596057` 均六 Job 全绿。
- Linux、Windows、macOS 已各取得 `3/3` 次无缓存成功样本；Job 执行时间中位数分别为 `133`、`212`、`96` 秒，最低基线不包含 Cache、P95 或七天调优结论。
- Gate 3 实现顺序固定为“RED 治理合同 → GREEN 治理脚本 → 七成员 Workspace → 无缓存三平台 CI → 真实失败恢复 → 冷构建基线”。
- Iced 只能直接存在于 presentation crate；最小窗口不建立 UI 设计系统，视觉与交互由 Gemini 实现或审阅。
- Gate 4 规划合同已进入 `main`；Issue `#26` 的 source-index 与五域功能目录 checkpoint `87537e6e4a0e6911dd1427cc23f52dcb805a4679` 已记录 `133` 条入口、`36` 个 feature、`3` 个排除和 `0` 个覆盖缺口。
- Issue `#26` / PR `#27` 已将 `36` 份五域行为合同、`11` 个合成或不可逆脱敏 fixture manifest 与验证器合入 `main`；来源 PR 的 `classify`、`governance`、`linux-quality`、`windows`、`macos`、`required` 均成功，Issue 已关闭，来源功能分支的远端、本地与远端跟踪引用均已清理。
- Gate 4 初始功能目录、独立 Closeout、`v1.2.42` 缓存与二十六路径重新审计均已完成；Issue `#47` 只负责把 PR `#45`、主干 CI 恢复与 Issue `#46` 的稳定证据回写到长期控制面，不迁移产品功能。
- 最新正式功能真源为 `v1.2.42`；活动审计缓存与功能目录审计基线均已对齐该 Release，`release_audit` 为 `current`，上游 `main` 的变化仍只进入 Issue `#20` 预警。
- Issue `#32` / PR `#49` 已完成 Windows/macOS 基线采集与 Evidence；该结果是预算 Discovery 输入，不是预算批准。
- Issue `#50` / PR `#51` 已通过 ADR `0004` 冻结预算对象、可比队列、五次独立 Run、run-level 稳健统计、错误语义和阶段升级合同；PR Final Head CI 与合并后主干 CI 均通过，Gate 5 仍未解锁。
- Issue `#55` / PR `#56` 已使 `Performance Baseline` 的手工 trigger 显式选择 `evidence` 或 `measure`，默认值为 `evidence`；它不改变自动 PR/push 行为，也不实施预算 CI。Issue `#54` 已完成八次串行 hosted Run：macOS 有八个同队列可比样本，Windows 只有两个初始队列可比样本和六个有效新队列样本，故没有落盘项目所有者预授权的预算数值。

## 下一步

1. 在 Issue `#57` 中只完成 GitHub-hosted 队列异构性的 Discovery，形成保持队列语义、修订语义或选择受控 Runner 的可复核决策材料；不得先行追加采样或修改合同。
2. 仅在项目所有者批准新的独立范围后，才执行新的采样、合同修订或 Runner 决策；预算数值仍必须满足每平台五个同队列样本。
3. 仅在预算复测、数值批准、预算 CI 双平台观察和 `release_audit=current` 全部满足后，建立首个 Gate 5 功能迁移 Issue。

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
- Issue `#38` 重新审计计划：`docs/plans/2026-07-25-issue-38-v1.2.42-catalog-reaudit.md`
- Issue `#38` Session Plan：`docs/plans/sessions/2026-07-25-issue-38-v1.2.42-catalog-reaudit.md`
- Issue `#38` Runtime Workflow：`docs/workflows/2026-07-25-issue-38-v1.2.42-catalog-reaudit-runtime.md`
- Issue `#38` 发现报告：`docs/reports/issue-38-v1.2.42-catalog-reaudit-discovery.md`
- Issue `#47` 重新审计 Closeout 报告：`docs/reports/issue-47-v1.2.42-catalog-reaudit-closeout.md`
- Issue `#32` 性能基线实施计划：`docs/plans/issue-32-performance-baseline.md`
- Issue `#32` Session Plan：`docs/plans/sessions/issue-32-performance-baseline.md`
- Issue `#32` Runtime Workflow：`docs/workflows/issue-32-performance-baseline-runtime.md`
- Issue `#32` 性能基线报告：`docs/reports/issue-32-performance-baseline.md`
- 性能预算 ADR：`docs/adr/0004-performance-budget-policy.md`
- Issue `#50` Discovery 计划：`docs/plans/2026-07-25-issue-50-performance-budget-discovery.md`
- Issue `#50` Session Plan：`docs/plans/sessions/2026-07-25-issue-50-performance-budget-discovery.md`
- Issue `#50` Runtime Workflow：`docs/workflows/2026-07-25-issue-50-performance-budget-discovery-runtime.md`
- Issue `#50` Discovery 报告：`docs/reports/issue-50-performance-budget-discovery.md`
- Issue `#52` 性能预算 Discovery Closeout 报告：`docs/reports/issue-52-performance-budget-closeout.md`
- Issue `#55` 复测入口计划：`docs/plans/2026-07-26-issue-55-performance-remeasurement-entry.md`
- Issue `#55` Session Plan：`docs/plans/sessions/2026-07-26-issue-55-performance-remeasurement-entry.md`
- Issue `#55` Runtime Workflow：`docs/workflows/2026-07-26-issue-55-performance-remeasurement-entry-runtime.md`
- Issue `#55` 报告：`docs/reports/issue-55-performance-remeasurement-entry.md`
- Issue `#54` 复测与数值计划：`docs/plans/2026-07-26-issue-54-performance-remeasurement-budget-approval.md`
- Issue `#54` Session Plan：`docs/plans/sessions/2026-07-26-issue-54-performance-remeasurement-budget-approval.md`
- Issue `#54` Runtime Workflow：`docs/workflows/2026-07-26-issue-54-performance-remeasurement-budget-approval-runtime.md`
- Issue `#54` 报告：`docs/reports/issue-54-performance-remeasurement-budget-approval.md`
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
