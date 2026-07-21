# inputcodex 项目总计划

schema_version: inputcodex.master-plan.v1
active_task: 2026-07-21-issue-12-gate-2-upstream-closeout
active_gate: Gate 2：上游基线合并证据收口
last_verified_gate: Gate 2：Issue #9 / PR #11 已 Squash Merge，上游 v1.2.41 审计快照已进入 main
next_legal_gate: 完成 Issue #12 closeout PR 并获得项目所有者下一任务决策；此前不得修改 upstream 或进入 Gate 3
tracking_issue_ref: https://github.com/nonononull/inputcodex/issues/12
transition_issue_ref: https://github.com/nonononull/inputcodex/issues/8
upstream_sync_issue_ref: https://github.com/nonononull/inputcodex/issues/9
active_branch_ref: codex/issue-12-gate-2-upstream-closeout
transition_branch_ref: codex/issue-8-gate-2-transition
active_plan_ref: docs/plans/2026-07-21-issue-12-gate-2-upstream-closeout.md
active_session_plan_ref: docs/plans/sessions/2026-07-21-issue-12-gate-2-upstream-closeout.md
active_runtime_workflow_ref: docs/workflows/2026-07-21-issue-12-gate-2-upstream-closeout-runtime.md
active_pr_ref: https://github.com/nonononull/inputcodex/pull/13
transition_pr_ref: https://github.com/nonononull/inputcodex/pull/10
upstream_sync_pr_ref: https://github.com/nonononull/inputcodex/pull/11
closed_delivery_ref: https://github.com/nonononull/inputcodex/pull/3, https://github.com/nonononull/inputcodex/pull/5, https://github.com/nonononull/inputcodex/pull/7, https://github.com/nonononull/inputcodex/pull/10, https://github.com/nonononull/inputcodex/pull/11
closeout_report_ref: docs/reports/issue-12-gate-2-upstream-closeout.md
active_ruleset_ref: https://github.com/nonononull/inputcodex/rules/19395456
active_ci_strategy_ref: docs/plans/2026-07-21-rust-ci-offload-strategy.md
active_ci_implementation_plan_ref: docs/plans/2026-07-21-rust-ci-offload-implementation-plan.md
decision_status: gate-2-upstream-baseline-merged-closeout-in-progress

## 当前状态

- Gate 1 已完成：Issue `#2` / PR `#3`、Issue `#4` / PR `#5`、Issue `#6` / PR `#7` 均已按治理链完成；筹备 Issue `#1` 已以 `completed` 关闭。
- PR `#7` 合并提交为 `c74b66422ba47f96bd3eb2b2385cdfb90541808e`，由 GitHub 生成有效签名；只有一个父提交 `b7404b0c63f2d2ba65474c077182c42a01cc9a64`，tree 为 `00f0f7fe0e408a1e6f218ee8e1be0d8442ed1e65`。
- PR `#7` 的 Review 对话总数、未解决数与 Checks 数量均为 `0`；`0 Checks` 只表示当前尚未配置 CI。
- `main-protection` Ruleset（ID `19395456`）仍为 `active`，只命中 `main`，禁止删除与 Force Push，要求解决全部 Review 对话，只允许 Squash Merge，单人阶段 required approvals 为 `0`。
- 上游正式 Release 基线仍为 `v1.2.41`，提交为 `3dafffcafb2566a1e8bce4b35671656d6adb3eda`，tree 为 `22e3a9c8ad15e18b972eae44a892b7980dca5ec2`。
- Issue `#9` / PR `#11` 已完成 Gate 2 上游基线导入；PR `#11` 于 `2026-07-21T19:01:02Z` Squash Merge，合并提交为 `dde08b725eb2bf4add7fbcfa955f3eaf4eb1bbc6`，Issue `#9` 已关闭。
- `upstream/CodexPlusPlus/` 当前包含 `277` 个审计文件，`upstream/source-lock.json` 记录 `24,175,877` 字节、manifest SHA-256 `3c9b16802f49a1bcb56fda9630d97edc52c918c30d1924145244d9239801d3d4` 和 `7` 份许可证/声明。
- 当前活动任务为 Issue `#12`，只负责回写合并证据与项目原生控制面；仓库仍无 Cargo Workspace、产品 Rust/Iced 源码或 GitHub Actions。
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
- Rust 全量编译与双平台验证默认在标准 GitHub-hosted runners 完成；当前 Gate 2 不创建 Workflow。
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

### Gate 2：导入上游基线（合并完成，收口中）

- [x] 创建 Issue `#9`，锁定当前上游正式 Release `v1.2.41` 与提交。
- [x] 创建 Gate 2 Session Plan、Runtime Workflow 和来源/许可证/纯净性验证范围。
- [x] 通过 Issue `#8` / PR `#10` 完成 Gate 1→2 控制面过渡。
- [x] 获得 Issue `#9` 的快照导入范围和项目所有者合并批准。
- [x] 通过独立 upstream-sync PR `#11` 只更新 `upstream/`、source-lock 和同步报告，并 Squash Merge 到 `main`。
- [ ] 通过 Issue `#12` 的独立 closeout PR 回写 merge ref、`build.md`、`err.md` 和最新控制面。
- [ ] 在快照导入后建立每 6 小时只管理 Issue 的上游监控 PR。

### Gate 3：纯 Rust 工作区骨架（锁定）

- 建立分层 Cargo Workspace、Iced 最小双平台窗口和依赖方向测试。
- 通过标准 Linux/Windows/macOS CI 验证，但不迁移业务功能或创建临时 UI 事实标准。

### Gate 4：功能目录与性能基线（锁定）

- 从上游快照生成首版功能矩阵、行为合同、脱敏夹具和性能预算。

### Gate 5：分域迁移（锁定）

- 按基础能力、供应商与网络、会话与数据、插件与脚本、远程集成与安装分域迁移。
- 每个可独立验收功能使用独立 Issue 和 PR，上游同步与功能重构永远分离。

### Gate 6：首个正式版本（锁定）

- 完成功能矩阵、双平台、性能预算、差异批准、签名、安装、升级、回滚和自主更新源。
- 首个目标版本为 `v1.2.41-inputcodex.1`。

## 当前验证入口

- 构建与 Gate 2 快照/closeout 验证：`build.md`。
- 排错与已知限制：`err.md`。
- 单一架构真源：`docs/plans/2026-07-21-architecture-governance.md`。
- Gate 1 最终 closeout：`docs/reports/issue-6-gate-1-finalization-closeout.md`。
- Gate 1→2 过渡计划：`docs/plans/2026-07-21-issue-8-gate-2-transition.md`。
- 已完成 Gate 2 同步计划：`docs/plans/2026-07-21-issue-9-gate-2-upstream-baseline.md`。
- 上游同步报告：`docs/reports/2026-07-21-upstream-v1.2.41-sync.md`。
- 当前 closeout 计划：`docs/plans/2026-07-21-issue-12-gate-2-upstream-closeout.md`。
- 当前 Session Plan：`docs/plans/sessions/2026-07-21-issue-12-gate-2-upstream-closeout.md`。
- 当前 Runtime Workflow：`docs/workflows/2026-07-21-issue-12-gate-2-upstream-closeout-runtime.md`。
- 当前 closeout 报告：`docs/reports/issue-12-gate-2-upstream-closeout.md`。

## 停止条件

- 上游最新正式 Release 或 `v1.2.41` 标签提交发生变化。
- 需要修改 `upstream/` 或 `source-lock.json`，但没有新的独立 upstream-sync Issue/PR 与项目所有者批准。
- 需要创建 Cargo、实现 UI、创建 Actions、发布资产、进入 Gate 3 或修改 AGOS。
- Fresh 验证失败、Ruleset 变化、Review 对话未闭环或出现未批准的一致性差异。
