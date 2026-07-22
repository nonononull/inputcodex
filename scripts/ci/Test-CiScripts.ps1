[CmdletBinding()]
param()

Set-StrictMode -Version Latest
$ErrorActionPreference = 'Stop'

$scriptDirectory = Split-Path -Parent $PSCommandPath
$classifierScript = Join-Path $scriptDirectory 'Classify-Changes.ps1'
$policyScript = Join-Path $scriptDirectory 'Verify-RepositoryPolicy.ps1'
$missingImplementations = @(
    $classifierScript
    $policyScript
) | Where-Object { -not (Test-Path -LiteralPath $_ -PathType Leaf) }

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

function Invoke-ClassifierCase {
    param(
        [Parameter(Mandatory)]
        [string]$Name,

        [Parameter(Mandatory)]
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
license = "MIT"
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
inputcodex-domain = { path = "../inputcodex-domain" }
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
inputcodex-domain = { path = "../inputcodex-domain" }
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
inputcodex-domain = { path = "../inputcodex-domain" }
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
inputcodex-platform = { path = "../inputcodex-platform" }
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
