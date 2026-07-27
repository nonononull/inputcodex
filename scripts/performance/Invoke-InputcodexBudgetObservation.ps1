[CmdletBinding()]
param(
    [Parameter(Mandatory)][string]$RepositoryRoot,
    [Parameter(Mandatory)][string]$BudgetPath,
    [Parameter(Mandatory)][string]$ResultPath,
    [Parameter(Mandatory)][ValidateSet('windows', 'macos')][string]$Platform,
    [string]$ExpectedBudgetSha256 = 'sha256:be07138908cd411925db963718b71062060f4fd4a50b910ab5d5f25f88d4ebe5'
)

Set-StrictMode -Version Latest
$ErrorActionPreference = 'Stop'

$script:Violations = [System.Collections.Generic.List[object]]::new()
$script:BudgetHash = $null
$script:ResultHash = $null

function Add-Violation {
    param(
        [Parameter(Mandatory)][string]$Code,
        [Parameter(Mandatory)][string]$Path,
        [Parameter(Mandatory)][string]$Message
    )

    $script:Violations.Add([pscustomobject][ordered]@{
        code = $Code
        path = $Path
        message = $Message
    })
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

function Resolve-InputPath {
    param(
        [Parameter(Mandatory)][string]$Root,
        [Parameter(Mandatory)][string]$Path
    )

    if ([System.IO.Path]::IsPathFullyQualified($Path)) {
        return [System.IO.Path]::GetFullPath($Path)
    }
    [System.IO.Path]::GetFullPath((Join-Path $Root $Path))
}

function Get-JsonProperty {
    param(
        [AllowNull()]$Object,
        [Parameter(Mandatory)][string]$Name
    )

    if ($null -eq $Object) { return $null }
    $property = $Object.PSObject.Properties[$Name]
    if ($null -eq $property) { return $null }
    $property.Value
}

function Read-JsonDocument {
    param(
        [Parameter(Mandatory)][string]$Path,
        [Parameter(Mandatory)][string]$MissingCode,
        [Parameter(Mandatory)][string]$InvalidCode
    )

    if (-not (Test-Path -LiteralPath $Path -PathType Leaf)) {
        Add-Violation -Code $MissingCode -Path $Path -Message 'JSON 文件不存在。'
        return $null
    }

    try {
        Get-Content -LiteralPath $Path -Raw -Encoding utf8 | ConvertFrom-Json -Depth 100
    }
    catch {
        Add-Violation -Code $InvalidCode -Path $Path -Message $_.Exception.Message
        $null
    }
}

function Convert-ToDecimalValue {
    param(
        [AllowNull()]$Value,
        [Parameter(Mandatory)][string]$Code,
        [Parameter(Mandatory)][string]$Path,
        [Parameter(Mandatory)][string]$Message
    )

    try {
        if ($null -eq $Value) { throw '值为空。' }
        [decimal]$Value
    }
    catch {
        Add-Violation -Code $Code -Path $Path -Message $Message
        $null
    }
}

function Test-Sha256Value {
    param([AllowNull()]$Value)

    [string]$Value -match '^sha256:[0-9a-f]{64}$'
}

function Get-ExpectedCandidateUnits {
    [ordered]@{
        'presentation.first_view.elapsed|median' = 'milliseconds'
        'desktop.idle.working_set|median' = 'mebibytes'
        'rust.application-load-complete|median' = 'nanoseconds_per_operation'
        'rust.application-cancel-stale|median' = 'nanoseconds_per_operation'
        'rust.parity-repository-validation|median' = 'nanoseconds_per_operation'
        'presentation.first_view.elapsed|p95' = 'milliseconds'
        'desktop.idle.working_set|p95' = 'mebibytes'
        'rust.application-load-complete|p95' = 'nanoseconds_per_operation'
        'rust.application-cancel-stale|p95' = 'nanoseconds_per_operation'
        'rust.parity-repository-validation|p95' = 'nanoseconds_per_operation'
    }
}

function Get-ResultMetricValue {
    param(
        [Parameter(Mandatory)]$Result,
        [Parameter(Mandatory)][string]$Metric,
        [Parameter(Mandatory)][string]$Lane,
        [Parameter(Mandatory)][string]$Path
    )

    $statistics = $null
    $divisor = [decimal]1
    if ($Metric -eq 'presentation.first_view.elapsed') {
        $presentation = Get-JsonProperty -Object $Result -Name 'presentation_first_view'
        $statistics = Get-JsonProperty -Object $presentation -Name 'elapsed_nanoseconds_statistics'
        $divisor = [decimal]1000000
    }
    elseif ($Metric -eq 'desktop.idle.working_set') {
        $desktop = Get-JsonProperty -Object $Result -Name 'desktop_idle_resources'
        $statistics = Get-JsonProperty -Object $desktop -Name 'working_set_bytes_statistics'
        $divisor = [decimal]1048576
    }
    elseif ($Metric.StartsWith('rust.', [System.StringComparison]::Ordinal)) {
        $scenarioName = $Metric.Substring(5)
        $scenarioMatches = @((Get-JsonProperty -Object $Result -Name 'rust_scenarios') | Where-Object {
            (Get-JsonProperty -Object $_ -Name 'name') -eq $scenarioName
        })
        if ($scenarioMatches.Count -ne 1) {
            Add-Violation -Code 'RESULT_METRIC_MISSING' -Path $Path -Message "结果缺少唯一 Rust 指标 $Metric。"
            return $null
        }
        $statistics = Get-JsonProperty -Object $scenarioMatches[0] -Name 'nanoseconds_per_operation_statistics'
    }
    else {
        Add-Violation -Code 'BUDGET_METRIC_INVALID' -Path $Path -Message "预算包含不支持的指标 $Metric。"
        return $null
    }

    $rawValue = Get-JsonProperty -Object $statistics -Name $Lane
    $value = Convert-ToDecimalValue `
        -Value $rawValue `
        -Code 'RESULT_METRIC_MISSING' `
        -Path $Path `
        -Message "结果缺少指标 $Metric/$Lane 的数值。"
    if ($null -eq $value) { return $null }
    $value / $divisor
}

function Get-EnvironmentFingerprintHash {
    param(
        [Parameter(Mandatory)]$Environment,
        [Parameter(Mandatory)][string]$Path
    )

    try {
        $fingerprint = [ordered]@{
            image_version = [string](Get-JsonProperty -Object $Environment -Name 'image_version')
            os_description = [string](Get-JsonProperty -Object $Environment -Name 'os_description')
            processor = ([string](Get-JsonProperty -Object $Environment -Name 'processor')).Trim()
            logical_processor_count = [int64](Get-JsonProperty -Object $Environment -Name 'logical_processor_count')
            total_memory_bytes = [int64](Get-JsonProperty -Object $Environment -Name 'total_memory_bytes')
        }
        if ([string]::IsNullOrWhiteSpace($fingerprint.image_version) -or
            [string]::IsNullOrWhiteSpace($fingerprint.os_description) -or
            [string]::IsNullOrWhiteSpace($fingerprint.processor) -or
            $fingerprint.logical_processor_count -le 0 -or
            $fingerprint.total_memory_bytes -le 0) {
            throw '完整环境指纹字段缺失或无效。'
        }
        Get-Sha256Text -Text ($fingerprint | ConvertTo-Json -Compress)
    }
    catch {
        Add-Violation -Code 'RESULT_ENVIRONMENT_INVALID' -Path $Path -Message $_.Exception.Message
        $null
    }
}

function Test-ResultContract {
    param(
        [Parameter(Mandatory)]$Result,
        [Parameter(Mandatory)]$Config,
        [Parameter(Mandatory)][string]$ExpectedPlatform,
        [Parameter(Mandatory)][string]$Path,
        [Parameter(Mandatory)][string]$ConfigHash
    )

    if ((Get-JsonProperty -Object $Result -Name 'schema_version') -ne 'inputcodex.performance-result.v1') {
        Add-Violation -Code 'RESULT_SCHEMA_INVALID' -Path $Path -Message '结果 schema_version 无效。'
    }
    if ([int](Get-JsonProperty -Object $Result -Name 'issue_number') -ne 32) {
        Add-Violation -Code 'RESULT_ISSUE_INVALID' -Path $Path -Message '结果必须归属 Issue #32 测量合同。'
    }
    if ((Get-JsonProperty -Object $Result -Name 'status') -ne 'complete') {
        Add-Violation -Code 'RESULT_STATUS_INVALID' -Path $Path -Message '结果状态必须为 complete。'
    }
    if ((Get-JsonProperty -Object $Result -Name 'platform') -ne $ExpectedPlatform) {
        Add-Violation -Code 'RESULT_PLATFORM_INVALID' -Path $Path -Message "结果平台必须为 $ExpectedPlatform。"
    }

    $source = Get-JsonProperty -Object $Result -Name 'source'
    if ([string](Get-JsonProperty -Object $source -Name 'commit') -notmatch '^[0-9a-f]{40}$' -or
        [string](Get-JsonProperty -Object $source -Name 'tree') -notmatch '^[0-9a-f]{40}$') {
        Add-Violation -Code 'RESULT_SOURCE_INVALID' -Path $Path -Message '结果 commit/tree 必须为 40 位 SHA。'
    }
    if ((Get-JsonProperty -Object $source -Name 'config_sha256') -ne $ConfigHash) {
        Add-Violation -Code 'RESULT_CONFIG_HASH_INVALID' -Path $Path -Message '结果配置哈希与固定测量配置不一致。'
    }
    foreach ($hashField in 'implementation_sha256', 'input_sha256') {
        if (-not (Test-Sha256Value -Value (Get-JsonProperty -Object $source -Name $hashField))) {
            Add-Violation -Code 'RESULT_SOURCE_HASH_INVALID' -Path $Path -Message "结果 $hashField 不是有效 SHA-256。"
        }
    }

    $environment = Get-JsonProperty -Object $Result -Name 'environment'
    if ((Get-JsonProperty -Object $environment -Name 'runner_environment') -ne 'github-hosted') {
        Add-Violation -Code 'RESULT_RUNNER_INVALID' -Path $Path -Message '结果只能来自 github-hosted runner。'
    }

    $presentationConfig = Get-JsonProperty -Object $Config -Name 'probe'
    $presentation = Get-JsonProperty -Object $Result -Name 'presentation_first_view'
    foreach ($field in 'minimum_successful_samples', 'maximum_attempts', 'timeout_seconds') {
        if ([int](Get-JsonProperty -Object $presentation -Name $field) -ne [int](Get-JsonProperty -Object $presentationConfig -Name $field)) {
            Add-Violation -Code 'RESULT_SAMPLE_CONTRACT_INVALID' -Path $Path -Message "首次 view 字段 $field 不符合固定测量合同。"
        }
    }
    $attempts = @((Get-JsonProperty -Object $presentation -Name 'attempts'))
    $successfulAttempts = @($attempts | Where-Object { (Get-JsonProperty -Object $_ -Name 'status') -eq 'success' })
    if ($attempts.Count -gt [int](Get-JsonProperty -Object $presentationConfig -Name 'maximum_attempts') -or
        $successfulAttempts.Count -lt [int](Get-JsonProperty -Object $presentationConfig -Name 'minimum_successful_samples')) {
        Add-Violation -Code 'RESULT_SAMPLE_CONTRACT_INVALID' -Path $Path -Message '首次 view 样本数量不符合固定测量合同。'
    }

    $idleConfig = Get-JsonProperty -Object $Config -Name 'idle_resources'
    $desktop = Get-JsonProperty -Object $Result -Name 'desktop_idle_resources'
    foreach ($field in 'settle_seconds', 'sample_interval_seconds') {
        if ([int](Get-JsonProperty -Object $desktop -Name $field) -ne [int](Get-JsonProperty -Object $idleConfig -Name $field)) {
            Add-Violation -Code 'RESULT_SAMPLE_CONTRACT_INVALID' -Path $Path -Message "空闲资源字段 $field 不符合固定测量合同。"
        }
    }
    if (@((Get-JsonProperty -Object $desktop -Name 'samples')).Count -ne [int](Get-JsonProperty -Object $idleConfig -Name 'sample_count')) {
        Add-Violation -Code 'RESULT_SAMPLE_CONTRACT_INVALID' -Path $Path -Message '空闲资源样本数量不符合固定测量合同。'
    }

    $scenarioResults = @((Get-JsonProperty -Object $Result -Name 'rust_scenarios'))
    $scenarioConfigs = @((Get-JsonProperty -Object $Config -Name 'rust_scenarios'))
    if ($scenarioResults.Count -ne $scenarioConfigs.Count) {
        Add-Violation -Code 'RESULT_METRIC_MISSING' -Path $Path -Message 'Rust 场景数量不符合固定测量合同。'
    }
    foreach ($scenarioConfig in $scenarioConfigs) {
        $scenarioName = [string](Get-JsonProperty -Object $scenarioConfig -Name 'name')
        $matches = @($scenarioResults | Where-Object { (Get-JsonProperty -Object $_ -Name 'name') -eq $scenarioName })
        if ($matches.Count -ne 1) {
            Add-Violation -Code 'RESULT_METRIC_MISSING' -Path $Path -Message "缺少唯一 Rust 场景 $scenarioName。"
            continue
        }
        $scenario = $matches[0]
        $warmups = @((Get-JsonProperty -Object $scenario -Name 'warmups'))
        $samples = @((Get-JsonProperty -Object $scenario -Name 'samples'))
        if ($warmups.Count -ne [int](Get-JsonProperty -Object $scenarioConfig -Name 'warmup_runs') -or
            $samples.Count -ne [int](Get-JsonProperty -Object $scenarioConfig -Name 'sample_count')) {
            Add-Violation -Code 'RESULT_SAMPLE_CONTRACT_INVALID' -Path $Path -Message "$scenarioName 的样本数量不符合固定测量合同。"
        }
        if (@($samples | Where-Object {
            [uint64](Get-JsonProperty -Object $_ -Name 'iterations') -ne [uint64](Get-JsonProperty -Object $scenarioConfig -Name 'iterations')
        }).Count -gt 0) {
            Add-Violation -Code 'RESULT_SAMPLE_CONTRACT_INVALID' -Path $Path -Message "$scenarioName 的迭代次数不符合固定测量合同。"
        }
        $checksums = @($samples | ForEach-Object { [string](Get-JsonProperty -Object $_ -Name 'checksum') })
        if ($checksums.Count -eq 0 -or
            @($checksums | Where-Object { [string]::IsNullOrWhiteSpace($_) -or $_ -eq '0' }).Count -gt 0 -or
            @($checksums | Sort-Object -Unique).Count -ne 1) {
            Add-Violation -Code 'RESULT_SCENARIO_CHECKSUM_INVALID' -Path $Path -Message "$scenarioName 的 checksum 必须非零且一致。"
        }
    }
}

try {
    if (-not (Test-Path -LiteralPath $RepositoryRoot -PathType Container)) {
        Add-Violation -Code 'REPOSITORY_ROOT_NOT_FOUND' -Path $RepositoryRoot -Message '仓库根目录不存在。'
        throw '仓库根目录不存在。'
    }

    $resolvedRoot = (Resolve-Path -LiteralPath $RepositoryRoot).Path
    $resolvedBudgetPath = Resolve-InputPath -Root $resolvedRoot -Path $BudgetPath
    $resolvedResultPath = Resolve-InputPath -Root $resolvedRoot -Path $ResultPath
    $configPath = Join-Path $resolvedRoot 'benchmarks/config/issue-32-baseline.json'

    $budget = Read-JsonDocument -Path $resolvedBudgetPath -MissingCode 'BUDGET_FILE_MISSING' -InvalidCode 'BUDGET_JSON_INVALID'
    $result = Read-JsonDocument -Path $resolvedResultPath -MissingCode 'RESULT_FILE_MISSING' -InvalidCode 'RESULT_JSON_INVALID'
    $config = Read-JsonDocument -Path $configPath -MissingCode 'MEASUREMENT_CONFIG_MISSING' -InvalidCode 'MEASUREMENT_CONFIG_INVALID'

    if ($null -ne $budget) {
        $script:BudgetHash = Get-NormalizedTextHash -Path $resolvedBudgetPath
        if ($script:BudgetHash -ne $ExpectedBudgetSha256) {
            Add-Violation -Code 'BUDGET_HASH_MISMATCH' -Path $resolvedBudgetPath -Message "预算哈希必须为 $ExpectedBudgetSha256，实际为 $script:BudgetHash。"
        }
        if ((Get-JsonProperty -Object $budget -Name 'schema_version') -ne 'inputcodex.performance-budget-approval.v1') {
            Add-Violation -Code 'BUDGET_SCHEMA_INVALID' -Path $resolvedBudgetPath -Message '预算 schema_version 无效。'
        }
        if ([int](Get-JsonProperty -Object $budget -Name 'issue_number') -ne 59) {
            Add-Violation -Code 'BUDGET_ISSUE_INVALID' -Path $resolvedBudgetPath -Message '预算必须归属 Issue #59。'
        }
        if ((Get-JsonProperty -Object $budget -Name 'status') -ne 'approved-observation') {
            Add-Violation -Code 'BUDGET_STATUS_INVALID' -Path $resolvedBudgetPath -Message '预算状态必须为 approved-observation。'
        }
        if ([bool](Get-JsonProperty -Object $budget -Name 'budget_ci_enabled') -or [bool](Get-JsonProperty -Object $budget -Name 'gate_5_unlocked')) {
            Add-Violation -Code 'BUDGET_GUARD_INVALID' -Path $resolvedBudgetPath -Message '预算 CI 与 Gate 5 必须继续锁定。'
        }
    }

    if ($null -ne $config) {
        if ((Get-JsonProperty -Object $config -Name 'schema_version') -ne 'inputcodex.performance-baseline.v1' -or
            [int](Get-JsonProperty -Object $config -Name 'issue_number') -ne 32) {
            Add-Violation -Code 'MEASUREMENT_CONFIG_INVALID' -Path $configPath -Message '固定测量配置身份无效。'
        }
    }

    $platformBudget = $null
    $candidates = @()
    if ($null -ne $budget) {
        $platforms = Get-JsonProperty -Object $budget -Name 'platforms'
        $platformBudget = Get-JsonProperty -Object $platforms -Name $Platform
        if ($null -eq $platformBudget) {
            Add-Violation -Code 'BUDGET_PLATFORM_MISSING' -Path $resolvedBudgetPath -Message "预算缺少 $Platform 平台。"
        }
        else {
            $expectedUnits = Get-ExpectedCandidateUnits
            $seenKeys = @{}
            $candidates = @((Get-JsonProperty -Object $platformBudget -Name 'blocking_candidates'))
            foreach ($candidate in $candidates) {
                $metric = [string](Get-JsonProperty -Object $candidate -Name 'metric')
                $lane = [string](Get-JsonProperty -Object $candidate -Name 'lane')
                $unit = [string](Get-JsonProperty -Object $candidate -Name 'unit')
                $key = "$metric|$lane"
                if ($seenKeys.ContainsKey($key)) {
                    Add-Violation -Code 'BUDGET_METRIC_DUPLICATED' -Path $resolvedBudgetPath -Message "预算指标重复：$key。"
                }
                else {
                    $seenKeys[$key] = $true
                }
                if (-not $expectedUnits.Contains($key)) {
                    Add-Violation -Code 'BUDGET_METRIC_INVALID' -Path $resolvedBudgetPath -Message "预算指标不受支持：$key。"
                }
                elseif ($unit -ne $expectedUnits[$key]) {
                    Add-Violation -Code 'BUDGET_UNIT_INVALID' -Path $resolvedBudgetPath -Message "预算指标 $key 的单位必须为 $($expectedUnits[$key])。"
                }
                $warning = Convert-ToDecimalValue -Value (Get-JsonProperty -Object $candidate -Name 'warning_limit') -Code 'BUDGET_LIMIT_INVALID' -Path $resolvedBudgetPath -Message "预算指标 $key 的 warning_limit 无效。"
                $blocking = Convert-ToDecimalValue -Value (Get-JsonProperty -Object $candidate -Name 'blocking_limit') -Code 'BUDGET_LIMIT_INVALID' -Path $resolvedBudgetPath -Message "预算指标 $key 的 blocking_limit 无效。"
                $quantum = Convert-ToDecimalValue -Value (Get-JsonProperty -Object $candidate -Name 'quantum') -Code 'BUDGET_LIMIT_INVALID' -Path $resolvedBudgetPath -Message "预算指标 $key 的 quantum 无效。"
                if ($null -ne $warning -and $null -ne $blocking -and ($warning -lt 0 -or $blocking -lt $warning)) {
                    Add-Violation -Code 'BUDGET_LIMIT_INVALID' -Path $resolvedBudgetPath -Message "预算指标 $key 的阈值顺序无效。"
                }
                if ($null -ne $quantum -and $quantum -lt 0) {
                    Add-Violation -Code 'BUDGET_LIMIT_INVALID' -Path $resolvedBudgetPath -Message "预算指标 $key 的 quantum 不得为负数。"
                }
            }
            foreach ($expectedKey in $expectedUnits.Keys) {
                if (-not $seenKeys.ContainsKey($expectedKey)) {
                    Add-Violation -Code 'BUDGET_METRIC_MISSING' -Path $resolvedBudgetPath -Message "预算缺少指标 $expectedKey。"
                }
            }
            if ($candidates.Count -ne 10) {
                Add-Violation -Code 'BUDGET_METRIC_COUNT_INVALID' -Path $resolvedBudgetPath -Message '预算候选必须恰好为十个。'
            }
            if (-not (Test-Sha256Value -Value (Get-JsonProperty -Object $platformBudget -Name 'environment_fingerprint_sha256'))) {
                Add-Violation -Code 'BUDGET_ENVIRONMENT_FINGERPRINT_INVALID' -Path $resolvedBudgetPath -Message '预算完整环境指纹无效。'
            }
        }
    }

    if ($null -ne $result -and $null -ne $config) {
        $script:ResultHash = Get-NormalizedTextHash -Path $resolvedResultPath
        Test-ResultContract `
            -Result $result `
            -Config $config `
            -ExpectedPlatform $Platform `
            -Path $resolvedResultPath `
            -ConfigHash (Get-NormalizedTextHash -Path $configPath)
    }

    if ($script:Violations.Count -gt 0) {
        $failure = [pscustomobject][ordered]@{
            schema_version = 'inputcodex.performance-budget-observation.v1'
            ok = $false
            platform = $Platform
            comparable = $false
            classification = 'contract-invalid'
            budget_sha256 = $script:BudgetHash
            result_sha256 = $script:ResultHash
            observation_count = 0
            observations = @()
            violation_count = $script:Violations.Count
            violations = @($script:Violations)
        }
        [Console]::Out.WriteLine((ConvertTo-Json -InputObject $failure -Depth 100 -Compress))
        exit 1
    }

    $environment = Get-JsonProperty -Object $result -Name 'environment'
    $actualFingerprint = Get-EnvironmentFingerprintHash -Environment $environment -Path $resolvedResultPath
    if ($script:Violations.Count -gt 0) {
        throw '环境指纹计算失败。'
    }
    $expectedFingerprint = [string](Get-JsonProperty -Object $platformBudget -Name 'environment_fingerprint_sha256')
    $comparable = $actualFingerprint -eq $expectedFingerprint
    $observations = [System.Collections.Generic.List[object]]::new()
    $hasWarning = $false
    $hasBlocking = $false

    foreach ($candidate in $candidates) {
        $metric = [string](Get-JsonProperty -Object $candidate -Name 'metric')
        $lane = [string](Get-JsonProperty -Object $candidate -Name 'lane')
        $unit = [string](Get-JsonProperty -Object $candidate -Name 'unit')
        $value = Get-ResultMetricValue -Result $result -Metric $metric -Lane $lane -Path $resolvedResultPath
        if ($null -eq $value) { continue }
        $warning = [decimal](Get-JsonProperty -Object $candidate -Name 'warning_limit')
        $blocking = [decimal](Get-JsonProperty -Object $candidate -Name 'blocking_limit')
        $status = 'not-comparable'
        if ($comparable) {
            if ($value -ge $blocking) {
                $status = 'blocking-observed'
                $hasBlocking = $true
            }
            elseif ($value -ge $warning) {
                $status = 'warning-observed'
                $hasWarning = $true
            }
            else {
                $status = 'within-budget'
            }
        }
        $observations.Add([pscustomobject][ordered]@{
            metric = $metric
            lane = $lane
            unit = $unit
            value = $value
            warning_limit = $warning
            blocking_limit = $blocking
            status = $status
        })
    }

    if ($script:Violations.Count -gt 0 -or $observations.Count -ne 10) {
        if ($observations.Count -ne 10) {
            Add-Violation -Code 'RESULT_METRIC_MISSING' -Path $resolvedResultPath -Message '未能提取十个预算观察指标。'
        }
        $failure = [pscustomobject][ordered]@{
            schema_version = 'inputcodex.performance-budget-observation.v1'
            ok = $false
            platform = $Platform
            comparable = $false
            classification = 'contract-invalid'
            budget_sha256 = $script:BudgetHash
            result_sha256 = $script:ResultHash
            observation_count = $observations.Count
            observations = @($observations)
            violation_count = $script:Violations.Count
            violations = @($script:Violations)
        }
        [Console]::Out.WriteLine((ConvertTo-Json -InputObject $failure -Depth 100 -Compress))
        exit 1
    }

    $classification = if (-not $comparable) {
        'not-comparable'
    }
    elseif ($hasBlocking) {
        'blocking-observed'
    }
    elseif ($hasWarning) {
        'warning-observed'
    }
    else {
        'within-budget'
    }

    $output = [pscustomobject][ordered]@{
        schema_version = 'inputcodex.performance-budget-observation.v1'
        ok = $true
        platform = $Platform
        comparable = $comparable
        classification = $classification
        budget_sha256 = $script:BudgetHash
        result_sha256 = $script:ResultHash
        environment_fingerprint_sha256 = $actualFingerprint
        expected_environment_fingerprint_sha256 = $expectedFingerprint
        observation_count = $observations.Count
        observations = @($observations)
        violation_count = 0
        violations = @()
    }
    [Console]::Out.WriteLine((ConvertTo-Json -InputObject $output -Depth 100 -Compress))
    exit 0
}
catch {
    if ($script:Violations.Count -eq 0) {
        Add-Violation -Code 'OBSERVATION_EXCEPTION' -Path $RepositoryRoot -Message $_.Exception.Message
    }
    $failure = [pscustomobject][ordered]@{
        schema_version = 'inputcodex.performance-budget-observation.v1'
        ok = $false
        platform = $Platform
        comparable = $false
        classification = 'contract-invalid'
        budget_sha256 = $script:BudgetHash
        result_sha256 = $script:ResultHash
        observation_count = 0
        observations = @()
        violation_count = $script:Violations.Count
        violations = @($script:Violations)
    }
    [Console]::Out.WriteLine((ConvertTo-Json -InputObject $failure -Depth 100 -Compress))
    exit 1
}
