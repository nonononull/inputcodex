# Session Plan：Issue #17 Gate 3 纯 Rust Workspace 骨架规划

schema_version: inputcodex.session-plan.v1
task_id: 2026-07-22-issue-17-gate-3-rust-workspace-plan
work_class: standard
task_status: local-verified-pr-pending
task_summary: 仅冻结 Gate 3 Rust Workspace、Iced 隔离、双平台抽象、性能诊断和首版三平台 CI 的后续实现合同，不创建产品源码。
project_root: C:/Users/dashuai/Documents/inputcodex
tracking_issue_ref: https://github.com/nonononull/inputcodex/issues/17
branch_ref: codex/issue-17-gate-3-rust-workspace-plan
baseline_ref: 113476fb96623452f9a69526edabc73a57d812a1
decision_status: approved-planning-only
approved_decision_ref: user-message:approve-gate-3-planning-2026-07-22
session_plan_ref: docs/plans/sessions/2026-07-22-issue-17-gate-3-rust-workspace-plan.md
implementation_plan_ref: docs/plans/2026-07-22-issue-17-gate-3-rust-workspace-plan.md
runtime_workflow_ref: docs/workflows/2026-07-22-issue-17-gate-3-rust-workspace-plan-runtime.md
report_ref: docs/reports/issue-17-gate-3-rust-workspace-plan.md
scope_hash: sha256:0c4fc5017aed0b5883b5cb597b2afc2680646479de32916cc4e720aff67dfd05
mutation_intent: 只更新 11 个批准的治理文档路径并补齐 Gate 2 最终证据；不创建 Cargo、Rust、Iced、产品 CI、发布或功能实现。
executor_enforcement: 提交、PR 与 Review 前逐项核对允许路径、禁止表面、Fresh GitHub 基线和仓库中不存在产品 Cargo/Rust/Iced 文件。
delivery_contract: inputcodex.issue-pr-merge.v1
external_agos_policy: available-optional-unavailable-bypass-no-optimization
external_agos_execution: needs-input-unregistered-recorded-and-bypassed

## 一、批准决策

- Decision：Gate 2 已完成，允许进入 Gate 3 的规划阶段。
- Decision：本 Session 只覆盖发现、方案、Issue、Session Plan、Runtime Workflow、规划报告与规划 PR。
- Decision：不得创建 `Cargo.toml`、`Cargo.lock`、`rust-toolchain.toml`、`.rs`、Iced UI 或新的产品 Workflow。
- Decision：规划采用“合同先行，规划与实现拆分”；规划 PR 合并后，源码实现仍需新的 Issue 和项目所有者明确批准。
- Decision：性能优先、功能一致、双平台一致、无广告、纯 Rust/Iced、Iced 仅展示层和外部 AGOS 可绕过约束保持不变。
- Reason：直接搭建 Workspace 会让未经评审的 crate 边界、Iced feature、平台接口和 CI 失败语义过早成为事实，重现上游卡顿与职责混乱风险。

## 二、Fresh 启动基线

- `HEAD` 与 `origin/main` 均为 `113476fb96623452f9a69526edabc73a57d812a1`，启动工作树干净。
- PR `#15` 已于 `2026-07-22T04:13:54Z` Squash Merge，合并提交只有一个父提交，GitHub 签名验证为 `valid`。
- Issue `#14` 已按 `COMPLETED` 关闭；远端功能分支已删除。
- Actions `29890586102` 与 `29890641799` 均成功；第一次创建、第二次复用唯一状态 Issue `#16`，告警数量为 `0`。
- 上游最新正式 Release 仍为 `v1.2.41`，标签提交为 `3dafffcafb2566a1e8bce4b35671656d6adb3eda`；上游 `main` 为 `6fa0a57decbb3382771a981247e6922799e97f5d`。
- Ruleset `19395456` 为 active、无 bypass、required approvals `0`、必须解决 Review 对话并只允许 Squash Merge。
- 仓库无 `.codegraph/`，未擅自初始化 CodeGraph。

## 三、Brainstorming 记录

```yaml
brainstorming:
  status: approved
  owner_intent:
    - 性能优先，其次保持上游正式 Release 的有效功能一致
    - 使用纯 Rust 与 Iced，禁止 TypeScript、JavaScript 业务代码和 WebView
    - 每一步使用 Issue、PR、Review、CI 与 Squash Merge
    - 无效功能、有害副作用或错误语义必须先沟通并建立一致性例外 Issue
  approaches:
    - id: direct-workspace
      decision: rejected
      reason: 未冻结边界就创建源码，返工和架构漂移风险最高
    - id: contract-first-split-delivery
      decision: selected
      reason: 规划与实现分离，Workspace 与首版三平台 CI 在后续同一实现 PR 中验证
    - id: adr-only
      decision: rejected
      reason: 缺少任务级允许路径、Runtime Workflow 和交付证据
  approved_decision_ref: user-message:approve-gate-3-planning-2026-07-22
```

## 四、允许操作

```yaml
allowed_operations:
  - 创建和维护 Issue #17 的计划、Session Plan、Runtime Workflow 与初始报告
  - 更新 README、Master Plan、总架构方案、Rust CI 实施计划、build.md 与 err.md
  - 回写 Issue #14 / PR #15 已完成的动态证据
  - 运行只读本地知识查询、Fresh Git/GitHub 核对和文档验证
  - 创建普通提交、普通 push、关联规划 PR 和 Review/CI 证据
  - 在获得新的明确授权后以 Squash Merge 合并规划 PR
```

## 五、允许路径

```yaml
allowed_paths:
  - README.md
  - build.md
  - err.md
  - docs/plans/PROJECT-MASTER-PLAN.md
  - docs/plans/2026-07-21-architecture-governance.md
  - docs/plans/2026-07-21-rust-ci-offload-implementation-plan.md
  - docs/plans/2026-07-22-issue-17-gate-3-rust-workspace-plan.md
  - docs/plans/sessions/2026-07-22-issue-17-gate-3-rust-workspace-plan.md
  - docs/workflows/2026-07-22-issue-17-gate-3-rust-workspace-plan-runtime.md
  - docs/reports/issue-17-gate-3-rust-workspace-plan.md
  - docs/reports/issue-14-gate-2-upstream-watch.md
allowed_path_count: 11
scope_hash: sha256:0c4fc5017aed0b5883b5cb597b2afc2680646479de32916cc4e720aff67dfd05
```

## 六、禁止操作

- 不创建或修改产品 Cargo、Rust、Iced、UI、功能、数据库、网络、平台实现或测试代码。
- 不创建 `.github/workflows/ci.yml`、Release Workflow、required check、Runner、Cache 或 Artifact 配置。
- 不修改 `upstream/`、`source-lock.json`、机器维护 Issue `#16`、Ruleset、仓库合并开关或 `main` 历史。
- 不导入半成品参考仓库，不让上游 Tauri/React、注入脚本或远程推荐列表进入运行面。
- 不修改、修复、登记或优化 AGOS 的 Registry、脚本、规则、Workflow 或 Vault。
- 不把规划 PR 的批准解释为后续实现或合并授权。

## 七、本地知识与外部辅助

```yaml
local_knowledge_lookup:
  query: inputcodex 纯 Rust Iced Workspace crate 分层 平台抽象 性能 可诊断 三平台 CI
  tool: gbrain 0.41.14.0
  result: no-results
  conclusion: 使用项目现有架构、CI 计划、GitHub Fresh 证据和项目所有者决策作为权威输入
external_agos:
  command_status: report-only-completed
  result: needs-input
  registration_status: unregistered
  project_git_foundation: ready
  project_entry_docs: ready
  action: 按项目约束记录并绕过，继续项目原生流程
  forbidden_followup: 不在 inputcodex 中修复或优化 AGOS
```

## 八、版本与许可证候选证据

- 规划时 Rust 官方 stable 为 `1.97.1 (8bab26f4f 2026-07-14)`；后续实现候选固定为 `1.97.1`，创建工具链文件前必须 Fresh 复核。
- Iced 最新正式 Release 与 crates.io 默认稳定版本为 `0.14.0`；声明 MSRV `1.88`、许可证 MIT、checksum `000e01026c93ba643f8357a3db3ada0e6555265a377f6f9291c472f6dd701fb3`、未撤回。
- `inputcodex` 当前仓库许可证为 GNU AGPLv3；后续 Cargo package 元数据的 SPDX 表达必须在实现 Issue 中结合项目所有者意图和仓库声明再次确认，不在本次文档任务中擅自决定 `only` 或 `or-later`。
- 所有第三方依赖必须在后续实现 Issue 中核对精确版本、许可证、来源、维护状态和最小 feature 集。

## 九、执行批次

1. Fresh 核对 Git、GitHub、Ruleset、上游 Release、Issue `#16` 与 Gate 2 closeout。
2. 创建 Issue `#17` 和独立规划分支。
3. 落盘 Gate 3 主计划、Session Plan、Runtime Workflow 和初始报告。
4. 更新项目入口、架构/CI 真源、构建验证和 Gate 2 最终报告。
5. 运行本地文档合同、允许路径、禁止表面和 diff 验证。
6. 精确暂存、提交、普通 push，并创建关联 Issue `#17` 的非 Draft PR。
7. 完成 Review 根因闭环和现有 CI；等待项目所有者新的 Squash Merge 授权。

## 十、验证合同

```yaml
project_verification_commands:
  - git diff --check
  - git diff --cached --check
  - 校验 main...HEAD 和工作树路径均属于 11 条允许路径
  - 校验仓库不存在产品 Cargo.toml、Cargo.lock、rust-toolchain.toml 或 .rs 文件
  - 校验除既有 upstream-watch.yml 外没有新增 Workflow
  - 校验 Master Plan、README、计划、Session、Runtime、报告互相引用
  - Fresh 核对 Issue #17、Ruleset 19395456、上游 v1.2.41 与 Issue #16
ci_expectation:
  - 规划 PR 只运行现有 Upstream Watch validate
  - watch Job 在 pull_request 事件中必须 skipped
  - 本 PR 不创建 CI Workflow，也不把不存在的检查加入 Ruleset
```

## 十一、停止条件

- 上游最新正式 Release、冻结 tag 提交、Ruleset 或机器状态发生物质变化。
- 变更路径超出允许集合，或出现 Cargo、Rust、Iced、产品 Workflow、功能实现或 UI。
- Rust/Iced 版本或许可证证据无法 Fresh 复核，却试图用浮动版本继续。
- 需要修改 AGOS、`upstream/`、Issue `#16`、Ruleset、发布资产或 `main` 历史。
- Review 对话未完成根因闭环、CI 失败、路径验证失败或缺少项目所有者合并授权。

## 十二、交付证据

tracking_issue_ref: https://github.com/nonononull/inputcodex/issues/17
review_ref: pending
pr_ref: pending
ci_ref: pending
merge_ref: pending
owner_merge_authorization_ref: pending-new-owner-authorization-required
