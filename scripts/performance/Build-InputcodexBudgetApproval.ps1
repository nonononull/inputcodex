[CmdletBinding()]
param(
    [Parameter(Mandatory)]
    [string]$RepositoryRoot,

    [string]$HistoricalManifestPath = 'benchmarks/results/issue-54/manifest.json',

    [string]$CurrentManifestPath = 'benchmarks/results/issue-59/manifest.json',

    [Parameter(Mandatory)]
    [string]$OutputPath
)

Set-StrictMode -Version Latest
$ErrorActionPreference = 'Stop'
$script:InvariantCulture = [System.Globalization.CultureInfo]::InvariantCulture
$resolvedRoot = (Resolve-Path -LiteralPath $RepositoryRoot).Path

function Stop-Budget {
    param(
        [Parameter(Mandatory)][string]$Code,
        [Parameter(Mandatory)][string]$Message
    )

    throw "${Code}: $Message"
}

function Resolve-ProjectPath {
    param(
        [Parameter(Mandatory)][string]$Root,
        [Parameter(Mandatory)][string]$Path,
        [switch]$MustExist
    )

    $fullPath = if ([System.IO.Path]::IsPathRooted($Path)) {
        [System.IO.Path]::GetFullPath($Path)
    }
    else {
        [System.IO.Path]::GetFullPath((Join-Path $Root $Path))
    }
    if ($MustExist -and -not (Test-Path -LiteralPath $fullPath -PathType Leaf)) {
        Stop-Budget -Code 'BUDGET_INPUT_MISSING' -Message "输入文件不存在：$Path"
    }
    $fullPath
}

function Read-JsonFile {
    param([Parameter(Mandatory)][string]$Path)

    try {
        Get-Content -LiteralPath $Path -Raw -Encoding utf8 | ConvertFrom-Json -Depth 100
    }
    catch {
        Stop-Budget -Code 'BUDGET_JSON_INVALID' -Message "$Path 不是有效 JSON：$($_.Exception.Message)"
    }
}

function Get-Sha256Text {
    param([Parameter(Mandatory)][string]$Text)

    $bytes = [System.Text.UTF8Encoding]::new($false).GetBytes($Text)
    'sha256:' + [Convert]::ToHexString([System.Security.Cryptography.SHA256]::HashData($bytes)).ToLowerInvariant()
}

function Get-NormalizedTextHash {
    param([Parameter(Mandatory)][string]$Path)

    $text = [System.IO.File]::ReadAllText($Path, [System.Text.Encoding]::UTF8)
    Get-Sha256Text -Text ([regex]::Replace($text, '\r\n?', "`n"))
}

function Convert-ToDecimal {
    param([Parameter(Mandatory)]$Value)

    [decimal]::Parse([Convert]::ToString($Value, $script:InvariantCulture), [System.Globalization.NumberStyles]::Float, $script:InvariantCulture)
}

function Get-Median {
    param([Parameter(Mandatory)][object[]]$Values)

    if ($Values.Count -eq 0) {
        Stop-Budget -Code 'BUDGET_EMPTY_VALUES' -Message '无法对空集合计算中位数。'
    }
    $sorted = @($Values | ForEach-Object { Convert-ToDecimal -Value $_ } | Sort-Object)
    $middle = [int][math]::Floor($sorted.Count / 2)
    if (($sorted.Count % 2) -eq 1) {
        return [decimal]$sorted[$middle]
    }
    ([decimal]$sorted[$middle - 1] + [decimal]$sorted[$middle]) / [decimal]2
}

function Round-UpToQuantum {
    param(
        [Parameter(Mandatory)][decimal]$Value,
        [Parameter(Mandatory)][decimal]$Quantum
    )

    if ($Quantum -le 0) {
        Stop-Budget -Code 'BUDGET_QUANTUM_INVALID' -Message "量子必须为正数：$Quantum"
    }
    [decimal]::Ceiling($Value / $Quantum) * $Quantum
}

function Test-SequenceEqual {
    param(
        [AllowNull()][object[]]$Left,
        [AllowNull()][object[]]$Right
    )

    (@($Left) | ConvertTo-Json -Compress -Depth 20) -eq (@($Right) | ConvertTo-Json -Compress -Depth 20)
}

function Get-ResultEvidence {
    param(
        [Parameter(Mandatory)]$Run,
        [Parameter(Mandatory)][string]$Platform,
        [Parameter(Mandatory)][int]$IssueNumber,
        [Parameter(Mandatory)][string]$Slot
    )

    $summary = $Run.results.$Platform
    if ($null -eq $summary) {
        Stop-Budget -Code 'BUDGET_RESULT_SUMMARY_MISSING' -Message "Issue #$IssueNumber/$Slot 缺少 $Platform 摘要。"
    }
    $resultPath = Resolve-ProjectPath -Root $resolvedRoot -Path ([string]$summary.path) -MustExist
    $normalizedHash = Get-NormalizedTextHash -Path $resultPath
    if ($normalizedHash -ne [string]$summary.normalized_sha256) {
        Stop-Budget -Code 'BUDGET_RESULT_HASH_MISMATCH' -Message "Issue #$IssueNumber/$Slot/$Platform 归一化哈希漂移。"
    }

    $result = Read-JsonFile -Path $resultPath
    if ($result.schema_version -ne 'inputcodex.performance-result.v1' -or $result.status -ne 'complete' -or $result.platform -ne $Platform) {
        Stop-Budget -Code 'BUDGET_RESULT_IDENTITY_INVALID' -Message "Issue #$IssueNumber/$Slot/$Platform 结果身份或状态无效。"
    }
    if ([string]$result.github.run_id -ne [string]$Run.github.run_id -or [int]$result.github.run_attempt -ne [int]$Run.github.run_attempt) {
        Stop-Budget -Code 'BUDGET_RUN_ID_MISMATCH' -Message "Issue #$IssueNumber/$Slot/$Platform Run/attempt 与 manifest 不一致。"
    }
    if ([string]$result.source.commit -ne [string]$summary.source.commit -or [string]$result.source.tree -ne [string]$summary.source.tree) {
        Stop-Budget -Code 'BUDGET_SOURCE_MISMATCH' -Message "Issue #$IssueNumber/$Slot/$Platform source 与 manifest 不一致。"
    }
    if ($result.environment.runner_environment -ne 'github-hosted') {
        Stop-Budget -Code 'BUDGET_RUNNER_INVALID' -Message "Issue #$IssueNumber/$Slot/$Platform 不是 github-hosted。"
    }

    $rustRelease = ([regex]::Match([string]$result.environment.rustc, '(?m)^release:\s*(.+)$')).Groups[1].Value.Trim()
    $rustHost = ([regex]::Match([string]$result.environment.rustc, '(?m)^host:\s*(.+)$')).Groups[1].Value.Trim()
    if (-not $rustRelease -or -not $rustHost) {
        Stop-Budget -Code 'BUDGET_RUST_IDENTITY_INVALID' -Message "Issue #$IssueNumber/$Slot/$Platform 无法提取 Rust release/host。"
    }

    $sampleContract = [ordered]@{
        version = 'inputcodex.performance-sample-contract.v1'
        first_view = [ordered]@{
            minimum_successful_samples = [int64]$result.presentation_first_view.minimum_successful_samples
            maximum_attempts = [int64]$result.presentation_first_view.maximum_attempts
            timeout_seconds = [int64]$result.presentation_first_view.timeout_seconds
        }
        idle_resources = [ordered]@{
            settle_seconds = [int64]$result.desktop_idle_resources.settle_seconds
            sample_interval_seconds = [int64]$result.desktop_idle_resources.sample_interval_seconds
            sample_count = [int64]@($result.desktop_idle_resources.samples).Count
        }
        rust_scenarios = @($result.rust_scenarios | ForEach-Object {
            [ordered]@{
                name = [string]$_.name
                sample_count = [int64]@($_.samples).Count
            }
        })
    }
    $sampleContractHash = Get-Sha256Text -Text ($sampleContract | ConvertTo-Json -Compress -Depth 20)
    $hardKey = [string]::Join('|', @(
        [string]$result.schema_version,
        ([string]$result.platform).ToLowerInvariant(),
        [string]$result.environment.runner_arch,
        [string]$result.environment.runner_environment,
        [string]$result.environment.image_os,
        "$rustRelease/$rustHost",
        [string]$result.source.config_sha256,
        [string]$result.source.input_sha256,
        [string]$sampleContractHash
    ))
    $hardKeyHash = Get-Sha256Text -Text $hardKey
    $fingerprint = [ordered]@{
        image_version = [string]$result.environment.image_version
        os_description = [string]$result.environment.os_description
        processor = ([string]$result.environment.processor).Trim()
        logical_processor_count = [int64]$result.environment.logical_processor_count
        total_memory_bytes = [int64]$result.environment.total_memory_bytes
    }
    $fingerprintHash = Get-Sha256Text -Text ($fingerprint | ConvertTo-Json -Compress)
    if ($hardKeyHash -ne [string]$summary.hard_key_sha256 -or $fingerprintHash -ne [string]$summary.environment_fingerprint_sha256) {
        Stop-Budget -Code 'BUDGET_MANIFEST_SUMMARY_MISMATCH' -Message "Issue #$IssueNumber/$Slot/$Platform hard key 或完整指纹摘要漂移。"
    }

    $attempts = @($result.presentation_first_view.attempts)
    $successfulAttempts = @($attempts | Where-Object { $_.status -eq 'success' })
    if ($successfulAttempts.Count -lt [int]$result.presentation_first_view.minimum_successful_samples) {
        Stop-Budget -Code 'BUDGET_SAMPLE_COUNT_INVALID' -Message "Issue #$IssueNumber/$Slot/$Platform 首次 view 成功样本不足。"
    }
    if (@($result.desktop_idle_resources.samples).Count -ne 60) {
        Stop-Budget -Code 'BUDGET_SAMPLE_COUNT_INVALID' -Message "Issue #$IssueNumber/$Slot/$Platform 空闲资源样本数不是 60。"
    }

    foreach ($scenario in @($result.rust_scenarios)) {
        if (@($scenario.samples).Count -ne 20) {
            Stop-Budget -Code 'BUDGET_SAMPLE_COUNT_INVALID' -Message "Issue #$IssueNumber/$Slot/$Platform/$($scenario.name) 样本数不是 20。"
        }
        $checksums = @($scenario.samples | ForEach-Object { [string]$_.checksum } | Sort-Object -Unique)
        if ($checksums.Count -ne 1) {
            Stop-Budget -Code 'BUDGET_CHECKSUM_INCONSISTENT' -Message "Issue #$IssueNumber/$Slot/$Platform/$($scenario.name) 单 Run checksum 不一致。"
        }
        $summaryScenario = @($summary.sample_integrity.rust_scenarios | Where-Object { $_.name -eq $scenario.name })
        if ($summaryScenario.Count -ne 1 -or [string]$summaryScenario[0].checksum -ne [string]$checksums[0]) {
            Stop-Budget -Code 'BUDGET_MANIFEST_SUMMARY_MISMATCH' -Message "Issue #$IssueNumber/$Slot/$Platform/$($scenario.name) checksum 摘要漂移。"
        }
        if (-not (Test-SequenceEqual -Left @($summaryScenario[0].outlier_indices) -Right @($scenario.nanoseconds_per_operation_statistics.outlier_indices))) {
            Stop-Budget -Code 'BUDGET_MANIFEST_SUMMARY_MISMATCH' -Message "Issue #$IssueNumber/$Slot/$Platform/$($scenario.name) IQR 摘要漂移。"
        }
    }

    [pscustomobject][ordered]@{
        issue_number = $IssueNumber
        slot = $Slot
        run_id = [string]$Run.github.run_id
        run_attempt = [int]$Run.github.run_attempt
        result_path = [string]$summary.path
        normalized_sha256 = $normalizedHash
        hard_key_sha256 = $hardKeyHash
        environment_fingerprint_sha256 = $fingerprintHash
        result = $result
    }
}

function Assert-UniqueRuns {
    param(
        [Parameter(Mandatory)][object[]]$Records,
        [Parameter(Mandatory)][string]$Platform
    )

    if (@($Records.run_id | Sort-Object -Unique).Count -ne $Records.Count) {
        Stop-Budget -Code 'BUDGET_DUPLICATE_RUN_ID' -Message "$Platform 来源包含重复 run_id。"
    }
}

function New-SourceValue {
    param(
        [Parameter(Mandatory)]$Record,
        [Parameter(Mandatory)][decimal]$Value,
        [AllowNull()][object[]]$OutlierIndices,
        [AllowNull()][string]$Checksum
    )

    $source = [ordered]@{
        issue_number = [int]$Record.issue_number
        slot = [string]$Record.slot
        run_id = [string]$Record.run_id
        value = $Value
        outlier_indices = @($OutlierIndices | ForEach-Object { [int]$_ })
    }
    if ($Checksum) {
        $source.checksum = $Checksum
    }
    [pscustomobject]$source
}

function New-ValueSummary {
    param([Parameter(Mandatory)][object[]]$SourceValues)

    $values = @($SourceValues | ForEach-Object { Convert-ToDecimal -Value $_.value })
    $center = Get-Median -Values $values
    $deviations = @($values | ForEach-Object { [decimal]::Abs($_ - $center) })
    [ordered]@{
        source_values = $SourceValues
        minimum = [decimal]($values | Measure-Object -Minimum).Minimum
        maximum = [decimal]($values | Measure-Object -Maximum).Maximum
        center = $center
        mad = Get-Median -Values $deviations
    }
}

function New-BudgetLane {
    param(
        [Parameter(Mandatory)][string]$Metric,
        [Parameter(Mandatory)][string]$Lane,
        [Parameter(Mandatory)][string]$Unit,
        [Parameter(Mandatory)][decimal]$Quantum,
        [Parameter(Mandatory)][object[]]$SourceValues
    )

    $summary = New-ValueSummary -SourceValues $SourceValues
    $warningMad = [decimal]3 * [decimal]$summary.mad
    $warningPercent = [decimal]0.10 * [decimal]$summary.center
    $warningMargin = if ($warningMad -gt $warningPercent) { $warningMad } else { $warningPercent }
    $blockingMad = [decimal]5 * [decimal]$summary.mad
    $blockingPercent = [decimal]0.20 * [decimal]$summary.center
    $blockingMargin = if ($blockingMad -gt $blockingPercent) { $blockingMad } else { $blockingPercent }
    [pscustomobject][ordered]@{
        metric = $Metric
        lane = $Lane
        unit = $Unit
        quantum = $Quantum
        source_values = $summary.source_values
        minimum = $summary.minimum
        maximum = $summary.maximum
        center = $summary.center
        mad = $summary.mad
        warning_margin = $warningMargin
        blocking_margin = $blockingMargin
        warning_limit = Round-UpToQuantum -Value ([decimal]$summary.center + $warningMargin) -Quantum $Quantum
        blocking_limit = Round-UpToQuantum -Value ([decimal]$summary.center + $blockingMargin) -Quantum $Quantum
    }
}

function New-ObservationLane {
    param(
        [Parameter(Mandatory)][string]$Metric,
        [Parameter(Mandatory)][string]$Lane,
        [Parameter(Mandatory)][string]$Unit,
        [Parameter(Mandatory)][object[]]$SourceValues
    )

    $summary = New-ValueSummary -SourceValues $SourceValues
    [pscustomobject][ordered]@{
        metric = $Metric
        lane = $Lane
        unit = $Unit
        source_values = $summary.source_values
        minimum = $summary.minimum
        maximum = $summary.maximum
        center = $summary.center
        mad = $summary.mad
    }
}

function Get-StatisticsSources {
    param(
        [Parameter(Mandatory)][object[]]$Records,
        [Parameter(Mandatory)][scriptblock]$ValueSelector,
        [Parameter(Mandatory)][scriptblock]$OutlierSelector,
        [AllowNull()][scriptblock]$ChecksumSelector
    )

    @($Records | ForEach-Object {
        $record = $_
        $checksum = if ($ChecksumSelector) { [string](& $ChecksumSelector $record) } else { $null }
        New-SourceValue -Record $record -Value (Convert-ToDecimal -Value (& $ValueSelector $record)) -OutlierIndices @(& $OutlierSelector $record) -Checksum $checksum
    })
}

function New-PlatformBudget {
    param(
        [Parameter(Mandatory)][string]$Platform,
        [Parameter(Mandatory)][object[]]$Records
    )

    $blockingCandidates = [System.Collections.Generic.List[object]]::new()
    foreach ($lane in @('median', 'p95')) {
        $sources = Get-StatisticsSources -Records $Records -ValueSelector {
            param($record)
            (Convert-ToDecimal -Value $record.result.presentation_first_view.elapsed_nanoseconds_statistics.$lane) / [decimal]1000000
        } -OutlierSelector {
            param($record)
            @($record.result.presentation_first_view.elapsed_nanoseconds_statistics.outlier_indices)
        } -ChecksumSelector $null
        $blockingCandidates.Add((New-BudgetLane -Metric 'presentation.first_view.elapsed' -Lane $lane -Unit 'milliseconds' -Quantum ([decimal]1) -SourceValues $sources))

        $sources = Get-StatisticsSources -Records $Records -ValueSelector {
            param($record)
            (Convert-ToDecimal -Value $record.result.desktop_idle_resources.working_set_bytes_statistics.$lane) / [decimal]1048576
        } -OutlierSelector {
            param($record)
            @($record.result.desktop_idle_resources.working_set_bytes_statistics.outlier_indices)
        } -ChecksumSelector $null
        $blockingCandidates.Add((New-BudgetLane -Metric 'desktop.idle.working_set' -Lane $lane -Unit 'mebibytes' -Quantum ([decimal]1) -SourceValues $sources))

        foreach ($scenarioName in @('application-load-complete', 'application-cancel-stale', 'parity-repository-validation')) {
            $checksums = @($Records | ForEach-Object {
                $scenario = @($_.result.rust_scenarios | Where-Object { $_.name -eq $scenarioName })
                if ($scenario.Count -ne 1) {
                    Stop-Budget -Code 'BUDGET_SCENARIO_MISSING' -Message "$Platform/$scenarioName 场景缺失或重复。"
                }
                [string]$scenario[0].samples[0].checksum
            } | Sort-Object -Unique)
            if ($checksums.Count -ne 1) {
                Stop-Budget -Code 'BUDGET_CHECKSUM_INCONSISTENT' -Message "$Platform/$scenarioName 跨 Run checksum 不一致。"
            }
            $sources = Get-StatisticsSources -Records $Records -ValueSelector {
                param($record)
                $scenario = @($record.result.rust_scenarios | Where-Object { $_.name -eq $scenarioName })[0]
                Convert-ToDecimal -Value $scenario.nanoseconds_per_operation_statistics.$lane
            } -OutlierSelector {
                param($record)
                $scenario = @($record.result.rust_scenarios | Where-Object { $_.name -eq $scenarioName })[0]
                @($scenario.nanoseconds_per_operation_statistics.outlier_indices)
            } -ChecksumSelector {
                param($record)
                $scenario = @($record.result.rust_scenarios | Where-Object { $_.name -eq $scenarioName })[0]
                [string]$scenario.samples[0].checksum
            }
            $blockingCandidates.Add((New-BudgetLane -Metric "rust.$scenarioName" -Lane $lane -Unit 'nanoseconds_per_operation' -Quantum ([decimal]0.001) -SourceValues $sources))
        }
    }

    $observations = [System.Collections.Generic.List[object]]::new()
    foreach ($lane in @('median', 'p95')) {
        $sources = Get-StatisticsSources -Records $Records -ValueSelector {
            param($record)
            Convert-ToDecimal -Value $record.result.desktop_idle_resources.cpu_percent_statistics.$lane
        } -OutlierSelector {
            param($record)
            @($record.result.desktop_idle_resources.cpu_percent_statistics.outlier_indices)
        } -ChecksumSelector $null
        $observations.Add((New-ObservationLane -Metric 'desktop.idle.cpu' -Lane $lane -Unit 'percent' -SourceValues $sources))
    }
    $sources = Get-StatisticsSources -Records $Records -ValueSelector {
        param($record)
        Convert-ToDecimal -Value $record.result.build.desktop_release.binary_size_bytes
    } -OutlierSelector { param($record) @() } -ChecksumSelector $null
    $observations.Add((New-ObservationLane -Metric 'desktop.release.binary_size' -Lane 'value' -Unit 'bytes' -SourceValues $sources))

    $diagnostics = [System.Collections.Generic.List[object]]::new()
    foreach ($definition in @(
        [pscustomobject]@{ metric = 'desktop.release.build_duration'; unit = 'milliseconds'; selector = { param($record) Convert-ToDecimal -Value $record.result.build.desktop_release.duration_milliseconds } },
        [pscustomobject]@{ metric = 'baseline.release.build_duration'; unit = 'milliseconds'; selector = { param($record) Convert-ToDecimal -Value $record.result.build.baseline_release.duration_milliseconds } },
        [pscustomobject]@{ metric = 'baseline.release.binary_size'; unit = 'bytes'; selector = { param($record) Convert-ToDecimal -Value $record.result.build.baseline_release.binary_size_bytes } }
    )) {
        $sources = Get-StatisticsSources -Records $Records -ValueSelector $definition.selector -OutlierSelector { param($record) @() } -ChecksumSelector $null
        $diagnostics.Add((New-ObservationLane -Metric $definition.metric -Lane 'value' -Unit $definition.unit -SourceValues $sources))
    }

    [pscustomobject][ordered]@{
        hard_key_sha256 = [string]$Records[0].hard_key_sha256
        environment_fingerprint_sha256 = [string]$Records[0].environment_fingerprint_sha256
        source_runs = @($Records | ForEach-Object {
            [pscustomobject][ordered]@{
                issue_number = [int]$_.issue_number
                slot = [string]$_.slot
                run_id = [string]$_.run_id
                run_attempt = [int]$_.run_attempt
                result_path = [string]$_.result_path
                normalized_sha256 = [string]$_.normalized_sha256
            }
        })
        blocking_candidates = @($blockingCandidates)
        observations = @($observations)
        diagnostics = @($diagnostics)
    }
}

$historicalManifestFull = Resolve-ProjectPath -Root $resolvedRoot -Path $HistoricalManifestPath -MustExist
$currentManifestFull = Resolve-ProjectPath -Root $resolvedRoot -Path $CurrentManifestPath -MustExist
$historicalManifest = Read-JsonFile -Path $historicalManifestFull
$currentManifest = Read-JsonFile -Path $currentManifestFull
if ([int]$historicalManifest.issue_number -ne 54 -or [int]$currentManifest.issue_number -ne 59) {
    Stop-Budget -Code 'BUDGET_MANIFEST_IDENTITY_INVALID' -Message '历史/当前 manifest 必须归属 Issue #54/#59。'
}

$targetWindowsHardKey = [string]$currentManifest.policy.target_windows_hard_key_sha256
$targetWindowsFingerprint = [string]$currentManifest.policy.target_windows_environment_fingerprint_sha256
$windowsRecords = [System.Collections.Generic.List[object]]::new()
foreach ($slot in @($currentManifest.historical_evidence.windows_target_candidate_slots)) {
    $matches = @($historicalManifest.runs | Where-Object { $_.slot -eq $slot })
    if ($matches.Count -ne 1) {
        Stop-Budget -Code 'BUDGET_HISTORICAL_SLOT_INVALID' -Message "历史 Windows 槽位 $slot 缺失或重复。"
    }
    $record = Get-ResultEvidence -Run $matches[0] -Platform 'windows' -IssueNumber 54 -Slot ([string]$slot)
    if ($record.hard_key_sha256 -ne $targetWindowsHardKey -or $record.environment_fingerprint_sha256 -ne $targetWindowsFingerprint) {
        Stop-Budget -Code 'BUDGET_HISTORICAL_QUEUE_MISMATCH' -Message "历史 Windows 槽位 $slot 不属于目标完整队列。"
    }
    $windowsRecords.Add($record)
}
foreach ($run in @($currentManifest.runs)) {
    if ($run.classification.windows.value -ne 'comparable-valid') {
        continue
    }
    $record = Get-ResultEvidence -Run $run -Platform 'windows' -IssueNumber 59 -Slot ([string]$run.slot)
    if ($record.hard_key_sha256 -ne $targetWindowsHardKey -or $record.environment_fingerprint_sha256 -ne $targetWindowsFingerprint) {
        Stop-Budget -Code 'BUDGET_COMPARABLE_QUEUE_MISMATCH' -Message "Issue #59/$($run.slot)/windows 被标记 comparable-valid，但不属于目标完整队列。"
    }
    $windowsRecords.Add($record)
}
Assert-UniqueRuns -Records @($windowsRecords) -Platform 'windows'
if ($windowsRecords.Count -lt 5) {
    Stop-Budget -Code 'BUDGET_INSUFFICIENT_COMPARABLE_RUNS' -Message "Windows 目标队列只有 $($windowsRecords.Count) 个独立 Run。"
}

$macosCandidates = [System.Collections.Generic.List[object]]::new()
foreach ($run in @($historicalManifest.runs)) {
    if ($run.classification.macos.value -eq 'comparable-valid') {
        $macosCandidates.Add((Get-ResultEvidence -Run $run -Platform 'macos' -IssueNumber 54 -Slot ([string]$run.slot)))
    }
}
foreach ($run in @($currentManifest.runs)) {
    if ($run.classification.macos.value -eq 'comparable-valid') {
        $macosCandidates.Add((Get-ResultEvidence -Run $run -Platform 'macos' -IssueNumber 59 -Slot ([string]$run.slot)))
    }
}
if ($macosCandidates.Count -eq 0) {
    Stop-Budget -Code 'BUDGET_INSUFFICIENT_COMPARABLE_RUNS' -Message 'macOS 没有可比 Run。'
}
$targetMacosHardKey = [string]$macosCandidates[0].hard_key_sha256
$targetMacosFingerprint = [string]$macosCandidates[0].environment_fingerprint_sha256
foreach ($record in @($macosCandidates)) {
    if ($record.hard_key_sha256 -ne $targetMacosHardKey -or $record.environment_fingerprint_sha256 -ne $targetMacosFingerprint) {
        Stop-Budget -Code 'BUDGET_COMPARABLE_QUEUE_MISMATCH' -Message "macOS 来源 $($record.issue_number)/$($record.slot) 混入不同队列。"
    }
}
Assert-UniqueRuns -Records @($macosCandidates) -Platform 'macos'
if ($macosCandidates.Count -lt 5) {
    Stop-Budget -Code 'BUDGET_INSUFFICIENT_COMPARABLE_RUNS' -Message "macOS 队列只有 $($macosCandidates.Count) 个独立 Run。"
}

$document = [pscustomobject][ordered]@{
    schema_version = 'inputcodex.performance-budget-approval.v1'
    issue_number = 59
    status = 'approved-observation'
    budget_ci_enabled = $false
    gate_5_unlocked = $false
    formula = [pscustomobject][ordered]@{
        version = 'inputcodex.performance-budget-formula.v1'
        owner_preauthorization_ref = 'https://github.com/nonononull/inputcodex/issues/54#issuecomment-5081207478'
        owner_route_decision_ref = 'user-message:批准方案-A-EPYC-7763-四次固定串行复测-2026-07-26'
        center = 'median(run_level_values)'
        mad = 'median(abs(value - center))'
        warning_margin = 'max(3 * mad, 0.10 * center)'
        blocking_margin = 'max(5 * mad, 0.20 * center)'
        rounding = 'ceiling(value / quantum) * quantum'
        quantums = [pscustomobject][ordered]@{
            presentation_first_view_milliseconds = [decimal]1
            desktop_idle_working_set_mebibytes = [decimal]1
            rust_nanoseconds_per_operation = [decimal]0.001
        }
    }
    sources = [pscustomobject][ordered]@{
        historical_manifest_path = [System.IO.Path]::GetRelativePath($resolvedRoot, $historicalManifestFull).Replace('\', '/')
        historical_manifest_normalized_sha256 = Get-NormalizedTextHash -Path $historicalManifestFull
        current_manifest_path = [System.IO.Path]::GetRelativePath($resolvedRoot, $currentManifestFull).Replace('\', '/')
        current_manifest_normalized_sha256 = Get-NormalizedTextHash -Path $currentManifestFull
    }
    platforms = [pscustomobject][ordered]@{
        windows = New-PlatformBudget -Platform 'windows' -Records @($windowsRecords)
        macos = New-PlatformBudget -Platform 'macos' -Records @($macosCandidates)
    }
}

$outputFull = Resolve-ProjectPath -Root $resolvedRoot -Path $OutputPath
$outputParent = Split-Path -Parent $outputFull
if (-not (Test-Path -LiteralPath $outputParent -PathType Container)) {
    New-Item -ItemType Directory -Path $outputParent -Force | Out-Null
}
$outputJson = $document | ConvertTo-Json -Depth 100
[System.IO.File]::WriteAllText($outputFull, $outputJson + "`n", [System.Text.UTF8Encoding]::new($false))
Write-Output "BUDGET_APPROVAL_BUILT windows_runs=$($windowsRecords.Count) macos_runs=$($macosCandidates.Count) output=$outputFull"
