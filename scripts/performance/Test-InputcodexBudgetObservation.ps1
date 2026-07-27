[CmdletBinding()]
param(
    [Parameter(Mandatory)]
    [string]$RepositoryRoot
)

Set-StrictMode -Version Latest
$ErrorActionPreference = 'Stop'

$script:Passed = 0
$resolvedRoot = (Resolve-Path -LiteralPath $RepositoryRoot).Path
$observerScript = Join-Path $resolvedRoot 'scripts/performance/Invoke-InputcodexBudgetObservation.ps1'
$approvedBudgetPath = Join-Path $resolvedRoot 'benchmarks/budgets/issue-59-approved-observation.json'
$testRoot = Join-Path ([System.IO.Path]::GetTempPath()) "inputcodex-budget-observation-tests-$PID"

function Assert-Condition {
    param(
        [Parameter(Mandatory)][bool]$Condition,
        [Parameter(Mandatory)][string]$Message
    )

    if (-not $Condition) { throw $Message }
}

function Assert-Equal {
    param(
        [AllowNull()]$Expected,
        [AllowNull()]$Actual,
        [Parameter(Mandatory)][string]$Message
    )

    if ($Expected -ne $Actual) {
        throw "$Message；期望=$Expected，实际=$Actual"
    }
}

function Invoke-ContractTest {
    param(
        [Parameter(Mandatory)][string]$Name,
        [Parameter(Mandatory)][scriptblock]$Body
    )

    & $Body
    $script:Passed++
    Write-Host "PASS $Name"
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

function Write-LfJsonFile {
    param(
        [Parameter(Mandatory)][string]$Path,
        [Parameter(Mandatory)]$Value
    )

    $parent = Split-Path -Parent $Path
    if (-not (Test-Path -LiteralPath $parent)) {
        New-Item -ItemType Directory -Path $parent -Force | Out-Null
    }
    $json = ConvertTo-Json -InputObject $Value -Depth 100
    $normalized = ([regex]::Replace($json, '\r\n?', "`n")).TrimEnd("`n") + "`n"
    [System.IO.File]::WriteAllText($Path, $normalized, [System.Text.UTF8Encoding]::new($false))
}

function Copy-JsonValue {
    param([Parameter(Mandatory)]$Value)

    $Value | ConvertTo-Json -Depth 100 | ConvertFrom-Json -Depth 100
}

function Assert-SafeTestRoot {
    $resolvedTemp = [System.IO.Path]::GetFullPath([System.IO.Path]::GetTempPath())
    $resolvedTest = [System.IO.Path]::GetFullPath($testRoot)
    Assert-Condition -Condition ($resolvedTest.StartsWith($resolvedTemp, [System.StringComparison]::OrdinalIgnoreCase)) -Message "测试目录不在临时目录内：$resolvedTest"
}

function Invoke-Observer {
    param(
        [Parameter(Mandatory)][string]$BudgetPath,
        [Parameter(Mandatory)][string]$ResultPath,
        [Parameter(Mandatory)][string]$Platform,
        [string]$ExpectedBudgetSha256
    )

    $processInfo = [System.Diagnostics.ProcessStartInfo]::new()
    $processInfo.FileName = 'pwsh'
    $processInfo.UseShellExecute = $false
    $processInfo.RedirectStandardOutput = $true
    $processInfo.RedirectStandardError = $true
    foreach ($argument in @(
        '-NoProfile',
        '-File', $observerScript,
        '-RepositoryRoot', $resolvedRoot,
        '-BudgetPath', $BudgetPath,
        '-ResultPath', $ResultPath,
        '-Platform', $Platform
    )) {
        [void]$processInfo.ArgumentList.Add($argument)
    }
    if (-not [string]::IsNullOrWhiteSpace($ExpectedBudgetSha256)) {
        [void]$processInfo.ArgumentList.Add('-ExpectedBudgetSha256')
        [void]$processInfo.ArgumentList.Add($ExpectedBudgetSha256)
    }

    $process = [System.Diagnostics.Process]::new()
    $process.StartInfo = $processInfo
    [void]$process.Start()
    $stdout = $process.StandardOutput.ReadToEnd()
    $stderr = $process.StandardError.ReadToEnd()
    $process.WaitForExit()
    $jsonLine = @($stdout -split '\r?\n' | Where-Object { $_.TrimStart().StartsWith('{') } | Select-Object -Last 1)
    $json = if ($jsonLine.Count -eq 1) { $jsonLine[0] | ConvertFrom-Json -Depth 100 } else { $null }
    [pscustomobject]@{
        ExitCode = $process.ExitCode
        Output = ($stdout + $stderr).Trim()
        Json = $json
    }
}

function Set-ResultMetricValue {
    param(
        [Parameter(Mandatory)]$Result,
        [Parameter(Mandatory)][string]$Metric,
        [Parameter(Mandatory)][string]$Lane,
        [Parameter(Mandatory)][decimal]$Value
    )

    if ($Metric -eq 'presentation.first_view.elapsed') {
        $Result.presentation_first_view.elapsed_nanoseconds_statistics.$Lane = [decimal]$Value * 1000000
        return
    }
    if ($Metric -eq 'desktop.idle.working_set') {
        $Result.desktop_idle_resources.working_set_bytes_statistics.$Lane = [decimal]$Value * 1048576
        return
    }
    if ($Metric.StartsWith('rust.', [System.StringComparison]::Ordinal)) {
        $scenarioName = $Metric.Substring(5)
        $scenario = @($Result.rust_scenarios | Where-Object { $_.name -eq $scenarioName })
        Assert-Equal -Expected 1 -Actual $scenario.Count -Message "测试 fixture 缺少 Rust 场景 $scenarioName"
        $scenario[0].nanoseconds_per_operation_statistics.$Lane = [decimal]$Value
        return
    }
    throw "测试不支持的指标：$Metric"
}

function New-ComparableResult {
    param(
        [Parameter(Mandatory)][string]$Platform,
        [Parameter(Mandatory)]$Budget
    )

    $result = Get-Content -LiteralPath (Join-Path $resolvedRoot "benchmarks/results/issue-32/$Platform.json") -Raw -Encoding utf8 | ConvertFrom-Json -Depth 100
    foreach ($candidate in @($Budget.platforms.$Platform.blocking_candidates)) {
        Set-ResultMetricValue -Result $result -Metric ([string]$candidate.metric) -Lane ([string]$candidate.lane) -Value ([decimal]$candidate.warning_limit * [decimal]0.5)
    }
    $result
}

function Write-TestCase {
    param(
        [Parameter(Mandatory)][string]$Name,
        [Parameter(Mandatory)]$Budget,
        [Parameter(Mandatory)]$Result
    )

    $caseRoot = Join-Path $testRoot $Name
    $budgetPath = Join-Path $caseRoot 'budget.json'
    $resultPath = Join-Path $caseRoot 'result.json'
    Write-LfJsonFile -Path $budgetPath -Value $Budget
    Write-LfJsonFile -Path $resultPath -Value $Result
    [pscustomobject]@{
        BudgetPath = $budgetPath
        ResultPath = $resultPath
        BudgetHash = Get-NormalizedTextHash -Path $budgetPath
    }
}

Assert-SafeTestRoot
if (Test-Path -LiteralPath $testRoot) { Remove-Item -LiteralPath $testRoot -Recurse -Force }
New-Item -ItemType Directory -Path $testRoot -Force | Out-Null

try {
    Assert-Condition -Condition (Test-Path -LiteralPath $observerScript -PathType Leaf) -Message '预算观察器实现脚本不存在，应先完成 RED 再实现 GREEN。'
    Assert-Condition -Condition (Test-Path -LiteralPath $approvedBudgetPath -PathType Leaf) -Message '批准预算文件不存在。'
    $approvedBudgetHashBefore = Get-NormalizedTextHash -Path $approvedBudgetPath
    $approvedBudget = Get-Content -LiteralPath $approvedBudgetPath -Raw -Encoding utf8 | ConvertFrom-Json -Depth 100

    Invoke-ContractTest -Name 'within-budget 为非阻断分类' -Body {
        $case = Write-TestCase -Name 'within' -Budget (Copy-JsonValue $approvedBudget) -Result (New-ComparableResult -Platform 'windows' -Budget $approvedBudget)
        $run = Invoke-Observer -BudgetPath $case.BudgetPath -ResultPath $case.ResultPath -Platform 'windows' -ExpectedBudgetSha256 $case.BudgetHash
        Assert-Equal -Expected 0 -Actual $run.ExitCode -Message "within-budget 应成功，输出=$($run.Output)"
        Assert-Equal -Expected 'within-budget' -Actual $run.Json.classification -Message 'within-budget 分类错误'
        Assert-Equal -Expected 10 -Actual @($run.Json.observations).Count -Message '应输出十个候选观察'
    }

    Invoke-ContractTest -Name 'warning-observed 不阻断' -Body {
        $budget = Copy-JsonValue $approvedBudget
        $result = New-ComparableResult -Platform 'windows' -Budget $budget
        $candidate = $budget.platforms.windows.blocking_candidates[0]
        $value = ([decimal]$candidate.warning_limit + [decimal]$candidate.blocking_limit) / 2
        Set-ResultMetricValue -Result $result -Metric $candidate.metric -Lane $candidate.lane -Value $value
        $case = Write-TestCase -Name 'warning' -Budget $budget -Result $result
        $run = Invoke-Observer -BudgetPath $case.BudgetPath -ResultPath $case.ResultPath -Platform 'windows' -ExpectedBudgetSha256 $case.BudgetHash
        Assert-Equal -Expected 0 -Actual $run.ExitCode -Message "warning-observed 不得阻断，输出=$($run.Output)"
        Assert-Equal -Expected 'warning-observed' -Actual $run.Json.classification -Message 'warning 分类错误'
    }

    Invoke-ContractTest -Name 'blocking-observed 不阻断' -Body {
        $budget = Copy-JsonValue $approvedBudget
        $result = New-ComparableResult -Platform 'windows' -Budget $budget
        $candidate = $budget.platforms.windows.blocking_candidates[0]
        Set-ResultMetricValue -Result $result -Metric $candidate.metric -Lane $candidate.lane -Value ([decimal]$candidate.blocking_limit + [decimal]$candidate.quantum)
        $case = Write-TestCase -Name 'blocking' -Budget $budget -Result $result
        $run = Invoke-Observer -BudgetPath $case.BudgetPath -ResultPath $case.ResultPath -Platform 'windows' -ExpectedBudgetSha256 $case.BudgetHash
        Assert-Equal -Expected 0 -Actual $run.ExitCode -Message "blocking-observed 不得阻断，输出=$($run.Output)"
        Assert-Equal -Expected 'blocking-observed' -Actual $run.Json.classification -Message 'blocking 分类错误'
    }

    Invoke-ContractTest -Name '环境指纹不匹配报告 not-comparable' -Body {
        $budget = Copy-JsonValue $approvedBudget
        $result = New-ComparableResult -Platform 'windows' -Budget $budget
        $result.environment.processor = 'DIFFERENT HOSTED PROCESSOR'
        $case = Write-TestCase -Name 'not-comparable' -Budget $budget -Result $result
        $run = Invoke-Observer -BudgetPath $case.BudgetPath -ResultPath $case.ResultPath -Platform 'windows' -ExpectedBudgetSha256 $case.BudgetHash
        Assert-Equal -Expected 0 -Actual $run.ExitCode -Message "not-comparable 不得阻断，输出=$($run.Output)"
        Assert-Equal -Expected 'not-comparable' -Actual $run.Json.classification -Message 'not-comparable 分类错误'
        Assert-Equal -Expected $false -Actual ([bool]$run.Json.comparable) -Message '环境不匹配不得标为可比'
    }

    Invoke-ContractTest -Name '预算缺失必须失败' -Body {
        $result = New-ComparableResult -Platform 'windows' -Budget $approvedBudget
        $resultPath = Join-Path $testRoot 'missing-budget-result.json'
        Write-LfJsonFile -Path $resultPath -Value $result
        $run = Invoke-Observer -BudgetPath (Join-Path $testRoot 'missing-budget.json') -ResultPath $resultPath -Platform 'windows'
        Assert-Condition -Condition ($run.ExitCode -ne 0) -Message '预算缺失必须返回非零'
        Assert-Condition -Condition (@($run.Json.violations.code) -contains 'BUDGET_FILE_MISSING') -Message "预算缺失错误码不正确，输出=$($run.Output)"
    }

    Invoke-ContractTest -Name '预算哈希漂移必须失败' -Body {
        $budget = Copy-JsonValue $approvedBudget
        $budget.status = 'tampered'
        $case = Write-TestCase -Name 'budget-hash' -Budget $budget -Result (New-ComparableResult -Platform 'windows' -Budget $approvedBudget)
        $run = Invoke-Observer -BudgetPath $case.BudgetPath -ResultPath $case.ResultPath -Platform 'windows'
        Assert-Condition -Condition ($run.ExitCode -ne 0) -Message '预算哈希漂移必须失败'
        Assert-Condition -Condition (@($run.Json.violations.code) -contains 'BUDGET_HASH_MISMATCH') -Message "预算哈希错误码不正确，输出=$($run.Output)"
    }

    Invoke-ContractTest -Name '预算 schema 与 status 错误必须失败' -Body {
        $budget = Copy-JsonValue $approvedBudget
        $budget.schema_version = 'invalid'
        $budget.status = 'draft'
        $case = Write-TestCase -Name 'budget-schema' -Budget $budget -Result (New-ComparableResult -Platform 'windows' -Budget $approvedBudget)
        $run = Invoke-Observer -BudgetPath $case.BudgetPath -ResultPath $case.ResultPath -Platform 'windows' -ExpectedBudgetSha256 $case.BudgetHash
        Assert-Condition -Condition ($run.ExitCode -ne 0) -Message '预算 schema/status 错误必须失败'
        Assert-Condition -Condition (@($run.Json.violations.code) -contains 'BUDGET_SCHEMA_INVALID') -Message '缺少 BUDGET_SCHEMA_INVALID'
        Assert-Condition -Condition (@($run.Json.violations.code) -contains 'BUDGET_STATUS_INVALID') -Message '缺少 BUDGET_STATUS_INVALID'
    }

    Invoke-ContractTest -Name '平台不匹配必须失败' -Body {
        $case = Write-TestCase -Name 'platform' -Budget (Copy-JsonValue $approvedBudget) -Result (New-ComparableResult -Platform 'macos' -Budget $approvedBudget)
        $run = Invoke-Observer -BudgetPath $case.BudgetPath -ResultPath $case.ResultPath -Platform 'windows' -ExpectedBudgetSha256 $case.BudgetHash
        Assert-Condition -Condition ($run.ExitCode -ne 0) -Message '平台不匹配必须失败'
        Assert-Condition -Condition (@($run.Json.violations.code) -contains 'RESULT_PLATFORM_INVALID') -Message '缺少 RESULT_PLATFORM_INVALID'
    }

    Invoke-ContractTest -Name '单位错误和重复指标必须失败' -Body {
        $budget = Copy-JsonValue $approvedBudget
        $budget.platforms.windows.blocking_candidates[0].unit = 'seconds'
        $duplicate = Copy-JsonValue $budget.platforms.windows.blocking_candidates[1]
        $budget.platforms.windows.blocking_candidates = @($budget.platforms.windows.blocking_candidates) + $duplicate
        $case = Write-TestCase -Name 'unit-duplicate' -Budget $budget -Result (New-ComparableResult -Platform 'windows' -Budget $approvedBudget)
        $run = Invoke-Observer -BudgetPath $case.BudgetPath -ResultPath $case.ResultPath -Platform 'windows' -ExpectedBudgetSha256 $case.BudgetHash
        Assert-Condition -Condition ($run.ExitCode -ne 0) -Message '单位错误和重复指标必须失败'
        Assert-Condition -Condition (@($run.Json.violations.code) -contains 'BUDGET_UNIT_INVALID') -Message '缺少 BUDGET_UNIT_INVALID'
        Assert-Condition -Condition (@($run.Json.violations.code) -contains 'BUDGET_METRIC_DUPLICATED') -Message '缺少 BUDGET_METRIC_DUPLICATED'
    }

    Invoke-ContractTest -Name '指标缺失必须失败' -Body {
        $result = New-ComparableResult -Platform 'windows' -Budget $approvedBudget
        $result.rust_scenarios = @($result.rust_scenarios | Where-Object { $_.name -ne 'application-cancel-stale' })
        $case = Write-TestCase -Name 'metric-missing' -Budget (Copy-JsonValue $approvedBudget) -Result $result
        $run = Invoke-Observer -BudgetPath $case.BudgetPath -ResultPath $case.ResultPath -Platform 'windows' -ExpectedBudgetSha256 $case.BudgetHash
        Assert-Condition -Condition ($run.ExitCode -ne 0) -Message '指标缺失必须失败'
        Assert-Condition -Condition (@($run.Json.violations.code) -contains 'RESULT_METRIC_MISSING') -Message "缺少 RESULT_METRIC_MISSING，输出=$($run.Output)"
    }

    Invoke-ContractTest -Name '场景 checksum 错误必须失败' -Body {
        $result = New-ComparableResult -Platform 'windows' -Budget $approvedBudget
        $result.rust_scenarios[0].samples[0].checksum = 0
        $case = Write-TestCase -Name 'checksum' -Budget (Copy-JsonValue $approvedBudget) -Result $result
        $run = Invoke-Observer -BudgetPath $case.BudgetPath -ResultPath $case.ResultPath -Platform 'windows' -ExpectedBudgetSha256 $case.BudgetHash
        Assert-Condition -Condition ($run.ExitCode -ne 0) -Message 'checksum 错误必须失败'
        Assert-Condition -Condition (@($run.Json.violations.code) -contains 'RESULT_SCENARIO_CHECKSUM_INVALID') -Message '缺少 RESULT_SCENARIO_CHECKSUM_INVALID'
    }

    $approvedBudgetHashAfter = Get-NormalizedTextHash -Path $approvedBudgetPath
    Assert-Equal -Expected $approvedBudgetHashBefore -Actual $approvedBudgetHashAfter -Message '观察测试不得修改批准预算'
    Assert-Equal -Expected 'sha256:be07138908cd411925db963718b71062060f4fd4a50b910ab5d5f25f88d4ebe5' -Actual $approvedBudgetHashAfter -Message '批准预算哈希漂移'
    Write-Host "BUDGET_OBSERVATION_GREEN passed=$script:Passed"
}
finally {
    Assert-SafeTestRoot
    if (Test-Path -LiteralPath $testRoot) { Remove-Item -LiteralPath $testRoot -Recurse -Force }
}
