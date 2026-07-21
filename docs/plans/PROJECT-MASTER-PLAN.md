# inputcodex 项目总计划

schema_version: inputcodex.master-plan.v1
active_task: 2026-07-21-issue-4-gate-1-closeout
active_gate: Gate 1：合并证据 closeout
last_verified_gate: Gate 1 子里程碑：Issue #2 / PR #3 已 Squash Merge
next_legal_gate: 完成 Issue #4 closeout PR 的项目所有者 Review 与 Squash Merge；随后以独立 Issue/PR 补齐 GitHub 模板与标签，Gate 1 完成前不得进入 Gate 2。
tracking_issue_ref: https://github.com/nonononull/inputcodex/issues/4
active_branch_ref: codex/issue-4-gate-1-closeout
active_plan_ref: docs/plans/2026-07-21-issue-4-gate-1-closeout.md
active_session_plan_ref: docs/plans/sessions/2026-07-21-issue-4-gate-1-closeout.md
active_runtime_workflow_ref: docs/workflows/2026-07-21-issue-4-gate-1-closeout-runtime.md
active_pr_ref: pending:issue-4-closeout-pr
closed_delivery_ref: https://github.com/nonononull/inputcodex/pull/3
closeout_report_ref: docs/reports/issue-2-architecture-governance-closeout.md
active_ruleset_ref: https://github.com/nonononull/inputcodex/rules/19395456
active_ci_strategy_ref: docs/plans/2026-07-21-rust-ci-offload-strategy.md
active_ci_implementation_plan_ref: docs/plans/2026-07-21-rust-ci-offload-implementation-plan.md
decision_status: approved

## 当前状态

- 当前只执行文档、治理冻结与 `main` Ruleset 落地，不导入上游源码，不创建 Rust/Iced 工程，不实现 GitHub Actions，不发布资产。
- `BigPizzaV3/CodexPlusPlus` 最新正式 Release 是功能真源；当前冻结基线为 `v1.2.41`，提交 `3dafffcafb2566a1e8bce4b35671656d6adb3eda`。
- `zsr131550/CodexPlusPlus` 仅作半成品参考，不作为代码底座或功能真源。
- Issue `#2` / PR `#3` 交付链已完成：PR 于 `2026-07-21T13:15:51Z` Squash Merge，Issue 于 `2026-07-21T13:15:52Z` 关闭，旧功能分支已删除。
- PR `#3` 合并提交为 `0e11375997ff10fdc0c233b31c8468af2d9a4f44`；只有一个父提交，且 merge tree 与最终 PR Head tree 完全一致。
- 合并时 Review 对话总数与未解决数均为 `0`；Checks 数量为 `0`，语义是当前未配置 Actions/required checks，不是伪造的 CI 通过。
- GitHub Ruleset `main-protection`（ID `19395456`）已激活且只命中 `main`；禁止删除、禁止 Force Push、要求 PR、要求解决 Review 对话并只允许 Squash Merge，当前单人阶段 required approvals 为 `0`。
- 已批准 Rust CI 云端卸载策略与实施计划：本地只做轻量定向验证，全量 Workspace、Windows/macOS 和发布构建交给标准 GitHub-hosted runners；未来按上游监控、Workspace+三平台 CI、Cache 调优、Ruleset 升级四个独立 Issue/PR 推进，当前只落盘计划，不创建 Workflow。
- 当前 Issue `#4` 只负责通过独立分支和 PR 回写上述 closeout 证据与最新 Master Plan；不进入源码、Actions、Ruleset 或发布范围。

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
- `main` 永久禁止 `--force` 和 `--force-with-lease`；错误历史与紧急修复只能通过 `revert` 和关联 Issue/PR 处理。
- `main` 永久禁止删除；项目所有者与管理员无例外，误删后只能从最后一个权威提交恢复并建立事故 Issue。
- 所有 Review 对话必须在确定根因、完成处理并回写验证证据后才能解决和合并；不成立的反馈也必须有可复核证据与 reviewer 或所有者确认。
- 单人维护阶段 required approvals 为 `0` 且必须保留所有者决策证据；第二名具备合并权限的人类维护者加入后，在下一次合并前提升为 `1`。
- Rust 全量编译与双平台验证默认在标准 GitHub-hosted runners 完成；禁止默认 Larger Runner、本机 self-hosted runner 和无边界 Cache/Artifact。

## 阶段索引

### Gate 0：仓库准备（已验证）

- 已建立本地与 GitHub 公开仓库、GNU AGPLv3、根文档、Issue #1 和筹备 closeout。
- 历史证据：`docs/plans/2026-07-21-bootstrap.md`、`docs/reports/2026-07-21-bootstrap-closeout.md`。

### Gate 1：方案与治理冻结（当前）

- [x] 用户逐项批准纯 Rust/Iced、功能一致、完整上游快照、每 6 小时监控、自主版本和 Issue/PR 治理。
- [x] 起草单一真源、项目语境、两份 ADR、Major Session Plan 和 Runtime Workflow。
- [x] 完成 Fresh 文档、Git、GitHub 元数据和快照治理验证。
- [x] 提交并推送当前分支，创建包含 `Closes #2` 的 PR `#3`。
- [x] 创建并验证仅作用于 `main` 的 GitHub Ruleset；证据见 `docs/reports/2026-07-21-main-protection-rollout.md`。
- [x] 批准并落盘 Rust CI 云端编译卸载策略；当前不创建 Workflow。
- [x] 批准并落盘 Rust CI 云端卸载实施计划；Gate 2/3、Cache 调优和 required check 均保持锁定并要求独立 Issue/PR。
- [x] 项目所有者完成 Review 并以 Squash Merge 合并 PR `#3`；Issue `#2` 已关闭，旧分支已删除。
- [ ] 通过 Issue `#4` 的独立 PR 合并 closeout 报告和最新控制面；未经再次授权不得自动合并。
- [ ] 通过后续独立 Issue/PR 建立 Issue/PR 模板与标签。

### Gate 2：导入上游基线（锁定）

- 通过独立 `upstream-sync` Issue 和 PR 导入 `v1.2.41` 完整快照。
- 创建 `source-lock.json`、同步报告和快照纯净性校验。
- 按 `docs/plans/2026-07-21-rust-ci-offload-implementation-plan.md` 使用标准 `ubuntu-latest` Runner 建立每 6 小时的上游监控工作流；只管理 Issue，不编译 Rust，不自动同步、实现或合并。

### Gate 3：纯 Rust 工作区骨架（锁定）

- 建立分层 Cargo Workspace、Iced 最小双平台窗口和依赖方向测试。
- 按 `docs/plans/2026-07-21-rust-ci-offload-strategy.md` 与 `docs/plans/2026-07-21-rust-ci-offload-implementation-plan.md` 在同一 Gate 3 Issue/PR 建立 Workspace、标准 Linux/Windows/macOS CI、格式、测试、依赖、许可证和更新源归属检查。
- 不迁移业务功能，不创建临时 UI 事实标准。

### Gate 4：功能目录与性能基线（锁定）

- 从上游快照生成首版功能矩阵、合同和脱敏夹具。
- 测量上游、半成品与新骨架的启动、交互、加载、内存、CPU 和后台行为。
- 通过独立 Issue/PR 批准可复现的性能预算和回归门槛。

### Gate 5：分域迁移（锁定）

- 按基础能力、供应商与网络、会话与数据、插件与脚本、远程集成与安装五个域分批迁移。
- 每个可独立验收功能使用独立 Issue 和 PR；上游同步 PR 与功能迁移 PR 永远分离。

### Gate 6：首个正式版本（锁定）

- 功能矩阵、双平台、性能预算、差异批准、签名、安装、升级、回滚和自主更新源全部通过。
- 首个目标版本为 `v1.2.41-inputcodex.1`；如基线在 Gate 2 前变化，必须新 Issue 重新决策。

## 当前验证入口

- 构建与文档验证：`build.md`。
- 排错与已知限制：`err.md`。
- 单一架构真源：`docs/plans/2026-07-21-architecture-governance.md`。
- Issue `#2` closeout 报告：`docs/reports/issue-2-architecture-governance-closeout.md`。
- 当前任务计划：`docs/plans/2026-07-21-issue-4-gate-1-closeout.md`。
- 当前 Session Plan：`docs/plans/sessions/2026-07-21-issue-4-gate-1-closeout.md`。
- 当前运行图：`docs/workflows/2026-07-21-issue-4-gate-1-closeout-runtime.md`。

## 停止条件

- 用户改变已批准硬约束或当前 docs-only 范围。
- 上游最新正式 Release 或 `v1.2.41` 标签提交发生变化。
- 需要导入源码、实现 UI、创建 Actions、发布资产或跨仓修改 AI Growth OS 控制面。
- Fresh 验证失败、分支不正确、PR 无法关联 Issue #4、GitHub closeout 事实发生变化，或出现未批准的一致性差异。
