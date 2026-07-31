[CmdletBinding()]
param(
    [string]$RepositoryRoot = (Join-Path $PSScriptRoot '../..'),
    [string]$PolicyPath = '.github/autonomous-refactor-policy.json'
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

function Test-ExactStringSet {
    param(
        [AllowNull()]$Actual,
        [Parameter(Mandatory)][string[]]$Expected
    )

    $rawActualValues = @($Actual)
    if (@($rawActualValues | Where-Object { $_ -isnot [string] }).Count -ne 0) {
        return $false
    }
    $actualValues = @($rawActualValues | Sort-Object -Unique)
    $expectedValues = @($Expected | Sort-Object -Unique)
    return [string]::Join([char]10, $actualValues) -ceq [string]::Join([char]10, $expectedValues)
}

function Test-ExactStringSequence {
    param(
        [AllowNull()]$Actual,
        [Parameter(Mandatory)][string[]]$Expected
    )

    $actualValues = @($Actual)
    if ($actualValues.Count -ne $Expected.Count) {
        return $false
    }
    for ($index = 0; $index -lt $Expected.Count; $index += 1) {
        if ($actualValues[$index] -isnot [string] -or $actualValues[$index] -cne $Expected[$index]) {
            return $false
        }
    }
    return $true
}

function Test-ExactJsonBoolean {
    param(
        [AllowNull()]$Actual,
        [Parameter(Mandatory)][bool]$Expected
    )

    return ($Actual -is [bool] -and $Actual -eq $Expected)
}

function Test-ExactJsonInt64 {
    param(
        [AllowNull()]$Actual,
        [Parameter(Mandatory)][long]$Expected
    )

    return ($Actual -is [long] -and $Actual -eq $Expected)
}

$root = [System.IO.Path]::GetFullPath($RepositoryRoot)
$resolvedPolicyPath = if ([System.IO.Path]::IsPathRooted($PolicyPath)) {
    [System.IO.Path]::GetFullPath($PolicyPath)
} else {
    [System.IO.Path]::GetFullPath((Join-Path $root $PolicyPath))
}

if (-not (Test-Path -LiteralPath $resolvedPolicyPath -PathType Leaf)) {
    Write-Result -ExitCode 10 -Value ([pscustomobject][ordered]@{
        schema_version = 1
        ok = $false
        error_code = 'AUTONOMOUS_POLICY_MISSING'
        policy_path = $resolvedPolicyPath
    })
}

try {
    $raw = [System.IO.File]::ReadAllText($resolvedPolicyPath, [Text.UTF8Encoding]::new($false))
    $policy = $raw | ConvertFrom-Json -Depth 100
} catch {
    Write-Result -ExitCode 11 -Value ([pscustomobject][ordered]@{
        schema_version = 1
        ok = $false
        error_code = 'AUTONOMOUS_POLICY_INVALID_JSON'
        policy_path = $resolvedPolicyPath
    })
}

$violations = [System.Collections.Generic.List[object]]::new()
function Add-Violation {
    param(
        [Parameter(Mandatory)][string]$Code,
        [Parameter(Mandatory)][string]$Message
    )

    $violations.Add([pscustomobject][ordered]@{
        code = $Code
        message = $Message
    }) | Out-Null
}

if ((Get-PropertyValue $policy 'schema_version') -cne 'inputcodex.autonomous-refactor-policy.v1') {
    Add-Violation 'SCHEMA_VERSION' 'schema_version 必须为 inputcodex.autonomous-refactor-policy.v1'
}
if (-not (Test-ExactJsonBoolean -Actual (Get-PropertyValue $policy 'enabled') -Expected $true)) {
    Add-Violation 'POLICY_ENABLED' '自治策略必须显式 enabled=true'
}

$authorization = Get-PropertyValue $policy 'authorization'
if ((Get-PropertyValue $authorization 'mode') -cne 'bounded-standing-v1') {
    Add-Violation 'AUTHORIZATION_MODE' '授权模式必须为 bounded-standing-v1'
}
if ((Get-PropertyValue $authorization 'owner_authorization_ref') -cne 'https://github.com/nonononull/inputcodex/issues/111') {
    Add-Violation 'OWNER_AUTHORIZATION_REF' 'owner authorization 必须固定指向 Issue #111'
}
if (-not (Test-ExactJsonBoolean -Actual (Get-PropertyValue $authorization 'exact_head_binding') -Expected $true)) {
    Add-Violation 'EXACT_HEAD_BINDING' '每次合并必须绑定精确 Final Head'
}
if (-not (Test-ExactJsonBoolean -Actual (Get-PropertyValue $authorization 'policy_hash_binding') -Expected $true) -or
    -not (Test-ExactJsonBoolean -Actual (Get-PropertyValue $authorization 'refresh_on_head_change') -Expected $true)) {
    Add-Violation 'AUTHORIZATION_REFRESH' '策略 hash 必须绑定，Head 变化后必须刷新授权证据'
}

$execution = Get-PropertyValue $policy 'execution'
if ((Get-PropertyValue $execution 'base_ref') -cne 'origin/main') {
    Add-Violation 'BASE_REF' '自治任务必须从 origin/main 开始'
}
if (-not (Test-ExactJsonInt64 -Actual (Get-PropertyValue $execution 'max_active_writers') -Expected 1) -or
    -not (Test-ExactJsonInt64 -Actual (Get-PropertyValue $execution 'max_active_product_issues') -Expected 1) -or
    -not (Test-ExactJsonInt64 -Actual (Get-PropertyValue $execution 'max_active_product_prs') -Expected 1)) {
    Add-Violation 'SINGLE_WRITER' '写入者、active 产品 Issue 和 active 产品 PR 都必须最多一个'
}
if (-not (Test-ExactJsonBoolean -Actual (Get-PropertyValue $execution 'local_full_workspace_build') -Expected $false) -or
    -not (Test-ExactJsonBoolean -Actual (Get-PropertyValue $execution 'github_hosted_full_validation') -Expected $true)) {
    Add-Violation 'BUILD_PLACEMENT' '本地不得跑全量 Workspace，完整验证必须使用 GitHub-hosted runners'
}
$expectedDecisionOrder = @(
    'performance-first',
    'release-parity',
    'minimal-read-only-slice',
    'typed-owner-before-side-effects',
    'fail-closed-and-continue'
)
if (-not (Test-ExactStringSequence -Actual (Get-PropertyValue $execution 'decision_order') -Expected $expectedDecisionOrder)) {
    Add-Violation 'DECISION_ORDER' '自动决策顺序必须完整且不可漂移'
}

$mergeGate = Get-PropertyValue $policy 'merge_gate'
if ((Get-PropertyValue $mergeGate 'method') -cne 'squash') {
    Add-Violation 'MERGE_METHOD' 'main 只允许 Squash Merge'
}
if (-not (Test-ExactJsonBoolean -Actual (Get-PropertyValue $mergeGate 'github_native_auto_merge') -Expected $false)) {
    Add-Violation 'GITHUB_AUTO_MERGE' 'GitHub 原生 auto-merge 必须保持关闭'
}
if ((Get-PropertyValue $mergeGate 'required_mergeable_state') -cne 'CLEAN' -or
    (Get-PropertyValue $mergeGate 'required_release_audit') -cne 'current' -or
    -not (Test-ExactJsonInt64 -Actual (Get-PropertyValue $mergeGate 'required_review_threads') -Expected 0)) {
    Add-Violation 'REVIEW_GATE' 'mergeable、release audit 或 Review thread 门漂移'
}
if (-not (Test-ExactJsonInt64 -Actual (Get-PropertyValue $mergeGate 'required_artifacts') -Expected 0)) {
    Add-Violation 'ARTIFACT_GATE' '成功 CI 和 Performance Artifact 必须为 0'
}
foreach ($requiredFlag in @(
    'require_origin_main_freshness',
    'require_tree_equivalence',
    'require_single_parent',
    'require_valid_github_signature'
)) {
    if (-not (Test-ExactJsonBoolean -Actual (Get-PropertyValue $mergeGate $requiredFlag) -Expected $true)) {
        Add-Violation 'POST_MERGE_GATE' "缺少合并门：$requiredFlag"
    }
}

$workflowExpectations = [ordered]@{
    'CI' = @('classify', 'governance', 'release-audit', 'linux-quality', 'windows', 'macos', 'required')
    'Performance Baseline' = @('contract', 'windows', 'macos', 'required')
}
$workflows = @(Get-PropertyValue $mergeGate 'required_workflows')
foreach ($workflowName in $workflowExpectations.Keys) {
    $matches = @($workflows | Where-Object { (Get-PropertyValue $_ 'name') -ceq $workflowName })
    if ($matches.Count -ne 1 -or
        -not (Test-ExactStringSet -Actual (Get-PropertyValue $matches[0] 'jobs') -Expected $workflowExpectations[$workflowName])) {
        Add-Violation 'WORKFLOW_GATE' "Workflow Job 集合漂移：$workflowName"
    }
}
if ($workflows.Count -ne $workflowExpectations.Count) {
    Add-Violation 'WORKFLOW_GATE' 'required_workflows 只能包含 CI 与 Performance Baseline'
}

$upstreamSync = Get-PropertyValue $policy 'upstream_sync'
$upstreamSyncTaskKindProperty = $null
$upstreamSyncTaskMarkerProperty = $null
$upstreamSyncAllowedReleaseAuditProperty = $null
$upstreamSyncRequiredWorkflowProperty = $null
$upstreamSyncRequiredJobProperty = $null
if ($null -ne $upstreamSync) {
    $upstreamSyncTaskKindProperty = $upstreamSync.PSObject.Properties['task_kind']
    $upstreamSyncTaskMarkerProperty = $upstreamSync.PSObject.Properties['task_marker']
    $upstreamSyncAllowedReleaseAuditProperty = $upstreamSync.PSObject.Properties['allowed_release_audit']
    $upstreamSyncRequiredWorkflowProperty = $upstreamSync.PSObject.Properties['required_workflow']
    $upstreamSyncRequiredJobProperty = $upstreamSync.PSObject.Properties['required_job']
}
$upstreamSyncTaskKind = $null
$upstreamSyncTaskMarker = $null
$upstreamSyncAllowedReleaseAudit = $null
$upstreamSyncRequiredWorkflow = $null
$upstreamSyncRequiredJob = $null
if ($null -ne $upstreamSyncTaskKindProperty) { $upstreamSyncTaskKind = $upstreamSyncTaskKindProperty.Value }
if ($null -ne $upstreamSyncTaskMarkerProperty) { $upstreamSyncTaskMarker = $upstreamSyncTaskMarkerProperty.Value }
if ($null -ne $upstreamSyncAllowedReleaseAuditProperty) { $upstreamSyncAllowedReleaseAudit = $upstreamSyncAllowedReleaseAuditProperty.Value }
if ($null -ne $upstreamSyncRequiredWorkflowProperty) { $upstreamSyncRequiredWorkflow = $upstreamSyncRequiredWorkflowProperty.Value }
if ($null -ne $upstreamSyncRequiredJobProperty) { $upstreamSyncRequiredJob = $upstreamSyncRequiredJobProperty.Value }
$upstreamSyncStringsValid = $upstreamSync -is [pscustomobject] -and
    $upstreamSyncTaskKind -is [string] -and
    $upstreamSyncTaskMarker -is [string] -and
    $upstreamSyncAllowedReleaseAudit -is [string] -and
    $upstreamSyncRequiredWorkflow -is [string] -and
    $upstreamSyncRequiredJob -is [string]
$upstreamSyncWorkflowMatches = @()
if ($upstreamSyncRequiredWorkflow -is [string]) {
    $upstreamSyncWorkflowMatches = @($workflows | Where-Object {
        (Get-PropertyValue $_ 'name') -ceq $upstreamSyncRequiredWorkflow
    })
}
if (-not $upstreamSyncStringsValid -or
    $upstreamSyncTaskKind -cne 'upstream-sync' -or
    $upstreamSyncTaskMarker -cne '<!-- inputcodex:autonomous-refactor-task-kind:upstream-sync:v1 -->' -or
    $upstreamSyncAllowedReleaseAudit -cne 'stale-re-audit-required' -or
    $upstreamSyncRequiredWorkflow -cne 'CI' -or
    $upstreamSyncRequiredJob -cne 'release-audit' -or
    $upstreamSyncWorkflowMatches.Count -ne 1 -or
    @(Get-PropertyValue $upstreamSyncWorkflowMatches[0] 'jobs') -cnotcontains $upstreamSyncRequiredJob) {
    Add-Violation 'UPSTREAM_SYNC_POLICY' 'upstream-sync stale 例外必须绑定精确 marker、状态和成功 release-audit Job'
}

$retry = Get-PropertyValue $policy 'retry'
$maxAttempts = Get-PropertyValue $retry 'max_attempts'
$maxIterationMinutes = Get-PropertyValue $retry 'max_iteration_minutes'
$externalWaitSeconds = Get-PropertyValue $retry 'external_wait_seconds'
if ($maxAttempts -isnot [long] -or $maxAttempts -lt 1 -or $maxAttempts -gt 3 -or
    $maxIterationMinutes -isnot [long] -or $maxIterationMinutes -lt 1 -or $maxIterationMinutes -gt 360 -or
    $externalWaitSeconds -isnot [long] -or $externalWaitSeconds -lt 30 -or $externalWaitSeconds -gt 600) {
    Add-Violation 'RETRY_BOUND' '重试次数、单轮时长和外部等待必须有界'
}

$ui = Get-PropertyValue $policy 'ui'
if ((Get-PropertyValue $ui 'owner') -cne 'Gemini' -or
    -not (Test-ExactJsonBoolean -Actual (Get-PropertyValue $ui 'fallback_allowed') -Expected $false)) {
    Add-Violation 'UI_OWNER' 'UI owner 必须为 Gemini 且不得静默 fallback'
}

$requiredHardStops = @(
    'paid-runner',
    'self-hosted-runner',
    'secret-or-signing',
    'formal-release',
    'license-conflict',
    'force-push-main',
    'delete-main',
    'ruleset-bypass'
)
$hardStops = @(Get-PropertyValue $policy 'hard_stops')
$missingHardStops = @($requiredHardStops | Where-Object { $_ -notin $hardStops })
if ($missingHardStops.Count -ne 0) {
    Add-Violation 'HARD_STOPS' "缺少硬停止：$($missingHardStops -join ',')"
}
if ((Get-PropertyValue $policy 'first_candidate') -cne 'feature.session-data.token-usage-history') {
    Add-Violation 'FIRST_CANDIDATE' '首个候选必须为 token-usage-history'
}

$normalized = $policy | ConvertTo-Json -Depth 100 -Compress
$policyHash = [Convert]::ToHexString(
    [Security.Cryptography.SHA256]::HashData(
        [Text.UTF8Encoding]::new($false).GetBytes($normalized)
    )
).ToLowerInvariant()

$result = [pscustomobject][ordered]@{
    schema_version = 1
    ok = ($violations.Count -eq 0)
    policy_path = $resolvedPolicyPath
    policy_sha256 = "sha256:$policyHash"
    upstream_sync = [pscustomobject][ordered]@{
        task_kind = $upstreamSyncTaskKind
        task_marker = $upstreamSyncTaskMarker
        allowed_release_audit = $upstreamSyncAllowedReleaseAudit
        required_workflow = $upstreamSyncRequiredWorkflow
        required_job = $upstreamSyncRequiredJob
    }
    violation_count = $violations.Count
    violations = @($violations)
}

if ($violations.Count -ne 0) {
    Write-Result -Value $result -ExitCode 12
}
Write-Result -Value $result -ExitCode 0
