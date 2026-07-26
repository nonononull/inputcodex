[CmdletBinding()]
param(
    [Parameter(Mandatory)]
    [string]$RepositoryRoot,

    [string]$BudgetPath = 'benchmarks/budgets/issue-59-approved-observation.json'
)

Set-StrictMode -Version Latest
$ErrorActionPreference = 'Stop'

$script:Passed = 0
$resolvedRoot = (Resolve-Path -LiteralPath $RepositoryRoot).Path
$builderPath = Join-Path $resolvedRoot 'scripts/performance/Build-InputcodexBudgetApproval.ps1'
$historicalManifestPath = Join-Path $resolvedRoot 'benchmarks/results/issue-54/manifest.json'
$currentManifestPath = Join-Path $resolvedRoot 'benchmarks/results/issue-59/manifest.json'
$resolvedBudgetPath = if ([System.IO.Path]::IsPathRooted($BudgetPath)) { $BudgetPath } else { Join-Path $resolvedRoot $BudgetPath }

function Write-Pass {
    param([Parameter(Mandatory)][string]$Name)

    $script:Passed++
    Write-Output "PASS $Name"
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

    [decimal]::Parse(
        [Convert]::ToString($Value, [System.Globalization.CultureInfo]::InvariantCulture),
        [System.Globalization.NumberStyles]::Float,
        [System.Globalization.CultureInfo]::InvariantCulture
    )
}

function Get-Median {
    param([Parameter(Mandatory)][object[]]$Values)

    $sorted = @($Values | ForEach-Object { Convert-ToDecimal -Value $_ } | Sort-Object)
    if ($sorted.Count -eq 0) {
        throw 'BUDGET_FORMULA_INVALID: 公式输入不能为空。'
    }
    $middle = [int][math]::Floor($sorted.Count / 2)
    if (($sorted.Count % 2) -eq 1) {
        return [decimal]$sorted[$middle]
    }
    ([decimal]$sorted[$middle - 1] + [decimal]$sorted[$middle]) / [decimal]2
}

function Assert-FormulaValues {
    param([Parameter(Mandatory)]$Candidate)

    $values = @($Candidate.source_values | ForEach-Object { Convert-ToDecimal -Value $_.value })
    $center = Get-Median -Values $values
    $deviations = @($values | ForEach-Object { [decimal]::Abs($_ - $center) })
    $mad = Get-Median -Values $deviations
    $minimum = [decimal]($values | Measure-Object -Minimum).Minimum
    $maximum = [decimal]($values | Measure-Object -Maximum).Maximum
    $warningMargin = [decimal]::Max([decimal]3 * $mad, [decimal]0.10 * $center)
    $blockingMargin = [decimal]::Max([decimal]5 * $mad, [decimal]0.20 * $center)
    $quantum = Convert-ToDecimal -Value $Candidate.quantum
    $warningLimit = [decimal]::Ceiling(($center + $warningMargin) / $quantum) * $quantum
    $blockingLimit = [decimal]::Ceiling(($center + $blockingMargin) / $quantum) * $quantum
    $expected = @($minimum, $maximum, $center, $mad, $warningMargin, $blockingMargin, $warningLimit, $blockingLimit)
    $actual = @(
        $Candidate.minimum,
        $Candidate.maximum,
        $Candidate.center,
        $Candidate.mad,
        $Candidate.warning_margin,
        $Candidate.blocking_margin,
        $Candidate.warning_limit,
        $Candidate.blocking_limit
    ) | ForEach-Object { Convert-ToDecimal -Value $_ }
    for ($index = 0; $index -lt $expected.Count; $index++) {
        $delta = [decimal]::Abs([decimal]$expected[$index] - [decimal]$actual[$index])
        $quantumTolerance = $quantum / [decimal]1000000
        $relativeTolerance = [decimal]::Abs([decimal]$expected[$index]) * [decimal]0.000000000000001
        $tolerance = [decimal]::Max($quantumTolerance, $relativeTolerance)
        if ($delta -gt $tolerance) {
            throw "BUDGET_FORMULA_INVALID: $($Candidate.metric)/$($Candidate.lane) 的统计或限值无法独立复算。"
        }
    }
}

function Read-JsonFile {
    param([Parameter(Mandatory)][string]$Path)

    Get-Content -LiteralPath $Path -Raw -Encoding utf8 | ConvertFrom-Json -Depth 100
}

function Write-JsonFile {
    param(
        [Parameter(Mandatory)][string]$Path,
        [Parameter(Mandatory)]$Value
    )

    $parent = Split-Path -Parent $Path
    if (-not (Test-Path -LiteralPath $parent -PathType Container)) {
        New-Item -ItemType Directory -Path $parent -Force | Out-Null
    }
    $json = $Value | ConvertTo-Json -Depth 100
    [System.IO.File]::WriteAllText($Path, $json + "`n", [System.Text.UTF8Encoding]::new($false))
}

function Invoke-Builder {
    param(
        [Parameter(Mandatory)][string]$Root,
        [Parameter(Mandatory)][string]$HistoricalManifest,
        [Parameter(Mandatory)][string]$CurrentManifest,
        [Parameter(Mandatory)][string]$OutputPath
    )

    & $builderPath `
        -RepositoryRoot $Root `
        -HistoricalManifestPath $HistoricalManifest `
        -CurrentManifestPath $CurrentManifest `
        -OutputPath $OutputPath | Out-Null
}

function Assert-ExpectedFailure {
    param(
        [Parameter(Mandatory)][string]$Name,
        [Parameter(Mandatory)][string]$Code,
        [Parameter(Mandatory)][scriptblock]$Action
    )

    try {
        & $Action
    }
    catch {
        if ($_.Exception.Message -notmatch [regex]::Escape($Code)) {
            throw "TEST_UNEXPECTED_FAILURE: $Name 期望 $Code，实际为 $($_.Exception.Message)"
        }
        Write-Pass -Name $Name
        return
    }
    throw "TEST_EXPECTED_FAILURE_NOT_RAISED: $Name 未触发 $Code"
}

function Assert-BudgetDocument {
    param(
        [Parameter(Mandatory)][string]$ExpectedPath,
        [Parameter(Mandatory)][string]$ActualPath
    )

    $actual = Read-JsonFile -Path $ActualPath
    if ([bool]$actual.budget_ci_enabled) {
        throw 'BUDGET_CI_FORBIDDEN: 本任务不得启用预算 CI。'
    }
    if ([bool]$actual.gate_5_unlocked) {
        throw 'BUDGET_GATE5_FORBIDDEN: 本任务不得解锁 Gate 5。'
    }
    if ((Get-NormalizedTextHash -Path $ExpectedPath) -ne (Get-NormalizedTextHash -Path $ActualPath)) {
        throw 'BUDGET_DOCUMENT_MISMATCH: 预算文档与离线复算结果不一致。'
    }
}

function New-EvidenceFixture {
    param(
        [Parameter(Mandatory)][string]$BasePath,
        [Parameter(Mandatory)][string]$Name
    )

    $fixtureRoot = Join-Path $BasePath $Name
    $destination = Join-Path $fixtureRoot 'benchmarks/results'
    New-Item -ItemType Directory -Path $destination -Force | Out-Null
    Copy-Item -LiteralPath (Join-Path $resolvedRoot 'benchmarks/results/issue-54') -Destination $destination -Recurse
    Copy-Item -LiteralPath (Join-Path $resolvedRoot 'benchmarks/results/issue-59') -Destination $destination -Recurse
    $fixtureRoot
}

if (-not (Test-Path -LiteralPath $builderPath -PathType Leaf)) {
    throw 'BUDGET_BUILDER_MISSING: 缺少离线预算构建器。'
}
if (-not (Test-Path -LiteralPath $resolvedBudgetPath -PathType Leaf)) {
    throw 'BUDGET_DOCUMENT_MISSING: 缺少已入库预算 JSON。'
}

$tempRoot = Join-Path ([System.IO.Path]::GetTempPath()) ('inputcodex-budget-approval-' + [guid]::NewGuid().ToString('N'))
New-Item -ItemType Directory -Path $tempRoot | Out-Null
try {
    $expectedPath = Join-Path $tempRoot 'expected.json'
    Invoke-Builder -Root $resolvedRoot -HistoricalManifest $historicalManifestPath -CurrentManifest $currentManifestPath -OutputPath $expectedPath
    Assert-BudgetDocument -ExpectedPath $expectedPath -ActualPath $resolvedBudgetPath
    Write-Pass -Name '真实证据可确定性复算预算 JSON'

    $budget = Read-JsonFile -Path $expectedPath
    if ($budget.schema_version -ne 'inputcodex.performance-budget-approval.v1' -or [int]$budget.issue_number -ne 59 -or $budget.status -ne 'approved-observation') {
        throw 'BUDGET_DOCUMENT_IDENTITY_INVALID: 预算身份字段错误。'
    }
    if (@($budget.platforms.windows.source_runs).Count -ne 5 -or @($budget.platforms.macos.source_runs).Count -ne 12) {
        throw 'BUDGET_SOURCE_COUNT_INVALID: Windows/macOS 来源数必须为 5/12。'
    }
    foreach ($platform in @('windows', 'macos')) {
        $platformBudget = $budget.platforms.$platform
        if (@($platformBudget.source_runs.run_id | Sort-Object -Unique).Count -ne @($platformBudget.source_runs).Count) {
            throw "BUDGET_DUPLICATE_RUN_ID: $platform 存在重复 run_id。"
        }
        if (@($platformBudget.blocking_candidates).Count -ne 10) {
            throw "BUDGET_CANDIDATE_COUNT_INVALID: $platform 阻断候选必须为 10 条。"
        }
        if (@($platformBudget.observations).Count -ne 3 -or @($platformBudget.diagnostics).Count -ne 3) {
            throw "BUDGET_NONBLOCKING_COUNT_INVALID: $platform 观察/诊断必须为 3/3。"
        }
        foreach ($candidate in @($platformBudget.blocking_candidates)) {
            Assert-FormulaValues -Candidate $candidate
        }
        $forbidden = @($platformBudget.blocking_candidates.metric | Where-Object { $_ -match 'cpu|binary|build' })
        if ($forbidden.Count -ne 0) {
            throw "BUDGET_SCOPE_INVALID: $platform 把观察或诊断指标误列为阻断候选。"
        }
    }
    Write-Pass -Name '队列来源、候选指标与非阻断分区正确'
    Write-Pass -Name '统计、MAD、安全裕量和量子舍入可独立复算'

    $fixture = New-EvidenceFixture -BasePath $tempRoot -Name 'insufficient'
    $currentPath = Join-Path $fixture 'benchmarks/results/issue-59/manifest.json'
    $current = Read-JsonFile -Path $currentPath
    $current.runs = @($current.runs | Where-Object { $_.slot -ne 'run-04' })
    Write-JsonFile -Path $currentPath -Value $current
    Assert-ExpectedFailure -Name '不足五个可比样本被拒绝' -Code 'BUDGET_INSUFFICIENT_COMPARABLE_RUNS' -Action {
        Invoke-Builder -Root $fixture -HistoricalManifest (Join-Path $fixture 'benchmarks/results/issue-54/manifest.json') -CurrentManifest $currentPath -OutputPath (Join-Path $fixture 'out.json')
    }

    $fixture = New-EvidenceFixture -BasePath $tempRoot -Name 'duplicate'
    $currentPath = Join-Path $fixture 'benchmarks/results/issue-59/manifest.json'
    $current = Read-JsonFile -Path $currentPath
    $current.historical_evidence.windows_target_candidate_slots = @('run-03', 'run-05', 'run-03')
    Write-JsonFile -Path $currentPath -Value $current
    Assert-ExpectedFailure -Name '重复 run 被拒绝' -Code 'BUDGET_DUPLICATE_RUN_ID' -Action {
        Invoke-Builder -Root $fixture -HistoricalManifest (Join-Path $fixture 'benchmarks/results/issue-54/manifest.json') -CurrentManifest $currentPath -OutputPath (Join-Path $fixture 'out.json')
    }

    $fixture = New-EvidenceFixture -BasePath $tempRoot -Name 'mixed-cohort'
    $currentPath = Join-Path $fixture 'benchmarks/results/issue-59/manifest.json'
    $current = Read-JsonFile -Path $currentPath
    (@($current.runs | Where-Object { $_.slot -eq 'run-03' })[0]).classification.windows.value = 'comparable-valid'
    Write-JsonFile -Path $currentPath -Value $current
    Assert-ExpectedFailure -Name '混合环境队列被拒绝' -Code 'BUDGET_COMPARABLE_QUEUE_MISMATCH' -Action {
        Invoke-Builder -Root $fixture -HistoricalManifest (Join-Path $fixture 'benchmarks/results/issue-54/manifest.json') -CurrentManifest $currentPath -OutputPath (Join-Path $fixture 'out.json')
    }

    $fixture = New-EvidenceFixture -BasePath $tempRoot -Name 'checksum'
    $currentPath = Join-Path $fixture 'benchmarks/results/issue-59/manifest.json'
    $current = Read-JsonFile -Path $currentPath
    $run = @($current.runs | Where-Object { $_.slot -eq 'run-04' })[0]
    $resultPath = Join-Path $fixture $run.results.windows.path
    $result = Read-JsonFile -Path $resultPath
    $changedChecksum = ([uint64]$result.rust_scenarios[0].samples[0].checksum + 1).ToString()
    $result.rust_scenarios[0].samples[0].checksum = $changedChecksum
    Write-JsonFile -Path $resultPath -Value $result
    $run.results.windows.normalized_sha256 = Get-NormalizedTextHash -Path $resultPath
    $run.results.windows.sample_integrity.rust_scenarios[0].checksum = $changedChecksum
    Write-JsonFile -Path $currentPath -Value $current
    Assert-ExpectedFailure -Name 'checksum 不一致被拒绝' -Code 'BUDGET_CHECKSUM_INCONSISTENT' -Action {
        Invoke-Builder -Root $fixture -HistoricalManifest (Join-Path $fixture 'benchmarks/results/issue-54/manifest.json') -CurrentManifest $currentPath -OutputPath (Join-Path $fixture 'out.json')
    }

    $tampered = Read-JsonFile -Path $expectedPath
    $tampered.platforms.windows.blocking_candidates[0].warning_limit = [decimal]$tampered.platforms.windows.blocking_candidates[0].warning_limit + [decimal]$tampered.platforms.windows.blocking_candidates[0].quantum
    $tamperedPath = Join-Path $tempRoot 'tampered-formula.json'
    Write-JsonFile -Path $tamperedPath -Value $tampered
    Assert-ExpectedFailure -Name 'warning/blocking 公式漂移被拒绝' -Code 'BUDGET_FORMULA_INVALID' -Action {
        $tamperedBudget = Read-JsonFile -Path $tamperedPath
        Assert-FormulaValues -Candidate $tamperedBudget.platforms.windows.blocking_candidates[0]
    }

    $tampered = Read-JsonFile -Path $expectedPath
    $changed = $false
    foreach ($candidate in @($tampered.platforms.windows.blocking_candidates)) {
        foreach ($sourceValue in @($candidate.source_values)) {
            if (@($sourceValue.outlier_indices).Count -gt 0) {
                $sourceValue.outlier_indices = @()
                $changed = $true
                break
            }
        }
        if ($changed) { break }
    }
    if (-not $changed) { throw 'TEST_FIXTURE_INVALID: 未找到可验证的 IQR 标记。' }
    $tamperedPath = Join-Path $tempRoot 'tampered-iqr.json'
    Write-JsonFile -Path $tamperedPath -Value $tampered
    Assert-ExpectedFailure -Name 'IQR 标记丢失被拒绝' -Code 'BUDGET_DOCUMENT_MISMATCH' -Action {
        Assert-BudgetDocument -ExpectedPath $expectedPath -ActualPath $tamperedPath
    }

    $tampered = Read-JsonFile -Path $expectedPath
    $tampered.budget_ci_enabled = $true
    $tamperedPath = Join-Path $tempRoot 'tampered-ci.json'
    Write-JsonFile -Path $tamperedPath -Value $tampered
    Assert-ExpectedFailure -Name '预算 CI 越权被拒绝' -Code 'BUDGET_CI_FORBIDDEN' -Action {
        Assert-BudgetDocument -ExpectedPath $expectedPath -ActualPath $tamperedPath
    }

    Write-Output "BUDGET_APPROVAL_GREEN passed=$script:Passed"
}
finally {
    if (Test-Path -LiteralPath $tempRoot) {
        $resolvedTemp = (Resolve-Path -LiteralPath $tempRoot).Path
        $systemTemp = [System.IO.Path]::GetFullPath([System.IO.Path]::GetTempPath())
        if (-not $resolvedTemp.StartsWith($systemTemp, [System.StringComparison]::OrdinalIgnoreCase)) {
            throw "TEST_TEMP_PATH_INVALID: 临时目录越界：$resolvedTemp"
        }
        Remove-Item -LiteralPath $resolvedTemp -Recurse -Force
    }
}
