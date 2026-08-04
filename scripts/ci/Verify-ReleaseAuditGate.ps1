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

    return ,$property.Value
}

function Test-ExactJsonString {
    param(
        [AllowNull()]$Actual,
        [AllowNull()]$Expected
    )

    return ($Actual -is [string] -and
        $Expected -is [string] -and
        [string]::Equals($Actual, $Expected, [StringComparison]::Ordinal))
}

function Test-JsonStringPattern {
    param(
        [AllowNull()]$Actual,
        [Parameter(Mandatory)][string]$Pattern
    )

    return ($Actual -is [string] -and $Actual -cmatch $Pattern)
}

function Read-StrictJsonDocument {
    param(
        [Parameter(Mandatory)]
        [string]$Path,

        [Parameter(Mandatory)]
        [string]$ErrorCode,

        [Parameter(Mandatory)]
        [string]$ErrorMessage,

        [Parameter(Mandatory)]
        [ValidateSet('object', 'array')]
        [string]$ExpectedRoot
    )

    if (-not (Test-Path -LiteralPath $Path -PathType Leaf)) {
        Add-ReleaseAuditError -Code $ErrorCode -Message $ErrorMessage
        return $null
    }

    try {
        $content = Get-Content -LiteralPath $Path -Raw -Encoding utf8
        $value = $content | ConvertFrom-Json -Depth 100 -NoEnumerate
        $rootValid = if ($ExpectedRoot -ceq 'object') {
            $value -is [System.Management.Automation.PSCustomObject]
        } else {
            $value -is [System.Array]
        }
        if (-not $rootValid) {
            Add-ReleaseAuditError -Code $ErrorCode -Message "$ErrorMessage：JSON 根必须是 $ExpectedRoot。"
            return $null
        }
        return ,$value
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

    $invalidState = [pscustomobject][ordered]@{
        status = 'invalid'
        requires_reaudit = $false
        valid = $false
    }
    if ($SourceLock -isnot [System.Management.Automation.PSCustomObject]) {
        Add-ReleaseAuditError -Code 'RELEASE_AUDIT_INVALID' -Message "$Location 的 JSON 根必须是 object。"
        return $invalidState
    }

    $snapshot = Get-ObjectPropertyValue -Object $SourceLock -Name 'snapshot'
    $audit = Get-ObjectPropertyValue -Object $SourceLock -Name 'release_audit'
    if ($snapshot -isnot [System.Management.Automation.PSCustomObject] -or
        $audit -isnot [System.Management.Automation.PSCustomObject]) {
        Add-ReleaseAuditError -Code 'RELEASE_AUDIT_INVALID' -Message "$Location 缺少 object 类型的 snapshot 或 release_audit。"
        return $invalidState
    }
    $snapshotTag = Get-ObjectPropertyValue -Object $snapshot -Name 'release_tag'
    $snapshotCommit = Get-ObjectPropertyValue -Object $snapshot -Name 'commit'
    $catalogRelease = Get-ObjectPropertyValue -Object $audit -Name 'catalog_release'
    if ($catalogRelease -isnot [System.Management.Automation.PSCustomObject]) {
        Add-ReleaseAuditError -Code 'RELEASE_AUDIT_INVALID' -Message "$Location 缺少 object 类型的 catalog_release。"
        return $invalidState
    }
    $catalogTag = Get-ObjectPropertyValue -Object $catalogRelease -Name 'tag'
    $catalogCommit = Get-ObjectPropertyValue -Object $catalogRelease -Name 'commit'
    $schemaVersion = Get-ObjectPropertyValue -Object $audit -Name 'schema_version'
    $status = Get-ObjectPropertyValue -Object $audit -Name 'status'
    $staleReason = Get-ObjectPropertyValue -Object $audit -Name 'stale_reason'
    $reAuditIssueRef = Get-ObjectPropertyValue -Object $audit -Name 're_audit_issue_ref'
    $valid = $true

    if ($snapshotTag -isnot [string] -or [string]::IsNullOrWhiteSpace($snapshotTag) -or
        -not (Test-JsonStringPattern -Actual $snapshotCommit -Pattern '^[0-9a-f]{40}$') -or
        $catalogTag -isnot [string] -or [string]::IsNullOrWhiteSpace($catalogTag) -or
        -not (Test-JsonStringPattern -Actual $catalogCommit -Pattern '^[0-9a-f]{40}$')) {
        $valid = $false
        Add-ReleaseAuditError -Code 'RELEASE_AUDIT_INVALID' -Message "$Location 缺少有效的快照或目录 Release。"
    }

    if (-not (Test-ExactJsonString -Actual $schemaVersion -Expected $releaseAuditSchema)) {
        $valid = $false
        Add-ReleaseAuditError -Code 'RELEASE_AUDIT_INVALID' -Message "$Location 的 schema_version 不受支持。"
    }

    $snapshotMatchesCatalog = (Test-ExactJsonString -Actual $snapshotTag -Expected $catalogTag) -and
        (Test-ExactJsonString -Actual $snapshotCommit -Expected $catalogCommit)
    if (Test-ExactJsonString -Actual $status -Expected $currentStatus) {
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

    if (Test-ExactJsonString -Actual $status -Expected $staleStatus) {
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
    $invalidState
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
    if ($Changes -isnot [System.Array]) {
        Add-ReleaseAuditError -Code 'RELEASE_AUDIT_INVALID_CHANGESET' -Message 'PR 变更集 JSON 根必须是 array。'
        return @($paths)
    }

    foreach ($change in $Changes) {
        if ($change -isnot [System.Management.Automation.PSCustomObject]) {
            Add-ReleaseAuditError -Code 'RELEASE_AUDIT_INVALID_CHANGESET' -Message '变更集元素必须是 object。'
            continue
        }
        $status = Get-ObjectPropertyValue -Object $change -Name 'status'
        $path = Get-ObjectPropertyValue -Object $change -Name 'path'
        $oldPath = Get-ObjectPropertyValue -Object $change -Name 'old_path'

        if ($status -isnot [string] -or @('A', 'M', 'D', 'R', 'C') -cnotcontains $status -or
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

function Test-SafeSnapshotRelativePath {
    param($Value)

    if ($Value -isnot [string] -or [string]::IsNullOrWhiteSpace($Value) -or
        $Value.StartsWith('/', [StringComparison]::Ordinal) -or
        $Value.Contains('\') -or
        $Value -match '^[A-Za-z]:' -or
        $Value.IndexOfAny([char[]]@(0..31)) -ge 0) {
        return $false
    }
    $segments = $Value.Split('/')
    return @($segments | Where-Object { $_ -in @('', '.', '..') }).Count -eq 0
}

function Invoke-GitText {
    param(
        [Parameter(Mandatory)][string]$RepositoryRoot,
        [Parameter(Mandatory)][string[]]$Arguments
    )

    $output = @(& git -C $RepositoryRoot @Arguments 2>&1 | ForEach-Object { $_.ToString() })
    if ($LASTEXITCODE -ne 0) {
        throw "git $($Arguments -join ' ') 失败：$($output -join [Environment]::NewLine)"
    }
    [string[]]$output
}

function Get-GitBlobBytes {
    param(
        [Parameter(Mandatory)][string]$RepositoryRoot,
        [Parameter(Mandatory)][string]$BlobOid
    )

    $startInfo = [Diagnostics.ProcessStartInfo]::new()
    $startInfo.FileName = 'git'
    $startInfo.UseShellExecute = $false
    $startInfo.RedirectStandardOutput = $true
    $startInfo.RedirectStandardError = $true
    foreach ($argument in @('-C', $RepositoryRoot, 'cat-file', 'blob', $BlobOid)) {
        $startInfo.ArgumentList.Add($argument)
    }

    $process = [Diagnostics.Process]::new()
    $process.StartInfo = $startInfo
    $buffer = [IO.MemoryStream]::new()
    try {
        if (-not $process.Start()) { throw '无法启动 git cat-file' }
        $stderrTask = $process.StandardError.ReadToEndAsync()
        $process.StandardOutput.BaseStream.CopyTo($buffer)
        $process.WaitForExit()
        $stderr = $stderrTask.GetAwaiter().GetResult()
        if ($process.ExitCode -ne 0) {
            throw "git cat-file 失败：$stderr"
        }
        return ,$buffer.ToArray()
    }
    finally {
        $buffer.Dispose()
        $process.Dispose()
    }
}

function Get-GitSnapshotProjection {
    param(
        [Parameter(Mandatory)][string]$RepositoryRoot,
        [Parameter(Mandatory)][string]$SnapshotPath
    )

    $treeOutput = @(Invoke-GitText -RepositoryRoot $RepositoryRoot -Arguments @('rev-parse', "HEAD:$SnapshotPath"))
    if ($treeOutput.Count -ne 1 -or $treeOutput[0] -cnotmatch '^[0-9a-f]{40}$') {
        throw '无法解析快照 Git tree'
    }

    $prefix = "$SnapshotPath/"
    $entries = [Collections.Generic.List[object]]::new()
    foreach ($line in @(Invoke-GitText -RepositoryRoot $RepositoryRoot -Arguments @(
        '-c', 'core.quotepath=false', 'ls-tree', '-r', '--full-tree', 'HEAD', '--', $SnapshotPath
    ))) {
        if ($line -notmatch '^(?<mode>[0-9]{6}) (?<type>[a-z]+) (?<oid>[0-9a-f]{40})\t(?<path>.+)$') {
            throw "无法解析快照 Git entry：$line"
        }
        $gitPath = $Matches.path
        if (-not $gitPath.StartsWith($prefix, [StringComparison]::Ordinal)) {
            throw "快照 Git entry 越界：$gitPath"
        }
        $entries.Add([pscustomobject][ordered]@{
            path = $gitPath.Substring($prefix.Length)
            mode = $Matches.mode
            type = $Matches.type
            git_blob_sha1 = $Matches.oid
        }) | Out-Null
    }
    [pscustomobject][ordered]@{
        tree_oid = $treeOutput[0]
        entries = $entries.ToArray()
    }
}

function Test-UpstreamSnapshotIntegrity {
    param(
        [Parameter(Mandatory)][string]$RepositoryRoot,
        $SourceLock
    )

    $problems = [Collections.Generic.List[string]]::new()
    $problemPaths = [Collections.Generic.List[string]]::new()
    function Add-IntegrityProblem {
        param([Parameter(Mandatory)][string]$Message, [string]$Path)
        $problems.Add($Message) | Out-Null
        if (-not [string]::IsNullOrWhiteSpace($Path) -and -not $problemPaths.Contains($Path)) {
            $problemPaths.Add($Path) | Out-Null
        }
    }

    if ($SourceLock -isnot [System.Management.Automation.PSCustomObject]) {
        Add-ReleaseAuditError -Code 'UPSTREAM_SNAPSHOT_INTEGRITY_INVALID' -Message 'source-lock JSON 根必须是 object。'
        return
    }
    $snapshot = Get-ObjectPropertyValue -Object $SourceLock -Name 'snapshot'
    $snapshotPath = Get-ObjectPropertyValue -Object $snapshot -Name 'path'
    if ($snapshot -isnot [System.Management.Automation.PSCustomObject] -or
        -not (Test-ExactJsonString `
            -Actual (Get-ObjectPropertyValue -Object $SourceLock -Name 'schema_version') `
            -Expected 'inputcodex.source-lock.v1') -or
        -not (Test-ExactJsonString -Actual $snapshotPath -Expected 'upstream/CodexPlusPlus')) {
        Add-IntegrityProblem 'source-lock schema 或 snapshot.path 漂移'
    }

    $filesProperty = if ($null -ne $SourceLock) { $SourceLock.PSObject.Properties['files'] } else { $null }
    if ($null -eq $filesProperty -or $filesProperty.Value -isnot [System.Array] -or $filesProperty.Value.Count -eq 0) {
        Add-IntegrityProblem 'source-lock files 必须为非空数组'
    }
    $files = if ($null -ne $filesProperty -and $filesProperty.Value -is [System.Array]) {
        @($filesProperty.Value)
    } else {
        @()
    }

    $expected = [Collections.Generic.Dictionary[string,object]]::new([StringComparer]::Ordinal)
    $previousPath = $null
    foreach ($file in $files) {
        if ($file -isnot [System.Management.Automation.PSCustomObject]) {
            Add-IntegrityProblem 'source-lock file 记录必须是 object'
            continue
        }
        $path = Get-ObjectPropertyValue -Object $file -Name 'path'
        $mode = Get-ObjectPropertyValue -Object $file -Name 'mode'
        $size = Get-ObjectPropertyValue -Object $file -Name 'size'
        $blob = Get-ObjectPropertyValue -Object $file -Name 'git_blob_sha1'
        $sha256 = Get-ObjectPropertyValue -Object $file -Name 'sha256'
        if (-not (Test-SafeSnapshotRelativePath $path) -or
            $mode -isnot [string] -or @('100644', '100755') -cnotcontains $mode -or
            $size -isnot [long] -or $size -lt 0 -or
            -not (Test-JsonStringPattern -Actual $blob -Pattern '^[0-9a-f]{40}$') -or
            -not (Test-JsonStringPattern -Actual $sha256 -Pattern '^[0-9a-f]{64}$')) {
            Add-IntegrityProblem 'source-lock file 记录 schema 无效' $(if ($path -is [string]) { $path } else { $null })
            continue
        }
        if ($null -ne $previousPath -and [StringComparer]::Ordinal.Compare($previousPath, $path) -ge 0) {
            Add-IntegrityProblem 'source-lock files 必须按 Ordinal 严格递增且无重复' $path
        }
        $previousPath = $path
        if ($expected.ContainsKey($path)) {
            Add-IntegrityProblem 'source-lock files 包含重复路径' $path
            continue
        }
        $expected.Add($path, $file)
    }

    $projection = $null
    try {
        $projection = Get-GitSnapshotProjection -RepositoryRoot $RepositoryRoot -SnapshotPath 'upstream/CodexPlusPlus'
    }
    catch {
        Add-IntegrityProblem "无法读取快照 Git 投影：$($_.Exception.Message)"
    }
    $actual = [Collections.Generic.Dictionary[string,object]]::new([StringComparer]::Ordinal)
    if ($null -ne $projection) {
        foreach ($entry in @($projection.entries)) {
            if (-not (Test-SafeSnapshotRelativePath $entry.path) -or $entry.type -cne 'blob' -or
                $entry.mode -notin @('100644', '100755') -or $actual.ContainsKey($entry.path)) {
                Add-IntegrityProblem 'Git 快照包含非法、非 blob 或重复 entry' $entry.path
                continue
            }
            $actual.Add($entry.path, $entry)
        }
    }

    foreach ($path in $expected.Keys) {
        if (-not $actual.ContainsKey($path)) { Add-IntegrityProblem 'Git 快照缺少 source-lock 路径' $path }
    }
    foreach ($path in $actual.Keys) {
        if (-not $expected.ContainsKey($path)) { Add-IntegrityProblem 'Git 快照包含 source-lock 外路径' $path }
    }

    $manifestBuilder = [Text.StringBuilder]::new()
    $totalBytes = 0L
    $largest = $null
    $directorySet = [Collections.Generic.HashSet[string]]::new([StringComparer]::Ordinal)
    foreach ($file in $files) {
        $path = Get-ObjectPropertyValue -Object $file -Name 'path'
        if ($path -isnot [string] -or -not $expected.ContainsKey($path) -or -not $actual.ContainsKey($path)) { continue }
        $entry = $actual[$path]
        if (-not (Test-ExactJsonString `
            -Actual (Get-ObjectPropertyValue -Object $file -Name 'mode') -Expected $entry.mode)) {
            Add-IntegrityProblem 'Git mode 与 source-lock 不一致' $path
        }
        if (-not (Test-ExactJsonString `
            -Actual (Get-ObjectPropertyValue -Object $file -Name 'git_blob_sha1') -Expected $entry.git_blob_sha1)) {
            Add-IntegrityProblem 'Git blob 与 source-lock 不一致' $path
        }
        try {
            [byte[]]$blobBytes = Get-GitBlobBytes -RepositoryRoot $RepositoryRoot -BlobOid $entry.git_blob_sha1
            $actualSize = [long]$blobBytes.Length
            $actualSha256 = [Convert]::ToHexString([Security.Cryptography.SHA256]::HashData($blobBytes)).ToLowerInvariant()
            if ($actualSize -ne (Get-ObjectPropertyValue -Object $file -Name 'size')) {
                Add-IntegrityProblem 'Git blob 字节数与 source-lock 不一致' $path
            }
            if (-not (Test-ExactJsonString `
                -Actual (Get-ObjectPropertyValue -Object $file -Name 'sha256') -Expected $actualSha256)) {
                Add-IntegrityProblem 'Git blob SHA-256 与 source-lock 不一致' $path
            }
            [void]$manifestBuilder.Append($actualSha256).Append('  ').Append($path).Append("`n")
            $totalBytes += $actualSize
            if ($null -eq $largest -or $actualSize -gt $largest.size -or
                ($actualSize -eq $largest.size -and [StringComparer]::Ordinal.Compare($path, $largest.path) -lt 0)) {
                $largest = [pscustomobject][ordered]@{
                    path = $path
                    mode = $entry.mode
                    size = $actualSize
                    git_blob_sha1 = $entry.git_blob_sha1
                    sha256 = $actualSha256
                }
            }
        }
        catch {
            Add-IntegrityProblem "无法读取 Git blob：$($_.Exception.Message)" $path
        }

        $segments = $path.Split('/')
        for ($depth = 1; $depth -lt $segments.Length; $depth += 1) {
            [void]$directorySet.Add([string]::Join('/', $segments[0..($depth - 1)]))
        }
    }

    $manifest = Get-ObjectPropertyValue -Object $SourceLock -Name 'manifest'
    $manifestHash = [Convert]::ToHexString(
        [Security.Cryptography.SHA256]::HashData([Text.UTF8Encoding]::new($false).GetBytes($manifestBuilder.ToString()))
    ).ToLowerInvariant()
    if ($manifest -isnot [System.Management.Automation.PSCustomObject] -or
        -not (Test-ExactJsonString `
            -Actual (Get-ObjectPropertyValue -Object $manifest -Name 'algorithm') -Expected 'sha256') -or
        -not (Test-ExactJsonString `
            -Actual (Get-ObjectPropertyValue -Object $manifest -Name 'format') `
            -Expected '<sha256><two spaces><posix path><newline>') -or
        -not (Test-ExactJsonString `
            -Actual (Get-ObjectPropertyValue -Object $manifest -Name 'sha256') -Expected $manifestHash) -or
        (Get-ObjectPropertyValue -Object $manifest -Name 'file_count') -isnot [long] -or
        (Get-ObjectPropertyValue -Object $manifest -Name 'file_count') -ne $files.Count -or
        (Get-ObjectPropertyValue -Object $manifest -Name 'total_bytes') -isnot [long] -or
        (Get-ObjectPropertyValue -Object $manifest -Name 'total_bytes') -ne $totalBytes) {
        Add-IntegrityProblem 'manifest hash、计数或总字节漂移'
    }
    $largestFile = Get-ObjectPropertyValue -Object $manifest -Name 'largest_file'
    if ($null -eq $largest -or
        $largestFile -isnot [System.Management.Automation.PSCustomObject] -or
        -not (Test-ExactJsonString `
            -Actual (Get-ObjectPropertyValue -Object $largestFile -Name 'path') -Expected $largest.path) -or
        -not (Test-ExactJsonString `
            -Actual (Get-ObjectPropertyValue -Object $largestFile -Name 'mode') -Expected $largest.mode) -or
        (Get-ObjectPropertyValue -Object $largestFile -Name 'size') -isnot [long] -or
        (Get-ObjectPropertyValue -Object $largestFile -Name 'size') -ne $largest.size -or
        -not (Test-ExactJsonString `
            -Actual (Get-ObjectPropertyValue -Object $largestFile -Name 'git_blob_sha1') `
            -Expected $largest.git_blob_sha1) -or
        -not (Test-ExactJsonString `
            -Actual (Get-ObjectPropertyValue -Object $largestFile -Name 'sha256') -Expected $largest.sha256)) {
        Add-IntegrityProblem 'manifest largest_file 漂移'
    }

    $tree = Get-ObjectPropertyValue -Object $SourceLock -Name 'tree'
    if ($null -eq $projection -or
        $tree -isnot [System.Management.Automation.PSCustomObject] -or
        $snapshot -isnot [System.Management.Automation.PSCustomObject] -or
        -not (Test-ExactJsonString `
            -Actual (Get-ObjectPropertyValue -Object $tree -Name 'sha') -Expected $projection.tree_oid) -or
        -not (Test-ExactJsonString `
            -Actual (Get-ObjectPropertyValue -Object $snapshot -Name 'commit_tree') -Expected $projection.tree_oid) -or
        (Get-ObjectPropertyValue -Object $tree -Name 'file_count') -isnot [long] -or
        (Get-ObjectPropertyValue -Object $tree -Name 'file_count') -ne $files.Count -or
        (Get-ObjectPropertyValue -Object $tree -Name 'directory_count') -isnot [long] -or
        (Get-ObjectPropertyValue -Object $tree -Name 'directory_count') -ne $directorySet.Count -or
        (Get-ObjectPropertyValue -Object $tree -Name 'entry_count') -isnot [long] -or
        (Get-ObjectPropertyValue -Object $tree -Name 'entry_count') -ne ($files.Count + $directorySet.Count) -or
        (Get-ObjectPropertyValue -Object $tree -Name 'submodule_count') -isnot [long] -or
        (Get-ObjectPropertyValue -Object $tree -Name 'submodule_count') -ne 0) {
        Add-IntegrityProblem 'Git tree 与 source-lock tree 统计漂移'
    }

    if ($problems.Count -ne 0) {
        Add-ReleaseAuditError `
            -Code 'UPSTREAM_SNAPSHOT_INTEGRITY_INVALID' `
            -Message ([string]::Join('；', $problems)) `
            -Paths $problemPaths.ToArray()
    }
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

$headSourceLock = Read-StrictJsonDocument `
    -Path (Join-Path $resolvedRepositoryRoot 'upstream/source-lock.json') `
    -ErrorCode 'RELEASE_AUDIT_INVALID' `
    -ErrorMessage '无法读取当前 source-lock' `
    -ExpectedRoot object
$headState = Get-ReleaseAuditState -SourceLock $headSourceLock -Location '当前 source-lock'
Test-UpstreamSnapshotIntegrity -RepositoryRoot $resolvedRepositoryRoot -SourceLock $headSourceLock

if (-not $hasInputFile) {
    Write-Result `
        -Status $headState.status `
        -RequiresReaudit $headState.requires_reaudit `
        -ReleaseAuditChanged $false
}

$baseSourceLock = Read-StrictJsonDocument `
    -Path $BaseSourceLockPath `
    -ErrorCode 'RELEASE_AUDIT_INVALID' `
    -ErrorMessage '无法读取基线 source-lock' `
    -ExpectedRoot object
$changes = Read-StrictJsonDocument `
    -Path $InputFile `
    -ErrorCode 'RELEASE_AUDIT_INVALID_CHANGESET' `
    -ErrorMessage '无法读取 PR 变更集' `
    -ExpectedRoot array
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
