[CmdletBinding()]
param(
    [Parameter(Mandatory)]
    [string]$RepositoryRoot,

    [Parameter(Mandatory)]
    [ValidatePattern('^[0-9a-fA-F]{40}$')]
    [string]$Base,

    [Parameter(Mandatory)]
    [ValidatePattern('^[0-9a-fA-F]{40}$')]
    [string]$Head,

    [Parameter(Mandatory)]
    [string]$OutputFile
)

Set-StrictMode -Version Latest
$ErrorActionPreference = 'Stop'

function Invoke-GitDiff {
    param(
        [Parameter(Mandatory)]
        [string]$WorkingDirectory,

        [Parameter(Mandatory)]
        [string]$BaseCommit,

        [Parameter(Mandatory)]
        [string]$HeadCommit
    )

    $startInfo = [System.Diagnostics.ProcessStartInfo]::new()
    $startInfo.FileName = 'git'
    $startInfo.WorkingDirectory = $WorkingDirectory
    $startInfo.UseShellExecute = $false
    $startInfo.RedirectStandardOutput = $true
    $startInfo.RedirectStandardError = $true
    $startInfo.StandardOutputEncoding = [System.Text.UTF8Encoding]::new($false)
    $startInfo.StandardErrorEncoding = [System.Text.UTF8Encoding]::new($false)

    foreach ($argument in @(
        '-c',
        'core.quotepath=false',
        'diff',
        '--name-status',
        '-z',
        '--find-renames',
        '--find-copies',
        '--diff-filter=ACDMRT',
        $BaseCommit,
        $HeadCommit,
        '--'
    )) {
        $startInfo.ArgumentList.Add($argument)
    }

    $process = [System.Diagnostics.Process]::new()
    $process.StartInfo = $startInfo

    try {
        if (-not $process.Start()) {
            throw '无法启动 Git 变更收集进程。'
        }

        $standardOutputTask = $process.StandardOutput.ReadToEndAsync()
        $standardErrorTask = $process.StandardError.ReadToEndAsync()
        $process.WaitForExit()
        $standardOutput = $standardOutputTask.GetAwaiter().GetResult()
        $standardError = $standardErrorTask.GetAwaiter().GetResult()

        if ($process.ExitCode -ne 0) {
            throw "Git 变更收集失败，退出码=$($process.ExitCode)：$($standardError.Trim())"
        }

        $standardOutput
    }
    finally {
        $process.Dispose()
    }
}

function ConvertFrom-GitNameStatus {
    param(
        [Parameter(Mandatory)]
        [AllowEmptyString()]
        [string]$RawOutput
    )

    $changes = [System.Collections.Generic.List[object]]::new()
    if ($RawOutput.Length -eq 0) {
        return @($changes)
    }

    $tokens = $RawOutput.Split([char]0)
    $index = 0

    while ($index -lt $tokens.Count) {
        $statusToken = $tokens[$index]
        $index += 1

        if ($statusToken.Length -eq 0) {
            if ($index -eq $tokens.Count) {
                break
            }

            throw 'Git 变更输出包含空状态记录。'
        }

        if ($statusToken -notmatch '^(?<status>[ACDMRT])(?<score>[0-9]{1,3})?$') {
            throw "Git 变更输出包含未知状态：$statusToken"
        }

        $status = $Matches.status
        if ($status -eq 'T') {
            $status = 'M'
        }

        if ($Matches.status -in @('R', 'C')) {
            if (($index + 1) -ge $tokens.Count) {
                throw "Git $($Matches.status) 记录缺少新旧路径。"
            }

            $oldPath = $tokens[$index]
            $path = $tokens[$index + 1]
            $index += 2
            if ($oldPath.Length -eq 0 -or $path.Length -eq 0) {
                throw "Git $($Matches.status) 记录包含空路径。"
            }

            $changes.Add([pscustomobject][ordered]@{
                status = $status
                old_path = $oldPath
                path = $path
            })
            continue
        }

        if ($index -ge $tokens.Count) {
            throw "Git $status 记录缺少路径。"
        }

        $path = $tokens[$index]
        $index += 1
        if ($path.Length -eq 0) {
            throw "Git $status 记录包含空路径。"
        }

        $changes.Add([pscustomobject][ordered]@{
            status = $status
            path = $path
        })
    }

    @($changes)
}

$resolvedRepositoryRoot = [System.IO.Path]::GetFullPath($RepositoryRoot)
if (-not (Test-Path -LiteralPath $resolvedRepositoryRoot -PathType Container)) {
    throw "仓库根目录不存在：$resolvedRepositoryRoot"
}

$resolvedOutputFile = [System.IO.Path]::GetFullPath($OutputFile)
$outputDirectory = [System.IO.Path]::GetDirectoryName($resolvedOutputFile)
if ([string]::IsNullOrWhiteSpace($outputDirectory)) {
    throw "输出文件缺少父目录：$resolvedOutputFile"
}

if (-not (Test-Path -LiteralPath $outputDirectory -PathType Container)) {
    New-Item -ItemType Directory -Path $outputDirectory -Force | Out-Null
}

$rawOutput = Invoke-GitDiff -WorkingDirectory $resolvedRepositoryRoot -BaseCommit $Base -HeadCommit $Head
$changes = @(ConvertFrom-GitNameStatus -RawOutput $rawOutput)
$json = ConvertTo-Json -InputObject $changes -Depth 5
[System.IO.File]::WriteAllText(
    $resolvedOutputFile,
    "$json$([Environment]::NewLine)",
    [System.Text.UTF8Encoding]::new($false)
)
