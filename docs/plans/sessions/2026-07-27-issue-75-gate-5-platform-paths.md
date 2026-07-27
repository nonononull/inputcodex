# Issue #75：Gate 5 平台路径解析 Implementation Session Plan

> **执行要求：** 实施时必须使用 `superpowers:test-driven-development`、`superpowers:systematic-debugging` 与 `superpowers:verification-before-completion`，按本文件逐项执行；除非项目所有者另行明确要求，不启用 subagent。步骤使用复选框记录，最终 Squash Merge 始终保留单独授权门。

**目标：** 以纯 Rust 分层架构实现 Windows/macOS 等价的平台路径解析能力，并以受保护路径值、稳定错误码和可重复测试阻止相对目录回退、私人路径泄露及错误空成功。

**架构：** `inputcodex-domain` 只保存不泄露绝对路径的值对象与快照；`inputcodex-application` 定义请求、端口和同步用例；`inputcodex-platform` 读取固定环境键与固定候选位置，通过无 I/O 纯解析函数生成结果。Windows 只使用 `windows 0.58.0` 安全 WinRT 投影，macOS 只使用标准库固定路径探测，Iced、基础设施、网络、写入和缓存均不参与。

**技术栈：** Rust `1.97.1`、Edition `2024`、标准库、`windows = 0.58.0` 安全 WinRT、现有 Parity YAML/验证器、PowerShell 7、GitHub-hosted Windows/macOS CI。

## Session Contract

```yaml
schema_version: agos.session-plan.v1
architecture_contract_version: agos.brainstorming-gate.v1
task_id: issue-75
work_class: standard
task_summary: 迁移 feature.foundation-platform.platform-paths 的纯 Rust 双平台路径解析能力
project_root: C:/Users/dashuai/Documents/inputcodex
tracking_issue_ref: https://github.com/nonononull/inputcodex/issues/75
parity_exception_ref: https://github.com/nonononull/inputcodex/issues/74
baseline_ref: fc1683aabda4afb27ca333387ec954b6a405d2df
baseline_tree: d17a038fcb4fc986565f121283481eb38cdfbc33
branch: codex/issue-75-gate-5-platform-paths
implementation_plan_ref: docs/plans/2026-07-27-issue-75-gate-5-platform-paths.md
runtime_workflow_ref: docs/workflows/2026-07-27-issue-75-gate-5-platform-paths-runtime.md
decision_status: approved
approval_source: direct-user
approved_decision_ref: session-plan:issue-75#decision
owner_decision_ref: codex-session-user-message-approved-written-spec-2026-07-27
written_spec_status: approved
written_spec_approval_local_time: 2026-07-27 17:39:03 +08:00
scope_approval_status: approved
scope_hash: sha256:ae5e0f5143355feee9b280da7c44fdd5cdf759ec2ae71fc69167040bf302cb37
scope_approval_ref: https://github.com/nonononull/inputcodex/issues/75#issuecomment-5092021020
implementation_authorization_local_time: 2026-07-27 21:36:01 +08:00
allowed_operations: exact-thirty-path-tdd, local-lightweight-verification, git-checkpoints, normal-commit, normal-push, non-draft-pr, review-ci
post_approval_operations: exact-thirty-path-tdd, local-lightweight-verification, git-checkpoints, normal-commit, normal-push, non-draft-pr, review-ci
mutation_intent: source-and-control-plane
business_mutation_intent: source-authorized-within-exact-scope
executor_enforcement: exact-thirty-paths, no-force-push, squash-only, never-delete-main
delivery_contract: agos.issue-pr-merge.v1
review_strategy: final-head changed-surface review; every conversation records root cause, treatment and verification evidence
ci_expectation: standard GitHub-hosted CI plus Windows/macOS compile-test and non-required Performance Baseline observation
merge_policy: squash-only, no-force-push, never-delete-main
final_merge_authorization: pending-separate-owner-authorization
```

## Global Constraints

- 软件名称固定为 `inputcodex`；禁止广告、推广、遥测、TypeScript、JavaScript 业务代码、WebView 与上游运行面复制。
- Windows 与 macOS 从首版起保持功能语义一致；Linux 只返回 `PLATFORM_PATHS_UNSUPPORTED`，不伪装成功。
- Iced 只能存在于展示层；本任务不修改 `inputcodex-presentation`、桌面应用或任何 UI。
- 私人绝对路径不得实现 `Display`、Serde 序列化或自动派生 `Debug`；错误、日志、CI、Review 和报告不得包含真实绝对路径。
- `CODEX_HOME` 非空时必须是已存在绝对目录；空白视为未配置；默认路径只能是绝对用户目录下 `.codex`。
- Windows 状态根固定为 `%LOCALAPPDATA%\inputcodex`；macOS 固定为 `$HOME/Library/Application Support/inputcodex`。
- 未安装 Codex 是 `LoadCompletion::Ready` 且 `codex_installation=None`；配置或系统读取错误必须是 `Failed`，不得使用 `Empty` 混淆语义。
- Windows 只新增已在 `Cargo.lock` 中的 `windows = 0.58.0` 直接依赖，特性固定为 `ApplicationModel`、`Foundation_Collections`、`Management_Deployment`、`Storage`；禁止 `unsafe` 和直接 Win32 FFI。
- 本地只运行相关 crate 与政策验证；Iced、全 Workspace、macOS 真实编译和全量 Release 验证交给 GitHub-hosted runners。
- 当前尚未获得实施授权；本文件与 Runtime Workflow 推送后必须停在所有者批准门。

## Brainstorming

```yaml
brainstorming:
  superpowers_skill: superpowers:brainstorming
  user_decision: 项目所有者批准方案 B 平台路径解析、拒绝静默回退、完整设计与书面规范；实施范围仍等待单独批准
  selected_business_path: inputcodex.gate5.foundation-platform.platform-paths
  selected_option: 方案 B，平台路径解析基础依赖
  owner_decisions:
    - 批准平台路径解析作为首个 Gate 5 产品功能迁移
    - 批准拒绝相对目录和无效显式路径的静默回退
    - 批准完整分层设计和书面规范
  rejected_behaviors:
    - 上游无效 CODEX_HOME 静默回退
    - 用户目录缺失时使用相对 .codex
    - 状态目录使用相对 .codex-session-delete
    - 将 Codex++、CodexPlusPlus、inputcodex 或管理器识别为 Codex 应用
  verification_commands:
    - cargo test --locked --offline --ignore-rust-version -p inputcodex-domain -p inputcodex-application -p inputcodex-platform -p inputcodex-parity
    - pwsh -NoProfile -File scripts/ci/Verify-ReleaseAuditGate.ps1 -RepositoryRoot .
    - pwsh -NoProfile -File scripts/ci/Verify-RepositoryPolicy.ps1 -RepositoryRoot .
    - git diff --check
  implementation_authorization: approved
```

## Local Knowledge Lookup

```yaml
local_knowledge_lookup:
  command_status: local_knowledge_lookup command unavailable
  gbrain_queries:
    - 查询平台路径解析、CODEX_HOME、Windows Package Family、macOS Applications、路径隐私与 Gate 5 前置；命令不可用后改用项目 Issue、计划、上游缓存和 Git 历史
  gitnexus_repositories: []
  codegraph_status: not-initialized
  graph_policy: 不擅自初始化 CodeGraph 或 GitNexus
  agos_default_entry: needs-input/unregistered; ReportOnly 后按项目规则绕过
  external_gap_policy: 只记录，不修改、登记、修复或优化 AGOS
  vault_refs:
    - D:/Android_source/ai-growth-os/components/vault/07-Workflows/Core/AI-Growth-OS-Brainstorming-Gate-And-Session-Plan.md
    - D:/Android_source/ai-growth-os/components/vault/08-Skills/AI-Growth-OS.md
  rules_refs:
    - D:/Android_source/ai-growth-os/components/rules/rules/workflows/ai-growth-os-auto-application.md
    - D:/Android_source/ai-growth-os/components/rules/rules/workflows/ai-growth-os-brainstorming-gate.md
  project_refs:
    - AGENTS.md
    - README.md
    - build.md
    - err.md
    - docs/plans/PROJECT-MASTER-PLAN.md
    - docs/plans/2026-07-27-issue-75-gate-5-platform-paths.md
    - upstream/CodexPlusPlus/crates/codex-plus-core/src/app_paths.rs
    - upstream/CodexPlusPlus/crates/codex-plus-core/src/codex_home.rs
    - upstream/CodexPlusPlus/crates/codex-plus-core/src/paths.rs
  missing_coverage:
    - 没有 inputcodex GitNexus/CodeGraph 索引，不能使用图查询验证调用关系
    - windows 0.58.0 官方源码临时下载出现 403/timeout；使用锁文件版本、官方 API 名称和 Hosted 编译作为最终真实性门
```

## Superpowers 与项目方法

```yaml
superpowers_method_discipline:
  upstream_superpowers_ref: https://github.com/obra/superpowers
  local_superpowers_state: loaded-from-C:/Users/dashuai/.codex/superpowers
  using_superpowers: superpowers:using-superpowers
  brainstorming: superpowers:brainstorming，完整设计与书面规范已由项目所有者批准
  worktree_isolation:
    skill: superpowers:using-git-worktrees
    evidence: 当前已在从最新 origin/main 建立的 codex/issue-75-gate-5-platform-paths 隔离分支；实施前重新核对 git-dir/common-dir，不在当前规划阶段创建额外工作树
  planning_execution:
    writing_skill: superpowers:writing-plans
    execution_control: 项目原生书面规范、Session Plan 与 Runtime Workflow；所有者实施授权后按任务顺序执行
  test_driven_development:
    skill: superpowers:test-driven-development plus project-native build.md commands
    cycle: domain RED/GREEN -> application RED/GREEN -> platform common RED/GREEN -> windows/macos RED/GREEN -> parity RED/GREEN
  verification_before_completion:
    skill: superpowers:verification-before-completion
    evidence: 提交、推送、PR 和完成声明前运行本文件与 build.md 的定向验证，并核验 GitHub-hosted CI
  systematic_debugging:
    skill: superpowers:systematic-debugging
    evidence: 计划阶段 crates.io 403、官方 raw timeout 与 Windows 大补丁命令行长度失败均先确认外部或工具根因，未把失败写成产品事实
  code_review:
    requesting_skill: superpowers:requesting-code-review
    receiving_skill: superpowers:receiving-code-review
    evidence: 最终 Head 检查 changed surface、所有 Review 对话、根因处理和验证证据
  finishing_branch:
    skill: superpowers:finishing-a-development-branch
    evidence: 只创建非 Draft PR；最终 Squash Merge 等待项目所有者针对 Final Head 的单独授权
  evidence_writeback:
    active_control_plane: project-native docs/plans, docs/workflows, docs/reports, GitHub Issue and PR
    archive_policy: docs/superpowers remains archive-only, not the active control plane
```

## 决策

```yaml
decision:
  outcome: approved
  selected_architecture: domain protected values plus application port plus platform pure core and system adapters
  selected_feature: feature.foundation-platform.platform-paths
  written_spec_status: approved
  implementation_scope_status: approved
  accepted_semantics:
    - 未安装是 Ready 且 installation=None
    - 空白 CODEX_HOME 使用绝对用户目录下 .codex
    - 非空无效 CODEX_HOME 和无效显式路径明确失败
    - Windows 与 macOS 只扫描固定候选并拒绝管理器路径
  rejected_semantics:
    - 相对 .codex 或 .codex-session-delete 回退
    - 无效显式路径或无效非空 CODEX_HOME 静默自动探测
    - 私人绝对路径进入 Display、Serde、错误、日志或 CI
```

## Change Contract

```yaml
change_contract:
  target_contract:
    expected_behavior: Windows 与 macOS 通过同一应用端口返回受保护的 Codex 应用、CODEX_HOME 和 inputcodex 状态路径；未安装为 Ready+None，错误使用六个稳定诊断码且不泄露绝对路径
    evidence_refs:
      - https://github.com/nonononull/inputcodex/issues/74
      - https://github.com/nonononull/inputcodex/issues/75
      - docs/plans/2026-07-27-issue-75-gate-5-platform-paths.md
  preserved_invariants:
    - name: 现有加载协调器取消与过期结果语义不变
      baseline_ref: fc1683aabda4afb27ca333387ec954b6a405d2df crates/inputcodex-application/src/lib.rs
      regression_ref: inputcodex-application loading_state 与新增 platform_paths 测试全部通过
    - name: 平台层继续禁止 unsafe 且 Iced 不进入非展示层
      baseline_ref: fc1683aabda4afb27ca333387ec954b6a405d2df workspace lints and crate roots
      regression_ref: targeted clippy plus repository policy plus direct windows dependency feature audit
    - name: release_audit 与性能预算观察合同不变
      baseline_ref: fc1683aabda4afb27ca333387ec954b6a405d2df
      regression_ref: Verify-ReleaseAuditGate current and existing non-required Performance Baseline observation
  adjacent_surfaces:
    - name: 应用概览与生命周期
      why_adjacent: 后续功能会消费平台路径，但本任务不得读取版本、AppUserModelID、启动、停止或 Watcher
    - name: 设置与会话
      why_adjacent: 本任务只派生文件路径，不创建目录、不读写设置、不定位数据库或迁移会话
    - name: 上游缓存与一致性目录
      why_adjacent: 只读取 v1.2.43 审计证据并更新 platform-paths 自身合同，不修改 upstream 快照或其他 feature
  historical_state_refs:
    - Issue #26 / PR #27 Gate 4 功能目录
    - Issue #65 / PR #72 v1.2.43 功能目录重新审计
    - Issue #63 / PR #73 approved-observation 预算 CI
  stale_verdict_invalidation_refs:
    - origin/main、上游正式 Release、Release commit/tree、release_audit、三十路径或 scope_hash 任一变化时，本计划失效
  regression_checks:
    - surface: domain/application/platform 路径合同
      command_or_evidence_ref: cargo test --locked --offline --ignore-rust-version -p inputcodex-domain -p inputcodex-application -p inputcodex-platform
    - surface: parity 目录与来源副作用
      command_or_evidence_ref: cargo test --locked --offline --ignore-rust-version -p inputcodex-parity --test catalog_repository
    - surface: 架构、Release Audit、范围与空白
      command_or_evidence_ref: Verify-RepositoryPolicy.ps1 plus Verify-ReleaseAuditGate.ps1 plus scope_hash replay plus git diff --check
  sibling_regression_guard:
    status: pending
    evidence_ref: 实施后运行四 crate 定向测试、Clippy、仓库政策和 Hosted CI
    closeout_rule: passed-or-blocked-before-done
  protected_feature_replay:
    status: planned
    known_good_features:
      - feature: 现有 PlatformPort current_platform 和 LoadCoordinator 状态机保持通过
      - feature: release_audit=current、性能预算数值和 approved-observation 合同保持不变
    baseline_evidence_ref: fc1683aabda4afb27ca333387ec954b6a405d2df and Issue #63/#65 merged evidence
    post_change_replay_plan_ref: docs/workflows/2026-07-27-issue-75-gate-5-platform-paths-runtime.md
    post_change_replay_ref: 最终本地门禁与 Final Head GitHub-hosted CI/Performance observation
    expected_result: 只有批准三十路径的必要子集变化，平台路径行为实现且相邻产品功能、预算、Release Audit、Ruleset、upstream 和 UI 零语义漂移
    owner_visible_status: pending
    regression_status: pending
    forbidden_ops_until_replay:
      - Squash Merge、关闭 Issue #75、修改预算/Ruleset/Release/upstream/AGOS 或开始下一 Gate 5 功能
```

以下三十路径按 Windows PowerShell `Sort-Object` 排序，以 UTF-8 无 BOM、LF 连接并追加末尾 LF 后计算 SHA-256，正式执行哈希为 `sha256:ae5e0f5143355feee9b280da7c44fdd5cdf759ec2ae71fc69167040bf302cb37`：

```text
AGENTS.md
build.md
Cargo.lock
Cargo.toml
CONTEXT.md
crates/inputcodex-application/src/lib.rs
crates/inputcodex-application/src/platform_paths.rs
crates/inputcodex-application/tests/platform_paths.rs
crates/inputcodex-domain/src/lib.rs
crates/inputcodex-domain/src/platform_paths.rs
crates/inputcodex-domain/tests/platform_paths.rs
crates/inputcodex-parity/src/validation.rs
crates/inputcodex-parity/tests/catalog_repository.rs
crates/inputcodex-platform/Cargo.toml
crates/inputcodex-platform/src/lib.rs
crates/inputcodex-platform/src/platform_paths.rs
crates/inputcodex-platform/src/platform_paths/macos.rs
crates/inputcodex-platform/src/platform_paths/windows.rs
crates/inputcodex-platform/tests/platform_paths.rs
docs/plans/2026-07-27-issue-75-gate-5-platform-paths.md
docs/plans/PROJECT-MASTER-PLAN.md
docs/plans/sessions/2026-07-27-issue-75-gate-5-platform-paths.md
docs/reports/issue-75-gate-5-platform-paths.md
docs/workflows/2026-07-27-issue-75-gate-5-platform-paths-runtime.md
err.md
parity/contracts/foundation-platform.yml
parity/features/foundation-platform.yml
parity/features/source-index.yml
parity/README.md
README.md
```

## File Map

| 路径 | 单一职责 |
| --- | --- |
| `crates/inputcodex-domain/src/platform_paths.rs` | 私有路径、安装来源、安装结果和快照模型 |
| `crates/inputcodex-application/src/platform_paths.rs` | 请求、端口与同步用例，不读取环境或文件系统 |
| `crates/inputcodex-platform/src/platform_paths.rs` | 环境快照、共享根目录规则、派生文件名和系统入口 |
| `crates/inputcodex-platform/src/platform_paths/windows.rs` | Windows 包与独立安装候选，不包含共享业务规则 |
| `crates/inputcodex-platform/src/platform_paths/macos.rs` | macOS 固定应用目录候选，不包含共享业务规则 |
| 三个 `tests/platform_paths.rs` | 对外合同、隐私和用例状态；平台纯选择细节放在对应源文件单元测试 |
| Parity 验证器、三文件与仓库测试 | 将 `implemented` 生命周期、错误语义、`process-read` 和 Issue `#74/#75` 固定为机器合同 |
| 根文档、Master Plan、报告 | 记录 Gate 5 首个产品切片、构建入口、排错与审计证据 |

---

### Task 1：冻结规划控制面与授权边界

**Files:**
- Modify: `docs/plans/2026-07-27-issue-75-gate-5-platform-paths.md`
- Create: `docs/plans/sessions/2026-07-27-issue-75-gate-5-platform-paths.md`
- Create: `docs/workflows/2026-07-27-issue-75-gate-5-platform-paths-runtime.md`

**Interfaces:**
- Consumes: Issue `#75`、Issue `#74`、书面规范批准消息、三十路径哈希。
- Produces: 实施前唯一有效的 Session Plan、Runtime Workflow 和所有者授权门。

- [x] **Step 1：验证当前只修改三份规划文件**

```powershell
$allowed = @(
  'docs/plans/2026-07-27-issue-75-gate-5-platform-paths.md',
  'docs/plans/sessions/2026-07-27-issue-75-gate-5-platform-paths.md',
  'docs/workflows/2026-07-27-issue-75-gate-5-platform-paths-runtime.md'
)
$changed = @(git diff --name-only | Sort-Object)
$outside = @($changed | Where-Object { $_ -notin $allowed })
if ($outside.Count -ne 0) { throw "规划阶段越界：$($outside -join ', ')" }
```

预期：无越界路径；任何 Rust、Cargo、Parity、根状态文档变化都必须立即回退并停止。

- [x] **Step 2：提交并普通推送规划检查点**

```powershell
git add -- docs/plans/2026-07-27-issue-75-gate-5-platform-paths.md docs/plans/sessions/2026-07-27-issue-75-gate-5-platform-paths.md docs/workflows/2026-07-27-issue-75-gate-5-platform-paths-runtime.md
git commit -m "docs: 固定 issue 75 平台路径执行计划"
git push origin codex/issue-75-gate-5-platform-paths
```

预期：远端 Head 与本地 Head 相同；随后回写 Issue `#75`，等待项目所有者明确批准三十路径、`scope_hash` 和实施/PR 范围。未批准时不得执行 Task 2。

### Task 2：领域路径与快照合同

**Files:**
- Create: `crates/inputcodex-domain/tests/platform_paths.rs`
- Create: `crates/inputcodex-domain/src/platform_paths.rs`
- Modify: `crates/inputcodex-domain/src/lib.rs`

**Interfaces:**
- Consumes: `std::path::{Path, PathBuf}`。
- Produces: `PrivatePath::new(PathBuf) -> Result<PrivatePath, PrivatePathError>`、`ApplicationInstallSource`、`CodexInstallation`、`PlatformPathsSnapshot`。

- [ ] **Step 1：写领域 RED**

```rust
use std::path::PathBuf;

use inputcodex_domain::{
    ApplicationInstallSource, CodexInstallation, PlatformPathsSnapshot, PrivatePath,
    PrivatePathError,
};

#[test]
fn 私有路径拒绝空值和相对路径且_debug_脱敏() {
    assert_eq!(PrivatePath::new(PathBuf::new()), Err(PrivatePathError::Empty));
    assert_eq!(
        PrivatePath::new(PathBuf::from("relative/.codex")),
        Err(PrivatePathError::Relative)
    );

    let absolute = std::env::temp_dir().join("inputcodex-private-path");
    let value = PrivatePath::new(absolute.clone()).expect("临时目录路径应为绝对路径");
    let debug = format!("{value:?}");

    assert_eq!(debug, "PrivatePath(<redacted>)");
    assert!(!debug.contains(&absolute.to_string_lossy().to_string()));
    assert_eq!(value.as_path(), absolute.as_path());
}

#[test]
fn 平台快照保留明确空安装且所有路径继续脱敏() {
    let root = std::env::temp_dir().join("inputcodex-platform-paths-domain");
    let private = |name: &str| PrivatePath::new(root.join(name)).expect("派生路径应为绝对路径");
    let installation = CodexInstallation::new(
        private("Codex.app"),
        private("Codex.app/Contents/MacOS/Codex"),
        ApplicationInstallSource::Explicit,
    );
    let snapshot = PlatformPathsSnapshot::new(
        private(".codex"),
        private("state"),
        private("state/settings.json"),
        private("state/latest-status.json"),
        private("state/inputcodex.log"),
        Some(installation),
    );

    assert!(snapshot.codex_installation().is_some());
    assert!(!format!("{snapshot:?}").contains(&root.to_string_lossy().to_string()));
}
```

- [ ] **Step 2：运行 RED**

```powershell
cargo test --locked --offline --ignore-rust-version -p inputcodex-domain --test platform_paths
```

预期：编译失败，原因是 `platform_paths` 模块及五个公开类型尚不存在。

- [ ] **Step 3：写最小领域 GREEN**

```rust
use std::{fmt, path::{Path, PathBuf}};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PrivatePathError { Empty, Relative }

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct PrivatePath(PathBuf);

impl PrivatePath {
    pub fn new(path: PathBuf) -> Result<Self, PrivatePathError> {
        if path.as_os_str().is_empty() { return Err(PrivatePathError::Empty); }
        if !path.is_absolute() { return Err(PrivatePathError::Relative); }
        Ok(Self(path))
    }

    #[must_use]
    pub fn as_path(&self) -> &Path { &self.0 }
}

impl fmt::Debug for PrivatePath {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("PrivatePath(<redacted>)")
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ApplicationInstallSource {
    Explicit,
    WindowsPackage,
    WindowsStandalone,
    MacosSystemApplications,
    MacosUserApplications,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CodexInstallation {
    application_root: PrivatePath,
    executable: PrivatePath,
    source: ApplicationInstallSource,
}

impl CodexInstallation {
    pub const fn new(application_root: PrivatePath, executable: PrivatePath, source: ApplicationInstallSource) -> Self {
        Self { application_root, executable, source }
    }
    pub const fn application_root(&self) -> &PrivatePath { &self.application_root }
    pub const fn executable(&self) -> &PrivatePath { &self.executable }
    pub const fn source(&self) -> ApplicationInstallSource { self.source }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PlatformPathsSnapshot {
    codex_home: PrivatePath,
    inputcodex_state_root: PrivatePath,
    settings_file: PrivatePath,
    latest_status_file: PrivatePath,
    diagnostic_log_file: PrivatePath,
    codex_installation: Option<CodexInstallation>,
}

impl PlatformPathsSnapshot {
    pub const fn new(codex_home: PrivatePath, inputcodex_state_root: PrivatePath, settings_file: PrivatePath, latest_status_file: PrivatePath, diagnostic_log_file: PrivatePath, codex_installation: Option<CodexInstallation>) -> Self {
        Self { codex_home, inputcodex_state_root, settings_file, latest_status_file, diagnostic_log_file, codex_installation }
    }
    pub const fn codex_home(&self) -> &PrivatePath { &self.codex_home }
    pub const fn inputcodex_state_root(&self) -> &PrivatePath { &self.inputcodex_state_root }
    pub const fn settings_file(&self) -> &PrivatePath { &self.settings_file }
    pub const fn latest_status_file(&self) -> &PrivatePath { &self.latest_status_file }
    pub const fn diagnostic_log_file(&self) -> &PrivatePath { &self.diagnostic_log_file }
    pub const fn codex_installation(&self) -> Option<&CodexInstallation> { self.codex_installation.as_ref() }
}
```

在 `lib.rs` 中只增加 `mod platform_paths;` 与上述类型的 `pub use`；不得为路径类型增加 `Display`、Serde、字符串 getter 或拥有值导出。

- [ ] **Step 4：运行 GREEN 并提交**

```powershell
cargo test --locked --offline --ignore-rust-version -p inputcodex-domain --test platform_paths
cargo test --locked --offline --ignore-rust-version -p inputcodex-domain
git add -- crates/inputcodex-domain/src/lib.rs crates/inputcodex-domain/src/platform_paths.rs crates/inputcodex-domain/tests/platform_paths.rs
git commit -m "feat: 增加受保护平台路径领域模型"
```

预期：领域新旧测试全部通过，`Debug` 中不存在真实绝对路径。

### Task 3：应用端口、请求与加载语义

**Files:**
- Create: `crates/inputcodex-application/tests/platform_paths.rs`
- Create: `crates/inputcodex-application/src/platform_paths.rs`
- Modify: `crates/inputcodex-application/src/lib.rs`

**Interfaces:**
- Consumes: `PlatformPathsSnapshot`、`ApplicationError`、`LoadCompletion`、`LoadCoordinator`、`RequestId`。
- Produces: `PlatformPathsRequest`、`PlatformPathsPort::resolve`、`ResolvePlatformPaths::execute`。

- [ ] **Step 1：写应用 RED**

```rust
use std::path::PathBuf;

use inputcodex_application::{
    ApplicationError, ErrorKind, LoadCoordinator, LoadState, PlatformPathsPort,
    PlatformPathsRequest, RequestId, ResolvePlatformPaths, TransitionOutcome,
};
use inputcodex_domain::{PlatformPathsSnapshot, PrivatePath};

#[derive(Clone)]
struct StubPort(Result<PlatformPathsSnapshot, ApplicationError>);

impl PlatformPathsPort for StubPort {
    fn resolve(&self, _request: &PlatformPathsRequest) -> Result<PlatformPathsSnapshot, ApplicationError> {
        self.0.clone()
    }
}

fn empty_snapshot() -> PlatformPathsSnapshot {
    let root = std::env::temp_dir().join("inputcodex-application-paths");
    let private = |name: &str| PrivatePath::new(root.join(name)).expect("绝对路径");
    PlatformPathsSnapshot::new(private(".codex"), private("state"), private("state/settings.json"), private("state/latest-status.json"), private("state/inputcodex.log"), None)
}

#[test]
fn 未安装返回_ready_快照而不是_empty() {
    let use_case = ResolvePlatformPaths::new(StubPort(Ok(empty_snapshot())));
    assert!(matches!(use_case.execute(&PlatformPathsRequest::default()), inputcodex_application::LoadCompletion::Ready(snapshot) if snapshot.codex_installation().is_none()));
}

#[test]
fn 失败保持稳定分类且请求_debug_不泄露路径() {
    let path = std::env::temp_dir().join("private/Codex.app");
    let request = PlatformPathsRequest::new(Some(path.clone()));
    assert!(!format!("{request:?}").contains(&path.to_string_lossy().to_string()));

    let use_case = ResolvePlatformPaths::new(StubPort(Err(ApplicationError::unavailable("EXPLICIT_CODEX_PATH_INVALID"))));
    let completion = use_case.execute(&request);
    assert!(matches!(completion, inputcodex_application::LoadCompletion::Failed(error) if error.kind() == ErrorKind::Unavailable && error.code().as_str() == "EXPLICIT_CODEX_PATH_INVALID"));
}

#[test]
fn 取消后的同步结果按现有协调器规则变为过期() {
    let request_id = RequestId::new(7);
    let use_case = ResolvePlatformPaths::new(StubPort(Ok(empty_snapshot())));
    let completion = use_case.execute(&PlatformPathsRequest::default());
    let mut coordinator = LoadCoordinator::default();
    coordinator.begin(request_id);
    assert_eq!(coordinator.cancel(request_id), TransitionOutcome::Applied);
    assert_eq!(coordinator.complete(request_id, completion), TransitionOutcome::Stale);
    assert!(matches!(coordinator.state(), LoadState::Cancelling { request_id: current } if *current == request_id));
}
```

- [ ] **Step 2：运行 RED**

```powershell
cargo test --locked --offline --ignore-rust-version -p inputcodex-application --test platform_paths
```

预期：编译失败，原因是应用端口、请求与用例尚不存在。

- [ ] **Step 3：写最小应用 GREEN**

```rust
use std::{fmt, path::{Path, PathBuf}};
use inputcodex_domain::PlatformPathsSnapshot;
use crate::{ApplicationError, LoadCompletion};

#[derive(Clone, Default, PartialEq, Eq)]
pub struct PlatformPathsRequest { explicit_application_path: Option<PathBuf> }

impl PlatformPathsRequest {
    pub const fn new(explicit_application_path: Option<PathBuf>) -> Self { Self { explicit_application_path } }
    pub fn explicit_application_path(&self) -> Option<&Path> { self.explicit_application_path.as_deref() }
}

impl fmt::Debug for PlatformPathsRequest {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let value = self.explicit_application_path.as_ref().map(|_| "<redacted>");
        formatter.debug_struct("PlatformPathsRequest").field("explicit_application_path", &value).finish()
    }
}

pub trait PlatformPathsPort {
    fn resolve(&self, request: &PlatformPathsRequest) -> Result<PlatformPathsSnapshot, ApplicationError>;
}

#[derive(Debug, Clone)]
pub struct ResolvePlatformPaths<P> { port: P }

impl<P> ResolvePlatformPaths<P> {
    pub const fn new(port: P) -> Self { Self { port } }
    pub const fn port(&self) -> &P { &self.port }
}

impl<P: PlatformPathsPort> ResolvePlatformPaths<P> {
    pub fn execute(&self, request: &PlatformPathsRequest) -> LoadCompletion<PlatformPathsSnapshot> {
        match self.port.resolve(request) {
            Ok(snapshot) => LoadCompletion::Ready(snapshot),
            Err(error) => LoadCompletion::Failed(error),
        }
    }
}
```

同时为 `ApplicationError` 增加 `pub const fn internal(code: &'static str) -> Self`，映射到 `ErrorKind::Internal`；`lib.rs` 只做模块和公开类型转出。

- [ ] **Step 4：运行 GREEN 并提交**

```powershell
cargo test --locked --offline --ignore-rust-version -p inputcodex-application --test platform_paths
cargo test --locked --offline --ignore-rust-version -p inputcodex-application
git add -- crates/inputcodex-application/src/lib.rs crates/inputcodex-application/src/platform_paths.rs crates/inputcodex-application/tests/platform_paths.rs
git commit -m "feat: 增加平台路径应用端口与用例"
```

预期：未安装只产生 `Ready`，失败不产生 `Empty`，取消和过期结果继续由现有协调器阻断。

### Task 4：共享解析核心与固定错误语义

**Files:**
- Create: `crates/inputcodex-platform/src/platform_paths.rs`
- Modify: `crates/inputcodex-platform/src/lib.rs`
- Create: `crates/inputcodex-platform/tests/platform_paths.rs`

**Interfaces:**
- Consumes: `PlatformPathsRequest`、`PlatformPathsPort`、领域快照。
- Produces: `SystemPlatformPaths`、内部 `PathProbe`、`CommonInputs`、`resolve_common_paths`。

- [ ] **Step 1：写共享规则 RED**

在 `src/platform_paths.rs` 的 `#[cfg(test)]` 单元测试中用内存 `PathProbe` 固定以下断言：空白 `CODEX_HOME` 使用绝对用户 `.codex`；非空相对、不存在或文件路径返回 `CODEX_HOME_INVALID`；用户目录缺失返回 `USER_HOME_UNAVAILABLE`；Windows `LOCALAPPDATA` 缺失返回 `INPUTCODEX_STATE_ROOT_UNAVAILABLE`；三个派生文件名严格为 `settings.json`、`latest-status.json`、`inputcodex.log`。

在 `tests/platform_paths.rs` 写公开合同：

```rust
use inputcodex_application::PlatformPathsPort;
use inputcodex_platform::SystemPlatformPaths;

#[test]
fn 系统平台路径解析器实现应用端口且不需要_unsafe() {
    fn assert_port<T: PlatformPathsPort + Default>() {}
    assert_port::<SystemPlatformPaths>();
}
```

- [ ] **Step 2：运行 RED**

```powershell
cargo test --locked --offline --ignore-rust-version -p inputcodex-platform --test platform_paths
```

预期：编译失败，原因是 `SystemPlatformPaths` 尚不存在。

- [ ] **Step 3：实现共享解析骨架**

```rust
const SETTINGS_FILE: &str = "settings.json";
const LATEST_STATUS_FILE: &str = "latest-status.json";
const DIAGNOSTIC_LOG_FILE: &str = "inputcodex.log";

trait PathProbe {
    fn is_dir(&self, path: &std::path::Path) -> bool;
    fn is_file(&self, path: &std::path::Path) -> bool;
}

#[derive(Debug, Clone, Copy, Default)]
struct SystemPathProbe;

impl PathProbe for SystemPathProbe {
    fn is_dir(&self, path: &std::path::Path) -> bool { path.is_dir() }
    fn is_file(&self, path: &std::path::Path) -> bool { path.is_file() }
}

struct CommonInputs {
    user_home: Option<std::path::PathBuf>,
    codex_home: Option<std::ffi::OsString>,
    inputcodex_state_root: Option<std::path::PathBuf>,
}

fn resolve_common_paths(
    inputs: CommonInputs,
    installation: Option<inputcodex_domain::CodexInstallation>,
    probe: &impl PathProbe,
) -> Result<inputcodex_domain::PlatformPathsSnapshot, inputcodex_application::ApplicationError> {
    let user_home = inputs.user_home
        .filter(|path| path.is_absolute() && probe.is_dir(path))
        .ok_or_else(|| inputcodex_application::ApplicationError::unavailable("USER_HOME_UNAVAILABLE"))?;
    let state_root = inputs.inputcodex_state_root
        .filter(|path| path.is_absolute())
        .ok_or_else(|| inputcodex_application::ApplicationError::unavailable("INPUTCODEX_STATE_ROOT_UNAVAILABLE"))?;
    let codex_home = match inputs.codex_home {
        None => user_home.join(".codex"),
        Some(value) if value.to_string_lossy().trim().is_empty() => user_home.join(".codex"),
        Some(value) => {
            let path = std::path::PathBuf::from(value);
            if !path.is_absolute() || !probe.is_dir(&path) {
                return Err(inputcodex_application::ApplicationError::unavailable("CODEX_HOME_INVALID"));
            }
            path
        }
    };
    let private = |path| inputcodex_domain::PrivatePath::new(path)
        .map_err(|_| inputcodex_application::ApplicationError::internal("PLATFORM_PATHS_FAILED"));
    Ok(inputcodex_domain::PlatformPathsSnapshot::new(
        private(codex_home)?,
        private(state_root.clone())?,
        private(state_root.join(SETTINGS_FILE))?,
        private(state_root.join(LATEST_STATUS_FILE))?,
        private(state_root.join(DIAGNOSTIC_LOG_FILE))?,
        installation,
    ))
}
```

`SystemPlatformPaths::resolve` 必须按 `cfg(target_os)` 选择 Windows/macOS；其他平台直接返回 `ApplicationError::unsupported("PLATFORM_PATHS_UNSUPPORTED")`。环境读取只允许 `CODEX_HOME`、Windows `USERPROFILE`/`LOCALAPPDATA`、macOS `HOME`，不得读取当前目录作为后备。

- [ ] **Step 4：运行共享 GREEN**

```powershell
cargo test --locked --offline --ignore-rust-version -p inputcodex-platform
```

预期：共享根目录、派生文件名、unsupported 和公开端口合同通过；尚未加入平台候选时对应平台发现测试仍保持 RED。

### Task 5：Windows 与 macOS 平台适配器

**Files:**
- Modify: `Cargo.toml`
- Modify if required by Cargo: `Cargo.lock`
- Modify: `crates/inputcodex-platform/Cargo.toml`
- Create: `crates/inputcodex-platform/src/platform_paths/windows.rs`
- Create: `crates/inputcodex-platform/src/platform_paths/macos.rs`
- Modify: `crates/inputcodex-platform/src/platform_paths.rs`

**Interfaces:**
- Consumes: `PathProbe`、固定环境根、`PlatformPathsRequest`。
- Produces: `windows::resolve_explicit/discover`、`macos::resolve_explicit/discover`，均返回 `Option<CodexInstallation>` 或稳定错误。

- [ ] **Step 1：写平台选择 RED**

Windows 单元测试必须覆盖：三个 Package Family、数值版本降序、同版本身份顺序、`app` 子目录或根目录安全可执行文件、三个 standalone 根、大小写等价 `Codex.exe`/`ChatGPT.exe`、管理器路径拒绝、显式无效不回退。macOS 单元测试必须覆盖：`/Applications` 优先 `$HOME/Applications`、四个固定 `.app` 名称顺序、显式 `.app` 与 `Contents/MacOS` 可执行文件、管理器路径拒绝、最多八个自动候选。

- [ ] **Step 2：固定依赖合同**

```toml
# Cargo.toml [workspace.dependencies]
windows = { version = "=0.58.0", default-features = false, features = [
    "ApplicationModel",
    "Foundation_Collections",
    "Management_Deployment",
    "Storage",
] }

# crates/inputcodex-platform/Cargo.toml
[dependencies]
inputcodex-application.workspace = true
inputcodex-domain.workspace = true

[target.'cfg(target_os = "windows")'.dependencies]
windows.workspace = true
```

不得加入 `directories`、`dirs`、Tokio、Serde、正则、shell 或新依赖家族；`Cargo.lock` 若内容未变化则保持不动。

- [ ] **Step 3：实现 Windows 安全 WinRT 查询**

```rust
#[cfg(target_os = "windows")]
fn registered_package_candidates() -> Result<Vec<WindowsPackageCandidate>, ApplicationError> {
    use windows::{Management::Deployment::PackageManager, core::HSTRING};

    let manager = PackageManager::new()
        .map_err(|_| ApplicationError::internal("PLATFORM_PATHS_FAILED"))?;
    let mut candidates = Vec::new();
    for (identity_order, family_name) in PACKAGE_FAMILY_NAMES.iter().enumerate() {
        let packages = manager
            .FindPackagesByPackageFamilyName(&HSTRING::from(*family_name))
            .map_err(|_| ApplicationError::internal("PLATFORM_PATHS_FAILED"))?;
        for package in packages {
            let package = package.map_err(|_| ApplicationError::internal("PLATFORM_PATHS_FAILED"))?;
            let id = package.Id().map_err(|_| ApplicationError::internal("PLATFORM_PATHS_FAILED"))?;
            let version = id.Version().map_err(|_| ApplicationError::internal("PLATFORM_PATHS_FAILED"))?;
            let root = package
                .InstalledLocation()
                .and_then(|folder| folder.Path())
                .map(|path| std::path::PathBuf::from(path.to_string()))
                .map_err(|_| ApplicationError::internal("PLATFORM_PATHS_FAILED"))?;
            candidates.push(WindowsPackageCandidate {
                identity_order,
                version: [version.Major, version.Minor, version.Build, version.Revision],
                root,
            });
        }
    }
    Ok(candidates)
}
```

固定 Family 为 `OpenAI.Codex_2p2nqsd0c76g0`、`OpenAI.CodexBeta_2p2nqsd0c76g0`、`OpenAI.ChatGPT-Desktop_2p2nqsd0c76g0`。只创建一个 `PackageManager`，每个固定 Family 最多查询一次；查询错误直接失败，不静默退到 standalone。候选只接受 `Codex.exe` 或 `ChatGPT.exe`，应用根只允许包根、包根 `app` 或三个固定 standalone 目录。

- [ ] **Step 4：实现 macOS 固定候选**

```rust
const APP_NAMES: [&str; 4] = ["Codex.app", "OpenAI Codex.app", "OpenAI.Codex.app", "ChatGPT.app"];
const EXECUTABLE_NAMES: [&str; 2] = ["Codex", "ChatGPT"];

fn discover(home: &std::path::Path, probe: &impl PathProbe) -> Option<CodexInstallation> {
    let roots = [
        (std::path::PathBuf::from("/Applications"), ApplicationInstallSource::MacosSystemApplications),
        (home.join("Applications"), ApplicationInstallSource::MacosUserApplications),
    ];
    roots.into_iter().find_map(|(root, source)| {
        APP_NAMES.into_iter().find_map(|name| installation_from_bundle(root.join(name), source, probe))
    })
}
```

显式路径必须绝对；若为 `Contents/MacOS/Codex` 或 `ChatGPT`，只回溯到恰好包含 `.app/Contents/MacOS` 的应用包。任何路径组成部分大小写归一化后命中 `inputcodex`、`codex++`、`codexplusplus` 或 `codex-plus-manager` 都返回 `EXPLICIT_CODEX_PATH_INVALID`。

- [ ] **Step 5：运行双平台本地 GREEN 并提交**

```powershell
cargo test --locked --offline --ignore-rust-version -p inputcodex-platform
cargo test --locked --offline --ignore-rust-version -p inputcodex-domain -p inputcodex-application -p inputcodex-platform
cargo clippy --locked --offline --ignore-rust-version -p inputcodex-domain -p inputcodex-application -p inputcodex-platform --all-targets -- -D warnings
git add -- Cargo.toml Cargo.lock crates/inputcodex-platform/Cargo.toml crates/inputcodex-platform/src/lib.rs crates/inputcodex-platform/src/platform_paths.rs crates/inputcodex-platform/src/platform_paths/windows.rs crates/inputcodex-platform/src/platform_paths/macos.rs crates/inputcodex-platform/tests/platform_paths.rs
git commit -m "feat: 实现双平台路径解析适配器"
```

预期：Windows 本机定向编译测试通过；macOS 纯选择单元测试在所有平台通过，真实 macOS 编译等待 Hosted CI。

### Task 6：Parity 目录和来源副作用 TDD

**Files:**
- Modify: `crates/inputcodex-parity/src/validation.rs`
- Modify: `crates/inputcodex-parity/tests/catalog_repository.rs`
- Modify: `parity/contracts/foundation-platform.yml`
- Modify: `parity/features/foundation-platform.yml`
- Modify: `parity/features/source-index.yml`
- Modify: `parity/README.md`

**Interfaces:**
- Consumes: 实现后的稳定行为、Issue `#74/#75`。
- Produces: `implemented` 功能状态、六个稳定错误码、`filesystem-read + process-read` 来源事实。

- [ ] **Step 1：写 Parity RED**

在 `catalog_repository.rs` 增加精确文本断言：`status: implemented`、`issue:74`、`issue:75`、六个错误码、未安装 `Ready + installation=None`、三个来源入口均包含 `side_effects: [filesystem-read, process-read]`，并断言仍不存在广告、远程推荐、写入或网络副作用。

- [ ] **Step 2：运行 RED**

```powershell
cargo test --locked --offline --ignore-rust-version -p inputcodex-parity --test catalog_repository
```

预期：目录精确断言先因 `unassessed`、错误码不完整和 `process-read` 缺失而失败；最小目录更新后，完整仓库测试继续以 `InvalidInitialParityStatus @ feature.foundation-platform.platform-paths` 证明旧 Gate 4 验证器尚未接受首个已实现 Gate 5 功能。

- [ ] **Step 3：最小更新目录与合同**

`feature.foundation-platform.platform-paths` 只改为 `implemented` 并补 Issue `#74/#75`；baseline contract 增加 `process-read`、六个错误码和未安装语义；`core-module:app_paths`、`core-module:codex_home`、`core-module:paths` 三项副作用改为 `[filesystem-read, process-read]`。仓库生命周期验证只新增接受 `ParityStatus::Implemented`，继续拒绝 `planned`、`implementing`、`verified`、`exception-approved` 和 `retired`。不得修改其他 feature 状态或来源入口。

- [ ] **Step 4：运行 GREEN 并提交**

```powershell
cargo test --locked --offline --ignore-rust-version -p inputcodex-parity --test catalog_repository
cargo test --locked --offline --ignore-rust-version -p inputcodex-parity
git add -- crates/inputcodex-parity/src/validation.rs crates/inputcodex-parity/tests/catalog_repository.rs parity/contracts/foundation-platform.yml parity/features/foundation-platform.yml parity/features/source-index.yml parity/README.md
git commit -m "test: 固定平台路径一致性合同"
```

预期：Parity 全部测试通过，未触碰其他域、fixture 或上游快照。

### Task 7：项目控制面、构建与排错收口

**Files:**
- Modify: `AGENTS.md`
- Modify: `build.md`
- Modify: `CONTEXT.md`
- Modify: `README.md`
- Modify: `docs/plans/PROJECT-MASTER-PLAN.md`
- Modify: `docs/plans/2026-07-27-issue-75-gate-5-platform-paths.md`
- Modify: `docs/plans/sessions/2026-07-27-issue-75-gate-5-platform-paths.md`
- Modify: `docs/workflows/2026-07-27-issue-75-gate-5-platform-paths-runtime.md`
- Create: `docs/reports/issue-75-gate-5-platform-paths.md`
- Modify: `err.md`

**Interfaces:**
- Consumes: TDD 与本地验证真实输出。
- Produces: Gate 5 首个切片状态、定向构建命令、错误处理记录和反递归报告。

- [ ] **Step 1：更新状态但不预写动态 GitHub 证据**

根文档只声明“Issue `#75` 实现已进入 PR 验证”或其真实阶段；不得在 PR 创建前写虚构 Head、CI、Review、签名、Tree 或合并提交。报告记录设计、范围、RED/GREEN、本地命令与输出；最终 Head、Review、CI、授权和合并证据保留在 GitHub Issue/PR 评论，避免递归 Closeout。

- [ ] **Step 2：把正确命令写入 `build.md`**

```powershell
cargo test --locked --offline --ignore-rust-version -p inputcodex-domain --test platform_paths
cargo test --locked --offline --ignore-rust-version -p inputcodex-application --test platform_paths
cargo test --locked --offline --ignore-rust-version -p inputcodex-platform --test platform_paths
cargo test --locked --offline --ignore-rust-version -p inputcodex-parity --test catalog_repository
cargo fmt --all -- --check
pwsh -NoProfile -File scripts/ci/Verify-RepositoryPolicy.ps1 -RepositoryRoot .
git diff --check
```

- [ ] **Step 3：记录真实排错**

只有实际出现且已确定根因的问题才写入 `err.md`；网络文档下载的 `403/timeout` 只能记录为计划阶段外部资料获取失败，结论必须说明仓库未受影响、Windows API 最终采用锁定版本安全 WinRT 合同，不得把网络失败伪装为产品错误。

- [ ] **Step 4：提交控制面同步**

```powershell
git add -- AGENTS.md build.md CONTEXT.md README.md docs/plans/PROJECT-MASTER-PLAN.md docs/plans/2026-07-27-issue-75-gate-5-platform-paths.md docs/plans/sessions/2026-07-27-issue-75-gate-5-platform-paths.md docs/workflows/2026-07-27-issue-75-gate-5-platform-paths-runtime.md docs/reports/issue-75-gate-5-platform-paths.md err.md
git commit -m "docs: 收口平台路径迁移证据"
```

### Task 8：最终本地验证、PR、Review 与 Hosted CI

**Files:**
- Verify: 全部三十路径中的实际差异子集。

**Interfaces:**
- Consumes: 最终实现 Head。
- Produces: 可供项目所有者单独 Squash Merge 授权的 PR Final Head 与证据。

- [ ] **Step 1：运行最终轻量门禁**

```powershell
cargo test --locked --offline --ignore-rust-version -p inputcodex-domain -p inputcodex-application -p inputcodex-platform -p inputcodex-parity
cargo clippy --locked --offline --ignore-rust-version -p inputcodex-domain -p inputcodex-application -p inputcodex-platform -p inputcodex-parity --all-targets -- -D warnings
cargo fmt --all -- --check
pwsh -NoProfile -File scripts/ci/Test-CiScripts.ps1
pwsh -NoProfile -File scripts/ci/Verify-ReleaseAuditGate.ps1 -RepositoryRoot .
pwsh -NoProfile -File scripts/ci/Verify-RepositoryPolicy.ps1 -RepositoryRoot .
git diff --check
```

预期：定向测试与 Clippy 全绿、CI 合同 `35/35`、`release_audit=current`、仓库政策零违规、无空白错误。

- [ ] **Step 2：验证范围与私人路径泄露**

使用 Runtime Workflow 中的精确脚本复算三十路径哈希；`git diff --name-only origin/main...HEAD` 必须是批准集合子集。对三十路径与测试输出搜索本机用户目录、`C:\Users\dashuai`、`/Users/` 和未脱敏 `Debug`；真实路径只允许出现在项目根控制面固定路径字段，不得出现在产品错误、快照 Debug 或测试断言输出。

- [ ] **Step 3：普通推送并创建非 Draft PR**

```powershell
git push origin codex/issue-75-gate-5-platform-paths
gh pr create --repo nonononull/inputcodex --base main --head codex/issue-75-gate-5-platform-paths --title "feat: 迁移平台路径解析能力" --body-file $env:TEMP\issue-75-pr-body.md
```

PR 正文必须包含 `Closes #75`、Issue `#74`、三十路径哈希、RED/GREEN 命令、隐私边界、Windows 安全 WinRT 特性、无 UI/写入/网络/缓存声明和最终 Squash Merge 单独授权门。

- [ ] **Step 4：核验 Review 与 Hosted CI**

最终 Head 要求标准 CI 全部预期 Job 成功、Windows/macOS 真实编译测试成功、Performance Baseline 非 required observation 成功、成功 Artifact 数为 `0`。任何失败先使用 `superpowers:systematic-debugging` 确定根因；所有 Review 对话必须写明根因、处理方式和验证证据后解决。

- [ ] **Step 5：停止在独立合并授权门**

只向项目所有者报告 Final Head SHA、Review 对话数、CI/Performance Run、Artifact 数、范围和 `scope_hash`；没有针对 Final Head 的单独“授权 Squash Merge”不得合并。

## Self-Review

```yaml
spec_coverage: passed
placeholder_scan: passed-no-prohibited-placeholder-markers
type_consistency: passed
scope_check: passed-single-feature
dependency_check: passed-windows-0.58.0-existing-family
unsafe_check: passed-safe-winrt-only
privacy_check: passed-redacted-path-values-and-request
platform_check: passed-equivalent-windows-macos-contract
ui_boundary: passed-no-ui
implementation_started: false
implementation_authorization: approved
```

本计划完整覆盖书面规范的领域、应用、共享解析、Windows、macOS、Parity、文档、验证和交付要求。项目所有者已批准三十路径、`scope_hash` 和实施/PR 操作，当前允许针对已确认的 Parity 初始状态根因继续 TDD；最终 Squash Merge 仍保留单独授权门。
