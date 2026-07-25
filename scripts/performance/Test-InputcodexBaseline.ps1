[CmdletBinding()]
param(
    [Parameter(Mandatory)]
    [string]$RepositoryRoot,

    [Parameter(Mandatory)]
    [ValidateSet('Contract', 'Evidence')]
    [string]$Mode
)

Set-StrictMode -Version Latest
$ErrorActionPreference = 'Stop'

$script:Violations = [System.Collections.Generic.List[object]]::new()
$script:ConfigHash = $null
$script:ImplementationHash = $null
$script:InputHash = $null

function Add-Violation {
    param(
        [Parameter(Mandatory)]
        [string]$Code,
        [Parameter(Mandatory)]
        [string]$Path,
        [Parameter(Mandatory)]
        [string]$Message
    )

    $script:Violations.Add([pscustomobject][ordered]@{
        code = $Code
        path = $Path
        message = $Message
    })
}

function Assert-Condition {
    param(
        [Parameter(Mandatory)]
        [bool]$Condition,
        [Parameter(Mandatory)]
        [string]$Code,
        [Parameter(Mandatory)]
        [string]$Path,
        [Parameter(Mandatory)]
        [string]$Message
    )

    if (-not $Condition) {
        Add-Violation -Code $Code -Path $Path -Message $Message
    }
}

function Get-Sha256Text {
    param([Parameter(Mandatory)][string]$Text)

    $bytes = [System.Text.UTF8Encoding]::new($false).GetBytes($Text)
    $hash = [System.Security.Cryptography.SHA256]::HashData($bytes)
    'sha256:' + [Convert]::ToHexString($hash).ToLowerInvariant()
}

function Get-NormalizedTextHash {
    param([Parameter(Mandatory)][string]$Path)

    $text = [System.IO.File]::ReadAllText($Path, [System.Text.Encoding]::UTF8)
    Get-Sha256Text -Text ([regex]::Replace($text, '\r\n?', "`n"))
}

function Get-RawFileHash {
    param([Parameter(Mandatory)][string]$Path)

    'sha256:' + (Get-FileHash -LiteralPath $Path -Algorithm SHA256).Hash.ToLowerInvariant()
}

function Get-CombinedTextHash {
    param(
        [Parameter(Mandatory)][string]$Root,
        [Parameter(Mandatory)][string[]]$RelativePaths
    )

    $entries = foreach ($relativePath in @($RelativePaths | Sort-Object -Unique)) {
        $fullPath = Join-Path $Root $relativePath
        if (-not (Test-Path -LiteralPath $fullPath -PathType Leaf)) {
            throw "哈希输入文件不存在：$relativePath"
        }
        $hash = Get-NormalizedTextHash -Path $fullPath
        '{0}={1}' -f $relativePath.Replace('\', '/'), $hash.Substring(7)
    }

    Get-Sha256Text -Text (($entries -join "`n") + "`n")
}

function Get-RelativeFilePaths {
    param(
        [Parameter(Mandatory)][string]$Root,
        [Parameter(Mandatory)][string[]]$Directories,
        [Parameter(Mandatory)][scriptblock]$Filter
    )

    $paths = [System.Collections.Generic.List[string]]::new()
    foreach ($directory in $Directories) {
        $fullDirectory = Join-Path $Root $directory
        if (-not (Test-Path -LiteralPath $fullDirectory -PathType Container)) {
            continue
        }
        foreach ($file in Get-ChildItem -LiteralPath $fullDirectory -Recurse -File) {
            if (& $Filter $file) {
                $paths.Add([System.IO.Path]::GetRelativePath($Root, $file.FullName).Replace('\', '/'))
            }
        }
    }
    @($paths | Sort-Object -Unique)
}

function Read-JsonFile {
    param([Parameter(Mandatory)][string]$Path)

    Get-Content -LiteralPath $Path -Raw -Encoding utf8 | ConvertFrom-Json -Depth 100
}

function Assert-Hash {
    param(
        [AllowNull()]$Actual,
        [Parameter(Mandatory)][string]$Expected,
        [Parameter(Mandatory)][string]$Path,
        [Parameter(Mandatory)][string]$Field
    )

    Assert-Condition `
        -Condition ([string]$Actual -eq $Expected) `
        -Code 'HASH_MISMATCH' `
        -Path $Path `
        -Message "$Field 必须等于当前合同哈希 $Expected，实际为 $Actual。"
}

function Test-PlatformResult {
    param(
        [Parameter(Mandatory)][string]$Path,
        [Parameter(Mandatory)][string]$ExpectedPlatform,
        [Parameter(Mandatory)]$Config
    )

    try {
        $result = Read-JsonFile -Path $Path
        $relativePath = [System.IO.Path]::GetRelativePath($resolvedRoot, $Path).Replace('\', '/')

        Assert-Condition -Condition ($result.schema_version -eq 'inputcodex.performance-result.v1') -Code 'RESULT_SCHEMA_INVALID' -Path $relativePath -Message '结果 schema_version 无效。'
        Assert-Condition -Condition ([int]$result.issue_number -eq 32) -Code 'RESULT_ISSUE_INVALID' -Path $relativePath -Message '结果必须归属 Issue #32。'
        Assert-Condition -Condition ($result.platform -eq $ExpectedPlatform) -Code 'RESULT_PLATFORM_INVALID' -Path $relativePath -Message "结果平台必须为 $ExpectedPlatform。"
        Assert-Condition -Condition ($result.status -eq 'complete') -Code 'RESULT_STATUS_INVALID' -Path $relativePath -Message '结果状态必须为 complete。'
        Assert-Condition -Condition ([string]$result.source.commit -match '^[0-9a-f]{40}$') -Code 'RESULT_COMMIT_INVALID' -Path $relativePath -Message '测量 commit 必须为 40 位 SHA。'
        Assert-Condition -Condition ([string]$result.source.tree -match '^[0-9a-f]{40}$') -Code 'RESULT_TREE_INVALID' -Path $relativePath -Message '测量 tree 必须为 40 位 SHA。'
        Assert-Hash -Actual $result.source.config_sha256 -Expected $script:ConfigHash -Path $relativePath -Field 'config_sha256'
        Assert-Hash -Actual $result.source.implementation_sha256 -Expected $script:ImplementationHash -Path $relativePath -Field 'implementation_sha256'
        Assert-Hash -Actual $result.source.input_sha256 -Expected $script:InputHash -Path $relativePath -Field 'input_sha256'
        Assert-Condition -Condition ($result.environment.runner_environment -eq 'github-hosted') -Code 'RUNNER_ENVIRONMENT_INVALID' -Path $relativePath -Message '真实结果只能来自 github-hosted runner。'
        Assert-Condition -Condition ([string]$result.github.run_id -match '^\d+$') -Code 'RUN_ID_INVALID' -Path $relativePath -Message '结果缺少 GitHub run_id。'
        Assert-Condition -Condition ([int]$result.github.run_attempt -ge 1) -Code 'RUN_ATTEMPT_INVALID' -Path $relativePath -Message '结果缺少有效 run_attempt。'
        Assert-Condition -Condition ([long]$result.build.desktop_release.binary_size_bytes -gt 0) -Code 'BINARY_SIZE_INVALID' -Path $relativePath -Message 'Release 二进制大小必须大于零。'
        Assert-Condition -Condition ([double]$result.build.desktop_release.duration_milliseconds -gt 0) -Code 'BUILD_DURATION_INVALID' -Path $relativePath -Message '桌面 Release 构建耗时必须大于零。'

        $attempts = @($result.presentation_first_view.attempts)
        $successfulAttempts = @($attempts | Where-Object { $_.status -eq 'success' })
        Assert-Condition -Condition ($attempts.Count -le [int]$Config.probe.maximum_attempts) -Code 'FIRST_VIEW_ATTEMPT_OVERFLOW' -Path $relativePath -Message '首次 view 尝试数超过配置上限。'
        Assert-Condition -Condition ($successfulAttempts.Count -ge [int]$Config.probe.minimum_successful_samples) -Code 'FIRST_VIEW_SAMPLE_SHORTAGE' -Path $relativePath -Message '首次 view 成功样本不足。'
        Assert-Condition -Condition (@($successfulAttempts | Where-Object { [long]$_.elapsed_nanoseconds -le 0 }).Count -eq 0) -Code 'FIRST_VIEW_SAMPLE_INVALID' -Path $relativePath -Message '首次 view 成功样本必须包含正耗时。'

        $idleSamples = @($result.desktop_idle_resources.samples)
        Assert-Condition -Condition ($idleSamples.Count -eq [int]$Config.idle_resources.sample_count) -Code 'IDLE_SAMPLE_COUNT_INVALID' -Path $relativePath -Message '空闲资源样本数不符合配置。'
        Assert-Condition -Condition (@($idleSamples | Where-Object { [long]$_.working_set_bytes -le 0 }).Count -eq 0) -Code 'IDLE_WORKING_SET_INVALID' -Path $relativePath -Message '空闲 Working Set 样本必须大于零。'
        Assert-Condition -Condition (@($idleSamples | Where-Object { [double]$_.cpu_percent -lt 0 }).Count -eq 0) -Code 'IDLE_CPU_INVALID' -Path $relativePath -Message '空闲 CPU 样本不得为负数。'

        $scenarioResults = @($result.rust_scenarios)
        Assert-Condition -Condition ($scenarioResults.Count -eq 3) -Code 'SCENARIO_COUNT_INVALID' -Path $relativePath -Message 'Rust 场景结果必须恰好为三个。'
        foreach ($scenarioConfig in @($Config.rust_scenarios)) {
            $matches = @($scenarioResults | Where-Object { $_.name -eq $scenarioConfig.name })
            Assert-Condition -Condition ($matches.Count -eq 1) -Code 'SCENARIO_MISSING' -Path $relativePath -Message "缺少唯一场景结果 $($scenarioConfig.name)。"
            if ($matches.Count -ne 1) { continue }
            $scenario = $matches[0]
            Assert-Condition -Condition (@($scenario.warmups).Count -eq [int]$scenarioConfig.warmup_runs) -Code 'SCENARIO_WARMUP_COUNT_INVALID' -Path $relativePath -Message "$($scenarioConfig.name) 预热数不符合配置。"
            Assert-Condition -Condition (@($scenario.samples).Count -eq [int]$scenarioConfig.sample_count) -Code 'SCENARIO_SAMPLE_COUNT_INVALID' -Path $relativePath -Message "$($scenarioConfig.name) 样本数不符合配置。"
            Assert-Condition -Condition (@($scenario.samples | Where-Object { [uint64]$_.iterations -ne [uint64]$scenarioConfig.iterations }).Count -eq 0) -Code 'SCENARIO_ITERATIONS_INVALID' -Path $relativePath -Message "$($scenarioConfig.name) 迭代数不符合配置。"
            Assert-Condition -Condition (@($scenario.samples | Where-Object { [uint64]$_.checksum -eq 0 }).Count -eq 0) -Code 'SCENARIO_CHECKSUM_INVALID' -Path $relativePath -Message "$($scenarioConfig.name) checksum 不得为零。"
        }

        $result
    }
    catch {
        Add-Violation -Code 'RESULT_PARSE_FAILED' -Path $Path -Message $_.Exception.Message
        $null
    }
}

try {
    if (-not (Test-Path -LiteralPath $RepositoryRoot -PathType Container)) {
        Add-Violation -Code 'REPOSITORY_ROOT_NOT_FOUND' -Path $RepositoryRoot -Message '仓库根目录不存在。'
        throw '仓库根目录不存在。'
    }

    $resolvedRoot = (Resolve-Path -LiteralPath $RepositoryRoot).Path
    $requiredPaths = @(
        '.github/workflows/performance-baseline.yml',
        'benchmarks/config/issue-32-baseline.json',
        'benchmarks/inputcodex-baseline/Cargo.lock',
        'benchmarks/inputcodex-baseline/Cargo.toml',
        'benchmarks/inputcodex-baseline/build.md',
        'benchmarks/inputcodex-baseline/err.md',
        'benchmarks/inputcodex-baseline/src/lib.rs',
        'benchmarks/inputcodex-baseline/src/main.rs',
        'benchmarks/inputcodex-baseline/tests/baseline_contract.rs',
        'benchmarks/README.md',
        'crates/inputcodex-presentation/src/lib.rs',
        'scripts/ci/Test-CiScripts.ps1',
        'scripts/performance/Invoke-InputcodexBaseline.ps1',
        'scripts/performance/Test-InputcodexBaseline.ps1'
    )
    foreach ($relativePath in $requiredPaths) {
        Assert-Condition -Condition (Test-Path -LiteralPath (Join-Path $resolvedRoot $relativePath) -PathType Leaf) -Code 'REQUIRED_FILE_MISSING' -Path $relativePath -Message '性能基线合同文件不存在。'
    }

    $configPath = Join-Path $resolvedRoot 'benchmarks/config/issue-32-baseline.json'
    $config = Read-JsonFile -Path $configPath
    Assert-Condition -Condition ($config.schema_version -eq 'inputcodex.performance-baseline.v1') -Code 'CONFIG_SCHEMA_INVALID' -Path 'benchmarks/config/issue-32-baseline.json' -Message '配置 schema_version 无效。'
    Assert-Condition -Condition ([int]$config.issue_number -eq 32) -Code 'CONFIG_ISSUE_INVALID' -Path 'benchmarks/config/issue-32-baseline.json' -Message '配置必须归属 Issue #32。'
    Assert-Condition -Condition ($config.rust_toolchain -eq '1.97.1') -Code 'CONFIG_TOOLCHAIN_INVALID' -Path 'benchmarks/config/issue-32-baseline.json' -Message 'Rust 工具链必须固定为 1.97.1。'
    Assert-Condition -Condition ($config.probe.environment_variable -eq 'INPUTCODEX_PERFORMANCE_PROBE') -Code 'CONFIG_PROBE_ENV_INVALID' -Path 'benchmarks/config/issue-32-baseline.json' -Message '探针环境变量无效。'
    Assert-Condition -Condition ($config.probe.enabled_value -eq '1') -Code 'CONFIG_PROBE_VALUE_INVALID' -Path 'benchmarks/config/issue-32-baseline.json' -Message '探针只能由值 1 开启。'
    Assert-Condition -Condition ($config.probe.ready_marker -eq 'INPUTCODEX_PERFORMANCE_READY_V1') -Code 'CONFIG_PROBE_MARKER_INVALID' -Path 'benchmarks/config/issue-32-baseline.json' -Message '探针标记无效。'
    Assert-Condition -Condition ([int]$config.probe.minimum_successful_samples -eq 5) -Code 'CONFIG_FIRST_VIEW_MIN_INVALID' -Path 'benchmarks/config/issue-32-baseline.json' -Message '首次 view 最少成功样本必须为 5。'
    Assert-Condition -Condition ([int]$config.probe.maximum_attempts -eq 7) -Code 'CONFIG_FIRST_VIEW_ATTEMPTS_INVALID' -Path 'benchmarks/config/issue-32-baseline.json' -Message '首次 view 最大尝试数必须为 7。'
    Assert-Condition -Condition ([int]$config.probe.timeout_seconds -eq 15) -Code 'CONFIG_FIRST_VIEW_TIMEOUT_INVALID' -Path 'benchmarks/config/issue-32-baseline.json' -Message '首次 view 单次超时必须为 15 秒。'
    Assert-Condition -Condition ([int]$config.idle_resources.settle_seconds -eq 30) -Code 'CONFIG_IDLE_SETTLE_INVALID' -Path 'benchmarks/config/issue-32-baseline.json' -Message '空闲稳定等待必须为 30 秒。'
    Assert-Condition -Condition ([int]$config.idle_resources.sample_interval_seconds -eq 1) -Code 'CONFIG_IDLE_INTERVAL_INVALID' -Path 'benchmarks/config/issue-32-baseline.json' -Message '空闲采样间隔必须为 1 秒。'
    Assert-Condition -Condition ([int]$config.idle_resources.sample_count -eq 60) -Code 'CONFIG_IDLE_COUNT_INVALID' -Path 'benchmarks/config/issue-32-baseline.json' -Message '空闲采样数必须为 60。'
    Assert-Condition -Condition ([int]$config.artifacts.success_retention_days -eq 1) -Code 'CONFIG_SUCCESS_RETENTION_INVALID' -Path 'benchmarks/config/issue-32-baseline.json' -Message '成功 Artifact 必须保留 1 天。'
    Assert-Condition -Condition ([int]$config.artifacts.failure_retention_days -eq 7) -Code 'CONFIG_FAILURE_RETENTION_INVALID' -Path 'benchmarks/config/issue-32-baseline.json' -Message '失败 Artifact 必须保留 7 天。'

    $scenarioConfigs = @($config.rust_scenarios)
    $expectedScenarioNames = @('application-load-complete', 'application-cancel-stale', 'parity-repository-validation')
    Assert-Condition -Condition ($scenarioConfigs.Count -eq 3) -Code 'CONFIG_SCENARIO_COUNT_INVALID' -Path 'benchmarks/config/issue-32-baseline.json' -Message 'Rust 场景必须恰好为三个。'
    foreach ($expectedScenarioName in $expectedScenarioNames) {
        $matches = @($scenarioConfigs | Where-Object { $_.name -eq $expectedScenarioName })
        Assert-Condition -Condition ($matches.Count -eq 1) -Code 'CONFIG_SCENARIO_INVALID' -Path 'benchmarks/config/issue-32-baseline.json' -Message "场景 $expectedScenarioName 必须唯一存在。"
        if ($matches.Count -eq 1) {
            Assert-Condition -Condition ([int]$matches[0].warmup_runs -eq 3) -Code 'CONFIG_WARMUP_INVALID' -Path 'benchmarks/config/issue-32-baseline.json' -Message "$expectedScenarioName 必须预热 3 次。"
            Assert-Condition -Condition ([int]$matches[0].sample_count -eq 20) -Code 'CONFIG_SAMPLE_COUNT_INVALID' -Path 'benchmarks/config/issue-32-baseline.json' -Message "$expectedScenarioName 必须保留 20 个样本。"
            Assert-Condition -Condition ([uint64]$matches[0].iterations -gt 0) -Code 'CONFIG_ITERATIONS_INVALID' -Path 'benchmarks/config/issue-32-baseline.json' -Message "$expectedScenarioName 迭代数必须大于零。"
        }
    }

    $presentationText = Get-Content -LiteralPath (Join-Path $resolvedRoot 'crates/inputcodex-presentation/src/lib.rs') -Raw -Encoding utf8
    Assert-Condition -Condition ($presentationText.Contains('INPUTCODEX_PERFORMANCE_PROBE')) -Code 'PROBE_ENV_MISSING' -Path 'crates/inputcodex-presentation/src/lib.rs' -Message '展示层缺少性能探针环境变量。'
    Assert-Condition -Condition ($presentationText.Contains('INPUTCODEX_PERFORMANCE_READY_V1')) -Code 'PROBE_MARKER_MISSING' -Path 'crates/inputcodex-presentation/src/lib.rs' -Message '展示层缺少稳定 ready 标记。'

    $baselineManifestText = Get-Content -LiteralPath (Join-Path $resolvedRoot 'benchmarks/inputcodex-baseline/Cargo.toml') -Raw -Encoding utf8
    Assert-Condition -Condition ($baselineManifestText -match '(?m)^\[workspace\]\r?$') -Code 'ISOLATED_WORKSPACE_MISSING' -Path 'benchmarks/inputcodex-baseline/Cargo.toml' -Message '性能测量工程必须声明独立 Workspace。'
    $rootManifestText = Get-Content -LiteralPath (Join-Path $resolvedRoot 'Cargo.toml') -Raw -Encoding utf8
    Assert-Condition -Condition (-not $rootManifestText.Contains('benchmarks/inputcodex-baseline')) -Code 'ROOT_WORKSPACE_POLLUTED' -Path 'Cargo.toml' -Message '根 Workspace 禁止加入性能测量工程。'

    $implementationPaths = [System.Collections.Generic.List[string]]::new()
    foreach ($relativePath in @(
        '.github/workflows/performance-baseline.yml', 'Cargo.lock', 'Cargo.toml', 'rust-toolchain.toml',
        'benchmarks/inputcodex-baseline/Cargo.lock', 'benchmarks/inputcodex-baseline/Cargo.toml',
        'benchmarks/inputcodex-baseline/src/lib.rs', 'benchmarks/inputcodex-baseline/src/main.rs',
        'benchmarks/inputcodex-baseline/tests/baseline_contract.rs', 'scripts/ci/Test-CiScripts.ps1',
        'scripts/performance/Invoke-InputcodexBaseline.ps1', 'scripts/performance/Test-InputcodexBaseline.ps1'
    )) { $implementationPaths.Add($relativePath) }

    $codePaths = Get-RelativeFilePaths `
        -Root $resolvedRoot `
        -Directories @('apps/inputcodex-desktop', 'crates/inputcodex-domain', 'crates/inputcodex-application', 'crates/inputcodex-infrastructure', 'crates/inputcodex-platform', 'crates/inputcodex-presentation', 'crates/inputcodex-parity') `
        -Filter { param($file) $file.Extension -eq '.rs' -or $file.Name -eq 'Cargo.toml' }
    foreach ($codePath in $codePaths) { $implementationPaths.Add($codePath) }

    $inputPaths = [System.Collections.Generic.List[string]]::new()
    foreach ($parityPath in Get-RelativeFilePaths -Root $resolvedRoot -Directories @('parity') -Filter { param($file) $true }) {
        $inputPaths.Add($parityPath)
    }
    $inputPaths.Add('upstream/source-lock.json')

    $script:ConfigHash = Get-NormalizedTextHash -Path $configPath
    $script:ImplementationHash = Get-CombinedTextHash -Root $resolvedRoot -RelativePaths @($implementationPaths)
    $script:InputHash = Get-CombinedTextHash -Root $resolvedRoot -RelativePaths @($inputPaths)

    if ($Mode -eq 'Evidence') {
        $windowsPath = Join-Path $resolvedRoot 'benchmarks/results/issue-32/windows.json'
        $macosPath = Join-Path $resolvedRoot 'benchmarks/results/issue-32/macos.json'
        $manifestPath = Join-Path $resolvedRoot 'benchmarks/results/issue-32/manifest.json'
        foreach ($evidencePath in @($windowsPath, $macosPath, $manifestPath)) {
            $relativePath = [System.IO.Path]::GetRelativePath($resolvedRoot, $evidencePath).Replace('\', '/')
            Assert-Condition -Condition (Test-Path -LiteralPath $evidencePath -PathType Leaf) -Code 'EVIDENCE_FILE_MISSING' -Path $relativePath -Message '性能证据文件不存在。'
        }

        if ((Test-Path -LiteralPath $windowsPath -PathType Leaf) -and
            (Test-Path -LiteralPath $macosPath -PathType Leaf) -and
            (Test-Path -LiteralPath $manifestPath -PathType Leaf)) {
            $windows = Test-PlatformResult -Path $windowsPath -ExpectedPlatform 'windows' -Config $config
            $macos = Test-PlatformResult -Path $macosPath -ExpectedPlatform 'macos' -Config $config
            try {
                $manifest = Read-JsonFile -Path $manifestPath
                Assert-Condition -Condition ($manifest.schema_version -eq 'inputcodex.performance-manifest.v1') -Code 'MANIFEST_SCHEMA_INVALID' -Path 'benchmarks/results/issue-32/manifest.json' -Message '组合 manifest schema_version 无效。'
                Assert-Condition -Condition ([int]$manifest.issue_number -eq 32) -Code 'MANIFEST_ISSUE_INVALID' -Path 'benchmarks/results/issue-32/manifest.json' -Message '组合 manifest 必须归属 Issue #32。'
                Assert-Hash -Actual $manifest.config_sha256 -Expected $script:ConfigHash -Path 'benchmarks/results/issue-32/manifest.json' -Field 'config_sha256'
                Assert-Hash -Actual $manifest.implementation_sha256 -Expected $script:ImplementationHash -Path 'benchmarks/results/issue-32/manifest.json' -Field 'implementation_sha256'
                Assert-Hash -Actual $manifest.input_sha256 -Expected $script:InputHash -Path 'benchmarks/results/issue-32/manifest.json' -Field 'input_sha256'
                Assert-Condition -Condition ($manifest.results.windows.sha256 -eq (Get-RawFileHash -Path $windowsPath)) -Code 'WINDOWS_RESULT_HASH_INVALID' -Path 'benchmarks/results/issue-32/manifest.json' -Message 'Windows 结果文件哈希不匹配。'
                Assert-Condition -Condition ($manifest.results.macos.sha256 -eq (Get-RawFileHash -Path $macosPath)) -Code 'MACOS_RESULT_HASH_INVALID' -Path 'benchmarks/results/issue-32/manifest.json' -Message 'macOS 结果文件哈希不匹配。'
                Assert-Condition -Condition ([string]$manifest.github.run_id -match '^\d+$') -Code 'MANIFEST_RUN_ID_INVALID' -Path 'benchmarks/results/issue-32/manifest.json' -Message '组合 manifest 缺少 run_id。'
                Assert-Condition -Condition ([long]$manifest.artifacts.windows.id -gt 0) -Code 'WINDOWS_ARTIFACT_ID_INVALID' -Path 'benchmarks/results/issue-32/manifest.json' -Message '组合 manifest 缺少 Windows Artifact ID。'
                Assert-Condition -Condition ([long]$manifest.artifacts.macos.id -gt 0) -Code 'MACOS_ARTIFACT_ID_INVALID' -Path 'benchmarks/results/issue-32/manifest.json' -Message '组合 manifest 缺少 macOS Artifact ID。'

                if ($null -ne $windows -and $null -ne $macos) {
                    Assert-Condition -Condition ($windows.source.commit -eq $macos.source.commit) -Code 'RESULT_COMMIT_DIVERGED' -Path 'benchmarks/results/issue-32/manifest.json' -Message '两平台测量 commit 不一致。'
                    Assert-Condition -Condition ($windows.source.tree -eq $macos.source.tree) -Code 'RESULT_TREE_DIVERGED' -Path 'benchmarks/results/issue-32/manifest.json' -Message '两平台测量 tree 不一致。'
                    Assert-Condition -Condition ($manifest.measurement_commit -eq $windows.source.commit) -Code 'MANIFEST_COMMIT_INVALID' -Path 'benchmarks/results/issue-32/manifest.json' -Message '组合 manifest 的 measurement_commit 不匹配。'
                    Assert-Condition -Condition ($manifest.measurement_tree -eq $windows.source.tree) -Code 'MANIFEST_TREE_INVALID' -Path 'benchmarks/results/issue-32/manifest.json' -Message '组合 manifest 的 measurement_tree 不匹配。'
                    Assert-Condition -Condition ([string]$manifest.github.run_id -eq [string]$windows.github.run_id -and [string]$manifest.github.run_id -eq [string]$macos.github.run_id) -Code 'MANIFEST_RUN_DIVERGED' -Path 'benchmarks/results/issue-32/manifest.json' -Message '两平台与 manifest 的 run_id 必须一致。'
                }
            }
            catch {
                Add-Violation -Code 'MANIFEST_PARSE_FAILED' -Path 'benchmarks/results/issue-32/manifest.json' -Message $_.Exception.Message
            }
        }
    }
}
catch {
    if ($script:Violations.Count -eq 0) {
        Add-Violation -Code 'VALIDATION_EXCEPTION' -Path $RepositoryRoot -Message $_.Exception.Message
    }
}

$output = [pscustomobject][ordered]@{
    schema_version = 'inputcodex.performance-validation.v1'
    ok = $script:Violations.Count -eq 0
    mode = $Mode
    config_sha256 = $script:ConfigHash
    implementation_sha256 = $script:ImplementationHash
    input_sha256 = $script:InputHash
    violation_count = $script:Violations.Count
    violations = @($script:Violations)
}

[Console]::Out.WriteLine((ConvertTo-Json -InputObject $output -Depth 100 -Compress))
if ($script:Violations.Count -gt 0) { exit 1 }
exit 0
