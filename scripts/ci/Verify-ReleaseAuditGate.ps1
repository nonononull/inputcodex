[CmdletBinding()]
param(
    [string]$RepositoryRoot = '.',

    [string]$InputFile,

    [string]$BaseSourceLockPath
)

Set-StrictMode -Version Latest
$ErrorActionPreference = 'Stop'

$releaseAuditSchema = 'inputcodex.release-audit.v1'
$currentStatus = 'current'
$staleStatus = 'stale-re-audit-required'
$reAuditIssuePrefix = 'https://github.com/nonononull/inputcodex/issues/'
$errors = [System.Collections.Generic.List[object]]::new()

function Add-ReleaseAuditError {
    param(
        [Parameter(Mandatory)]
        [string]$Code,

        [Parameter(Mandatory)]
        [string]$Message,

        [string[]]$Paths = @()
    )

    $errors.Add([pscustomobject][ordered]@{
        code = $Code
        message = $Message
        paths = @($Paths | Sort-Object -Unique)
    })
}

function Get-ObjectPropertyValue {
    param(
        $Object,

        [Parameter(Mandatory)]
        [string]$Name
    )

    if ($null -eq $Object) {
        return $null
    }

    $property = $Object.PSObject.Properties[$Name]
    if ($null -eq $property) {
        return $null
    }

    $property.Value
}

function Read-JsonObject {
    param(
        [Parameter(Mandatory)]
        [string]$Path,

        [Parameter(Mandatory)]
        [string]$ErrorCode,

        [Parameter(Mandatory)]
        [string]$ErrorMessage
    )

    if (-not (Test-Path -LiteralPath $Path -PathType Leaf)) {
        Add-ReleaseAuditError -Code $ErrorCode -Message $ErrorMessage
        return $null
    }

    try {
        $content = Get-Content -LiteralPath $Path -Raw -Encoding utf8
        return $content | ConvertFrom-Json -Depth 100
    }
    catch {
        Add-ReleaseAuditError -Code $ErrorCode -Message "$ErrorMessage：$($_.Exception.Message)"
        return $null
    }
}

function Test-ValidReauditIssueRef {
    param(
        $Value
    )

    if ($Value -isnot [string]) {
        return $false
    }

    $issueNumber = $Value.Substring([Math]::Min($Value.Length, $reAuditIssuePrefix.Length))
    $Value.StartsWith($reAuditIssuePrefix, [System.StringComparison]::Ordinal) -and
        $issueNumber -match '^[1-9][0-9]*$'
}

function Get-ReleaseAuditState {
    param(
        $SourceLock,

        [Parameter(Mandatory)]
        [string]$Location
    )

    $snapshot = Get-ObjectPropertyValue -Object $SourceLock -Name 'snapshot'
    $audit = Get-ObjectPropertyValue -Object $SourceLock -Name 'release_audit'
    $snapshotTag = Get-ObjectPropertyValue -Object $snapshot -Name 'release_tag'
    $snapshotCommit = Get-ObjectPropertyValue -Object $snapshot -Name 'commit'
    $catalogRelease = Get-ObjectPropertyValue -Object $audit -Name 'catalog_release'
    $catalogTag = Get-ObjectPropertyValue -Object $catalogRelease -Name 'tag'
    $catalogCommit = Get-ObjectPropertyValue -Object $catalogRelease -Name 'commit'
    $schemaVersion = Get-ObjectPropertyValue -Object $audit -Name 'schema_version'
    $status = Get-ObjectPropertyValue -Object $audit -Name 'status'
    $staleReason = Get-ObjectPropertyValue -Object $audit -Name 'stale_reason'
    $reAuditIssueRef = Get-ObjectPropertyValue -Object $audit -Name 're_audit_issue_ref'
    $valid = $true

    if ($snapshotTag -isnot [string] -or [string]::IsNullOrWhiteSpace($snapshotTag) -or
        $snapshotCommit -isnot [string] -or [string]::IsNullOrWhiteSpace($snapshotCommit) -or
        $catalogTag -isnot [string] -or [string]::IsNullOrWhiteSpace($catalogTag) -or
        $catalogCommit -isnot [string] -or [string]::IsNullOrWhiteSpace($catalogCommit)) {
        $valid = $false
        Add-ReleaseAuditError -Code 'RELEASE_AUDIT_INVALID' -Message "$Location 缺少有效的快照或目录 Release。"
    }

    if ($schemaVersion -ne $releaseAuditSchema) {
        $valid = $false
        Add-ReleaseAuditError -Code 'RELEASE_AUDIT_INVALID' -Message "$Location 的 schema_version 不受支持。"
    }

    $snapshotMatchesCatalog = $snapshotTag -eq $catalogTag -and $snapshotCommit -eq $catalogCommit
    if ($status -eq $currentStatus) {
        if (-not $snapshotMatchesCatalog -or $null -ne $staleReason -or $null -ne $reAuditIssueRef) {
            $valid = $false
            Add-ReleaseAuditError -Code 'RELEASE_AUDIT_INVALID' -Message "$Location 的 current 状态必须与目录审计基线一致且没有 stale 说明。"
        }
        return [pscustomobject][ordered]@{
            status = $currentStatus
            requires_reaudit = $false
            valid = $valid
        }
    }

    if ($status -eq $staleStatus) {
        if ($snapshotMatchesCatalog) {
            $valid = $false
            Add-ReleaseAuditError -Code 'RELEASE_AUDIT_INVALID' -Message "$Location 的 stale 状态必须对应不同的快照与目录审计基线。"
        }
        if ($staleReason -isnot [string] -or [string]::IsNullOrWhiteSpace($staleReason)) {
            $valid = $false
            Add-ReleaseAuditError -Code 'RELEASE_AUDIT_INVALID' -Message "$Location 的 stale 状态必须包含重新审计根因。"
        }
        if (-not (Test-ValidReauditIssueRef -Value $reAuditIssueRef)) {
            $valid = $false
            Add-ReleaseAuditError -Code 'RELEASE_AUDIT_INVALID' -Message "$Location 的 stale 状态必须关联 inputcodex 重新审计 Issue。"
        }
        return [pscustomobject][ordered]@{
            status = $staleStatus
            requires_reaudit = $valid
            valid = $valid
        }
    }

    Add-ReleaseAuditError -Code 'RELEASE_AUDIT_INVALID' -Message "$Location 包含未知状态。"
    [pscustomobject][ordered]@{
        status = 'invalid'
        requires_reaudit = $false
        valid = $false
    }
}

function Get-ReleaseAuditFingerprint {
    param(
        $SourceLock
    )

    $audit = Get-ObjectPropertyValue -Object $SourceLock -Name 'release_audit'
    if ($null -eq $audit) {
        return $null
    }

    $audit | ConvertTo-Json -Depth 20 -Compress
}

function Get-ChangedPaths {
    param(
        $Changes
    )

    $paths = [System.Collections.Generic.List[string]]::new()
    if ($null -eq $Changes) {
        return @($paths)
    }

    foreach ($change in @($Changes)) {
        $status = Get-ObjectPropertyValue -Object $change -Name 'status'
        $path = Get-ObjectPropertyValue -Object $change -Name 'path'
        $oldPath = Get-ObjectPropertyValue -Object $change -Name 'old_path'

        if ($status -isnot [string] -or $status -notin @('A', 'M', 'D', 'R', 'C') -or
            $path -isnot [string] -or [string]::IsNullOrWhiteSpace($path)) {
            Add-ReleaseAuditError -Code 'RELEASE_AUDIT_INVALID_CHANGESET' -Message '变更集包含无效记录。'
            continue
        }

        $paths.Add($path.Replace('\', '/'))
        if ($null -ne $oldPath) {
            if ($oldPath -isnot [string] -or [string]::IsNullOrWhiteSpace($oldPath)) {
                Add-ReleaseAuditError -Code 'RELEASE_AUDIT_INVALID_CHANGESET' -Message '变更集包含无效 old_path。'
                continue
            }
            $paths.Add($oldPath.Replace('\', '/'))
        }
    }

    @($paths | Sort-Object -Unique)
}

function Test-BlockedProductPath {
    param(
        [Parameter(Mandatory)]
        [string]$Path
    )

    $Path -in @('Cargo.toml', 'Cargo.lock') -or
        $Path -like 'benchmarks/*' -or
        $Path -like 'apps/*' -or
        ($Path -like 'crates/*' -and $Path -notlike 'crates/inputcodex-parity/*')
}

function Write-Result {
    param(
        [Parameter(Mandatory)]
        [string]$Status,

        [Parameter(Mandatory)]
        [bool]$RequiresReaudit,

        [Parameter(Mandatory)]
        [bool]$ReleaseAuditChanged,

        [string[]]$BlockedPaths = @()
    )

    $result = [pscustomobject][ordered]@{
        schema_version = 1
        ok = $errors.Count -eq 0
        status = $Status
        requires_reaudit = $RequiresReaudit
        release_audit_changed = $ReleaseAuditChanged
        blocked_paths = @($BlockedPaths | Sort-Object -Unique)
        errors = @($errors.ToArray())
    }
    $result | ConvertTo-Json -Depth 20 -Compress | Write-Output
    exit $(if ($errors.Count -eq 0) { 0 } else { 2 })
}

$resolvedRepositoryRoot = [System.IO.Path]::GetFullPath($RepositoryRoot)
if (-not (Test-Path -LiteralPath $resolvedRepositoryRoot -PathType Container)) {
    Add-ReleaseAuditError -Code 'RELEASE_AUDIT_INVALID' -Message '仓库根目录不存在。'
    Write-Result -Status 'invalid' -RequiresReaudit $false -ReleaseAuditChanged $false
}

$hasInputFile = -not [string]::IsNullOrWhiteSpace($InputFile)
$hasBaseSourceLock = -not [string]::IsNullOrWhiteSpace($BaseSourceLockPath)
if ($hasInputFile -ne $hasBaseSourceLock) {
    Add-ReleaseAuditError -Code 'RELEASE_AUDIT_INVALID' -Message 'PR 门禁必须同时提供变更集和基线 source-lock。'
    Write-Result -Status 'invalid' -RequiresReaudit $false -ReleaseAuditChanged $false
}

$headSourceLock = Read-JsonObject `
    -Path (Join-Path $resolvedRepositoryRoot 'upstream/source-lock.json') `
    -ErrorCode 'RELEASE_AUDIT_INVALID' `
    -ErrorMessage '无法读取当前 source-lock'
$headState = Get-ReleaseAuditState -SourceLock $headSourceLock -Location '当前 source-lock'

if (-not $hasInputFile) {
    Write-Result `
        -Status $headState.status `
        -RequiresReaudit $headState.requires_reaudit `
        -ReleaseAuditChanged $false
}

$baseSourceLock = Read-JsonObject `
    -Path $BaseSourceLockPath `
    -ErrorCode 'RELEASE_AUDIT_INVALID' `
    -ErrorMessage '无法读取基线 source-lock'
$null = Get-ReleaseAuditState -SourceLock $baseSourceLock -Location '基线 source-lock'
$changes = Read-JsonObject `
    -Path $InputFile `
    -ErrorCode 'RELEASE_AUDIT_INVALID_CHANGESET' `
    -ErrorMessage '无法读取 PR 变更集'
$changedPaths = Get-ChangedPaths -Changes $changes
$blockedPaths = @($changedPaths | Where-Object { Test-BlockedProductPath -Path $_ })
$releaseAuditChanged = (Get-ReleaseAuditFingerprint -SourceLock $baseSourceLock) -ne
    (Get-ReleaseAuditFingerprint -SourceLock $headSourceLock)
$sourceLockChanged = $changedPaths -contains 'upstream/source-lock.json'

if ($blockedPaths.Count -gt 0 -and $releaseAuditChanged -and $sourceLockChanged) {
    Add-ReleaseAuditError `
        -Code 'RELEASE_AUDIT_CHANGED_WITH_BLOCKED_PATH' `
        -Message '同一 PR 不能同时更新 release_audit 与受阻产品路径。' `
        -Paths $blockedPaths
}
elseif ($blockedPaths.Count -gt 0 -and $headState.requires_reaudit) {
    Add-ReleaseAuditError `
        -Code 'RELEASE_AUDIT_REAUDIT_REQUIRED' `
        -Message '当前快照已 stale，完成目录重新审计前不得修改性能、预算或产品迁移路径。' `
        -Paths $blockedPaths
}

Write-Result `
    -Status $headState.status `
    -RequiresReaudit $headState.requires_reaudit `
    -ReleaseAuditChanged $releaseAuditChanged `
    -BlockedPaths $blockedPaths
