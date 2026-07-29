# inputcodex 构建与验证说明

## 文档职责

本文件只维护可重复构建、测试和验证命令，不再承担活动任务面板、逐 PR 合并流水账或
主干状态真源职责。稳定产品 Gate 与下一合法阶段见
`docs/plans/PROJECT-MASTER-PLAN.md`；动态 Head、Review、CI、授权和合并后证据见对应的
GitHub Issue、PR 与 Actions。

Gate 3 七成员 Rust Workspace、Gate 4 上游审计/功能目录/性能基线，以及 Gate 5 的平台路径、
应用概览、版本与启动意图、运行时环境冲突、Relay 环境、设置、诊断日志和 Relay 状态只读观察
能力已经建立。
该说明只用于选择正确命令，不能替代任务计划和 GitHub 新鲜证据。

## 文档变更轻量验证

纯文档任务不运行本地 Workspace 全量编译或 Iced 桌面构建。至少执行：

```powershell
$ErrorActionPreference = 'Stop'
Get-Date -Format 'yyyy-MM-dd HH:mm:ss.fff zzz'
pwsh -NoProfile -File scripts/ci/Test-CiScripts.ps1
pwsh -NoProfile -File scripts/ci/Verify-RepositoryPolicy.ps1 -RepositoryRoot .
git diff --check
```

路径范围、Markdown 链接和任务特有内容守卫由对应 Session Plan 与 Runtime Workflow 定义。

仓库当前有 `upstream/CodexPlusPlus/` 审计快照、七成员纯 Rust Workspace 和首版无缓存三平台 `CI` Workflow。本文件当前提供二十九个检查点：

1. 上游快照、manifest、许可证与提交 blob/mode 验证。
2. PR `#11` Squash Merge、Issue `#9` 关闭和 `main` tree 验证。
3. Issue `#12` / PR `#13` closeout 合并证据验证。
4. Issue `#14` 上游监控合同、Workflow、允许路径与合并后幂等验证。
5. Issue `#17` Gate 3 规划文档、允许路径和禁止产品表面验证。
6. Issue `#19` Gate 3 实现控制面、批准引用、范围哈希和 RED 前置门禁验证。
7. Issue `#19` 治理 RED 合同的 AST、非零退出码、稳定标记和实现缺失根因验证。
8. Issue `#19` 路径分类、许可证、仓库政策与冷构建日志脚本的 `30/30` GREEN 合同验证。
9. Issue `#19` 七成员 Workspace、锁文件、轻量 crate 测试和 Iced 边界验证。
10. Issue `#19` 首版 `CI` Workflow 的 YAML、Job、权限、Action 固定 SHA、无 Cache 与 Artifact 白名单静态验证。
11. Issue `#19` 五类真实失败语义、三平台各 `3/3` 次无缓存成功样本、最终修复全绿运行与冷构建基线报告验证。
12. Issue `#22` Gate 3 merge/tree/Issue/CI 证据、14 条 closeout 路径和受保护表面验证。
13. Issue `#24` Gate 4 规划批准引用、9 条最大路径、两阶段拆分和执行锁定验证。
14. Issue `#26` 功能目录执行控制面、8 条当前路径、36 条最大范围和新 scope hash 验证。
15. Issue `#35` Release 审计解耦、stale PR 路径门禁、`required` 汇总依赖和定向 Rust 验证。
16. Issue `#32` 隔离性能测量合同、原始样本结构、专用 Workflow 和预算/优化隔离验证。
17. Issue `#50` 性能预算 ADR、同平台可比队列、阶段门禁、九路径范围与长期状态验证。
18. Issue `#52` 性能预算 Discovery 合并后稳定状态、八路径范围、反递归边界与主干证据验证。
19. Issue `#61` 性能预算数值合并后稳定状态、八路径范围、反递归边界、预算复算与双套主干证据验证。
20. Issue `#63` 十三路径、只读预算观察器、自动 observation、非阻断分类、Artifact 边界与 Gate 5 解锁前置验证。
21. Issue `#75` 三十路径、平台路径分层合同、双平台适配器、隐私边界、Parity 状态和最终本地轻量门禁验证。
22. Issue `#78` 二十九路径、应用概览只读事实、版本未知、`NotObserved`、有界元数据读取、Parity 重分类和最终本地轻量门禁验证。
23. Issue `#81` 二十三路径、编译版本复用、启动意图纯解析、非法显式值、旧变量禁止、Parity 状态和最终本地轻量门禁验证。
24. Issue `#86` 二十四路径、当前进程环境只读观察、明确来源覆盖、隐私边界、Parity 单入口修订和最终本地轻量门禁验证。
25. Issue `#89` 三十路径、Relay 环境只读聚合、Windows 注册表覆盖、`.env` 元数据、Clash 有界读取、Parity 分拆和最终本地轻量门禁验证。
26. Issue `#92` 二十七路径、设置文件有界只读观察、缺失与空对象分离、六个稳定错误、Parity 单入口分拆和最终本地轻量门禁验证。
27. Issue `#95` 二十四路径、诊断日志尾部有界观察、损坏记录分类、Parity 单入口分拆和最终本地轻量门禁验证。
28. Issue `#98` 二十七路径、Relay 双文档有界观察、凭据存在事实、配置完整性、Parity 单入口分拆和最终本地轻量门禁验证。
29. Issue `#101` 二十四路径、上下文能力固定文件有界观察、条目最小投影、严格 TOML 失败语义、Parity 单入口分拆和最终本地轻量门禁验证。

当前禁止：

- 在没有新的独立 upstream-sync Issue/PR 与项目所有者批准时修改 `upstream/` 或 `source-lock.json`。
- 把三平台各 `3/3` 次最低冷构建基线解释为已经完成 Cache、P95、七天观测或最终性能预算。
- 在 Issue `#26` control-plane checkpoint 中创建实际 `parity/` 数据或 `benchmarks/`，或修改 Cargo、Rust、测试、CI、`upstream/`、Ruleset 或 AGOS。
- 创建 Release Workflow、安装包、签名、更新资产、临时 UI 或 WebView。
- 修改 Ruleset、required checks 或仓库级合并开关。
- 修改或优化外部 AGOS。
- 在 Issue `#50` 中填写预算数值、运行新 hosted 测量、修改性能实现/Workflow、实施优化或解锁 Gate 5。
- 在 Issue `#52` 中修改代码、Cargo、`benchmarks/`、Workflow、Ruleset、Release、AGOS、预算数值、性能优化或 Gate 5 产品功能。
- 在 Issue `#61` 中修改预算数值、公式、量子、队列、样本、`benchmarks/`、代码、Cargo、Workflow、Ruleset、Release、AGOS、性能优化或 Gate 5 产品功能。
- 在 Issue `#63` 中修改预算 JSON、历史 Evidence、预算数值/公式、Ruleset、required checks、产品、Gate 5、`upstream/` 或 AGOS，或把阈值分类改成阻断退出码。
- 在 Issue `#75` 中创建 UI、Iced 视图、目录/文件写入、网络、缓存、后台线程、第二个产品 feature、新依赖家族、直接 Win32 FFI、`unsafe`、预算、Release、Ruleset、`upstream/` 或 AGOS 改动。
- 在 Issue `#78` 中读取历史启动状态、枚举进程、观察 PID/debug port、写文件、联网、缓存、启动线程、调用 shell、引入 UI/Iced、新依赖、第三个产品 feature、预算、Release、Ruleset、`upstream/` 或 AGOS 改动。
- 在 Issue `#81` 中兼容旧启动变量、打开 UI、联网、检查/下载/执行更新、读写文件、缓存、启动线程、调用 shell、引入 Iced/Tauri/WebView、新依赖、第四个产品 feature、预算、Release、Ruleset、`upstream/` 或 AGOS 改动。
- 在 Issue `#89` 中测试网络、返回代理值或路径、读取 `.env` 内容、修改环境/文件/注册表、调用子进程、启动线程/Watcher、打开 UI、注入、迁移 `core-module:proxy`、修改 Workflow/Ruleset、Release、`upstream/` 或 AGOS。
- 在 Issue `#92` 中公开任意路径读取、返回设置字段或内容、写文件、联网、调用子进程、启动线程/Watcher、打开 UI、使用 `unsafe`、迁移保存/重置/底层设置管理、修改 Workflow/Ruleset、Release、`upstream/` 或 AGOS。
- 在 Issue `#95` 中接受任意路径/长度/过滤器、读取完整日志、返回正文/字段/事件/detail/PID/时间戳/实际路径/用户名/机器名/凭据、写入或清理日志、复制诊断报告、联网、调用子进程、启动线程/Watcher、打开 UI、注入、使用 `unsafe`、迁移其余诊断总功能、修改 Cargo/Workflow/Ruleset/Release/`upstream/` 或 AGOS。
- 在 Issue `#98` 中公开任意路径、返回账号/Token/Provider/URL/字段/内容/认证来源/实际路径、写文件、修改环境、联网、调用子进程、启动线程/Watcher、打开 UI、注入、使用 `unsafe`、迁移完整 Relay 文件读取/保存/切换/回填、修改 Workflow/Ruleset/Release/`upstream/` 或 AGOS。
- 在 Issue `#101` 中接受任意路径或配置正文、返回完整 TOML/摘要/命令/参数/环境变量/Header/URL/Token/账号/实际路径、写文件、联网、调用子进程、启动线程/Watcher、打开 UI、注入、使用 `unsafe`、新增依赖、迁移上下文增加/删除/同步/提取/设置正文解析、修改 Workflow/Ruleset/Release/`upstream/` 或 AGOS。

## Issue #104 本地会话目录只读观察本地轻量验证

在仓库根目录、分支 `codex/issue-104-gate-5-local-session-directory-observation` 执行。Git 时间只使用系统默认本机时间；不得设置 `GIT_AUTHOR_DATE` 或 `GIT_COMMITTER_DATE`。Issue `#104` 已取得项目所有者对 A1、二十九路径和 `candidate_scope_hash` 的集中授权；完整 Workspace 与 Windows/macOS/Linux 编译继续交给标准 GitHub-hosted CI。

```powershell
$ErrorActionPreference = 'Stop'

function Assert-NativeSuccess {
  param([Parameter(Mandatory)][string]$Label)
  if ($LASTEXITCODE -ne 0) { throw "$Label 失败：$LASTEXITCODE" }
}

Get-Date -Format 'yyyy-MM-dd HH:mm:ss.fff zzz'

cargo test --locked --offline --all-targets `
  -p inputcodex-domain `
  -p inputcodex-application `
  -p inputcodex-platform `
  -p inputcodex-parity
Assert-NativeSuccess 'Issue #104 定向测试'

cargo clippy --locked --offline --all-targets `
  -p inputcodex-domain `
  -p inputcodex-application `
  -p inputcodex-platform `
  -p inputcodex-parity -- -D warnings
Assert-NativeSuccess 'Issue #104 定向 Clippy'

cargo fmt --all -- --check
Assert-NativeSuccess 'Issue #104 rustfmt'

pwsh -NoProfile -File scripts/ci/Test-CiScripts.ps1
Assert-NativeSuccess 'Issue #104 CI 合同'

pwsh -NoProfile -File scripts/ci/Verify-RepositoryPolicy.ps1 -RepositoryRoot .
Assert-NativeSuccess 'Issue #104 仓库政策'

pwsh -NoProfile -File scripts/ci/Verify-ReleaseAuditGate.ps1 -RepositoryRoot .
Assert-NativeSuccess 'Issue #104 Release Audit'

cargo metadata --locked --offline --no-deps --format-version 1 | Out-Null
Assert-NativeSuccess 'Issue #104 Cargo metadata'

$tree = @(cargo tree --locked --offline -p inputcodex-platform)
Assert-NativeSuccess 'Issue #104 依赖树'
if (-not ($tree -match '^rusqlite v0\.40\.1$')) {
  throw 'Issue #104 必须锁定 rusqlite v0.40.1。'
}
if ($tree -match 'tokio|async-std|sqlx|diesel|deadpool') {
  throw 'Issue #104 禁止引入异步 runtime、ORM 或连接池依赖。'
}

$approved = @(
  'AGENTS.md',
  'build.md',
  'Cargo.lock',
  'Cargo.toml',
  'CONTEXT.md',
  'crates/inputcodex-application/src/lib.rs',
  'crates/inputcodex-application/src/local_session_directory_observation.rs',
  'crates/inputcodex-application/tests/local_session_directory_observation.rs',
  'crates/inputcodex-domain/src/lib.rs',
  'crates/inputcodex-domain/src/local_session_directory_observation.rs',
  'crates/inputcodex-domain/tests/local_session_directory_observation.rs',
  'crates/inputcodex-parity/tests/catalog_repository.rs',
  'crates/inputcodex-platform/Cargo.toml',
  'crates/inputcodex-platform/src/lib.rs',
  'crates/inputcodex-platform/src/local_session_directory_observation.rs',
  'crates/inputcodex-platform/tests/local_session_directory_observation.rs',
  'docs/plans/2026-07-29-issue-104-gate-5-local-session-directory-observation.md',
  'docs/plans/PROJECT-MASTER-PLAN.md',
  'docs/plans/sessions/2026-07-29-issue-104-gate-5-local-session-directory-observation.md',
  'docs/reports/issue-104-gate-5-local-session-directory-observation.md',
  'docs/workflows/2026-07-29-issue-104-gate-5-local-session-directory-observation-runtime.md',
  'err.md',
  'parity/README.md',
  'parity/contracts/session-data.yml',
  'parity/features/session-data.yml',
  'parity/features/source-index.yml',
  'parity/fixtures/feature.session-data.local-session-directory-observation/baseline.yml',
  'parity/fixtures/feature.session-data.local-session-directory-observation/manifest.yml',
  'README.md'
) | Sort-Object
$scopePayload = ($approved -join "`n") + "`n"
$scopeHash = [Convert]::ToHexString(
  [Security.Cryptography.SHA256]::HashData(
    [Text.UTF8Encoding]::new($false).GetBytes($scopePayload)
  )
).ToLowerInvariant()
if ($approved.Count -ne 29) { throw "Issue #104 路径数量漂移：$($approved.Count)" }
if ($scopeHash -ne '47dcb2c181daa61a8df073e7f3ada069bf8e3d9b95df0c57f7709bcb6cde211d') {
  throw "Issue #104 scope_hash 漂移：sha256:$scopeHash"
}

$baseline = 'origin/main'
$actual = @(
  git -c core.quotePath=false diff --name-only "$baseline...HEAD"
  git -c core.quotePath=false diff --name-only
  git -c core.quotePath=false ls-files --others --exclude-standard
) | Where-Object { $_ } | Sort-Object -Unique
$outside = @($actual | Where-Object { $_ -notin $approved })
if ($outside.Count -ne 0) { throw "Issue #104 越界路径：$($outside -join ', ')" }

$actualPayload = ($actual -join "`n") + "`n"
$actualHash = [Convert]::ToHexString(
  [Security.Cryptography.SHA256]::HashData(
    [Text.UTF8Encoding]::new($false).GetBytes($actualPayload)
  )
).ToLowerInvariant()
$errChanged = $actual -contains 'err.md'
if ($errChanged) {
  if ($actual.Count -ne 29 -or $actualHash -ne $scopeHash) {
    throw "Issue #104 含 err.md 时实际范围漂移：count=$($actual.Count) sha256:$actualHash"
  }
} else {
  $expectedActual = @($approved | Where-Object { $_ -ne 'err.md' })
  if ($actual.Count -ne 28 -or (Compare-Object $expectedActual $actual).Count -ne 0) {
    throw "Issue #104 无新根因时实际路径必须精确排除 err.md：$($actual -join ', ')"
  }
  if ($actualHash -ne 'acee55e9539631f6eca4fb557d27999c7815d20ae15a4b4ea932db243079cb8c') {
    throw "Issue #104 常态实际范围哈希漂移：sha256:$actualHash"
  }
}

$protected = @(
  git -c core.quotePath=false diff --name-only "$baseline...HEAD" -- `
    .github apps benchmarks scripts upstream `
    crates/inputcodex-infrastructure crates/inputcodex-presentation
  git -c core.quotePath=false diff --name-only -- `
    .github apps benchmarks scripts upstream `
    crates/inputcodex-infrastructure crates/inputcodex-presentation
) | Where-Object { $_ } | Sort-Object -Unique
if ($protected.Count -ne 0) { throw "Issue #104 修改受保护路径：$($protected -join ', ')" }

$rootCargo = Get-Content -LiteralPath Cargo.toml -Raw
if ($rootCargo -notmatch '(?m)^rusqlite = \{ version = "=0\.40\.1", default-features = false, features = \["bundled", "hooks"\] \}$') {
  throw 'Workspace 必须精确锁定 rusqlite 0.40.1 bundled/hooks。'
}
$platformCargo = Get-Content -LiteralPath crates/inputcodex-platform/Cargo.toml -Raw
if ($platformCargo -notmatch '(?m)^rusqlite\.workspace = true$') {
  throw 'Platform crate 必须通过 workspace 使用 rusqlite。'
}

$productionPaths = @(
  'crates/inputcodex-domain/src/local_session_directory_observation.rs',
  'crates/inputcodex-application/src/local_session_directory_observation.rs',
  'crates/inputcodex-platform/src/local_session_directory_observation.rs'
)
$production = ($productionPaths | ForEach-Object { Get-Content -LiteralPath $_ -Raw }) -join "`n"
foreach ($required in @(
  'LOCAL_SESSION_DIRECTORY_INVALID_PAGINATION',
  'LOCAL_SESSION_DIRECTORY_INVALID_SQLITE_HOME',
  'LOCAL_SESSION_DIRECTORY_TOO_MANY_DATABASES',
  'LOCAL_SESSION_DIRECTORY_UNSUPPORTED_SCHEMA',
  'LOCAL_SESSION_DIRECTORY_UNAVAILABLE',
  'LOCAL_SESSION_DIRECTORY_TIMEOUT',
  'LOCAL_SESSION_DIRECTORY_CANCELLED',
  'SQLITE_OPEN_READ_ONLY',
  'query_only',
  'progress_handler'
)) {
  if (-not $production.Contains($required)) { throw "Issue #104 缺少稳定合同：$required" }
}
foreach ($forbidden in @(
  'Connection::open(',
  'execute_batch(',
  '.execute(',
  '.transaction(',
  'CREATE TABLE',
  'CREATE INDEX',
  'ALTER TABLE',
  'DELETE FROM',
  'INSERT INTO',
  'UPDATE ',
  'TcpStream',
  'UdpSocket',
  'Command::new',
  'iced',
  'unsafe {'
)) {
  if ($production.Contains($forbidden)) { throw "Issue #104 禁止能力命中：$forbidden" }
}

$ownerName = [Environment]::UserName
$addedDiff = @(git -c core.quotePath=false diff --unified=0 "$baseline...HEAD" -- $productionPaths)
Assert-NativeSuccess 'Issue #104 新增行隐私扫描准备'
$addedLines = @($addedDiff | Where-Object { $_ -cmatch '^\+(?!\+\+)' })
if (-not [string]::IsNullOrWhiteSpace($ownerName)) {
  $privateLeaks = @($addedLines | Select-String -SimpleMatch -CaseSensitive $ownerName)
  if ($privateLeaks.Count -ne 0) { throw "Issue #104 泄露本机用户标识。" }
}
$absolutePathLeaks = @($addedLines | Select-String -Pattern '(?i)[A-Z]:\\Users\\')
if ($absolutePathLeaks.Count -ne 0) { throw 'Issue #104 泄露 Windows 用户绝对路径。' }

$fixtureText = Get-Content -LiteralPath `
  parity/fixtures/feature.session-data.local-session-directory-observation/baseline.yml -Raw
if ($fixtureText -match '(?i)[A-Z]:\\Users\\|/Users/|/home/|token|api[_-]?key|bearer') {
  throw 'Issue #104 合成 fixture 包含私人路径或凭据标记。'
}

git diff --check
Assert-NativeSuccess 'Issue #104 Git 空白检查'
```

预期：四 crate tests/Clippy、`rustfmt`、CI 合同、仓库政策、Release Audit、Cargo metadata、依赖树、范围哈希、只读 SQLite、安全和隐私门禁均通过。完整 Workspace、三平台与 Performance Baseline 继续由 GitHub-hosted Actions 验收。

## Issue #101 上下文能力只读目录观察本地轻量验证

在仓库根目录、分支 `codex/issue-101-gate-5-context-entry-observation` 执行。Git 时间只使用系统
默认本机时间；不得设置 `GIT_AUTHOR_DATE` 或 `GIT_COMMITTER_DATE`。Issue `#101` 已取得项目所有者
对二十四路径和 `candidate_scope_hash` 的批准；以下命令用于实现后的本地轻量验证。

```powershell
$ErrorActionPreference = 'Stop'

function Assert-NativeSuccess {
  param([Parameter(Mandatory)][string]$Label)
  if ($LASTEXITCODE -ne 0) { throw "$Label 失败：$LASTEXITCODE" }
}

Get-Date -Format 'yyyy-MM-dd HH:mm:ss.fff zzz'

cargo test --locked --offline --all-targets `
  -p inputcodex-domain `
  -p inputcodex-application `
  -p inputcodex-platform `
  -p inputcodex-parity
Assert-NativeSuccess 'Issue #101 定向测试'

cargo clippy --locked --offline --all-targets `
  -p inputcodex-domain `
  -p inputcodex-application `
  -p inputcodex-platform `
  -p inputcodex-parity -- -D warnings
Assert-NativeSuccess 'Issue #101 定向 Clippy'

cargo fmt --all -- --check
Assert-NativeSuccess 'Issue #101 rustfmt'

pwsh -NoProfile -File scripts/ci/Test-CiScripts.ps1
Assert-NativeSuccess 'Issue #101 CI 合同'

pwsh -NoProfile -File scripts/ci/Verify-RepositoryPolicy.ps1 -RepositoryRoot .
Assert-NativeSuccess 'Issue #101 仓库政策'

pwsh -NoProfile -File scripts/ci/Verify-ReleaseAuditGate.ps1 -RepositoryRoot .
Assert-NativeSuccess 'Issue #101 Release Audit'

cargo metadata --locked --offline --no-deps --format-version 1 | Out-Null
Assert-NativeSuccess 'Issue #101 Cargo metadata'

$approved = @(
  'AGENTS.md',
  'build.md',
  'CONTEXT.md',
  'crates/inputcodex-application/src/context_entry_observation.rs',
  'crates/inputcodex-application/src/lib.rs',
  'crates/inputcodex-application/tests/context_entry_observation.rs',
  'crates/inputcodex-domain/src/context_entry_observation.rs',
  'crates/inputcodex-domain/src/lib.rs',
  'crates/inputcodex-domain/tests/context_entry_observation.rs',
  'crates/inputcodex-parity/tests/catalog_repository.rs',
  'crates/inputcodex-platform/src/context_entry_observation.rs',
  'crates/inputcodex-platform/src/lib.rs',
  'crates/inputcodex-platform/tests/context_entry_observation.rs',
  'docs/plans/2026-07-29-issue-101-gate-5-context-entry-observation.md',
  'docs/plans/PROJECT-MASTER-PLAN.md',
  'docs/plans/sessions/2026-07-29-issue-101-gate-5-context-entry-observation.md',
  'docs/reports/issue-101-gate-5-context-entry-observation.md',
  'docs/workflows/2026-07-29-issue-101-gate-5-context-entry-observation-runtime.md',
  'err.md',
  'parity/contracts/provider-network.yml',
  'parity/features/provider-network.yml',
  'parity/features/source-index.yml',
  'parity/README.md',
  'README.md'
) | Sort-Object
$scopePayload = ($approved -join "`n") + "`n"
$scopeHash = [Convert]::ToHexString(
  [Security.Cryptography.SHA256]::HashData(
    [Text.UTF8Encoding]::new($false).GetBytes($scopePayload)
  )
).ToLowerInvariant()
if ($approved.Count -ne 24) { throw "Issue #101 路径数量漂移：$($approved.Count)" }
if ($scopeHash -ne '5b96235eb1fa7832e5710f7343917a5c2512bc50a46198ed584323366dd34372') {
  throw "Issue #101 scope_hash 漂移：sha256:$scopeHash"
}

$baseline = 'origin/main'
$actual = @(
  git -c core.quotePath=false diff --name-only "$baseline...HEAD"
  git -c core.quotePath=false diff --name-only
  git -c core.quotePath=false ls-files --others --exclude-standard
) | Where-Object { $_ } | Sort-Object -Unique
$outside = @($actual | Where-Object { $_ -notin $approved })
if ($outside.Count -ne 0) { throw "Issue #101 越界路径：$($outside -join ', ')" }
$actualPayload = ($actual -join "`n") + "`n"
$actualHash = [Convert]::ToHexString(
  [Security.Cryptography.SHA256]::HashData(
    [Text.UTF8Encoding]::new($false).GetBytes($actualPayload)
  )
).ToLowerInvariant()
$errChanged = $actual -contains 'err.md'
if ($errChanged) {
  if ($actual.Count -ne 24 -or $actualHash -ne $scopeHash) {
    throw "Issue #101 含 err.md 时实际范围漂移：count=$($actual.Count) sha256:$actualHash"
  }
} else {
  $expectedActual = @($approved | Where-Object { $_ -ne 'err.md' })
  if ($actual.Count -ne 23 -or (Compare-Object $expectedActual $actual).Count -ne 0) {
    throw "Issue #101 无新根因时实际路径必须精确排除 err.md：$($actual -join ', ')"
  }
  if ($actualHash -ne '08b223934a07a66d91e5cf2e1b340a243ea460d6c4edc266f58d30101c478d47') {
    throw "Issue #101 常态实际范围哈希漂移：sha256:$actualHash"
  }
}

$protected = @(
  git -c core.quotePath=false diff --name-only "$baseline...HEAD" -- `
    Cargo.toml Cargo.lock .github apps upstream scripts benchmarks
  git -c core.quotePath=false diff --name-only -- `
    Cargo.toml Cargo.lock .github apps upstream scripts benchmarks
) | Where-Object { $_ } | Sort-Object -Unique
if ($protected.Count -ne 0) { throw "Issue #101 修改受保护路径：$($protected -join ', ')" }

$dependencyDiff = @(git -c core.quotePath=false diff "$baseline...HEAD" -- Cargo.toml Cargo.lock crates/*/Cargo.toml)
foreach ($forbiddenDependency in @('rusqlite', 'reqwest', 'sysinfo', 'tokio', 'hyper')) {
  if ($dependencyDiff -match [regex]::Escape($forbiddenDependency)) {
    throw "Issue #101 禁止新增依赖：$forbiddenDependency"
  }
}

function Get-YamlListItemBlock {
  param([Parameter(Mandatory)][string]$Text, [Parameter(Mandatory)][string]$Id)
  $match = [regex]::Match($Text, "(?ms)^  - id: $([regex]::Escape($Id))\r?\n.*?(?=^  - id: |\z)")
  if (-not $match.Success) { throw "Issue #101 缺少目录条目：$Id" }
  $match.Value
}

$featureText = Get-Content -Raw -LiteralPath parity/features/provider-network.yml
$observation = Get-YamlListItemBlock -Text $featureText -Id 'feature.provider-network.context-entry-observation'
if ($observation -notmatch 'status: implemented' -or
    $observation -notmatch 'tauri-command:read_live_context_entries') {
  throw 'Issue #101 上下文目录观察 feature 漂移。'
}
$umbrella = Get-YamlListItemBlock -Text $featureText -Id 'feature.provider-network.context-entry-management'
if ($umbrella -notmatch 'status: unassessed' -or
    $umbrella -match 'tauri-command:read_live_context_entries') {
  throw 'Issue #101 原上下文管理总功能漂移。'
}

$sourceText = Get-Content -Raw -LiteralPath parity/features/source-index.yml
$readLive = Get-YamlListItemBlock -Text $sourceText -Id 'tauri-command:read_live_context_entries'
if ($readLive -notmatch 'side_effects: \[filesystem-read\]' -or
    $readLive -notmatch 'feature_id: feature\.provider-network\.context-entry-observation') {
  throw 'Issue #101 read_live_context_entries 归属或副作用漂移。'
}
foreach ($sourceId in @(
  'tauri-command:delete_context_entry',
  'tauri-command:extract_relay_common_config',
  'tauri-command:list_context_entries',
  'tauri-command:sync_live_context_entries',
  'tauri-command:upsert_context_entry'
)) {
  $block = Get-YamlListItemBlock -Text $sourceText -Id $sourceId
  if ($block -notmatch 'side_effects: \[filesystem-read, filesystem-write\]' -or
      $block -notmatch 'feature_id: feature\.provider-network\.context-entry-management') {
    throw "Issue #101 原上下文管理入口漂移：$sourceId"
  }
}

$platformSource = Get-Content -Raw -LiteralPath crates/inputcodex-platform/src/context_entry_observation.rs
$productionSource = [regex]::Split($platformSource, '(?m)^#\[cfg\(test\)\]')[0]
foreach ($required in @(
  'SystemPlatformPaths.resolve',
  'symlink_metadata',
  'File::open',
  '256 * 1024',
  'DocumentMut',
  'ContextEntryCatalogObservation'
)) {
  if (-not $productionSource.Contains($required)) { throw "Issue #101 缺少平台门禁：$required" }
}
foreach ($forbidden in @(
  'read_to_string',
  'normalize_duplicate_toml_text',
  'toml_body',
  'summary',
  'fs::write',
  'OpenOptions',
  'std::process::Command',
  'Command::new',
  'std::thread',
  'rusqlite',
  'reqwest',
  'hyper',
  'TcpStream',
  'UdpSocket',
  'iced',
  'unsafe {'
)) {
  if ($productionSource.Contains($forbidden)) { throw "Issue #101 禁止能力命中：$forbidden" }
}

$ownerName = [Environment]::UserName
$productionPaths = @(
  'crates/inputcodex-domain/src/context_entry_observation.rs',
  'crates/inputcodex-application/src/context_entry_observation.rs',
  'crates/inputcodex-platform/src/context_entry_observation.rs'
)
$addedDiff = @(git -c core.quotePath=false diff --unified=0 "$baseline...HEAD" -- $productionPaths)
if ($LASTEXITCODE -ne 0) { throw 'Issue #101 新增行隐私扫描准备失败。' }
$addedLines = @($addedDiff | Where-Object { $_ -cmatch '^\+(?!\+\+)' })
if (-not [string]::IsNullOrWhiteSpace($ownerName)) {
  $privateLeaks = @($addedLines | Select-String -SimpleMatch -CaseSensitive $ownerName)
  if ($privateLeaks.Count -ne 0) { throw "Issue #101 泄露本机用户标识：$($privateLeaks -join '; ')" }
}
$absolutePathLeaks = @($addedLines | Select-String -Pattern '(?i)[A-Z]:\\Users\\')
if ($absolutePathLeaks.Count -ne 0) { throw "Issue #101 泄露 Windows 用户绝对路径：$($absolutePathLeaks -join '; ')" }

git diff --check
Assert-NativeSuccess 'Issue #101 Git 空白检查'
```

预期：四 crate tests/Clippy、`rustfmt`、CI 合同、Release Audit、仓库政策和 Cargo metadata 均通过；
批准上限为 `24` 路径且哈希为
`sha256:5b96235eb1fa7832e5710f7343917a5c2512bc50a46198ed584323366dd34372`。本次已形成
`DocumentMut` span 新根因，实际范围必须包含 `err.md` 并精确使用同一 `24` 路径哈希；没有新根因的
复用任务才使用排除 `err.md` 的 `23` 路径和
`sha256:08b223934a07a66d91e5cf2e1b340a243ea460d6c4edc266f58d30101c478d47`。只重映射
`read_live_context_entries`，不新增依赖、不返回配置正文且原上下文管理总功能继续 `unassessed`。

## Issue #98 Relay 认证与配置状态只读观察本地轻量验证

在仓库根目录、分支 `codex/issue-98-gate-5-relay-status-observation` 执行。Git 时间只使用系统
默认本机时间；不得设置 `GIT_AUTHOR_DATE` 或 `GIT_COMMITTER_DATE`。

```powershell
$ErrorActionPreference = 'Stop'

function Assert-NativeSuccess {
  param([Parameter(Mandatory)][string]$Label)
  if ($LASTEXITCODE -ne 0) { throw "$Label 失败：$LASTEXITCODE" }
}

Get-Date -Format 'yyyy-MM-dd HH:mm:ss.fff zzz'

cargo test --locked --offline --all-targets -p inputcodex-domain -p inputcodex-application -p inputcodex-platform -p inputcodex-parity
Assert-NativeSuccess 'Issue #98 四 crate 测试'

cargo clippy --locked --offline --all-targets -p inputcodex-domain -p inputcodex-application -p inputcodex-platform -p inputcodex-parity -- -D warnings
Assert-NativeSuccess 'Issue #98 四 crate Clippy'

cargo fmt --all -- --check
Assert-NativeSuccess 'Issue #98 rustfmt 检查'

pwsh -NoProfile -File scripts/ci/Test-CiScripts.ps1
Assert-NativeSuccess 'Issue #98 CI 合同'

pwsh -NoProfile -File scripts/ci/Verify-ReleaseAuditGate.ps1 -RepositoryRoot .
Assert-NativeSuccess 'Issue #98 Release Audit'

pwsh -NoProfile -File scripts/ci/Verify-RepositoryPolicy.ps1 -RepositoryRoot .
Assert-NativeSuccess 'Issue #98 Repository Policy'

cargo metadata --locked --offline --no-deps | Out-Null
Assert-NativeSuccess 'Issue #98 Cargo metadata'
```

范围、哈希、依赖、目录分拆、隐私与禁止能力验证：

```powershell
$ErrorActionPreference = 'Stop'
$baseline = 'b7c4174671caba806162a42e82b7bc0b20f73bf5'
$candidate = [string[]]@(
  'AGENTS.md',
  'CONTEXT.md',
  'Cargo.lock',
  'Cargo.toml',
  'README.md',
  'build.md',
  'crates/inputcodex-application/src/lib.rs',
  'crates/inputcodex-application/src/relay_status_observation.rs',
  'crates/inputcodex-application/tests/relay_status_observation.rs',
  'crates/inputcodex-domain/src/lib.rs',
  'crates/inputcodex-domain/src/relay_status_observation.rs',
  'crates/inputcodex-domain/tests/relay_status_observation.rs',
  'crates/inputcodex-parity/tests/catalog_repository.rs',
  'crates/inputcodex-platform/Cargo.toml',
  'crates/inputcodex-platform/src/lib.rs',
  'crates/inputcodex-platform/src/relay_status_observation.rs',
  'crates/inputcodex-platform/tests/relay_status_observation.rs',
  'docs/plans/2026-07-29-issue-98-gate-5-relay-status-observation.md',
  'docs/plans/PROJECT-MASTER-PLAN.md',
  'docs/plans/sessions/2026-07-29-issue-98-gate-5-relay-status-observation.md',
  'docs/reports/issue-98-gate-5-relay-status-observation.md',
  'docs/workflows/2026-07-29-issue-98-gate-5-relay-status-observation-runtime.md',
  'err.md',
  'parity/README.md',
  'parity/contracts/provider-network.yml',
  'parity/features/provider-network.yml',
  'parity/features/source-index.yml'
)
$orderedCandidate = [string[]]$candidate.Clone()
[Array]::Sort($orderedCandidate, [StringComparer]::Ordinal)
$payload = [string]::Join("`n", $orderedCandidate) + "`n"
$scopeHash = [Convert]::ToHexString(
  [Security.Cryptography.SHA256]::HashData(
    [Text.UTF8Encoding]::new($false).GetBytes($payload)
  )
).ToLowerInvariant()
if ($candidate.Count -ne 27) { throw "Issue #98 路径数量漂移：$($candidate.Count)" }
if ($scopeHash -ne 'b1dda60cda57d4be9344b3fa0c74a49b6087b9bdf03fceb5a772ec7e893d63a5') {
  throw "Issue #98 scope_hash 漂移：sha256:$scopeHash"
}

$actualSet = [Collections.Generic.HashSet[string]]::new([StringComparer]::Ordinal)
foreach ($path in @(git -c core.quotePath=false diff --name-only "$baseline...HEAD" --)) {
  if ($path) { [void]$actualSet.Add($path.Replace('\', '/')) }
}
if ($LASTEXITCODE -ne 0) { throw 'Issue #98 已提交路径收集失败。' }
foreach ($path in @(git -c core.quotePath=false diff --name-only --)) {
  if ($path) { [void]$actualSet.Add($path.Replace('\', '/')) }
}
if ($LASTEXITCODE -ne 0) { throw 'Issue #98 未暂存路径收集失败。' }
foreach ($path in @(git -c core.quotePath=false diff --cached --name-only --)) {
  if ($path) { [void]$actualSet.Add($path.Replace('\', '/')) }
}
if ($LASTEXITCODE -ne 0) { throw 'Issue #98 已暂存路径收集失败。' }
foreach ($path in @(git -c core.quotePath=false ls-files --others --exclude-standard)) {
  if ($path) { [void]$actualSet.Add($path.Replace('\', '/')) }
}
if ($LASTEXITCODE -ne 0) { throw 'Issue #98 未跟踪路径收集失败。' }

$allowedSet = [Collections.Generic.HashSet[string]]::new([StringComparer]::Ordinal)
foreach ($path in $candidate) { [void]$allowedSet.Add($path) }
$actual = [string[]]$actualSet
[Array]::Sort($actual, [StringComparer]::Ordinal)
$unexpected = @($actual | Where-Object { -not $allowedSet.Contains($_) })
if ($unexpected.Count -ne 0) { throw "Issue #98 路径越界：$($unexpected -join ', ')" }
if ($actual.Count -ne 27) { throw "Issue #98 实际路径数量漂移：$($actual.Count)" }
$actualPayload = [string]::Join("`n", $actual) + "`n"
$actualHash = [Convert]::ToHexString(
  [Security.Cryptography.SHA256]::HashData(
    [Text.UTF8Encoding]::new($false).GetBytes($actualPayload)
  )
).ToLowerInvariant()
if ($actualHash -ne $scopeHash) { throw "Issue #98 实际范围哈希漂移：sha256:$actualHash" }

$rootManifest = Get-Content -Raw -LiteralPath Cargo.toml
if (-not $rootManifest.Contains('toml_edit = { version = "=0.25.13", default-features = false, features = ["parse"] }')) {
  throw 'Issue #98 toml_edit Workspace 依赖合同漂移。'
}
$platformManifest = Get-Content -Raw -LiteralPath crates/inputcodex-platform/Cargo.toml
if (-not $platformManifest.Contains('toml_edit.workspace = true')) {
  throw 'Issue #98 Platform toml_edit 依赖合同漂移。'
}
$lockText = Get-Content -Raw -LiteralPath Cargo.lock
if (-not $lockText.Contains('name = "toml_edit"') -or
    -not $lockText.Contains('version = "0.25.13+spec-1.1.0"')) {
  throw 'Issue #98 Cargo.lock 未保持批准的 toml_edit 锁定实体。'
}

function Get-YamlListItemBlock {
  param([Parameter(Mandatory)][string]$Text, [Parameter(Mandatory)][string]$Id)
  $match = [regex]::Match($Text, "(?ms)^  - id: $([regex]::Escape($Id))\r?\n.*?(?=^  - id: |\z)")
  if (-not $match.Success) { throw "Issue #98 缺少目录条目：$Id" }
  $match.Value
}

$featureText = Get-Content -Raw -LiteralPath parity/features/provider-network.yml
$observation = Get-YamlListItemBlock -Text $featureText -Id 'feature.provider-network.relay-status-observation'
if ($observation -notmatch 'status: implemented' -or
    $observation -notmatch 'tauri-command:relay_status') {
  throw 'Issue #98 Relay 状态观察 feature 漂移。'
}
$umbrella = Get-YamlListItemBlock -Text $featureText -Id 'feature.provider-network.relay-profile-management'
if ($umbrella -notmatch 'status: unassessed' -or $umbrella -match 'tauri-command:relay_status') {
  throw 'Issue #98 原 Relay 配置管理总功能漂移。'
}

$sourceText = Get-Content -Raw -LiteralPath parity/features/source-index.yml
$relayStatus = Get-YamlListItemBlock -Text $sourceText -Id 'tauri-command:relay_status'
if ($relayStatus -notmatch 'side_effects: \[filesystem-read\]' -or
    $relayStatus -notmatch 'feature_id: feature\.provider-network\.relay-status-observation') {
  throw 'Issue #98 relay_status 归属或副作用漂移。'
}
foreach ($sourceId in @('core-module:relay_config', 'tauri-command:backfill_relay_profile_from_live', 'tauri-command:read_relay_files', 'tauri-command:save_relay_file', 'tauri-command:switch_relay_profile')) {
  $block = Get-YamlListItemBlock -Text $sourceText -Id $sourceId
  if ($block -notmatch 'side_effects: \[filesystem-read, filesystem-write\]' -or
      $block -notmatch 'feature_id: feature\.provider-network\.relay-profile-management') {
    throw "Issue #98 原 Relay 配置管理入口漂移：$sourceId"
  }
}

$platformSource = Get-Content -Raw -LiteralPath crates/inputcodex-platform/src/relay_status_observation.rs
$productionSource = [regex]::Split($platformSource, '(?m)^#\[cfg\(test\)\]')[0]
foreach ($required in @('SystemPlatformPaths.resolve', 'fs::symlink_metadata', 'File::open', 'file.take(limit as u64 + 1)', 'serde_json::from_slice::<Value>', 'parse::<DocumentMut>()', 'RelayStatusObservation::new')) {
  if (-not $productionSource.Contains($required)) { throw "Issue #98 缺少平台门禁：$required" }
}
foreach ($forbidden in @('pub trait RelayStatusFileProbe', 'pub fn observe_relay_status_files', 'read_to_string', 'fs::write', 'OpenOptions', 'std::process::Command', 'Command::new', 'std::thread', 'reqwest', 'hyper', 'TcpStream', 'UdpSocket', 'iced', 'unsafe {')) {
  if ($productionSource.Contains($forbidden)) { throw "Issue #98 禁止能力命中：$forbidden" }
}

$ownerName = [Environment]::UserName
$addedDiff = @(git -c core.quotePath=false diff --unified=0 "$baseline" -- $actual)
if ($LASTEXITCODE -ne 0) { throw 'Issue #98 新增行隐私扫描准备失败。' }
$addedLines = @($addedDiff | Where-Object { $_ -cmatch '^\+(?!\+\+)' })
if (-not [string]::IsNullOrWhiteSpace($ownerName)) {
  $privateLeaks = @($addedLines | Select-String -SimpleMatch -CaseSensitive $ownerName)
  if ($privateLeaks.Count -ne 0) { throw "Issue #98 泄露本机用户标识：$($privateLeaks -join '; ')" }
}
$absolutePathLeaks = @($addedLines | Select-String -Pattern '(?i)[A-Z]:\\Users\\')
if ($absolutePathLeaks.Count -ne 0) { throw "Issue #98 泄露 Windows 用户绝对路径：$($absolutePathLeaks -join '; ')" }

git diff --check
if ($LASTEXITCODE -ne 0) { throw "Issue #98 Git 空白检查失败：$LASTEXITCODE" }
```

预期：四 crate tests/Clippy、`rustfmt`、CI 合同、Release Audit、仓库政策和 Cargo metadata 均通过；
候选与实际范围均为 `27` 路径且哈希为
`sha256:b1dda60cda57d4be9344b3fa0c74a49b6087b9bdf03fceb5a772ec7e893d63a5`；feature/contract
为 `41/41`、source 为 `133`、fixture manifest 为 `11`；Relay 状态观察只读两个固定文档，隐私与
禁止能力扫描命中为 `0`。

## Issue #95 诊断日志只读结构观察本地轻量验证

在仓库根目录、分支 `codex/issue-95-gate-5-diagnostic-log-observation` 执行。Git 时间只使用系统
默认本机时间；不得设置 `GIT_AUTHOR_DATE` 或 `GIT_COMMITTER_DATE`。

```powershell
$ErrorActionPreference = 'Stop'

Get-Date -Format 'yyyy-MM-dd HH:mm:ss.fff zzz'

cargo test --locked --offline --all-targets -p inputcodex-domain -p inputcodex-application -p inputcodex-platform -p inputcodex-parity
if ($LASTEXITCODE -ne 0) { throw "Issue #95 四 crate 测试失败：$LASTEXITCODE" }

cargo clippy --locked --offline --all-targets -p inputcodex-domain -p inputcodex-application -p inputcodex-platform -p inputcodex-parity -- -D warnings
if ($LASTEXITCODE -ne 0) { throw "Issue #95 四 crate Clippy 失败：$LASTEXITCODE" }

cargo fmt --all -- --check
if ($LASTEXITCODE -ne 0) { throw "Issue #95 rustfmt 检查失败：$LASTEXITCODE" }

pwsh -NoProfile -File scripts/ci/Test-CiScripts.ps1
if ($LASTEXITCODE -ne 0) { throw "Issue #95 CI 合同失败：$LASTEXITCODE" }

pwsh -NoProfile -File scripts/ci/Verify-ReleaseAuditGate.ps1 -RepositoryRoot .
if ($LASTEXITCODE -ne 0) { throw "Issue #95 Release Audit 失败：$LASTEXITCODE" }

pwsh -NoProfile -File scripts/ci/Verify-RepositoryPolicy.ps1 -RepositoryRoot .
if ($LASTEXITCODE -ne 0) { throw "Issue #95 Repository Policy 失败：$LASTEXITCODE" }
```

范围、哈希、目录分拆、隐私与禁止能力验证：

```powershell
$ErrorActionPreference = 'Stop'
$baseline = '9587549c3f1bb334507075499f806485d83fce6a'
$candidate = [string[]]@(
  'AGENTS.md',
  'CONTEXT.md',
  'README.md',
  'build.md',
  'crates/inputcodex-application/src/diagnostic_log_observation.rs',
  'crates/inputcodex-application/src/lib.rs',
  'crates/inputcodex-application/tests/diagnostic_log_observation.rs',
  'crates/inputcodex-domain/src/diagnostic_log_observation.rs',
  'crates/inputcodex-domain/src/lib.rs',
  'crates/inputcodex-domain/tests/diagnostic_log_observation.rs',
  'crates/inputcodex-parity/tests/catalog_repository.rs',
  'crates/inputcodex-platform/src/diagnostic_log_observation.rs',
  'crates/inputcodex-platform/src/lib.rs',
  'crates/inputcodex-platform/tests/diagnostic_log_observation.rs',
  'docs/plans/2026-07-28-issue-95-gate-5-diagnostic-log-observation.md',
  'docs/plans/PROJECT-MASTER-PLAN.md',
  'docs/plans/sessions/2026-07-28-issue-95-gate-5-diagnostic-log-observation.md',
  'docs/reports/issue-95-gate-5-diagnostic-log-observation.md',
  'docs/workflows/2026-07-28-issue-95-gate-5-diagnostic-log-observation-runtime.md',
  'err.md',
  'parity/README.md',
  'parity/contracts/foundation-platform.yml',
  'parity/features/foundation-platform.yml',
  'parity/features/source-index.yml'
)
[Array]::Sort($candidate, [StringComparer]::Ordinal)
$payload = [string]::Join("`n", $candidate) + "`n"
$scopeHash = [Convert]::ToHexString(
  [Security.Cryptography.SHA256]::HashData(
    [Text.UTF8Encoding]::new($false).GetBytes($payload)
  )
).ToLowerInvariant()
if ($candidate.Count -ne 24) { throw "Issue #95 路径数量漂移：$($candidate.Count)" }
if ($scopeHash -ne '8d407c269436c655e12ff94035183de6aa50dc7759fbc75f9cb7b6f9b0349d38') {
  throw "Issue #95 scope_hash 漂移：sha256:$scopeHash"
}

$committed = @(git -c core.quotePath=false diff --name-only "$baseline...HEAD" --)
$unstaged = @(git -c core.quotePath=false diff --name-only --)
$staged = @(git -c core.quotePath=false diff --cached --name-only --)
$untracked = @(git -c core.quotePath=false ls-files --others --exclude-standard)
$actual = @($committed + $unstaged + $staged + $untracked | Where-Object { $_ } | Sort-Object -Unique)
$unexpected = @($actual | Where-Object { $_ -notin $candidate })
if ($unexpected.Count -ne 0) { throw "Issue #95 路径越界：$($unexpected -join ', ')" }
if ($actual.Count -ne 23) { throw "Issue #95 实际路径数量漂移：$($actual.Count)" }
if ($actual -contains 'err.md') { throw 'Issue #95 没有新根因，不应修改 err.md。' }
if (@($actual | Where-Object { $_ -match '(^|/)Cargo\.(toml|lock)$' }).Count -ne 0) {
  throw 'Issue #95 禁止修改 Cargo 依赖或锁文件。'
}

function Get-YamlListItemBlock {
  param([Parameter(Mandatory)][string]$Text, [Parameter(Mandatory)][string]$Id)
  $match = [regex]::Match($Text, "(?ms)^  - id: $([regex]::Escape($Id))\r?\n.*?(?=^  - id: |\z)")
  if (-not $match.Success) { throw "Issue #95 缺少目录条目：$Id" }
  $match.Value
}

$sourceText = Get-Content -Raw -LiteralPath parity/features/source-index.yml
$readLatest = Get-YamlListItemBlock -Text $sourceText -Id 'tauri-command:read_latest_logs'
if ($readLatest -notmatch 'side_effects: \[filesystem-read\]' -or
    $readLatest -notmatch 'feature_id: feature\.foundation-platform\.diagnostic-log-observation') {
  throw 'Issue #95 read_latest_logs 归属或副作用漂移。'
}
foreach ($sourceId in @('core-module:diagnostic_log', 'tauri-command:clear_logs', 'tauri-command:copy_diagnostics', 'tauri-command:write_diagnostic_event')) {
  $block = Get-YamlListItemBlock -Text $sourceText -Id $sourceId
  if ($block -notmatch 'side_effects: \[filesystem-read, filesystem-write, clipboard-write\]' -or
      $block -notmatch 'feature_id: feature\.foundation-platform\.diagnostics') {
    throw "Issue #95 原诊断总功能入口漂移：$sourceId"
  }
}

$platformSource = Get-Content -Raw -LiteralPath crates/inputcodex-platform/src/diagnostic_log_observation.rs
$productionSource = [regex]::Split($platformSource, '(?m)^#\[cfg\(test\)\]')[0]
foreach ($required in @('SystemPlatformPaths.resolve', 'fs::symlink_metadata', 'File::open', 'file.seek(SeekFrom::Start(start))', 'file.take(limit as u64)', 'serde_json::from_slice::<Value>', 'DiagnosticLogObservation::new')) {
  if (-not $productionSource.Contains($required)) { throw "Issue #95 缺少平台门禁：$required" }
}
foreach ($forbidden in @('pub trait DiagnosticLogFileProbe', 'pub fn observe_diagnostic_log_file', 'read_to_string', 'fs::write', 'OpenOptions', 'std::process::Command', 'Command::new', 'std::thread', 'reqwest', 'hyper', 'TcpStream', 'UdpSocket', 'iced', 'unsafe {')) {
  if ($productionSource.Contains($forbidden)) { throw "Issue #95 禁止能力命中：$forbidden" }
}

$ownerName = [Environment]::UserName
if (-not [string]::IsNullOrWhiteSpace($ownerName)) {
  $addedDiff = @(git -c core.quotePath=false diff --unified=0 "$baseline" -- $actual)
  if ($LASTEXITCODE -ne 0) { throw 'Issue #95 新增行隐私扫描准备失败。' }
  $addedLines = @($addedDiff | Where-Object { $_ -cmatch '^\+(?!\+\+)' })
  $privateLeaks = @($addedLines | Select-String -SimpleMatch -CaseSensitive $ownerName)
  $untrackedScanPaths = @($untracked | Where-Object { Test-Path -LiteralPath $_ })
  if ($untrackedScanPaths.Count -ne 0) {
    $privateLeaks += @(rg -n --fixed-strings $ownerName -- $untrackedScanPaths 2>$null)
    if ($LASTEXITCODE -notin 0, 1) { throw 'Issue #95 未跟踪文件隐私扫描执行失败。' }
  }
  if ($privateLeaks.Count -ne 0) {
    throw "Issue #95 泄露本机用户标识：$($privateLeaks -join '; ')"
  }
}

git diff --check
if ($LASTEXITCODE -ne 0) { throw "Issue #95 Git 空白检查失败：$LASTEXITCODE" }
```

预期：四 crate tests/Clippy、`rustfmt`、CI 合同、Release Audit 与仓库政策均通过；候选范围保持
`24` 路径和批准哈希，实际使用其中 `23` 路径且 `err.md` 不变；Cargo 无变化；feature/contract
为 `40/40`、source 为 `133`、fixture manifest 为 `11`，诊断日志观察不公开任意路径且不包含
正文泄露、写入、网络、子进程、线程、Watcher、UI、注入或 `unsafe`。

## Issue #92 设置只读观察本地轻量验证

在仓库根目录、分支 `codex/issue-92-gate-5-settings-observation` 执行。Git 时间只使用系统默认
本机时间；不得设置 `GIT_AUTHOR_DATE` 或 `GIT_COMMITTER_DATE`。

```powershell
$ErrorActionPreference = 'Stop'

Get-Date -Format 'yyyy-MM-dd HH:mm:ss.fff zzz'

cargo test --locked --offline --all-targets -p inputcodex-domain -p inputcodex-application -p inputcodex-platform -p inputcodex-parity
if ($LASTEXITCODE -ne 0) { throw "Issue #92 四 crate 测试失败：$LASTEXITCODE" }

cargo clippy --locked --offline --all-targets -p inputcodex-domain -p inputcodex-application -p inputcodex-platform -p inputcodex-parity -- -D warnings
if ($LASTEXITCODE -ne 0) { throw "Issue #92 四 crate Clippy 失败：$LASTEXITCODE" }

cargo fmt --all -- --check
if ($LASTEXITCODE -ne 0) { throw "Issue #92 rustfmt 检查失败：$LASTEXITCODE" }

pwsh -NoProfile -File scripts/ci/Test-CiScripts.ps1
if ($LASTEXITCODE -ne 0) { throw "Issue #92 CI 合同失败：$LASTEXITCODE" }

pwsh -NoProfile -File scripts/ci/Verify-ReleaseAuditGate.ps1 -RepositoryRoot .
if ($LASTEXITCODE -ne 0) { throw "Issue #92 Release Audit 失败：$LASTEXITCODE" }

pwsh -NoProfile -File scripts/ci/Verify-RepositoryPolicy.ps1 -RepositoryRoot .
if ($LASTEXITCODE -ne 0) { throw "Issue #92 Repository Policy 失败：$LASTEXITCODE" }
```

范围、哈希、依赖、目录分拆、隐私与禁止能力验证：

```powershell
$ErrorActionPreference = 'Stop'
$baseline = 'a5559f4a873a81d91ed09b571503523a78a45118'
$candidate = [string[]]@(
  'AGENTS.md',
  'CONTEXT.md',
  'Cargo.lock',
  'Cargo.toml',
  'README.md',
  'build.md',
  'crates/inputcodex-application/src/lib.rs',
  'crates/inputcodex-application/src/settings_observation.rs',
  'crates/inputcodex-application/tests/settings_observation.rs',
  'crates/inputcodex-domain/src/lib.rs',
  'crates/inputcodex-domain/src/settings_observation.rs',
  'crates/inputcodex-domain/tests/settings_observation.rs',
  'crates/inputcodex-parity/tests/catalog_repository.rs',
  'crates/inputcodex-platform/Cargo.toml',
  'crates/inputcodex-platform/src/lib.rs',
  'crates/inputcodex-platform/src/settings_observation.rs',
  'crates/inputcodex-platform/tests/settings_observation.rs',
  'docs/plans/2026-07-28-issue-92-gate-5-settings-observation.md',
  'docs/plans/PROJECT-MASTER-PLAN.md',
  'docs/plans/sessions/2026-07-28-issue-92-gate-5-settings-observation.md',
  'docs/reports/issue-92-gate-5-settings-observation.md',
  'docs/workflows/2026-07-28-issue-92-gate-5-settings-observation-runtime.md',
  'err.md',
  'parity/README.md',
  'parity/contracts/foundation-platform.yml',
  'parity/features/foundation-platform.yml',
  'parity/features/source-index.yml'
)
[Array]::Sort($candidate, [StringComparer]::Ordinal)
$payload = [string]::Join("`n", $candidate) + "`n"
$scopeHash = [Convert]::ToHexString(
  [Security.Cryptography.SHA256]::HashData(
    [Text.UTF8Encoding]::new($false).GetBytes($payload)
  )
).ToLowerInvariant()
if ($candidate.Count -ne 27) { throw "Issue #92 路径数量漂移：$($candidate.Count)" }
if ($scopeHash -ne 'ca252684075d32de7aaf2ca066f12822ce48a5b01d1b0fcf67df146ea792baf1') {
  throw "Issue #92 scope_hash 漂移：sha256:$scopeHash"
}

$changed = @(
  git -c core.quotePath=false diff --name-only $baseline -- |
    Where-Object { -not [string]::IsNullOrWhiteSpace($_) }
)
$unexpected = @($changed | Where-Object { $_ -notin $candidate })
if ($unexpected.Count -ne 0) { throw "Issue #92 路径越界：$($unexpected -join ', ')" }
if ($changed.Count -ne 26) { throw "Issue #92 实际路径数量漂移：$($changed.Count)" }
if ($changed -contains 'err.md') { throw 'Issue #92 没有新根因，不应修改 err.md。' }

$baselineLock = @(git show "${baseline}:Cargo.lock")
if ($LASTEXITCODE -ne 0) { throw 'Issue #92 无法读取基线 Cargo.lock。' }
$currentLock = Get-Content -LiteralPath Cargo.lock
$baselinePackages = @(
  $baselineLock | Select-String '^name = "([^"]+)"$' |
    ForEach-Object { $_.Matches[0].Groups[1].Value } | Sort-Object -Unique
)
$currentPackages = @(
  $currentLock | Select-String '^name = "([^"]+)"$' |
    ForEach-Object { $_.Matches[0].Groups[1].Value } | Sort-Object -Unique
)
$newPackages = @($currentPackages | Where-Object { $_ -notin $baselinePackages })
$removedPackages = @($baselinePackages | Where-Object { $_ -notin $currentPackages })
if (($newPackages -join ',') -ne 'serde_json,zmij' -or $removedPackages.Count -ne 0) {
  throw "Issue #92 锁文件漂移：新增=$($newPackages -join ',') 移除=$($removedPackages -join ',')"
}

$rootManifest = Get-Content -Raw -LiteralPath Cargo.toml
$platformManifest = Get-Content -Raw -LiteralPath crates/inputcodex-platform/Cargo.toml
if ($rootManifest -notmatch 'serde_json = "=1\.0\.149"') { throw 'Issue #92 serde_json 版本漂移。' }
if ($platformManifest -notmatch 'serde_json\.workspace = true') { throw 'Issue #92 Platform 未复用 workspace 依赖。' }

$registryRoot = Join-Path $HOME '.cargo/registry/src'
$serdeManifest = Get-ChildItem -LiteralPath $registryRoot -Recurse -File -Filter Cargo.toml |
  Where-Object { $_.Directory.Name -eq 'serde_json-1.0.149' } | Select-Object -First 1
$zmijManifest = Get-ChildItem -LiteralPath $registryRoot -Recurse -File -Filter Cargo.toml |
  Where-Object { $_.Directory.Name -eq 'zmij-1.0.23' } | Select-Object -First 1
if ($null -eq $serdeManifest -or $null -eq $zmijManifest) { throw 'Issue #92 依赖许可证元数据未缓存。' }
if ((Get-Content -Raw -LiteralPath $serdeManifest.FullName) -notmatch 'license = "MIT OR Apache-2\.0"') {
  throw 'Issue #92 serde_json 许可证漂移。'
}
if ((Get-Content -Raw -LiteralPath $zmijManifest.FullName) -notmatch 'license = "MIT"') {
  throw 'Issue #92 zmij 许可证漂移。'
}

function Get-YamlListItemBlock {
  param([Parameter(Mandatory)][string]$Text, [Parameter(Mandatory)][string]$Id)
  $match = [regex]::Match($Text, "(?ms)^  - id: $([regex]::Escape($Id))\r?\n.*?(?=^  - id: |\z)")
  if (-not $match.Success) { throw "Issue #92 缺少目录条目：$Id" }
  $match.Value
}

$sourceText = Get-Content -Raw -LiteralPath parity/features/source-index.yml
$loadSettings = Get-YamlListItemBlock -Text $sourceText -Id 'tauri-command:load_settings'
if ($loadSettings -notmatch 'side_effects: \[filesystem-read\]' -or
    $loadSettings -notmatch 'feature_id: feature\.foundation-platform\.settings-observation') {
  throw 'Issue #92 load_settings 归属或副作用漂移。'
}
foreach ($sourceId in @('core-module:settings', 'tauri-command:reset_settings', 'tauri-command:save_settings')) {
  $block = Get-YamlListItemBlock -Text $sourceText -Id $sourceId
  if ($block -notmatch 'side_effects: \[filesystem-read, filesystem-write\]' -or
      $block -notmatch 'feature_id: feature\.foundation-platform\.settings-management') {
    throw "Issue #92 原设置管理入口漂移：$sourceId"
  }
}

$platformSource = Get-Content -Raw -LiteralPath crates/inputcodex-platform/src/settings_observation.rs
foreach ($required in @(
  'SystemPlatformPaths.resolve',
  'fs::symlink_metadata',
  'File::open',
  'file.take(limit as u64 + 1)',
  'serde_json::from_slice::<Value>',
  'object.len()'
)) {
  if (-not $platformSource.Contains($required)) { throw "Issue #92 缺少平台门禁：$required" }
}
foreach ($forbidden in @(
  'pub trait SettingsFileProbe',
  'pub fn observe_settings_file',
  'fs::write',
  'OpenOptions',
  'std::process::Command',
  'Command::new',
  'std::thread',
  'reqwest',
  'hyper',
  'TcpStream',
  'UdpSocket',
  'iced',
  'unsafe {'
)) {
  if ($platformSource.Contains($forbidden)) { throw "Issue #92 禁止能力命中：$forbidden" }
}

$ownerName = [Environment]::UserName
if (-not [string]::IsNullOrWhiteSpace($ownerName)) {
  $privateLeaks = @(rg -n --fixed-strings $ownerName crates parity docs/reports/issue-92-gate-5-settings-observation.md 2>$null)
  if ($LASTEXITCODE -eq 0 -and $privateLeaks.Count -ne 0) {
    throw "Issue #92 泄露本机用户标识：$($privateLeaks -join '; ')"
  }
  if ($LASTEXITCODE -notin 0, 1) { throw 'Issue #92 隐私扫描执行失败。' }
}

git diff --check
if ($LASTEXITCODE -ne 0) { throw "Issue #92 Git 空白检查失败：$LASTEXITCODE" }
```

预期：四 crate tests/Clippy、`rustfmt`、CI 合同、Release Audit 与仓库政策均通过；候选范围保持
`27` 路径和批准哈希，实际使用其中 `26` 路径且 `err.md` 不变；Cargo 只新增
`serde_json 1.0.149` 与其必要锁包 `zmij 1.0.23`；feature/contract 为 `39/39`、source 为
`133`、fixture manifest 为 `11`，设置读取不公开任意路径且不包含写入、网络、子进程、线程、
Watcher、UI 或 `unsafe`。

## Issue #89 Relay 环境只读观察本地轻量验证

在仓库根目录、分支 `codex/issue-89-gate-5-relay-environment-observation` 执行。Git 时间只使用
系统默认本机时间；不得设置 `GIT_AUTHOR_DATE` 或 `GIT_COMMITTER_DATE`。

```powershell
$ErrorActionPreference = 'Stop'

Get-Date -Format 'yyyy-MM-dd HH:mm:ss.fff zzz'

cargo test --locked --offline -p inputcodex-domain -p inputcodex-application -p inputcodex-platform -p inputcodex-parity
if ($LASTEXITCODE -ne 0) { throw "Issue #89 四 crate 测试失败：$LASTEXITCODE" }

cargo clippy --locked --offline -p inputcodex-domain -p inputcodex-application -p inputcodex-platform -p inputcodex-parity --all-targets -- -D warnings
if ($LASTEXITCODE -ne 0) { throw "Issue #89 四 crate Clippy 失败：$LASTEXITCODE" }

cargo fmt --all -- --check
if ($LASTEXITCODE -ne 0) { throw "Issue #89 rustfmt 检查失败：$LASTEXITCODE" }

pwsh -NoProfile -File scripts/ci/Test-CiScripts.ps1
if ($LASTEXITCODE -ne 0) { throw "Issue #89 CI 合同失败：$LASTEXITCODE" }

pwsh -NoProfile -File scripts/ci/Verify-ReleaseAuditGate.ps1 -RepositoryRoot .
if ($LASTEXITCODE -ne 0) { throw "Issue #89 Release Audit 失败：$LASTEXITCODE" }

pwsh -NoProfile -File scripts/ci/Verify-RepositoryPolicy.ps1 -RepositoryRoot .
if ($LASTEXITCODE -ne 0) { throw "Issue #89 Repository Policy 失败：$LASTEXITCODE" }
```

范围、依赖、目录分拆、隐私与禁止能力验证：

```powershell
$ErrorActionPreference = 'Stop'

$candidate = [string[]]@(
  'AGENTS.md',
  'CONTEXT.md',
  'Cargo.lock',
  'Cargo.toml',
  'README.md',
  'build.md',
  'crates/inputcodex-application/src/lib.rs',
  'crates/inputcodex-application/src/relay_environment_observation.rs',
  'crates/inputcodex-application/tests/relay_environment_observation.rs',
  'crates/inputcodex-domain/src/lib.rs',
  'crates/inputcodex-domain/src/relay_environment_observation.rs',
  'crates/inputcodex-domain/tests/relay_environment_observation.rs',
  'crates/inputcodex-parity/tests/catalog_repository.rs',
  'crates/inputcodex-platform/Cargo.toml',
  'crates/inputcodex-platform/src/lib.rs',
  'crates/inputcodex-platform/src/platform_paths.rs',
  'crates/inputcodex-platform/src/relay_environment_observation.rs',
  'crates/inputcodex-platform/src/relay_environment_observation/macos.rs',
  'crates/inputcodex-platform/src/relay_environment_observation/windows.rs',
  'crates/inputcodex-platform/tests/relay_environment_observation.rs',
  'docs/plans/2026-07-28-issue-89-gate-5-relay-environment-observation.md',
  'docs/plans/PROJECT-MASTER-PLAN.md',
  'docs/plans/sessions/2026-07-28-issue-89-gate-5-relay-environment-observation.md',
  'docs/reports/issue-89-gate-5-relay-environment-observation.md',
  'docs/workflows/2026-07-28-issue-89-gate-5-relay-environment-observation-runtime.md',
  'err.md',
  'parity/README.md',
  'parity/contracts/provider-network.yml',
  'parity/features/provider-network.yml',
  'parity/features/source-index.yml'
)
[Array]::Sort($candidate, [StringComparer]::Ordinal)
$payload = [string]::Join("`n", $candidate) + "`n"
$scopeHash = [Convert]::ToHexString(
  [Security.Cryptography.SHA256]::HashData(
    [Text.UTF8Encoding]::new($false).GetBytes($payload)
  )
).ToLowerInvariant()
if ($candidate.Count -ne 30) { throw "Issue #89 路径数量漂移：$($candidate.Count)" }
if ($scopeHash -ne '0adc20d0ed4d73ae645a5ffb23d7208f7aaabfea92c4d6fd62e0da3a120e8f77') {
  throw "Issue #89 scope_hash 漂移：sha256:$scopeHash"
}

$changed = @(
  git -c core.quotePath=false diff --name-only origin/main...HEAD
  git -c core.quotePath=false diff --name-only
  git -c core.quotePath=false ls-files --others --exclude-standard
) | Where-Object { $_ } | Sort-Object -Unique
$outside = @($changed | Where-Object { $_ -notin $candidate })
if ($outside.Count -ne 0) { throw "Issue #89 越界路径：$($outside -join ', ')" }

$protected = @(
  git -c core.quotePath=false diff --name-only origin/main...HEAD -- apps .github/workflows upstream scripts benchmarks
  git -c core.quotePath=false diff --name-only -- apps .github/workflows upstream scripts benchmarks
) | Where-Object { $_ } | Sort-Object -Unique
if ($protected.Count -ne 0) { throw "Issue #89 修改受保护路径：$($protected -join ', ')" }

$rootCargo = Get-Content -Raw -LiteralPath Cargo.toml
if (($rootCargo.Split('windows-registry = "=0.6.1"').Count - 1) -ne 1) {
  throw 'Issue #89 根 Cargo 必须精确登记一次 windows-registry = 0.6.1。'
}
$platformCargo = Get-Content -Raw -LiteralPath crates/inputcodex-platform/Cargo.toml
$windowsTarget = [regex]::Match(
  $platformCargo,
  '(?ms)^\[target\.''cfg\(target_os = "windows"\)''\.dependencies\]\r?\n.*?(?=^\[|\z)'
).Value
if ($windowsTarget -notmatch '(?m)^windows-registry\.workspace = true$') {
  throw 'Issue #89 windows-registry 必须只进入 Windows target 依赖。'
}

$lockDiff = @(git diff --unified=1 origin/main...HEAD -- Cargo.lock)
$newPackageNames = @()
foreach ($line in $lockDiff) {
  if ($line -match '^\+name = "([^"]+)"$') { $newPackageNames += $Matches[1] }
}
$expectedPackages = @('windows-registry', 'windows-result', 'windows-strings')
if ((@($newPackageNames | Sort-Object) -join ',') -ne ($expectedPackages -join ',')) {
  throw "Issue #89 Cargo.lock 新包漂移：$($newPackageNames -join ', ')"
}
$lockText = Get-Content -Raw -LiteralPath Cargo.lock
foreach ($package in @(
  @{ Name = 'windows-registry'; Version = '0.6.1' },
  @{ Name = 'windows-result'; Version = '0.4.1' },
  @{ Name = 'windows-strings'; Version = '0.5.1' }
)) {
  $pattern = "(?ms)\[\[package\]\]\r?\nname = `"$([regex]::Escape($package.Name))`"\r?\nversion = `"$([regex]::Escape($package.Version))`""
  if ($lockText -notmatch $pattern) { throw "Issue #89 锁文件缺少 $($package.Name) $($package.Version)" }
}

$sourceText = Get-Content -Raw -LiteralPath parity/features/source-index.yml
foreach ($sourceId in @('core-module:relay_environment', 'tauri-command:check_relay_environment')) {
  $block = [regex]::Match(
    $sourceText,
    "(?ms)^  - id: $([regex]::Escape($sourceId))\r?\n.*?(?=^  - id: |\z)"
  ).Value
  if ($block -notmatch 'side_effects: \[environment-read, filesystem-read\]' -or
      $block -notmatch 'feature_id: feature\.provider-network\.relay-environment-observation') {
    throw "Issue #89 Relay 入口归属或副作用漂移：$sourceId"
  }
}
$proxyBlock = [regex]::Match(
  $sourceText,
  '(?ms)^  - id: core-module:proxy\r?\n.*?(?=^  - id: |\z)'
).Value
if ($proxyBlock -notmatch 'side_effects: \[environment-read, network-read\]' -or
    $proxyBlock -notmatch 'feature_id: feature\.provider-network\.network-environment') {
  throw 'Issue #89 core-module:proxy 被错误迁入只读子能力。'
}

$featureText = Get-Content -Raw -LiteralPath parity/features/provider-network.yml
$relayFeature = [regex]::Match(
  $featureText,
  '(?ms)^  - id: feature\.provider-network\.relay-environment-observation\r?\n.*?(?=^  - id: |\z)'
).Value
$networkFeature = [regex]::Match(
  $featureText,
  '(?ms)^  - id: feature\.provider-network\.network-environment\r?\n.*?(?=^  - id: |\z)'
).Value
if ($relayFeature -notmatch '(?m)^    status: implemented$' -or
    $relayFeature -notmatch 'core-module:relay_environment' -or
    $relayFeature -notmatch 'tauri-command:check_relay_environment') {
  throw 'Issue #89 新 Relay 子能力状态或入口不完整。'
}
if ($networkFeature -notmatch '(?m)^    status: unassessed$' -or
    $networkFeature -notmatch 'core-module:proxy' -or
    $networkFeature -match 'relay_environment|check_relay_environment') {
  throw 'Issue #89 原网络环境总功能被错误改写。'
}

$contractText = Get-Content -Raw -LiteralPath parity/contracts/provider-network.yml
if ($contractText -notmatch 'contract\.feature\.provider-network\.relay-environment-observation\.baseline' -or
    $contractText -notmatch 'Clash 单候选最多读取 64 KiB') {
  throw 'Issue #89 Relay 行为合同缺失或有界读取语义漂移。'
}

$production = @(
  'crates/inputcodex-domain/src/relay_environment_observation.rs',
  'crates/inputcodex-application/src/relay_environment_observation.rs',
  'crates/inputcodex-platform/src/relay_environment_observation.rs',
  'crates/inputcodex-platform/src/relay_environment_observation/windows.rs',
  'crates/inputcodex-platform/src/relay_environment_observation/macos.rs'
)
$forbiddenProduct = @(
  rg -n 'reqwest|hyper|TcpStream|UdpSocket|Command::new|std::process::Command|std::thread|thread::spawn|notify::|Watcher|fs::write|write_all|OpenOptions|set_var|remove_var|RegSetValue|KEY_SET_VALUE|unsafe\s*\{|iced|tauri|webview' $production 2>$null
)
if ($LASTEXITCODE -eq 0 -and $forbiddenProduct.Count -ne 0) {
  throw "Issue #89 命中禁止运行能力：$($forbiddenProduct -join '; ')"
}
if ($LASTEXITCODE -notin 0, 1) { throw 'Issue #89 禁止能力扫描执行失败。' }

$sharedPlatform = Get-Content -Raw -LiteralPath crates/inputcodex-platform/src/relay_environment_observation.rs
if ($sharedPlatform -notmatch 'const CLASH_CONFIG_LIMIT: usize = 64 \* 1024;') {
  throw 'Issue #89 Clash 单候选上限不是 64 KiB。'
}
$dotenvBlock = [regex]::Match(
  $sharedPlatform,
  '(?ms)^fn inspect_codex_dotenv\(.*?(?=^fn inspect_clash_candidates)'
).Value
if ($dotenvBlock -notmatch '\.metadata\(' -or $dotenvBlock -match 'read_limited|File::open|read_to_') {
  throw 'Issue #89 .env 检查读取了内容或没有使用元数据。'
}
foreach ($platformFile in @(
  'crates/inputcodex-platform/src/relay_environment_observation/windows.rs',
  'crates/inputcodex-platform/src/relay_environment_observation/macos.rs'
)) {
  $platformSource = Get-Content -Raw -LiteralPath $platformFile
  if (($platformSource.Split('std::env::vars_os()').Count - 1) -ne 1) {
    throw "Issue #89 平台入口没有精确扫描一次当前进程环境：$platformFile"
  }
}

$ownerName = [Environment]::UserName
if (-not [string]::IsNullOrWhiteSpace($ownerName)) {
  $privateLeaks = @(rg -n --fixed-strings $ownerName crates parity docs/reports/issue-89-gate-5-relay-environment-observation.md 2>$null)
  if ($LASTEXITCODE -eq 0 -and $privateLeaks.Count -ne 0) {
    throw "Issue #89 泄露本机用户标识：$($privateLeaks -join '; ')"
  }
  if ($LASTEXITCODE -notin 0, 1) { throw 'Issue #89 隐私扫描执行失败。' }
}

git diff --check
if ($LASTEXITCODE -ne 0) { throw "Issue #89 Git 空白检查失败：$LASTEXITCODE" }
```

预期：四 crate 测试与 Clippy 全绿、`rustfmt` 退出码为 `0`、CI 合同 `35/35`、
`release_audit=current`、仓库政策违规数为 `0`，实际差异全部位于批准的三十路径内；
Cargo 只新增 Windows target 的 `windows-registry = 0.6.1` 及三个必要锁包；两个 Relay 入口
映射到新只读子能力，`core-module:proxy` 与原网络环境总功能继续 `unassessed`；生产实现不含
网络、写入、子进程、线程、Watcher、UI 或 `unsafe`，`.env` 只读取元数据，Clash 单候选最多
读取 `64 KiB`，双平台当前进程环境各精确扫描一次，隐私匹配数为 `0`。

## Issue #86 运行时环境冲突只读观察本地轻量验证

在仓库根目录、分支 `codex/issue-86-gate-5-runtime-environment-observation` 执行。Git 时间只使用系统默认本机时间；不得设置 `GIT_AUTHOR_DATE` 或 `GIT_COMMITTER_DATE`。

```powershell
$ErrorActionPreference = 'Stop'

Get-Date -Format 'yyyy-MM-dd HH:mm:ss.fff zzz'

cargo test --locked --offline --ignore-rust-version -p inputcodex-domain -p inputcodex-application -p inputcodex-platform -p inputcodex-parity
if ($LASTEXITCODE -ne 0) { throw "Issue #86 四 crate 测试失败：$LASTEXITCODE" }

cargo clippy --locked --offline --ignore-rust-version -p inputcodex-domain -p inputcodex-application -p inputcodex-platform -p inputcodex-parity --all-targets -- -D warnings
if ($LASTEXITCODE -ne 0) { throw "Issue #86 四 crate Clippy 失败：$LASTEXITCODE" }

cargo fmt --all -- --check
if ($LASTEXITCODE -ne 0) { throw "Issue #86 rustfmt 检查失败：$LASTEXITCODE" }

pwsh -NoProfile -File scripts/ci/Test-CiScripts.ps1
if ($LASTEXITCODE -ne 0) { throw "Issue #86 CI 合同失败：$LASTEXITCODE" }

pwsh -NoProfile -File scripts/ci/Verify-ReleaseAuditGate.ps1 -RepositoryRoot .
if ($LASTEXITCODE -ne 0) { throw "Issue #86 Release Audit 失败：$LASTEXITCODE" }

pwsh -NoProfile -File scripts/ci/Verify-RepositoryPolicy.ps1 -RepositoryRoot .
if ($LASTEXITCODE -ne 0) { throw "Issue #86 Repository Policy 失败：$LASTEXITCODE" }
```

范围、哈希、单入口目录修订与禁止能力验证：

```powershell
$ErrorActionPreference = 'Stop'

$candidate = @(
  'AGENTS.md',
  'CONTEXT.md',
  'README.md',
  'build.md',
  'crates/inputcodex-application/src/lib.rs',
  'crates/inputcodex-application/src/runtime_environment_observation.rs',
  'crates/inputcodex-application/tests/runtime_environment_observation.rs',
  'crates/inputcodex-domain/src/lib.rs',
  'crates/inputcodex-domain/src/runtime_environment_observation.rs',
  'crates/inputcodex-domain/tests/runtime_environment_observation.rs',
  'crates/inputcodex-parity/tests/catalog_repository.rs',
  'crates/inputcodex-platform/src/lib.rs',
  'crates/inputcodex-platform/src/runtime_environment_observation.rs',
  'crates/inputcodex-platform/tests/runtime_environment_observation.rs',
  'docs/plans/2026-07-28-issue-86-gate-5-runtime-environment-observation.md',
  'docs/plans/PROJECT-MASTER-PLAN.md',
  'docs/plans/sessions/2026-07-28-issue-86-gate-5-runtime-environment-observation.md',
  'docs/reports/issue-86-gate-5-runtime-environment-observation.md',
  'docs/workflows/2026-07-28-issue-86-gate-5-runtime-environment-observation-runtime.md',
  'err.md',
  'parity/README.md',
  'parity/contracts/foundation-platform.yml',
  'parity/features/foundation-platform.yml',
  'parity/features/source-index.yml'
)
[Array]::Sort($candidate, [StringComparer]::Ordinal)
$payload = [string]::Join("`n", $candidate) + "`n"
$scopeHash = [Convert]::ToHexString(
  [Security.Cryptography.SHA256]::HashData(
    [Text.UTF8Encoding]::new($false).GetBytes($payload)
  )
).ToLowerInvariant()
if ($candidate.Count -ne 24) { throw "Issue #86 路径数量漂移：$($candidate.Count)" }
if ($scopeHash -ne 'dd1d784ffe3149bf130c6bd678050d6aea3059f33a405abee5e2cc3f9735bb59') {
  throw "Issue #86 scope_hash 漂移：sha256:$scopeHash"
}

$changed = @(
  git -c core.quotePath=false diff --name-only origin/main...HEAD
  git -c core.quotePath=false diff --name-only
  git -c core.quotePath=false ls-files --others --exclude-standard
) | Where-Object { $_ } | Sort-Object -Unique
$outside = @($changed | Where-Object { $_ -notin $candidate })
if ($outside.Count -ne 0) { throw "Issue #86 越界路径：$($outside -join ', ')" }

$protected = @(
  git -c core.quotePath=false diff --name-only origin/main...HEAD -- Cargo.toml Cargo.lock apps .github/workflows upstream scripts
  git -c core.quotePath=false diff --name-only -- Cargo.toml Cargo.lock apps .github/workflows upstream scripts
) | Where-Object { $_ } | Sort-Object -Unique
if ($protected.Count -ne 0) { throw "Issue #86 修改受保护路径：$($protected -join ', ')" }

$sourceIndexStat = @(git diff --numstat origin/main...HEAD -- parity/features/source-index.yml)
if ($sourceIndexStat.Count -ne 1 -or $sourceIndexStat[0] -notmatch '^2\s+2\s+parity/features/source-index\.yml$') {
  throw "Issue #86 source-index 差异不是批准的 2 行替换：$($sourceIndexStat -join '; ')"
}
$sourceText = Get-Content -Raw -LiteralPath parity/features/source-index.yml
$checkBlock = [regex]::Match($sourceText, '(?ms)^  - id: tauri-command:check_env_conflicts\r?\n.*?(?=^  - id: )').Value
if ($checkBlock -notmatch 'side_effects: \[environment-read\]' -or
    $checkBlock -notmatch 'feature_id: feature\.foundation-platform\.runtime-environment-conflict-observation' -or
    $checkBlock -match 'environment-write') {
  throw 'Issue #86 check_env_conflicts 单入口修订不符合批准语义。'
}
foreach ($sourceId in @('core-module:env_conflicts', 'tauri-command:remove_env_conflicts')) {
  $block = [regex]::Match($sourceText, "(?ms)^  - id: $([regex]::Escape($sourceId))\r?\n.*?(?=^  - id: )").Value
  if ($block -notmatch 'side_effects: \[environment-read, environment-write\]' -or
      $block -notmatch 'feature_id: feature\.foundation-platform\.environment-conflicts') {
    throw "Issue #86 破坏性总功能入口归属漂移：$sourceId"
  }
}

$production = @(
  'crates/inputcodex-domain/src/runtime_environment_observation.rs',
  'crates/inputcodex-application/src/runtime_environment_observation.rs',
  'crates/inputcodex-platform/src/runtime_environment_observation.rs'
)
$forbiddenProduct = @(
  rg -n 'std::env::set_var|std::env::remove_var|std::fs|File::|OpenOptions|reqwest|ureq|TcpStream|UdpSocket|std::thread|thread::spawn|std::process::Command|tokio::spawn|unsafe\s*\{|iced|tauri|webview' $production 2>$null
)
if ($LASTEXITCODE -eq 0 -and $forbiddenProduct.Count -ne 0) {
  throw "Issue #86 命中禁止运行能力：$($forbiddenProduct -join '; ')"
}
if ($LASTEXITCODE -notin 0, 1) { throw 'Issue #86 禁止能力扫描执行失败。' }

$platformSource = Get-Content -Raw -LiteralPath 'crates/inputcodex-platform/src/runtime_environment_observation.rs'
if (($platformSource.Split('std::env::vars_os()').Count - 1) -ne 1) {
  throw 'Issue #86 系统入口必须精确调用一次 std::env::vars_os()。'
}
if ($platformSource -match 'value\.(to_string|to_str|into_string)|format!\([^\r\n]*value') {
  throw 'Issue #86 环境值进入了转换或格式化路径。'
}

$ownerName = [Environment]::UserName
if (-not [string]::IsNullOrWhiteSpace($ownerName)) {
  $privateLeaks = @(rg -n --fixed-strings $ownerName crates parity docs/reports/issue-86-gate-5-runtime-environment-observation.md 2>$null)
  if ($LASTEXITCODE -eq 0 -and $privateLeaks.Count -ne 0) {
    throw "Issue #86 泄露本机用户标识：$($privateLeaks -join '; ')"
  }
  if ($LASTEXITCODE -notin 0, 1) { throw 'Issue #86 隐私扫描执行失败。' }
}

git diff --check
if ($LASTEXITCODE -ne 0) { throw "Issue #86 Git 空白检查失败：$LASTEXITCODE" }
```

预期：四 crate 测试与 Clippy 全绿、`rustfmt` 退出码为 `0`、CI 合同 `35/35`、`release_audit=current`、仓库政策违规数为 `0`，实际差异全部位于批准的二十四路径内；`source-index.yml` 只替换 `check_env_conflicts` 的两行副作用和 feature 归属，生产实现不含写环境、文件、网络、线程、子进程、UI 或 `unsafe`，系统入口精确调用一次 `std::env::vars_os()`。

## Issue #81 版本与启动意图本地轻量验证

在仓库根目录、分支 `codex/issue-81-gate-5-version-startup` 执行。Git 时间只使用系统默认本机时间；不得设置 `GIT_AUTHOR_DATE` 或 `GIT_COMMITTER_DATE`。

```powershell
$ErrorActionPreference = 'Stop'
Get-Date -Format 'yyyy-MM-dd HH:mm:ss.fff zzz'

cargo test --locked --offline --ignore-rust-version -p inputcodex-domain -p inputcodex-application -p inputcodex-platform -p inputcodex-parity
if ($LASTEXITCODE -ne 0) { throw "Issue #81 四 crate 测试失败：$LASTEXITCODE" }

cargo clippy --locked --offline --ignore-rust-version -p inputcodex-domain -p inputcodex-application -p inputcodex-platform -p inputcodex-parity --all-targets -- -D warnings
if ($LASTEXITCODE -ne 0) { throw "Issue #81 四 crate Clippy 失败：$LASTEXITCODE" }

cargo fmt --all -- --check
if ($LASTEXITCODE -ne 0) { throw "Issue #81 rustfmt 检查失败：$LASTEXITCODE" }

pwsh -NoProfile -File scripts/ci/Test-CiScripts.ps1
if ($LASTEXITCODE -ne 0) { throw "Issue #81 CI 合同失败：$LASTEXITCODE" }

pwsh -NoProfile -File scripts/ci/Verify-ReleaseAuditGate.ps1 -RepositoryRoot .
if ($LASTEXITCODE -ne 0) { throw "Issue #81 Release Audit 失败：$LASTEXITCODE" }

pwsh -NoProfile -File scripts/ci/Verify-RepositoryPolicy.ps1 -RepositoryRoot .
if ($LASTEXITCODE -ne 0) { throw "Issue #81 Repository Policy 失败：$LASTEXITCODE" }
```

范围、哈希与禁止能力验证：

```powershell
$candidate = [string[]]@(
  'AGENTS.md',
  'CONTEXT.md',
  'README.md',
  'build.md',
  'crates/inputcodex-application/src/lib.rs',
  'crates/inputcodex-application/src/version_startup.rs',
  'crates/inputcodex-application/tests/version_startup.rs',
  'crates/inputcodex-domain/src/lib.rs',
  'crates/inputcodex-domain/src/version_startup.rs',
  'crates/inputcodex-domain/tests/version_startup.rs',
  'crates/inputcodex-parity/tests/catalog_repository.rs',
  'crates/inputcodex-platform/src/lib.rs',
  'crates/inputcodex-platform/src/version_startup.rs',
  'crates/inputcodex-platform/tests/version_startup.rs',
  'docs/plans/2026-07-28-issue-81-gate-5-version-startup.md',
  'docs/plans/PROJECT-MASTER-PLAN.md',
  'docs/plans/sessions/2026-07-28-issue-81-gate-5-version-startup.md',
  'docs/reports/issue-81-gate-5-version-startup.md',
  'docs/workflows/2026-07-28-issue-81-gate-5-version-startup-runtime.md',
  'err.md',
  'parity/README.md',
  'parity/contracts/foundation-platform.yml',
  'parity/features/foundation-platform.yml'
)
[Array]::Sort($candidate, [StringComparer]::Ordinal)
$payload = [string]::Join("`n", $candidate) + "`n"
$scopeHash = [Convert]::ToHexString(
  [Security.Cryptography.SHA256]::HashData(
    [Text.UTF8Encoding]::new($false).GetBytes($payload)
  )
).ToLowerInvariant()
if ($candidate.Count -ne 23) { throw "Issue #81 路径数量漂移：$($candidate.Count)" }
if ($scopeHash -ne 'c1ef2c00a445dd2bd60dc5f5b375cb27d1e467a3d457d7eb53b7ec82a304aafe') {
  throw "Issue #81 scope_hash 漂移：sha256:$scopeHash"
}

$changed = @(
  git -c core.quotePath=false diff --name-only origin/main...HEAD
  git -c core.quotePath=false diff --name-only
  git -c core.quotePath=false ls-files --others --exclude-standard
) | Where-Object { $_ } | Sort-Object -Unique
$outside = @($changed | Where-Object { $_ -notin $candidate })
if ($outside.Count -ne 0) { throw "Issue #81 越界路径：$($outside -join ', ')" }

$protected = @(
  git -c core.quotePath=false diff --name-only origin/main...HEAD -- Cargo.toml Cargo.lock parity/features/source-index.yml
  git -c core.quotePath=false diff --name-only -- Cargo.toml Cargo.lock parity/features/source-index.yml
) | Where-Object { $_ } | Sort-Object -Unique
if ($protected.Count -ne 0) { throw "Issue #81 修改受保护路径：$($protected -join ', ')" }

$production = @(
  'crates/inputcodex-domain/src/version_startup.rs',
  'crates/inputcodex-application/src/version_startup.rs',
  'crates/inputcodex-platform/src/version_startup.rs'
)
$legacy = @(rg -n --fixed-strings 'CODEX_PLUS_SHOW_UPDATE' $production 2>$null)
if ($LASTEXITCODE -eq 0 -and $legacy.Count -ne 0) {
  throw "Issue #81 命中旧启动变量：$($legacy -join '; ')"
}
if ($LASTEXITCODE -notin 0, 1) { throw 'Issue #81 旧变量扫描执行失败。' }

$forbiddenProduct = @(
  rg -n 'std::fs|File::|OpenOptions|fs::|reqwest|ureq|TcpStream|UdpSocket|std::thread|thread::spawn|std::process::Command|tokio::spawn|unsafe\s*\{|iced|tauri|webview' $production 2>$null
)
if ($LASTEXITCODE -eq 0 -and $forbiddenProduct.Count -ne 0) {
  throw "Issue #81 命中禁止运行能力：$($forbiddenProduct -join '; ')"
}
if ($LASTEXITCODE -notin 0, 1) { throw 'Issue #81 禁止能力扫描执行失败。' }

$ownerName = [Environment]::UserName
if (-not [string]::IsNullOrWhiteSpace($ownerName)) {
  $privateLeaks = @(rg -n --fixed-strings $ownerName crates parity docs/reports/issue-81-gate-5-version-startup.md 2>$null)
  if ($LASTEXITCODE -eq 0 -and $privateLeaks.Count -ne 0) {
    throw "Issue #81 泄露本机用户标识：$($privateLeaks -join '; ')"
  }
  if ($LASTEXITCODE -notin 0, 1) { throw 'Issue #81 隐私扫描执行失败。' }
}

git diff --check
if ($LASTEXITCODE -ne 0) { throw "Issue #81 Git 空白检查失败：$LASTEXITCODE" }
```

预期：四 crate 测试与 Clippy 全绿、`rustfmt` 退出码为 `0`、CI 合同 `35/35`、`release_audit=current`、仓库政策违规数为 `0`，实际差异全部位于批准的二十三路径内，`Cargo.toml`、`Cargo.lock` 与 `source-index.yml` 保持未修改，生产实现不含旧变量、UI、网络、读写、shell、线程或 `unsafe`。

## Issue #78 应用概览只读事实本地轻量验证

在仓库根目录、分支 `codex/issue-78-gate-5-application-overview` 执行。Git 时间只使用系统默认本机时间；不得设置 `GIT_AUTHOR_DATE` 或 `GIT_COMMITTER_DATE`。

```powershell
$ErrorActionPreference = 'Stop'

cargo test --locked --offline --ignore-rust-version -p inputcodex-domain -p inputcodex-application -p inputcodex-platform -p inputcodex-parity
if ($LASTEXITCODE -ne 0) { throw "Issue #78 四 crate 测试失败：$LASTEXITCODE" }

cargo clippy --locked --offline --ignore-rust-version -p inputcodex-domain -p inputcodex-application -p inputcodex-platform -p inputcodex-parity --all-targets -- -D warnings
if ($LASTEXITCODE -ne 0) { throw "Issue #78 四 crate Clippy 失败：$LASTEXITCODE" }

cargo fmt --all -- --check
if ($LASTEXITCODE -ne 0) { throw "Issue #78 rustfmt 检查失败：$LASTEXITCODE" }

pwsh -NoProfile -File scripts/ci/Test-CiScripts.ps1
if ($LASTEXITCODE -ne 0) { throw "Issue #78 CI 合同失败：$LASTEXITCODE" }

pwsh -NoProfile -File scripts/ci/Verify-ReleaseAuditGate.ps1 -RepositoryRoot .
if ($LASTEXITCODE -ne 0) { throw "Issue #78 Release Audit 失败：$LASTEXITCODE" }

pwsh -NoProfile -File scripts/ci/Verify-RepositoryPolicy.ps1 -RepositoryRoot .
if ($LASTEXITCODE -ne 0) { throw "Issue #78 Repository Policy 失败：$LASTEXITCODE" }
```

候选路径、规范化规则和哈希复算以 `docs/workflows/2026-07-27-issue-78-gate-5-application-overview-runtime.md` 的 `$candidate` 区块为准。实际差异必须全部属于该集合：

```powershell
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
    throw "Issue #78 泄露本机用户标识：$($privateLeaks -join '; ')"
  }
  if ($LASTEXITCODE -notin 0, 1) { throw 'Issue #78 隐私扫描执行失败。' }
}

$forbiddenProduct = @(
  rg -n 'std::process::Command|tokio::spawn|thread::spawn|reqwest|TcpStream|UdpSocket|File::create|OpenOptions|fs::write|create_dir|unsafe\s*\{' crates/inputcodex-domain/src/application_overview.rs crates/inputcodex-application/src/application_overview.rs crates/inputcodex-platform/src/application_overview.rs crates/inputcodex-platform/src/application_overview 2>$null
)
if ($LASTEXITCODE -eq 0 -and $forbiddenProduct.Count -ne 0) {
  throw "Issue #78 命中禁止运行能力：$($forbiddenProduct -join '; ')"
}
if ($LASTEXITCODE -notin 0, 1) { throw 'Issue #78 禁止能力扫描执行失败。' }

$historyDependency = @(
  rg -n 'latest-status\.json|LaunchHistoryRecord|StatusStore' crates/inputcodex-domain/src/application_overview.rs crates/inputcodex-application/src/application_overview.rs crates/inputcodex-platform/src/application_overview.rs crates/inputcodex-platform/src/application_overview 2>$null
)
if ($LASTEXITCODE -eq 0 -and $historyDependency.Count -ne 0) {
  throw "Issue #78 命中历史状态隐藏依赖：$($historyDependency -join '; ')"
}
if ($LASTEXITCODE -notin 0, 1) { throw 'Issue #78 历史依赖扫描执行失败。' }

git diff --check
if ($LASTEXITCODE -ne 0) { throw "Issue #78 Git 空白检查失败：$LASTEXITCODE" }
```

预期：四 crate 测试与 Clippy 全绿、`rustfmt` 退出码为 `0`、CI 合同 `35/35`、`release_audit=current`、仓库政策违规数为 `0`，所有变更均位于批准的二十九路径内，产品实现不泄露私人路径且不包含历史状态、写入、网络、shell、线程或 `unsafe`。

## Issue #75 平台路径迁移本地轻量验证

Issue `#75` 本机只验证四个相关 crate、现有治理脚本、精确范围与隐私边界；禁止运行 Iced、完整 Workspace 重型构建、Release 打包或真实性能测量。Windows/macOS 真实编译与非 required observation 由公开仓库标准 GitHub-hosted Runner 完成。

```powershell
$ErrorActionPreference = 'Stop'
Get-Date -Format 'yyyy-MM-dd HH:mm:ss.fff zzz'

cargo test --locked --offline --ignore-rust-version -p inputcodex-domain -p inputcodex-application -p inputcodex-platform -p inputcodex-parity
cargo clippy --locked --offline --ignore-rust-version -p inputcodex-domain -p inputcodex-application -p inputcodex-platform -p inputcodex-parity --all-targets -- -D warnings
cargo fmt --all -- --check

pwsh -NoProfile -File scripts/ci/Test-CiScripts.ps1
pwsh -NoProfile -File scripts/ci/Verify-ReleaseAuditGate.ps1 -RepositoryRoot .
pwsh -NoProfile -File scripts/ci/Verify-RepositoryPolicy.ps1 -RepositoryRoot .

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
$scopeHash = [Convert]::ToHexString(
  [Security.Cryptography.SHA256]::HashData([Text.UTF8Encoding]::new($false).GetBytes($payload))
).ToLowerInvariant()
if ($sorted.Count -ne 30) { throw "Issue #75 路径数量漂移：$($sorted.Count)" }
if ($scopeHash -ne 'ae5e0f5143355feee9b280da7c44fdd5cdf759ec2ae71fc69167040bf302cb37') {
  throw "Issue #75 scope_hash 漂移：sha256:$scopeHash"
}

$changed = @(
  git diff --name-only origin/main...HEAD
  git diff --name-only
  git ls-files --others --exclude-standard
) | Where-Object { $_ } | Sort-Object -Unique
$outside = @($changed | Where-Object { $_ -notin $sorted })
if ($outside.Count -ne 0) { throw "Issue #75 越界路径：$($outside -join ', ')" }
if ($changed.Count -ne 30) { throw "Issue #75 最终差异应精确覆盖 30 路径，实际为 $($changed.Count)" }

$privateLeaks = @(rg -n --fixed-strings 'dashuai' crates parity 2>$null)
if ($LASTEXITCODE -eq 0 -and $privateLeaks.Count -ne 0) {
  throw "Issue #75 产品或 Parity 表面泄露本机用户标识：$($privateLeaks -join '; ')"
}
if ($LASTEXITCODE -notin 0, 1) { throw 'Issue #75 隐私扫描执行失败。' }

git diff --check
Write-Output "ISSUE75_LOCAL_VERIFY_OK scope_hash=sha256:$scopeHash changed=$($changed.Count)"
```

预期：四个相关 crate 测试与 Clippy 全绿；`Test-CiScripts.ps1`、Release Audit 和 Repository Policy 通过；范围精确为 `30` 路径且无本机用户标识泄露；`git diff --check` 退出码为 `0`。

## Issue #63 性能预算 Observation 本地轻量验证

Issue `#63` 本机只运行 PowerShell 合同、仓库政策、精确范围和空白检查；不得在项目所有者机器执行完整 Rust Workspace、桌面 Release 或真实性能采集。Windows/macOS 真实 observation 由公开仓库 GitHub-hosted Runner 完成。

```powershell
$baseline = '15e91708b41548f523e26ede4c7ca4de41badf77'
$approvedPaths = @(
  '.github/workflows/performance-baseline.yml'
  'AGENTS.md'
  'build.md'
  'docs/plans/2026-07-26-issue-63-performance-budget-ci-observation.md'
  'docs/plans/PROJECT-MASTER-PLAN.md'
  'docs/plans/sessions/2026-07-26-issue-63-performance-budget-ci-observation.md'
  'docs/reports/issue-63-performance-budget-ci-observation.md'
  'docs/workflows/2026-07-26-issue-63-performance-budget-ci-observation-runtime.md'
  'err.md'
  'README.md'
  'scripts/ci/Test-CiScripts.ps1'
  'scripts/performance/Invoke-InputcodexBudgetObservation.ps1'
  'scripts/performance/Test-InputcodexBudgetObservation.ps1'
) | Sort-Object

$branch = (git branch --show-current).Trim()
if ($branch -ne 'codex/issue-63-performance-budget-ci-observation') {
  throw "Issue #63 当前分支不正确：$branch"
}

$committed = @(git diff --name-only "$baseline...HEAD")
$unstaged = @(git diff --name-only)
$staged = @(git diff --cached --name-only)
$untracked = @(git ls-files --others --exclude-standard)
$actualPaths = @($committed + $unstaged + $staged + $untracked | Where-Object { $_ } | Sort-Object -Unique)
$unexpectedPaths = @($actualPaths | Where-Object { $_ -notin $approvedPaths })
$missingPaths = @($approvedPaths | Where-Object { $_ -notin $actualPaths })
if ($unexpectedPaths.Count -ne 0 -or $missingPaths.Count -ne 0 -or $actualPaths.Count -ne 13) {
  [pscustomobject]@{
    actual = $actualPaths -join ', '
    unexpected = $unexpectedPaths -join ', '
    missing = $missingPaths -join ', '
  } | Format-List
  throw 'Issue #63 实际差异不是批准的精确十三路径。'
}

$scopeText = ($approvedPaths -join "`n") + "`n"
$scopeBytes = [System.Text.UTF8Encoding]::new($false).GetBytes($scopeText)
$scopeHash = [Convert]::ToHexString([System.Security.Cryptography.SHA256]::HashData($scopeBytes)).ToLowerInvariant()
if ($scopeHash -ne 'd5eb57c1b93dc2b7acc47ba78c8f514af2a2c98e8661df389774713a7b47d8dc') {
  throw "Issue #63 scope_hash 漂移：$scopeHash"
}

function Get-NormalizedTextSha256 {
  param([Parameter(Mandatory)][string]$Path)

  $text = [System.IO.File]::ReadAllText($Path, [System.Text.Encoding]::UTF8)
  $normalized = [regex]::Replace($text, '\r\n?', "`n")
  $bytes = [System.Text.UTF8Encoding]::new($false).GetBytes($normalized)
  'sha256:' + [Convert]::ToHexString([System.Security.Cryptography.SHA256]::HashData($bytes)).ToLowerInvariant()
}

$budgetPath = 'benchmarks/budgets/issue-59-approved-observation.json'
$budgetHash = Get-NormalizedTextSha256 -Path $budgetPath
if ($budgetHash -ne 'sha256:be07138908cd411925db963718b71062060f4fd4a50b910ab5d5f25f88d4ebe5') {
  throw "批准预算哈希漂移：$budgetHash"
}

pwsh -NoProfile -File scripts/performance/Test-InputcodexBudgetObservation.ps1 -RepositoryRoot .
if ($LASTEXITCODE -ne 0) { throw '预算观察器测试失败。' }

pwsh -NoProfile -File scripts/ci/Test-CiScripts.ps1
if ($LASTEXITCODE -ne 0) { throw 'CI 合同测试失败。' }

pwsh -NoProfile -File scripts/ci/Verify-RepositoryPolicy.ps1 -RepositoryRoot .
if ($LASTEXITCODE -ne 0) { throw '仓库政策验证失败。' }

git diff --check
if ($LASTEXITCODE -ne 0) { throw 'git diff --check 失败。' }
```

期望输出包含：

```text
BUDGET_OBSERVATION_GREEN passed=12
CI_CONTRACT_GREEN passed=35
"ok":true
```

PR 创建后，`pull_request` 事件会自动选择 `observation`。如需在同一分支显式重放，可使用：

```powershell
gh workflow run 'Performance Baseline' --repo nonononull/inputcodex --ref codex/issue-63-performance-budget-ci-observation -f mode=observation
```

真实 Run 必须满足 Windows/macOS Job 成功、Step Summary 含结构化 observation JSON、成功 Artifact 数为 `0`；阈值分类不得导致失败，合同错误必须失败并仅保留七天最小诊断。

## Issue #61 性能预算数值 Closeout 本地轻量验证

Issue `#61` 只修改八份治理与状态文档，不运行完整 Workspace、桌面 Release 或真实性能采集。以下命令同时覆盖已提交、未暂存、已暂存和未跟踪路径：

```powershell
$baseline = 'e225144831a0928bfa3aaa0d169a054779005812'
$approvedPaths = @(
  'AGENTS.md'
  'build.md'
  'docs/plans/2026-07-26-issue-61-performance-budget-closeout.md'
  'docs/plans/PROJECT-MASTER-PLAN.md'
  'docs/plans/sessions/2026-07-26-issue-61-performance-budget-closeout.md'
  'docs/reports/issue-61-performance-budget-closeout.md'
  'docs/workflows/2026-07-26-issue-61-performance-budget-closeout-runtime.md'
  'README.md'
) | Sort-Object

$branch = (git branch --show-current).Trim()
if ($branch -ne 'codex/issue-61-performance-budget-closeout') {
  throw "Issue #61 当前分支不正确：$branch"
}

$committed = @(git diff --name-only "$baseline...HEAD")
$unstaged = @(git diff --name-only)
$staged = @(git diff --cached --name-only)
$untracked = @(git ls-files --others --exclude-standard)
$actualPaths = @($committed + $unstaged + $staged + $untracked | Where-Object { $_ } | Sort-Object -Unique)
$unexpectedPaths = @($actualPaths | Where-Object { $_ -notin $approvedPaths })
$missingPaths = @($approvedPaths | Where-Object { $_ -notin $actualPaths })
if ($unexpectedPaths.Count -ne 0 -or $missingPaths.Count -ne 0 -or $actualPaths.Count -ne 8) {
  [pscustomobject]@{
    actual = $actualPaths -join ', '
    unexpected = $unexpectedPaths -join ', '
    missing = $missingPaths -join ', '
  } | Format-List
  throw 'Issue #61 实际差异不是批准的精确八路径。'
}

$scopeText = ($approvedPaths -join "`n") + "`n"
$scopeBytes = [System.Text.UTF8Encoding]::new($false).GetBytes($scopeText)
$scopeHash = [Convert]::ToHexString([System.Security.Cryptography.SHA256]::HashData($scopeBytes)).ToLowerInvariant()
if ($scopeHash -ne 'dafe55bfc38c38782558c1577215d227ac8c83b7110735c4ddd58b48d66264b5') {
  throw "Issue #61 scope_hash 漂移：$scopeHash"
}

$requiredContent = @{
  'AGENTS.md' = @(
    'Issue `#59` / PR `#60` 已完成预算数值交付'
    'e225144831a0928bfa3aaa0d169a054779005812'
    '30194897171'
    'Issue `#61` 是 Issue `#59` / PR `#60` 稳定事实的八路径反递归 Closeout'
  )
  'README.md' = @(
    'e225144831a0928bfa3aaa0d169a054779005812'
    '建立独立预算 CI Issue'
    'docs/reports/issue-61-performance-budget-closeout.md'
  )
  'build.md' = @(
    '十九个检查点'
    '截至 2026 年 7 月 26 日'
    'e225144831a0928bfa3aaa0d169a054779005812'
    'Issue #61 性能预算数值 Closeout 本地轻量验证'
    'BUDGET_APPROVAL_GREEN passed=10'
  )
  'docs/plans/PROJECT-MASTER-PLAN.md' = @(
    'active_task: none-awaiting-performance-budget-ci-observation'
    'performance_budget_values_merge_ref: e225144831a0928bfa3aaa0d169a054779005812'
    'performance_budget_values_closeout_issue_ref: https://github.com/nonononull/inputcodex/issues/61'
    '30194897166'
  )
  'docs/plans/2026-07-26-issue-61-performance-budget-closeout.md' = @(
    'scope_hash: sha256:dafe55bfc38c38782558c1577215d227ac8c83b7110735c4ddd58b48d66264b5'
    '本 Closeout PR 合并后不得再为同一状态创建二次 Closeout'
  )
  'docs/plans/sessions/2026-07-26-issue-61-performance-budget-closeout.md' = @(
    'approved_scope_ref: https://github.com/nonononull/inputcodex/issues/61#issuecomment-5082819125'
    'merge_ref: owner-authorization-required-for-final-head'
  )
  'docs/reports/issue-61-performance-budget-closeout.md' = @(
    'BUDGET_APPROVAL_GREEN passed=10'
    'budget_ci_enabled=false'
    '本 Closeout 合并后不需要再创建同类 Closeout'
  )
  'docs/workflows/2026-07-26-issue-61-performance-budget-closeout-runtime.md' = @(
    'workflow_status: immutable-execution-contract'
    '最终 Squash Merge 必须由项目所有者针对最终 Head 单独授权'
  )
}

foreach ($entry in $requiredContent.GetEnumerator()) {
  $text = Get-Content -LiteralPath $entry.Key -Raw
  foreach ($required in $entry.Value) {
    if (-not $text.Contains($required)) {
      throw "Issue #61 稳定事实缺失：$($entry.Key) -> $required"
    }
  }
}

$longTermFiles = @('AGENTS.md', 'README.md', 'docs/plans/PROJECT-MASTER-PLAN.md')
$forbiddenPatterns = @(
  '当前仍待 Issue `#59` 的 PR、Review/CI 与 Squash Merge'
  '完成 Issue `#59` 的 Fresh 验证、非 Draft PR、Review/CI'
  'active_task: issue-59-epyc-7763-fixed-remeasurement'
  'active_pr_ref: none-awaiting-issue-59-measurement-pr'
  '当前仅余 PR、Review/CI、Final Head 授权与 Squash Merge'
)
foreach ($file in $longTermFiles) {
  $text = Get-Content -LiteralPath $file -Raw
  foreach ($pattern in $forbiddenPatterns) {
    if ($text.Contains($pattern)) {
      throw "Issue #61 仍含过期待合并状态：$file -> $pattern"
    }
  }
}

$placeholderFiles = @($approvedPaths | Where-Object { $_ -ne 'build.md' })
foreach ($file in $placeholderFiles) {
  $text = Get-Content -LiteralPath $file -Raw
  if ($text -match '<pending>|\bTBD\b|__[A-Z0-9_]+__') {
    throw "Issue #61 文件仍含占位符：$file"
  }
}

function Get-NormalizedTextSha256 {
  param([Parameter(Mandatory = $true)][string]$Path)
  $text = [System.IO.File]::ReadAllText((Resolve-Path -LiteralPath $Path).Path)
  $normalized = $text.Replace("`r`n", "`n").Replace("`r", "`n")
  $bytes = [System.Text.UTF8Encoding]::new($false).GetBytes($normalized)
  return 'sha256:' + [Convert]::ToHexString([System.Security.Cryptography.SHA256]::HashData($bytes)).ToLowerInvariant()
}

$budgetHash = Get-NormalizedTextSha256 -Path 'benchmarks/budgets/issue-59-approved-observation.json'
if ($budgetHash -ne 'sha256:be07138908cd411925db963718b71062060f4fd4a50b910ab5d5f25f88d4ebe5') {
  throw "Issue #59 已批准预算 JSON 漂移：$budgetHash"
}

pwsh -NoProfile -File scripts/performance/Test-InputcodexBaseline.ps1 -RepositoryRoot . -Mode Evidence
if ($LASTEXITCODE -ne 0) { throw '性能 Evidence 验证失败。' }
pwsh -NoProfile -File scripts/performance/Test-InputcodexBudgetApproval.ps1 -RepositoryRoot .
if ($LASTEXITCODE -ne 0) { throw '性能预算数值验证失败。' }
pwsh -NoProfile -File scripts/ci/Test-CiScripts.ps1
if ($LASTEXITCODE -ne 0) { throw 'CI 合同验证失败。' }
pwsh -NoProfile -File scripts/ci/Verify-RepositoryPolicy.ps1 -RepositoryRoot .
if ($LASTEXITCODE -ne 0) { throw '仓库政策验证失败。' }
git diff --check
if ($LASTEXITCODE -ne 0) { throw 'git diff --check 失败。' }
Write-Output "ISSUE_61_CLOSEOUT_GREEN paths=$($actualPaths.Count) scope_hash=sha256:$scopeHash"
```

## Issue #52 性能预算 Discovery Closeout 本地轻量验证

Issue `#52` 只修改八份治理与状态文档，不运行完整 Workspace、桌面 Release 或真实性能采集。以下命令同时覆盖已提交、未暂存、已暂存和未跟踪路径：

```powershell
$baseline = 'fea8824c652665df710a7e6ef941854060eb6e1f'
$approvedPaths = @(
  'AGENTS.md'
  'README.md'
  'build.md'
  'docs/plans/PROJECT-MASTER-PLAN.md'
  'docs/plans/2026-07-25-issue-52-performance-budget-closeout.md'
  'docs/plans/sessions/2026-07-25-issue-52-performance-budget-closeout.md'
  'docs/reports/issue-52-performance-budget-closeout.md'
  'docs/workflows/2026-07-25-issue-52-performance-budget-closeout-runtime.md'
) | Sort-Object

$branch = (git branch --show-current).Trim()
if ($branch -ne 'codex/issue-52-performance-budget-closeout') {
  throw "Issue #52 当前分支不正确：$branch"
}

$committed = @(git diff --name-only "$baseline...HEAD")
$unstaged = @(git diff --name-only)
$staged = @(git diff --cached --name-only)
$untracked = @(git ls-files --others --exclude-standard)
$actualPaths = @($committed + $unstaged + $staged + $untracked | Where-Object { $_ } | Sort-Object -Unique)

$pathDiff = @(Compare-Object -ReferenceObject $approvedPaths -DifferenceObject $actualPaths)
if ($pathDiff.Count -ne 0) {
  $pathDiff | Format-Table -AutoSize
  throw 'Issue #52 实际差异不是批准的八路径。'
}

$scopeText = ($approvedPaths -join "`n") + "`n"
$scopeBytes = [System.Text.UTF8Encoding]::new($false).GetBytes($scopeText)
$scopeHash = [Convert]::ToHexString([System.Security.Cryptography.SHA256]::HashData($scopeBytes)).ToLowerInvariant()
if ($scopeHash -ne 'af1cfffe1e72b847b212874ab6348bb6f375c54a43564cc702abb24145efb513') {
  throw "Issue #52 scope_hash 漂移：$scopeHash"
}

$requiredFacts = @{
  'AGENTS.md' = @('Issue `#50` / PR `#51` 已完成', 'fea8824c652665df710a7e6ef941854060eb6e1f', '30175592979', '独立性能复测与数值批准 Issue')
  'README.md' = @('Issue `#50` / PR `#51` 已以单父 Squash 提交', '30175592979', 'Issue `#50` 已按 `COMPLETED` 关闭')
  'docs/plans/PROJECT-MASTER-PLAN.md' = @('active_task: none-awaiting-performance-remeasurement-and-budget-approval', 'fea8824c652665df710a7e6ef941854060eb6e1f', 'next_legal_gate: 创建独立性能复测与数值批准 Issue')
  'docs/reports/issue-52-performance-budget-closeout.md' = @('30175592979', '单父', '反递归')
}
foreach ($entry in $requiredFacts.GetEnumerator()) {
  $content = Get-Content -LiteralPath $entry.Key -Raw
  foreach ($fact in $entry.Value) {
    if (-not $content.Contains($fact)) {
      throw "Issue #52 缺少稳定事实：$($entry.Key) -> $fact"
    }
  }
}

$stalePatterns = @(
  'active_task: issue-50-performance-budget-discovery'
  '处于九路径方法冻结、Review/CI、对话闭环和项目所有者单独 Squash Merge 决策阶段'
  '只允许在 PR #51 已批准九路径'
  'active_branch_ref: codex/issue-50-performance-budget-discovery'
  'active_pr_ref: https://github.com/nonononull/inputcodex/pull/51'
  '完成 Issue `#50` 的 ADR、Discovery 报告、长期状态同步、Review/CI 与 Squash Merge'
)
$longTermFiles = @('AGENTS.md', 'README.md', 'docs/plans/PROJECT-MASTER-PLAN.md')
foreach ($pattern in $stalePatterns) {
  $matches = @(Select-String -LiteralPath $longTermFiles -SimpleMatch -Pattern $pattern)
  if ($matches.Count -ne 0) {
    $matches | Format-Table Path, LineNumber, Line -AutoSize
    throw "Issue #52 长期入口仍包含过期事实：$pattern"
  }
}

$placeholderMatches = @(Select-String -LiteralPath 'docs/reports/issue-52-performance-budget-closeout.md' -Pattern '\b(TBD|TODO|FIXME|WIP)\b')
if ($placeholderMatches.Count -ne 0) {
  $placeholderMatches | Format-Table Path, LineNumber, Line -AutoSize
  throw 'Issue #52 Closeout 报告仍含占位符。'
}

pwsh -NoProfile -File scripts/performance/Test-InputcodexBaseline.ps1 -RepositoryRoot . -Mode Evidence
if ($LASTEXITCODE -ne 0) { throw 'Issue #52 性能 Evidence 验证失败。' }

pwsh -NoProfile -File scripts/ci/Test-CiScripts.ps1
if ($LASTEXITCODE -ne 0) { throw 'Issue #52 CI 合同验证失败。' }

pwsh -NoProfile -File scripts/ci/Verify-RepositoryPolicy.ps1 -RepositoryRoot .
if ($LASTEXITCODE -ne 0) { throw 'Issue #52 仓库政策验证失败。' }

git diff --check
if ($LASTEXITCODE -ne 0) { throw 'Issue #52 未暂存差异空白检查失败。' }

git diff --cached --check
if ($LASTEXITCODE -ne 0) { throw 'Issue #52 已暂存差异空白检查失败。' }

Write-Output "ISSUE52_CLOSEOUT_VERIFY_OK scope_hash=sha256:$scopeHash"
```

## Issue #50 性能预算 Discovery 本地轻量验证

Issue `#50` 只修改九份文档，禁止运行完整 Workspace、桌面 Release 或真实性能采集。以下命令同时覆盖已提交、未暂存、已暂存和未跟踪路径：

```powershell
$baseline = 'fd9db9ca1c150b7db34dda8acc09b6f0cc357a17'
$approvedPaths = @(
  'AGENTS.md'
  'README.md'
  'build.md'
  'docs/adr/0004-performance-budget-policy.md'
  'docs/plans/PROJECT-MASTER-PLAN.md'
  'docs/plans/2026-07-25-issue-50-performance-budget-discovery.md'
  'docs/plans/sessions/2026-07-25-issue-50-performance-budget-discovery.md'
  'docs/reports/issue-50-performance-budget-discovery.md'
  'docs/workflows/2026-07-25-issue-50-performance-budget-discovery-runtime.md'
) | Sort-Object

$committed = @(git diff --name-only "$baseline...HEAD")
$unstaged = @(git diff --name-only)
$staged = @(git diff --cached --name-only)
$untracked = @(git ls-files --others --exclude-standard)
$actualPaths = @($committed + $unstaged + $staged + $untracked | Where-Object { $_ } | Sort-Object -Unique)

$pathDiff = @(Compare-Object -ReferenceObject $approvedPaths -DifferenceObject $actualPaths)
if ($pathDiff.Count -ne 0) {
  $pathDiff | Format-Table -AutoSize
  throw 'Issue #50 实际差异不是批准的九路径。'
}

$scopeText = ($approvedPaths -join "`n") + "`n"
$scopeBytes = [System.Text.UTF8Encoding]::new($false).GetBytes($scopeText)
$scopeHash = [Convert]::ToHexString([System.Security.Cryptography.SHA256]::HashData($scopeBytes)).ToLowerInvariant()
if ($scopeHash -ne 'af1c248c46d54741f9c77ab3621cd66ccd40e3fa50698d377c788fcb0b93205f') {
  throw "Issue #50 scope_hash 漂移：$scopeHash"
}

$requiredFacts = @{
  'AGENTS.md' = @('Issue `#32` 的完成只表示性能基线', 'Issue `#50` 已冻结九路径', 'Issue `#75` 是首个 Gate 5 产品迁移切片')
  'README.md' = @('至少五次独立 Run', 'approved-observation', 'Issue `#50` Discovery 报告')
  'docs/adr/0004-performance-budget-policy.md' = @('comparable-valid', 'new-cohort-valid', 'baseline-only', 'enforced')
  'docs/plans/PROJECT-MASTER-PLAN.md' = @('fd9db9ca1c150b7db34dda8acc09b6f0cc357a17', 'Issue `#50`', 'approved-observation')
  'docs/reports/issue-50-performance-budget-discovery.md' = @('当前每个平台只有一次独立有效 Run', '方案 A', '后续互斥 Issue')
}

foreach ($entry in $requiredFacts.GetEnumerator()) {
  $content = Get-Content -LiteralPath $entry.Key -Raw
  foreach ($fact in $entry.Value) {
    if (-not $content.Contains($fact)) {
      throw "Issue #50 缺少长期事实：$($entry.Key) -> $fact"
    }
  }
}

$stalePatterns = @(
  'PR `#49` 最终 Head 仍需'
  '当前只允许最终 Evidence、Review/CI 与所有者合并决策'
  '首个目标版本为 `v1.2.41-inputcodex.1`'
)
$longTermFiles = @('AGENTS.md', 'README.md', 'docs/plans/PROJECT-MASTER-PLAN.md')
foreach ($pattern in $stalePatterns) {
  $matches = @(Select-String -LiteralPath $longTermFiles -SimpleMatch -Pattern $pattern)
  if ($matches.Count -ne 0) {
    $matches | Format-Table Path, LineNumber, Line -AutoSize
    throw "Issue #50 长期入口仍包含过期事实：$pattern"
  }
}

pwsh -NoProfile -File scripts/performance/Test-InputcodexBaseline.ps1 -RepositoryRoot . -Mode Evidence
if ($LASTEXITCODE -ne 0) { throw 'Issue #50 性能 Evidence 验证失败。' }

pwsh -NoProfile -File scripts/ci/Test-CiScripts.ps1
if ($LASTEXITCODE -ne 0) { throw 'Issue #50 CI 合同验证失败。' }

pwsh -NoProfile -File scripts/ci/Verify-RepositoryPolicy.ps1 -RepositoryRoot .
if ($LASTEXITCODE -ne 0) { throw 'Issue #50 仓库政策验证失败。' }

git diff --check
if ($LASTEXITCODE -ne 0) { throw 'Issue #50 未暂存差异空白检查失败。' }

git diff --cached --check
if ($LASTEXITCODE -ne 0) { throw 'Issue #50 已暂存差异空白检查失败。' }

Write-Output "ISSUE50_DISCOVERY_VERIFY_OK scope_hash=sha256:$scopeHash"
```

## Issue #32 独立性能基线本地轻量验证

Issue `#32` 只允许验证隔离测量合同和原始样本结构；Windows/macOS 真实测量、Iced 桌面运行和 Release 构建只在公开 GitHub-hosted runner 上运行。不要在项目所有者本机执行完整基线、上游/半成品或全量 Workspace 编译。

```powershell
cargo test --manifest-path benchmarks/inputcodex-baseline/Cargo.toml --locked --offline
if ($LASTEXITCODE -ne 0) { throw 'Issue #32 隔离测量工程测试失败。' }

pwsh -NoProfile -File scripts/performance/Test-InputcodexBaseline.ps1 -RepositoryRoot . -Mode Evidence
if ($LASTEXITCODE -ne 0) { throw 'Issue #32 性能证据验证失败。' }

pwsh -NoProfile -File scripts/ci/Test-CiScripts.ps1
if ($LASTEXITCODE -ne 0) { throw 'Issue #32 CI 合同验证失败。' }

pwsh -NoProfile -File scripts/ci/Verify-RepositoryPolicy.ps1 -RepositoryRoot .
if ($LASTEXITCODE -ne 0) { throw 'Issue #32 仓库政策验证失败。' }

git diff --check
if ($LASTEXITCODE -ne 0) { throw 'Issue #32 差异空白检查失败。' }
```

`benchmarks/results/issue-32/` 当前固定引用 Performance Run `30170535534` Attempt `1`、测量提交 `42bc2e9ce7cf2e88d0602ebdc638213854793f96` 与 tree `94fc484124d1557ece1d76f27abc5ea1bc5ea592`。两个临时成功 Artifact 已在结果入库后删除且该 Run Artifact 数为 `0`；失败 Run `30170128309` 的诊断 Artifact `8622687822` 按 7 天合同保留，禁止上传 `target/`。

## 环境要求

- PowerShell 7。
- Git。
- GitHub CLI `gh`，已登录 `nonononull`。
- Python 3 与 PyYAML。
- Rust `1.97.1`、`rustfmt` 与 `clippy`；本机缺少精确工具链时不得改成浮动 `stable`。

```powershell
Set-Location 'C:\Users\dashuai\Documents\inputcodex'
$ErrorActionPreference = 'Stop'
Set-StrictMode -Version Latest
```

原生 `git`、`gh`、`python` 命令后必须立即检查 `$LASTEXITCODE`。只有一行输出时使用 `@(...)` 归一化，禁止把空 stdout 当成成功证据。

## Issue #38 `v1.2.42` 目录重新审计规划 checkpoint（历史）

本节只复现提交 `1ec07928ea100fb9dfcc4948688154eb2e020198` 的八路径 Discovery/Plan checkpoint，不用于当前二十六路径实施：

```powershell
$expectedPaths = [string[]]@(
  'AGENTS.md',
  'build.md',
  'docs/plans/2026-07-25-issue-38-v1.2.42-catalog-reaudit.md',
  'docs/plans/PROJECT-MASTER-PLAN.md',
  'docs/plans/sessions/2026-07-25-issue-38-v1.2.42-catalog-reaudit.md',
  'docs/reports/issue-38-v1.2.42-catalog-reaudit-discovery.md',
  'docs/workflows/2026-07-25-issue-38-v1.2.42-catalog-reaudit-runtime.md',
  'README.md'
)

$unstaged = @(git diff --name-only --relative)
if ($LASTEXITCODE -ne 0) { throw '读取 Issue #38 未暂存路径失败。' }
$staged = @(git diff --cached --name-only --relative)
if ($LASTEXITCODE -ne 0) { throw '读取 Issue #38 已暂存路径失败。' }
$untracked = @(git ls-files --others --exclude-standard)
if ($LASTEXITCODE -ne 0) { throw '读取 Issue #38 未跟踪路径失败。' }

$actualPaths = @(
  $unstaged + $staged + $untracked |
    Where-Object { -not [string]::IsNullOrWhiteSpace($_) } |
    Sort-Object -Unique
)
$scopeDiff = @(Compare-Object -ReferenceObject ($expectedPaths | Sort-Object) -DifferenceObject $actualPaths)
if ($scopeDiff.Count -ne 0) {
  $scopeDiff | Format-Table | Out-String | Write-Host
  throw 'Issue #38 当前变更并集不等于八路径规划范围。'
}

$payload = (($expectedPaths | Sort-Object) -join "`n") + "`n"
$bytes = [System.Text.UTF8Encoding]::new($false).GetBytes($payload)
$scopeHash = [Convert]::ToHexString([System.Security.Cryptography.SHA256]::HashData($bytes)).ToLowerInvariant()
if ($scopeHash -ne 'c7c32b7d07f5f1b04acba9c465e1bc4bc5021228b18c438e85b40d7db5f56add') {
  throw "Issue #38 八路径 scope_hash 漂移：$scopeHash"
}

pwsh -NoProfile -File D:\Android_source\ai-growth-os\components\rules\scripts\verify-session-plan.ps1 -Path docs/plans/sessions/2026-07-25-issue-38-v1.2.42-catalog-reaudit.md
if ($LASTEXITCODE -ne 0) { throw 'Issue #38 Session Plan 验证失败。' }

pwsh -NoProfile -File scripts/ci/Verify-ReleaseAuditGate.ps1 -RepositoryRoot .
if ($LASTEXITCODE -ne 0) { throw 'Issue #38 规划阶段 Release Audit 验证失败。' }

pwsh -NoProfile -File scripts/ci/Verify-RepositoryPolicy.ps1 -RepositoryRoot .
if ($LASTEXITCODE -ne 0) { throw 'Issue #38 仓库政策验证失败。' }

git diff --check
if ($LASTEXITCODE -ne 0) { throw 'Issue #38 差异空白检查失败。' }

Write-Output "ISSUE38_PLANNING_CHECKPOINT_OK scope_hash=sha256:$scopeHash"
```

该历史 checkpoint 的 `Verify-ReleaseAuditGate.ps1` 输出应为 `status=stale-re-audit-required`；当前实施验证使用下一节。

## Issue #38 二十六路径实施验证

项目所有者已批准二十六路径与 `sha256:a384353e947bcb9d95b51ac5ccce49ef9558ca34580c130307a64b6d868819af`。本节把规划提交后的已提交差异、暂存差异、未暂存差异和未跟踪文件取并集，因此在 GREEN 提交前后均可复用：

```powershell
$baseline = '1ec07928ea100fb9dfcc4948688154eb2e020198'
$expectedPaths = [string[]]@(
  'AGENTS.md',
  'build.md',
  'crates/inputcodex-parity/tests/catalog_repository.rs',
  'docs/plans/2026-07-25-issue-38-v1.2.42-catalog-reaudit.md',
  'docs/plans/PROJECT-MASTER-PLAN.md',
  'docs/plans/sessions/2026-07-25-issue-38-v1.2.42-catalog-reaudit.md',
  'docs/reports/issue-38-v1.2.42-catalog-reaudit-discovery.md',
  'docs/workflows/2026-07-25-issue-38-v1.2.42-catalog-reaudit-runtime.md',
  'parity/contracts/foundation-platform.yml',
  'parity/contracts/plugin-script.yml',
  'parity/contracts/provider-network.yml',
  'parity/contracts/remote-install.yml',
  'parity/contracts/session-data.yml',
  'parity/features/foundation-platform.yml',
  'parity/features/plugin-script.yml',
  'parity/features/provider-network.yml',
  'parity/features/remote-install.yml',
  'parity/features/session-data.yml',
  'parity/features/source-index.yml',
  'parity/fixtures/feature.plugin-script.dream-skin-library/baseline.yml',
  'parity/fixtures/feature.plugin-script.dream-skin-library/manifest.yml',
  'parity/fixtures/feature.session-data.local-session-management/baseline.yml',
  'parity/fixtures/feature.session-data.local-session-management/manifest.yml',
  'parity/README.md',
  'README.md',
  'upstream/source-lock.json'
)

$committed = @(git diff --name-only --relative "$baseline..HEAD")
if ($LASTEXITCODE -ne 0) { throw '读取 Issue #38 已提交实施路径失败。' }
$unstaged = @(git diff --name-only --relative)
if ($LASTEXITCODE -ne 0) { throw '读取 Issue #38 未暂存实施路径失败。' }
$staged = @(git diff --cached --name-only --relative)
if ($LASTEXITCODE -ne 0) { throw '读取 Issue #38 已暂存实施路径失败。' }
$untracked = @(git ls-files --others --exclude-standard)
if ($LASTEXITCODE -ne 0) { throw '读取 Issue #38 未跟踪实施路径失败。' }

$actualPaths = @(
  $committed + $unstaged + $staged + $untracked |
    Where-Object { -not [string]::IsNullOrWhiteSpace($_) } |
    Sort-Object -Unique
)
$scopeDiff = @(Compare-Object -ReferenceObject ($expectedPaths | Sort-Object) -DifferenceObject $actualPaths)
if ($scopeDiff.Count -ne 0) {
  $scopeDiff | Format-Table | Out-String | Write-Host
  throw 'Issue #38 实际变更并集不等于批准的二十六路径。'
}

$payload = (($expectedPaths | Sort-Object) -join "`n") + "`n"
$bytes = [System.Text.UTF8Encoding]::new($false).GetBytes($payload)
$scopeHash = [Convert]::ToHexString([System.Security.Cryptography.SHA256]::HashData($bytes)).ToLowerInvariant()
if ($scopeHash -ne 'a384353e947bcb9d95b51ac5ccce49ef9558ca34580c130307a64b6d868819af') {
  throw "Issue #38 二十六路径 scope_hash 漂移：$scopeHash"
}

cargo test -p inputcodex-parity --test catalog_repository --offline
if ($LASTEXITCODE -ne 0) { throw 'Issue #38 Parity 定向测试失败。' }

cargo fmt --all -- --check
if ($LASTEXITCODE -ne 0) { throw 'Issue #38 Rust 格式检查失败。' }

pwsh -NoProfile -File D:\Android_source\ai-growth-os\components\rules\scripts\verify-session-plan.ps1 -Path docs/plans/sessions/2026-07-25-issue-38-v1.2.42-catalog-reaudit.md
if ($LASTEXITCODE -ne 0) { throw 'Issue #38 Session Plan 验证失败。' }

pwsh -NoProfile -File scripts/ci/Verify-ReleaseAuditGate.ps1 -RepositoryRoot .
if ($LASTEXITCODE -ne 0) { throw 'Issue #38 Release Audit 验证失败。' }

pwsh -NoProfile -File scripts/ci/Verify-RepositoryPolicy.ps1 -RepositoryRoot .
if ($LASTEXITCODE -ne 0) { throw 'Issue #38 仓库政策验证失败。' }

git diff --check
if ($LASTEXITCODE -ne 0) { throw 'Issue #38 差异空白检查失败。' }

Write-Output "ISSUE38_IMPLEMENTATION_VERIFY_OK scope_hash=sha256:$scopeHash"
```

`Verify-ReleaseAuditGate.ps1` 必须输出 `status=current`、`requires_reaudit=false`。完整 Workspace 与 Windows/macOS/Linux 验证继续交给公开仓库的标准 GitHub-hosted runners。

## Issue #35 Release 审计基线解耦本地验证

本节只验证 `release_audit` 的结构、目录 Release 对齐和 PR 门禁合同；不得更新 `upstream/CodexPlusPlus/`、创建 `benchmarks/`、修改产品 crate、Cargo、Release、Ruleset 或 AGOS：

```powershell
pwsh -NoProfile -File scripts/ci/Test-CiScripts.ps1
if ($LASTEXITCODE -ne 0) { throw 'Issue #35 CI 门禁合同失败。' }

pwsh -NoProfile -File scripts/ci/Verify-ReleaseAuditGate.ps1 -RepositoryRoot .
if ($LASTEXITCODE -ne 0) { throw 'Issue #35 Release 审计结构失败。' }

cargo test -p inputcodex-parity --test catalog_repository --offline release_audit_显式解耦快照与功能目录审计基线
if ($LASTEXITCODE -ne 0) { throw 'Issue #35 Rust 定向审计行为失败。' }

pwsh -NoProfile -File scripts/ci/Verify-RepositoryPolicy.ps1 -RepositoryRoot .
if ($LASTEXITCODE -ne 0) { throw 'Issue #35 仓库政策失败。' }

cargo fmt --all -- --check
if ($LASTEXITCODE -ne 0) { throw 'Issue #35 Rust 格式失败。' }

git diff --check
if ($LASTEXITCODE -ne 0) { throw 'Issue #35 差异空白检查失败。' }

Write-Output 'ISSUE35_RELEASE_AUDIT_LOCAL_VERIFY_OK'
```

`release_audit.status = stale-re-audit-required` 时，本命令仍可通过结构验证；PR 上的 `release-audit` Job 才读取 base/head 变更并阻断 `benchmarks/`、`apps/`、产品 crate、`Cargo.toml` 与 `Cargo.lock`。完整 Workspace、Windows/macOS 构建与发布构建仍由标准 GitHub-hosted runners 运行。

## Issue #26 Gate 4 功能目录实现本地验证

本节验证 Issue `#26` 的完整 36 条最大写入范围、Parity 行为合同与脱敏 fixture；不得修改产品、CI、Ruleset、Release、`upstream/`、`benchmarks/` 或 AGOS：

```powershell
$baseline = '431682296f53e86de1184c732b0d4748857c9390'
$expectedBranch = 'codex/issue-26-gate-4-feature-catalog'
$scopePaths = [string[]]@(
  'AGENTS.md',
  'Cargo.lock',
  'Cargo.toml',
  'README.md',
  'build.md',
  'crates/inputcodex-parity/Cargo.toml',
  'crates/inputcodex-parity/build.md',
  'crates/inputcodex-parity/err.md',
  'crates/inputcodex-parity/src/catalog.rs',
  'crates/inputcodex-parity/src/contract.rs',
  'crates/inputcodex-parity/src/fixture.rs',
  'crates/inputcodex-parity/src/lib.rs',
  'crates/inputcodex-parity/src/validation.rs',
  'crates/inputcodex-parity/tests/catalog_repository.rs',
  'crates/inputcodex-parity/tests/catalog_schema.rs',
  'crates/inputcodex-parity/tests/contract_schema.rs',
  'crates/inputcodex-parity/tests/fixture_safety.rs',
  'docs/plans/2026-07-22-issue-26-gate-4-feature-catalog-implementation.md',
  'docs/plans/PROJECT-MASTER-PLAN.md',
  'docs/plans/sessions/2026-07-22-issue-26-gate-4-feature-catalog.md',
  'docs/reports/issue-26-gate-4-feature-catalog.md',
  'docs/workflows/2026-07-22-issue-26-gate-4-feature-catalog-runtime.md',
  'err.md',
  'parity/README.md',
  'parity/contracts/foundation-platform.yml',
  'parity/contracts/plugin-script.yml',
  'parity/contracts/provider-network.yml',
  'parity/contracts/remote-install.yml',
  'parity/contracts/session-data.yml',
  'parity/features/foundation-platform.yml',
  'parity/features/plugin-script.yml',
  'parity/features/provider-network.yml',
  'parity/features/remote-install.yml',
  'parity/features/session-data.yml',
  'parity/features/source-index.yml',
  'parity/fixtures/**'
)
$expectedScopeHash = 'e8a1cbccfc3f0026e90fcb49264de5ea69980fa2e1faa03b520d9bedaf61e772'

$branch = (git branch --show-current).Trim()
if ($LASTEXITCODE -ne 0 -or $branch -ne $expectedBranch) {
  throw "Issue #26 当前分支不正确：$branch"
}
$committedChanges = @(git -c core.quotePath=false diff --name-only "$baseline...HEAD")
if ($LASTEXITCODE -ne 0) { throw '读取 Issue #26 已提交变更路径失败。' }
$workingChanges = @(git -c core.quotePath=false diff --name-only)
if ($LASTEXITCODE -ne 0) { throw '读取 Issue #26 工作树变更路径失败。' }
$stagedChanges = @(git -c core.quotePath=false diff --cached --name-only)
if ($LASTEXITCODE -ne 0) { throw '读取 Issue #26 暂存变更路径失败。' }
$untrackedChanges = @(git -c core.quotePath=false ls-files --others --exclude-standard)
if ($LASTEXITCODE -ne 0) { throw '读取 Issue #26 未跟踪变更路径失败。' }
$changed = @($committedChanges + $workingChanges + $stagedChanges + $untrackedChanges) |
  Where-Object { $_ } |
  Sort-Object -Unique
$unexpected = @(
  foreach ($path in $changed) {
    $isAllowed = $scopePaths -contains $path -or
      $path.StartsWith('parity/fixtures/', [StringComparison]::Ordinal)
    if (-not $isAllowed) { $path }
  }
)
if ($unexpected.Count -ne 0) {
  throw "Issue #26 变更越过批准范围：$($unexpected -join ',')"
}

[Array]::Sort($scopePaths, [StringComparer]::Ordinal)
$scopeText = ($scopePaths -join "`n") + "`n"
$scopeBytes = [System.Text.UTF8Encoding]::new($false).GetBytes($scopeText)
$scopeHash = ([System.BitConverter]::ToString(
  [System.Security.Cryptography.SHA256]::HashData($scopeBytes)
)).Replace('-', '').ToLowerInvariant()
if ($scopePaths.Count -ne 36 -or $scopeHash -ne $expectedScopeHash) {
  throw "Issue #26 scope hash 不一致：count=$($scopePaths.Count) hash=$scopeHash"
}

$controlFiles = @(
  'docs/plans/2026-07-22-issue-26-gate-4-feature-catalog-implementation.md',
  'docs/plans/sessions/2026-07-22-issue-26-gate-4-feature-catalog.md',
  'docs/workflows/2026-07-22-issue-26-gate-4-feature-catalog-runtime.md',
  'docs/reports/issue-26-gate-4-feature-catalog.md'
)
$controlText = ($controlFiles | ForEach-Object {
  Get-Content -LiteralPath $_ -Raw -Encoding UTF8
}) -join "`n"
foreach ($required in @(
  'https://github.com/nonononull/inputcodex/issues/26',
  'user-message:create-issue-26-session-plan-runtime-scope-hash-2026-07-22',
  'sha256:e8a1cbccfc3f0026e90fcb49264de5ea69980fa2e1faa03b520d9bedaf61e772',
  '431682296f53e86de1184c732b0d4748857c9390',
  'v1.2.41',
  '3dafffcafb2566a1e8bce4b35671656d6adb3eda',
  '91376ee3518cb5fe5ec8eead179418f706c25870',
  'implementation_decision_ref: user-message:approve-issue-26-implementation-2026-07-22',
  'issuecomment:5047650154'
)) {
  if (-not $controlText.Contains($required)) { throw "Issue #26 控制面缺少：$required" }
}
if ($controlText -match '(?i)TODO|TBD|FIXME|待补|待定') {
  throw 'Issue #26 控制面存在未批准占位标记。'
}

$statusText = @(
  Get-Content -LiteralPath 'AGENTS.md' -Raw -Encoding UTF8
  Get-Content -LiteralPath 'README.md' -Raw -Encoding UTF8
  Get-Content -LiteralPath 'docs/plans/PROJECT-MASTER-PLAN.md' -Raw -Encoding UTF8
) -join "`n"
if ($statusText -match 'active_task:\s*2026-07-22-issue-24-gate-4-feature-performance-plan|Issue `#24` 是当前 Gate 4 规划任务') {
  throw 'Issue #26 未清除 Gate 4 规划的陈旧活动状态。'
}

$env:RUSTUP_TOOLCHAIN = '1.93.1-x86_64-pc-windows-msvc'
cargo metadata --locked --offline --no-deps --format-version 1 | Out-Null
if ($LASTEXITCODE -ne 0) { throw 'Issue #26 Cargo metadata 失败。' }
cargo fmt --all -- --check
if ($LASTEXITCODE -ne 0) { throw 'Issue #26 rustfmt 失败。' }
cargo check --locked --offline --ignore-rust-version -p inputcodex-parity
if ($LASTEXITCODE -ne 0) { throw 'Issue #26 parity check 失败。' }
cargo clippy --locked --offline --ignore-rust-version -p inputcodex-parity -- -D warnings
if ($LASTEXITCODE -ne 0) { throw 'Issue #26 Clippy 严格门禁失败。' }
cargo test --locked --offline --ignore-rust-version -p inputcodex-parity
if ($LASTEXITCODE -ne 0) { throw 'Issue #26 parity 测试失败。' }
& .\scripts\ci\Test-CiScripts.ps1
if ($LASTEXITCODE -ne 0) { throw 'Issue #26 治理合同失败。' }
& .\scripts\ci\Verify-RepositoryPolicy.ps1 -RepositoryRoot .
if ($LASTEXITCODE -ne 0) { throw 'Issue #26 真实仓库政策失败。' }
git diff --check
if ($LASTEXITCODE -ne 0) { throw 'Issue #26 工作树存在空白错误。' }
Write-Output 'ISSUE26_GATE4_FEATURE_CATALOG_IMPLEMENTATION_OK'
```

## Issue #24 Gate 4 规划本地验证

本节只验证 Gate 4 规划控制面，不创建功能矩阵数据、合同测试、性能基准或产品改动：

```powershell
$baseline = 'f470c062037042a1f7833a29cdcf216f6c0f5601'
$expectedBranch = 'codex/issue-24-gate-4-planning'
$expectedPaths = @(
  'AGENTS.md',
  'README.md',
  'build.md',
  'err.md',
  'docs/plans/PROJECT-MASTER-PLAN.md',
  'docs/plans/2026-07-22-issue-24-gate-4-feature-performance-plan.md',
  'docs/plans/sessions/2026-07-22-issue-24-gate-4-feature-performance-plan.md',
  'docs/workflows/2026-07-22-issue-24-gate-4-feature-performance-runtime.md',
  'docs/reports/issue-24-gate-4-feature-performance-plan.md'
)

$branch = (git branch --show-current).Trim()
if ($LASTEXITCODE -ne 0 -or $branch -ne $expectedBranch) {
  throw "Issue #24 当前分支不正确：$branch"
}
$changed = @(
  git -c core.quotePath=false diff --name-only $baseline
  git -c core.quotePath=false ls-files --others --exclude-standard
) | Where-Object { $_ } | Sort-Object -Unique
if ($LASTEXITCODE -ne 0) { throw '读取 Issue #24 变更路径失败。' }
$unexpected = @($changed | Where-Object { $_ -notin $expectedPaths })
$missing = @($expectedPaths | Where-Object { $_ -notin $changed })
if ($unexpected.Count -ne 0 -or $missing.Count -ne 0 -or $changed.Count -ne 9) {
  throw "Issue #24 路径不一致；越界=$($unexpected -join ',')；缺失=$($missing -join ',')；总数=$($changed.Count)"
}

$protected = @($changed | Where-Object {
  $_ -in @('Cargo.toml','Cargo.lock','rust-toolchain.toml','upstream/source-lock.json') -or
  $_ -like 'apps/*' -or $_ -like 'crates/*' -or $_ -like 'scripts/ci/*' -or
  $_ -like '.github/workflows/*' -or $_ -like 'upstream/*' -or
  $_ -like 'parity/*' -or $_ -like 'benchmarks/*' -or $_ -match '(?i)agos'
})
if ($protected.Count -ne 0) { throw "Issue #24 触及受保护路径：$($protected -join ',')" }

$controlFiles = @(
  'docs/plans/2026-07-22-issue-24-gate-4-feature-performance-plan.md',
  'docs/plans/sessions/2026-07-22-issue-24-gate-4-feature-performance-plan.md',
  'docs/workflows/2026-07-22-issue-24-gate-4-feature-performance-runtime.md',
  'docs/reports/issue-24-gate-4-feature-performance-plan.md'
)
$controlText = ($controlFiles | ForEach-Object { Get-Content -LiteralPath $_ -Raw -Encoding UTF8 }) -join "`n"
foreach ($required in @(
  'https://github.com/nonononull/inputcodex/issues/24',
  'user-message:approve-gate-4-option-2-planning-2026-07-22',
  'sha256:72e2f5d774080a55599297909600aba3c9f58470710b71db25d3690a61a1cbf0',
  'f470c062037042a1f7833a29cdcf216f6c0f5601',
  'v1.2.41',
  '91376ee3518cb5fe5ec8eead179418f706c25870'
)) {
  if (-not $controlText.Contains($required)) { throw "Issue #24 控制面缺少：$required" }
}

$statusText = @(
  Get-Content -LiteralPath 'AGENTS.md' -Raw -Encoding UTF8
  Get-Content -LiteralPath 'README.md' -Raw -Encoding UTF8
  Get-Content -LiteralPath 'docs/plans/PROJECT-MASTER-PLAN.md' -Raw -Encoding UTF8
) -join "`n"
if ($statusText -match 'active_task:\s*2026-07-22-issue-22-gate-3-closeout|Gate 4 功能目录与性能预算仍处于锁定状态|Issue `#22` 是当前独立 closeout 任务') {
  throw 'Issue #24 未清除 Gate 3 closeout 的陈旧活动状态。'
}

& .\scripts\ci\Test-CiScripts.ps1
if ($LASTEXITCODE -ne 0) { throw 'Issue #24 治理合同失败。' }
& .\scripts\ci\Verify-RepositoryPolicy.ps1 -RepositoryRoot .
if ($LASTEXITCODE -ne 0) { throw 'Issue #24 真实仓库政策失败。' }
git diff --check
if ($LASTEXITCODE -ne 0) { throw 'Issue #24 工作树存在空白错误。' }
Write-Output 'ISSUE24_GATE4_PLANNING_LOCAL_VERIFY_OK'
```

## Issue #22 Gate 3 closeout 本地验证

本节只验证 14 条治理/文档路径和已合并证据，不执行本地全 Workspace/Iced 重型编译：

```powershell
$baseline = '0716ec0debcd3e059cc4ca88a072232841ca73b4'
$expectedBranch = 'codex/issue-22-gate-3-closeout'
$expectedPaths = @(
  'AGENTS.md',
  'README.md',
  'build.md',
  'err.md',
  'docs/plans/PROJECT-MASTER-PLAN.md',
  'docs/plans/2026-07-21-rust-ci-offload-implementation-plan.md',
  'docs/plans/sessions/2026-07-22-issue-19-gate-3-rust-workspace-ci.md',
  'docs/workflows/2026-07-22-issue-19-gate-3-rust-workspace-ci-runtime.md',
  'docs/reports/issue-19-gate-3-rust-workspace-ci.md',
  'docs/reports/rust-ci-cold-baseline.md',
  'docs/plans/2026-07-22-issue-22-gate-3-closeout.md',
  'docs/plans/sessions/2026-07-22-issue-22-gate-3-closeout.md',
  'docs/workflows/2026-07-22-issue-22-gate-3-closeout-runtime.md',
  'docs/reports/issue-22-gate-3-closeout.md'
)

$branch = (git branch --show-current).Trim()
if ($LASTEXITCODE -ne 0 -or $branch -ne $expectedBranch) {
  throw "Issue #22 当前分支不正确：$branch"
}
$changed = @(
  git -c core.quotePath=false diff --name-only $baseline
  git -c core.quotePath=false ls-files --others --exclude-standard
) | Where-Object { $_ } | Sort-Object -Unique
if ($LASTEXITCODE -ne 0) { throw '读取 Issue #22 变更路径失败。' }
$unexpected = @($changed | Where-Object { $_ -notin $expectedPaths })
$missing = @($expectedPaths | Where-Object { $_ -notin $changed })
if ($unexpected.Count -ne 0 -or $missing.Count -ne 0 -or $changed.Count -ne 14) {
  throw "Issue #22 路径不一致；越界=$($unexpected -join ',')；缺失=$($missing -join ',')；总数=$($changed.Count)"
}

$protected = @($changed | Where-Object {
  $_ -in @('Cargo.toml','Cargo.lock','rust-toolchain.toml','.github/workflows/ci.yml') -or
  $_ -like 'apps/*' -or $_ -like 'crates/*' -or $_ -like 'scripts/ci/*' -or
  $_ -like 'upstream/*' -or $_ -match '(?i)agos'
})
if ($protected.Count -ne 0) { throw "Issue #22 触及受保护路径：$($protected -join ',')" }

$controlFiles = @(
  'docs/plans/2026-07-22-issue-22-gate-3-closeout.md',
  'docs/plans/sessions/2026-07-22-issue-22-gate-3-closeout.md',
  'docs/workflows/2026-07-22-issue-22-gate-3-closeout-runtime.md',
  'docs/reports/issue-22-gate-3-closeout.md'
)
$controlText = ($controlFiles | ForEach-Object { Get-Content -LiteralPath $_ -Raw -Encoding UTF8 }) -join "`n"
foreach ($required in @(
  'user-message:create-gate3-closeout-through-squash-merge-2026-07-22',
  'sha256:16760a8ce385b171b007451a43a3acb604a7b8ffc06b098b5482b8d803115ec8',
  '0716ec0debcd3e059cc4ca88a072232841ca73b4',
  '4881ce609370f77181d9545474c029ab0c5d4972',
  '29919596057'
)) {
  if (-not $controlText.Contains($required)) { throw "Issue #22 控制面缺少：$required" }
}

$sourceControl = @(
  Get-Content -LiteralPath 'AGENTS.md' -Raw -Encoding UTF8
  Get-Content -LiteralPath 'docs/plans/sessions/2026-07-22-issue-19-gate-3-rust-workspace-ci.md' -Raw -Encoding UTF8
  Get-Content -LiteralPath 'docs/workflows/2026-07-22-issue-19-gate-3-rust-workspace-ci-runtime.md' -Raw -Encoding UTF8
  Get-Content -LiteralPath 'docs/reports/issue-19-gate-3-rust-workspace-ci.md' -Raw -Encoding UTF8
) -join "`n"
if ($sourceControl -match '当前仓库尚未导入应用源码|pr-review-ready-owner-merge-authorization-pending|merge_ref:\s*pending|Issue `#19` 仍 OPEN') {
  throw 'Issue #22 未清除来源 Gate 3 的陈旧状态。'
}

$scriptPaths = @(
  'scripts/ci/Collect-Changes.ps1',
  'scripts/ci/Classify-Changes.ps1',
  'scripts/ci/Verify-RepositoryPolicy.ps1',
  'scripts/ci/Test-CiScripts.ps1'
)
foreach ($scriptPath in $scriptPaths) {
  $tokens = $null
  $errors = $null
  [void][System.Management.Automation.Language.Parser]::ParseFile((Resolve-Path -LiteralPath $scriptPath), [ref]$tokens, [ref]$errors)
  if (@($errors).Count -ne 0) { throw "$scriptPath AST 解析失败。" }
}

pwsh -NoProfile -File scripts/ci/Test-CiScripts.ps1
if ($LASTEXITCODE -ne 0) { throw 'Issue #22 治理合同失败。' }
pwsh -NoProfile -File scripts/ci/Verify-RepositoryPolicy.ps1 -RepositoryRoot .
if ($LASTEXITCODE -ne 0) { throw 'Issue #22 真实仓库政策失败。' }
git diff --check
if ($LASTEXITCODE -ne 0) { throw 'Issue #22 工作树存在空白错误。' }
Write-Output 'ISSUE22_GATE3_CLOSEOUT_LOCAL_VERIFY_OK'
```

## Issue #19 Workspace 本地轻量验证

标准命令由 Rust `1.97.1` 执行：

```powershell
pwsh -NoProfile -File scripts/ci/Test-CiScripts.ps1
pwsh -NoProfile -File scripts/ci/Verify-RepositoryPolicy.ps1 -RepositoryRoot .
cargo metadata --locked --no-deps --format-version 1
cargo fmt --all -- --check
cargo check --locked -p inputcodex-domain
cargo test --locked -p inputcodex-domain
```

本机在 2026 年 7 月 22 日安装 `1.97.1` minimal 工具链超过 5 分钟仍无完成证据，已终止残留进程并按项目 CI 卸载合同绕过。以下命令只证明轻量代码可在现有 `1.93.1` 上运行，不构成精确工具链或桌面编译证据：

```powershell
$env:RUSTUP_TOOLCHAIN = '1.93.1-x86_64-pc-windows-msvc'
cargo metadata --locked --offline --no-deps --format-version 1
cargo fmt --all -- --check
cargo check --locked --offline --ignore-rust-version -p inputcodex-domain
cargo test --locked --offline --ignore-rust-version `
  -p inputcodex-domain `
  -p inputcodex-application `
  -p inputcodex-infrastructure `
  -p inputcodex-platform `
  -p inputcodex-parity `
  -p inputcodex-presentation `
  --no-default-features
```

`Cargo.lock` 当前包含 `336` 个 package 记录，其中 `329` 个有 registry source、`7` 个是本 Workspace 包；Workspace 许可证必须与根 `LICENSE` 一致并固定为 `AGPL-3.0-only`。Iced 必须为 `0.14.0` 且 checksum 为 `000e01026c93ba643f8357a3db3ada0e6555265a377f6f9291c472f6dd701fb3`；根清单只允许 Iced feature `wgpu`、`thread-pool`、`x11`、`wayland`，禁止 `webgl`、`web-colors`、`crisp` 和默认 features。

`cargo check -p inputcodex-desktop`、Iced 运行时、Windows/macOS 编译与全 Workspace 测试只在标准 GitHub-hosted runners 执行；不得为本地取证下载并编译完整 `329` 个外部包依赖图。

## Issue #19 治理 RED checkpoint 验证

当前 RED 合同只允许新增 `scripts/ci/Test-CiScripts.ps1` 与任务控制面；执行必须因为 `Classify-Changes.ps1` 和 `Verify-RepositoryPolicy.ps1` 尚不存在而失败：

```powershell
$testScript = (Resolve-Path -LiteralPath 'scripts/ci/Test-CiScripts.ps1').Path
$tokens = $null
$parseErrors = $null
[void][System.Management.Automation.Language.Parser]::ParseFile(
  $testScript,
  [ref]$tokens,
  [ref]$parseErrors
)
if ($parseErrors.Count -ne 0) {
  throw "RED 合同存在 AST 错误：$($parseErrors.Message -join '; ')"
}

if ((Test-Path -LiteralPath 'scripts/ci/Classify-Changes.ps1') -or
    (Test-Path -LiteralPath 'scripts/ci/Verify-RepositoryPolicy.ps1')) {
  throw 'RED checkpoint 不允许治理实现提前存在。'
}

$powerShellExecutable = (Get-Process -Id $PID).Path
$output = @(& $powerShellExecutable -NoLogo -NoProfile -File $testScript 2>&1)
$redExitCode = $LASTEXITCODE
$redText = ($output | ForEach-Object { $_.ToString() }) -join "`n"
$redMarkerCount = ([regex]::Matches($redText, 'CI_CONTRACT_RED_MISSING_IMPLEMENTATION')).Count
if ($redExitCode -ne 10 -or $redMarkerCount -ne 1) {
  throw "RED 根因不可信；exit=$redExitCode；marker_count=$redMarkerCount；output=$redText"
}
```

验证通过时必须同时得到 `AST_ERROR_COUNT=0`、`RED_EXIT_CODE=10` 和 `RED_MARKER_COUNT=1`；这不是 GREEN，也不得解释为治理能力已经实现。

## Issue #19 治理 GREEN checkpoint 验证

```powershell
$scripts = @(
  'scripts/ci/Test-CiScripts.ps1',
  'scripts/ci/Collect-Changes.ps1',
  'scripts/ci/Classify-Changes.ps1',
  'scripts/ci/Verify-RepositoryPolicy.ps1'
)
foreach ($scriptPath in $scripts) {
  $tokens = $null
  $parseErrors = $null
  [void][System.Management.Automation.Language.Parser]::ParseFile(
    (Resolve-Path -LiteralPath $scriptPath).Path,
    [ref]$tokens,
    [ref]$parseErrors
  )
  if ($parseErrors.Count -ne 0) {
    throw "$scriptPath 存在 AST 错误：$($parseErrors.Message -join '; ')"
  }
}

$powerShellExecutable = (Get-Process -Id $PID).Path
$output = @(& $powerShellExecutable -NoLogo -NoProfile -File 'scripts/ci/Test-CiScripts.ps1' 2>&1)
$greenExitCode = $LASTEXITCODE
$greenText = ($output | ForEach-Object { $_.ToString() }) -join "`n"
if ($greenExitCode -ne 0 -or $greenText -notmatch 'CI_CONTRACT_GREEN passed=30') {
  throw "治理合同未 GREEN；exit=$greenExitCode；output=$greenText"
}

git diff --check
if ($LASTEXITCODE -ne 0) { throw 'GREEN checkpoint 存在空白错误。' }
```

GREEN 夹具覆盖空 diff、文档/重型路径、删除/重命名、真实 Git NUL 变更收集、非法路径、`AGPL-3.0-only` Workspace 许可证、Iced 越层、`upstream/` Workspace 越界、生产脚本语言、Tauri/WebView、广告/遥测、非本仓更新源、精确依赖方向、TOML 内联与表形式的依赖声明，以及三平台冷构建指标同时写入控制台日志与 Step Summary。

## Issue #19 首版 CI 本地静态验证

本地只验证 Workflow 语法和治理合同，不执行三平台 Rust 全量编译：

```powershell
python -c "from pathlib import Path; import yaml; data=yaml.safe_load(Path('.github/workflows/ci.yml').read_text(encoding='utf-8')); jobs=data['jobs']; invalid=[name for name,job in jobs.items() if any('runner.' in str(value) for value in (job.get('env') or {}).values())]; assert not invalid, f'runner context is unavailable in job-level env: {invalid}'; print('CI_YAML_PARSE_OK')"
if ($LASTEXITCODE -ne 0) { throw 'CI Workflow YAML 解析失败。' }

$workflow = Get-Content -LiteralPath '.github/workflows/ci.yml' -Raw -Encoding utf8
$requiredFragments = @(
  'name: CI',
  'contents: read',
  'cancel-in-progress: true',
  'classify:',
  'governance:',
  'linux-quality:',
  'windows:',
  'macos:',
  'required:',
  'if: ${{ always() }}',
  'retention-days: 7',
  'actions/checkout@3d3c42e5aac5ba805825da76410c181273ba90b1',
  'actions/upload-artifact@043fb46d1a93c77aae656e7c1c64a875d1fc6a0a'
)
foreach ($fragment in $requiredFragments) {
  if (-not $workflow.Contains($fragment)) { throw "CI Workflow 缺少合同片段：$fragment" }
}
if ($workflow -match '(?im)uses:\s*[^\s@]+@(?![0-9a-f]{40}\b)') {
  throw "CI Workflow 存在未固定到 40 位 SHA 的 Action：$($Matches[0])"
}
if ($workflow -match '(?im)cache|target/\*\*|target\\\*\*') {
  throw "首版 CI 出现禁止的 Cache 或 target Artifact：$($Matches[0])"
}

pwsh -NoProfile -File scripts/ci/Test-CiScripts.ps1
if ($LASTEXITCODE -ne 0) { throw 'CI 治理合同失败。' }
pwsh -NoProfile -File scripts/ci/Verify-RepositoryPolicy.ps1 -RepositoryRoot .
if ($LASTEXITCODE -ne 0) { throw '真实仓库政策失败。' }
git diff --check
if ($LASTEXITCODE -ne 0) { throw 'CI checkpoint 存在空白错误。' }
```

真实 Rust `1.97.1`、Linux Clippy/Workspace 测试、Windows/macOS 桌面构建、`required` 汇总和失败 Artifact 只能由关联 PR 的标准 GitHub-hosted runners 证明。CI 稳定前不得把 `CI / required` 写入 `main` Ruleset。

截至本轮收口，治理、rustfmt、通用 Rust 编译、Windows 条件编译、macOS 条件编译五类失败语义均已通过普通提交完成 RED→GREEN；最新修复运行 `29917649550` 六 Job 全绿且成功 Artifact 数为 `0`。Linux、Windows、macOS 已分别接受运行 `29911337652`、`29913139948`、`29914029406`，达到各 `3/3` 次无缓存成功样本；完整测量与失败运行引用见 `docs/reports/rust-ci-cold-baseline.md`。

正式合并前仍需在最终 PR Head 上重新确认所有适用 Job 成功、Review 对话为 `0`、自动合并关闭、Ruleset 无漂移，并取得项目所有者新的明确 Squash Merge 授权。

## Issue #19 Gate 3 实现控制面 checkpoint 验证

本节只用于 RED 批次开始前的首个命名 checkpoint；创建治理脚本或 Cargo Workspace 后，必须按 Runtime Workflow 更新为对应批次验证：

```powershell
$baseline = '477d110a9b284e127af365f5278901bcfa69e093'
$expectedBranch = 'codex/issue-19-gate-3-rust-workspace-ci'
$expectedPaths = @(
  'README.md',
  'build.md',
  'err.md',
  'docs/plans/PROJECT-MASTER-PLAN.md',
  'docs/plans/2026-07-21-rust-ci-offload-implementation-plan.md',
  'docs/plans/sessions/2026-07-22-issue-19-gate-3-rust-workspace-ci.md',
  'docs/workflows/2026-07-22-issue-19-gate-3-rust-workspace-ci-runtime.md',
  'docs/reports/issue-17-gate-3-rust-workspace-plan.md',
  'docs/reports/issue-19-gate-3-rust-workspace-ci.md'
)

$branch = (git branch --show-current).Trim()
if ($LASTEXITCODE -ne 0 -or $branch -ne $expectedBranch) {
  throw "Issue #19 当前分支不正确：$branch"
}
$trackedChanges = @(git diff --name-only $baseline | Where-Object { $_ })
if ($LASTEXITCODE -ne 0) { throw '读取 Issue #19 已跟踪变更路径失败。' }
$untrackedChanges = @(git ls-files --others --exclude-standard | Where-Object { $_ })
if ($LASTEXITCODE -ne 0) { throw '读取 Issue #19 未跟踪变更路径失败。' }
$changedPaths = @(($trackedChanges + $untrackedChanges) | Sort-Object -Unique)
$unexpected = @($changedPaths | Where-Object { $_ -notin $expectedPaths })
$missing = @($expectedPaths | Where-Object { $_ -notin $changedPaths })
if ($unexpected.Count -ne 0 -or $missing.Count -ne 0) {
  throw "Issue #19 checkpoint 路径不一致；越界=$($unexpected -join ',')；缺失=$($missing -join ',')"
}

$requiredFiles = @(
  'docs/plans/sessions/2026-07-22-issue-19-gate-3-rust-workspace-ci.md',
  'docs/workflows/2026-07-22-issue-19-gate-3-rust-workspace-ci-runtime.md',
  'docs/reports/issue-19-gate-3-rust-workspace-ci.md'
)
foreach ($path in $requiredFiles) {
  if (-not (Test-Path -LiteralPath $path)) { throw "缺少 Issue #19 控制面文件：$path" }
}

$controlText = ($requiredFiles | ForEach-Object { Get-Content -LiteralPath $_ -Raw }) -join "`n"
if ($controlText -notmatch 'user-message:approve-gate-3-implementation-2026-07-22' -or
    $controlText -notmatch 'sha256:2e101627480012d57d6d0472a08cfbe03fc401f6ac74ef3ae1e6a42929ed61ba' -or
    $controlText -match '__ISSUE_|pending-self-reference') {
  throw 'Issue #19 控制面缺少批准/范围证据或仍含占位符。'
}

$productCargo = @(Get-ChildItem -Recurse -File -Include Cargo.toml,Cargo.lock,rust-toolchain.toml -ErrorAction SilentlyContinue | Where-Object { $_.FullName -notmatch '[\\/]upstream[\\/]' })
$productRust = @(Get-ChildItem -Recurse -File -Filter '*.rs' -ErrorAction SilentlyContinue | Where-Object { $_.FullName -notmatch '[\\/]upstream[\\/]' })
if ($productCargo.Count -ne 0 -or $productRust.Count -ne 0 -or (Test-Path -LiteralPath '.github/workflows/ci.yml')) {
  throw '控制面 checkpoint 禁止提前出现产品 Cargo/Rust 或 CI Workflow。'
}

git diff --check $baseline
if ($LASTEXITCODE -ne 0) { throw 'Issue #19 checkpoint diff 检查失败。' }
Write-Output 'ISSUE19_GATE3_CONTROL_PLANE_VERIFY_OK'
```

## Issue #17 Gate 3 规划本地验证

本节只验证治理文档与禁止表面，不编译 Rust、不运行全 Workspace，也不联网写 GitHub：

```powershell
$baseline = '113476fb96623452f9a69526edabc73a57d812a1'
$allowedPaths = @(
  'README.md',
  'build.md',
  'err.md',
  'docs/plans/PROJECT-MASTER-PLAN.md',
  'docs/plans/2026-07-21-architecture-governance.md',
  'docs/plans/2026-07-21-rust-ci-offload-implementation-plan.md',
  'docs/plans/2026-07-22-issue-17-gate-3-rust-workspace-plan.md',
  'docs/plans/sessions/2026-07-22-issue-17-gate-3-rust-workspace-plan.md',
  'docs/workflows/2026-07-22-issue-17-gate-3-rust-workspace-plan-runtime.md',
  'docs/reports/issue-17-gate-3-rust-workspace-plan.md',
  'docs/reports/issue-14-gate-2-upstream-watch.md'
)

$changedPaths = @(
  git -c core.quotePath=false diff --name-only "$baseline...HEAD"
  git -c core.quotePath=false diff --name-only
  git -c core.quotePath=false ls-files --others --exclude-standard
) | Where-Object { $_ } | Sort-Object -Unique

$unexpectedPaths = @($changedPaths | Where-Object { $_ -notin $allowedPaths })
if ($unexpectedPaths.Count -ne 0) {
  throw "Issue #17 存在越界路径：$($unexpectedPaths -join ', ')"
}
if ($changedPaths.Count -ne 11) {
  throw "Issue #17 完整规划应修改 11 条路径，实际为 $($changedPaths.Count)。"
}

$requiredFiles = @(
  'docs/plans/2026-07-22-issue-17-gate-3-rust-workspace-plan.md',
  'docs/plans/sessions/2026-07-22-issue-17-gate-3-rust-workspace-plan.md',
  'docs/workflows/2026-07-22-issue-17-gate-3-rust-workspace-plan-runtime.md',
  'docs/reports/issue-17-gate-3-rust-workspace-plan.md'
)
foreach ($path in $requiredFiles) {
  if (-not (Test-Path -LiteralPath $path)) { throw "缺少 Gate 3 规划文件：$path" }
}

$productCargoFiles = @(
  Get-ChildItem -LiteralPath . -Recurse -File -Include 'Cargo.toml','Cargo.lock','rust-toolchain.toml' |
  Where-Object { $_.FullName -notmatch '[\\/]upstream[\\/]' -and $_.FullName -notmatch '[\\/]\.git[\\/]' }
)
$productRustFiles = @(
  Get-ChildItem -LiteralPath . -Recurse -File -Filter '*.rs' |
  Where-Object { $_.FullName -notmatch '[\\/]upstream[\\/]' -and $_.FullName -notmatch '[\\/]\.git[\\/]' }
)
if ($productCargoFiles.Count -ne 0 -or $productRustFiles.Count -ne 0) {
  throw 'Issue #17 规划阶段禁止出现产品 Cargo 或 Rust 文件。'
}

$workflowNames = @(Get-ChildItem -LiteralPath '.github/workflows' -File | Select-Object -ExpandProperty Name)
if ($workflowNames.Count -ne 1 -or $workflowNames[0] -ne 'upstream-watch.yml') {
  throw 'Issue #17 规划阶段不得新增或替换产品 Workflow。'
}

$master = Get-Content -LiteralPath 'docs/plans/PROJECT-MASTER-PLAN.md' -Raw
if ($master -notmatch 'active_task: 2026-07-22-issue-17-gate-3-rust-workspace-plan' -or
    $master -notmatch 'tracking_issue_ref: https://github.com/nonononull/inputcodex/issues/17' -or
    $master -notmatch 'gate-3-planning-approved-implementation-locked') {
  throw 'Master Plan 未正确指向 Issue #17。'
}

git diff --check
if ($LASTEXITCODE -ne 0) { throw 'Issue #17 工作树 diff 检查失败。' }
Write-Output 'ISSUE17_GATE3_PLANNING_LOCAL_VERIFY_OK'
```

## Issue #14 上游监控本地验证

本地验证不联网、不写 GitHub Issue，也不编译 Rust。运行：

```powershell
$previousPycachePrefix = $env:PYTHONPYCACHEPREFIX
$env:PYTHONPYCACHEPREFIX = Join-Path ([IO.Path]::GetTempPath()) 'inputcodex-issue14-pycache'
try {
  python -m unittest discover -s .github/scripts/tests -p 'test_*.py' -v
  if ($LASTEXITCODE -ne 0) { throw '上游监控无网络合同测试失败。' }

  python -m py_compile .github/scripts/upstream_watch.py .github/scripts/tests/test_upstream_watch.py
  if ($LASTEXITCODE -ne 0) { throw '上游监控 Python 编译检查失败。' }

  python .github/scripts/upstream_watch.py --validate-only
  if ($LASTEXITCODE -ne 0) { throw '上游监控冻结基线验证失败。' }

  @'
from pathlib import Path
import yaml

path = Path('.github/workflows/upstream-watch.yml')
data = yaml.load(path.read_text(encoding='utf-8'), Loader=yaml.BaseLoader)
triggers = data['on']
assert triggers['schedule'] == [{'cron': '17 */6 * * *'}]
assert 'workflow_dispatch' in triggers
assert 'pull_request' in triggers
assert data['permissions'] == {'contents': 'read'}
assert data['env'] == {'PYTHONPYCACHEPREFIX': '/tmp/inputcodex-pycache'}
assert data['jobs']['watch']['permissions'] == {'contents': 'read', 'issues': 'write'}
assert data['jobs']['watch']['if'] == "github.event_name != 'pull_request'"
assert data['jobs']['watch']['timeout-minutes'] == '10'
print('UPSTREAM_WATCH_WORKFLOW_YAML_OK')
'@ | python -
  if ($LASTEXITCODE -ne 0) { throw '上游监控 Workflow YAML 合同失败。' }
} finally {
  if ($null -eq $previousPycachePrefix) {
    Remove-Item Env:PYTHONPYCACHEPREFIX -ErrorAction SilentlyContinue
  } else {
    $env:PYTHONPYCACHEPREFIX = $previousPycachePrefix
  }
}
```

允许路径和禁止修改面验证：

```powershell
$allowedPaths = @(
  '.github/scripts/tests/test_upstream_watch.py',
  '.github/scripts/upstream_watch.py',
  '.github/workflows/upstream-watch.yml',
  'README.md',
  'build.md',
  'docs/plans/2026-07-22-issue-14-gate-2-upstream-watch.md',
  'docs/plans/PROJECT-MASTER-PLAN.md',
  'docs/plans/sessions/2026-07-22-issue-14-gate-2-upstream-watch.md',
  'docs/reports/issue-14-gate-2-upstream-watch.md',
  'docs/workflows/2026-07-22-issue-14-gate-2-upstream-watch-runtime.md',
  'err.md'
)

$branch = git branch --show-current
if ($LASTEXITCODE -ne 0 -or $branch -ne 'codex/issue-14-gate-2-upstream-watch') {
  throw "当前 Issue #14 分支不正确：$branch"
}

$committed = @(git -c core.quotePath=false diff --name-only origin/main...HEAD)
if ($LASTEXITCODE -ne 0) { throw '读取 Issue #14 已提交差异失败。' }
$working = @(git -c core.quotePath=false diff --name-only)
if ($LASTEXITCODE -ne 0) { throw '读取 Issue #14 工作树差异失败。' }
$staged = @(git -c core.quotePath=false diff --cached --name-only)
if ($LASTEXITCODE -ne 0) { throw '读取 Issue #14 暂存差异失败。' }
$untracked = @(git -c core.quotePath=false ls-files --others --exclude-standard)
if ($LASTEXITCODE -ne 0) { throw '读取 Issue #14 未跟踪路径失败。' }
$changed = @($committed + $working + $staged + $untracked | Sort-Object -Unique)
$unexpected = @($changed | Where-Object { $_ -notin $allowedPaths })
if ($unexpected.Count -ne 0) {
  throw "Issue #14 混入未批准路径：$($unexpected -join ', ')"
}

foreach ($path in $allowedPaths) {
  if (-not (Test-Path -LiteralPath $path -PathType Leaf)) {
    throw "缺少 Issue #14 允许文件：$path"
  }
}

git diff --quiet origin/main...HEAD -- upstream
if ($LASTEXITCODE -ne 0) { throw 'Issue #14 已提交差异修改了 upstream。' }
git diff --quiet -- upstream
if ($LASTEXITCODE -ne 0) { throw 'Issue #14 工作树修改了 upstream。' }
git diff --cached --quiet -- upstream
if ($LASTEXITCODE -ne 0) { throw 'Issue #14 暂存区修改了 upstream。' }

foreach ($path in @('Cargo.toml', 'Cargo.lock', 'package.json', 'package-lock.json', 'target')) {
  if (Test-Path -LiteralPath $path) { throw "Issue #14 禁止出现：$path" }
}

Write-Output 'ISSUE14_UPSTREAM_WATCH_LOCAL_VERIFY_OK'
```

合并后在 `main` 执行两次真实监控：

```powershell
gh workflow run upstream-watch.yml --repo nonononull/inputcodex --ref main
if ($LASTEXITCODE -ne 0) { throw '首次触发上游监控失败。' }

gh run list --repo nonononull/inputcodex --workflow upstream-watch.yml --limit 5
if ($LASTEXITCODE -ne 0) { throw '读取上游监控运行列表失败。' }
```

等待首次运行成功并建立唯一状态 Issue 后，再触发第二次；第二次必须复用状态 Issue 且不得创建重复告警。失败必须先查 `err.md`，不得通过盲目重跑掩盖根因。

## Gate 2 快照离线验证

```powershell
@'
import hashlib
import json
import subprocess
from pathlib import Path, PurePosixPath

lock = json.loads(Path('upstream/source-lock.json').read_text(encoding='utf-8'))
root = Path(lock['snapshot']['path'])
expected = lock['files']
expected_paths = [item['path'] for item in expected]
actual_paths = sorted(path.relative_to(root).as_posix() for path in root.rglob('*') if path.is_file())
assert actual_paths == expected_paths
assert len(actual_paths) == lock['manifest']['file_count'] == 277
assert sum(1 for path in root.rglob('*') if path.is_dir()) == lock['tree']['directory_count'] == 70
assert not (root / '.git').exists()

manifest = bytearray()
total_bytes = 0
for item in expected:
    data = (root / PurePosixPath(item['path'])).read_bytes()
    blob = hashlib.sha1(b'blob ' + str(len(data)).encode('ascii') + b'\0' + data).hexdigest()
    sha256 = hashlib.sha256(data).hexdigest()
    assert len(data) == item['size']
    assert blob == item['git_blob_sha1']
    assert sha256 == item['sha256']
    manifest.extend(f"{sha256}  {item['path']}\n".encode('utf-8'))
    total_bytes += len(data)

assert total_bytes == lock['manifest']['total_bytes'] == 24175877
assert hashlib.sha256(manifest).hexdigest() == lock['manifest']['sha256']
assert len(lock['license']['preserved_files']) == 7

tree = subprocess.run(
    ['git', '-c', 'core.quotePath=false', 'ls-tree', '-r', '-z', 'HEAD', '--', root.as_posix()],
    check=True,
    stdout=subprocess.PIPE,
).stdout
entries = {}
for record in tree.split(b'\0'):
    if record:
        meta, raw_path = record.split(b'\t', 1)
        mode, object_type, sha1 = meta.decode('ascii').split()
        path = raw_path.decode('utf-8').removeprefix(root.as_posix() + '/')
        entries[path] = (mode, object_type, sha1)
assert sorted(entries) == expected_paths
for item in expected:
    mode, object_type, sha1 = entries[item['path']]
    assert object_type == 'blob'
    assert mode == item['mode']
    assert sha1 == item['git_blob_sha1']

print('UPSTREAM_SNAPSHOT_CURRENT_VERIFY_OK')
'@ | python -
if ($LASTEXITCODE -ne 0) { throw '上游快照离线验证失败。' }
```

## PR #11 合并与 Issue #9 关闭验证

```powershell
$repo = 'nonononull/inputcodex'
$mergeCommit = 'dde08b725eb2bf4add7fbcfa955f3eaf4eb1bbc6'
$mergeParent = '216d400006ad3f1dd2587ca367abb19d0191949f'
$mergeTree = 'd0c90b9bfda70de800788782180080d50d914564'

$pr = gh pr view 11 --repo $repo --json state,mergedAt,mergeCommit,headRefOid | ConvertFrom-Json
if ($LASTEXITCODE -ne 0 -or
    $pr.state -ne 'MERGED' -or
    $pr.mergeCommit.oid -ne $mergeCommit -or
    $pr.headRefOid -ne '90d35a72cffb4a13c5f7588a147e19cbd75b14c6') {
  throw 'PR #11 合并证据不一致。'
}

$issue = gh issue view 9 --repo $repo --json state,closedAt | ConvertFrom-Json
if ($LASTEXITCODE -ne 0 -or $issue.state -ne 'CLOSED') {
  throw 'Issue #9 未关闭。'
}

$main = (gh api repos/$repo/git/ref/heads/main | ConvertFrom-Json).object.sha
if ($LASTEXITCODE -ne 0 -or $main -ne $mergeCommit) {
  throw '远端 main 未指向 PR #11 merge commit。'
}

git fetch origin main
if ($LASTEXITCODE -ne 0) { throw '刷新 origin/main 失败。' }
$parents = @((git show -s --format='%P' $mergeCommit) -split ' ' | Where-Object { $_ })
if ($LASTEXITCODE -ne 0 -or $parents.Count -ne 1 -or $parents[0] -ne $mergeParent) {
  throw 'PR #11 不是预期的单父 Squash 提交。'
}
$actualTree = git show -s --format='%T' $mergeCommit
if ($LASTEXITCODE -ne 0 -or $actualTree -ne $mergeTree) {
  throw 'PR #11 merge tree 不一致。'
}
$changed = @(git -c core.quotePath=false diff --name-only $mergeParent $mergeCommit)
$unexpected = @($changed | Where-Object {
  $_ -notlike 'upstream/*' -and $_ -ne 'docs/reports/2026-07-21-upstream-v1.2.41-sync.md'
})
if ($LASTEXITCODE -ne 0 -or $changed.Count -ne 279 -or $unexpected.Count -ne 0) {
  throw 'PR #11 合并差异范围不一致。'
}

Write-Output 'PR11_GATE2_MERGE_VERIFY_OK'
```

## Issue #12 closeout 本地验证

```powershell
$allowedPaths = @(
  'README.md',
  'build.md',
  'err.md',
  'docs/plans/PROJECT-MASTER-PLAN.md',
  'docs/plans/2026-07-21-issue-9-gate-2-upstream-baseline.md',
  'docs/plans/sessions/2026-07-21-issue-9-gate-2-upstream-baseline.md',
  'docs/workflows/2026-07-21-issue-9-gate-2-upstream-baseline-runtime.md',
  'docs/plans/2026-07-21-issue-12-gate-2-upstream-closeout.md',
  'docs/plans/sessions/2026-07-21-issue-12-gate-2-upstream-closeout.md',
  'docs/workflows/2026-07-21-issue-12-gate-2-upstream-closeout-runtime.md',
  'docs/reports/issue-12-gate-2-upstream-closeout.md'
)

foreach ($path in $allowedPaths) {
  if (-not (Test-Path -LiteralPath $path -PathType Leaf)) {
    throw "缺少 Issue #12 控制文件：$path"
  }
}

$branch = git branch --show-current
if ($LASTEXITCODE -ne 0 -or $branch -ne 'codex/issue-12-gate-2-upstream-closeout') {
  throw "当前 closeout 分支不正确：$branch"
}

$committed = @(git -c core.quotePath=false diff --name-only origin/main...HEAD)
if ($LASTEXITCODE -ne 0) { throw '读取 closeout 已提交差异失败。' }
$working = @(git -c core.quotePath=false diff --name-only)
if ($LASTEXITCODE -ne 0) { throw '读取 closeout 工作树差异失败。' }
$staged = @(git -c core.quotePath=false diff --cached --name-only)
if ($LASTEXITCODE -ne 0) { throw '读取 closeout 暂存差异失败。' }
$changed = @($committed + $working + $staged | Sort-Object -Unique)
$unexpected = @($changed | Where-Object { $_ -notin $allowedPaths })
if ($unexpected.Count -ne 0) {
  throw "Issue #12 混入未批准路径：$($unexpected -join ', ')"
}

git diff --quiet origin/main...HEAD -- upstream
if ($LASTEXITCODE -ne 0) { throw 'closeout 已提交差异修改了 upstream。' }
git diff --quiet -- upstream
if ($LASTEXITCODE -ne 0) { throw 'closeout 工作树修改了 upstream。' }
git diff --cached --quiet -- upstream
if ($LASTEXITCODE -ne 0) { throw 'closeout 暂存区修改了 upstream。' }

foreach ($path in @('Cargo.toml', 'Cargo.lock', 'package.json', 'package-lock.json', 'pnpm-lock.yaml', 'yarn.lock', 'target', 'node_modules', 'dist')) {
  if (Test-Path -LiteralPath $path) { throw "仓库根目录出现未批准路径：$path" }
}
if (Test-Path -LiteralPath '.github/workflows') {
  if (@(Get-ChildItem -LiteralPath '.github/workflows' -Recurse -File).Count -ne 0) {
    throw 'Issue #12 不得创建 GitHub Actions Workflow。'
  }
}

git diff --check
if ($LASTEXITCODE -ne 0) { throw '工作树 diff 检查失败。' }
git diff --cached --check
if ($LASTEXITCODE -ne 0) { throw '暂存区 diff 检查失败。' }

Write-Output "ISSUE12_CHANGED_PATHS=$($changed.Count)"
Write-Output 'ISSUE12_CLOSEOUT_LOCAL_VERIFY_OK'
```

## 历史：Gate 2 规划阶段本地 Fresh 验证

> 本节只保留 PR `#10` 合并前后的历史控制面证据，其中“禁止出现 upstream”与固定旧分支的断言不再是当前门禁。当前任务必须使用前述快照、合并与 Issue `#12` 验证命令。

```powershell
$expectedFiles = @(
  'AGENTS.md',
  'README.md',
  'build.md',
  'err.md',
  'docs/plans/PROJECT-MASTER-PLAN.md',
  'docs/plans/2026-07-21-architecture-governance.md',
  'docs/plans/2026-07-21-issue-8-gate-2-transition.md',
  'docs/plans/2026-07-21-issue-9-gate-2-upstream-baseline.md',
  'docs/plans/sessions/2026-07-21-issue-8-gate-2-transition.md',
  'docs/plans/sessions/2026-07-21-issue-9-gate-2-upstream-baseline.md',
  'docs/workflows/2026-07-21-issue-8-gate-2-transition-runtime.md',
  'docs/workflows/2026-07-21-issue-9-gate-2-upstream-baseline-runtime.md',
  'docs/reports/issue-6-gate-1-finalization-closeout.md',
  '.github/pull_request_template.md',
  '.github/ISSUE_TEMPLATE/config.yml',
  '.github/ISSUE_TEMPLATE/upstream-watch.yml',
  '.github/ISSUE_TEMPLATE/upstream-sync.yml',
  '.github/ISSUE_TEMPLATE/feature-parity.yml',
  '.github/ISSUE_TEMPLATE/parity-exception.yml',
  '.github/ISSUE_TEMPLATE/performance.yml',
  '.github/ISSUE_TEMPLATE/architecture.yml',
  '.github/ISSUE_TEMPLATE/release.yml',
  '.github/ISSUE_TEMPLATE/bug.yml'
)

foreach ($path in $expectedFiles) {
  if (-not (Test-Path -LiteralPath $path -PathType Leaf)) {
    throw "缺少 Gate 2 控制文件：$path"
  }
}

$forbiddenPaths = @(
  'upstream',
  'source-lock.json',
  'Cargo.toml',
  'Cargo.lock',
  'package.json',
  'package-lock.json',
  'pnpm-lock.yaml',
  'yarn.lock'
)

foreach ($path in $forbiddenPaths) {
  if (Test-Path -LiteralPath $path) {
    throw "Gate 2 规划阶段禁止出现：$path"
  }
}

$rustFiles = @(
  Get-ChildItem -LiteralPath . -Recurse -File -Filter '*.rs' |
  Where-Object { $_.FullName -notmatch '[\\/]\.git[\\/]' }
)
if ($rustFiles.Count -ne 0) {
  throw "Gate 2 规划阶段禁止 Rust 源码，发现 $($rustFiles.Count) 个文件。"
}

if (Test-Path -LiteralPath '.github/workflows') {
  $workflowFiles = @(Get-ChildItem -LiteralPath '.github/workflows' -Recurse -File)
  if ($workflowFiles.Count -ne 0) {
    throw 'Gate 2 规划阶段禁止 GitHub Actions Workflow。'
  }
}

$currentFiles = @(
  'README.md',
  'docs/plans/PROJECT-MASTER-PLAN.md',
  'docs/plans/2026-07-21-architecture-governance.md',
  'docs/plans/2026-07-21-issue-8-gate-2-transition.md',
  'docs/plans/2026-07-21-issue-9-gate-2-upstream-baseline.md',
  'docs/plans/sessions/2026-07-21-issue-9-gate-2-upstream-baseline.md',
  'docs/workflows/2026-07-21-issue-9-gate-2-upstream-baseline-runtime.md'
)

$stalePatterns = @(
  'active_task: 2026-07-21-issue-6-gate-1-finalization',
  'PR `#7`.*OPEN',
  'PR `#7`.*等待项目所有者 Review',
  'Issue `#1`.*保持 OPEN',
  'Issue `#6`.*保持 OPEN'
)

foreach ($pattern in $stalePatterns) {
  $matches = @(Select-String -LiteralPath $currentFiles -Pattern $pattern)
  if ($matches.Count -ne 0) {
    throw "发现过期 Gate 1 状态：$pattern"
  }
}

$requiredStatements = @(
  @{ Path = 'README.md'; Pattern = 'Gate 2 上游基线规划阶段' },
  @{ Path = 'README.md'; Pattern = 'Issue `#9`' },
  @{ Path = 'docs/plans/PROJECT-MASTER-PLAN.md'; Pattern = 'active_task: 2026-07-21-issue-9-gate-2-upstream-baseline' },
  @{ Path = 'docs/plans/PROJECT-MASTER-PLAN.md'; Pattern = 'active_gate: Gate 2' },
  @{ Path = 'docs/plans/PROJECT-MASTER-PLAN.md'; Pattern = 'transition_pr_ref: https://github.com/nonononull/inputcodex/pull/10' },
  @{ Path = 'docs/reports/issue-6-gate-1-finalization-closeout.md'; Pattern = 'c74b66422ba47f96bd3eb2b2385cdfb90541808e' },
  @{ Path = 'docs/plans/2026-07-21-issue-9-gate-2-upstream-baseline.md'; Pattern = '尚未批准快照写入' }
)

foreach ($statement in $requiredStatements) {
  if (-not (Select-String -LiteralPath $statement.Path -SimpleMatch $statement.Pattern -Quiet)) {
    throw "缺少 Gate 2 硬约束：$($statement.Path) -> $($statement.Pattern)"
  }
}

$branch = git branch --show-current
if ($LASTEXITCODE -ne 0 -or $branch -ne 'codex/issue-8-gate-2-transition') {
  throw "当前过渡分支不正确：$branch"
}

git diff --check
if ($LASTEXITCODE -ne 0) {
  throw 'git diff --check 失败。'
}

Write-Output 'GATE2_LOCAL_CONTROL_PLANE_VERIFY_OK'
```

## Issue Forms YAML 验证

```powershell
$python = @"
from pathlib import Path
import yaml

root = Path('.github/ISSUE_TEMPLATE')
expected = {
    'upstream-watch.yml': {'type:upstream-watch'},
    'upstream-sync.yml': {'type:upstream-sync', 'gate:2'},
    'feature-parity.yml': {'type:feature-parity'},
    'parity-exception.yml': {'type:parity-exception', 'status:needs-owner-decision'},
    'performance.yml': {'type:performance'},
    'architecture.yml': {'type:architecture'},
    'release.yml': {'type:release', 'gate:6'},
    'bug.yml': {'type:bug'},
}

for filename, required_labels in expected.items():
    data = yaml.safe_load((root / filename).read_text(encoding='utf-8'))
    assert isinstance(data, dict), filename
    assert data.get('name') and data.get('description') and data.get('title'), filename
    assert required_labels <= set(data.get('labels') or []), filename
    body = data.get('body')
    assert isinstance(body, list) and body, filename
    ids = [item.get('id') for item in body if isinstance(item, dict) and item.get('id')]
    assert len(ids) == len(set(ids)), (filename, ids)

config = yaml.safe_load((root / 'config.yml').read_text(encoding='utf-8'))
assert config == {'blank_issues_enabled': False, 'contact_links': []}
print('ISSUE_FORMS_YAML_VERIFY_OK')
"@

$python | python -
if ($LASTEXITCODE -ne 0) {
  throw 'Issue Forms YAML 验证失败。'
}
```

## PR #7 与 Gate 1 closeout 核验

```powershell
$repo = 'nonononull/inputcodex'

$issue1 = gh issue view 1 --repo $repo --json state,stateReason,closedAt | ConvertFrom-Json
if ($LASTEXITCODE -ne 0) { throw '读取 Issue #1 失败。' }
$issue6 = gh issue view 6 --repo $repo --json state,closedAt | ConvertFrom-Json
if ($LASTEXITCODE -ne 0) { throw '读取 Issue #6 失败。' }
$pr7 = gh pr view 7 --repo $repo --json state,mergedAt,mergeCommit,headRefOid,statusCheckRollup | ConvertFrom-Json
if ($LASTEXITCODE -ne 0) { throw '读取 PR #7 失败。' }

if ($issue1.state -ne 'CLOSED' -or $issue1.stateReason -ne 'COMPLETED') {
  throw 'Issue #1 未以 completed 关闭。'
}
if ($issue6.state -ne 'CLOSED') {
  throw 'Issue #6 未关闭。'
}
if ($pr7.state -ne 'MERGED' -or
    $pr7.mergeCommit.oid -ne 'c74b66422ba47f96bd3eb2b2385cdfb90541808e' -or
    $pr7.headRefOid -ne 'e8b8631685e1b2f4361897016250b525f6d7c813' -or
    @($pr7.statusCheckRollup).Count -ne 0) {
  throw 'PR #7 合并证据变化。'
}

$commit = gh api repos/$repo/git/commits/c74b66422ba47f96bd3eb2b2385cdfb90541808e | ConvertFrom-Json
if ($LASTEXITCODE -ne 0) { throw '读取 PR #7 merge commit 失败。' }
if (-not $commit.verification.verified -or
    $commit.verification.reason -ne 'valid' -or
    @($commit.parents).Count -ne 1 -or
    $commit.parents[0].sha -ne 'b7404b0c63f2d2ba65474c077182c42a01cc9a64' -or
    $commit.tree.sha -ne '00f0f7fe0e408a1e6f218ee8e1be0d8442ed1e65') {
  throw 'PR #7 签名、parent 或 tree 证据变化。'
}

$query = 'query($owner:String!,$name:String!,$number:Int!){repository(owner:$owner,name:$name){pullRequest(number:$number){reviewThreads(first:100){nodes{isResolved}}}}}'
$review = gh api graphql -f query=$query -F owner=nonononull -F name=inputcodex -F number=7 | ConvertFrom-Json
if ($LASTEXITCODE -ne 0) { throw '读取 PR #7 Review 对话失败。' }
$unresolved = @($review.data.repository.pullRequest.reviewThreads.nodes | Where-Object { -not $_.isResolved })
if ($unresolved.Count -ne 0) {
  throw "PR #7 仍有 $($unresolved.Count) 个未解决 Review 对话。"
}

gh api repos/$repo/git/ref/heads/codex/issue-6-gate-1-finalization --silent 2>$null
if ($LASTEXITCODE -ne 1) {
  throw 'PR #7 远端旧分支仍存在或查询异常。'
}

Write-Output 'PR7_GATE1_CLOSEOUT_VERIFY_OK'
```

## Gate 2 Issues、Ruleset 与上游基线核验

```powershell
$repo = 'nonononull/inputcodex'

$issue8 = gh issue view 8 --repo $repo --json state,labels,url | ConvertFrom-Json
if ($LASTEXITCODE -ne 0) { throw '读取 Issue #8 失败。' }
$issue9 = gh issue view 9 --repo $repo --json state,labels,url | ConvertFrom-Json
if ($LASTEXITCODE -ne 0) { throw '读取 Issue #9 失败。' }
if ($issue8.state -ne 'OPEN' -or $issue9.state -ne 'OPEN') {
  throw '过渡 PR 合并前 Issue #8/#9 都必须保持 OPEN。'
}
foreach ($label in @('type:architecture', 'gate:1', 'gate:2')) {
  if ($label -notin @($issue8.labels.name)) { throw "Issue #8 缺少标签：$label" }
}
foreach ($label in @('type:upstream-sync', 'gate:2')) {
  if ($label -notin @($issue9.labels.name)) { throw "Issue #9 缺少标签：$label" }
}

$ruleset = gh api repos/$repo/rulesets/19395456 | ConvertFrom-Json
if ($LASTEXITCODE -ne 0) { throw '读取 Ruleset 失败。' }
$pullRequestRule = @($ruleset.rules | Where-Object { $_.type -eq 'pull_request' })
$allowedMethods = (@($pullRequestRule[0].parameters.allowed_merge_methods) -join ',')
if ($ruleset.enforcement -ne 'active' -or
    @($ruleset.bypass_actors).Count -ne 0 -or
    @($ruleset.rules | Where-Object { $_.type -eq 'deletion' }).Count -ne 1 -or
    @($ruleset.rules | Where-Object { $_.type -eq 'non_fast_forward' }).Count -ne 1 -or
    $pullRequestRule.Count -ne 1 -or
    $pullRequestRule[0].parameters.required_approving_review_count -ne 0 -or
    -not $pullRequestRule[0].parameters.required_review_thread_resolution -or
    $allowedMethods -ne 'squash') {
  throw 'main-protection Ruleset 不符合批准值。'
}

$release = gh api repos/BigPizzaV3/CodexPlusPlus/releases/latest | ConvertFrom-Json
if ($LASTEXITCODE -ne 0 -or $release.tag_name -ne 'v1.2.41') {
  throw '上游最新正式 Release 已变化。'
}
$upstreamCommit = gh api repos/BigPizzaV3/CodexPlusPlus/commits/v1.2.41 | ConvertFrom-Json
if ($LASTEXITCODE -ne 0 -or $upstreamCommit.sha -ne '3dafffcafb2566a1e8bce4b35671656d6adb3eda') {
  throw '上游 v1.2.41 提交已变化。'
}

$workflows = gh api repos/$repo/actions/workflows | ConvertFrom-Json
if ($LASTEXITCODE -ne 0 -or $workflows.total_count -ne 0) {
  throw 'Gate 2 规划阶段不允许 Actions Workflow。'
}
$releases = @(gh api repos/$repo/releases | ConvertFrom-Json)
if ($LASTEXITCODE -ne 0 -or $releases.Count -ne 0) {
  throw 'Gate 2 规划阶段不允许项目 Release。'
}

Write-Output 'GATE2_ISSUES_RULESET_UPSTREAM_VERIFY_OK'
```

## Issue #8 过渡 PR 合并前复核

本节在过渡 PR 创建并回写真实 URL 后执行：

```powershell
$repo = 'nonononull/inputcodex'
$branch = 'codex/issue-8-gate-2-transition'

$pullRequests = @(
  gh pr list --repo $repo --head $branch --state open --json number,state,isDraft,mergeStateStatus,headRefOid,body,url |
  ConvertFrom-Json
)
if ($LASTEXITCODE -ne 0) { throw '读取 Issue #8 过渡 PR 失败。' }
if ($pullRequests.Count -ne 1 -or
    $pullRequests[0].number -ne 10 -or
    $pullRequests[0].state -ne 'OPEN' -or
    $pullRequests[0].isDraft -or
    $pullRequests[0].mergeStateStatus -ne 'CLEAN' -or
    $pullRequests[0].body -notmatch 'Closes\s+#8') {
  throw 'Issue #8 过渡 PR 状态不符合授权合并条件。'
}

$localHead = git rev-parse HEAD
if ($LASTEXITCODE -ne 0) { throw '读取本地 HEAD 失败。' }
$trackingHead = git rev-parse refs/remotes/origin/codex/issue-8-gate-2-transition
if ($LASTEXITCODE -ne 0) { throw '读取过渡分支远端跟踪 HEAD 失败。' }
if ($localHead -ne $trackingHead -or $localHead -ne $pullRequests[0].headRefOid) {
  throw '本地、远端跟踪和过渡 PR Head 不一致。'
}

$query = 'query($owner:String!,$name:String!,$number:Int!){repository(owner:$owner,name:$name){pullRequest(number:$number){reviewThreads(first:100){nodes{isResolved}} autoMergeRequest{enabledAt}}}}'
$review = gh api graphql -f query=$query -F owner=nonononull -F name=inputcodex -F number=10 | ConvertFrom-Json
if ($LASTEXITCODE -ne 0) { throw '读取过渡 PR Review 对话失败。' }
$unresolved = @($review.data.repository.pullRequest.reviewThreads.nodes | Where-Object { -not $_.isResolved })
if ($unresolved.Count -ne 0 -or $null -ne $review.data.repository.pullRequest.autoMergeRequest) {
  throw '过渡 PR 存在未解决 Review 对话或启用了自动合并。'
}

Write-Output 'ISSUE8_TRANSITION_PR_PREMERGE_VERIFY_OK'
```

## Gate 2 过渡合并后最终核验

本节只在 PR `#10` Squash Merge 后执行：

```powershell
$repo = 'nonononull/inputcodex'

$issue8 = gh issue view 8 --repo $repo --json state,closedAt | ConvertFrom-Json
if ($LASTEXITCODE -ne 0 -or $issue8.state -ne 'CLOSED') {
  throw 'Issue #8 未关闭。'
}
$issue9 = gh issue view 9 --repo $repo --json state | ConvertFrom-Json
if ($LASTEXITCODE -ne 0 -or $issue9.state -ne 'OPEN') {
  throw 'Issue #9 应保持 OPEN。'
}
$pr10 = gh pr view 10 --repo $repo --json state,mergeCommit,statusCheckRollup | ConvertFrom-Json
if ($LASTEXITCODE -ne 0 -or $pr10.state -ne 'MERGED' -or @($pr10.statusCheckRollup).Count -ne 0) {
  throw 'PR #10 未完成预期 Squash Merge。'
}
$main = (gh api repos/$repo/git/ref/heads/main | ConvertFrom-Json).object.sha
if ($LASTEXITCODE -ne 0 -or $main -ne $pr10.mergeCommit.oid) {
  throw 'main 未指向 PR #10 merge commit。'
}
gh api repos/$repo/git/ref/heads/codex/issue-8-gate-2-transition --silent 2>$null
if ($LASTEXITCODE -ne 1) {
  throw 'PR #10 远端过渡分支仍存在或查询异常。'
}

Write-Output 'GATE2_TRANSITION_FINAL_VERIFY_OK'
```

## Git 快照与提交前验证

```powershell
$branch = git branch --show-current
if ($LASTEXITCODE -ne 0 -or $branch -ne 'codex/issue-8-gate-2-transition') {
  throw "当前分支不正确：$branch"
}

$head = git rev-parse HEAD
if ($LASTEXITCODE -ne 0) { throw '读取 HEAD 失败。' }

git status --short --branch
if ($LASTEXITCODE -ne 0) { throw '读取 Git 状态失败。' }
git diff --check
if ($LASTEXITCODE -ne 0) { throw 'git diff --check 失败。' }
git diff --stat
if ($LASTEXITCODE -ne 0) { throw '读取 diff 统计失败。' }

Write-Output "GIT_SNAPSHOT_BRANCH=$branch"
Write-Output "GIT_SNAPSHOT_HEAD=$head"
```

暂存后执行：

```powershell
git diff --cached --check
if ($LASTEXITCODE -ne 0) { throw 'cached diff 检查失败。' }
git diff --cached --stat
if ($LASTEXITCODE -ne 0) { throw '读取 cached diff 统计失败。' }
git status --short --branch
```

## Issue #55 显式性能复测入口本地轻量验证

Issue `#55` 只允许修改十四条路径，`scope_hash` 固定为 `sha256:372c8c3942d492a9372603f5bc6bbae42ae8013c7603a092c294d24be4edb1be`。本机不得运行完整 Workspace、桌面 Release 或真实性能采集；测量行为只在推送后由 GitHub-hosted Runner 的显式 `workflow_dispatch mode=measure` 验收。三条 Issue `#32` Evidence 路径只能使用该 Run 的成功 Artifact 刷新，不能删除或伪造历史样本。

```powershell
$baseline = '0678d03981ac0aef2051eb2d3711221ac2a50d29'
$approvedPaths = @(
  '.github/workflows/performance-baseline.yml'
  'AGENTS.md'
  'benchmarks/results/issue-32/macos.json'
  'benchmarks/results/issue-32/manifest.json'
  'benchmarks/results/issue-32/windows.json'
  'build.md'
  'docs/plans/2026-07-26-issue-55-performance-remeasurement-entry.md'
  'docs/plans/PROJECT-MASTER-PLAN.md'
  'docs/plans/sessions/2026-07-26-issue-55-performance-remeasurement-entry.md'
  'docs/reports/issue-55-performance-remeasurement-entry.md'
  'docs/workflows/2026-07-26-issue-55-performance-remeasurement-entry-runtime.md'
  'err.md'
  'README.md'
  'scripts/ci/Test-CiScripts.ps1'
) | Sort-Object
$committed = @(git diff --name-only "$baseline...HEAD")
$unstaged = @(git diff --name-only)
$staged = @(git diff --cached --name-only)
$untracked = @(git ls-files --others --exclude-standard)
$actualPaths = @($committed + $unstaged + $staged + $untracked | Where-Object { $_ } | Sort-Object -Unique)
$pathDiff = @(Compare-Object -ReferenceObject $approvedPaths -DifferenceObject $actualPaths)
if ($pathDiff.Count -ne 0) { $pathDiff | Format-Table -AutoSize; throw 'Issue #55 实际差异不是批准的十四条路径。' }
$scopeText = ($approvedPaths -join "`n") + "`n"
$scopeBytes = [System.Text.UTF8Encoding]::new($false).GetBytes($scopeText)
$scopeHash = [Convert]::ToHexString([System.Security.Cryptography.SHA256]::HashData($scopeBytes)).ToLowerInvariant()
if ($scopeHash -ne '372c8c3942d492a9372603f5bc6bbae42ae8013c7603a092c294d24be4edb1be') { throw "Issue #55 scope_hash 漂移：$scopeHash" }

pwsh -NoProfile -File scripts/performance/Test-InputcodexBaseline.ps1 -RepositoryRoot . -Mode Evidence
if ($LASTEXITCODE -ne 0) { throw '性能 Evidence 验证失败。' }
pwsh -NoProfile -File scripts/ci/Test-CiScripts.ps1
if ($LASTEXITCODE -ne 0) { throw 'CI 合同验证失败。' }
pwsh -NoProfile -File scripts/ci/Verify-RepositoryPolicy.ps1 -RepositoryRoot .
if ($LASTEXITCODE -ne 0) { throw '仓库政策验证失败。' }
git diff --check
if ($LASTEXITCODE -ne 0) { throw 'git diff --check 失败。' }
Write-Output 'ISSUE_55_LOCAL_GREEN'
```

推送后使用以下命令进行唯一一次 hosted 行为验收；必须在 Windows/macOS 都成功后才允许进入 PR 收口：

```powershell
gh workflow run 'Performance Baseline' --repo nonononull/inputcodex --ref codex/issue-55-performance-remeasurement-entry -f mode=measure
```

## Issue #59 EPYC 7763 四次固定串行复测本地轻量验证

Issue `#59` 的完整允许集合固定为三十八条路径，`scope_hash` 固定为 `sha256:d0577e546d2209d10373eccdf335bbcf3cd4caad7906163838c88b461da0b570`。本机不执行完整 Workspace 或性能采样；四次测量只能使用 GitHub-hosted `Performance Baseline` 的显式 `mode=measure`，且必须严格串行。

控制面、历史证据和任意已完成槽位使用以下命令验证：

```powershell
$baseline = 'd9d1ed77b9796ac6a99e250d1547217a39426aa9'
$approvedPaths = @(
  'AGENTS.md'
  'README.md'
  'benchmarks/budgets/issue-59-approved-observation.json'
  'benchmarks/results/issue-54/manifest.json'
  'benchmarks/results/issue-54/runs/run-01/macos.json'
  'benchmarks/results/issue-54/runs/run-01/windows.json'
  'benchmarks/results/issue-54/runs/run-02/macos.json'
  'benchmarks/results/issue-54/runs/run-02/windows.json'
  'benchmarks/results/issue-54/runs/run-03/macos.json'
  'benchmarks/results/issue-54/runs/run-03/windows.json'
  'benchmarks/results/issue-54/runs/run-04/macos.json'
  'benchmarks/results/issue-54/runs/run-04/windows.json'
  'benchmarks/results/issue-54/runs/run-05/macos.json'
  'benchmarks/results/issue-54/runs/run-05/windows.json'
  'benchmarks/results/issue-54/runs/run-06/macos.json'
  'benchmarks/results/issue-54/runs/run-06/windows.json'
  'benchmarks/results/issue-54/runs/run-07/macos.json'
  'benchmarks/results/issue-54/runs/run-07/windows.json'
  'benchmarks/results/issue-54/runs/run-08/macos.json'
  'benchmarks/results/issue-54/runs/run-08/windows.json'
  'benchmarks/results/issue-59/manifest.json'
  'benchmarks/results/issue-59/runs/run-01/macos.json'
  'benchmarks/results/issue-59/runs/run-01/windows.json'
  'benchmarks/results/issue-59/runs/run-02/macos.json'
  'benchmarks/results/issue-59/runs/run-02/windows.json'
  'benchmarks/results/issue-59/runs/run-03/macos.json'
  'benchmarks/results/issue-59/runs/run-03/windows.json'
  'benchmarks/results/issue-59/runs/run-04/macos.json'
  'benchmarks/results/issue-59/runs/run-04/windows.json'
  'build.md'
  'docs/plans/2026-07-26-issue-59-epyc-7763-fixed-remeasurement.md'
  'docs/plans/PROJECT-MASTER-PLAN.md'
  'docs/plans/sessions/2026-07-26-issue-59-epyc-7763-fixed-remeasurement.md'
  'docs/reports/issue-59-epyc-7763-fixed-remeasurement.md'
  'docs/workflows/2026-07-26-issue-59-epyc-7763-fixed-remeasurement-runtime.md'
  'err.md'
  'scripts/performance/Build-InputcodexBudgetApproval.ps1'
  'scripts/performance/Test-InputcodexBudgetApproval.ps1'
) | Sort-Object

$branch = (git branch --show-current).Trim()
if ($branch -ne 'codex/issue-59-epyc-7763-fixed-remeasurement') {
  throw "Issue #59 当前分支不正确：$branch"
}

$committed = @(git diff --name-only "$baseline...HEAD")
$unstaged = @(git diff --name-only)
$staged = @(git diff --cached --name-only)
$untracked = @(git ls-files --others --exclude-standard)
$actualPaths = @($committed + $unstaged + $staged + $untracked | Where-Object { $_ } | Sort-Object -Unique)
$unexpectedPaths = @($actualPaths | Where-Object { $_ -notin $approvedPaths })
if ($unexpectedPaths.Count -ne 0) {
  $unexpectedPaths | Format-Table -AutoSize
  throw 'Issue #59 出现批准集合外路径。'
}

$scopeText = ($approvedPaths -join "`n") + "`n"
$scopeBytes = [System.Text.UTF8Encoding]::new($false).GetBytes($scopeText)
$scopeHash = [Convert]::ToHexString([System.Security.Cryptography.SHA256]::HashData($scopeBytes)).ToLowerInvariant()
if ($scopeHash -ne 'd0577e546d2209d10373eccdf335bbcf3cd4caad7906163838c88b461da0b570') {
  throw "Issue #59 scope_hash 漂移：$scopeHash"
}

function Get-NormalizedTextSha256 {
  param([Parameter(Mandatory = $true)][string]$Path)
  $text = [System.IO.File]::ReadAllText((Resolve-Path -LiteralPath $Path).Path)
  $normalized = $text.Replace("`r`n", "`n").Replace("`r", "`n")
  $bytes = [System.Text.UTF8Encoding]::new($false).GetBytes($normalized)
  return 'sha256:' + [Convert]::ToHexString([System.Security.Cryptography.SHA256]::HashData($bytes)).ToLowerInvariant()
}

$historicalManifestPath = 'benchmarks/results/issue-54/manifest.json'
$historicalManifestHash = Get-NormalizedTextSha256 -Path $historicalManifestPath
if ($historicalManifestHash -ne 'sha256:72567fe96f61d19d4eca8a5347e3d3fcea7df823975946ec3f464a43d229f1ae') {
  throw "Issue #54 只读 manifest 漂移：$historicalManifestHash"
}
$historicalManifest = Get-Content -LiteralPath $historicalManifestPath -Raw | ConvertFrom-Json -Depth 100
if ($historicalManifest.runs.Count -ne 8) { throw 'Issue #54 历史 Run 数不是 8。' }
foreach ($run in $historicalManifest.runs) {
  foreach ($platform in @('windows', 'macos')) {
    $result = $run.results.$platform
    if ((Get-NormalizedTextSha256 -Path $result.path) -ne $result.normalized_sha256) {
      throw "Issue #54 历史结果哈希漂移：$($result.path)"
    }
    Get-Content -LiteralPath $result.path -Raw | ConvertFrom-Json -Depth 100 | Out-Null
  }
}

$manifest = Get-Content -LiteralPath 'benchmarks/results/issue-59/manifest.json' -Raw | ConvertFrom-Json -Depth 100
if ($manifest.issue_number -ne 59) { throw 'Issue #59 manifest issue_number 错误。' }
if ($manifest.base.max_serial_runs -ne 4) { throw 'Issue #59 max_serial_runs 必须为 4。' }
if ($manifest.policy.target_windows_processor -ne 'AMD EPYC 7763 64-Core Processor') { throw 'Issue #59 目标处理器漂移。' }
if ($manifest.policy.target_windows_environment_fingerprint_sha256 -ne 'sha256:f3954543f3cec519568345d9f40341ddeb8991a7d93b3a274cc324b047fb00cb') { throw 'Issue #59 目标完整环境指纹漂移。' }
if (@($manifest.historical_evidence.windows_target_candidate_slots).Count -ne 3) { throw 'Issue #59 历史严格目标样本数必须为 3。' }
if ('run-04' -notin @($manifest.historical_evidence.windows_same_processor_other_fingerprint_slots)) { throw 'Issue #59 必须保留同 CPU 不同指纹的 run-04。' }
if ($manifest.runs.Count -gt 4) { throw 'Issue #59 出现超过四个测量槽位。' }
$expectedSlots = @('run-01', 'run-02', 'run-03', 'run-04')
foreach ($run in $manifest.runs) {
  if ($run.slot -notin $expectedSlots) { throw "Issue #59 非法槽位：$($run.slot)" }
  foreach ($platform in @('windows', 'macos')) {
    $result = $run.results.$platform
    if ((Get-NormalizedTextSha256 -Path $result.path) -ne $result.normalized_sha256) {
      throw "Issue #59 结果哈希漂移：$($result.path)"
    }
    Get-Content -LiteralPath $result.path -Raw | ConvertFrom-Json -Depth 100 | Out-Null
  }
}

pwsh -NoProfile -File scripts/performance/Test-InputcodexBaseline.ps1 -RepositoryRoot . -Mode Evidence
if ($LASTEXITCODE -ne 0) { throw '性能 Evidence 验证失败。' }
pwsh -NoProfile -File scripts/ci/Test-CiScripts.ps1
if ($LASTEXITCODE -ne 0) { throw 'CI 合同验证失败。' }
pwsh -NoProfile -File scripts/ci/Verify-RepositoryPolicy.ps1 -RepositoryRoot .
if ($LASTEXITCODE -ne 0) { throw '仓库政策验证失败。' }
git diff --check
if ($LASTEXITCODE -ne 0) { throw 'git diff --check 失败。' }
Write-Output "ISSUE_59_CONTROL_GREEN actual_paths=$($actualPaths.Count) runs=$($manifest.runs.Count)"
```

四个槽位全部结束且预算控制面实际生成后，再执行：

```powershell
$budgetPreviewPath = Join-Path $env:TEMP 'inputcodex-issue59-budget-preview.json'
pwsh -NoProfile -File scripts/performance/Build-InputcodexBudgetApproval.ps1 `
  -RepositoryRoot . `
  -OutputPath $budgetPreviewPath
if ($LASTEXITCODE -ne 0) { throw '性能预算离线构建失败。' }

pwsh -NoProfile -File scripts/performance/Test-InputcodexBudgetApproval.ps1 -RepositoryRoot .
if ($LASTEXITCODE -ne 0) { throw '性能预算数值验证失败。' }

Remove-Item -LiteralPath $budgetPreviewPath -Force
```

验证器会在系统临时目录重新生成预算 JSON，与入库文档做归一化哈希比较，并独立复算 center、MAD、安全裕量和量子舍入；期望输出 `BUDGET_APPROVAL_GREEN passed=10`。预算 JSON 固定为 `approved-observation` 证据，`budget_ci_enabled=false`、`gate_5_unlocked=false`。

每个槽位只允许在前一个槽位完全闭环后触发：

```powershell
gh workflow run 'Performance Baseline' --repo nonononull/inputcodex --ref codex/issue-59-epyc-7763-fixed-remeasurement -f mode=measure
```

## Issue #65：`v1.2.43` 功能目录重新审计

本任务只允许在已批准二十四路径内运行本地轻量验证；完整 Workspace 与三平台构建继续交给标准 GitHub-hosted CI。

```powershell
cargo test -p inputcodex-parity --test catalog_repository --offline
pwsh -NoProfile -File scripts/ci/Test-CiScripts.ps1
pwsh -NoProfile -File scripts/ci/Verify-RepositoryPolicy.ps1 -RepositoryRoot .
git diff --check
```

二十四路径范围哈希使用项目所有者 Windows 本机 PowerShell 的 `Sort-Object`、UTF-8 无 BOM、LF 拼接和末尾换行复算：

```powershell
$paths = @(
  'AGENTS.md',
  'README.md',
  'build.md',
  'crates/inputcodex-parity/tests/catalog_repository.rs',
  'docs/plans/2026-07-27-issue-65-v1.2.43-catalog-reaudit.md',
  'docs/plans/PROJECT-MASTER-PLAN.md',
  'docs/plans/sessions/2026-07-27-issue-65-v1.2.43-catalog-reaudit.md',
  'docs/reports/issue-65-v1.2.43-catalog-reaudit-discovery.md',
  'docs/workflows/2026-07-27-issue-65-v1.2.43-catalog-reaudit-runtime.md',
  'parity/README.md',
  'parity/contracts/foundation-platform.yml',
  'parity/contracts/plugin-script.yml',
  'parity/contracts/provider-network.yml',
  'parity/contracts/remote-install.yml',
  'parity/contracts/session-data.yml',
  'parity/features/foundation-platform.yml',
  'parity/features/plugin-script.yml',
  'parity/features/provider-network.yml',
  'parity/features/remote-install.yml',
  'parity/features/session-data.yml',
  'parity/features/source-index.yml',
  'parity/fixtures/feature.session-data.provider-metadata-maintenance/baseline.yml',
  'parity/fixtures/feature.session-data.provider-metadata-maintenance/manifest.yml',
  'upstream/source-lock.json'
) | Sort-Object
$text = ($paths -join "`n") + "`n"
$bytes = [System.Text.UTF8Encoding]::new($false).GetBytes($text)
$hash = [System.Security.Cryptography.SHA256]::HashData($bytes)
$scopeHash = 'sha256:' + [Convert]::ToHexString($hash).ToLowerInvariant()
if ($scopeHash -ne 'sha256:82234e7aacce0bd6c57994529ccf74371052ed906dc8371324b90e41f697d7b7') {
  throw "Issue #65 范围哈希漂移：$scopeHash"
}
```

期望目录测试为 `12 passed; 0 failed`，来源、feature、合同、fixture、例外、排除与覆盖缺口计数分别保持 `133/36/36/11/10/3/0`；`release_audit.status=current` 且 stale 字段为 `null`。

## 外部 AGOS 使用边界

Issue `#17` 曾以 report-only 运行 AGOS 默认入口，结果为 `needs-input/unregistered`；已按项目规则记录并绕过。AGOS 不属于环境要求或合并门禁；不得在本规划 PR 中修改、修复或优化其 Registry、脚本、规则、Workflow 或 Vault。

## 后续维护规则

- 后续任何 `upstream/` 或 `source-lock.json` 修改必须使用新的 upstream-sync Issue/PR，并更新锁定文件、同步报告和本节快照验证常量。
- Issue `#14` / PR `#15` 与两次真实运行已经完成；Issue `#17` 只批准 Gate 3 规划，不得扩展为 Workspace、CI、UI 或功能实现授权。
- 建立首个 Cargo Workspace 时再加入 Rust 构建、测试、基准和三平台 CI 命令。
- 任何错误先查 `err.md`，重复问题优先复用既有根因与处理方案。
