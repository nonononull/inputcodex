[CmdletBinding()]
param(
    [string]$RepositoryRoot = '.',

    [string]$InputFile,

    [string]$BaseSourceLockPath,

    [ValidateSet('none', 'live', 'fixture')]
    [string]$RemoteValidationMode = 'none',

    [string]$RemoteEvidencePath,

    [string]$ArchivePath,

    [string]$TemporaryDirectory
)

Set-StrictMode -Version Latest
$ErrorActionPreference = 'Stop'

$releaseAuditSchema = 'inputcodex.release-audit.v1'
$currentStatus = 'current'
$staleStatus = 'stale-re-audit-required'
$reAuditIssuePrefix = 'https://github.com/nonononull/inputcodex/issues/'
$sourceLockSchemaVersion = 'inputcodex.source-lock.v1'
$upstreamRepository = 'BigPizzaV3/CodexPlusPlus'
$upstreamRepositoryUrl = 'https://github.com/BigPizzaV3/CodexPlusPlus'
$maximumArchiveBytes = 64L * 1024L * 1024L
$remoteEvidenceSchemaVersion = 'inputcodex.release-audit-remote-evidence.v1'
$errors = [System.Collections.Generic.List[object]]::new()

$jsonStringSchema = [ordered]@{ Kind = 'string' }
$jsonInt64Schema = [ordered]@{ Kind = 'int64' }
$jsonBooleanSchema = [ordered]@{ Kind = 'boolean' }
$jsonNullableStringSchema = [ordered]@{ Kind = 'nullable-string' }
$sourceFileSchema = [ordered]@{
    Kind = 'object'
    Fields = [ordered]@{
        path = $jsonStringSchema
        mode = $jsonStringSchema
        size = $jsonInt64Schema
        git_blob_sha1 = $jsonStringSchema
        sha256 = $jsonStringSchema
    }
}
$preservedLicenseFileSchema = [ordered]@{
    Kind = 'object'
    Fields = [ordered]@{
        path = $jsonStringSchema
        kind = $jsonStringSchema
        size = $jsonInt64Schema
        sha256 = $jsonStringSchema
    }
}
$generatorToolSchema = [ordered]@{
    Kind = 'object'
    Fields = [ordered]@{
        version = $jsonStringSchema
        purpose = $jsonStringSchema
    }
}
$sourceLockSchema = [ordered]@{
    Kind = 'object'
    Fields = [ordered]@{
        schema_version = $jsonStringSchema
        generated_at = $jsonStringSchema
        snapshot = [ordered]@{
            Kind = 'object'
            Fields = [ordered]@{
                path = $jsonStringSchema
                repository = $jsonStringSchema
                repository_url = $jsonStringSchema
                release_tag = $jsonStringSchema
                release_url = $jsonStringSchema
                release_published_at = $jsonStringSchema
                commit = $jsonStringSchema
                commit_message = $jsonStringSchema
                commit_tree = $jsonStringSchema
                commit_verification = [ordered]@{
                    Kind = 'object'
                    Fields = [ordered]@{
                        verified = $jsonBooleanSchema
                        reason = $jsonStringSchema
                    }
                }
            }
        }
        release_audit = [ordered]@{
            Kind = 'object'
            Fields = [ordered]@{
                schema_version = $jsonStringSchema
                catalog_release = [ordered]@{
                    Kind = 'object'
                    Fields = [ordered]@{
                        tag = $jsonStringSchema
                        commit = $jsonStringSchema
                    }
                }
                status = $jsonStringSchema
                stale_reason = $jsonNullableStringSchema
                re_audit_issue_ref = $jsonNullableStringSchema
            }
        }
        archive = [ordered]@{
            Kind = 'object'
            Fields = [ordered]@{
                format = $jsonStringSchema
                url = $jsonStringSchema
                sha256 = $jsonStringSchema
                bytes = $jsonInt64Schema
            }
        }
        tree = [ordered]@{
            Kind = 'object'
            Fields = [ordered]@{
                sha = $jsonStringSchema
                entry_count = $jsonInt64Schema
                directory_count = $jsonInt64Schema
                file_count = $jsonInt64Schema
                submodule_count = $jsonInt64Schema
            }
        }
        manifest = [ordered]@{
            Kind = 'object'
            Fields = [ordered]@{
                algorithm = $jsonStringSchema
                format = $jsonStringSchema
                sha256 = $jsonStringSchema
                file_count = $jsonInt64Schema
                total_bytes = $jsonInt64Schema
                largest_file = $sourceFileSchema
            }
        }
        license = [ordered]@{
            Kind = 'object'
            Fields = [ordered]@{
                repository_license_key = $jsonStringSchema
                repository_license_name = $jsonStringSchema
                repository_license_spdx_id = $jsonStringSchema
                preserved_files = [ordered]@{
                    Kind = 'array'
                    MinimumCount = 1
                    Item = $preservedLicenseFileSchema
                }
            }
        }
        scope = [ordered]@{
            Kind = 'object'
            Fields = [ordered]@{
                audit_only = $jsonBooleanSchema
                participates_in_product_build = $jsonBooleanSchema
                imports_git_history = $jsonBooleanSchema
                excluded_runtime_material = [ordered]@{
                    Kind = 'array'
                    MinimumCount = 1
                    Item = $jsonStringSchema
                }
                source_lock_is_not_part_of_snapshot_manifest = $jsonBooleanSchema
            }
        }
        verification = [ordered]@{
            Kind = 'object'
            Fields = [ordered]@{
                archive_path_safety = $jsonStringSchema
                archive_utf8_filename_handling = $jsonStringSchema
                git_blob_sha1_per_file = $jsonStringSchema
                tree_completeness = $jsonStringSchema
                staging_and_workspace_copy = $jsonStringSchema
            }
        }
        files = [ordered]@{
            Kind = 'array'
            MinimumCount = 1
            Item = $sourceFileSchema
        }
        generator = [ordered]@{
            Kind = 'object'
            Fields = [ordered]@{
                name = $jsonStringSchema
                version = $jsonStringSchema
                method = $jsonStringSchema
                host = [ordered]@{
                    Kind = 'object'
                    Fields = [ordered]@{
                        os = $jsonStringSchema
                        powershell = $jsonStringSchema
                    }
                }
                tools = [ordered]@{
                    Kind = 'object'
                    Fields = [ordered]@{
                        python = $generatorToolSchema
                        git = $generatorToolSchema
                        github_cli = $generatorToolSchema
                    }
                }
            }
        }
    }
}
$remoteEvidenceSchema = [ordered]@{
    Kind = 'object'
    Fields = [ordered]@{
        schema_version = $jsonStringSchema
        release = [ordered]@{
            Kind = 'object'
            Fields = [ordered]@{
                tag_name = $jsonStringSchema
                html_url = $jsonStringSchema
                published_at = $jsonStringSchema
                draft = $jsonBooleanSchema
                prerelease = $jsonBooleanSchema
            }
        }
        tag_ref = [ordered]@{
            Kind = 'object'
            Fields = [ordered]@{
                ref = $jsonStringSchema
                object_type = $jsonStringSchema
                commit = $jsonStringSchema
            }
        }
        commit = [ordered]@{
            Kind = 'object'
            Fields = [ordered]@{
                sha = $jsonStringSchema
                message = $jsonStringSchema
                tree = $jsonStringSchema
                verification = [ordered]@{
                    Kind = 'object'
                    Fields = [ordered]@{
                        verified = $jsonBooleanSchema
                        reason = $jsonStringSchema
                    }
                }
            }
        }
        license = [ordered]@{
            Kind = 'object'
            Fields = [ordered]@{
                key = $jsonStringSchema
                name = $jsonStringSchema
                spdx_id = $jsonStringSchema
                path = $jsonStringSchema
                git_blob_sha1 = $jsonStringSchema
            }
        }
        archive = [ordered]@{
            Kind = 'object'
            Fields = [ordered]@{
                url = $jsonStringSchema
            }
        }
    }
}

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

function ConvertFrom-StrictJsonElement {
    param(
        [Parameter(Mandatory)]
        [System.Text.Json.JsonElement]$Element,

        [Parameter(Mandatory)]
        [string]$Location
    )

    switch ($Element.ValueKind) {
        ([System.Text.Json.JsonValueKind]::Object) {
            $properties = [ordered]@{}
            $names = [Collections.Generic.HashSet[string]]::new([StringComparer]::Ordinal)
            foreach ($property in $Element.EnumerateObject()) {
                if (-not $names.Add($property.Name)) {
                    throw [IO.InvalidDataException]::new("$Location 包含重复 key：$($property.Name)")
                }
                $properties[$property.Name] = ConvertFrom-StrictJsonElement `
                    -Element $property.Value `
                    -Location "$Location.$($property.Name)"
            }
            $object = [pscustomobject]$properties
            return ,$object
        }
        ([System.Text.Json.JsonValueKind]::Array) {
            $items = [Collections.Generic.List[object]]::new()
            $index = 0
            foreach ($item in $Element.EnumerateArray()) {
                $items.Add((ConvertFrom-StrictJsonElement -Element $item -Location "$Location[$index]"))
                $index += 1
            }
            return ,$items.ToArray()
        }
        ([System.Text.Json.JsonValueKind]::String) {
            return $Element.GetString()
        }
        ([System.Text.Json.JsonValueKind]::Number) {
            $number = 0L
            if (-not $Element.TryGetInt64([ref]$number)) {
                throw [IO.InvalidDataException]::new("$Location 必须是 Int64 JSON 整数。")
            }
            return $number
        }
        ([System.Text.Json.JsonValueKind]::True) {
            return $true
        }
        ([System.Text.Json.JsonValueKind]::False) {
            return $false
        }
        ([System.Text.Json.JsonValueKind]::Null) {
            return $null
        }
        default {
            throw [IO.InvalidDataException]::new("$Location 包含不受支持的 JSON token。")
        }
    }
}

function ConvertFrom-StrictJsonText {
    param(
        [Parameter(Mandatory)]
        [string]$Content,

        [Parameter(Mandatory)]
        [string]$Location
    )

    $options = [System.Text.Json.JsonDocumentOptions]::new()
    $options.AllowTrailingCommas = $false
    $options.CommentHandling = [System.Text.Json.JsonCommentHandling]::Disallow
    $options.MaxDepth = 100
    $document = [System.Text.Json.JsonDocument]::Parse($Content, $options)
    try {
        return ,(ConvertFrom-StrictJsonElement -Element $document.RootElement -Location $Location)
    }
    finally {
        $document.Dispose()
    }
}

function Test-JsonSchemaNode {
    param(
        [AllowNull()]$Value,

        [Parameter(Mandatory)]
        [System.Collections.IDictionary]$Schema,

        [Parameter(Mandatory)]
        [string]$Location,

        [Parameter(Mandatory)]
        [AllowEmptyCollection()]
        [Collections.Generic.List[string]]$Problems
    )

    $valid = $true
    switch ($Schema.Kind) {
        'object' {
            if ($Value -isnot [System.Management.Automation.PSCustomObject]) {
                $Problems.Add("$Location 必须是 object") | Out-Null
                return $false
            }
            $actual = [Collections.Generic.Dictionary[string,object]]::new([StringComparer]::Ordinal)
            foreach ($property in $Value.PSObject.Properties) {
                $actual.Add($property.Name, $property.Value)
            }
            $expectedNames = [Collections.Generic.HashSet[string]]::new([StringComparer]::Ordinal)
            foreach ($name in [string[]]$Schema.Fields.Keys) {
                [void]$expectedNames.Add($name)
            }
            foreach ($name in [string[]]$actual.Keys) {
                if (-not $expectedNames.Contains($name)) {
                    $Problems.Add("$Location 包含未知字段 $name") | Out-Null
                    $valid = $false
                }
            }
            foreach ($name in [string[]]$Schema.Fields.Keys) {
                if (-not $actual.ContainsKey($name)) {
                    $Problems.Add("$Location 缺少字段 $name") | Out-Null
                    $valid = $false
                    continue
                }
                if (-not (Test-JsonSchemaNode `
                    -Value $actual[$name] `
                    -Schema $Schema.Fields[$name] `
                    -Location "$Location.$name" `
                    -Problems $Problems)) {
                    $valid = $false
                }
            }
        }
        'array' {
            if ($Value -isnot [System.Array]) {
                $Problems.Add("$Location 必须是 array") | Out-Null
                return $false
            }
            if ($Schema.Contains('MinimumCount') -and $Value.Count -lt $Schema.MinimumCount) {
                $Problems.Add("$Location 元素数不得小于 $($Schema.MinimumCount)") | Out-Null
                $valid = $false
            }
            for ($index = 0; $index -lt $Value.Count; $index += 1) {
                if (-not (Test-JsonSchemaNode `
                    -Value $Value[$index] `
                    -Schema $Schema.Item `
                    -Location "$Location[$index]" `
                    -Problems $Problems)) {
                    $valid = $false
                }
            }
        }
        'string' {
            if ($Value -isnot [string]) {
                $Problems.Add("$Location 必须是 string") | Out-Null
                $valid = $false
            }
        }
        'nullable-string' {
            if ($null -ne $Value -and $Value -isnot [string]) {
                $Problems.Add("$Location 必须是 string 或 null") | Out-Null
                $valid = $false
            }
        }
        'int64' {
            if ($Value -isnot [long]) {
                $Problems.Add("$Location 必须是 Int64") | Out-Null
                $valid = $false
            }
        }
        'boolean' {
            if ($Value -isnot [bool]) {
                $Problems.Add("$Location 必须是 Boolean") | Out-Null
                $valid = $false
            }
        }
        default {
            throw "未知内部 JSON schema kind：$($Schema.Kind)"
        }
    }
    return $valid
}

function Test-SourceLockSchema {
    param(
        $SourceLock,

        [Parameter(Mandatory)]
        [string]$Location
    )

    $problems = [Collections.Generic.List[string]]::new()
    $valid = Test-JsonSchemaNode `
        -Value $SourceLock `
        -Schema $sourceLockSchema `
        -Location $Location `
        -Problems $problems
    if (-not $valid) {
        Add-ReleaseAuditError `
            -Code 'RELEASE_AUDIT_INVALID' `
            -Message ([string]::Join('；', $problems.ToArray()))
    }
    return $valid
}

function Get-ExpectedUpstreamUrls {
    param(
        [Parameter(Mandatory)]
        [string]$ReleaseTag
    )

    [pscustomobject][ordered]@{
        repository_url = $upstreamRepositoryUrl
        release_url = "$upstreamRepositoryUrl/releases/tag/$ReleaseTag"
        archive_url = "https://codeload.github.com/$upstreamRepository/tar.gz/refs/tags/$ReleaseTag"
    }
}

function Test-ExactUtcTimestamp {
    param($Value)

    if ($Value -isnot [string] -or
        $Value -cnotmatch '^[0-9]{4}-[0-9]{2}-[0-9]{2}T[0-9]{2}:[0-9]{2}:[0-9]{2}Z$') {
        return $false
    }
    $parsed = [DateTimeOffset]::MinValue
    return [DateTimeOffset]::TryParseExact(
        $Value,
        "yyyy-MM-dd'T'HH:mm:ss'Z'",
        [Globalization.CultureInfo]::InvariantCulture,
        [Globalization.DateTimeStyles]::AssumeUniversal -bor [Globalization.DateTimeStyles]::AdjustToUniversal,
        [ref]$parsed
    )
}

function Test-SourceLockIdentity {
    param(
        [Parameter(Mandatory)]
        $SourceLock,

        [Parameter(Mandatory)]
        [string]$Location
    )

    $valid = $true
    $snapshot = Get-ObjectPropertyValue -Object $SourceLock -Name 'snapshot'
    $archive = Get-ObjectPropertyValue -Object $SourceLock -Name 'archive'
    $tree = Get-ObjectPropertyValue -Object $SourceLock -Name 'tree'
    $urls = Get-ExpectedUpstreamUrls -ReleaseTag (Get-ObjectPropertyValue -Object $snapshot -Name 'release_tag')

    $checks = @(
        @{ Value = Get-ObjectPropertyValue -Object $SourceLock -Name 'schema_version'; Expected = $sourceLockSchemaVersion; Message = 'schema_version 漂移' },
        @{ Value = Get-ObjectPropertyValue -Object $snapshot -Name 'repository'; Expected = $upstreamRepository; Message = 'snapshot.repository 漂移' },
        @{ Value = Get-ObjectPropertyValue -Object $snapshot -Name 'repository_url'; Expected = $urls.repository_url; Message = 'snapshot.repository_url 不能由固定仓库推导' },
        @{ Value = Get-ObjectPropertyValue -Object $snapshot -Name 'release_url'; Expected = $urls.release_url; Message = 'snapshot.release_url 不能由 repository/tag 推导' },
        @{ Value = Get-ObjectPropertyValue -Object $archive -Name 'format'; Expected = 'tar.gz'; Message = 'archive.format 必须为 tar.gz' },
        @{ Value = Get-ObjectPropertyValue -Object $archive -Name 'url'; Expected = $urls.archive_url; Message = 'archive.url 不能由 repository/tag 推导' },
        @{ Value = Get-ObjectPropertyValue -Object $snapshot -Name 'commit_tree'; Expected = Get-ObjectPropertyValue -Object $tree -Name 'sha'; Message = 'snapshot.commit_tree 必须等于 tree.sha' }
    )
    foreach ($check in $checks) {
        if (-not (Test-ExactJsonString -Actual $check.Value -Expected $check.Expected)) {
            $valid = $false
            Add-ReleaseAuditError -Code 'RELEASE_AUDIT_INVALID' -Message "$Location：$($check.Message)"
        }
    }

    $commit = Get-ObjectPropertyValue -Object $snapshot -Name 'commit'
    $treeSha = Get-ObjectPropertyValue -Object $tree -Name 'sha'
    $releaseTag = Get-ObjectPropertyValue -Object $snapshot -Name 'release_tag'
    if ($releaseTag -isnot [string] -or $releaseTag -cnotmatch '^v[0-9]+\.[0-9]+\.[0-9]+$') {
        $valid = $false
        Add-ReleaseAuditError -Code 'RELEASE_AUDIT_INVALID' -Message "$Location.snapshot.release_tag 必须是规范 vX.Y.Z。"
    }
    foreach ($timestampProperty in @(
        @{ Object = $SourceLock; Name = 'generated_at'; Path = 'generated_at' },
        @{ Object = $snapshot; Name = 'release_published_at'; Path = 'snapshot.release_published_at' }
    )) {
        if (-not (Test-ExactUtcTimestamp `
            -Value (Get-ObjectPropertyValue -Object $timestampProperty.Object -Name $timestampProperty.Name))) {
            $valid = $false
            Add-ReleaseAuditError -Code 'RELEASE_AUDIT_INVALID' -Message "$Location.$($timestampProperty.Path) 必须是精确 UTC RFC3339 秒。"
        }
    }
    if (-not (Test-JsonStringPattern -Actual $commit -Pattern '^[0-9a-f]{40}$')) {
        $valid = $false
        Add-ReleaseAuditError -Code 'RELEASE_AUDIT_INVALID' -Message "$Location.snapshot.commit 必须是小写 40 位 SHA-1。"
    }
    if (-not (Test-JsonStringPattern -Actual $treeSha -Pattern '^[0-9a-f]{40}$')) {
        $valid = $false
        Add-ReleaseAuditError -Code 'RELEASE_AUDIT_INVALID' -Message "$Location.tree.sha 必须是小写 40 位 SHA-1。"
    }
    $archiveSha = Get-ObjectPropertyValue -Object $archive -Name 'sha256'
    $archiveBytes = Get-ObjectPropertyValue -Object $archive -Name 'bytes'
    if (-not (Test-JsonStringPattern -Actual $archiveSha -Pattern '^[0-9a-f]{64}$') -or
        $archiveBytes -le 0 -or $archiveBytes -gt $maximumArchiveBytes) {
        $valid = $false
        Add-ReleaseAuditError -Code 'RELEASE_AUDIT_INVALID' -Message "$Location.archive 的 sha256/bytes 不符合严格边界。"
    }
    $commitMessage = Get-ObjectPropertyValue -Object $snapshot -Name 'commit_message'
    $verification = Get-ObjectPropertyValue -Object $snapshot -Name 'commit_verification'
    $verificationReason = Get-ObjectPropertyValue -Object $verification -Name 'reason'
    if ($commitMessage -isnot [string] -or [string]::IsNullOrWhiteSpace($commitMessage) -or
        $verificationReason -isnot [string] -or [string]::IsNullOrWhiteSpace($verificationReason)) {
        $valid = $false
        Add-ReleaseAuditError -Code 'RELEASE_AUDIT_INVALID' -Message "$Location.snapshot commit message/verification reason 必须非空。"
    }
    $license = Get-ObjectPropertyValue -Object $SourceLock -Name 'license'
    foreach ($name in @('repository_license_key', 'repository_license_name', 'repository_license_spdx_id')) {
        $value = Get-ObjectPropertyValue -Object $license -Name $name
        if ($value -isnot [string] -or [string]::IsNullOrWhiteSpace($value)) {
            $valid = $false
            Add-ReleaseAuditError -Code 'UPSTREAM_SNAPSHOT_INTEGRITY_INVALID' -Message "$Location.license.$name 必须是非空 string。"
        }
    }
    return $valid
}

function Test-LicenseEvidencePath {
    param([Parameter(Mandatory)][string]$Path)

    $leaf = $Path.Substring($Path.LastIndexOf('/') + 1)
    $options = [Text.RegularExpressions.RegexOptions]::IgnoreCase -bor
        [Text.RegularExpressions.RegexOptions]::CultureInvariant
    return [regex]::IsMatch(
        $leaf,
        '^(LICENSE|LICENCE|NOTICE|NOTICES|COPYING|COPYRIGHT)([._-].*)?$|^THIRD[_-]PARTY[_-](LICENSE|LICENCES|NOTICE|NOTICES)([._-].*)?$',
        $options
    )
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
        $value = ConvertFrom-StrictJsonText -Content $content -Location $Path
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

    if ($SourceLock -isnot [System.Management.Automation.PSCustomObject]) {
        return $null
    }

    [pscustomobject][ordered]@{
        schema_version = Get-ObjectPropertyValue -Object $SourceLock -Name 'schema_version'
        generated_at = Get-ObjectPropertyValue -Object $SourceLock -Name 'generated_at'
        snapshot = Get-ObjectPropertyValue -Object $SourceLock -Name 'snapshot'
        release_audit = Get-ObjectPropertyValue -Object $SourceLock -Name 'release_audit'
        archive = Get-ObjectPropertyValue -Object $SourceLock -Name 'archive'
        tree = Get-ObjectPropertyValue -Object $SourceLock -Name 'tree'
        manifest = Get-ObjectPropertyValue -Object $SourceLock -Name 'manifest'
        license = Get-ObjectPropertyValue -Object $SourceLock -Name 'license'
        scope = Get-ObjectPropertyValue -Object $SourceLock -Name 'scope'
        verification = Get-ObjectPropertyValue -Object $SourceLock -Name 'verification'
        files = Get-ObjectPropertyValue -Object $SourceLock -Name 'files'
        generator = Get-ObjectPropertyValue -Object $SourceLock -Name 'generator'
    } | ConvertTo-Json -Depth 100 -Compress
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

    if (-not (Test-SourceLockSchema -SourceLock $SourceLock -Location '当前 source-lock')) {
        return
    }
    [void](Test-SourceLockIdentity -SourceLock $SourceLock -Location '当前 source-lock')
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

    $license = Get-ObjectPropertyValue -Object $SourceLock -Name 'license'
    $preservedFiles = Get-ObjectPropertyValue -Object $license -Name 'preserved_files'
    $licenseEvidencePaths = [Collections.Generic.HashSet[string]]::new([StringComparer]::Ordinal)
    foreach ($path in [string[]]$expected.Keys) {
        if (Test-LicenseEvidencePath -Path $path) {
            [void]$licenseEvidencePaths.Add($path)
        }
    }
    $preservedPaths = [Collections.Generic.HashSet[string]]::new([StringComparer]::Ordinal)
    $previousPreservedPath = $null
    foreach ($preservedFile in @($preservedFiles)) {
        $path = Get-ObjectPropertyValue -Object $preservedFile -Name 'path'
        if ($path -isnot [string] -or -not (Test-SafeSnapshotRelativePath -Value $path)) {
            Add-IntegrityProblem 'license.preserved_files 包含非法路径'
            continue
        }
        if ($null -ne $previousPreservedPath -and
            [StringComparer]::Ordinal.Compare($previousPreservedPath, $path) -ge 0) {
            Add-IntegrityProblem 'license.preserved_files 必须按 Ordinal 严格递增且无重复' $path
        }
        $previousPreservedPath = $path
        if (-not $preservedPaths.Add($path)) {
            Add-IntegrityProblem 'license.preserved_files 包含重复路径' $path
            continue
        }
        if (-not $licenseEvidencePaths.Contains($path) -or -not $expected.ContainsKey($path)) {
            Add-IntegrityProblem 'license.preserved_files 包含非许可证证据或额外路径' $path
            continue
        }
        $sourceFile = $expected[$path]
        $leaf = $path.Substring($path.LastIndexOf('/') + 1)
        $expectedKind = if ($path -ceq 'LICENSE') {
            'repository-license'
        } elseif ($leaf -cmatch '^(NOTICE|NOTICES)([._-].*)?$' -or
            $leaf -cmatch '^THIRD[_-]PARTY[_-]NOTICES?([._-].*)?$') {
            'third-party-notice'
        } else {
            'embedded-third-party-license'
        }
        if (-not (Test-ExactJsonString `
            -Actual (Get-ObjectPropertyValue -Object $preservedFile -Name 'kind') `
            -Expected $expectedKind)) {
            Add-IntegrityProblem 'license.preserved_files kind 与证据路径不一致' $path
        }
        if ((Get-ObjectPropertyValue -Object $preservedFile -Name 'size') -ne
            (Get-ObjectPropertyValue -Object $sourceFile -Name 'size') -or
            -not (Test-ExactJsonString `
                -Actual (Get-ObjectPropertyValue -Object $preservedFile -Name 'sha256') `
                -Expected (Get-ObjectPropertyValue -Object $sourceFile -Name 'sha256'))) {
            Add-IntegrityProblem 'license.preserved_files size/sha256 与 files/Git blob 不一致' $path
        }
    }
    foreach ($path in $licenseEvidencePaths) {
        if (-not $preservedPaths.Contains($path)) {
            Add-IntegrityProblem '快照许可证证据未进入 license.preserved_files' $path
        }
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

function Get-BoundedArchiveEvidence {
    param(
        [Parameter(Mandatory)]
        [string]$Path
    )

    if (-not (Test-Path -LiteralPath $Path -PathType Leaf)) {
        throw "archive 文件不存在：$Path"
    }
    $stream = [IO.File]::Open($Path, [IO.FileMode]::Open, [IO.FileAccess]::Read, [IO.FileShare]::Read)
    $hasher = [Security.Cryptography.IncrementalHash]::CreateHash(
        [Security.Cryptography.HashAlgorithmName]::SHA256
    )
    $buffer = [byte[]]::new(81920)
    $bytes = 0L
    try {
        while (($read = $stream.Read($buffer, 0, $buffer.Length)) -gt 0) {
            $bytes += [long]$read
            if ($bytes -gt $maximumArchiveBytes) {
                throw "archive 超过 64 MiB 上限：$bytes"
            }
            $hasher.AppendData($buffer, 0, $read)
        }
        $sha256 = [Convert]::ToHexString($hasher.GetHashAndReset()).ToLowerInvariant()
        return ,[pscustomobject][ordered]@{
            path = [IO.Path]::GetFullPath($Path)
            bytes = $bytes
            sha256 = $sha256
        }
    }
    finally {
        $hasher.Dispose()
        $stream.Dispose()
    }
}

function Invoke-FixedGitHubJsonRequest {
    param(
        [Parameter(Mandatory)]
        [Net.Http.HttpClient]$Client,

        [Parameter(Mandatory)]
        [string]$Uri,

        [Parameter(Mandatory)]
        [string]$Location
    )

    $request = [Net.Http.HttpRequestMessage]::new([Net.Http.HttpMethod]::Get, $Uri)
    $request.Headers.Accept.ParseAdd('application/vnd.github+json')
    $request.Headers.Add('X-GitHub-Api-Version', '2022-11-28')
    $cancellation = [Threading.CancellationTokenSource]::new([TimeSpan]::FromSeconds(30))
    $response = $null
    try {
        $response = $Client.SendAsync(
            $request,
            [Net.Http.HttpCompletionOption]::ResponseContentRead,
            $cancellation.Token
        ).GetAwaiter().GetResult()
        if (-not $response.IsSuccessStatusCode) {
            throw "$Location 返回 HTTP $([int]$response.StatusCode)。"
        }
        $content = $response.Content.ReadAsStringAsync().GetAwaiter().GetResult()
        return ,(ConvertFrom-StrictJsonText -Content $content -Location $Location)
    }
    finally {
        if ($null -ne $response) { $response.Dispose() }
        $cancellation.Dispose()
        $request.Dispose()
    }
}

function Invoke-BoundedArchiveDownload {
    param(
        [Parameter(Mandatory)]
        [Net.Http.HttpClient]$Client,

        [Parameter(Mandatory)]
        [string]$Uri,

        [Parameter(Mandatory)]
        [string]$DestinationPath
    )

    $request = [Net.Http.HttpRequestMessage]::new([Net.Http.HttpMethod]::Get, $Uri)
    $cancellation = [Threading.CancellationTokenSource]::new([TimeSpan]::FromSeconds(120))
    $response = $null
    $input = $null
    $output = $null
    $hasher = $null
    $completed = $false
    try {
        $response = $Client.SendAsync(
            $request,
            [Net.Http.HttpCompletionOption]::ResponseHeadersRead,
            $cancellation.Token
        ).GetAwaiter().GetResult()
        if (-not $response.IsSuccessStatusCode) {
            throw "codeload archive 返回 HTTP $([int]$response.StatusCode)。"
        }
        $contentLength = $response.Content.Headers.ContentLength
        if ($null -ne $contentLength -and [long]$contentLength -gt $maximumArchiveBytes) {
            throw "codeload archive Content-Length 超过 64 MiB：$contentLength"
        }

        $input = $response.Content.ReadAsStreamAsync().GetAwaiter().GetResult()
        $output = [IO.File]::Open(
            $DestinationPath,
            [IO.FileMode]::CreateNew,
            [IO.FileAccess]::Write,
            [IO.FileShare]::None
        )
        $hasher = [Security.Cryptography.IncrementalHash]::CreateHash(
            [Security.Cryptography.HashAlgorithmName]::SHA256
        )
        $buffer = [byte[]]::new(81920)
        $bytes = 0L
        while (($read = $input.ReadAsync(
            $buffer,
            0,
            $buffer.Length,
            $cancellation.Token
        ).GetAwaiter().GetResult()) -gt 0) {
            $bytes += [long]$read
            if ($bytes -gt $maximumArchiveBytes) {
                throw "codeload archive 下载超过 64 MiB：$bytes"
            }
            $output.Write($buffer, 0, $read)
            $hasher.AppendData($buffer, 0, $read)
        }
        $output.Flush()
        $sha256 = [Convert]::ToHexString($hasher.GetHashAndReset()).ToLowerInvariant()
        $completed = $true
        return ,[pscustomobject][ordered]@{
            path = [IO.Path]::GetFullPath($DestinationPath)
            bytes = $bytes
            sha256 = $sha256
        }
    }
    finally {
        if ($null -ne $hasher) { $hasher.Dispose() }
        if ($null -ne $output) { $output.Dispose() }
        if ($null -ne $input) { $input.Dispose() }
        if ($null -ne $response) { $response.Dispose() }
        $cancellation.Dispose()
        $request.Dispose()
        if (-not $completed -and (Test-Path -LiteralPath $DestinationPath -PathType Leaf)) {
            Remove-Item -LiteralPath $DestinationPath -Force
        }
    }
}

function Get-LiveRemoteReleaseEvidence {
    param(
        [Parameter(Mandatory)]
        $SourceLock,

        [Parameter(Mandatory)]
        [string]$TemporaryDirectory
    )

    if ([string]::IsNullOrWhiteSpace($env:GH_TOKEN)) {
        throw 'live 远端验证缺少只读 GH_TOKEN。'
    }
    if (-not (Test-Path -LiteralPath $TemporaryDirectory -PathType Container)) {
        throw 'live 远端验证临时目录不存在。'
    }

    $snapshot = Get-ObjectPropertyValue -Object $SourceLock -Name 'snapshot'
    $releaseTag = Get-ObjectPropertyValue -Object $snapshot -Name 'release_tag'
    $escapedTag = [Uri]::EscapeDataString($releaseTag)
    $apiRoot = "https://api.github.com/repos/$upstreamRepository"
    $client = [Net.Http.HttpClient]::new()
    $client.Timeout = [Threading.Timeout]::InfiniteTimeSpan
    $client.DefaultRequestHeaders.UserAgent.ParseAdd('inputcodex-release-audit/1')
    $client.DefaultRequestHeaders.Authorization = [Net.Http.Headers.AuthenticationHeaderValue]::new(
        'Bearer',
        $env:GH_TOKEN
    )
    $archiveClient = [Net.Http.HttpClient]::new()
    $archiveClient.Timeout = [Threading.Timeout]::InfiniteTimeSpan
    $archiveClient.DefaultRequestHeaders.UserAgent.ParseAdd('inputcodex-release-audit/1')
    $archivePath = Join-Path $TemporaryDirectory ("upstream-release-{0}.tar.gz" -f [guid]::NewGuid().ToString('N'))
    try {
        $release = Invoke-FixedGitHubJsonRequest `
            -Client $client `
            -Uri "$apiRoot/releases/tags/$escapedTag" `
            -Location 'GitHub release API'
        $tagRef = Invoke-FixedGitHubJsonRequest `
            -Client $client `
            -Uri "$apiRoot/git/ref/tags/$escapedTag" `
            -Location 'GitHub tag ref API'
        $refObject = Get-ObjectPropertyValue -Object $tagRef -Name 'object'
        $refObjectType = Get-ObjectPropertyValue -Object $refObject -Name 'type'
        $resolvedType = $refObjectType
        $resolvedSha = Get-ObjectPropertyValue -Object $refObject -Name 'sha'
        for ($depth = 0; $resolvedType -ceq 'tag' -and $depth -lt 5; $depth += 1) {
            $tagObject = Invoke-FixedGitHubJsonRequest `
                -Client $client `
                -Uri "$apiRoot/git/tags/$resolvedSha" `
                -Location 'GitHub annotated tag API'
            $target = Get-ObjectPropertyValue -Object $tagObject -Name 'object'
            $resolvedType = Get-ObjectPropertyValue -Object $target -Name 'type'
            $resolvedSha = Get-ObjectPropertyValue -Object $target -Name 'sha'
        }
        if ($resolvedType -cne 'commit' -or
            -not (Test-JsonStringPattern -Actual $resolvedSha -Pattern '^[0-9a-f]{40}$')) {
            throw 'GitHub tag ref 未在五层内解析为 commit。'
        }

        $commit = Invoke-FixedGitHubJsonRequest `
            -Client $client `
            -Uri "$apiRoot/commits/$resolvedSha" `
            -Location 'GitHub commit API'
        $license = Invoke-FixedGitHubJsonRequest `
            -Client $client `
            -Uri "$apiRoot/license?ref=$([Uri]::EscapeDataString($resolvedSha))" `
            -Location 'GitHub license API'
        $commitPayload = Get-ObjectPropertyValue -Object $commit -Name 'commit'
        $commitTree = Get-ObjectPropertyValue -Object $commitPayload -Name 'tree'
        $commitVerification = Get-ObjectPropertyValue -Object $commitPayload -Name 'verification'
        $licenseMetadata = Get-ObjectPropertyValue -Object $license -Name 'license'
        $archiveUrl = (Get-ExpectedUpstreamUrls -ReleaseTag $releaseTag).archive_url
        $archiveEvidence = Invoke-BoundedArchiveDownload `
            -Client $archiveClient `
            -Uri $archiveUrl `
            -DestinationPath $archivePath
        $evidence = [pscustomobject][ordered]@{
            schema_version = $remoteEvidenceSchemaVersion
            release = [pscustomobject][ordered]@{
                tag_name = Get-ObjectPropertyValue -Object $release -Name 'tag_name'
                html_url = Get-ObjectPropertyValue -Object $release -Name 'html_url'
                published_at = Get-ObjectPropertyValue -Object $release -Name 'published_at'
                draft = Get-ObjectPropertyValue -Object $release -Name 'draft'
                prerelease = Get-ObjectPropertyValue -Object $release -Name 'prerelease'
            }
            tag_ref = [pscustomobject][ordered]@{
                ref = Get-ObjectPropertyValue -Object $tagRef -Name 'ref'
                object_type = $refObjectType
                commit = $resolvedSha
            }
            commit = [pscustomobject][ordered]@{
                sha = Get-ObjectPropertyValue -Object $commit -Name 'sha'
                message = Get-ObjectPropertyValue -Object $commitPayload -Name 'message'
                tree = Get-ObjectPropertyValue -Object $commitTree -Name 'sha'
                verification = [pscustomobject][ordered]@{
                    verified = Get-ObjectPropertyValue -Object $commitVerification -Name 'verified'
                    reason = Get-ObjectPropertyValue -Object $commitVerification -Name 'reason'
                }
            }
            license = [pscustomobject][ordered]@{
                key = Get-ObjectPropertyValue -Object $licenseMetadata -Name 'key'
                name = Get-ObjectPropertyValue -Object $licenseMetadata -Name 'name'
                spdx_id = Get-ObjectPropertyValue -Object $licenseMetadata -Name 'spdx_id'
                path = Get-ObjectPropertyValue -Object $license -Name 'path'
                git_blob_sha1 = Get-ObjectPropertyValue -Object $license -Name 'sha'
            }
            archive = [pscustomobject][ordered]@{
                url = $archiveUrl
            }
        }
        return ,[pscustomobject][ordered]@{
            evidence = $evidence
            archive = $archiveEvidence
            archive_path = $archivePath
        }
    }
    catch {
        if (Test-Path -LiteralPath $archivePath -PathType Leaf) {
            Remove-Item -LiteralPath $archivePath -Force
        }
        throw
    }
    finally {
        $archiveClient.Dispose()
        $client.Dispose()
    }
}

function Test-RemoteReleaseEvidence {
    param(
        [Parameter(Mandatory)]$SourceLock,
        [Parameter(Mandatory)]$Evidence,
        [Parameter(Mandatory)]$ArchiveEvidence
    )

    $problems = [Collections.Generic.List[string]]::new()
    if (-not (Test-JsonSchemaNode `
        -Value $Evidence `
        -Schema $remoteEvidenceSchema `
        -Location 'remote evidence' `
        -Problems $problems)) {
        Add-ReleaseAuditError `
            -Code 'RELEASE_AUDIT_REMOTE_EVIDENCE_INVALID' `
            -Message ([string]::Join('；', $problems.ToArray()))
        return $false
    }

    $snapshot = Get-ObjectPropertyValue -Object $SourceLock -Name 'snapshot'
    $archive = Get-ObjectPropertyValue -Object $SourceLock -Name 'archive'
    $license = Get-ObjectPropertyValue -Object $SourceLock -Name 'license'
    $release = Get-ObjectPropertyValue -Object $Evidence -Name 'release'
    $tagRef = Get-ObjectPropertyValue -Object $Evidence -Name 'tag_ref'
    $commit = Get-ObjectPropertyValue -Object $Evidence -Name 'commit'
    $commitVerification = Get-ObjectPropertyValue -Object $commit -Name 'verification'
    $remoteLicense = Get-ObjectPropertyValue -Object $Evidence -Name 'license'
    $remoteArchive = Get-ObjectPropertyValue -Object $Evidence -Name 'archive'
    $snapshotVerification = Get-ObjectPropertyValue -Object $snapshot -Name 'commit_verification'
    $releaseTag = Get-ObjectPropertyValue -Object $snapshot -Name 'release_tag'
    $expectedUrls = Get-ExpectedUpstreamUrls -ReleaseTag $releaseTag

    $pairs = @(
        @{ Actual = Get-ObjectPropertyValue -Object $Evidence -Name 'schema_version'; Expected = $remoteEvidenceSchemaVersion; Name = 'schema_version' },
        @{ Actual = Get-ObjectPropertyValue -Object $release -Name 'tag_name'; Expected = $releaseTag; Name = 'release.tag_name' },
        @{ Actual = Get-ObjectPropertyValue -Object $release -Name 'html_url'; Expected = $expectedUrls.release_url; Name = 'release.html_url' },
        @{ Actual = Get-ObjectPropertyValue -Object $release -Name 'published_at'; Expected = Get-ObjectPropertyValue -Object $snapshot -Name 'release_published_at'; Name = 'release.published_at' },
        @{ Actual = Get-ObjectPropertyValue -Object $tagRef -Name 'ref'; Expected = "refs/tags/$releaseTag"; Name = 'tag_ref.ref' },
        @{ Actual = Get-ObjectPropertyValue -Object $tagRef -Name 'commit'; Expected = Get-ObjectPropertyValue -Object $snapshot -Name 'commit'; Name = 'tag_ref.commit' },
        @{ Actual = Get-ObjectPropertyValue -Object $commit -Name 'sha'; Expected = Get-ObjectPropertyValue -Object $snapshot -Name 'commit'; Name = 'commit.sha' },
        @{ Actual = Get-ObjectPropertyValue -Object $commit -Name 'message'; Expected = Get-ObjectPropertyValue -Object $snapshot -Name 'commit_message'; Name = 'commit.message' },
        @{ Actual = Get-ObjectPropertyValue -Object $commit -Name 'tree'; Expected = Get-ObjectPropertyValue -Object $snapshot -Name 'commit_tree'; Name = 'commit.tree' },
        @{ Actual = Get-ObjectPropertyValue -Object $commitVerification -Name 'reason'; Expected = Get-ObjectPropertyValue -Object $snapshotVerification -Name 'reason'; Name = 'commit.verification.reason' },
        @{ Actual = Get-ObjectPropertyValue -Object $remoteLicense -Name 'key'; Expected = Get-ObjectPropertyValue -Object $license -Name 'repository_license_key'; Name = 'license.key' },
        @{ Actual = Get-ObjectPropertyValue -Object $remoteLicense -Name 'name'; Expected = Get-ObjectPropertyValue -Object $license -Name 'repository_license_name'; Name = 'license.name' },
        @{ Actual = Get-ObjectPropertyValue -Object $remoteLicense -Name 'spdx_id'; Expected = Get-ObjectPropertyValue -Object $license -Name 'repository_license_spdx_id'; Name = 'license.spdx_id' },
        @{ Actual = Get-ObjectPropertyValue -Object $remoteLicense -Name 'path'; Expected = 'LICENSE'; Name = 'license.path' },
        @{ Actual = Get-ObjectPropertyValue -Object $remoteArchive -Name 'url'; Expected = $expectedUrls.archive_url; Name = 'archive.url' },
        @{ Actual = $ArchiveEvidence.sha256; Expected = Get-ObjectPropertyValue -Object $archive -Name 'sha256'; Name = 'archive.sha256' }
    )
    foreach ($pair in $pairs) {
        if (-not (Test-ExactJsonString -Actual $pair.Actual -Expected $pair.Expected)) {
            $problems.Add("remote $($pair.Name) 不匹配") | Out-Null
        }
    }
    if ((Get-ObjectPropertyValue -Object $release -Name 'draft') -ne $false -or
        (Get-ObjectPropertyValue -Object $release -Name 'prerelease') -ne $false) {
        $problems.Add('remote release 必须是非 draft、非 prerelease') | Out-Null
    }
    if (@('commit', 'tag') -cnotcontains (Get-ObjectPropertyValue -Object $tagRef -Name 'object_type')) {
        $problems.Add('remote tag ref object_type 非法') | Out-Null
    }
    if ((Get-ObjectPropertyValue -Object $commitVerification -Name 'verified') -ne
        (Get-ObjectPropertyValue -Object $snapshotVerification -Name 'verified')) {
        $problems.Add('remote commit.verification.verified 不匹配') | Out-Null
    }
    if ($ArchiveEvidence.bytes -isnot [long] -or
        $ArchiveEvidence.bytes -ne (Get-ObjectPropertyValue -Object $archive -Name 'bytes')) {
        $problems.Add('remote archive.bytes 不匹配') | Out-Null
    }

    $sourceFilesProperty = $SourceLock.PSObject.Properties['files']
    $licenseFile = @(
        $sourceFilesProperty.Value |
            Where-Object { (Get-ObjectPropertyValue -Object $_ -Name 'path') -ceq 'LICENSE' }
    )
    if ($licenseFile.Count -ne 1 -or
        -not (Test-ExactJsonString `
            -Actual (Get-ObjectPropertyValue -Object $remoteLicense -Name 'git_blob_sha1') `
            -Expected (Get-ObjectPropertyValue -Object $licenseFile[0] -Name 'git_blob_sha1'))) {
        $problems.Add('remote license Git blob 与快照 LICENSE 不匹配') | Out-Null
    }

    if ($problems.Count -ne 0) {
        Add-ReleaseAuditError `
            -Code 'RELEASE_AUDIT_REMOTE_EVIDENCE_INVALID' `
            -Message ([string]::Join('；', $problems.ToArray()))
        return $false
    }
    return $true
}

function Invoke-RemoteReleaseValidation {
    param(
        [Parameter(Mandatory)][string]$RepositoryRoot,
        [Parameter(Mandatory)]$SourceLock
    )

    if ($RemoteValidationMode -ceq 'none') {
        if (-not [string]::IsNullOrWhiteSpace($RemoteEvidencePath) -or
            -not [string]::IsNullOrWhiteSpace($ArchivePath) -or
            -not [string]::IsNullOrWhiteSpace($TemporaryDirectory)) {
            Add-ReleaseAuditError `
                -Code 'RELEASE_AUDIT_REMOTE_EVIDENCE_INVALID' `
                -Message 'none 模式不得注入远端 evidence、archive 或临时目录。'
        }
        return
    }

    if ($RemoteValidationMode -ceq 'fixture') {
        if ([string]::IsNullOrWhiteSpace($RemoteEvidencePath) -or
            [string]::IsNullOrWhiteSpace($ArchivePath) -or
            -not [string]::IsNullOrWhiteSpace($TemporaryDirectory)) {
            Add-ReleaseAuditError `
                -Code 'RELEASE_AUDIT_REMOTE_EVIDENCE_INVALID' `
                -Message 'fixture 模式必须且只能提供 RemoteEvidencePath 与 ArchivePath。'
            return
        }
        $evidence = Read-StrictJsonDocument `
            -Path $RemoteEvidencePath `
            -ErrorCode 'RELEASE_AUDIT_REMOTE_EVIDENCE_INVALID' `
            -ErrorMessage '无法读取合成远端 evidence' `
            -ExpectedRoot object
        try {
            $archiveEvidence = Get-BoundedArchiveEvidence -Path $ArchivePath
            [void](Test-RemoteReleaseEvidence `
                -SourceLock $SourceLock `
                -Evidence $evidence `
                -ArchiveEvidence $archiveEvidence)
        }
        catch {
            Add-ReleaseAuditError `
                -Code 'RELEASE_AUDIT_REMOTE_EVIDENCE_INVALID' `
                -Message "合成 archive 验证失败：$($_.Exception.Message)"
        }
        return
    }

    if (-not [string]::IsNullOrWhiteSpace($RemoteEvidencePath) -or
        -not [string]::IsNullOrWhiteSpace($ArchivePath) -or
        [string]::IsNullOrWhiteSpace($TemporaryDirectory)) {
        Add-ReleaseAuditError `
            -Code 'RELEASE_AUDIT_REMOTE_EVIDENCE_INVALID' `
            -Message 'live 模式必须且只能提供 runner.temp 临时目录。'
        return
    }
    $resolvedTemp = [IO.Path]::GetFullPath($TemporaryDirectory)
    $relativeTemp = [IO.Path]::GetRelativePath([IO.Path]::GetFullPath($RepositoryRoot), $resolvedTemp)
    if ($relativeTemp -ceq '.' -or
        (-not $relativeTemp.StartsWith("..$([IO.Path]::DirectorySeparatorChar)", [StringComparison]::Ordinal) -and
            $relativeTemp -cne '..')) {
        Add-ReleaseAuditError `
            -Code 'RELEASE_AUDIT_REMOTE_EVIDENCE_INVALID' `
            -Message 'live 模式临时目录必须位于仓库工作树之外。'
        return
    }

    $liveResult = $null
    try {
        $liveResult = Get-LiveRemoteReleaseEvidence `
            -SourceLock $SourceLock `
            -TemporaryDirectory $resolvedTemp
        [void](Test-RemoteReleaseEvidence `
            -SourceLock $SourceLock `
            -Evidence $liveResult.evidence `
            -ArchiveEvidence $liveResult.archive)
    }
    catch {
        Add-ReleaseAuditError `
            -Code 'RELEASE_AUDIT_REMOTE_EVIDENCE_INVALID' `
            -Message "live 远端验证失败：$($_.Exception.Message)"
    }
    finally {
        if ($null -ne $liveResult -and
            (Test-Path -LiteralPath $liveResult.archive_path -PathType Leaf)) {
            Remove-Item -LiteralPath $liveResult.archive_path -Force
        }
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
if ($null -eq $headSourceLock) {
    Write-Result -Status 'invalid' -RequiresReaudit $false -ReleaseAuditChanged $false
}
$headState = Get-ReleaseAuditState -SourceLock $headSourceLock -Location '当前 source-lock'
Test-UpstreamSnapshotIntegrity -RepositoryRoot $resolvedRepositoryRoot -SourceLock $headSourceLock
if ($RemoteValidationMode -ceq 'none' -or $errors.Count -eq 0) {
    Invoke-RemoteReleaseValidation `
        -RepositoryRoot $resolvedRepositoryRoot `
        -SourceLock $headSourceLock
}

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
if ($null -eq $baseSourceLock -or $null -eq $changes) {
    Write-Result `
        -Status $headState.status `
        -RequiresReaudit $headState.requires_reaudit `
        -ReleaseAuditChanged $false
}
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
