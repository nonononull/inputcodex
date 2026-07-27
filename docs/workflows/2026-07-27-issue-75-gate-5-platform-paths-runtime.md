# Issue #75：Gate 5 平台路径解析 Runtime Workflow

```yaml
task_id: issue-75-gate-5-platform-paths
tracking_issue_ref: https://github.com/nonononull/inputcodex/issues/75
parity_exception_ref: https://github.com/nonononull/inputcodex/issues/74
session_plan_ref: docs/plans/sessions/2026-07-27-issue-75-gate-5-platform-paths.md
implementation_plan_ref: docs/plans/2026-07-27-issue-75-gate-5-platform-paths.md
approved_decision_ref: https://github.com/nonononull/inputcodex/issues/75#issuecomment-5092021020
selected_business_path: inputcodex.gate5.foundation-platform.platform-paths
baseline_ref: fc1683aabda4afb27ca333387ec954b6a405d2df
baseline_tree: d17a038fcb4fc986565f121283481eb38cdfbc33
branch: codex/issue-75-gate-5-platform-paths
scope_hash: sha256:ae5e0f5143355feee9b280da7c44fdd5cdf759ec2ae71fc69167040bf302cb37
scope_status: approved-frozen
scope_approval_ref: https://github.com/nonononull/inputcodex/issues/75#issuecomment-5092021020
current_execution_status: local-verification-passed-pr-pending
current_allowed_operations: exact-thirty-path-tdd, lightweight-local-verification, git-checkpoints, normal-commit, normal-push, non-draft-pr, review-ci
post_approval_allowed_operations: exact-thirty-path-tdd, lightweight-local-verification, git-checkpoints, normal-commit, normal-push, non-draft-pr, review-ci
executor_enforcement: exact-thirty-paths, no-force-push, never-delete-main, squash-only
local_time_source: Windows Get-Date
pr_ref: pending-not-created
ci_ref: pending-pr-head
merge_ref: pending-separate-owner-authorization
```

## 工作流节点

1. `startup-baseline`：核对 `origin/main=fc1683aabda4afb27ca333387ec954b6a405d2df`、隔离分支、Issue `#74/#75`、干净启动面和 Windows 本机时间。
2. `written-spec`：书面规范已批准；Windows API 合同修正为 `windows 0.58.0` 安全 WinRT，不允许直接 Win32 FFI 或 `unsafe`。
3. `planning-control`：只落盘三份规划文件并运行文档、范围、控制字符和空白检查；普通提交、普通推送并回写 Issue `#75`。
4. `owner-implementation-gate`：项目所有者已明确批准三十路径、`scope_hash`、TDD、轻量验证、提交、普通推送、非 Draft PR 与 Review/CI；新增路径只允许验证器接受 `ParityStatus::Implemented`，最终 Squash Merge 仍单独授权。
5. `domain-red-green`：先写 `PrivatePath`、安装来源和快照 RED，再做最小 GREEN；路径 `Debug` 必须脱敏。
6. `application-red-green`：先写请求、端口、用例、Ready+None、失败和取消/过期结果 RED，再做最小 GREEN。
7. `platform-common-red-green`：固定 `CODEX_HOME`、用户目录、状态根、派生文件名和 unsupported 错误，再实现共享纯解析核心。
8. `windows-red-green`：固定三个 Package Family、版本排序、standalone、安全可执行名、管理器拒绝和显式失败；使用安全 WinRT 实现。
9. `macos-red-green`：固定两个根、四个应用名、显式 bundle/可执行路径、安全名称和管理器拒绝。
10. `parity-red-green`：先使目录测试因 `unassessed`、错误码和 `process-read` 缺失而失败，再最小更新平台路径 feature、contract 和三个来源入口。
11. `control-plane-sync`：按真实阶段更新 `AGENTS.md`、`build.md`、`CONTEXT.md`、`README.md`、Master Plan、Parity README、报告和 `err.md`。
12. `local-verify`：运行四 crate 定向测试/Clippy、格式、CI 合同、Release Audit、Repository Policy、范围、隐私和空白检查。
13. `delivery`：普通推送并创建关联 Issue `#75` 的非 Draft PR；禁止 force push。
14. `review-ci`：最终 Head 核验 Review 对话、标准 CI、Windows/macOS 真实编译测试、Performance observation 和 Artifact `0`；失败必须先确定根因。
15. `merge-gate`：只报告 Final Head 与证据，等待项目所有者单独授权 Squash Merge。

## 精确写入范围

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

实际差异允许是上述集合的子集；任何新增、删除、重命名或移出集合都必须硬停止、重算哈希并取得项目所有者重新批准。路径在集合内也不授权与 Issue `#75` 无关的修改。

## 阶段授权

| 阶段 | 允许 | 禁止 |
| --- | --- | --- |
| 当前规划 | 三份规划文件、文档校验、普通提交/推送、Issue 回写 | Rust、Cargo、Parity、根状态文档、报告、PR、实现 CI、合并 |
| 所有者实施批准后 | 三十路径 TDD、轻量验证、checkpoint 提交、普通推送、非 Draft PR、Review/CI | 扩范围、force push、删除或改写 `main`、修改 Ruleset/预算/Release/`upstream/`/AGOS |
| 产品边界 | 路径发现、验证、选择和安全返回 | UI、Iced、写入、网络、缓存、后台线程、概览、版本、生命周期、设置、会话、Watcher、安装更新 |
| 最终合并 | 仅在 Final Head 单独授权后 Squash Merge | Merge Commit、Rebase Merge、未解决 Review 对话、未知根因失败、提前关闭 Issue |

## 范围哈希验证

```powershell
$approved = [string[]]@(
  'AGENTS.md',
  'build.md',
  'Cargo.lock',
  'Cargo.toml',
  'CONTEXT.md',
  'crates/inputcodex-application/src/lib.rs',
  'crates/inputcodex-application/src/platform_paths.rs',
  'crates/inputcodex-application/tests/platform_paths.rs',
  'crates/inputcodex-domain/src/lib.rs',
  'crates/inputcodex-domain/src/platform_paths.rs',
  'crates/inputcodex-domain/tests/platform_paths.rs',
  'crates/inputcodex-parity/src/validation.rs',
  'crates/inputcodex-parity/tests/catalog_repository.rs',
  'crates/inputcodex-platform/Cargo.toml',
  'crates/inputcodex-platform/src/lib.rs',
  'crates/inputcodex-platform/src/platform_paths.rs',
  'crates/inputcodex-platform/src/platform_paths/macos.rs',
  'crates/inputcodex-platform/src/platform_paths/windows.rs',
  'crates/inputcodex-platform/tests/platform_paths.rs',
  'docs/plans/2026-07-27-issue-75-gate-5-platform-paths.md',
  'docs/plans/PROJECT-MASTER-PLAN.md',
  'docs/plans/sessions/2026-07-27-issue-75-gate-5-platform-paths.md',
  'docs/reports/issue-75-gate-5-platform-paths.md',
  'docs/workflows/2026-07-27-issue-75-gate-5-platform-paths-runtime.md',
  'err.md',
  'parity/contracts/foundation-platform.yml',
  'parity/features/foundation-platform.yml',
  'parity/features/source-index.yml',
  'parity/README.md',
  'README.md'
)
$sorted = @($approved | Sort-Object)
$payload = ($sorted -join "`n") + "`n"
$hash = [Convert]::ToHexString(
  [Security.Cryptography.SHA256]::HashData([Text.UTF8Encoding]::new($false).GetBytes($payload))
).ToLowerInvariant()
if ($sorted.Count -ne 30) { throw "路径数量漂移：$($sorted.Count)" }
if ($hash -ne 'ae5e0f5143355feee9b280da7c44fdd5cdf759ec2ae71fc69167040bf302cb37') {
  throw "scope_hash 漂移：sha256:$hash"
}
$changed = @(git diff --name-only origin/main...HEAD | Sort-Object)
$outside = @($changed | Where-Object { $_ -notin $sorted })
if ($outside.Count -ne 0) { throw "Issue #75 越界路径：$($outside -join ', ')" }
```

规划阶段将 `$changed` 改为 `git diff --name-only`，并额外要求结果只能是三份规划文件。

## TDD 验证矩阵

| Checkpoint | RED 预期 | GREEN 命令 |
| --- | --- | --- |
| Domain | 新类型无法导入 | `cargo test --locked --offline --ignore-rust-version -p inputcodex-domain --test platform_paths` |
| Application | 请求、端口、用例无法导入 | `cargo test --locked --offline --ignore-rust-version -p inputcodex-application --test platform_paths` |
| Platform Common | `SystemPlatformPaths` 不存在 | `cargo test --locked --offline --ignore-rust-version -p inputcodex-platform --test platform_paths` |
| Windows/macOS | 固定候选、错误和排序断言失败 | `cargo test --locked --offline --ignore-rust-version -p inputcodex-platform` |
| Parity | `unassessed`、错误码和 `process-read` 缺失 | `cargo test --locked --offline --ignore-rust-version -p inputcodex-parity --test catalog_repository` |

RED 失败必须与当前任务预期一致；出现依赖下载、工具链、网络、平台 API、测试竞争或其他根因时，先查 `err.md` 并使用系统化排错，不得把非预期失败当作 TDD 证据。

## 稳定接口与错误

```text
PrivatePath
ApplicationInstallSource
CodexInstallation
PlatformPathsSnapshot
PlatformPathsRequest
PlatformPathsPort::resolve
ResolvePlatformPaths::execute
SystemPlatformPaths
```

```text
PLATFORM_PATHS_UNSUPPORTED
EXPLICIT_CODEX_PATH_INVALID
CODEX_HOME_INVALID
USER_HOME_UNAVAILABLE
INPUTCODEX_STATE_ROOT_UNAVAILABLE
PLATFORM_PATHS_FAILED
```

`Unsupported` 只用于不支持的平台；必需环境、用户目录、显式路径和 `CODEX_HOME` 错误使用 `Unavailable`；Windows API 或不可分类系统读取错误使用 `Internal`。所有错误只携带静态诊断码。

## Windows/macOS 硬合同

- Windows Package Family 固定为 `OpenAI.Codex_2p2nqsd0c76g0`、`OpenAI.CodexBeta_2p2nqsd0c76g0`、`OpenAI.ChatGPT-Desktop_2p2nqsd0c76g0`。
- Windows 只创建一个 `PackageManager`，每个 Family 最多调用一次 `FindPackagesByPackageFamilyName`；版本按 `[Major, Minor, Build, Revision]` 降序，同版本按 Family 列表顺序。
- Windows standalone 固定为 `%LOCALAPPDATA%\OpenAI\Codex\bin`、`%LOCALAPPDATA%\OpenAI\Codex`、`%LOCALAPPDATA%\Programs\OpenAI\Codex`。
- Windows 可执行文件只接受大小写等价的 `Codex.exe` 与 `ChatGPT.exe`。
- macOS 根顺序固定为 `/Applications`、`$HOME/Applications`；应用名顺序固定为 `Codex.app`、`OpenAI Codex.app`、`OpenAI.Codex.app`、`ChatGPT.app`。
- macOS 可执行名称只接受 `Codex` 与 `ChatGPT`；没有已存在安全名称时只派生 `Contents/MacOS/Codex`，不扫描其他文件。
- 显式路径无效时直接返回 `EXPLICIT_CODEX_PATH_INVALID`，两端都不得自动回退。
- 未发现安装时返回快照 `codex_installation=None`，共享根目录和派生文件仍必须有效。

## 最终本地门禁

```powershell
cargo test --locked --offline --ignore-rust-version -p inputcodex-domain -p inputcodex-application -p inputcodex-platform -p inputcodex-parity
cargo clippy --locked --offline --ignore-rust-version -p inputcodex-domain -p inputcodex-application -p inputcodex-platform -p inputcodex-parity --all-targets -- -D warnings
cargo fmt --all -- --check
pwsh -NoProfile -File scripts/ci/Test-CiScripts.ps1
pwsh -NoProfile -File scripts/ci/Verify-ReleaseAuditGate.ps1 -RepositoryRoot .
pwsh -NoProfile -File scripts/ci/Verify-RepositoryPolicy.ps1 -RepositoryRoot .
git diff --check
```

禁止本地运行 Iced、完整 Workspace 重型构建、Release 打包或性能测量。真实 macOS 编译与非 required observation 由公开仓库标准 GitHub-hosted runner 执行。

## Hosted Review/CI 门

- PR 必须是非 Draft，关联 `Closes #75` 并引用 Issue `#74`。
- Final Head 标准 CI 的全部预期 Job 成功；Windows/macOS 真实编译测试成功。
- Performance Baseline 只运行现有非 required observation，不修改预算数值、公式或 required 状态。
- 成功 Artifact 数必须为 `0`；失败诊断只允许现有七天最小 Artifact 合同。
- 所有 Review 对话在合并前解决，并逐条留下根因、处理方式与验证证据。
- Final Head、Review、CI、Artifact、Tree、签名和合并动态证据只写 GitHub Issue/PR；不得为本任务递归创建二次 Closeout。

## 当前证据

```yaml
evidence:
  written_spec_commit: 8a4aef3134974e0c9a25b1e3bd0eccf6f263be1d
  written_spec_tree: e63a4e257dce648288feaa5ac89e86c59757a223
  planning_commit: 61c7c840ee106e30b9b656cac7dfa3499bc7e85b
  authorization_commit: be0f7a93325d1102f998798dabf960b8effaef06
  domain_commit: 67447913fa656f30dd2e6d3c65707acca7c20869
  application_commit: 7e52ec2c4ea2667c22a66e3bae7888eb3cb9e2ce
  platform_commit: 593c447262f1b1aa0ea578bb4a6a0a65037799a6
  scope_expansion_commit: a6e4a28e00c976aa91bca14afb1729ae7e6af194
  parity_commit: be5673c82154fe2777046283158a152d11ead62d
  baseline_tests:
    domain: 1/1
    application: 3/3
    platform: 1/1
    parity: all test targets passed
  scope_count: 30
  scope_hash: sha256:ae5e0f5143355feee9b280da7c44fdd5cdf759ec2ae71fc69167040bf302cb37
  implementation_authorization_ref: https://github.com/nonononull/inputcodex/issues/75#issuecomment-5092021020
  implementation_authorization_local_time: 2026-07-27 21:36:01 +08:00
  implementation_started: true
  implementation_authorization: approved
  domain_application_platform_tdd: passed
  parity_red: 9-passed-4-failed-invalid-initial-parity-status
  parity_green: catalog-repository-13-of-13-and-full-crate-passed
  parity_clippy: passed
  control_plane_sync: ready-to-commit
  final_local_verification: passed-2026-07-27-21-57-22-plus08
  ci_contract: 35/35-passed
  release_audit: current
  repository_policy: passed-zero-violations
  scope_verification: 30/30-passed
  privacy_verification: passed
  pr_created: false
  final_merge_authorization: pending
  agos_session_plan_verifier: passed
  agos_runtime_verifier: bypassed-unregistered-heavy-runtime-contract; project-native validation remains authoritative
```

## 停止门

- 未取得项目所有者对三十路径、`scope_hash`、实施和 PR 操作的明确批准。
- `origin/main`、上游正式 Release、Release commit/tree 或 `release_audit=current` 发生变化。
- `git diff --name-only` 出现批准集合之外的路径，或规划阶段出现三份规划文件之外的路径。
- 需要 UI、Iced、写入、网络、缓存、后台线程、新依赖家族、直接 Win32 FFI 或 `unsafe`。
- Windows/macOS 无法实现等价语义，或新增副作用、无效功能、错误语义争议。
- RED 原因与计划不一致，或 Review/CI/Performance 失败根因未确定。
- 私人绝对路径进入 `Display`、Serde、自动 `Debug`、错误、日志、CI、Review 或报告。
- 请求 force push、删除/改写 `main`、修改 Ruleset/预算/Release/`upstream/`/AGOS。
- 最终 Squash Merge 尚未获得项目所有者针对 Final Head 的单独授权。
