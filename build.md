# inputcodex 构建与验证说明

## 当前状态

截至 2026 年 7 月 22 日，PR `#21` 已将 Gate 3 七成员 Workspace、首版无缓存三平台 CI、五类失败语义与三平台最低冷构建基线 Squash Merge 到 `main`；PR `#23` 已完成独立 closeout。Issue `#24` / PR `#25` 已将 Gate 4 两阶段规划合同 Squash Merge 为 `431682296f53e86de1184c732b0d4748857c9390`，Issue `#24` 已按 `COMPLETED` 关闭。Issue `#26` 的 source-index 与五域功能目录 checkpoint `87537e6e4a0e6911dd1427cc23f52dcb805a4679` 已普通 push，Issue 评论 `5048930060` 记录 `133` 条入口、`36` 个 feature、`3` 个排除和 `0` 个覆盖缺口；当前进入合同与 fixture，最终合并仍需独立授权。

仓库当前有 `upstream/CodexPlusPlus/` 审计快照、七成员纯 Rust Workspace 和首版无缓存三平台 `CI` Workflow。本文件当前提供十四个检查点：

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

当前禁止：

- 在没有新的独立 upstream-sync Issue/PR 与项目所有者批准时修改 `upstream/` 或 `source-lock.json`。
- 把三平台各 `3/3` 次最低冷构建基线解释为已经完成 Cache、P95、七天观测或最终性能预算。
- 在 Issue `#26` control-plane checkpoint 中创建实际 `parity/` 数据或 `benchmarks/`，或修改 Cargo、Rust、测试、CI、`upstream/`、Ruleset 或 AGOS。
- 创建 Release Workflow、安装包、签名、更新资产、临时 UI 或 WebView。
- 修改 Ruleset、required checks 或仓库级合并开关。
- 修改或优化外部 AGOS。

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

## 外部 AGOS 使用边界

Issue `#17` 曾以 report-only 运行 AGOS 默认入口，结果为 `needs-input/unregistered`；已按项目规则记录并绕过。AGOS 不属于环境要求或合并门禁；不得在本规划 PR 中修改、修复或优化其 Registry、脚本、规则、Workflow 或 Vault。

## 后续维护规则

- 后续任何 `upstream/` 或 `source-lock.json` 修改必须使用新的 upstream-sync Issue/PR，并更新锁定文件、同步报告和本节快照验证常量。
- Issue `#14` / PR `#15` 与两次真实运行已经完成；Issue `#17` 只批准 Gate 3 规划，不得扩展为 Workspace、CI、UI 或功能实现授权。
- 建立首个 Cargo Workspace 时再加入 Rust 构建、测试、基准和三平台 CI 命令。
- 任何错误先查 `err.md`，重复问题优先复用既有根因与处理方案。
