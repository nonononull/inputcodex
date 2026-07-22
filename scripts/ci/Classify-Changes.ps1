[CmdletBinding()]
param(
    [Parameter(Mandatory)]
    [ValidateNotNullOrEmpty()]
    [string]$InputFile
)

Set-StrictMode -Version Latest
$ErrorActionPreference = 'Stop'

function Write-JsonResult {
    param(
        [Parameter(Mandatory)]
        $Value,

        [Parameter(Mandatory)]
        [int]$ExitCode
    )

    $json = ConvertTo-Json -InputObject $Value -Depth 20 -Compress
    [Console]::Out.WriteLine($json)
    exit $ExitCode
}

function New-InputError {
    param(
        [Parameter(Mandatory)]
        [string]$Code,

        [Parameter(Mandatory)]
        [string]$Message,

        [int]$Index = -1,

        [string]$Field = ''
    )

    [pscustomobject]@{
        code = $Code
        message = $Message
        index = $Index
        field = $Field
    }
}

function Test-RepositoryRelativePath {
    param(
        [AllowNull()]
        [AllowEmptyString()]
        [string]$Path
    )

    if ([string]::IsNullOrWhiteSpace($Path)) {
        return [pscustomobject]@{ valid = $false; reason = '路径不能为空。' }
    }

    if ($Path -match '[\x00-\x1F\x7F]') {
        return [pscustomobject]@{ valid = $false; reason = '路径不能包含控制字符。' }
    }

    if ($Path.Contains('\')) {
        return [pscustomobject]@{ valid = $false; reason = '路径必须使用正斜杠。' }
    }

    if ($Path.StartsWith('/') -or $Path -match '^[A-Za-z]:') {
        return [pscustomobject]@{ valid = $false; reason = '路径必须是仓库相对路径。' }
    }

    $segments = @($Path.Split('/'))
    if ($segments.Count -eq 0 -or $segments -contains '' -or $segments -contains '.' -or $segments -contains '..') {
        return [pscustomobject]@{ valid = $false; reason = '路径不能包含空段、当前目录或父目录段。' }
    }

    return [pscustomobject]@{ valid = $true; reason = '' }
}

function Test-DocumentationPath {
    param(
        [Parameter(Mandatory)]
        [string]$Path
    )

    if ($Path -in @('README.md', 'build.md', 'err.md')) {
        return $true
    }

    if ($Path.StartsWith('docs/', [System.StringComparison]::Ordinal)) {
        return $true
    }

    return [System.IO.Path]::GetExtension($Path).Equals('.md', [System.StringComparison]::OrdinalIgnoreCase)
}

$errors = [System.Collections.Generic.List[object]]::new()

if (-not (Test-Path -LiteralPath $InputFile -PathType Leaf)) {
    $errors.Add((New-InputError -Code 'INPUT_FILE_NOT_FOUND' -Message "输入文件不存在：$InputFile" -Field 'InputFile'))
    Write-JsonResult -Value ([pscustomobject]@{
        schema_version = 1
        ok = $false
        errors = @($errors)
    }) -ExitCode 2
}

try {
    $rawInput = Get-Content -LiteralPath $InputFile -Raw -Encoding utf8
    $jsonDocument = [System.Text.Json.JsonDocument]::Parse($rawInput)
}
catch {
    $errors.Add((New-InputError -Code 'INVALID_JSON' -Message "输入不是有效 JSON：$($_.Exception.Message)" -Field 'InputFile'))
    Write-JsonResult -Value ([pscustomobject]@{
        schema_version = 1
        ok = $false
        errors = @($errors)
    }) -ExitCode 2
}

try {
    if ($jsonDocument.RootElement.ValueKind -ne [System.Text.Json.JsonValueKind]::Array) {
        $errors.Add((New-InputError -Code 'ROOT_ARRAY_REQUIRED' -Message '输入 JSON 根节点必须是数组。' -Field 'InputFile'))
        Write-JsonResult -Value ([pscustomobject]@{
            schema_version = 1
            ok = $false
            errors = @($errors)
        }) -ExitCode 2
    }

    $changes = @($rawInput | ConvertFrom-Json -Depth 20)
}
finally {
    $jsonDocument.Dispose()
}

$normalizedChanges = [System.Collections.Generic.List[object]]::new()
$allowedStatuses = @('A', 'M', 'D', 'R', 'C')

for ($index = 0; $index -lt $changes.Count; $index += 1) {
    $change = $changes[$index]
    if ($null -eq $change -or $change -isnot [psobject]) {
        $errors.Add((New-InputError -Code 'CHANGE_OBJECT_REQUIRED' -Message '每条变更必须是 JSON 对象。' -Index $index))
        continue
    }

    $statusProperty = $change.PSObject.Properties['status']
    $pathProperty = $change.PSObject.Properties['path']
    if ($null -eq $statusProperty -or $statusProperty.Value -isnot [string] -or $statusProperty.Value -notin $allowedStatuses) {
        $errors.Add((New-InputError -Code 'INVALID_STATUS' -Message 'status 必须是 A、M、D、R 或 C。' -Index $index -Field 'status'))
        continue
    }

    if ($null -eq $pathProperty -or $pathProperty.Value -isnot [string]) {
        $errors.Add((New-InputError -Code 'INVALID_PATH' -Message 'path 必须是字符串。' -Index $index -Field 'path'))
        continue
    }

    $pathValidation = Test-RepositoryRelativePath -Path $pathProperty.Value
    if (-not $pathValidation.valid) {
        $errors.Add((New-InputError -Code 'INVALID_PATH' -Message $pathValidation.reason -Index $index -Field 'path'))
        continue
    }

    $oldPath = $null
    $oldPathProperty = $change.PSObject.Properties['old_path']
    if ($statusProperty.Value -in @('R', 'C')) {
        if ($null -eq $oldPathProperty -or $oldPathProperty.Value -isnot [string]) {
            $errors.Add((New-InputError -Code 'OLD_PATH_REQUIRED' -Message '重命名或复制记录必须包含字符串 old_path。' -Index $index -Field 'old_path'))
            continue
        }

        $oldPathValidation = Test-RepositoryRelativePath -Path $oldPathProperty.Value
        if (-not $oldPathValidation.valid) {
            $errors.Add((New-InputError -Code 'INVALID_PATH' -Message $oldPathValidation.reason -Index $index -Field 'old_path'))
            continue
        }

        $oldPath = $oldPathProperty.Value
    }
    elseif ($null -ne $oldPathProperty) {
        if ($oldPathProperty.Value -isnot [string]) {
            $errors.Add((New-InputError -Code 'INVALID_PATH' -Message 'old_path 必须是字符串。' -Index $index -Field 'old_path'))
            continue
        }

        $oldPathValidation = Test-RepositoryRelativePath -Path $oldPathProperty.Value
        if (-not $oldPathValidation.valid) {
            $errors.Add((New-InputError -Code 'INVALID_PATH' -Message $oldPathValidation.reason -Index $index -Field 'old_path'))
            continue
        }

        $oldPath = $oldPathProperty.Value
    }

    $normalizedChanges.Add([pscustomobject]@{
        status = $statusProperty.Value
        path = $pathProperty.Value
        old_path = $oldPath
    })
}

if ($errors.Count -gt 0) {
    Write-JsonResult -Value ([pscustomobject]@{
        schema_version = 1
        ok = $false
        errors = @($errors | Sort-Object index, field, code)
    }) -ExitCode 2
}

$allPaths = [System.Collections.Generic.List[string]]::new()
foreach ($change in $normalizedChanges) {
    $allPaths.Add($change.path)
    if ($null -ne $change.old_path) {
        $allPaths.Add($change.old_path)
    }
}

$changedPaths = @($allPaths | Sort-Object -Unique)
$docsPaths = @($changedPaths | Where-Object { Test-DocumentationPath -Path $_ })
$heavyPaths = @($changedPaths | Where-Object { -not (Test-DocumentationPath -Path $_) })
$hasChanges = $normalizedChanges.Count -gt 0
$docsOnly = $hasChanges -and $heavyPaths.Count -eq 0
$heavy = $heavyPaths.Count -gt 0

Write-JsonResult -Value ([pscustomobject]@{
    schema_version = 1
    ok = $true
    has_changes = $hasChanges
    docs_only = $docsOnly
    heavy = $heavy
    change_count = $normalizedChanges.Count
    changed_paths = @($changedPaths)
    docs_paths = @($docsPaths)
    heavy_paths = @($heavyPaths)
    errors = @()
}) -ExitCode 0
