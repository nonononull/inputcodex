# Issue #78：Gate 5 应用概览 Runtime Workflow

```yaml
status: pr-open-ci-correction-1-local-fix-ready
task_id: issue-78
issue_ref: https://github.com/nonononull/inputcodex/issues/78
parity_exception_ref: https://github.com/nonononull/inputcodex/issues/77
branch_ref: codex/issue-78-gate-5-application-overview
baseline_ref: a06a97fd59ce125306a13202c8f1a07656c797a0
baseline_tree: b669aa6610e976542a74f404ff4f87b36864816b
upstream_release: v1.2.43
upstream_commit: 5036ff056b5c629f19356396b17d6eeb70da664c
planning_scope_count: 3
planning_scope_hash: sha256:3f81a54c18c07b6889ad8219b0c1605e4b989f997117141fc2d4baae46ebbeb3
candidate_scope_count: 29
candidate_scope_hash: sha256:b46a940ff7dbf4bbc9bfdb69d04d755468e12409d9618837d8ff310490eb5ae4
scope_approval_ref: https://github.com/nonononull/inputcodex/issues/78#issuecomment-5093844784
scope_approval_status: approved
implementation_authorization: authorized
remote_write_authorization: authorized-normal-push-and-non-draft-pr
final_merge_authorization: pending-separate-gate
```

## 工作流节点

```text
ISSUE_77_DECISION_COMPLETED
  -> ISSUE_78_CREATED
  -> DISCOVERY_COMPLETED
  -> ISOLATED_BRANCH_CREATED
  -> WRITTEN_DESIGN_CREATED
  -> SESSION_PLAN_CREATED
  -> RUNTIME_WORKFLOW_CREATED
  -> PLANNING_SCOPE_VERIFIED
  -> OWNER_CANDIDATE_SCOPE_APPROVAL_REQUIRED
  -> TDD_DOMAIN_RED_GREEN
  -> TDD_APPLICATION_RED_GREEN
  -> TDD_INSTALLATION_ENTRY_RED_GREEN
  -> TDD_PLATFORM_OVERVIEW_RED_GREEN
  -> TDD_PARITY_RED_GREEN
  -> CONTROL_PLANE_CLOSEOUT
  -> LOCAL_LIGHTWEIGHT_VERIFICATION
  -> OWNER_REMOTE_WRITE_APPROVAL
  -> COMMIT_PUSH_NON_DRAFT_PR
  -> FINAL_HEAD_REVIEW_CI
  -> OWNER_SQUASH_MERGE_APPROVAL
  -> SQUASH_MERGE
  -> POST_MERGE_VERIFICATION
```

当前只允许执行到 `PLANNING_SCOPE_VERIFIED`。后续节点均被候选范围批准门锁定。

## 当前规划写入范围

```text
docs/plans/2026-07-27-issue-78-gate-5-application-overview.md
docs/plans/sessions/2026-07-27-issue-78-gate-5-application-overview.md
docs/workflows/2026-07-27-issue-78-gate-5-application-overview-runtime.md
```

规范化：`StringComparer.Ordinal` 升序、UTF-8 无 BOM、LF 分隔、末尾 LF；哈希固定为 `sha256:3f81a54c18c07b6889ad8219b0c1605e4b989f997117141fc2d4baae46ebbeb3`。

## 候选完整实施范围

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

候选哈希固定为 `sha256:b46a940ff7dbf4bbc9bfdb69d04d755468e12409d9618837d8ff310490eb5ae4`，但当前尚未批准。

## 阶段授权

| 阶段 | 当前授权 | 允许操作 |
| --- | --- | --- |
| Discovery | 已完成 | 只读项目、上游快照和 GitHub Issue |
| 隔离分支 | 已批准并完成 | 从最新 `origin/main` 创建 `codex/issue-78-gate-5-application-overview` |
| 规划写入 | 已批准 | 只写三份规划文档并验证 |
| TDD 实现 | 未授权 | 等待 `29` 路径与候选哈希明确批准 |
| 提交/推送/PR | 未授权 | 等待项目所有者明确远端写入批准 |
| Squash Merge | 未授权 | Final Head Review/CI 后单独批准 |

## 双哈希验证

```powershell
$ErrorActionPreference = 'Stop'

$planning = [string[]]@(
  'docs/plans/2026-07-27-issue-78-gate-5-application-overview.md',
  'docs/plans/sessions/2026-07-27-issue-78-gate-5-application-overview.md',
  'docs/workflows/2026-07-27-issue-78-gate-5-application-overview-runtime.md'
)
[Array]::Sort($planning, [StringComparer]::Ordinal)
$planningText = [string]::Join("`n", $planning) + "`n"
$planningHash = [Convert]::ToHexString(
  [Security.Cryptography.SHA256]::HashData(
    [Text.UTF8Encoding]::new($false).GetBytes($planningText)
  )
).ToLowerInvariant()
if ($planning.Count -ne 3) { throw "Issue #78 规划路径数量漂移：$($planning.Count)" }
if ($planningHash -ne '3f81a54c18c07b6889ad8219b0c1605e4b989f997117141fc2d4baae46ebbeb3') {
  throw "Issue #78 planning_scope_hash 漂移：sha256:$planningHash"
}

$candidate = [string[]]@(
  'AGENTS.md',
  'CONTEXT.md',
  'README.md',
  'build.md',
  'crates/inputcodex-application/src/application_overview.rs',
  'crates/inputcodex-application/src/lib.rs',
  'crates/inputcodex-application/tests/application_overview.rs',
  'crates/inputcodex-domain/src/application_overview.rs',
  'crates/inputcodex-domain/src/lib.rs',
  'crates/inputcodex-domain/tests/application_overview.rs',
  'crates/inputcodex-parity/tests/catalog_repository.rs',
  'crates/inputcodex-platform/src/application_overview.rs',
  'crates/inputcodex-platform/src/application_overview/macos.rs',
  'crates/inputcodex-platform/src/application_overview/windows.rs',
  'crates/inputcodex-platform/src/lib.rs',
  'crates/inputcodex-platform/src/platform_paths.rs',
  'crates/inputcodex-platform/src/platform_paths/macos.rs',
  'crates/inputcodex-platform/src/platform_paths/windows.rs',
  'crates/inputcodex-platform/tests/application_overview.rs',
  'docs/plans/2026-07-27-issue-78-gate-5-application-overview.md',
  'docs/plans/PROJECT-MASTER-PLAN.md',
  'docs/plans/sessions/2026-07-27-issue-78-gate-5-application-overview.md',
  'docs/reports/issue-78-gate-5-application-overview.md',
  'docs/workflows/2026-07-27-issue-78-gate-5-application-overview-runtime.md',
  'err.md',
  'parity/README.md',
  'parity/contracts/foundation-platform.yml',
  'parity/features/foundation-platform.yml',
  'parity/features/source-index.yml'
)
[Array]::Sort($candidate, [StringComparer]::Ordinal)
$candidateText = [string]::Join("`n", $candidate) + "`n"
$candidateHash = [Convert]::ToHexString(
  [Security.Cryptography.SHA256]::HashData(
    [Text.UTF8Encoding]::new($false).GetBytes($candidateText)
  )
).ToLowerInvariant()
if ($candidate.Count -ne 29) { throw "Issue #78 候选路径数量漂移：$($candidate.Count)" }
if ($candidateHash -ne 'b46a940ff7dbf4bbc9bfdb69d04d755468e12409d9618837d8ff310490eb5ae4') {
  throw "Issue #78 candidate_scope_hash 漂移：sha256:$candidateHash"
}
```

## 当前范围验证

```powershell
$changed = @(
  git diff --name-only origin/main...HEAD
  git diff --name-only
  git ls-files --others --exclude-standard
) | Where-Object { $_ } | Sort-Object -Unique

$outside = @($changed | Where-Object { $_ -notin $planning })
if ($outside.Count -ne 0) {
  throw "Issue #78 当前规划越界路径：$($outside -join ', ')"
}
if ($changed.Count -ne 3) {
  throw "Issue #78 当前规划应精确覆盖 3 路径，实际为 $($changed.Count)"
}
Write-Output "ISSUE78_PLANNING_SCOPE_GREEN changed=$($changed.Count) planning_scope_hash=sha256:$planningHash candidate_scope_hash=sha256:$candidateHash"
```

## TDD 执行矩阵

| 批次 | RED | GREEN | 回归 |
| --- | --- | --- | --- |
| Domain | 版本校验、安装状态、NotObserved、采集时间、脱敏 | 最小领域值与快照 | `inputcodex-domain` 全测试/Clippy |
| Application | 三类 Ready、Failed、非 Empty、请求脱敏、过期结果 | 请求、端口、用例 | `inputcodex-application` 全测试/Clippy |
| Installation Entry | 概览安装检测不依赖状态目录，候选只执行一次 | 提取 crate 内部入口 | Issue #75 平台路径测试全回放 |
| Platform Overview | Windows/macOS 有界版本元数据、时间、构建版本、错误 | `SystemApplicationOverview` | `inputcodex-platform` 全测试/Clippy |
| Parity | 状态、决策、来源归属、副作用、错误和排除项 | 最小 YAML/README 更新 | `catalog_repository` 全测试/Clippy |

每个 RED 必须真实失败，GREEN 必须只实现当前测试所需最小行为。任何测试失败先查 `err.md`；已有根因优先复用，未知根因按“复现、定位、最小假设、验证、记录”闭环。

## 稳定接口

```text
ApplicationVersion::new(String) -> Result<ApplicationVersion, ApplicationVersionError>
ApplicationVersion::as_str(&self) -> &str

InstallationState::Installed {
  installation: CodexInstallation,
  version: InstalledVersion
}
InstallationState::NotInstalled

InstalledVersion::Known(ApplicationVersion)
InstalledVersion::Unknown(InstalledVersionUnknownReason)

LiveProcessState::NotObserved
CollectedAtUnixMs::new(u64)

ApplicationOverviewRequest::new(Option<PathBuf>)
ApplicationOverviewPort::load(&ApplicationOverviewRequest)
  -> Result<ApplicationOverview, ApplicationError>
LoadApplicationOverview<P>::execute(&ApplicationOverviewRequest)
  -> LoadCompletion<ApplicationOverview>

SystemApplicationOverview: ApplicationOverviewPort
```

成功语义固定为：

```text
Ready(Installed(Known), NotObserved)
Ready(Installed(Unknown), NotObserved)
Ready(NotInstalled, NotObserved)
```

本 feature 不产生 `LoadCompletion::Empty`，也不在成功快照内部嵌套失败状态。

## 稳定错误

```text
APPLICATION_OVERVIEW_UNSUPPORTED
EXPLICIT_CODEX_PATH_INVALID
APPLICATION_OVERVIEW_DISCOVERY_FAILED
APPLICATION_OVERVIEW_TIME_UNAVAILABLE
APPLICATION_OVERVIEW_BUILD_VERSION_INVALID
```

版本元数据问题只使用：

```text
InstalledVersionUnknownReason::MetadataMissing
InstalledVersionUnknownReason::MetadataUnreadable
InstalledVersionUnknownReason::MetadataInvalid
```

错误与 Debug 均不得包含绝对路径、用户名、环境变量值或文件内容。

## Windows/macOS 硬合同

### 共通

- 安装发现每请求只执行一次。
- 显式路径优先且非法时不回退。
- 不读取 `CODEX_HOME`、`inputcodex` 状态根、设置、历史状态或日志路径。
- 不读取 `latest-status.json`，实时状态固定 `NotObserved`。
- 不写文件、不联网、不缓存、不启动线程、不调用 shell、不使用 `unsafe`。

### Windows

- 固定三个包家族和三个 standalone 根，保持 Issue `#75` 顺序。
- 版本只检查元数据根目录名和一个 `<metadata_root>/version`。
- 版本文件最大 `256` 字节。
- 禁止 `canonicalize`、注册表、PATH 和递归扫描。

### macOS

- 固定两个应用根、每根四个名称，保持 Issue `#75` 顺序。
- 版本只读取一个 `Contents/Info.plist`。
- 最大 `65536` 字节，只支持 UTF-8 XML 文本。
- `CFBundleShortVersionString` 优先，`CFBundleVersion` 回退。
- 禁止 `plutil`、`defaults`、外部 XML 库和 Objective-C runtime。

## 实施后本地轻量门禁

候选范围获批并完成实现后运行：

```powershell
$ErrorActionPreference = 'Stop'
Get-Date -Format 'yyyy-MM-dd HH:mm:ss.fff zzz'

cargo test --locked --offline --ignore-rust-version -p inputcodex-domain -p inputcodex-application -p inputcodex-platform -p inputcodex-parity
cargo clippy --locked --offline --ignore-rust-version -p inputcodex-domain -p inputcodex-application -p inputcodex-platform -p inputcodex-parity --all-targets -- -D warnings
cargo fmt --all -- --check

pwsh -NoProfile -File scripts/ci/Test-CiScripts.ps1
pwsh -NoProfile -File scripts/ci/Verify-ReleaseAuditGate.ps1 -RepositoryRoot .
pwsh -NoProfile -File scripts/ci/Verify-RepositoryPolicy.ps1 -RepositoryRoot .

$changed = @(
  git diff --name-only origin/main...HEAD
  git diff --name-only
  git ls-files --others --exclude-standard
) | Where-Object { $_ } | Sort-Object -Unique
$outside = @($changed | Where-Object { $_ -notin $candidate })
if ($outside.Count -ne 0) { throw "Issue #78 越界路径：$($outside -join ', ')" }

$ownerName = [Environment]::UserName
if (-not [string]::IsNullOrWhiteSpace($ownerName)) {
  $privateLeaks = @(rg -n --fixed-strings $ownerName crates parity docs/reports/issue-78-gate-5-application-overview.md 2>$null)
  if ($LASTEXITCODE -eq 0 -and $privateLeaks.Count -ne 0) {
    throw "Issue #78 产品或证据表面泄露本机用户标识：$($privateLeaks -join '; ')"
  }
  if ($LASTEXITCODE -notin 0, 1) { throw 'Issue #78 隐私扫描执行失败。' }
}

$forbiddenProduct = @(
  rg -n 'std::process::Command|tokio::spawn|thread::spawn|reqwest|unsafe\s*\{' crates/inputcodex-domain crates/inputcodex-application crates/inputcodex-platform 2>$null
)
if ($LASTEXITCODE -eq 0 -and $forbiddenProduct.Count -ne 0) {
  throw "Issue #78 命中禁止运行能力：$($forbiddenProduct -join '; ')"
}
if ($LASTEXITCODE -notin 0, 1) { throw 'Issue #78 禁止能力扫描执行失败。' }

git diff --check
Write-Output "ISSUE78_LOCAL_VERIFY_OK candidate_scope_hash=sha256:$candidateHash changed=$($changed.Count)"
```

预期：四 crate 测试/Clippy 全绿，格式、CI 合同、Release Audit、Repository Policy、范围、隐私、禁止能力和空白检查通过。实现允许只覆盖批准集合的必要子集，不要求为了凑数修改全部 `29` 路径。

## Hosted Review/CI 门

1. 普通推送后创建非 Draft PR，并关联 Issue `#78` 与 Issue `#77`。
2. Final Head 必须完成 changed-surface Review。
3. 每条 Review 对话记录根因、处理方式和验证证据后才能解决。
4. 标准 CI 的预期 Job、Windows/macOS 编译测试和非 required Performance observation 必须符合仓库当前合同。
5. 成功 Run 的 Artifact 数保持 `0`；失败只允许既有合同定义的最小七天诊断。
6. 外部 GitHub Actions 事故不得通过代码绕过；应保存服务端事实并重跑同一合法 Head。
7. 最终 Squash Merge 只能在项目所有者单独授权后执行。

## 当前证据

```yaml
current_evidence:
  local_time_source: Windows Get-Date
  planning_authorization_observed_at: 2026-07-27 23:34:39 +08:00
  branch: codex/issue-78-gate-5-application-overview
  head: a06a97fd59ce125306a13202c8f1a07656c797a0
  tree: b669aa6610e976542a74f404ff4f87b36864816b
  origin_main: a06a97fd59ce125306a13202c8f1a07656c797a0
  upstream_release: v1.2.43
  upstream_commit: 5036ff056b5c629f19356396b17d6eeb70da664c
  release_audit: current
  requires_reaudit: false
  issue_77: CLOSED/COMPLETED
  issue_78: OPEN
  codegraph: absent-not-initialized
  gitnexus: no-indexed-repository
  agos: optional-bypassed-no-external-mutation
  implementation_changes: domain-application-platform-parity-and-control-plane-complete
  domain_checkpoint: f9db364b0fe21c105af427c878d00063e3f76886
  application_checkpoint: a1c09e59b7552a5f6dac9ba79e0025301cde39f6
  platform_checkpoint: 26751c5a72009008652ac48ef6d0af7a7753c332
  parity_checkpoint: a4012ddbf7ab929d136e0ab5bc7ac1ae61284f7d
  control_checkpoint: 27e8599edff2f0ddeacc24d1d188cfbeb6d85c68
  local_verification: passed
  changed_paths: 29
  ci_contract: 35/35
  repository_policy_violations: 0
  commit_push_pr: completed-pr-79
  pr_ref: https://github.com/nonononull/inputcodex/pull/79
  hosted_ci: correction-1-linux-quality-fix-pending-push
```

Issue `#77` 评论插值错误已经在原评论修正；稳定证据为 <https://github.com/nonononull/inputcodex/issues/77#issuecomment-5093148603>。

## 当前实施验证

规划文档验证仍作为实施门禁的一部分保留；完整 Rust、治理、范围、隐私和禁止能力命令使用本文件“实施后本地轻量门禁”区块：

```powershell
$ErrorActionPreference = 'Stop'

$required = @(
  'docs/plans/2026-07-27-issue-78-gate-5-application-overview.md',
  'docs/plans/sessions/2026-07-27-issue-78-gate-5-application-overview.md',
  'docs/workflows/2026-07-27-issue-78-gate-5-application-overview-runtime.md'
)
foreach ($path in $required) {
  if (-not (Test-Path -LiteralPath $path -PathType Leaf)) { throw "缺失规划文件：$path" }
  $text = Get-Content -LiteralPath $path -Raw -Encoding UTF8
  $placeholderPattern = ('T' + 'BD') + '|' + ('T' + 'ODO') + '|' + ('ISSUE' + '78_.*_CONTINUE')
  if ($text -match $placeholderPattern) { throw "规划文件包含占位符：$path" }
  if ($text -match '[\x00-\x08\x0B\x0C\x0E-\x1F]') { throw "规划文件包含非法控制字符：$path" }
}

git diff --check
Write-Output 'ISSUE78_PLANNING_DOCUMENTS_GREEN files=3'
```

## 停止门

任一条件命中即停止：

- 候选 `29` 路径或哈希批准证据失效。
- 当前变更超出三份规划文档，或实施后超出批准候选集合。
- 基线、上游 Release、Release Audit 或候选哈希漂移。
- 需要历史状态读取、进程枚举、PID/debug port、写入、网络、缓存、后台线程、shell、`unsafe`、UI 或新依赖。
- Windows/macOS 无法保持同一领域和应用合同。
- 需要修改 Cargo、Workflow、Ruleset、预算、Release、`upstream/` 或 AGOS。
- 测试、Review 或 CI 失败根因未确定并解决。
- 请求 force push、删除/改写 `main`、Merge Commit 或 Rebase Merge。

## 下一控制门

完成最终本地门禁后形成 Git checkpoint，普通推送并创建非 Draft PR；随后核验 Review、标准 CI、双平台编译、非 required Performance observation 和 Artifact `0`。全部根因与对话闭环后停止，等待项目所有者单独授权 Squash Merge。
