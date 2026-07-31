[CmdletBinding()]
param(
    [string]$RepositoryRoot = (Join-Path $PSScriptRoot '../..'),
    [string]$PolicyPath = '.github/autonomous-refactor-policy.json',
    [string]$SnapshotPath,
    [switch]$ReportOnly
)

Set-StrictMode -Version Latest
$ErrorActionPreference = 'Stop'

function Write-Result {
    param(
        [Parameter(Mandatory)]$Value,
        [Parameter(Mandatory)][int]$ExitCode
    )

    $Value | ConvertTo-Json -Depth 30 -Compress | Write-Output
    exit $ExitCode
}

function Get-PropertyValue {
    param(
        [AllowNull()]$Value,
        [Parameter(Mandatory)][string]$Name
    )

    if ($null -eq $Value) {
        return $null
    }
    $property = $Value.PSObject.Properties[$Name]
    if ($null -eq $property) {
        return $null
    }
    return $property.Value
}

function Invoke-GitRead {
    param([Parameter(Mandatory)][string[]]$Arguments)

    $output = @(& git -C $script:Root @Arguments 2>&1)
    if ($LASTEXITCODE -ne 0) {
        throw "git read failed: $($Arguments -join ' ')"
    }
    return @($output | ForEach-Object { $_.ToString() })
}

function Get-LiveSnapshot {
    $originMain = ((Invoke-GitRead @('rev-parse', 'origin/main')) -join '').Trim()
    $head = ((Invoke-GitRead @('rev-parse', 'HEAD')) -join '').Trim()
    $branch = ((Invoke-GitRead @('branch', '--show-current')) -join '').Trim()
    $expectedBase = ((Invoke-GitRead @('merge-base', 'HEAD', 'origin/main')) -join '').Trim()
    $worktreeClean = @((Invoke-GitRead @('status', '--porcelain=v1'))).Count -eq 0

    $sourceLockPath = Join-Path $script:Root 'upstream/source-lock.json'
    $releaseAudit = 'missing'
    if (Test-Path -LiteralPath $sourceLockPath -PathType Leaf) {
        try {
            $sourceLock = [System.IO.File]::ReadAllText($sourceLockPath, [Text.UTF8Encoding]::new($false)) |
                ConvertFrom-Json -Depth 100
            $releaseAudit = [string](Get-PropertyValue (Get-PropertyValue $sourceLock 'release_audit') 'status')
        } catch {
            $releaseAudit = 'invalid'
        }
    }

    $githubAvailable = $false
    $activeIssues = @()
    $activePrs = @()
    $ghCommand = Get-Command gh -ErrorAction SilentlyContinue
    if ($null -ne $ghCommand) {
        try {
            $issueOutput = @(& $ghCommand.Source issue list --repo nonononull/inputcodex --state open --limit 100 --json number,title,body,url 2>$null)
            if ($LASTEXITCODE -ne 0) { throw 'gh issue list failed' }
            $issues = @((($issueOutput -join [Environment]::NewLine) | ConvertFrom-Json -Depth 100))
            $activeIssues = @($issues |
                Where-Object {
                    [string](Get-PropertyValue $_ 'body') -match
                        'inputcodex:autonomous-refactor-(bootstrap|task):v1'
                } |
                ForEach-Object {
                    [pscustomobject][ordered]@{
                        number = Get-PropertyValue $_ 'number'
                        url = Get-PropertyValue $_ 'url'
                    }
                })

            $prOutput = @(& $ghCommand.Source pr list --repo nonononull/inputcodex --state open --limit 100 --json number,title,body,url,baseRefName,headRefOid,isDraft,mergeStateStatus 2>$null)
            if ($LASTEXITCODE -ne 0) { throw 'gh pr list failed' }
            $prs = @((($prOutput -join [Environment]::NewLine) | ConvertFrom-Json -Depth 100))
            $activePrs = @($prs |
                Where-Object {
                    [string](Get-PropertyValue $_ 'body') -match
                        'inputcodex:autonomous-refactor-pr:v1'
                } |
                ForEach-Object {
                    [pscustomobject][ordered]@{
                        number = Get-PropertyValue $_ 'number'
                        url = Get-PropertyValue $_ 'url'
                        base_ref = Get-PropertyValue $_ 'baseRefName'
                        head_oid = Get-PropertyValue $_ 'headRefOid'
                        is_draft = Get-PropertyValue $_ 'isDraft'
                        merge_state = Get-PropertyValue $_ 'mergeStateStatus'
                    }
                })
            $githubAvailable = $true
        } catch {
            $githubAvailable = $false
            $activeIssues = @()
            $activePrs = @()
        }
    }

    $paseoAvailable = $false
    $activeWriterCount = 0
    $paseoCommand = Get-Command paseo -ErrorAction SilentlyContinue
    if ($null -eq $paseoCommand) {
        $bundledPaseo = 'C:\Program Files\Paseo\resources\bin\paseo.cmd'
        if (Test-Path -LiteralPath $bundledPaseo -PathType Leaf) {
            $paseoExecutable = $bundledPaseo
        } else {
            $paseoExecutable = $null
        }
    } else {
        $paseoExecutable = $paseoCommand.Source
    }
    if ($null -ne $paseoExecutable) {
        try {
            $writerOutput = @(
                & $paseoExecutable ls --global --label 'project=inputcodex' --label 'role=writer' --json 2>$null
            )
            if ($LASTEXITCODE -ne 0) { throw 'paseo ls failed' }
            $writers = @((($writerOutput -join [Environment]::NewLine) | ConvertFrom-Json -Depth 100))
            $activeWriterCount = $writers.Count
            $paseoAvailable = $true
        } catch {
            $paseoAvailable = $false
            $activeWriterCount = 0
        }
    }

    return [pscustomobject][ordered]@{
        schema_version = 'inputcodex.autonomous-refactor-state-snapshot.v1'
        github_available = $githubAvailable
        paseo_available = $paseoAvailable
        release_audit = $releaseAudit
        observed_origin_main = $originMain
        expected_base = $expectedBase
        worktree_head = $head
        branch = $branch
        worktree_clean = $worktreeClean
        active_writer_count = [long]$activeWriterCount
        active_issues = @($activeIssues)
        active_prs = @($activePrs)
    }
}

$script:Root = [System.IO.Path]::GetFullPath($RepositoryRoot)
if (-not (Test-Path -LiteralPath $script:Root -PathType Container)) {
    Write-Result -ExitCode 10 -Value ([pscustomobject][ordered]@{
        schema_version = 1
        ok = $false
        error_code = 'AUTONOMOUS_STATE_REPOSITORY_MISSING'
    })
}

$resolvedPolicyPath = if ([System.IO.Path]::IsPathRooted($PolicyPath)) {
    [System.IO.Path]::GetFullPath($PolicyPath)
} else {
    [System.IO.Path]::GetFullPath((Join-Path $script:Root $PolicyPath))
}
$policyVerifier = Join-Path $script:Root 'scripts/ci/Verify-AutonomousRefactorPolicy.ps1'
$powerShellExecutable = (Get-Process -Id $PID).Path
$policyOutput = @(
    & $powerShellExecutable -NoLogo -NoProfile -File $policyVerifier -RepositoryRoot $script:Root -PolicyPath $resolvedPolicyPath 2>&1
)
$policyExitCode = $LASTEXITCODE
try {
    $policyResult = ($policyOutput -join [Environment]::NewLine) | ConvertFrom-Json -Depth 100
} catch {
    $policyResult = $null
}
if ($policyExitCode -ne 0 -or $null -eq $policyResult -or $policyResult.ok -ne $true) {
    Write-Result -ExitCode 12 -Value ([pscustomobject][ordered]@{
        schema_version = 1
        ok = $false
        error_code = 'AUTONOMOUS_STATE_POLICY_INVALID'
    })
}

$snapshotSource = 'live'
if (-not [string]::IsNullOrWhiteSpace($SnapshotPath)) {
    $resolvedSnapshotPath = if ([System.IO.Path]::IsPathRooted($SnapshotPath)) {
        [System.IO.Path]::GetFullPath($SnapshotPath)
    } else {
        [System.IO.Path]::GetFullPath((Join-Path $script:Root $SnapshotPath))
    }
    if (-not (Test-Path -LiteralPath $resolvedSnapshotPath -PathType Leaf)) {
        Write-Result -ExitCode 10 -Value ([pscustomobject][ordered]@{
            schema_version = 1
            ok = $false
            error_code = 'AUTONOMOUS_STATE_SNAPSHOT_MISSING'
        })
    }
    try {
        $snapshot = [System.IO.File]::ReadAllText($resolvedSnapshotPath, [Text.UTF8Encoding]::new($false)) |
            ConvertFrom-Json -Depth 100
        $snapshotSource = 'file'
    } catch {
        Write-Result -ExitCode 11 -Value ([pscustomobject][ordered]@{
            schema_version = 1
            ok = $false
            error_code = 'AUTONOMOUS_STATE_INVALID_SNAPSHOT'
        })
    }
} else {
    try {
        $snapshot = Get-LiveSnapshot
    } catch {
        Write-Result -ExitCode 11 -Value ([pscustomobject][ordered]@{
            schema_version = 1
            ok = $false
            error_code = 'AUTONOMOUS_STATE_LIVE_COLLECTION_FAILED'
        })
    }
}

$requiredProperties = @(
    'schema_version',
    'github_available',
    'paseo_available',
    'release_audit',
    'observed_origin_main',
    'expected_base',
    'worktree_head',
    'branch',
    'worktree_clean',
    'active_writer_count',
    'active_issues',
    'active_prs'
)
$missingProperties = @($requiredProperties | Where-Object { $null -eq $snapshot.PSObject.Properties[$_] })
$shaPattern = '^[0-9a-f]{40}$'
if ((Get-PropertyValue $snapshot 'schema_version') -cne 'inputcodex.autonomous-refactor-state-snapshot.v1' -or
    $missingProperties.Count -ne 0 -or
    [string](Get-PropertyValue $snapshot 'observed_origin_main') -cnotmatch $shaPattern -or
    [string](Get-PropertyValue $snapshot 'expected_base') -cnotmatch $shaPattern -or
    [string](Get-PropertyValue $snapshot 'worktree_head') -cnotmatch $shaPattern) {
    Write-Result -ExitCode 11 -Value ([pscustomobject][ordered]@{
        schema_version = 1
        ok = $false
        error_code = 'AUTONOMOUS_STATE_INVALID_SNAPSHOT'
    })
}

$activeIssues = @(Get-PropertyValue $snapshot 'active_issues')
$activePrs = @(Get-PropertyValue $snapshot 'active_prs')
$activeWriterCount = Get-PropertyValue $snapshot 'active_writer_count'
$reasonCodes = [System.Collections.Generic.List[string]]::new()

if ($activeWriterCount -isnot [long] -or $activeWriterCount -lt 0) {
    Write-Result -ExitCode 11 -Value ([pscustomobject][ordered]@{
        schema_version = 1
        ok = $false
        error_code = 'AUTONOMOUS_STATE_INVALID_SNAPSHOT'
    })
}
if ($activeWriterCount -gt 1) {
    $reasonCodes.Add('MULTIPLE_WRITERS') | Out-Null
}
if ($activeIssues.Count -gt 1) {
    $reasonCodes.Add('MULTIPLE_ACTIVE_ISSUES') | Out-Null
}
if ($activePrs.Count -gt 1) {
    $reasonCodes.Add('MULTIPLE_ACTIVE_PRS') | Out-Null
}
if ((Get-PropertyValue $snapshot 'release_audit') -cne 'current') {
    $reasonCodes.Add('RELEASE_AUDIT_STALE') | Out-Null
}
if ((Get-PropertyValue $snapshot 'expected_base') -cne (Get-PropertyValue $snapshot 'observed_origin_main')) {
    $reasonCodes.Add('ORIGIN_MAIN_DRIFT') | Out-Null
}
if ($activePrs.Count -eq 1) {
    if ((Get-PropertyValue $activePrs[0] 'base_ref') -cne 'main') {
        $reasonCodes.Add('PR_BASE_INVALID') | Out-Null
    }
    if ((Get-PropertyValue $activePrs[0] 'head_oid') -cne (Get-PropertyValue $snapshot 'worktree_head')) {
        $reasonCodes.Add('PR_HEAD_DRIFT') | Out-Null
    }
    if ($activeIssues.Count -eq 0) {
        $reasonCodes.Add('PR_WITHOUT_ISSUE') | Out-Null
    }
}
if ($activeIssues.Count -eq 0 -and (Get-PropertyValue $snapshot 'worktree_clean') -ne $true) {
    $reasonCodes.Add('ORPHANED_DIRTY_WORKTREE') | Out-Null
}

$externalReasons = [System.Collections.Generic.List[string]]::new()
if ((Get-PropertyValue $snapshot 'github_available') -ne $true) {
    $externalReasons.Add('GITHUB_UNAVAILABLE') | Out-Null
}
if ((Get-PropertyValue $snapshot 'paseo_available') -ne $true) {
    $externalReasons.Add('PASEO_UNAVAILABLE') | Out-Null
}

$hardStopReasons = @($reasonCodes)
if ($hardStopReasons.Count -ne 0) {
    $state = 'blocked-hard-stop'
    $nextAction = 'stop'
    $allReasons = $hardStopReasons
} elseif ($externalReasons.Count -ne 0) {
    $state = 'blocked-external-retry'
    $nextAction = 'retry-external'
    $allReasons = @($externalReasons)
} elseif ($activePrs.Count -eq 1) {
    $state = 'active-pr-review-ci'
    $nextAction = 'resume-pr'
    $allReasons = @()
} elseif ($activeIssues.Count -eq 1 -and (Get-PropertyValue $snapshot 'worktree_clean') -ne $true) {
    $state = 'active-worktree-execution'
    $nextAction = 'resume-worktree'
    $allReasons = @()
} elseif ($activeIssues.Count -eq 1) {
    $state = 'active-issue-planning'
    $nextAction = 'resume-issue'
    $allReasons = @()
} else {
    $state = 'idle-select-candidate'
    $nextAction = 'select-candidate'
    $allReasons = @()
}

Write-Result -ExitCode 0 -Value ([pscustomobject][ordered]@{
    schema_version = 1
    ok = $true
    report_only = [bool]$ReportOnly
    snapshot_source = $snapshotSource
    policy_sha256 = Get-PropertyValue $policyResult 'policy_sha256'
    state = $state
    next_action = $nextAction
    reason_codes = @($allReasons)
    active_issue = if ($activeIssues.Count -eq 1) { $activeIssues[0] } else { $null }
    active_pr = if ($activePrs.Count -eq 1) { $activePrs[0] } else { $null }
    expected_base = Get-PropertyValue $snapshot 'expected_base'
    observed_origin_main = Get-PropertyValue $snapshot 'observed_origin_main'
    observed_head = Get-PropertyValue $snapshot 'worktree_head'
    branch = Get-PropertyValue $snapshot 'branch'
    worktree_clean = Get-PropertyValue $snapshot 'worktree_clean'
    active_writer_count = $activeWriterCount
})
