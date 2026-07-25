[CmdletBinding()]
param(
    [Parameter(Mandatory)][string]$RepositoryRoot,
    [Parameter(Mandatory)][ValidateSet('windows', 'macos')][string]$Platform,
    [Parameter(Mandatory)][string]$OutputPath
)

Set-StrictMode -Version Latest
$ErrorActionPreference = 'Stop'

function Write-LfTextFile {
    param([Parameter(Mandatory)][string]$Path, [Parameter(Mandatory)][string]$Text)

    $parent = Split-Path -Parent $Path
    if (-not [string]::IsNullOrWhiteSpace($parent)) {
        New-Item -ItemType Directory -Path $parent -Force | Out-Null
    }
    $normalized = [regex]::Replace($Text, '\r\n?', "`n")
    [System.IO.File]::WriteAllText($Path, $normalized, [System.Text.UTF8Encoding]::new($false))
}

function Write-LfJsonFile {
    param([Parameter(Mandatory)][string]$Path, [Parameter(Mandatory)]$Value)

    Write-LfTextFile -Path $Path -Text ((ConvertTo-Json -InputObject $Value -Depth 100) + "`n")
}

function Invoke-LoggedNative {
    param(
        [Parameter(Mandatory)][string]$FilePath,
        [Parameter(Mandatory)][string[]]$Arguments,
        [Parameter(Mandatory)][string]$LogPath
    )

    $stopwatch = [System.Diagnostics.Stopwatch]::StartNew()
    $output = @(& $FilePath @Arguments 2>&1 | ForEach-Object { $_.ToString() })
    $exitCode = $LASTEXITCODE
    $stopwatch.Stop()
    Write-LfTextFile -Path $LogPath -Text (($output -join "`n") + "`n")
    if ($exitCode -ne 0) {
        throw "$FilePath 执行失败，退出码=$exitCode，日志=$LogPath"
    }

    [pscustomobject]@{
        duration_milliseconds = [Math]::Round($stopwatch.Elapsed.TotalMilliseconds, 3)
        output = $output
    }
}

function Get-Percentile {
    param(
        [Parameter(Mandatory)][double[]]$Values,
        [Parameter(Mandatory)][ValidateRange(0, 1)][double]$Percentile
    )

    $sorted = @($Values | Sort-Object)
    if ($sorted.Count -eq 1) { return $sorted[0] }
    $position = ($sorted.Count - 1) * $Percentile
    $lowerIndex = [Math]::Floor($position)
    $upperIndex = [Math]::Ceiling($position)
    if ($lowerIndex -eq $upperIndex) { return $sorted[$lowerIndex] }
    $weight = $position - $lowerIndex
    $sorted[$lowerIndex] + (($sorted[$upperIndex] - $sorted[$lowerIndex]) * $weight)
}

function Get-SampleStatistics {
    param([Parameter(Mandatory)][object[]]$Values)

    $numbers = @($Values | ForEach-Object { [double]$_ })
    if ($numbers.Count -eq 0) { throw '无法为空样本计算统计值。' }
    $q1 = Get-Percentile -Values $numbers -Percentile 0.25
    $q3 = Get-Percentile -Values $numbers -Percentile 0.75
    $iqr = $q3 - $q1
    $lowerBound = $q1 - (1.5 * $iqr)
    $upperBound = $q3 + (1.5 * $iqr)
    $outlierIndices = [System.Collections.Generic.List[int]]::new()
    for ($index = 0; $index -lt $numbers.Count; $index++) {
        if ($numbers[$index] -lt $lowerBound -or $numbers[$index] -gt $upperBound) {
            $outlierIndices.Add($index)
        }
    }

    [pscustomobject][ordered]@{
        minimum = ($numbers | Measure-Object -Minimum).Minimum
        median = Get-Percentile -Values $numbers -Percentile 0.5
        p50 = Get-Percentile -Values $numbers -Percentile 0.5
        p95 = Get-Percentile -Values $numbers -Percentile 0.95
        maximum = ($numbers | Measure-Object -Maximum).Maximum
        q1 = $q1
        q3 = $q3
        iqr = $iqr
        lower_outlier_bound = $lowerBound
        upper_outlier_bound = $upperBound
        outlier_indices = @($outlierIndices)
    }
}

function Read-TextLines {
    param([Parameter(Mandatory)][string]$Path)

    if (-not (Test-Path -LiteralPath $Path -PathType Leaf)) { return @() }
    @(Get-Content -LiteralPath $Path -Encoding utf8 -ErrorAction SilentlyContinue)
}

function Stop-ProbeProcess {
    param([AllowNull()][System.Diagnostics.Process]$Process)

    if ($null -eq $Process) { return }
    try {
        $Process.Refresh()
        if (-not $Process.HasExited) {
            Stop-Process -Id $Process.Id -Force -ErrorAction SilentlyContinue
            Wait-Process -Id $Process.Id -Timeout 5 -ErrorAction SilentlyContinue
        }
    }
    catch {
    }
}

function Start-DesktopProbe {
    param(
        [Parameter(Mandatory)][string]$BinaryPath,
        [Parameter(Mandatory)][string]$WorkingDirectory,
        [Parameter(Mandatory)][string]$AttemptDirectory,
        [Parameter(Mandatory)][string]$ReadyMarker,
        [Parameter(Mandatory)][int]$TimeoutSeconds,
        [switch]$KeepAlive
    )

    New-Item -ItemType Directory -Path $AttemptDirectory -Force | Out-Null
    $stdoutPath = Join-Path $AttemptDirectory 'stdout.log'
    $stderrPath = Join-Path $AttemptDirectory 'stderr.log'
    $process = $null
    $stopwatch = [System.Diagnostics.Stopwatch]::StartNew()
    $status = 'error'
    $errorMessage = $null
    $elapsedNanoseconds = $null
    $exitCode = $null
    $previousProbeValue = [Environment]::GetEnvironmentVariable('INPUTCODEX_PERFORMANCE_PROBE', 'Process')

    try {
        [Environment]::SetEnvironmentVariable('INPUTCODEX_PERFORMANCE_PROBE', '1', 'Process')
        $startParameters = @{
            FilePath = $BinaryPath
            WorkingDirectory = $WorkingDirectory
            PassThru = $true
            RedirectStandardOutput = $stdoutPath
            RedirectStandardError = $stderrPath
        }
        if ($IsWindows) { $startParameters.WindowStyle = 'Hidden' }
        $process = Start-Process @startParameters
    }
    catch {
        $errorMessage = $_.Exception.Message
    }
    finally {
        [Environment]::SetEnvironmentVariable('INPUTCODEX_PERFORMANCE_PROBE', $previousProbeValue, 'Process')
    }

    if ($null -ne $process) {
        while ($stopwatch.Elapsed.TotalSeconds -lt $TimeoutSeconds) {
            $stdoutText = if (Test-Path -LiteralPath $stdoutPath -PathType Leaf) {
                Get-Content -LiteralPath $stdoutPath -Raw -Encoding utf8 -ErrorAction SilentlyContinue
            } else { '' }
            if ($null -eq $stdoutText) { $stdoutText = '' }
            if ($stdoutText.Contains($ReadyMarker)) {
                $elapsedNanoseconds = [long][Math]::Round($stopwatch.ElapsedTicks * (1000000000.0 / [System.Diagnostics.Stopwatch]::Frequency))
                $status = 'success'
                break
            }
            $process.Refresh()
            if ($process.HasExited) {
                $status = 'process-exited'
                $exitCode = $process.ExitCode
                break
            }
            Start-Sleep -Milliseconds 25
        }
        if ($status -eq 'error') { $status = 'timeout' }
    }

    $stopwatch.Stop()
    if (-not $KeepAlive -or $status -ne 'success') {
        Stop-ProbeProcess -Process $process
        $process = $null
    }

    [pscustomobject]@{
        record = [pscustomobject][ordered]@{
            status = $status
            elapsed_nanoseconds = $elapsedNanoseconds
            exit_code = $exitCode
            error = $errorMessage
            stdout_lines = @(Read-TextLines -Path $stdoutPath)
            stderr_lines = @(Read-TextLines -Path $stderrPath)
        }
        process = $process
    }
}

function Invoke-RustScenarioSample {
    param(
        [Parameter(Mandatory)][string]$BinaryPath,
        [Parameter(Mandatory)][string]$Scenario,
        [Parameter(Mandatory)][string]$Root,
        [Parameter(Mandatory)][uint64]$Iterations,
        [Parameter(Mandatory)][int]$Index
    )

    $lines = @(& $BinaryPath $Scenario $Root $Iterations 2>&1 | ForEach-Object { $_.ToString() })
    $exitCode = $LASTEXITCODE
    if ($exitCode -ne 0) {
        throw "Rust 场景 $Scenario 第 $Index 次运行失败，退出码=$exitCode，输出=$($lines -join ' | ')"
    }
    $csv = @($lines | Where-Object { -not [string]::IsNullOrWhiteSpace($_) })[-1]
    $fields = $csv.Split(',')
    if ($fields.Count -ne 5 -or $fields[0] -ne $Scenario) {
        throw "Rust 场景 $Scenario 返回无效 CSV：$csv"
    }

    [pscustomobject][ordered]@{
        index = $Index
        iterations = [uint64]$fields[1]
        total_nanoseconds = [decimal]$fields[2]
        nanoseconds_per_operation = [decimal]$fields[3]
        checksum = [uint64]$fields[4]
    }
}

function Get-ProcessorName {
    if ($IsWindows) {
        return [string](Get-CimInstance Win32_Processor | Select-Object -First 1 -ExpandProperty Name)
    }
    $name = @(& sysctl -n machdep.cpu.brand_string 2>$null | ForEach-Object { $_.ToString() }) -join ' '
    if ([string]::IsNullOrWhiteSpace($name)) {
        $name = @(& sysctl -n hw.model 2>$null | ForEach-Object { $_.ToString() }) -join ' '
    }
    $name
}

function Get-TotalMemoryBytes {
    if ($IsWindows) {
        return [uint64](Get-CimInstance Win32_ComputerSystem).TotalPhysicalMemory
    }
    [uint64](@(& sysctl -n hw.memsize 2>$null)[0])
}

$resolvedRoot = $null
$diagnosticDirectory = $null
try {
    if ($env:GITHUB_ACTIONS -ne 'true') { throw '完整性能基线只能在 GitHub Actions 中运行。' }
    if ($env:RUNNER_ENVIRONMENT -ne 'github-hosted') { throw '完整性能基线只能在标准 github-hosted runner 上运行。' }
    if (($Platform -eq 'windows' -and -not $IsWindows) -or ($Platform -eq 'macos' -and -not $IsMacOS)) {
        throw "请求平台 $Platform 与当前 runner 不一致。"
    }
    if (-not (Test-Path -LiteralPath $RepositoryRoot -PathType Container)) { throw '仓库根目录不存在。' }

    $resolvedRoot = (Resolve-Path -LiteralPath $RepositoryRoot).Path
    $resolvedOutputPath = [System.IO.Path]::GetFullPath($OutputPath)
    if ($resolvedOutputPath -match '[\\/]target[\\/]') { throw '性能结果禁止写入 target 目录。' }
    $outputDirectory = Split-Path -Parent $resolvedOutputPath
    $reportDirectory = Split-Path -Parent $outputDirectory
    $diagnosticDirectory = Join-Path $reportDirectory 'diagnostics'
    New-Item -ItemType Directory -Path $outputDirectory -Force | Out-Null
    New-Item -ItemType Directory -Path $diagnosticDirectory -Force | Out-Null

    $powerShellExecutable = (Get-Process -Id $PID).Path
    $contractScript = Join-Path $resolvedRoot 'scripts/performance/Test-InputcodexBaseline.ps1'
    $contractLines = @(& $powerShellExecutable -NoLogo -NoProfile -File $contractScript -RepositoryRoot $resolvedRoot -Mode Contract 2>&1 | ForEach-Object { $_.ToString() })
    $contractExitCode = $LASTEXITCODE
    Write-LfTextFile -Path (Join-Path $diagnosticDirectory 'contract.json') -Text (($contractLines -join "`n") + "`n")
    if ($contractExitCode -ne 0) { throw "性能合同验证失败：$($contractLines -join ' | ')" }
    $contract = $contractLines[-1] | ConvertFrom-Json -Depth 100

    $configPath = Join-Path $resolvedRoot 'benchmarks/config/issue-32-baseline.json'
    $config = Get-Content -LiteralPath $configPath -Raw -Encoding utf8 | ConvertFrom-Json -Depth 100
    $commit = (@(& git -C $resolvedRoot rev-parse HEAD 2>&1))[0].ToString().Trim()
    $tree = (@(& git -C $resolvedRoot rev-parse 'HEAD^{tree}' 2>&1))[0].ToString().Trim()
    if ($commit -notmatch '^[0-9a-f]{40}$' -or $tree -notmatch '^[0-9a-f]{40}$') { throw '无法读取精确 Git commit/tree。' }

    $desktopBuild = Invoke-LoggedNative -FilePath 'cargo' -Arguments @('build', '--release', '--locked', '-p', 'inputcodex-desktop') -LogPath (Join-Path $diagnosticDirectory 'desktop-build.log')
    $baselineBuild = Invoke-LoggedNative -FilePath 'cargo' -Arguments @('build', '--release', '--locked', '--manifest-path', 'benchmarks/inputcodex-baseline/Cargo.toml') -LogPath (Join-Path $diagnosticDirectory 'baseline-build.log')

    $desktopBinaryPath = if ($Platform -eq 'windows') { Join-Path $resolvedRoot 'target/release/inputcodex-desktop.exe' } else { Join-Path $resolvedRoot 'target/release/inputcodex-desktop' }
    $baselineBinaryPath = if ($Platform -eq 'windows') { Join-Path $resolvedRoot 'benchmarks/inputcodex-baseline/target/release/inputcodex-baseline.exe' } else { Join-Path $resolvedRoot 'benchmarks/inputcodex-baseline/target/release/inputcodex-baseline' }
    foreach ($binaryPath in @($desktopBinaryPath, $baselineBinaryPath)) {
        if (-not (Test-Path -LiteralPath $binaryPath -PathType Leaf)) { throw "Release 二进制不存在：$binaryPath" }
    }

    $firstViewAttempts = [System.Collections.Generic.List[object]]::new()
    $successCount = 0
    for ($attemptIndex = 1; $attemptIndex -le [int]$config.probe.maximum_attempts; $attemptIndex++) {
        if ($successCount -ge [int]$config.probe.minimum_successful_samples) { break }
        $attempt = Start-DesktopProbe `
            -BinaryPath $desktopBinaryPath `
            -WorkingDirectory $resolvedRoot `
            -AttemptDirectory (Join-Path $diagnosticDirectory ("first-view-{0}" -f $attemptIndex)) `
            -ReadyMarker $config.probe.ready_marker `
            -TimeoutSeconds ([int]$config.probe.timeout_seconds)
        $record = $attempt.record
        $record | Add-Member -NotePropertyName attempt -NotePropertyValue $attemptIndex
        $firstViewAttempts.Add($record)
        if ($record.status -eq 'success') { $successCount += 1 }
    }
    if ($successCount -lt [int]$config.probe.minimum_successful_samples) {
        throw "首次 view 成功样本不足：$successCount/$($config.probe.minimum_successful_samples)"
    }
    $firstViewSuccessNanoseconds = @($firstViewAttempts | Where-Object { $_.status -eq 'success' } | ForEach-Object { $_.elapsed_nanoseconds })

    $idleStartup = Start-DesktopProbe `
        -BinaryPath $desktopBinaryPath `
        -WorkingDirectory $resolvedRoot `
        -AttemptDirectory (Join-Path $diagnosticDirectory 'idle-startup') `
        -ReadyMarker $config.probe.ready_marker `
        -TimeoutSeconds ([int]$config.probe.timeout_seconds) `
        -KeepAlive
    if ($idleStartup.record.status -ne 'success' -or $null -eq $idleStartup.process) {
        throw "空闲资源进程未成功到达首次 view：$($idleStartup.record.status)"
    }

    $idleSamples = [System.Collections.Generic.List[object]]::new()
    $idleProcess = $idleStartup.process
    try {
        Start-Sleep -Seconds ([int]$config.idle_resources.settle_seconds)
        $previousProcess = Get-Process -Id $idleProcess.Id -ErrorAction Stop
        $previousCpuMilliseconds = $previousProcess.TotalProcessorTime.TotalMilliseconds
        $previousTimestamp = [System.Diagnostics.Stopwatch]::GetTimestamp()
        $idleStopwatch = [System.Diagnostics.Stopwatch]::StartNew()

        for ($sampleIndex = 1; $sampleIndex -le [int]$config.idle_resources.sample_count; $sampleIndex++) {
            Start-Sleep -Seconds ([int]$config.idle_resources.sample_interval_seconds)
            $currentProcess = Get-Process -Id $idleProcess.Id -ErrorAction Stop
            $currentTimestamp = [System.Diagnostics.Stopwatch]::GetTimestamp()
            $currentCpuMilliseconds = $currentProcess.TotalProcessorTime.TotalMilliseconds
            $elapsedMilliseconds = ($currentTimestamp - $previousTimestamp) * 1000.0 / [System.Diagnostics.Stopwatch]::Frequency
            $cpuPercent = if ($elapsedMilliseconds -gt 0) {
                [Math]::Max(0, (($currentCpuMilliseconds - $previousCpuMilliseconds) / $elapsedMilliseconds) * 100.0 / [Environment]::ProcessorCount)
            } else { 0 }

            $idleSamples.Add([pscustomobject][ordered]@{
                index = $sampleIndex
                elapsed_since_ready_milliseconds = [Math]::Round($idleStopwatch.Elapsed.TotalMilliseconds, 3)
                working_set_bytes = [long]$currentProcess.WorkingSet64
                cpu_percent = [Math]::Round($cpuPercent, 6)
                total_cpu_milliseconds = [Math]::Round($currentCpuMilliseconds, 3)
            })
            $previousCpuMilliseconds = $currentCpuMilliseconds
            $previousTimestamp = $currentTimestamp
        }
        $idleStopwatch.Stop()
    }
    finally {
        Stop-ProbeProcess -Process $idleProcess
    }

    $scenarioResults = [System.Collections.Generic.List[object]]::new()
    foreach ($scenarioConfig in @($config.rust_scenarios)) {
        $warmups = [System.Collections.Generic.List[object]]::new()
        for ($index = 1; $index -le [int]$scenarioConfig.warmup_runs; $index++) {
            $warmups.Add((Invoke-RustScenarioSample -BinaryPath $baselineBinaryPath -Scenario $scenarioConfig.name -Root $resolvedRoot -Iterations ([uint64]$scenarioConfig.iterations) -Index $index))
        }
        $samples = [System.Collections.Generic.List[object]]::new()
        for ($index = 1; $index -le [int]$scenarioConfig.sample_count; $index++) {
            $samples.Add((Invoke-RustScenarioSample -BinaryPath $baselineBinaryPath -Scenario $scenarioConfig.name -Root $resolvedRoot -Iterations ([uint64]$scenarioConfig.iterations) -Index $index))
        }
        $scenarioResults.Add([pscustomobject][ordered]@{
            name = $scenarioConfig.name
            warmups = @($warmups)
            samples = @($samples)
            nanoseconds_per_operation_statistics = Get-SampleStatistics -Values @($samples | ForEach-Object { $_.nanoseconds_per_operation })
        })
    }

    $rustcOutput = @(& rustc -Vv 2>&1 | ForEach-Object { $_.ToString() }) -join "`n"
    $cargoOutput = @(& cargo -V 2>&1 | ForEach-Object { $_.ToString() }) -join "`n"
    $result = [pscustomobject][ordered]@{
        schema_version = 'inputcodex.performance-result.v1'
        issue_number = 32
        platform = $Platform
        status = 'complete'
        source = [pscustomobject][ordered]@{
            commit = $commit
            tree = $tree
            config_sha256 = $contract.config_sha256
            implementation_sha256 = $contract.implementation_sha256
            input_sha256 = $contract.input_sha256
        }
        github = [pscustomobject][ordered]@{
            run_id = [string]$env:GITHUB_RUN_ID
            run_attempt = [int]$env:GITHUB_RUN_ATTEMPT
            workflow = [string]$env:GITHUB_WORKFLOW
            event_name = [string]$env:GITHUB_EVENT_NAME
            ref = [string]$env:GITHUB_REF
            head_ref = [string]$env:GITHUB_HEAD_REF
        }
        environment = [pscustomobject][ordered]@{
            runner_os = [string]$env:RUNNER_OS
            runner_arch = [string]$env:RUNNER_ARCH
            runner_name = [string]$env:RUNNER_NAME
            runner_environment = [string]$env:RUNNER_ENVIRONMENT
            image_os = [string]$env:ImageOS
            image_version = [string]$env:ImageVersion
            os_description = [System.Runtime.InteropServices.RuntimeInformation]::OSDescription
            processor = Get-ProcessorName
            logical_processor_count = [Environment]::ProcessorCount
            total_memory_bytes = Get-TotalMemoryBytes
            rustc = $rustcOutput
            cargo = $cargoOutput
            powershell = $PSVersionTable.PSVersion.ToString()
        }
        build = [pscustomobject][ordered]@{
            desktop_release = [pscustomobject][ordered]@{
                duration_milliseconds = $desktopBuild.duration_milliseconds
                binary_name = [System.IO.Path]::GetFileName($desktopBinaryPath)
                binary_size_bytes = (Get-Item -LiteralPath $desktopBinaryPath).Length
            }
            baseline_release = [pscustomobject][ordered]@{
                duration_milliseconds = $baselineBuild.duration_milliseconds
                binary_name = [System.IO.Path]::GetFileName($baselineBinaryPath)
                binary_size_bytes = (Get-Item -LiteralPath $baselineBinaryPath).Length
            }
        }
        presentation_first_view = [pscustomobject][ordered]@{
            minimum_successful_samples = [int]$config.probe.minimum_successful_samples
            maximum_attempts = [int]$config.probe.maximum_attempts
            timeout_seconds = [int]$config.probe.timeout_seconds
            attempts = @($firstViewAttempts)
            elapsed_nanoseconds_statistics = Get-SampleStatistics -Values $firstViewSuccessNanoseconds
        }
        desktop_idle_resources = [pscustomobject][ordered]@{
            startup = $idleStartup.record
            settle_seconds = [int]$config.idle_resources.settle_seconds
            sample_interval_seconds = [int]$config.idle_resources.sample_interval_seconds
            samples = @($idleSamples)
            working_set_bytes_statistics = Get-SampleStatistics -Values @($idleSamples | ForEach-Object { $_.working_set_bytes })
            cpu_percent_statistics = Get-SampleStatistics -Values @($idleSamples | ForEach-Object { $_.cpu_percent })
        }
        rust_scenarios = @($scenarioResults)
        failed_first_view_attempts = @($firstViewAttempts | Where-Object { $_.status -ne 'success' })
    }

    Write-LfJsonFile -Path $resolvedOutputPath -Value $result
    Write-Host "INPUTCODEX_PERFORMANCE_RESULT_WRITTEN platform=$Platform path=$resolvedOutputPath"
}
catch {
    if ($null -ne $diagnosticDirectory) {
        Write-LfJsonFile -Path (Join-Path $diagnosticDirectory 'failure.json') -Value ([pscustomobject][ordered]@{
            schema_version = 'inputcodex.performance-failure.v1'
            platform = $Platform
            message = $_.Exception.Message
            script_stack = $_.ScriptStackTrace
        })
    }
    throw
}
