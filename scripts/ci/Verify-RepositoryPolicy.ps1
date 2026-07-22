[CmdletBinding()]
param(
    [Parameter(Mandatory)]
    [ValidateNotNullOrEmpty()]
    [string]$RepositoryRoot
)

Set-StrictMode -Version Latest
$ErrorActionPreference = 'Stop'
$script:Violations = [System.Collections.Generic.List[object]]::new()

function Add-PolicyViolation {
    param(
        [Parameter(Mandatory)]
        [string]$Code,

        [Parameter(Mandatory)]
        [string]$Path,

        [Parameter(Mandatory)]
        [string]$Message
    )

    $script:Violations.Add([pscustomobject]@{
        code = $Code
        path = $Path
        message = $Message
    })
}

function Write-PolicyResult {
    param(
        [Parameter(Mandatory)]
        [string]$ResolvedRoot
    )

    $sortedViolations = @($script:Violations | Sort-Object code, path, message)
    $ok = $sortedViolations.Count -eq 0
    $result = [pscustomobject]@{
        schema_version = 1
        ok = $ok
        repository_root = $ResolvedRoot.Replace('\', '/')
        violation_count = $sortedViolations.Count
        violations = @($sortedViolations)
    }

    [Console]::Out.WriteLine((ConvertTo-Json -InputObject $result -Depth 20 -Compress))
    if ($ok) {
        exit 0
    }

    exit 2
}

function Get-RepositoryRelativePath {
    param(
        [Parameter(Mandatory)]
        [string]$Root,

        [Parameter(Mandatory)]
        [string]$Path
    )

    [System.IO.Path]::GetRelativePath($Root, $Path).Replace('\', '/')
}

function Get-PackageName {
    param(
        [Parameter(Mandatory)]
        [string]$ManifestText
    )

    $packageSection = [regex]::Match(
        $ManifestText,
        '(?ms)^\s*\[package\]\s*(?<body>.*?)(?=^\s*\[|\z)'
    )
    if (-not $packageSection.Success) {
        return $null
    }

    $nameMatch = [regex]::Match(
        $packageSection.Groups['body'].Value,
        '(?m)^\s*name\s*=\s*"(?<name>[^"]+)"\s*(?:#.*)?$'
    )
    if (-not $nameMatch.Success) {
        return $null
    }

    $nameMatch.Groups['name'].Value
}

function Get-ManifestDependencies {
    param(
        [Parameter(Mandatory)]
        [string]$ManifestText
    )

    $dependencies = [System.Collections.Generic.List[object]]::new()
    $section = ''
    $tableDependency = $null

    foreach ($line in ($ManifestText -split "`r?`n")) {
        $sectionMatch = [regex]::Match($line, '^\s*\[(?<section>[^\]]+)\]\s*(?:#.*)?$')
        if ($sectionMatch.Success) {
            $section = $sectionMatch.Groups['section'].Value.Trim()
            $tableDependency = $null
            $tableDependencyMatch = [regex]::Match(
                $section,
                '^(?:(?:target\..+\.)?(?:dependencies|dev-dependencies|build-dependencies)|workspace\.dependencies)\.(?<alias>[A-Za-z0-9_-]+)$'
            )
            if ($tableDependencyMatch.Success) {
                $alias = $tableDependencyMatch.Groups['alias'].Value
                $tableDependency = [pscustomobject]@{
                    key = $alias
                    actual_name = $alias
                    section = $section
                }
                $dependencies.Add($tableDependency)
            }

            continue
        }

        if ($null -ne $tableDependency) {
            $tablePackageMatch = [regex]::Match(
                $line,
                '^\s*package\s*=\s*"(?<package>[A-Za-z0-9_-]+)"\s*(?:#.*)?$'
            )
            if ($tablePackageMatch.Success) {
                $tableDependency.actual_name = $tablePackageMatch.Groups['package'].Value
            }

            continue
        }

        $isDependencySection = $section -in @('dependencies', 'dev-dependencies', 'build-dependencies', 'workspace.dependencies') -or
            $section -match '\.(dependencies|dev-dependencies|build-dependencies)$'
        if (-not $isDependencySection) {
            continue
        }

        $dependencyMatch = [regex]::Match(
            $line,
            '^\s*(?<key>[A-Za-z0-9_-]+)\s*=\s*(?<value>.+?)\s*(?:#.*)?$'
        )
        if (-not $dependencyMatch.Success) {
            continue
        }

        $dependencyKey = $dependencyMatch.Groups['key'].Value
        $dependencyValue = $dependencyMatch.Groups['value'].Value
        $actualName = $dependencyKey
        $packageMatch = [regex]::Match(
            $dependencyValue,
            '\bpackage\s*=\s*"(?<package>[A-Za-z0-9_-]+)"'
        )
        if ($packageMatch.Success) {
            $actualName = $packageMatch.Groups['package'].Value
        }

        $dependencies.Add([pscustomobject]@{
            key = $dependencyKey
            actual_name = $actualName
            section = $section
        })
    }

    @($dependencies)
}

function Test-WebRuntimeDependency {
    param(
        [Parameter(Mandatory)]
        [string]$Name
    )

    $normalized = $Name.ToLowerInvariant()
    if ($normalized -in @('tauri', 'wry', 'web-view', 'webview2', 'webview2-com', 'webkit2gtk', 'webkit2gtk-sys')) {
        return $true
    }

    return $normalized.StartsWith('tauri-') -or
        $normalized.StartsWith('wry-') -or
        $normalized.Contains('webview')
}

function Test-AdTelemetryDependency {
    param(
        [Parameter(Mandatory)]
        [string]$Name
    )

    $normalized = $Name.ToLowerInvariant()
    if ($normalized -in @(
        'admob',
        'google-ads',
        'google-analytics',
        'firebase-analytics',
        'sentry',
        'appcenter',
        'amplitude',
        'mixpanel',
        'segment',
        'telemetry',
        'opentelemetry'
    )) {
        return $true
    }

    return $normalized.StartsWith('sentry-') -or
        $normalized.StartsWith('opentelemetry-') -or
        $normalized.StartsWith('admob-') -or
        $normalized.StartsWith('google-ads-')
}

if (-not (Test-Path -LiteralPath $RepositoryRoot -PathType Container)) {
    Add-PolicyViolation -Code 'REPOSITORY_ROOT_NOT_FOUND' -Path $RepositoryRoot -Message '仓库根目录不存在或不是目录。'
    Write-PolicyResult -ResolvedRoot $RepositoryRoot
}

$resolvedRoot = (Resolve-Path -LiteralPath $RepositoryRoot).Path
$rootManifestPath = Join-Path $resolvedRoot 'Cargo.toml'
$expectedMembers = @(
    'apps/inputcodex-desktop',
    'crates/inputcodex-domain',
    'crates/inputcodex-application',
    'crates/inputcodex-infrastructure',
    'crates/inputcodex-platform',
    'crates/inputcodex-presentation',
    'crates/inputcodex-parity'
)
$expectedPackages = [ordered]@{
    'apps/inputcodex-desktop' = 'inputcodex-desktop'
    'crates/inputcodex-domain' = 'inputcodex-domain'
    'crates/inputcodex-application' = 'inputcodex-application'
    'crates/inputcodex-infrastructure' = 'inputcodex-infrastructure'
    'crates/inputcodex-platform' = 'inputcodex-platform'
    'crates/inputcodex-presentation' = 'inputcodex-presentation'
    'crates/inputcodex-parity' = 'inputcodex-parity'
}
$allowedLocalDependencies = @{
    'inputcodex-domain' = @()
    'inputcodex-application' = @('inputcodex-domain')
    'inputcodex-infrastructure' = @('inputcodex-application')
    'inputcodex-platform' = @('inputcodex-application')
    'inputcodex-presentation' = @('inputcodex-application')
    'inputcodex-parity' = @('inputcodex-domain', 'inputcodex-application')
    'inputcodex-desktop' = @(
        'inputcodex-application',
        'inputcodex-infrastructure',
        'inputcodex-platform',
        'inputcodex-presentation'
    )
}

if (-not (Test-Path -LiteralPath $rootManifestPath -PathType Leaf)) {
    Add-PolicyViolation -Code 'WORKSPACE_MANIFEST_MISSING' -Path 'Cargo.toml' -Message '根 Cargo.toml 不存在。'
}
else {
    $rootManifestText = Get-Content -LiteralPath $rootManifestPath -Raw -Encoding utf8
    $workspacePackageMatch = [regex]::Match(
        $rootManifestText,
        '(?ms)^\s*\[workspace\.package\]\s*(?<body>.*?)(?=^\s*\[|\z)'
    )
    $workspaceLicenseMatch = if ($workspacePackageMatch.Success) {
        [regex]::Match(
            $workspacePackageMatch.Groups['body'].Value,
            '(?m)^\s*license\s*=\s*"(?<license>[^"]+)"\s*(?:#.*)?$'
        )
    }
    else {
        [System.Text.RegularExpressions.Match]::Empty
    }

    if (-not $workspaceLicenseMatch.Success -or $workspaceLicenseMatch.Groups['license'].Value -ne 'AGPL-3.0-only') {
        Add-PolicyViolation -Code 'WORKSPACE_LICENSE_INVALID' -Path 'Cargo.toml' -Message 'Workspace 许可证必须与仓库 LICENSE 一致并固定为 AGPL-3.0-only。'
    }

    $membersMatch = [regex]::Match(
        $rootManifestText,
        '(?ms)^\s*members\s*=\s*\[(?<body>.*?)\]'
    )

    if (-not $membersMatch.Success) {
        Add-PolicyViolation -Code 'WORKSPACE_MEMBERS_INVALID' -Path 'Cargo.toml' -Message 'Workspace 必须显式声明七个 members。'
    }
    else {
        $members = @(
            [regex]::Matches($membersMatch.Groups['body'].Value, '"(?<member>[^"]+)"') |
                ForEach-Object { $_.Groups['member'].Value }
        )

        foreach ($member in $members) {
            if ($member.Contains('*')) {
                Add-PolicyViolation -Code 'WORKSPACE_MEMBER_WILDCARD' -Path 'Cargo.toml' -Message "Workspace member 禁止通配：$member"
            }

            if ($member -eq 'upstream' -or $member.StartsWith('upstream/', [System.StringComparison]::Ordinal)) {
                Add-PolicyViolation -Code 'UPSTREAM_WORKSPACE_MEMBER' -Path 'Cargo.toml' -Message "upstream 审计快照不得进入 Workspace：$member"
            }
        }

        $missingMembers = @($expectedMembers | Where-Object { $_ -notin $members })
        $extraMembers = @($members | Where-Object { $_ -notin $expectedMembers })
        if ($missingMembers.Count -gt 0 -or $extraMembers.Count -gt 0 -or $members.Count -ne $expectedMembers.Count) {
            Add-PolicyViolation -Code 'WORKSPACE_MEMBERS_INVALID' -Path 'Cargo.toml' -Message (
                "Workspace 成员不符合七成员合同；缺失=$($missingMembers -join ',')；额外=$($extraMembers -join ',')"
            )
        }
    }

    foreach ($dependency in @(Get-ManifestDependencies -ManifestText $rootManifestText)) {
        if (Test-WebRuntimeDependency -Name $dependency.actual_name) {
            Add-PolicyViolation -Code 'WEB_RUNTIME_DEPENDENCY_FORBIDDEN' -Path 'Cargo.toml' -Message "禁止 WebView/Tauri 依赖：$($dependency.actual_name)"
        }

        if (Test-AdTelemetryDependency -Name $dependency.actual_name) {
            Add-PolicyViolation -Code 'AD_TELEMETRY_DEPENDENCY_FORBIDDEN' -Path 'Cargo.toml' -Message "禁止广告或远程遥测依赖：$($dependency.actual_name)"
        }
    }
}

$expectedManifestPaths = @{}
foreach ($member in $expectedMembers) {
    $manifestPath = [System.IO.Path]::GetFullPath((Join-Path $resolvedRoot "$member/Cargo.toml"))
    $expectedManifestPaths[$manifestPath] = $member
}

foreach ($sourceRootName in @('apps', 'crates')) {
    $sourceRoot = Join-Path $resolvedRoot $sourceRootName
    if (-not (Test-Path -LiteralPath $sourceRoot -PathType Container)) {
        continue
    }

    foreach ($manifestFile in @(Get-ChildItem -LiteralPath $sourceRoot -Recurse -File -Filter 'Cargo.toml')) {
        $fullManifestPath = [System.IO.Path]::GetFullPath($manifestFile.FullName)
        if (-not $expectedManifestPaths.ContainsKey($fullManifestPath)) {
            $relativeManifestPath = Get-RepositoryRelativePath -Root $resolvedRoot -Path $fullManifestPath
            Add-PolicyViolation -Code 'UNEXPECTED_CARGO_MANIFEST' -Path $relativeManifestPath -Message '只允许七个显式 Workspace 成员清单。'
        }
    }
}

foreach ($member in $expectedMembers) {
    $manifestPath = Join-Path $resolvedRoot "$member/Cargo.toml"
    $relativeManifestPath = "$member/Cargo.toml"
    if (-not (Test-Path -LiteralPath $manifestPath -PathType Leaf)) {
        Add-PolicyViolation -Code 'MEMBER_MANIFEST_MISSING' -Path $relativeManifestPath -Message 'Workspace 成员缺少 Cargo.toml。'
        continue
    }

    $manifestText = Get-Content -LiteralPath $manifestPath -Raw -Encoding utf8
    $packageName = Get-PackageName -ManifestText $manifestText
    $expectedPackageName = $expectedPackages[$member]
    if ($packageName -ne $expectedPackageName) {
        Add-PolicyViolation -Code 'PACKAGE_NAME_INVALID' -Path $relativeManifestPath -Message "包名必须为 $expectedPackageName，实际为 $packageName。"
        continue
    }

    $allowedDependencies = @($allowedLocalDependencies[$packageName])
    foreach ($dependency in @(Get-ManifestDependencies -ManifestText $manifestText)) {
        $dependencyName = $dependency.actual_name
        if ($dependencyName -eq 'iced' -and $packageName -ne 'inputcodex-presentation') {
            Add-PolicyViolation -Code 'ICED_LAYER_VIOLATION' -Path $relativeManifestPath -Message 'Iced 只能直接依赖于 inputcodex-presentation。'
        }

        if ($dependencyName.StartsWith('inputcodex-', [System.StringComparison]::Ordinal) -and $dependencyName -notin $allowedDependencies) {
            Add-PolicyViolation -Code 'DEPENDENCY_DIRECTION_INVALID' -Path $relativeManifestPath -Message "$packageName 不允许依赖 $dependencyName。"
        }

        if (Test-WebRuntimeDependency -Name $dependencyName) {
            Add-PolicyViolation -Code 'WEB_RUNTIME_DEPENDENCY_FORBIDDEN' -Path $relativeManifestPath -Message "禁止 WebView/Tauri 依赖：$dependencyName"
        }

        if (Test-AdTelemetryDependency -Name $dependencyName) {
            Add-PolicyViolation -Code 'AD_TELEMETRY_DEPENDENCY_FORBIDDEN' -Path $relativeManifestPath -Message "禁止广告或远程遥测依赖：$dependencyName"
        }
    }
}

$forbiddenScriptExtensions = @('.ts', '.tsx', '.js', '.jsx', '.mjs', '.cjs')
$scannableTextExtensions = @('.rs', '.toml', '.json', '.yaml', '.yml', '.ron')
$githubSourcePattern = 'https?://(?:api\.)?github\.com/(?:repos/)?(?<owner>[A-Za-z0-9_.-]+)/(?<repo>[A-Za-z0-9_.-]+)(?<suffix>/[^\s]*)?'

foreach ($sourceRootName in @('apps', 'crates')) {
    $sourceRoot = Join-Path $resolvedRoot $sourceRootName
    if (-not (Test-Path -LiteralPath $sourceRoot -PathType Container)) {
        continue
    }

    foreach ($file in @(Get-ChildItem -LiteralPath $sourceRoot -Recurse -File)) {
        $relativePath = Get-RepositoryRelativePath -Root $resolvedRoot -Path $file.FullName
        $extension = $file.Extension.ToLowerInvariant()
        if ($extension -in $forbiddenScriptExtensions) {
            Add-PolicyViolation -Code 'SCRIPT_LANGUAGE_FORBIDDEN' -Path $relativePath -Message '生产目录禁止 TypeScript 或 JavaScript 文件。'
            continue
        }

        if ($extension -notin $scannableTextExtensions) {
            continue
        }

        try {
            $content = Get-Content -LiteralPath $file.FullName -Raw -Encoding utf8
        }
        catch {
            Add-PolicyViolation -Code 'POLICY_SCAN_FAILED' -Path $relativePath -Message "无法读取受控文本文件：$($_.Exception.Message)"
            continue
        }

        foreach ($match in [regex]::Matches($content, $githubSourcePattern, [System.Text.RegularExpressions.RegexOptions]::IgnoreCase)) {
            $suffix = $match.Groups['suffix'].Value.ToLowerInvariant()
            if ($suffix -notmatch '/(releases?|download|updates?)(/|$)') {
                continue
            }

            $owner = $match.Groups['owner'].Value
            $repository = $match.Groups['repo'].Value.TrimEnd('.')
            $isOwnRepository = $owner.Equals('nonononull', [System.StringComparison]::OrdinalIgnoreCase) -and
                $repository.Equals('inputcodex', [System.StringComparison]::OrdinalIgnoreCase)
            if (-not $isOwnRepository) {
                Add-PolicyViolation -Code 'UPDATE_SOURCE_FORBIDDEN' -Path $relativePath -Message "Release 或更新源必须指向 nonononull/inputcodex：$($match.Value)"
            }
        }
    }
}

Write-PolicyResult -ResolvedRoot $resolvedRoot
