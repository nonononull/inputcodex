[CmdletBinding()]
param()

Set-StrictMode -Version Latest
$ErrorActionPreference = 'Stop'

$scriptDirectory = Split-Path -Parent $PSCommandPath
$repositoryRoot = (Resolve-Path -LiteralPath (Join-Path $scriptDirectory '../..')).Path
$classifierScript = Join-Path $scriptDirectory 'Classify-Changes.ps1'
$policyScript = Join-Path $scriptDirectory 'Verify-RepositoryPolicy.ps1'
$collectorScript = Join-Path $scriptDirectory 'Collect-Changes.ps1'
$releaseAuditGateScript = Join-Path $scriptDirectory 'Verify-ReleaseAuditGate.ps1'
$workflowPath = Join-Path $repositoryRoot '.github/workflows/ci.yml'
$missingImplementations = @(
    @(
        $classifierScript
        $policyScript
        $releaseAuditGateScript
    ) | Where-Object { -not (Test-Path -LiteralPath $_ -PathType Leaf) }
)

if ($missingImplementations.Count -gt 0) {
    [Console]::Error.WriteLine('CI_CONTRACT_RED_MISSING_IMPLEMENTATION')
    foreach ($missingImplementation in $missingImplementations) {
        [Console]::Error.WriteLine("missing=$missingImplementation")
    }

    exit 10
}

$script:PowerShellExecutable = (Get-Process -Id $PID).Path
$script:PassedCount = 0
$script:Failures = [System.Collections.Generic.List[string]]::new()
$testRoot = Join-Path ([System.IO.Path]::GetTempPath()) ("inputcodex-ci-contract-{0}" -f [guid]::NewGuid().ToString('N'))
New-Item -ItemType Directory -Path $testRoot -Force | Out-Null

function Invoke-ContractTest {
    param(
        [Parameter(Mandatory)]
        [string]$Name,

        [Parameter(Mandatory)]
        [scriptblock]$Body
    )

    try {
        & $Body
        $script:PassedCount += 1
        Write-Host "PASS $Name"
    }
    catch {
        $message = "FAIL $Name :: $($_.Exception.Message)"
        $script:Failures.Add($message)
        Write-Host $message
    }
}

function Assert-Equal {
    param(
        [Parameter(Mandatory)]
        $Expected,

        [Parameter(Mandatory)]
        $Actual,

        [Parameter(Mandatory)]
        [string]$Message
    )

    if ($Expected -ne $Actual) {
        throw "$Message；期望=<$Expected>，实际=<$Actual>"
    }
}

function Assert-True {
    param(
        [Parameter(Mandatory)]
        [bool]$Condition,

        [Parameter(Mandatory)]
        [string]$Message
    )

    if (-not $Condition) {
        throw $Message
    }
}

function Assert-Contains {
    param(
        [Parameter(Mandatory)]
        [object[]]$Collection,

        [Parameter(Mandatory)]
        $Expected,

        [Parameter(Mandatory)]
        [string]$Message
    )

    if ($Collection -notcontains $Expected) {
        throw "$Message；缺少=<$Expected>"
    }
}

function Invoke-ChildScript {
    param(
        [Parameter(Mandatory)]
        [string]$Path,

        [string[]]$Arguments = @()
    )

    $outputLines = @(& $script:PowerShellExecutable -NoLogo -NoProfile -File $Path @Arguments 2>&1)
    $exitCode = $LASTEXITCODE
    $output = ($outputLines | ForEach-Object { $_.ToString() }) -join [Environment]::NewLine
    $json = $null

    if (-not [string]::IsNullOrWhiteSpace($output)) {
        try {
            $json = $output | ConvertFrom-Json -Depth 100
        }
        catch {
            $json = $null
        }
    }

    [pscustomobject]@{
        ExitCode = $exitCode
        Output = $output
        Json = $json
    }
}

function Write-JsonFile {
    param(
        [Parameter(Mandatory)]
        [string]$Path,

        [Parameter(Mandatory)]
        $Value
    )

    $json = ConvertTo-Json -InputObject $Value -Depth 20
    Set-Content -LiteralPath $Path -Value $json -Encoding utf8NoBOM
}

function New-ReleaseAuditSourceLock {
    param(
        [Parameter(Mandatory)]
        [string]$SnapshotTag,

        [Parameter(Mandatory)]
        [string]$SnapshotCommit,

        [Parameter(Mandatory)]
        [string]$CatalogTag,

        [Parameter(Mandatory)]
        [string]$CatalogCommit,

        [Parameter(Mandatory)]
        [string]$Status,

        [AllowNull()]
        [object]$StaleReason,

        [AllowNull()]
        [object]$ReAuditIssueRef
    )

    [pscustomobject][ordered]@{
        snapshot = [pscustomobject][ordered]@{
            release_tag = $SnapshotTag
            commit = $SnapshotCommit
        }
        release_audit = [pscustomobject][ordered]@{
            schema_version = 'inputcodex.release-audit.v1'
            catalog_release = [pscustomobject][ordered]@{
                tag = $CatalogTag
                commit = $CatalogCommit
            }
            status = $Status
            stale_reason = $StaleReason
            re_audit_issue_ref = $ReAuditIssueRef
        }
    }
}

function Invoke-ReleaseAuditGateCase {
    param(
        [Parameter(Mandatory)]
        [string]$Name,

        [Parameter(Mandatory)]
        $BaseSourceLock,

        [Parameter(Mandatory)]
        $HeadSourceLock,

        [Parameter(Mandatory)]
        [AllowEmptyCollection()]
        [object[]]$Changes
    )

    $caseRoot = Join-Path $testRoot ("release-audit-{0}" -f $Name)
    $upstreamRoot = Join-Path $caseRoot 'upstream'
    New-Item -ItemType Directory -Path $upstreamRoot -Force | Out-Null

    $baseSourceLockPath = Join-Path $caseRoot 'base-source-lock.json'
    $headSourceLockPath = Join-Path $upstreamRoot 'source-lock.json'
    $changesPath = Join-Path $caseRoot 'changes.json'
    Write-JsonFile -Path $baseSourceLockPath -Value $BaseSourceLock
    Write-JsonFile -Path $headSourceLockPath -Value $HeadSourceLock
    Write-JsonFile -Path $changesPath -Value $Changes

    Invoke-ChildScript -Path $releaseAuditGateScript -Arguments @(
        '-RepositoryRoot', $caseRoot,
        '-InputFile', $changesPath,
        '-BaseSourceLockPath', $baseSourceLockPath
    )
}

function Assert-ReleaseAuditSuccess {
    param(
        [Parameter(Mandatory)]
        $Result,

        [Parameter(Mandatory)]
        [string]$ExpectedStatus,

        [Parameter(Mandatory)]
        [bool]$ExpectedReaudit
    )

    Assert-Equal -Expected 0 -Actual $Result.ExitCode -Message "Release 审计门应通过，输出=$($Result.Output)"
    Assert-True -Condition ($null -ne $Result.Json) -Message "Release 审计门必须输出 JSON，输出=$($Result.Output)"
    Assert-Equal -Expected $true -Actual $Result.Json.ok -Message 'Release 审计门通过时必须标记 ok'
    Assert-Equal -Expected $ExpectedStatus -Actual $Result.Json.status -Message 'Release 审计状态必须可诊断'
    Assert-Equal -Expected $ExpectedReaudit -Actual $Result.Json.requires_reaudit -Message '重新审计要求必须可诊断'
}

function Assert-ReleaseAuditFailureCode {
    param(
        [Parameter(Mandatory)]
        $Result,

        [Parameter(Mandatory)]
        [string]$Code
    )

    Assert-True -Condition ($Result.ExitCode -ne 0) -Message 'Release 审计门拒绝路径时必须返回非零退出码'
    Assert-True -Condition ($null -ne $Result.Json) -Message "Release 审计门失败时必须输出 JSON，输出=$($Result.Output)"
    Assert-Contains -Collection @($Result.Json.errors.code) -Expected $Code -Message 'Release 审计门必须返回稳定错误码'
}

function Invoke-ClassifierCase {
    param(
        [Parameter(Mandatory)]
        [string]$Name,

        [Parameter(Mandatory)]
        [AllowEmptyCollection()]
        [object[]]$Changes
    )

    $inputPath = Join-Path $testRoot ("classify-{0}.json" -f $Name)
    Write-JsonFile -Path $inputPath -Value $Changes
    Invoke-ChildScript -Path $classifierScript -Arguments @('-InputFile', $inputPath)
}

function Assert-ClassifierSuccess {
    param(
        [Parameter(Mandatory)]
        $Result
    )

    Assert-Equal -Expected 0 -Actual $Result.ExitCode -Message "路径分类脚本应成功，输出=$($Result.Output)"
    Assert-True -Condition ($null -ne $Result.Json) -Message "路径分类脚本必须输出 JSON，输出=$($Result.Output)"
}

function Assert-ClassifierFailureCode {
    param(
        [Parameter(Mandatory)]
        $Result,

        [Parameter(Mandatory)]
        [string]$Code
    )

    Assert-True -Condition ($Result.ExitCode -ne 0) -Message '非法路径输入必须返回非零退出码'
    Assert-True -Condition ($null -ne $Result.Json) -Message "非法路径输入必须输出 JSON 错误，输出=$($Result.Output)"
    Assert-Contains -Collection @($Result.Json.errors.code) -Expected $Code -Message '非法路径输入必须返回稳定错误码'
}

Invoke-ContractTest -Name '空 diff 返回确定的空分类' -Body {
    $result = Invoke-ClassifierCase -Name 'empty' -Changes @()
    Assert-ClassifierSuccess -Result $result
    Assert-Equal -Expected $false -Actual $result.Json.has_changes -Message '空 diff 不应标记为有变化'
    Assert-Equal -Expected $false -Actual $result.Json.docs_only -Message '空 diff 不应伪装为文档变更'
    Assert-Equal -Expected $false -Actual $result.Json.heavy -Message '空 diff 不应触发重型任务'
    Assert-Equal -Expected 0 -Actual $result.Json.change_count -Message '空 diff 的记录数必须为零'
}

Invoke-ContractTest -Name '纯文档 diff 不触发重型任务' -Body {
    $changes = @(
        [pscustomobject]@{ status = 'M'; path = 'docs/guide.md' }
        [pscustomobject]@{ status = 'A'; path = 'README.md' }
    )
    $result = Invoke-ClassifierCase -Name 'docs-only' -Changes $changes
    Assert-ClassifierSuccess -Result $result
    Assert-Equal -Expected $true -Actual $result.Json.has_changes -Message '文档 diff 应标记为有变化'
    Assert-Equal -Expected $true -Actual $result.Json.docs_only -Message '纯文档 diff 应被识别'
    Assert-Equal -Expected $false -Actual $result.Json.heavy -Message '纯文档 diff 不应触发重型任务'
    Assert-Contains -Collection @($result.Json.docs_paths) -Expected 'README.md' -Message '文档路径必须进入 docs_paths'
}

Invoke-ContractTest -Name 'Rust 源码 diff 触发重型任务' -Body {
    $changes = @(
        [pscustomobject]@{ status = 'M'; path = 'crates/inputcodex-domain/src/lib.rs' }
    )
    $result = Invoke-ClassifierCase -Name 'heavy-rust' -Changes $changes
    Assert-ClassifierSuccess -Result $result
    Assert-Equal -Expected $false -Actual $result.Json.docs_only -Message 'Rust 源码不能归为纯文档'
    Assert-Equal -Expected $true -Actual $result.Json.heavy -Message 'Rust 源码必须触发重型任务'
    Assert-Contains -Collection @($result.Json.heavy_paths) -Expected 'crates/inputcodex-domain/src/lib.rs' -Message '重型路径必须可审计'
}

Invoke-ContractTest -Name '删除记录参与分类' -Body {
    $changes = @(
        [pscustomobject]@{ status = 'D'; path = 'Cargo.lock' }
    )
    $result = Invoke-ClassifierCase -Name 'deleted-lock' -Changes $changes
    Assert-ClassifierSuccess -Result $result
    Assert-Equal -Expected $true -Actual $result.Json.heavy -Message '删除 Cargo.lock 必须触发重型任务'
    Assert-Contains -Collection @($result.Json.changed_paths) -Expected 'Cargo.lock' -Message '删除路径必须保留在输出中'
}

Invoke-ContractTest -Name '重命名同时审计新旧路径' -Body {
    $changes = @(
        [pscustomobject]@{ status = 'R'; old_path = 'docs/old.md'; path = 'docs/new.md' }
    )
    $result = Invoke-ClassifierCase -Name 'renamed-doc' -Changes $changes
    Assert-ClassifierSuccess -Result $result
    Assert-Equal -Expected $true -Actual $result.Json.docs_only -Message '文档间重命名仍应归为纯文档'
    Assert-Contains -Collection @($result.Json.changed_paths) -Expected 'docs/old.md' -Message '重命名旧路径必须保留'
    Assert-Contains -Collection @($result.Json.changed_paths) -Expected 'docs/new.md' -Message '重命名新路径必须保留'
}

$invalidPathCases = @(
    [pscustomobject]@{ name = 'path-traversal'; path = '../Cargo.toml' }
    [pscustomobject]@{ name = 'absolute-posix'; path = '/tmp/file.rs' }
    [pscustomobject]@{ name = 'absolute-windows'; path = 'C:/repo/file.rs' }
    [pscustomobject]@{ name = 'backslash'; path = 'docs\guide.md' }
    [pscustomobject]@{ name = 'control-character'; path = "docs/$([char]1)guide.md" }
)

foreach ($invalidPathCase in $invalidPathCases) {
    Invoke-ContractTest -Name "拒绝非法路径 $($invalidPathCase.name)" -Body {
        $changes = @(
            [pscustomobject]@{ status = 'M'; path = $invalidPathCase.path }
        )
        $result = Invoke-ClassifierCase -Name $invalidPathCase.name -Changes $changes
        Assert-ClassifierFailureCode -Result $result -Code 'INVALID_PATH'
    }
}

Invoke-ContractTest -Name '重命名缺失旧路径时失败' -Body {
    $changes = @(
        [pscustomobject]@{ status = 'R'; path = 'docs/new.md' }
    )
    $result = Invoke-ClassifierCase -Name 'rename-without-old-path' -Changes $changes
    Assert-ClassifierFailureCode -Result $result -Code 'OLD_PATH_REQUIRED'
}

Invoke-ContractTest -Name 'Release 审计门区分 current 与 stale 并阻断产品路径' -Body {
    $current = New-ReleaseAuditSourceLock `
        -SnapshotTag 'v1.2.41' `
        -SnapshotCommit '3dafffcafb2566a1e8bce4b35671656d6adb3eda' `
        -CatalogTag 'v1.2.41' `
        -CatalogCommit '3dafffcafb2566a1e8bce4b35671656d6adb3eda' `
        -Status 'current' `
        -StaleReason $null `
        -ReAuditIssueRef $null
    $stale = New-ReleaseAuditSourceLock `
        -SnapshotTag 'v1.2.42' `
        -SnapshotCommit '657cd33e009ad02515d30db6492cd4e669b06318' `
        -CatalogTag 'v1.2.41' `
        -CatalogCommit '3dafffcafb2566a1e8bce4b35671656d6adb3eda' `
        -Status 'stale-re-audit-required' `
        -StaleReason '上游 v1.2.42 已缓存，功能目录尚未完成复审' `
        -ReAuditIssueRef 'https://github.com/nonononull/inputcodex/issues/34'

    $result = Invoke-ReleaseAuditGateCase `
        -Name 'current-product-change' `
        -BaseSourceLock $current `
        -HeadSourceLock $current `
        -Changes @([pscustomobject]@{ status = 'M'; path = 'apps/inputcodex-desktop/src/main.rs' })
    Assert-ReleaseAuditSuccess -Result $result -ExpectedStatus 'current' -ExpectedReaudit $false

    $result = Invoke-ReleaseAuditGateCase `
        -Name 'current-empty-change-set' `
        -BaseSourceLock $current `
        -HeadSourceLock $current `
        -Changes @()
    Assert-ReleaseAuditSuccess -Result $result -ExpectedStatus 'current' -ExpectedReaudit $false

    $result = Invoke-ReleaseAuditGateCase `
        -Name 'stale-reaudit-only' `
        -BaseSourceLock $current `
        -HeadSourceLock $stale `
        -Changes @(
            [pscustomobject]@{ status = 'M'; path = 'upstream/source-lock.json' }
            [pscustomobject]@{ status = 'M'; path = 'upstream/CodexPlusPlus/README.md' }
            [pscustomobject]@{ status = 'M'; path = 'docs/reports/2026-07-22-upstream-v1.2.42-sync.md' }
            [pscustomobject]@{ status = 'M'; path = 'parity/features/source-index.yml' }
            [pscustomobject]@{ status = 'M'; path = 'crates/inputcodex-parity/src/validation.rs' }
        )
    Assert-ReleaseAuditSuccess -Result $result -ExpectedStatus 'stale-re-audit-required' -ExpectedReaudit $true

    foreach ($blockedPath in @(
        'benchmarks/cold-start.rs'
        'apps/inputcodex-desktop/src/main.rs'
        'crates/inputcodex-domain/src/lib.rs'
        'Cargo.toml'
        'Cargo.lock'
    )) {
        $result = Invoke-ReleaseAuditGateCase `
            -Name ("stale-blocked-{0}" -f ($blockedPath -replace '[^A-Za-z0-9]+', '-')) `
            -BaseSourceLock $current `
            -HeadSourceLock $stale `
            -Changes @([pscustomobject]@{ status = 'M'; path = $blockedPath })
        Assert-ReleaseAuditFailureCode -Result $result -Code 'RELEASE_AUDIT_REAUDIT_REQUIRED'
    }

    $renewedCurrent = New-ReleaseAuditSourceLock `
        -SnapshotTag 'v1.2.42' `
        -SnapshotCommit '657cd33e009ad02515d30db6492cd4e669b06318' `
        -CatalogTag 'v1.2.42' `
        -CatalogCommit '657cd33e009ad02515d30db6492cd4e669b06318' `
        -Status 'current' `
        -StaleReason $null `
        -ReAuditIssueRef $null
    $result = Invoke-ReleaseAuditGateCase `
        -Name 'audit-changed-with-product' `
        -BaseSourceLock $current `
        -HeadSourceLock $renewedCurrent `
        -Changes @(
            [pscustomobject]@{ status = 'M'; path = 'upstream/source-lock.json' }
            [pscustomobject]@{ status = 'M'; path = 'apps/inputcodex-desktop/src/main.rs' }
        )
    Assert-ReleaseAuditFailureCode -Result $result -Code 'RELEASE_AUDIT_CHANGED_WITH_BLOCKED_PATH'

    $invalidStale = New-ReleaseAuditSourceLock `
        -SnapshotTag 'v1.2.42' `
        -SnapshotCommit '657cd33e009ad02515d30db6492cd4e669b06318' `
        -CatalogTag 'v1.2.41' `
        -CatalogCommit '3dafffcafb2566a1e8bce4b35671656d6adb3eda' `
        -Status 'stale-re-audit-required' `
        -StaleReason '' `
        -ReAuditIssueRef 'https://github.com/nonononull/inputcodex/issues/34'
    $result = Invoke-ReleaseAuditGateCase `
        -Name 'invalid-stale-reason' `
        -BaseSourceLock $current `
        -HeadSourceLock $invalidStale `
        -Changes @([pscustomobject]@{ status = 'M'; path = 'parity/features/source-index.yml' })
    Assert-ReleaseAuditFailureCode -Result $result -Code 'RELEASE_AUDIT_INVALID'
}

Invoke-ContractTest -Name '冷构建指标同时写入日志与摘要' -Body {
    Assert-True -Condition (Test-Path -LiteralPath $workflowPath -PathType Leaf) -Message 'CI Workflow 必须存在'
    $workflow = Get-Content -LiteralPath $workflowPath -Raw

    Assert-Equal -Expected 3 -Actual ([regex]::Matches($workflow, [regex]::Escape('$metrics = Get-Content')).Count) -Message '三个平台都必须显式读取冷构建指标'
    Assert-Equal -Expected 3 -Actual ([regex]::Matches($workflow, [regex]::Escape('$metrics | Write-Output')).Count) -Message '三个平台都必须把冷构建指标写入控制台日志'
    Assert-Equal -Expected 3 -Actual ([regex]::Matches($workflow, [regex]::Escape('$metrics | Add-Content -LiteralPath $env:GITHUB_STEP_SUMMARY')).Count) -Message '三个平台都必须把冷构建指标写入 Step Summary'
}

Invoke-ContractTest -Name 'Release 审计门接入 PR 与 required 汇总' -Body {
    Assert-True -Condition (Test-Path -LiteralPath $workflowPath -PathType Leaf) -Message 'CI Workflow 必须存在'
    $workflow = Get-Content -LiteralPath $workflowPath -Raw

    Assert-True -Condition ($workflow -match '(?m)^  release-audit:$') -Message 'CI 必须存在独立 release-audit Job'
    Assert-True -Condition ($workflow -match 'Verify-ReleaseAuditGate\.ps1') -Message 'release-audit Job 必须执行审计门脚本'
    Assert-True -Condition ($workflow -match '(?s)required:.*?needs:.*?- release-audit') -Message 'required Job 必须依赖 release-audit Job'
}

function Write-Utf8File {
    param(
        [Parameter(Mandatory)]
        [string]$Path,

        [Parameter(Mandatory)]
        [string]$Content
    )

    $parent = Split-Path -Parent $Path
    if (-not [string]::IsNullOrWhiteSpace($parent)) {
        New-Item -ItemType Directory -Path $parent -Force | Out-Null
    }

    Set-Content -LiteralPath $Path -Value $Content -Encoding utf8NoBOM
}

function New-ValidRepositoryFixture {
    param(
        [Parameter(Mandatory)]
        [string]$Path
    )

    New-Item -ItemType Directory -Path $Path -Force | Out-Null

    $workspaceManifest = @"
[workspace]
resolver = "2"
members = [
    "apps/inputcodex-desktop",
    "crates/inputcodex-domain",
    "crates/inputcodex-application",
    "crates/inputcodex-infrastructure",
    "crates/inputcodex-platform",
    "crates/inputcodex-presentation",
    "crates/inputcodex-parity"
]

[workspace.package]
version = "0.1.0"
edition = "2024"
rust-version = "1.97"
license = "AGPL-3.0-only"
"@
    Write-Utf8File -Path (Join-Path $Path 'Cargo.toml') -Content $workspaceManifest

    $domainManifest = @"
[package]
name = "inputcodex-domain"
version.workspace = true
edition.workspace = true
rust-version.workspace = true
license.workspace = true
"@
    Write-Utf8File -Path (Join-Path $Path 'crates/inputcodex-domain/Cargo.toml') -Content $domainManifest
    Write-Utf8File -Path (Join-Path $Path 'crates/inputcodex-domain/src/lib.rs') -Content 'pub struct DomainMarker;'

    $applicationManifest = @"
[package]
name = "inputcodex-application"
version.workspace = true
edition.workspace = true
rust-version.workspace = true
license.workspace = true

[dependencies]
inputcodex-domain = { path = "../inputcodex-domain" }
"@
    Write-Utf8File -Path (Join-Path $Path 'crates/inputcodex-application/Cargo.toml') -Content $applicationManifest
    Write-Utf8File -Path (Join-Path $Path 'crates/inputcodex-application/src/lib.rs') -Content 'pub struct ApplicationMarker;'

    $infrastructureManifest = @"
[package]
name = "inputcodex-infrastructure"
version.workspace = true
edition.workspace = true
rust-version.workspace = true
license.workspace = true

[dependencies]
inputcodex-application = { path = "../inputcodex-application" }
"@
    Write-Utf8File -Path (Join-Path $Path 'crates/inputcodex-infrastructure/Cargo.toml') -Content $infrastructureManifest
    Write-Utf8File -Path (Join-Path $Path 'crates/inputcodex-infrastructure/src/lib.rs') -Content 'pub struct InfrastructureMarker;'

    $platformManifest = @"
[package]
name = "inputcodex-platform"
version.workspace = true
edition.workspace = true
rust-version.workspace = true
license.workspace = true

[dependencies]
inputcodex-application = { path = "../inputcodex-application" }
"@
    Write-Utf8File -Path (Join-Path $Path 'crates/inputcodex-platform/Cargo.toml') -Content $platformManifest
    Write-Utf8File -Path (Join-Path $Path 'crates/inputcodex-platform/src/lib.rs') -Content 'pub struct PlatformMarker;'

    $presentationManifest = @"
[package]
name = "inputcodex-presentation"
version.workspace = true
edition.workspace = true
rust-version.workspace = true
license.workspace = true

[dependencies]
inputcodex-application = { path = "../inputcodex-application" }
iced = "0.14.0"
"@
    Write-Utf8File -Path (Join-Path $Path 'crates/inputcodex-presentation/Cargo.toml') -Content $presentationManifest
    Write-Utf8File -Path (Join-Path $Path 'crates/inputcodex-presentation/src/lib.rs') -Content 'pub struct PresentationMarker;'

    $parityManifest = @"
[package]
name = "inputcodex-parity"
version.workspace = true
edition.workspace = true
rust-version.workspace = true
license.workspace = true

[dependencies]
inputcodex-domain = { path = "../inputcodex-domain" }
inputcodex-application = { path = "../inputcodex-application" }
"@
    Write-Utf8File -Path (Join-Path $Path 'crates/inputcodex-parity/Cargo.toml') -Content $parityManifest
    Write-Utf8File -Path (Join-Path $Path 'crates/inputcodex-parity/src/lib.rs') -Content 'pub struct ParityMarker;'

    $desktopManifest = @"
[package]
name = "inputcodex-desktop"
version.workspace = true
edition.workspace = true
rust-version.workspace = true
license.workspace = true

[dependencies]
inputcodex-application = { path = "../../crates/inputcodex-application" }
inputcodex-infrastructure = { path = "../../crates/inputcodex-infrastructure" }
inputcodex-platform = { path = "../../crates/inputcodex-platform" }
inputcodex-presentation = { path = "../../crates/inputcodex-presentation" }
"@
    Write-Utf8File -Path (Join-Path $Path 'apps/inputcodex-desktop/Cargo.toml') -Content $desktopManifest
    $desktopSource = 'pub const UPDATE_SOURCE: &str = "https://github.com/nonononull/inputcodex/releases/latest";'
    Write-Utf8File -Path (Join-Path $Path 'apps/inputcodex-desktop/src/main.rs') -Content $desktopSource
}

function Copy-RepositoryFixture {
    param(
        [Parameter(Mandatory)]
        [string]$Source,

        [Parameter(Mandatory)]
        [string]$Name
    )

    $destination = Join-Path $testRoot $Name
    Copy-Item -LiteralPath $Source -Destination $destination -Recurse
    $destination
}

function Invoke-PolicyCase {
    param(
        [Parameter(Mandatory)]
        [string]$RepositoryRoot
    )

    Invoke-ChildScript -Path $policyScript -Arguments @('-RepositoryRoot', $RepositoryRoot)
}

function Assert-PolicyFailureCode {
    param(
        [Parameter(Mandatory)]
        $Result,

        [Parameter(Mandatory)]
        [string]$Code
    )

    Assert-True -Condition ($Result.ExitCode -ne 0) -Message "违规仓库必须返回非零退出码，输出=$($Result.Output)"
    Assert-True -Condition ($null -ne $Result.Json) -Message "仓库政策脚本必须输出 JSON，输出=$($Result.Output)"
    Assert-Contains -Collection @($Result.Json.violations.code) -Expected $Code -Message '仓库政策脚本必须返回稳定违规码'
}

$validRepository = Join-Path $testRoot 'repository-valid'
New-ValidRepositoryFixture -Path $validRepository

Invoke-ContractTest -Name '合法七成员 Workspace 通过仓库政策' -Body {
    $result = Invoke-PolicyCase -RepositoryRoot $validRepository
    Assert-Equal -Expected 0 -Actual $result.ExitCode -Message "合法仓库应通过，输出=$($result.Output)"
    Assert-True -Condition ($null -ne $result.Json) -Message "仓库政策脚本必须输出 JSON，输出=$($result.Output)"
    Assert-Equal -Expected $true -Actual $result.Json.ok -Message '合法仓库的 ok 必须为 true'
    Assert-Equal -Expected 0 -Actual $result.Json.violation_count -Message '合法仓库不应含违规项'
}

Invoke-ContractTest -Name '拒绝非 AGPL-3.0-only Workspace 许可证' -Body {
    $repository = Copy-RepositoryFixture -Source $validRepository -Name 'repository-license'
    $manifestPath = Join-Path $repository 'Cargo.toml'
    $manifest = Get-Content -LiteralPath $manifestPath -Raw -Encoding utf8
    $manifest = $manifest.Replace('license = "AGPL-3.0-only"', 'license = "MIT"')
    Set-Content -LiteralPath $manifestPath -Value $manifest -Encoding utf8NoBOM
    $result = Invoke-PolicyCase -RepositoryRoot $repository
    Assert-PolicyFailureCode -Result $result -Code 'WORKSPACE_LICENSE_INVALID'
}

Invoke-ContractTest -Name '拒绝 Iced 越过展示层' -Body {
    $repository = Copy-RepositoryFixture -Source $validRepository -Name 'repository-iced-layer'
    Add-Content -LiteralPath (Join-Path $repository 'crates/inputcodex-domain/Cargo.toml') -Value "`n[dependencies]`niced = `"0.14.0`"" -Encoding utf8NoBOM
    $result = Invoke-PolicyCase -RepositoryRoot $repository
    Assert-PolicyFailureCode -Result $result -Code 'ICED_LAYER_VIOLATION'
}

Invoke-ContractTest -Name '拒绝 upstream 加入 Workspace' -Body {
    $repository = Copy-RepositoryFixture -Source $validRepository -Name 'repository-upstream-member'
    $manifestPath = Join-Path $repository 'Cargo.toml'
    $manifest = Get-Content -LiteralPath $manifestPath -Raw
    $replacement = @(
        '    "crates/inputcodex-parity",'
        '    "upstream/CodexPlusPlus"'
    ) -join [Environment]::NewLine
    $manifest = $manifest.Replace('    "crates/inputcodex-parity"', $replacement)
    Set-Content -LiteralPath $manifestPath -Value $manifest -Encoding utf8NoBOM
    $result = Invoke-PolicyCase -RepositoryRoot $repository
    Assert-PolicyFailureCode -Result $result -Code 'UPSTREAM_WORKSPACE_MEMBER'
}

Invoke-ContractTest -Name '拒绝生产目录 TypeScript 文件' -Body {
    $repository = Copy-RepositoryFixture -Source $validRepository -Name 'repository-typescript'
    Write-Utf8File -Path (Join-Path $repository 'apps/inputcodex-desktop/src/main.ts') -Content 'export const forbidden = true;'
    $result = Invoke-PolicyCase -RepositoryRoot $repository
    Assert-PolicyFailureCode -Result $result -Code 'SCRIPT_LANGUAGE_FORBIDDEN'
}

Invoke-ContractTest -Name '拒绝 Tauri 运行时依赖' -Body {
    $repository = Copy-RepositoryFixture -Source $validRepository -Name 'repository-tauri'
    Add-Content -LiteralPath (Join-Path $repository 'apps/inputcodex-desktop/Cargo.toml') -Value 'tauri = "2.0.0"' -Encoding utf8NoBOM
    $result = Invoke-PolicyCase -RepositoryRoot $repository
    Assert-PolicyFailureCode -Result $result -Code 'WEB_RUNTIME_DEPENDENCY_FORBIDDEN'
}

Invoke-ContractTest -Name '拒绝 TOML 表形式的 Tauri 别名依赖' -Body {
    $repository = Copy-RepositoryFixture -Source $validRepository -Name 'repository-tauri-table'
    $tableDependency = "`n[dependencies.desktop-runtime]`npackage = `"tauri`"`nversion = `"2.0.0`""
    Add-Content -LiteralPath (Join-Path $repository 'apps/inputcodex-desktop/Cargo.toml') -Value $tableDependency -Encoding utf8NoBOM
    $result = Invoke-PolicyCase -RepositoryRoot $repository
    Assert-PolicyFailureCode -Result $result -Code 'WEB_RUNTIME_DEPENDENCY_FORBIDDEN'
}

Invoke-ContractTest -Name '拒绝 WebView 运行时依赖' -Body {
    $repository = Copy-RepositoryFixture -Source $validRepository -Name 'repository-webview'
    Add-Content -LiteralPath (Join-Path $repository 'apps/inputcodex-desktop/Cargo.toml') -Value 'wry = "0.53.0"' -Encoding utf8NoBOM
    $result = Invoke-PolicyCase -RepositoryRoot $repository
    Assert-PolicyFailureCode -Result $result -Code 'WEB_RUNTIME_DEPENDENCY_FORBIDDEN'
}

Invoke-ContractTest -Name '拒绝广告依赖' -Body {
    $repository = Copy-RepositoryFixture -Source $validRepository -Name 'repository-advertising'
    Add-Content -LiteralPath (Join-Path $repository 'apps/inputcodex-desktop/Cargo.toml') -Value 'admob = "0.3.0"' -Encoding utf8NoBOM
    $result = Invoke-PolicyCase -RepositoryRoot $repository
    Assert-PolicyFailureCode -Result $result -Code 'AD_TELEMETRY_DEPENDENCY_FORBIDDEN'
}

Invoke-ContractTest -Name '拒绝远程遥测依赖' -Body {
    $repository = Copy-RepositoryFixture -Source $validRepository -Name 'repository-telemetry'
    Add-Content -LiteralPath (Join-Path $repository 'apps/inputcodex-desktop/Cargo.toml') -Value 'sentry = "0.40.0"' -Encoding utf8NoBOM
    $result = Invoke-PolicyCase -RepositoryRoot $repository
    Assert-PolicyFailureCode -Result $result -Code 'AD_TELEMETRY_DEPENDENCY_FORBIDDEN'
}

Invoke-ContractTest -Name '拒绝非本仓 Release 或更新源' -Body {
    $repository = Copy-RepositoryFixture -Source $validRepository -Name 'repository-update-source'
    $sourcePath = Join-Path $repository 'apps/inputcodex-desktop/src/main.rs'
    $source = (Get-Content -LiteralPath $sourcePath -Raw).Replace('nonononull/inputcodex', 'BigPizzaV3/CodexPlusPlus')
    Set-Content -LiteralPath $sourcePath -Value $source -Encoding utf8NoBOM
    $result = Invoke-PolicyCase -RepositoryRoot $repository
    Assert-PolicyFailureCode -Result $result -Code 'UPDATE_SOURCE_FORBIDDEN'
}

Invoke-ContractTest -Name '拒绝 Workspace 依赖方向反转' -Body {
    $repository = Copy-RepositoryFixture -Source $validRepository -Name 'repository-dependency-direction'
    Add-Content -LiteralPath (Join-Path $repository 'crates/inputcodex-domain/Cargo.toml') -Value "`n[dependencies]`ninputcodex-presentation = { path = `"../inputcodex-presentation`" }" -Encoding utf8NoBOM
    $result = Invoke-PolicyCase -RepositoryRoot $repository
    Assert-PolicyFailureCode -Result $result -Code 'DEPENDENCY_DIRECTION_INVALID'
}

$forbiddenDirectDependencyCases = @(
    [pscustomobject]@{
        name = 'infrastructure-domain'
        manifest = 'crates/inputcodex-infrastructure/Cargo.toml'
        dependency = 'inputcodex-domain = { path = "../inputcodex-domain" }'
    }
    [pscustomobject]@{
        name = 'platform-domain'
        manifest = 'crates/inputcodex-platform/Cargo.toml'
        dependency = 'inputcodex-domain = { path = "../inputcodex-domain" }'
    }
    [pscustomobject]@{
        name = 'presentation-domain'
        manifest = 'crates/inputcodex-presentation/Cargo.toml'
        dependency = 'inputcodex-domain = { path = "../inputcodex-domain" }'
    }
    [pscustomobject]@{
        name = 'parity-platform'
        manifest = 'crates/inputcodex-parity/Cargo.toml'
        dependency = 'inputcodex-platform = { path = "../inputcodex-platform" }'
    }
)

foreach ($forbiddenDirectDependencyCase in $forbiddenDirectDependencyCases) {
    Invoke-ContractTest -Name "拒绝越过批准箭头 $($forbiddenDirectDependencyCase.name)" -Body {
        $repository = Copy-RepositoryFixture -Source $validRepository -Name "repository-$($forbiddenDirectDependencyCase.name)"
        Add-Content -LiteralPath (Join-Path $repository $forbiddenDirectDependencyCase.manifest) -Value $forbiddenDirectDependencyCase.dependency -Encoding utf8NoBOM
        $result = Invoke-PolicyCase -RepositoryRoot $repository
        Assert-PolicyFailureCode -Result $result -Code 'DEPENDENCY_DIRECTION_INVALID'
    }
}

Invoke-ContractTest -Name '拒绝 TOML 表形式的依赖方向反转' -Body {
    $repository = Copy-RepositoryFixture -Source $validRepository -Name 'repository-dependency-table-direction'
    $tableDependency = "`n[dependencies.ui-layer]`npackage = `"inputcodex-presentation`"`npath = `"../inputcodex-presentation`""
    Add-Content -LiteralPath (Join-Path $repository 'crates/inputcodex-domain/Cargo.toml') -Value $tableDependency -Encoding utf8NoBOM
    $result = Invoke-PolicyCase -RepositoryRoot $repository
    Assert-PolicyFailureCode -Result $result -Code 'DEPENDENCY_DIRECTION_INVALID'
}

function Invoke-TestGit {
    param(
        [Parameter(Mandatory)]
        [string]$RepositoryRoot,

        [Parameter(Mandatory)]
        [string[]]$Arguments
    )

    $output = @(& git -C $RepositoryRoot @Arguments 2>&1 | ForEach-Object { $_.ToString() })
    if ($LASTEXITCODE -ne 0) {
        throw "测试 Git 命令失败：git -C $RepositoryRoot $($Arguments -join ' ')；输出=$($output -join [Environment]::NewLine)"
    }

    ,$output
}

Invoke-ContractTest -Name 'Git 变更收集器保留新增修改删除和重命名' -Body {
    Assert-True -Condition (Test-Path -LiteralPath $collectorScript -PathType Leaf) -Message 'CI_COLLECTOR_RED_MISSING_IMPLEMENTATION'

    $repository = Join-Path $testRoot 'collector-repository'
    New-Item -ItemType Directory -Path $repository -Force | Out-Null
    Invoke-TestGit -RepositoryRoot $repository -Arguments @('init', '--quiet') | Out-Null
    Invoke-TestGit -RepositoryRoot $repository -Arguments @('config', 'user.name', 'inputcodex-ci-test') | Out-Null
    Invoke-TestGit -RepositoryRoot $repository -Arguments @('config', 'user.email', 'ci-test@inputcodex.invalid') | Out-Null

    Write-Utf8File -Path (Join-Path $repository 'README.md') -Content 'initial'
    Write-Utf8File -Path (Join-Path $repository 'Cargo.lock') -Content 'lock'
    Write-Utf8File -Path (Join-Path $repository 'docs/old.md') -Content 'rename-me'
    Invoke-TestGit -RepositoryRoot $repository -Arguments @('add', '--all') | Out-Null
    Invoke-TestGit -RepositoryRoot $repository -Arguments @('commit', '--quiet', '-m', 'initial') | Out-Null
    $base = (Invoke-TestGit -RepositoryRoot $repository -Arguments @('rev-parse', 'HEAD'))[0].Trim()

    Write-Utf8File -Path (Join-Path $repository 'README.md') -Content 'changed'
    Remove-Item -LiteralPath (Join-Path $repository 'Cargo.lock') -Force
    Invoke-TestGit -RepositoryRoot $repository -Arguments @('mv', 'docs/old.md', 'docs/new.md') | Out-Null
    Write-Utf8File -Path (Join-Path $repository 'crates/inputcodex-domain/src/lib.rs') -Content '#![forbid(unsafe_code)]'
    Invoke-TestGit -RepositoryRoot $repository -Arguments @('add', '--all') | Out-Null
    Invoke-TestGit -RepositoryRoot $repository -Arguments @('commit', '--quiet', '-m', 'changes') | Out-Null
    $head = (Invoke-TestGit -RepositoryRoot $repository -Arguments @('rev-parse', 'HEAD'))[0].Trim()

    $outputFile = Join-Path $testRoot 'collector-output.json'
    $result = Invoke-ChildScript -Path $collectorScript -Arguments @(
        '-RepositoryRoot', $repository,
        '-Base', $base,
        '-Head', $head,
        '-OutputFile', $outputFile
    )
    Assert-Equal -Expected 0 -Actual $result.ExitCode -Message "变更收集器应成功，输出=$($result.Output)"
    Assert-True -Condition (Test-Path -LiteralPath $outputFile -PathType Leaf) -Message '变更收集器必须写出 JSON 文件'

    $changes = @(Get-Content -LiteralPath $outputFile -Raw -Encoding utf8 | ConvertFrom-Json -Depth 20)
    Assert-Equal -Expected 4 -Actual $changes.Count -Message '变更收集器记录数必须稳定'
    Assert-Equal -Expected 1 -Actual @($changes | Where-Object { $_.status -eq 'M' -and $_.path -eq 'README.md' }).Count -Message '必须保留修改记录'
    Assert-Equal -Expected 1 -Actual @($changes | Where-Object { $_.status -eq 'D' -and $_.path -eq 'Cargo.lock' }).Count -Message '必须保留删除记录'
    Assert-Equal -Expected 1 -Actual @($changes | Where-Object { $_.status -eq 'A' -and $_.path -eq 'crates/inputcodex-domain/src/lib.rs' }).Count -Message '必须保留新增记录'
    Assert-Equal -Expected 1 -Actual @($changes | Where-Object { $_.status -eq 'R' -and $_.old_path -eq 'docs/old.md' -and $_.path -eq 'docs/new.md' }).Count -Message '必须保留重命名新旧路径'
}

$resolvedTestRoot = [System.IO.Path]::GetFullPath($testRoot)
$resolvedTemporaryRoot = [System.IO.Path]::GetFullPath([System.IO.Path]::GetTempPath())
$isSafeTestRoot = $resolvedTestRoot.StartsWith($resolvedTemporaryRoot, [System.StringComparison]::OrdinalIgnoreCase) -and
    ([System.IO.Path]::GetFileName($resolvedTestRoot) -like 'inputcodex-ci-contract-*')

if ($isSafeTestRoot -and (Test-Path -LiteralPath $resolvedTestRoot)) {
    Remove-Item -LiteralPath $resolvedTestRoot -Recurse -Force
}

if ($script:Failures.Count -gt 0) {
    [Console]::Error.WriteLine('CI_CONTRACT_TEST_FAILURE')
    foreach ($failure in $script:Failures) {
        [Console]::Error.WriteLine($failure)
    }

    exit 1
}

Write-Host "CI_CONTRACT_GREEN passed=$script:PassedCount"
exit 0
