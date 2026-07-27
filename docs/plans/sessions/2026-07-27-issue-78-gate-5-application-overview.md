# Issue #78：Gate 5 应用概览 Implementation Session Plan

> **执行要求：** 当前只允许保存本 Session Plan，不允许执行实现步骤。项目所有者批准候选范围后，实施必须按测试先行、根因排错、轻量本地验证和 GitHub-hosted 双平台验证逐项执行；最终 Squash Merge 始终保留单独授权门。

**目标：** 以纯 Rust 实现 Windows/macOS 等价的应用概览只读事实能力，明确区分安装、版本未知、未安装、实时未观察和失败，彻底阻止历史启动记录冒充实时进程状态。

**架构：** `inputcodex-domain` 保存稳定快照；`inputcodex-application` 定义脱敏请求、端口和加载用例；`inputcodex-platform` 复用 Issue `#75` 的安装候选算法，但通过安装专用内部入口隔离 `CODEX_HOME` 与状态目录，并以有界标准库读取版本。Parity 只把批准后的只读子集标记为已实现。

**技术栈：** Rust `1.97.1`、Edition `2024`、标准库、现有 `windows = 0.58.0` 安全 WinRT 投影、现有 Parity YAML/验证器、PowerShell 7、公开仓库标准 GitHub-hosted Windows/macOS Runner。

## Session Contract

```yaml
schema_version: agos.session-plan.v1
architecture_contract_version: agos.brainstorming-gate.v1
task_id: issue-78
work_class: standard
task_summary: 迁移 feature.foundation-platform.application-overview 的纯 Rust 双平台只读事实能力
project_root: .
tracking_issue_ref: https://github.com/nonononull/inputcodex/issues/78
parity_exception_ref: https://github.com/nonononull/inputcodex/issues/77
parity_exception_decision_ref: https://github.com/nonononull/inputcodex/issues/77#issuecomment-5093148603
baseline_ref: a06a97fd59ce125306a13202c8f1a07656c797a0
baseline_tree: b669aa6610e976542a74f404ff4f87b36864816b
branch: codex/issue-78-gate-5-application-overview
implementation_plan_ref: docs/plans/2026-07-27-issue-78-gate-5-application-overview.md
runtime_workflow_ref: docs/workflows/2026-07-27-issue-78-gate-5-application-overview-runtime.md
decision_status: approved
approval_source: direct-user
approved_decision_ref: codex-session-user-message-approved-issue-77-scheme-a-2026-07-27
owner_decision_local_time: 2026-07-27 23:16:01 +08:00
planning_authorization_ref: codex-session-user-message-approved-issue-78-written-design-planning-scope-2026-07-27
planning_authorization_local_time: 2026-07-27 23:34:39 +08:00
current_phase: pr-preparation
planning_scope_count: 3
planning_scope_hash: sha256:3f81a54c18c07b6889ad8219b0c1605e4b989f997117141fc2d4baae46ebbeb3
candidate_scope_count: 29
candidate_scope_hash: sha256:b46a940ff7dbf4bbc9bfdb69d04d755468e12409d9618837d8ff310490eb5ae4
scope_approval_ref: https://github.com/nonononull/inputcodex/issues/78#issuecomment-5093844784
scope_approval_status: approved
implementation_authorization: authorized
allowed_operations: exact-twenty-nine-path-tdd, local-lightweight-verification, git-checkpoints, normal-commit, normal-push, non-draft-pr, review-ci
post_approval_operations: exact-twenty-nine-path-tdd, local-lightweight-verification, git-checkpoints, normal-commit, normal-push, non-draft-pr, review-ci
mutation_intent: approved-application-overview-readonly-implementation
business_mutation_intent: no-product-mutation-until-exact-scope-approval
executor_enforcement: exact-three-planning-paths-now, exact-twenty-nine-paths-after-approval, no-force-push, squash-only, never-delete-main
delivery_contract: agos.issue-pr-merge.v1
review_strategy: final-head changed-surface review; every conversation records root cause, treatment and verification evidence
ci_expectation: standard GitHub-hosted CI plus Windows/macOS compile-test and non-required observation; successful artifact count remains zero
final_merge_authorization: pending-separate-owner-gate
status: local-verification-passed-pr-pending
```

## Global Constraints

- 软件正式名称固定为 `inputcodex`。
- 禁止广告、推广、广告 SDK、付费导流、遥测和远程推荐。
- 禁止 TypeScript、JavaScript 业务代码、WebView 和 UI；本 Issue 不涉及 Gemini UI 工作。
- 领域、应用、平台和 Parity 不得依赖 Iced 类型。
- Windows 与 macOS 使用同一领域快照、应用端口、错误语义和加载状态。
- 禁止文件写入、网络、缓存、后台线程、shell、新依赖家族和 `unsafe`。
- 私人绝对路径只能存在于 `PrivatePath` 内部，自定义 `Debug` 必须脱敏。
- 上游最新正式 Release `v1.2.43 @ 5036ff056b5c629f19356396b17d6eeb70da664c` 只作为只读审计证据。
- 当前不得修改 `upstream/`、`.github/workflows/`、Ruleset、预算、Release 或 AGOS。
- 本机时间只使用 Windows `Get-Date`；不得用会话元数据或网络时间替代。
- 未查明并解决 Review/CI 失败根因前不得合并。

## Brainstorming

```yaml
brainstorming:
  status: approved
  approved_option: A-domain-separated-application-overview
  approved_decision_ref: https://github.com/nonononull/inputcodex/issues/77#issuecomment-5093148603
  problem:
    - 上游 load_overview 聚合安装、快捷方式、历史启动记录、更新占位、设置和诊断路径
    - 历史 latest-status.json 不能证明当前进程仍在运行
    - 损坏历史 JSON 被静默转成 None
    - 完整 PlatformPathsPort 解析无关状态目录，会错误阻断应用安装检测
  selected_design:
    - ApplicationOverview 只返回只读应用事实
    - LaunchHistoryRecord 与 LiveProcessState 明确分离
    - 当前 LiveProcessState 固定为 NotObserved
    - 安装发现使用平台专用内部入口，不解析 CODEX_HOME 或 inputcodex 状态根
    - 版本元数据未知不否定已确认安装
  rejected_options:
    - 完整复制上游 OverviewPayload
    - 用历史启动记录填充实时进程状态
    - 复用完整 PlatformPathsPort 作为概览端口
    - 为兼容版本读取引入 shell、无界扫描或新 XML 依赖
```

## Local Knowledge Lookup

```yaml
local_knowledge_lookup:
  project_docs:
    - AGENTS.md
    - README.md
    - build.md
    - err.md
    - CONTEXT.md
    - docs/plans/PROJECT-MASTER-PLAN.md
    - docs/plans/2026-07-27-issue-75-gate-5-platform-paths.md
    - docs/plans/sessions/2026-07-27-issue-75-gate-5-platform-paths.md
    - docs/workflows/2026-07-27-issue-75-gate-5-platform-paths-runtime.md
  code_evidence:
    - crates/inputcodex-domain/src/platform_paths.rs
    - crates/inputcodex-application/src/lib.rs
    - crates/inputcodex-application/src/platform_paths.rs
    - crates/inputcodex-platform/src/platform_paths.rs
    - crates/inputcodex-platform/src/platform_paths/windows.rs
    - crates/inputcodex-platform/src/platform_paths/macos.rs
  upstream_evidence:
    - upstream/CodexPlusPlus/apps/codex-plus-manager/src-tauri/src/commands.rs:460
    - upstream/CodexPlusPlus/apps/codex-plus-manager/src-tauri/src/commands.rs:4049
    - upstream/CodexPlusPlus/crates/codex-plus-core/src/status.rs:37
    - upstream/CodexPlusPlus/crates/codex-plus-core/src/app_paths.rs:438
  release_audit:
    status: current
    requires_reaudit: false
  graph_tools:
    codegraph_directory: absent
    gitnexus_repositories: []
    action: do-not-initialize; use project-native rg and direct reads
  agos:
    mode: optional-bypassed
    reason: current task uses project-native control plane; no external repository mutation or AGOS optimization is allowed
```

## 方法约束

- 计划遵循 `karpathy-guidelines`：只做最小产品切片，不顺手重构相邻模块。
- 实施时先写 RED 测试，再写最小 GREEN；测试失败先查 `err.md` 并按根因排错。
- 本机只运行 `build.md` 规定的定向 Rust、Parity、格式和项目脚本；完整 Workspace、双平台真实性与 Release 构建交给 GitHub-hosted Runner。
- 每个稳定批次形成 Git checkpoint；未取得候选范围批准前不得形成提交。
- 不启用默认多 agent 写码层；若未来使用 reviewer/verifier，只能处理有界只读审查且不得争夺文件所有权。

## 决策

```yaml
decision:
  id: issue-78-application-overview-readonly-facts
  status: approved-design-scope-pending
  owner: nonononull
  feature_id: feature.foundation-platform.application-overview
  stable_terms:
    application_overview: 一次只读应用事实快照
    launch_history_record: 历史启动记录，不代表实时进程
    live_process_state: 当前只允许 NotObserved
    installation_state: Installed 或 NotInstalled
    installed_version: Known 或带稳定原因的 Unknown
  success_semantics:
    - Ready + Installed + Known
    - Ready + Installed + Unknown
    - Ready + NotInstalled
    - 所有 Ready 均包含 NotObserved、inputcodex 版本和采集时间
  failure_semantics:
    - Unsupported platform -> APPLICATION_OVERVIEW_UNSUPPORTED
    - Invalid explicit path -> EXPLICIT_CODEX_PATH_INVALID
    - Discovery failure -> APPLICATION_OVERVIEW_DISCOVERY_FAILED
    - Invalid system time -> APPLICATION_OVERVIEW_TIME_UNAVAILABLE
    - Invalid build version -> APPLICATION_OVERVIEW_BUILD_VERSION_INVALID
  prohibited_semantics:
    - LoadCompletion::Empty 表示未安装
    - 历史记录填充实时状态
    - 版本未知把 Installed 降级为 NotInstalled 或 Failed
```

## Change Contract

```yaml
change_contract:
  target_contract:
    expected_behavior: Windows 与 macOS 通过同一应用端口返回安装事实、受保护安装引用、版本、inputcodex 构建版本、NotObserved 和采集时间；未安装与版本未知均为 Ready
    evidence_refs:
      - https://github.com/nonononull/inputcodex/issues/77
      - https://github.com/nonononull/inputcodex/issues/78
      - docs/plans/2026-07-27-issue-78-gate-5-application-overview.md
  preserved_invariants:
    - name: Issue #75 平台候选顺序与显式路径安全语义不变
      baseline_ref: a06a97fd59ce125306a13202c8f1a07656c797a0
      regression_ref: platform_paths 现有测试与新增 overview 测试同时通过
    - name: LoadCoordinator 取消和过期结果语义不变
      baseline_ref: crates/inputcodex-application/src/lib.rs
      regression_ref: application overview 加载测试与现有加载状态测试同时通过
    - name: 平台层禁止 unsafe 且非展示层不依赖 Iced
      baseline_ref: workspace lints and repository policy
      regression_ref: targeted clippy and Verify-RepositoryPolicy.ps1
    - name: release_audit 与性能预算观察合同不变
      baseline_ref: a06a97fd59ce125306a13202c8f1a07656c797a0
      regression_ref: Verify-ReleaseAuditGate current and unchanged performance workflow
  adjacent_surfaces:
    - name: 应用生命周期与历史记录
      why_adjacent: 只重新归属上游 status 来源，不读取 latest-status.json 或实现历史记录
    - name: 快捷方式、更新、设置和诊断
      why_adjacent: 上游 load_overview 聚合这些字段，但本切片明确排除
    - name: 平台路径
      why_adjacent: 只提取安装发现专用内部入口，不改变公开 PlatformPathsPort 合同
  stale_verdict_invalidation_refs:
    - origin/main、上游正式 Release、release_audit、候选路径或哈希任一变化时，本计划失效
  sibling_regression_guard:
    status: planned
    evidence_ref: 四 crate 定向测试、Clippy、仓库政策、Release Audit 和 Hosted CI
    closeout_rule: passed-or-blocked-before-done
```

## 当前规划范围

以下 `3` 路径是本轮唯一允许写入集合，规范化哈希为 `sha256:3f81a54c18c07b6889ad8219b0c1605e4b989f997117141fc2d4baae46ebbeb3`：

```text
docs/plans/2026-07-27-issue-78-gate-5-application-overview.md
docs/plans/sessions/2026-07-27-issue-78-gate-5-application-overview.md
docs/workflows/2026-07-27-issue-78-gate-5-application-overview-runtime.md
```

规范化算法固定为 `StringComparer.Ordinal` 升序、UTF-8 无 BOM、LF 分隔、保留末尾 LF。

## 候选实施范围

候选 `29` 路径与正式书面设计完全一致，规范化哈希为 `sha256:b46a940ff7dbf4bbc9bfdb69d04d755468e12409d9618837d8ff310490eb5ae4`：

```text
AGENTS.md
CONTEXT.md
README.md
build.md
crates/inputcodex-application/src/application_overview.rs
crates/inputcodex-application/src/lib.rs
crates/inputcodex-application/tests/application_overview.rs
crates/inputcodex-domain/src/application_overview.rs
crates/inputcodex-domain/src/lib.rs
crates/inputcodex-domain/tests/application_overview.rs
crates/inputcodex-parity/tests/catalog_repository.rs
crates/inputcodex-platform/src/application_overview.rs
crates/inputcodex-platform/src/application_overview/macos.rs
crates/inputcodex-platform/src/application_overview/windows.rs
crates/inputcodex-platform/src/lib.rs
crates/inputcodex-platform/src/platform_paths.rs
crates/inputcodex-platform/src/platform_paths/macos.rs
crates/inputcodex-platform/src/platform_paths/windows.rs
crates/inputcodex-platform/tests/application_overview.rs
docs/plans/2026-07-27-issue-78-gate-5-application-overview.md
docs/plans/PROJECT-MASTER-PLAN.md
docs/plans/sessions/2026-07-27-issue-78-gate-5-application-overview.md
docs/reports/issue-78-gate-5-application-overview.md
docs/workflows/2026-07-27-issue-78-gate-5-application-overview-runtime.md
err.md
parity/README.md
parity/contracts/foundation-platform.yml
parity/features/foundation-platform.yml
parity/features/source-index.yml
```

项目所有者尚未批准该候选集合。实施者必须在获得逐字批准后，先复算哈希并确认 `origin/main`、上游 Release 和 Release Audit 未漂移，才能开始 Task 1。

## File Map

| 路径组 | 职责 |
| --- | --- |
| `crates/inputcodex-domain/**/application_overview.rs` | 版本值、安装状态、实时未观察、采集时间和概览快照 |
| `crates/inputcodex-application/**/application_overview.rs` | 脱敏请求、端口、用例和加载映射 |
| `crates/inputcodex-platform/src/platform_paths*` | 提取安装发现专用 crate 内部入口，保持 Issue #75 行为 |
| `crates/inputcodex-platform/src/application_overview*` | 系统适配器、有界版本读取、构建版本和时间 |
| `crates/inputcodex-parity/tests/catalog_repository.rs` | 固定目录状态、来源归属、副作用和错误合同 |
| `parity/**foundation-platform.yml`、`source-index.yml` | 修正应用概览语义并把历史状态移回生命周期 |
| `AGENTS.md`、`README.md`、`CONTEXT.md`、Master Plan | 合并后稳定项目状态 |
| `build.md`、`err.md`、Runtime Workflow、报告 | 可重复验证、排错与执行证据 |

## Task 1：领域模型 TDD

**Files:**

- Create: `crates/inputcodex-domain/src/application_overview.rs`
- Create: `crates/inputcodex-domain/tests/application_overview.rs`
- Modify: `crates/inputcodex-domain/src/lib.rs`

**Interfaces:**

- Consumes: `CodexInstallation`、`PrivatePath`。
- Produces: `ApplicationVersion`、`ApplicationVersionError`、`InstalledVersionUnknownReason`、`InstalledVersion`、`InstallationState`、`LiveProcessState`、`CollectedAtUnixMs`、`ApplicationOverview`。

- [x] **Step 1：写领域 RED 测试**

测试必须逐项固定：版本首尾空白归一、空值、`129` 字节、控制字符、已知/未知版本、Installed/NotInstalled、NotObserved、采集时间访问器和 `PrivatePath` Debug 脱敏。

- [x] **Step 2：运行 RED**

```powershell
cargo test --locked --offline --ignore-rust-version -p inputcodex-domain --test application_overview
```

预期：因模块或公开类型尚不存在而失败；不得通过修改测试规避 RED。

- [x] **Step 3：实现最小领域合同**

严格按书面设计中的类型、枚举变体、构造器和访问器实现；不增加序列化、显示层文本、平台分支或历史记录类型。

- [x] **Step 4：运行 GREEN 与 crate 回归**

```powershell
cargo test --locked --offline --ignore-rust-version -p inputcodex-domain
cargo clippy --locked --offline --ignore-rust-version -p inputcodex-domain --all-targets -- -D warnings
```

预期：全部通过，且 `#![forbid(unsafe_code)]` 保持。

- [x] **Step 5：形成领域 Git checkpoint**

只在候选范围获批后执行；检查点必须只包含本 Task 三路径，不得提交其他改动。

## Task 2：应用端口与加载语义 TDD

**Files:**

- Create: `crates/inputcodex-application/src/application_overview.rs`
- Create: `crates/inputcodex-application/tests/application_overview.rs`
- Modify: `crates/inputcodex-application/src/lib.rs`

**Interfaces:**

- Consumes: `ApplicationOverview`、`ApplicationError`、`LoadCompletion`、`LoadCoordinator`、`RequestId`。
- Produces: `ApplicationOverviewRequest`、`ApplicationOverviewPort`、`LoadApplicationOverview<P>`。

- [x] **Step 1：写应用 RED 测试**

使用 stub port 固定五类结果：Installed+Known、Installed+Unknown、NotInstalled、端口失败、脱敏请求；额外断言用例从不产生 `LoadCompletion::Empty`，并回放取消与过期结果不覆盖新请求。

- [x] **Step 2：运行 RED**

```powershell
cargo test --locked --offline --ignore-rust-version -p inputcodex-application --test application_overview
```

预期：因请求、端口和用例尚不存在而失败。

- [x] **Step 3：实现最小应用合同**

请求只保存可选显式路径并自定义脱敏 `Debug`；端口返回完整领域快照；用例只把 `Ok` 映射为 `Ready`、`Err` 映射为 `Failed`。

- [x] **Step 4：运行 GREEN 与加载状态回归**

```powershell
cargo test --locked --offline --ignore-rust-version -p inputcodex-application
cargo clippy --locked --offline --ignore-rust-version -p inputcodex-application --all-targets -- -D warnings
```

预期：新增测试与既有加载状态测试全部通过。

- [x] **Step 5：形成应用 Git checkpoint**

只包含本 Task 三路径，并确认没有为概览复制第二套加载状态机。

## Task 3：安装发现专用入口 TDD

**Files:**

- Modify: `crates/inputcodex-platform/src/platform_paths.rs`
- Modify: `crates/inputcodex-platform/src/platform_paths/windows.rs`
- Modify: `crates/inputcodex-platform/src/platform_paths/macos.rs`

**Interfaces:**

- Consumes: `PlatformPathsRequest`、`CodexInstallation`、现有固定候选算法。
- Produces: Windows/macOS crate 内部 `resolve_installation_system(Option<&Path>, &impl PathProbe)`；`PathProbe` 与 `SystemPathProbe` 仅扩大到 `pub(crate)`。

- [x] **Step 1：扩充现有平台路径 RED 测试**

固定两个事实：完整路径解析与安装专用入口选择相同安装；安装专用入口在显式路径场景不读取或验证 `CODEX_HOME`、状态根、设置、历史状态或日志路径。

- [x] **Step 2：运行 RED**

```powershell
cargo test --locked --offline --ignore-rust-version -p inputcodex-platform platform_paths
```

预期：因专用入口尚不存在而失败，现有 Issue `#75` 测试仍保持通过或仅被新增 RED 阻断。

- [x] **Step 3：外科式提取入口**

只移动安装发现编排，不改变候选常量、排序、显式路径优先级、管理器过滤或公开 `SystemPlatformPaths` 结果。完整路径解析调用新入口一次，不得重复扫描。

- [x] **Step 4：回放 Issue #75 平台测试**

```powershell
cargo test --locked --offline --ignore-rust-version -p inputcodex-platform platform_paths
```

预期：Windows/macOS 固定候选、显式路径、安全过滤和 I/O 次数断言全部通过。

- [x] **Step 5：形成共享解析 Git checkpoint**

提交前使用 `git diff` 证明仅有可见性与调用编排变化，没有扩大公开 API 或候选面。

## Task 4：平台概览与有界版本读取 TDD

**Files:**

- Create: `crates/inputcodex-platform/src/application_overview.rs`
- Create: `crates/inputcodex-platform/src/application_overview/windows.rs`
- Create: `crates/inputcodex-platform/src/application_overview/macos.rs`
- Create: `crates/inputcodex-platform/tests/application_overview.rs`
- Modify: `crates/inputcodex-platform/src/lib.rs`

**Interfaces:**

- Consumes: `ApplicationOverviewPort`、`ApplicationOverviewRequest`、领域快照、安装专用入口。
- Produces: `SystemApplicationOverview`、Windows 版本解析器、macOS plist 版本解析器、统一有界文件读取和稳定错误映射。

- [x] **Step 1：写平台 RED 测试**

Windows 固定包目录、数字目录、单个 `version` 文件、缺失、I/O 错误、超限和非法文本；macOS 固定短版本优先、构建版本回退、缺失键、非 UTF-8/binary、超限和非法文本；共同固定安装发现一次、NotObserved、构建版本、时间和私人路径脱敏。

- [x] **Step 2：运行 RED**

```powershell
cargo test --locked --offline --ignore-rust-version -p inputcodex-platform --test application_overview
```

预期：因平台概览模块和适配器尚不存在而失败。

- [x] **Step 3：实现 Windows 最小 GREEN**

只检查固定元数据根、受控包名前缀和一个 `version` 文件；最大读取 `256` 字节，不调用 `canonicalize`、注册表、PATH、shell 或递归扫描。

- [x] **Step 4：实现 macOS 最小 GREEN**

只读取一个 `Contents/Info.plist`，最大 `65536` 字节；只解析 UTF-8 XML 文本，短版本优先、构建版本回退，不调用 `plutil` 或外部 XML 库。

- [x] **Step 5：实现系统适配器与错误边界**

版本问题只进入 `InstalledVersion::Unknown`；不支持平台、显式路径、发现失败、时间异常和构建版本异常使用五个固定错误。任何成功快照都固定 `LiveProcessState::NotObserved`。

- [x] **Step 6：运行平台 GREEN 与 Clippy**

```powershell
cargo test --locked --offline --ignore-rust-version -p inputcodex-platform
cargo clippy --locked --offline --ignore-rust-version -p inputcodex-platform --all-targets -- -D warnings
```

预期：新增概览测试与 Issue `#75` 平台路径回归全部通过。

- [x] **Step 7：形成平台 Git checkpoint**

确认 `Cargo.toml` 和 `Cargo.lock` 无变化，产品代码没有文件写入、网络、线程、shell 或 `unsafe`。

## Task 5：Parity 语义修正 TDD

**Files:**

- Modify: `parity/features/foundation-platform.yml`
- Modify: `parity/contracts/foundation-platform.yml`
- Modify: `parity/features/source-index.yml`
- Modify: `parity/README.md`
- Modify: `crates/inputcodex-parity/tests/catalog_repository.rs`

**Interfaces:**

- Consumes: Issue `#77` 决策、Issue `#78` 书面设计、上游 `load_overview`、`StatusStore::load_latest` 与 `codex_app_version` 证据。
- Produces: `application-overview=implemented`、修正后的双平台语义、历史状态来源重新归属、只读副作用和稳定错误合同。

- [x] **Step 1：写 Parity RED**

断言应用概览包含 Issue `#77/#78`、三类 Ready 成功、NotObserved、五个稳定错误、文件系统/进程读取；断言不包含历史状态、快捷方式、更新、设置、诊断、写入、网络、广告或远程推荐。断言 `core-module:status` 归属应用生命周期且副作用为文件读写。

- [x] **Step 2：运行 RED**

```powershell
cargo test --locked --offline --ignore-rust-version -p inputcodex-parity --test catalog_repository gate5_application_overview
```

预期：现有目录仍为 `unassessed`，测试失败。

- [x] **Step 3：更新最小目录合同**

只修改应用概览自身条目、合同、`core-module:status` 归属和对应说明；不得改变其他 feature 状态、总预算、Release Audit 或上游快照。

- [x] **Step 4：运行 Parity GREEN 与仓库级回归**

```powershell
cargo test --locked --offline --ignore-rust-version -p inputcodex-parity --test catalog_repository
cargo clippy --locked --offline --ignore-rust-version -p inputcodex-parity --all-targets -- -D warnings
```

预期：仓库级目录验证全部通过，来源覆盖缺口保持 `0`。

- [x] **Step 5：形成 Parity Git checkpoint**

确认只实现一个产品 feature，`feature.foundation-platform.application-lifecycle` 仍为未实现状态。

## Task 6：项目控制面与报告收口

**Files:**

- Modify: `AGENTS.md`
- Modify: `README.md`
- Modify: `CONTEXT.md`
- Modify: `build.md`
- Modify: `err.md`
- Modify: `docs/plans/PROJECT-MASTER-PLAN.md`
- Modify: `docs/plans/2026-07-27-issue-78-gate-5-application-overview.md`
- Modify: `docs/plans/sessions/2026-07-27-issue-78-gate-5-application-overview.md`
- Modify: `docs/workflows/2026-07-27-issue-78-gate-5-application-overview-runtime.md`
- Create: `docs/reports/issue-78-gate-5-application-overview.md`

**Interfaces:**

- Consumes: 最终实现 Head、定向验证结果、Parity 状态和所有根因记录。
- Produces: 可重复构建命令、排错条目、Master Plan 状态和 Issue `#78` 实施报告。

- [x] **Step 1：更新构建说明**

在 `build.md` 增加 Issue `#78` 定向测试、Clippy、格式、CI 合同、Release Audit、Repository Policy、范围、隐私和空白验证，不得要求本机完整 Workspace 或发布构建。

- [x] **Step 2：更新排错说明**

只记录本 Issue 新出现且已确定根因的问题；重复的 apply-patch、PowerShell 参数或 CRLF 问题引用既有条目，不重复制造结论。

- [x] **Step 3：回写项目状态**

README、CONTEXT、AGENTS 与 Master Plan 只写本 feature 的真实已验证状态；PR、CI、Review 和合并证据未发生前不得预写成功。

- [x] **Step 4：形成实施报告**

报告固定基线、候选哈希、实际差异、TDD RED/GREEN、性能上限、隐私扫描、未包含能力和待办门禁。

- [x] **Step 5：形成控制面 Git checkpoint**

确认报告与控制面没有递归创建 Issue `#75` Closeout，也没有改变外部 AGOS。

## Task 7：最终本地轻量验证

**Files:**

- Verify only: 候选 `29` 路径。

- [x] **Step 1：运行四 crate 定向测试**

```powershell
cargo test --locked --offline --ignore-rust-version -p inputcodex-domain -p inputcodex-application -p inputcodex-platform -p inputcodex-parity
```

预期：全部通过。

- [x] **Step 2：运行四 crate Clippy 与格式**

```powershell
cargo clippy --locked --offline --ignore-rust-version -p inputcodex-domain -p inputcodex-application -p inputcodex-platform -p inputcodex-parity --all-targets -- -D warnings
cargo fmt --all -- --check
```

预期：零 warning，格式检查退出码为 `0`。

- [x] **Step 3：运行项目脚本**

```powershell
pwsh -NoProfile -File scripts/ci/Test-CiScripts.ps1
pwsh -NoProfile -File scripts/ci/Verify-ReleaseAuditGate.ps1 -RepositoryRoot .
pwsh -NoProfile -File scripts/ci/Verify-RepositoryPolicy.ps1 -RepositoryRoot .
```

预期：CI 合同通过、`release_audit=current`、仓库政策违规数为 `0`。

- [x] **Step 4：复算范围与实际差异**

按 Runtime Workflow 的 PowerShell 区块复算 `29` 路径和 `sha256:b46a940ff7dbf4bbc9bfdb69d04d755468e12409d9618837d8ff310490eb5ae4`，并确认所有变更都在批准集合内。实现允许只修改批准集合的必要子集；任何集合外路径都必须失败。

- [x] **Step 5：运行隐私与禁止能力扫描**

扫描本机用户名、绝对用户目录、`latest-status.json` 运行依赖、网络、写入、shell、`unsafe`、Iced、广告、远程推荐和遥测；测试夹具中的语义断言必须与产品依赖区分。

- [x] **Step 6：运行空白与 Git 快照检查**

```powershell
git diff --check
git status --short --branch
```

预期：无空白错误，工作区只包含候选路径的预期差异。

## Task 8：提交、PR、Review 与 Hosted CI

候选 `29` 路径及远端写入操作已经通过 Issue `#78` 批准评论授权；本 Task 可以执行，但最终 Squash Merge 仍由独立授权门控制。

- [x] **Step 1：创建最终本地 Git checkpoint**

记录本机 `Get-Date`、HEAD、tree、差异路径、范围哈希和本地验证摘要；禁止覆写 Git 时间。

- [ ] **Step 2：普通提交与普通推送**

不得 force push；提交必须关联 Issue `#78`，分支固定为 `codex/issue-78-gate-5-application-overview`。

- [ ] **Step 3：创建非 Draft PR**

PR 必须说明 Issue `#77` 语义例外、三个 Ready 成功面、NotObserved、版本未知边界、I/O 上限和明确排除项。

- [ ] **Step 4：完成 Final Head Review**

每条 Review 对话必须记录根因、处理方式和验证证据；反馈不成立时必须提供可复核证据并取得确认。

- [ ] **Step 5：等待 GitHub-hosted CI**

Windows/macOS 编译测试、标准 CI 与非 required Performance observation 必须成功；成功 Artifact 数为 `0`。外部服务事故必须建立独立事故证据，不得伪造绿色。

- [ ] **Step 6：进入独立 Squash Merge 授权门**

只有 Final Head、Review、CI、全部对话和所有者决策证据齐全后，才向项目所有者申请 Squash Merge；不得自行合并。

## Self-Review

### 规范覆盖

- Issue `#77` 的三条核心决策均有领域类型、应用结果、平台边界和 Parity 断言对应。
- Windows/macOS 版本来源、错误码、Unknown/Failed 边界和 I/O 上限均已固定。
- 快捷方式、更新、设置、诊断、生命周期和 Watcher 均明确排除。
- 候选 `29` 路径包含实现、测试、Parity、构建、排错、计划、报告和项目状态所需最小表面；不包含 Cargo 或 Workflow。

### 占位符检查

本计划没有未决占位标记、“稍后补充”或未定义接口；所有未来状态使用明确控制门和可验证命令表达。

### 类型一致性

领域、应用和平台任务统一使用：

- `ApplicationOverview`；
- `ApplicationVersion`；
- `InstallationState::{Installed, NotInstalled}`；
- `InstalledVersion::{Known, Unknown}`；
- `LiveProcessState::NotObserved`；
- `CollectedAtUnixMs`；
- `ApplicationOverviewRequest`；
- `ApplicationOverviewPort`；
- `LoadApplicationOverview<P>`；
- `SystemApplicationOverview`。

### 当前执行结论

本 Session Plan 已进入获批实施阶段。Domain、Application、Platform、Parity 与控制面写入已完成；当前只允许执行最终本地验证、Git checkpoint、普通提交/推送、非 Draft PR 与 Review/CI。最终 Squash Merge 未获授权，不得自行合并。
