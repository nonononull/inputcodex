# inputcodex 项目总计划

schema_version: inputcodex.master-plan.v1
active_task: 2026-07-22-issue-35-release-catalog-decoupling
active_gate: Gate 4：功能目录与独立 Closeout 已完成；先解除上游缓存与目录审计的错误耦合，再推进缓存同步、重新审计和独立性能基线
last_verified_gate: Issue #35 已取得十四路径范围批准；Fresh 基线为 main 939f345、上游最新正式 Release v1.2.42@657cd33，Rust 与 CI 门禁定向 GREEN 已完成，PR 尚未创建
next_legal_gate: 完成 Issue #35 的 Review/CI/PR 后，以独立 upstream-sync Issue 缓存 v1.2.42 并显式进入 stale；完成对应目录重新审计恢复 current 后，才可执行已立项的独立性能基线 Issue #32
tracking_issue_ref: https://github.com/nonononull/inputcodex/issues/35
source_implementation_issue_ref: https://github.com/nonononull/inputcodex/issues/26
source_implementation_pr_ref: https://github.com/nonononull/inputcodex/pull/27
closed_gate_3_closeout_issue_ref: https://github.com/nonononull/inputcodex/issues/22
closed_implementation_issue_ref: https://github.com/nonononull/inputcodex/issues/19
gate_3_planning_issue_ref: https://github.com/nonononull/inputcodex/issues/17
upstream_watch_issue_ref: https://github.com/nonononull/inputcodex/issues/14
transition_issue_ref: https://github.com/nonononull/inputcodex/issues/8
upstream_sync_issue_ref: https://github.com/nonononull/inputcodex/issues/9
active_branch_ref: codex/issue-35-release-catalog-decoupling
transition_branch_ref: codex/issue-8-gate-2-transition
active_plan_ref: docs/plans/2026-07-22-issue-35-release-catalog-decoupling.md
active_session_plan_ref: docs/plans/sessions/2026-07-22-issue-35-release-catalog-decoupling.md
active_runtime_workflow_ref: docs/workflows/2026-07-22-issue-35-release-catalog-decoupling-runtime.md
active_pr_ref: not-created-for-issue-35
gate_3_closeout_pr_ref: https://github.com/nonononull/inputcodex/pull/23
gate_3_implementation_pr_ref: https://github.com/nonononull/inputcodex/pull/21
gate_3_planning_pr_ref: https://github.com/nonononull/inputcodex/pull/18
transition_pr_ref: https://github.com/nonononull/inputcodex/pull/10
upstream_sync_pr_ref: https://github.com/nonononull/inputcodex/pull/11
closed_delivery_ref: https://github.com/nonononull/inputcodex/pull/3, https://github.com/nonononull/inputcodex/pull/5, https://github.com/nonononull/inputcodex/pull/7, https://github.com/nonononull/inputcodex/pull/10, https://github.com/nonononull/inputcodex/pull/11, https://github.com/nonononull/inputcodex/pull/13, https://github.com/nonononull/inputcodex/pull/15, https://github.com/nonononull/inputcodex/pull/18, https://github.com/nonononull/inputcodex/pull/21, https://github.com/nonononull/inputcodex/pull/23, https://github.com/nonononull/inputcodex/pull/25, https://github.com/nonononull/inputcodex/pull/27
active_report_ref: docs/reports/issue-35-release-catalog-decoupling.md
gate_3_closeout_report_ref: docs/reports/issue-22-gate-3-closeout.md
gate_3_implementation_report_ref: docs/reports/issue-19-gate-3-rust-workspace-ci.md
gate_2_watch_report_ref: docs/reports/issue-14-gate-2-upstream-watch.md
active_ruleset_ref: https://github.com/nonononull/inputcodex/rules/19395456
active_ci_strategy_ref: docs/plans/2026-07-21-rust-ci-offload-strategy.md
active_ci_implementation_plan_ref: docs/plans/2026-07-21-rust-ci-offload-implementation-plan.md
decision_status: issue-35-release-audit-decoupling-approved-implementation-before-pr-performance-baseline-and-gate-5-blocked-until-current

## 当前状态

- Gate 1 已完成：Issue `#2` / PR `#3`、Issue `#4` / PR `#5`、Issue `#6` / PR `#7` 均已按治理链完成；筹备 Issue `#1` 已以 `completed` 关闭。
- PR `#7` 合并提交为 `c74b66422ba47f96bd3eb2b2385cdfb90541808e`，由 GitHub 生成有效签名；只有一个父提交 `b7404b0c63f2d2ba65474c077182c42a01cc9a64`，tree 为 `00f0f7fe0e408a1e6f218ee8e1be0d8442ed1e65`。
- PR `#7` 的 Review 对话总数、未解决数与 Checks 数量均为 `0`；`0 Checks` 只表示当前尚未配置 CI。
- `main-protection` Ruleset（ID `19395456`）仍为 `active`，只命中 `main`，禁止删除与 Force Push，要求解决全部 Review 对话，只允许 Squash Merge，单人阶段 required approvals 为 `0`。
- 当前完整缓存和功能目录审计基线仍为 `v1.2.41`，提交为 `3dafffcafb2566a1e8bce4b35671656d6adb3eda`，tree 为 `22e3a9c8ad15e18b972eae44a892b7980dca5ec2`。
- Issue `#9` / PR `#11` 已完成 Gate 2 上游基线导入；PR `#11` 于 `2026-07-21T19:01:02Z` Squash Merge，合并提交为 `dde08b725eb2bf4add7fbcfa955f3eaf4eb1bbc6`，Issue `#9` 已关闭。
- `upstream/CodexPlusPlus/` 当前包含 `277` 个审计文件，`upstream/source-lock.json` 记录 `24,175,877` 字节、manifest SHA-256 `3c9b16802f49a1bcb56fda9630d97edc52c918c30d1924145244d9239801d3d4` 和 `7` 份许可证/声明。
- Issue `#12` / PR `#13` 已完成上游基线 closeout；PR `#13` 的 Squash Merge 提交为 `5e64015075ddf2adef4bf685f50977b47b7f72e7`，Issue `#12` 已关闭。
- Issue `#14` / PR `#15` 已完成每 6 小时上游监控；最终 PR CI、两次 `main` 真实运行、唯一状态 Issue `#16`、分支清理和有效 GitHub 签名均已闭环。
- Issue `#17` / PR `#18` 已完成 Gate 3 规划交付；PR `#18` 的 Squash Merge 提交为 `477d110a9b284e127af365f5278901bcfa69e093`，Issue `#17` 已关闭。
- Issue `#19` / PR `#21` 已完成 Gate 3 实现：治理合同 `30/30`、七成员 Workspace、首版无缓存三平台 CI、五类失败语义与三平台各 `3/3` 次最低冷构建基线均已进入 `main`。
- PR `#21` 于 `2026-07-22T12:25:59Z` Squash Merge 为 `0716ec0debcd3e059cc4ca88a072232841ca73b4`；Issue `#19` 已按 `COMPLETED` 关闭，合并后 `main` 运行 `29919596057` 六 Job 全绿且成功 Artifact 数为 `0`。
- Issue `#22` / PR `#23` 已完成 Gate 3 独立 closeout；PR `#23` 于 `2026-07-22T13:05:34Z` Squash Merge 为 `f470c062037042a1f7833a29cdcf216f6c0f5601`，Issue `#22` 已按 `COMPLETED` 关闭，合并后 `main` 运行 `29922385227` 六 Job 全绿且成功 Artifact 数为 `0`。
- Issue `#24` / PR `#25` 已完成“两阶段拆分”规划合同；PR `#25` 于 2026 年 7 月 22 日 Squash Merge 为 `431682296f53e86de1184c732b0d4748857c9390`，Issue `#24` 已按 `COMPLETED` 关闭，合并后 `main` 运行 `29926710342` 六 Job 全绿且成功 Artifact 数为 `0`。
- Issue `#26` / PR `#27` 已完成功能目录执行：`133` 条入口映射为 `36` 个 feature、`3` 个排除、`10` 个 `exception-pending` 与 `0` 个覆盖缺口；`36` 份合同、`11` 个 fixture manifest 和验证器已合入 main。独立 Closeout 已由 Issue `#28` / PR `#29` 完成：Squash 提交 `c07da0cad33e09b5c54e528a8a6728a048c88c0b`、单父、tree `02ab8a3d8497ebb7b990e4078122b9bf916ef454`、GitHub 签名有效、合并后 main CI `29948874307` 六 Job 全成功且 Artifact 数为 `0`。
- Issue `#33` 已发现上游最新正式 Release `v1.2.42` / `657cd33e009ad02515d30db6492cd4e669b06318`；Issue `#34` 审计确认旧验证会把“已缓存但未复审”误判为损坏，故保持 blocked，未伪造目录一致性。
- Issue `#35` 已获十四路径范围批准，正在实现 `release_audit` 解耦、可诊断 stale 状态和 PR 路径门禁；本任务不更新 `upstream/CodexPlusPlus/` 字节、不创建性能基线或产品迁移。
- Issue `#8` 的过渡交付为 PR `#10`；该 PR 只包含文档与验证控制面，并按项目所有者明确授权执行 Squash Merge。
- AGOS 仍是可选外部辅助；本仓库可用原生控制面时不运行它，不在本任务中修改或优化它。

## 项目不变量

- 软件名称固定为 `inputcodex`。
- 性能优先不能通过静默删除有效功能实现。
- 有效功能默认与上游最新正式 Release 保持行为一致。
- Windows 与 macOS 从首版起功能一致。
- 禁止 TypeScript、JavaScript 业务代码和 WebView；产品使用 Rust 与 Iced。
- Iced 只能存在于展示层，领域、应用、基础设施和平台层不得依赖 Iced 类型。
- 广告、推广、导流和隐蔽遥测不得进入最终运行面。
- 上游完整快照只用于审计、映射和重构追踪，不参与新产品构建。
- 上游 Tauri/React 管理界面、现有注入脚本和远程推荐列表不得直接进入新架构或最终运行面。
- 无效功能、有害副作用和错误语义争议必须进入 `parity-exception` Issue，由项目所有者决定。
- 客户端更新、安装包、签名与下载地址只属于 `nonononull/inputcodex`。
- 所有 PR 合并到 `main` 只允许 Squash Merge；禁止 Merge Commit 和 Rebase Merge。
- `main` 永久禁止 Force Push、删除和绕过 Ruleset；错误历史只能通过关联 Issue/PR 的 revert 处理。
- 所有 Review 对话必须在确定根因、完成处理并回写验证证据后才能解决和合并。
- 单人维护阶段 required approvals 为 `0`，但必须保留所有者决策证据；第二名具备合并权限的人类维护者加入后，在下一次合并前提升为 `1`。
- Rust 全量编译与双平台验证默认在标准 GitHub-hosted runners 完成；Issue `#19` 是唯一获批的 Gate 3 Workspace 与首版 CI 实现任务。
- 权威控制面是 `AGENTS.md`、`README.md`、`build.md`、`err.md`、本 Master Plan、任务计划和 GitHub 证据；外部框架只能提供可选辅助。

## 阶段索引

### Gate 0：仓库准备（已验证）

- 已建立本地与 GitHub 公开仓库、许可证、根文档、Issue #1 和筹备 closeout。

### Gate 1：方案与治理冻结（已完成）

- [x] 冻结纯 Rust/Iced、性能优先、功能一致、双平台一致和无广告硬约束。
- [x] 完成架构治理、Ruleset、CI 云端卸载策略和项目原生验证入口。
- [x] PR `#3`、`#5`、`#7` 均以 Squash Merge 合入 `main`。
- [x] Issue `#1`、`#2`、`#4`、`#6` 均完成关闭证据；PR #7 旧分支已清理。
- [x] Issue Forms、PR 模板与项目标签已进入 `main`。

### Gate 2：导入上游基线与监控（已完成）

- [x] 创建 Issue `#9`，锁定当前上游正式 Release `v1.2.41` 与提交。
- [x] 创建 Gate 2 Session Plan、Runtime Workflow 和来源/许可证/纯净性验证范围。
- [x] 通过 Issue `#8` / PR `#10` 完成 Gate 1→2 控制面过渡。
- [x] 获得 Issue `#9` 的快照导入范围和项目所有者合并批准。
- [x] 通过独立 upstream-sync PR `#11` 只更新 `upstream/`、source-lock 和同步报告，并 Squash Merge 到 `main`。
- [x] 通过 Issue `#12` / PR `#13` 回写 merge ref、`build.md`、`err.md` 和最新控制面。
- [x] 通过 Issue `#14` / PR `#15` 建立每 6 小时只管理 Issue 的上游监控，并完成两次真实 Actions 幂等验证。

### Gate 3：纯 Rust 工作区骨架（已完成）

- [x] 通过 Issue `#17` / PR `#18` 冻结分层 Workspace、Iced 隔离、平台端口、加载状态、性能诊断和三平台 CI 合同。
- [x] 创建实现 Issue `#19`、独立分支、Session Plan、Runtime Workflow 与初始报告，并取得项目所有者实现批准。
- [x] 先建立 `scripts/ci` 的可信 RED/GREEN 治理合同，再创建七成员 Workspace；当前合同为 `30/30`，七成员 Workspace checkpoint 已推送。
- [x] 在同一实现 PR 中通过标准 Linux、Windows、macOS Runner、`required` 汇总、五类真实失败恢复和三平台各 `3/3` 次无缓存冷构建基线。
- [x] PR `#21` 最终 Head 六 Job 全绿、Review 对话为 `0`，已按明确授权 Squash Merge；Issue `#19` 已关闭，合并后主干 CI 全绿。
- [x] Issue `#22` / PR `#23` 已完成独立 closeout，merge/tree/签名/Issue/CI/Review/Ruleset 和分支删除证据均已闭环。
- 不迁移业务功能，不创建临时 UI 事实标准；最小窗口的视觉和交互默认由 Gemini 实现或审阅。

### Gate 4：功能目录已收口，性能基线待独立执行

- [x] 创建 Issue `#24`，批准采用“规划合同 → 两个独立执行 Issue”的拆分方案。
- [x] 冻结功能矩阵的稳定标识、证据路径、行为字段、既有一致性状态和决策引用。
- [x] 冻结行为合同、脱敏夹具规则、性能测量协议和可比环境要求，不填写未经实测的预算。
- [x] 定义“功能矩阵/行为合同/脱敏夹具”执行 Issue 与“性能基线/预算批准”执行 Issue 的互斥边界。
- [x] PR `#25` 通过 Review/CI 并按项目所有者授权 Squash Merge；Issue `#24` 已完成独立 closeout。
- [x] 创建功能目录执行 Issue `#26` 与独立分支。
- [x] 提交 Issue `#26` 任务计划、Session Plan、Runtime Workflow、36 条范围和新 scope hash checkpoint，并取得项目所有者实现批准。
- [x] 完成 RED schema、GREEN Rust 验证器与 source-index/五域功能目录 checkpoint；不得迁移产品功能。
- [x] 建立五域 `36` 份行为合同与必要的 `11` 个脱敏 fixture manifest，并完成完整本地仓库验证；产品、CI、Ruleset、Release、`upstream/`、`benchmarks/` 和 AGOS 保持零差异。
- [x] PR `#27` 已完成 Review/CI 并按项目所有者对具体 PR 与最终 Head 的授权 Squash Merge；Issue `#26` 已关闭，来源分支本地、远端和远端跟踪引用均已清理。
- [x] Issue `#28` / PR `#29` 已以独立 Closeout 回写来源 Issue、PR、Review、CI、Squash、签名、tree 与分支清理证据；PR `#29` 已按项目所有者授权 Squash Merge，Issue `#28` 已关闭，本任务未创建性能基线或优化。
- [ ] Issue `#35` 将完整快照与功能目录审计基线解耦；其 PR 通过后，独立 upstream-sync PR 才可缓存 `v1.2.42` 并进入显式 stale。
- [ ] Issue `#32` 独立性能基线已立项，但在缓存 stale 或目录重新审计未恢复 `current` 时不得实现、验证或合并；基线与优化保持不同 Issue/PR，必须重新冻结范围哈希、可比环境与项目所有者批准。

### Gate 5：分域迁移（锁定）

- 按基础能力、供应商与网络、会话与数据、插件与脚本、远程集成与安装分域迁移。
- 每个可独立验收功能使用独立 Issue 和 PR，上游同步与功能重构永远分离。
- `release_audit` 不是 `current` 时，任何 Gate 5 产品迁移 PR 都必须被门禁阻断。

### Gate 6：首个正式版本（锁定）

- 完成功能矩阵、双平台、性能预算、差异批准、签名、安装、升级、回滚和自主更新源。
- 首个目标版本为 `v1.2.41-inputcodex.1`。

## 当前验证入口

- 构建与当前 Gate 验证：`build.md`。
- 排错与已知限制：`err.md`。
- Release 审计基线解耦 ADR：`docs/adr/0003-release-snapshot-catalog-audit-decoupling.md`。
- 当前 Issue `#35` 实施计划：`docs/plans/2026-07-22-issue-35-release-catalog-decoupling.md`。
- 当前 Issue `#35` Session Plan：`docs/plans/sessions/2026-07-22-issue-35-release-catalog-decoupling.md`。
- 当前 Issue `#35` Runtime Workflow：`docs/workflows/2026-07-22-issue-35-release-catalog-decoupling-runtime.md`。
- 当前 Issue `#35` 报告：`docs/reports/issue-35-release-catalog-decoupling.md`。
- 单一架构真源：`docs/plans/2026-07-21-architecture-governance.md`。
- Gate 1 最终 closeout：`docs/reports/issue-6-gate-1-finalization-closeout.md`。
- Gate 1→2 过渡计划：`docs/plans/2026-07-21-issue-8-gate-2-transition.md`。
- 已完成 Gate 2 同步计划：`docs/plans/2026-07-21-issue-9-gate-2-upstream-baseline.md`。
- 上游同步报告：`docs/reports/2026-07-21-upstream-v1.2.41-sync.md`。
- 当前 closeout 计划：`docs/plans/2026-07-21-issue-12-gate-2-upstream-closeout.md`。
- Gate 2 基线 closeout Session Plan：`docs/plans/sessions/2026-07-21-issue-12-gate-2-upstream-closeout.md`。
- Gate 2 基线 closeout Runtime Workflow：`docs/workflows/2026-07-21-issue-12-gate-2-upstream-closeout-runtime.md`。
- Gate 2 基线 closeout 报告：`docs/reports/issue-12-gate-2-upstream-closeout.md`。
- 已完成上游监控计划：`docs/plans/2026-07-22-issue-14-gate-2-upstream-watch.md`。
- Gate 2 上游监控报告：`docs/reports/issue-14-gate-2-upstream-watch.md`。
- Gate 3 架构规划：`docs/plans/2026-07-22-issue-17-gate-3-rust-workspace-plan.md`。
- 已完成实现 Session Plan：`docs/plans/sessions/2026-07-22-issue-19-gate-3-rust-workspace-ci.md`。
- 已完成实现 Runtime Workflow：`docs/workflows/2026-07-22-issue-19-gate-3-rust-workspace-ci-runtime.md`。
- 已完成实现报告：`docs/reports/issue-19-gate-3-rust-workspace-ci.md`。
- 无缓存冷构建基线：`docs/reports/rust-ci-cold-baseline.md`。
- 当前 Gate 3 closeout 计划：`docs/plans/2026-07-22-issue-22-gate-3-closeout.md`。
- 当前 Gate 3 closeout Session Plan：`docs/plans/sessions/2026-07-22-issue-22-gate-3-closeout.md`。
- 当前 Gate 3 closeout Runtime Workflow：`docs/workflows/2026-07-22-issue-22-gate-3-closeout-runtime.md`。
- 当前 Gate 3 closeout 报告：`docs/reports/issue-22-gate-3-closeout.md`。
- 当前 Gate 4 规划：`docs/plans/2026-07-22-issue-24-gate-4-feature-performance-plan.md`。
- 当前 Gate 4 Session Plan：`docs/plans/sessions/2026-07-22-issue-24-gate-4-feature-performance-plan.md`。
- 当前 Gate 4 Runtime Workflow：`docs/workflows/2026-07-22-issue-24-gate-4-feature-performance-runtime.md`。
- 当前 Gate 4 初始报告：`docs/reports/issue-24-gate-4-feature-performance-plan.md`。
- 已完成 Issue `#26` 实现计划：`docs/plans/2026-07-22-issue-26-gate-4-feature-catalog-implementation.md`。
- 已完成 Issue `#26` Session Plan：`docs/plans/sessions/2026-07-22-issue-26-gate-4-feature-catalog.md`。
- 已完成 Issue `#26` Runtime Workflow：`docs/workflows/2026-07-22-issue-26-gate-4-feature-catalog-runtime.md`。
- 已完成 Issue `#26` 报告：`docs/reports/issue-26-gate-4-feature-catalog.md`。
- 已完成 Gate 4 Closeout 计划：`docs/plans/2026-07-22-issue-28-gate-4-feature-catalog-closeout.md`。
- 已完成 Gate 4 Closeout Session Plan：`docs/plans/sessions/2026-07-22-issue-28-gate-4-feature-catalog-closeout.md`。
- 已完成 Gate 4 Closeout Runtime Workflow：`docs/workflows/2026-07-22-issue-28-gate-4-feature-catalog-closeout-runtime.md`。
- 已完成 Gate 4 Closeout 报告：`docs/reports/issue-28-gate-4-feature-catalog-closeout.md`。

## 停止条件

- 上游最新正式 Release 或已核验的 `v1.2.42` / `657cd33e009ad02515d30db6492cd4e669b06318` 事实发生变化。
- 需要修改 `upstream/` 或 `source-lock.json`，但没有新的独立 upstream-sync Issue/PR 与项目所有者批准。
- `release_audit` 为 stale 时修改 `benchmarks/`、`apps/`、产品 crate、`Cargo.toml` 或 `Cargo.lock`，或在同一 PR 同时更新实际 audit 与受阻产品路径。
- 在独立性能基线 Issue 获得范围与项目所有者批准前创建 `benchmarks/`、测量脚本、原始样本、性能预算候选，或修改 Cargo/Rust、测试、CI、upstream、Ruleset、发布资产或 AGOS。
- 在独立性能基线 Issue 中创建性能优化、产品迁移、`parity-exception`、运行上游/半成品或填写绝对性能预算，但没有新的独立 Issue 与项目所有者批准。
- Fresh 验证失败、Ruleset 变化、Review 对话未闭环或出现未批准的一致性差异。
