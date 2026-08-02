[CmdletBinding()]
param()

Set-StrictMode -Version Latest
$ErrorActionPreference = 'Stop'

$scriptDirectory = Split-Path -Parent $PSCommandPath
$repositoryRoot = (Resolve-Path -LiteralPath (Join-Path $scriptDirectory '../..')).Path
$classifierScript = Join-Path $scriptDirectory 'Classify-Changes.ps1'
$policyScript = Join-Path $scriptDirectory 'Verify-RepositoryPolicy.ps1'
$collectorScript = Join-Path $scriptDirectory 'Collect-Changes.ps1'
$releaseAuditGateScript = Join-Path $scriptDirectory 'Verify-ReleaseAuditGate.ps1'
$autonomousPolicyScript = Join-Path $scriptDirectory 'Verify-AutonomousRefactorPolicy.ps1'
$autonomousPolicyPath = Join-Path $repositoryRoot '.github/autonomous-refactor-policy.json'
$autonomousStateScript = Join-Path $repositoryRoot 'scripts/automation/Get-AutonomousRefactorState.ps1'
$workflowPath = Join-Path $repositoryRoot '.github/workflows/ci.yml'
$performanceWorkflowPath = Join-Path $repositoryRoot '.github/workflows/performance-baseline.yml'
$performanceInvokeScript = Join-Path $repositoryRoot 'scripts/performance/Invoke-InputcodexBaseline.ps1'
$performanceTestScript = Join-Path $repositoryRoot 'scripts/performance/Test-InputcodexBaseline.ps1'
$performanceObserverScript = Join-Path $repositoryRoot 'scripts/performance/Invoke-InputcodexBudgetObservation.ps1'
$missingImplementations = @(
    @(
        $classifierScript
        $policyScript
        $releaseAuditGateScript
        $autonomousPolicyScript
        $autonomousPolicyPath
        $autonomousStateScript
    ) | Where-Object { -not (Test-Path -LiteralPath $_ -PathType Leaf) }
)

if ($missingImplementations.Count -gt 0) {
    [Console]::Error.WriteLine('CI_CONTRACT_RED_MISSING_IMPLEMENTATION')
    foreach ($missingImplementation in $missingImplementations) {
        [Console]::Error.WriteLine("missing=$missingImplementation")
    }

    exit 10
}

$script:PowerShellExecutable = (Get-Process -Id $PID).Path
$script:PassedCount = 0
$script:Failures = [System.Collections.Generic.List[string]]::new()
$script:AutonomousPolicySha256 = $null
$testRoot = Join-Path ([System.IO.Path]::GetTempPath()) ("inputcodex-ci-contract-{0}" -f [guid]::NewGuid().ToString('N'))
New-Item -ItemType Directory -Path $testRoot -Force | Out-Null

function Invoke-ContractTest {
    param(
        [Parameter(Mandatory)]
        [string]$Name,

        [Parameter(Mandatory)]
        [scriptblock]$Body
    )

    try {
        & $Body
        $script:PassedCount += 1
        Write-Host "PASS $Name"
    }
    catch {
        $message = "FAIL $Name :: $($_.Exception.Message)"
        $script:Failures.Add($message)
        Write-Host $message
    }
}

function Assert-Equal {
    param(
        [Parameter(Mandatory)]
        $Expected,

        [Parameter(Mandatory)]
        $Actual,

        [Parameter(Mandatory)]
        [string]$Message
    )

    if ($Expected -ne $Actual) {
        throw "$Message；期望=<$Expected>，实际=<$Actual>"
    }
}

function Assert-True {
    param(
        [Parameter(Mandatory)]
        [bool]$Condition,

        [Parameter(Mandatory)]
        [string]$Message
    )

    if (-not $Condition) {
        throw $Message
    }
}

function Assert-Contains {
    param(
        [Parameter(Mandatory)]
        [object[]]$Collection,

        [Parameter(Mandatory)]
        $Expected,

        [Parameter(Mandatory)]
        [string]$Message
    )

    if ($Collection -notcontains $Expected) {
        throw "$Message；缺少=<$Expected>"
    }
}

function Get-AutonomousStateFunctionSource {
    param([Parameter(Mandatory)][string]$Name)

    $source = Get-Content -LiteralPath $autonomousStateScript -Raw
    $tokens = $null
    $parseErrors = $null
    $ast = [System.Management.Automation.Language.Parser]::ParseInput(
        $source,
        [ref]$tokens,
        [ref]$parseErrors
    )
    Assert-Equal -Expected 0 -Actual @($parseErrors).Count -Message '自治状态脚本必须可供真实 helper 合同提取'

    $functions = @($ast.FindAll({
        param($node)
        $node -is [System.Management.Automation.Language.FunctionDefinitionAst] -and
        $node.Name -ceq $Name
    }, $true))
    Assert-Equal -Expected 1 -Actual $functions.Count -Message "自治状态脚本必须存在唯一生产 helper：$Name"
    return $functions[0].Extent.Text
}

function Invoke-ChildScript {
    param(
        [Parameter(Mandatory)]
        [string]$Path,

        [string[]]$Arguments = @()
    )

    $outputLines = @(& $script:PowerShellExecutable -NoLogo -NoProfile -File $Path @Arguments 2>&1)
    $exitCode = $LASTEXITCODE
    $output = ($outputLines | ForEach-Object { $_.ToString() }) -join [Environment]::NewLine
    $json = $null

    if (-not [string]::IsNullOrWhiteSpace($output)) {
        try {
            $json = $output | ConvertFrom-Json -Depth 100
        }
        catch {
            $json = $null
        }
    }

    [pscustomobject]@{
        ExitCode = $exitCode
        Output = $output
        Json = $json
    }
}

function Write-JsonFile {
    param(
        [Parameter(Mandatory)]
        [string]$Path,

        [Parameter(Mandatory)]
        $Value
    )

    $json = ConvertTo-Json -InputObject $Value -Depth 20
    Set-Content -LiteralPath $Path -Value $json -Encoding utf8NoBOM
}

function New-ValidAutonomousRefactorPolicy {
    [pscustomobject][ordered]@{
        schema_version = 'inputcodex.autonomous-refactor-policy.v1'
        enabled = $true
        authorization = [pscustomobject][ordered]@{
            mode = 'bounded-standing-v1'
            owner_authorization_ref = 'https://github.com/nonononull/inputcodex/issues/111'
            exact_head_binding = $true
            policy_hash_binding = $true
            refresh_on_head_change = $true
        }
        execution = [pscustomobject][ordered]@{
            base_ref = 'origin/main'
            max_active_writers = 1
            max_active_product_issues = 1
            max_active_product_prs = 1
            local_full_workspace_build = $false
            github_hosted_full_validation = $true
            decision_order = @(
                'performance-first'
                'release-parity'
                'minimal-read-only-slice'
                'typed-owner-before-side-effects'
                'fail-closed-and-continue'
            )
        }
        merge_gate = [pscustomobject][ordered]@{
            method = 'squash'
            github_native_auto_merge = $false
            required_mergeable_state = 'CLEAN'
            required_release_audit = 'current'
            required_review_threads = 0
            required_artifacts = 0
            require_origin_main_freshness = $true
            require_tree_equivalence = $true
            require_single_parent = $true
            require_valid_github_signature = $true
            required_workflows = @(
                [pscustomobject][ordered]@{
                    name = 'CI'
                    workflow_id = 318067078
                    path = '.github/workflows/ci.yml'
                    events = @('pull_request', 'push')
                    jobs = @(
                        'classify'
                        'governance'
                        'release-audit'
                        'linux-quality'
                        'windows'
                        'macos'
                        'required'
                    )
                }
                [pscustomobject][ordered]@{
                    name = 'Performance Baseline'
                    workflow_id = 320393981
                    path = '.github/workflows/performance-baseline.yml'
                    events = @('pull_request', 'push')
                    jobs = @('contract', 'windows', 'macos', 'required')
                }
            )
        }
        upstream_sync = [pscustomobject][ordered]@{
            task_kind = 'upstream-sync'
            task_marker = '<!-- inputcodex:autonomous-refactor-task-kind:upstream-sync:v1 -->'
            allowed_release_audit = 'stale-re-audit-required'
            required_workflow = 'CI'
            required_job = 'release-audit'
        }
        candidate_exhaustion = [pscustomobject][ordered]@{
            task_kind = 'candidate-exhausted'
            task_marker = '<!-- inputcodex:autonomous-refactor-task-kind:candidate-exhausted:v1 -->'
            required_label = 'status:needs-owner-decision'
            state = 'blocked-candidate-exhausted'
            next_action = 'await-owner-decision'
        }
        retry = [pscustomobject][ordered]@{
            max_attempts = 3
            max_iteration_minutes = 180
            external_wait_seconds = 120
        }
        ui = [pscustomobject][ordered]@{
            owner = 'Gemini'
            fallback_allowed = $false
        }
        hard_stops = @(
            'paid-runner'
            'self-hosted-runner'
            'secret-or-signing'
            'formal-release'
            'license-conflict'
            'force-push-main'
            'delete-main'
            'ruleset-bypass'
            'candidate-exhausted'
        )
        first_candidate = 'feature.session-data.token-usage-history'
    }
}

function Copy-AutonomousRefactorPolicy {
    param([Parameter(Mandatory)]$Policy)

    return ($Policy | ConvertTo-Json -Depth 30 | ConvertFrom-Json -Depth 30)
}

function Invoke-AutonomousPolicyCase {
    param(
        [Parameter(Mandatory)][string]$Name,
        [Parameter(Mandatory)]$Policy
    )

    $caseRoot = Join-Path $testRoot ("autonomous-policy-{0}" -f $Name)
    New-Item -ItemType Directory -Path $caseRoot -Force | Out-Null
    $policyPath = Join-Path $caseRoot 'policy.json'
    Write-JsonFile -Path $policyPath -Value $Policy
    Invoke-ChildScript -Path $autonomousPolicyScript -Arguments @(
        '-RepositoryRoot', $caseRoot,
        '-PolicyPath', $policyPath
    )
}

function Assert-AutonomousPolicyFailure {
    param(
        [Parameter(Mandatory)]$Result,
        [Parameter(Mandatory)][string]$ExpectedCode
    )

    Assert-Equal -Expected 12 -Actual $Result.ExitCode -Message "自治策略应失败，输出=$($Result.Output)"
    Assert-True -Condition ($null -ne $Result.Json) -Message "自治策略失败必须输出 JSON，输出=$($Result.Output)"
    Assert-Equal -Expected $false -Actual $Result.Json.ok -Message '自治策略失败必须标记 ok=false'
    Assert-Contains -Collection @($Result.Json.violations.code) -Expected $ExpectedCode -Message '自治策略失败码必须稳定'
}

Invoke-ContractTest -Name '合法无人值守重构策略通过并输出规范化哈希' -Body {
    $result = Invoke-ChildScript -Path $autonomousPolicyScript -Arguments @(
        '-RepositoryRoot', $repositoryRoot,
        '-PolicyPath', $autonomousPolicyPath
    )
    Assert-Equal -Expected 0 -Actual $result.ExitCode -Message "合法自治策略应通过，输出=$($result.Output)"
    Assert-True -Condition ($null -ne $result.Json) -Message "合法自治策略必须输出 JSON，输出=$($result.Output)"
    Assert-Equal -Expected $true -Actual $result.Json.ok -Message '合法自治策略必须标记 ok=true'
    Assert-True -Condition ([string]$result.Json.policy_sha256 -match '^sha256:[0-9a-f]{64}$') -Message '自治策略必须输出规范化 SHA-256'
    $script:AutonomousPolicySha256 = [string]$result.Json.policy_sha256
    Assert-Equal -Expected 'upstream-sync' -Actual $result.Json.upstream_sync.task_kind `
        -Message '策略输出必须投影 typed upstream-sync task kind'
    Assert-Equal -Expected '<!-- inputcodex:autonomous-refactor-task-kind:upstream-sync:v1 -->' `
        -Actual $result.Json.upstream_sync.task_marker -Message '策略输出必须投影精确 upstream-sync marker'
    Assert-Equal -Expected 'stale-re-audit-required' -Actual $result.Json.upstream_sync.allowed_release_audit `
        -Message '策略输出必须投影唯一允许的 stale 状态'
    Assert-Equal -Expected 'CI' -Actual $result.Json.upstream_sync.required_workflow `
        -Message '策略输出必须投影承载 release-audit 的 Workflow'
    Assert-Equal -Expected 'release-audit' -Actual $result.Json.upstream_sync.required_job `
        -Message '策略输出必须投影成功 release-audit Job 要求'
    Assert-Equal -Expected 2 -Actual @($result.Json.required_workflows).Count `
        -Message '策略输出必须投影两个强身份 Workflow'
    Assert-Equal -Expected 318067078 -Actual $result.Json.required_workflows[0].workflow_id `
        -Message '策略输出必须投影 CI workflow_id'
    Assert-Equal -Expected '.github/workflows/ci.yml' -Actual $result.Json.required_workflows[0].path `
        -Message '策略输出必须投影 CI path'
    Assert-Equal -Expected ([string]::Join([char]10, @('pull_request', 'push'))) `
        -Actual ([string]::Join([char]10, @($result.Json.required_workflows[0].events))) `
        -Message '策略输出必须投影 CI 的 PR/push 事件集合'
}

Invoke-ContractTest -Name '拒绝缺失的无人值守重构策略文件' -Body {
    $missingPath = Join-Path $testRoot 'missing-autonomous-policy.json'
    $result = Invoke-ChildScript -Path $autonomousPolicyScript -Arguments @(
        '-RepositoryRoot', $testRoot,
        '-PolicyPath', $missingPath
    )
    Assert-Equal -Expected 10 -Actual $result.ExitCode -Message "缺失策略应使用稳定退出码，输出=$($result.Output)"
    Assert-Equal -Expected 'AUTONOMOUS_POLICY_MISSING' -Actual $result.Json.error_code -Message '缺失策略错误码必须稳定'
}

Invoke-ContractTest -Name '拒绝非 bounded standing authorization' -Body {
    $policy = Copy-AutonomousRefactorPolicy (New-ValidAutonomousRefactorPolicy)
    $policy.authorization.mode = 'per-pr-owner'
    Assert-AutonomousPolicyFailure -Result (Invoke-AutonomousPolicyCase -Name 'authorization' -Policy $policy) -ExpectedCode 'AUTHORIZATION_MODE'
}

Invoke-ContractTest -Name '拒绝字符串伪装自治策略布尔与整数类型' -Body {
    $policy = Copy-AutonomousRefactorPolicy (New-ValidAutonomousRefactorPolicy)
    $policy.enabled = 'true'
    $policy.authorization.exact_head_binding = 'true'
    $policy.authorization.policy_hash_binding = 'true'
    $policy.authorization.refresh_on_head_change = 'true'
    $policy.execution.max_active_writers = '1'
    $policy.execution.local_full_workspace_build = 'false'
    $policy.merge_gate.github_native_auto_merge = 'false'
    $policy.merge_gate.required_review_threads = '0'
    $policy.merge_gate.required_artifacts = '0'
    $policy.merge_gate.require_single_parent = 'true'
    $policy.ui.fallback_allowed = 'false'

    $result = Invoke-AutonomousPolicyCase -Name 'strict-json-types' -Policy $policy
    foreach ($code in @(
        'POLICY_ENABLED',
        'EXACT_HEAD_BINDING',
        'AUTHORIZATION_REFRESH',
        'SINGLE_WRITER',
        'BUILD_PLACEMENT',
        'GITHUB_AUTO_MERGE',
        'REVIEW_GATE',
        'ARTIFACT_GATE',
        'POST_MERGE_GATE',
        'UI_OWNER'
    )) {
        Assert-AutonomousPolicyFailure -Result $result -ExpectedCode $code
    }
}

Invoke-ContractTest -Name '拒绝无人值守决策优先级重排' -Body {
    $policy = Copy-AutonomousRefactorPolicy (New-ValidAutonomousRefactorPolicy)
    $policy.execution.decision_order = @(
        'release-parity',
        'performance-first',
        'minimal-read-only-slice',
        'typed-owner-before-side-effects',
        'fail-closed-and-continue'
    )
    Assert-AutonomousPolicyFailure -Result (Invoke-AutonomousPolicyCase -Name 'decision-order' -Policy $policy) -ExpectedCode 'DECISION_ORDER'
}

Invoke-ContractTest -Name '拒绝 GitHub 原生 auto-merge' -Body {
    $policy = Copy-AutonomousRefactorPolicy (New-ValidAutonomousRefactorPolicy)
    $policy.merge_gate.github_native_auto_merge = $true
    Assert-AutonomousPolicyFailure -Result (Invoke-AutonomousPolicyCase -Name 'auto-merge' -Policy $policy) -ExpectedCode 'GITHUB_AUTO_MERGE'
}

Invoke-ContractTest -Name '拒绝非 Squash 或非精确 Head 合并' -Body {
    $policy = Copy-AutonomousRefactorPolicy (New-ValidAutonomousRefactorPolicy)
    $policy.merge_gate.method = 'rebase'
    $policy.authorization.exact_head_binding = $false
    $result = Invoke-AutonomousPolicyCase -Name 'merge-method' -Policy $policy
    Assert-AutonomousPolicyFailure -Result $result -ExpectedCode 'MERGE_METHOD'
    Assert-Contains -Collection @($result.Json.violations.code) -Expected 'EXACT_HEAD_BINDING' -Message '精确 Head 缺失必须被拒绝'
}

Invoke-ContractTest -Name '拒绝多个写入者或无界重试' -Body {
    $policy = Copy-AutonomousRefactorPolicy (New-ValidAutonomousRefactorPolicy)
    $policy.execution.max_active_writers = 2
    $policy.retry.max_attempts = 0
    $result = Invoke-AutonomousPolicyCase -Name 'writer-retry' -Policy $policy
    Assert-AutonomousPolicyFailure -Result $result -ExpectedCode 'SINGLE_WRITER'
    Assert-Contains -Collection @($result.Json.violations.code) -Expected 'RETRY_BOUND' -Message '重试上限缺失必须被拒绝'
}

Invoke-ContractTest -Name '拒绝缺失的 CI Performance 与零 Artifact 门' -Body {
    $policy = Copy-AutonomousRefactorPolicy (New-ValidAutonomousRefactorPolicy)
    $policy.merge_gate.required_artifacts = 1
    $policy.merge_gate.required_workflows = @($policy.merge_gate.required_workflows | Where-Object { $_.name -ne 'Performance Baseline' })
    $result = Invoke-AutonomousPolicyCase -Name 'merge-gates' -Policy $policy
    Assert-AutonomousPolicyFailure -Result $result -ExpectedCode 'ARTIFACT_GATE'
    Assert-Contains -Collection @($result.Json.violations.code) -Expected 'WORKFLOW_GATE' -Message 'Performance 门缺失必须被拒绝'
}

Invoke-ContractTest -Name '拒绝漂移的 Workflow 强身份绑定' -Body {
    foreach ($case in @(
        [pscustomobject]@{ Name = 'id'; Property = 'workflow_id'; Value = 999 },
        [pscustomobject]@{ Name = 'path'; Property = 'path'; Value = '.github/workflows/fake.yml' },
        [pscustomobject]@{ Name = 'events'; Property = 'events'; Value = @('workflow_dispatch') }
    )) {
        $policy = Copy-AutonomousRefactorPolicy (New-ValidAutonomousRefactorPolicy)
        $policy.merge_gate.required_workflows[0].($case.Property) = $case.Value
        Assert-AutonomousPolicyFailure `
            -Result (Invoke-AutonomousPolicyCase -Name "workflow-identity-$($case.Name)" -Policy $policy) `
            -ExpectedCode 'WORKFLOW_IDENTITY'
    }
}

Invoke-ContractTest -Name '拒绝漂移的 upstream-sync stale 例外' -Body {
    $cases = @(
        [pscustomobject]@{ Name = 'kind'; Property = 'task_kind'; Value = 'refactor' },
        [pscustomobject]@{ Name = 'marker'; Property = 'task_marker'; Value = '<!-- inputcodex:upstream-sync:v1 -->' },
        [pscustomobject]@{ Name = 'status'; Property = 'allowed_release_audit'; Value = 'current' },
        [pscustomobject]@{ Name = 'workflow'; Property = 'required_workflow'; Value = 'Performance Baseline' },
        [pscustomobject]@{ Name = 'job'; Property = 'required_job'; Value = 'required' }
    )
    foreach ($case in $cases) {
        $policy = Copy-AutonomousRefactorPolicy (New-ValidAutonomousRefactorPolicy)
        $policy.upstream_sync.($case.Property) = $case.Value
        Assert-AutonomousPolicyFailure `
            -Result (Invoke-AutonomousPolicyCase -Name "upstream-sync-$($case.Name)" -Policy $policy) `
            -ExpectedCode 'UPSTREAM_SYNC_POLICY'
    }
    foreach ($case in $cases) {
        $policy = Copy-AutonomousRefactorPolicy (New-ValidAutonomousRefactorPolicy)
        $policy.upstream_sync.($case.Property) = [object[]]@(
            (New-ValidAutonomousRefactorPolicy).upstream_sync.($case.Property)
        )
        Assert-AutonomousPolicyFailure `
            -Result (Invoke-AutonomousPolicyCase -Name "upstream-sync-array-$($case.Name)" -Policy $policy) `
            -ExpectedCode 'UPSTREAM_SYNC_POLICY'
    }
    $invalidShapes = Copy-AutonomousRefactorPolicy (New-ValidAutonomousRefactorPolicy)
    $invalidShapes.upstream_sync.task_kind = $null
    $invalidShapes.upstream_sync.task_marker = [pscustomobject]@{ value = 'upstream-sync' }
    Assert-AutonomousPolicyFailure `
        -Result (Invoke-AutonomousPolicyCase -Name 'upstream-sync-null-object' -Policy $invalidShapes) `
        -ExpectedCode 'UPSTREAM_SYNC_POLICY'
}

Invoke-ContractTest -Name '拒绝漂移的候选耗尽终态策略' -Body {
    $cases = @(
        [pscustomobject]@{ Name = 'kind'; Property = 'task_kind'; Value = 'refactor' }
        [pscustomobject]@{ Name = 'marker'; Property = 'task_marker'; Value = '<!-- inputcodex:autonomous-refactor-task-kind:candidate-exhausted:v2 -->' }
        [pscustomobject]@{ Name = 'label'; Property = 'required_label'; Value = 'status:blocked' }
        [pscustomobject]@{ Name = 'state'; Property = 'state'; Value = 'blocked-hard-stop' }
        [pscustomobject]@{ Name = 'action'; Property = 'next_action'; Value = 'stop' }
    )
    foreach ($case in $cases) {
        $policy = Copy-AutonomousRefactorPolicy (New-ValidAutonomousRefactorPolicy)
        $policy.candidate_exhaustion.($case.Property) = $case.Value
        Assert-AutonomousPolicyFailure `
            -Result (Invoke-AutonomousPolicyCase -Name "candidate-exhaustion-$($case.Name)" -Policy $policy) `
            -ExpectedCode 'CANDIDATE_EXHAUSTION_POLICY'
    }

    $arrayValue = Copy-AutonomousRefactorPolicy (New-ValidAutonomousRefactorPolicy)
    $arrayValue.candidate_exhaustion.task_kind = [object[]]@('candidate-exhausted')
    Assert-AutonomousPolicyFailure `
        -Result (Invoke-AutonomousPolicyCase -Name 'candidate-exhaustion-array' -Policy $arrayValue) `
        -ExpectedCode 'CANDIDATE_EXHAUSTION_POLICY'

    $objectArray = Copy-AutonomousRefactorPolicy (New-ValidAutonomousRefactorPolicy)
    $objectArray.candidate_exhaustion = [object[]]@($objectArray.candidate_exhaustion)
    Assert-AutonomousPolicyFailure `
        -Result (Invoke-AutonomousPolicyCase -Name 'candidate-exhaustion-object-array' -Policy $objectArray) `
        -ExpectedCode 'CANDIDATE_EXHAUSTION_POLICY'

    $extraField = Copy-AutonomousRefactorPolicy (New-ValidAutonomousRefactorPolicy)
    $extraField.candidate_exhaustion | Add-Member -NotePropertyName gate_5_product_complete -NotePropertyValue $true
    Assert-AutonomousPolicyFailure `
        -Result (Invoke-AutonomousPolicyCase -Name 'candidate-exhaustion-extra-field' -Policy $extraField) `
        -ExpectedCode 'CANDIDATE_EXHAUSTION_POLICY'
}

Invoke-ContractTest -Name '拒绝非 Gemini UI owner 或 UI fallback' -Body {
    $policy = Copy-AutonomousRefactorPolicy (New-ValidAutonomousRefactorPolicy)
    $policy.ui.owner = 'Codex'
    $policy.ui.fallback_allowed = $true
    Assert-AutonomousPolicyFailure -Result (Invoke-AutonomousPolicyCase -Name 'ui-owner' -Policy $policy) -ExpectedCode 'UI_OWNER'
}

Invoke-ContractTest -Name '拒绝缺失永久主干保护的硬停止集合' -Body {
    $policy = Copy-AutonomousRefactorPolicy (New-ValidAutonomousRefactorPolicy)
    $policy.hard_stops = @($policy.hard_stops | Where-Object { $_ -notin @('force-push-main', 'delete-main', 'ruleset-bypass') })
    Assert-AutonomousPolicyFailure -Result (Invoke-AutonomousPolicyCase -Name 'hard-stops' -Policy $policy) -ExpectedCode 'HARD_STOPS'
}

Invoke-ContractTest -Name '拒绝缺失候选耗尽硬停止' -Body {
    $policy = Copy-AutonomousRefactorPolicy (New-ValidAutonomousRefactorPolicy)
    $policy.hard_stops = @($policy.hard_stops | Where-Object { $_ -cne 'candidate-exhausted' })
    Assert-AutonomousPolicyFailure `
        -Result (Invoke-AutonomousPolicyCase -Name 'candidate-exhaustion-hard-stop' -Policy $policy) `
        -ExpectedCode 'HARD_STOPS'

    $caseDrift = Copy-AutonomousRefactorPolicy (New-ValidAutonomousRefactorPolicy)
    $caseDrift.hard_stops = @($caseDrift.hard_stops | ForEach-Object {
        if ($_ -ceq 'candidate-exhausted') { 'Candidate-Exhausted' } else { $_ }
    })
    Assert-AutonomousPolicyFailure `
        -Result (Invoke-AutonomousPolicyCase -Name 'candidate-exhaustion-hard-stop-case' -Policy $caseDrift) `
        -ExpectedCode 'HARD_STOPS'

    $invalidMember = Copy-AutonomousRefactorPolicy (New-ValidAutonomousRefactorPolicy)
    $invalidMember.hard_stops = [object[]]@($invalidMember.hard_stops + [pscustomobject]@{ value = 'extra-stop' })
    Assert-AutonomousPolicyFailure `
        -Result (Invoke-AutonomousPolicyCase -Name 'candidate-exhaustion-hard-stop-object' -Policy $invalidMember) `
        -ExpectedCode 'HARD_STOPS'
}

function New-ValidAutonomousStateSnapshot {
    [pscustomobject][ordered]@{
        schema_version = 'inputcodex.autonomous-refactor-state-snapshot.v1'
        github_available = $true
        paseo_available = $true
        release_audit = 'current'
        observed_origin_main = 'aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa'
        observed_remote_main = 'aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa'
        expected_base = 'aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa'
        worktree_head = 'bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb'
        branch = 'codex/issue-111-autonomous-refactor-control-plane'
        worktree_clean = $true
        active_writer_count = 0
        repository_settings = [pscustomobject][ordered]@{
            allow_auto_merge = $false
            allow_squash_merge = $true
            allow_merge_commit = $false
            allow_rebase_merge = $false
            default_branch = 'main'
        }
        actual_scope_count = 12
        actual_scope_hash = 'sha256:5d1f609ca2a5913e4e5df21f0fd04d6de2c6731cdd71d641812fbee80b5ad713'
        active_issues = @()
        active_prs = @()
        merged_prs = @()
    }
}

function New-SuccessfulAutonomousWorkflowEvidence {
    param(
        [Parameter(Mandatory)][string]$Name,
        [Parameter(Mandatory)][string[]]$Jobs,
        [Parameter(Mandatory)][long]$RunId,
        [string]$HeadOid = 'bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb',
        [ValidateSet('pull_request', 'push')][string]$Event = 'pull_request',
        [long]$PullRequestNumber = 112
    )

    $workflowId = if ($Name -ceq 'CI') { 318067078L } else { 320393981L }
    $workflowPath = if ($Name -ceq 'CI') {
        '.github/workflows/ci.yml'
    } else {
        '.github/workflows/performance-baseline.yml'
    }

    [pscustomobject][ordered]@{
        name = $Name
        workflow_id = $workflowId
        workflow_path = $workflowPath
        event = $Event
        pull_request_number = $PullRequestNumber
        run_id = $RunId
        head_oid = $HeadOid
        status = 'completed'
        conclusion = 'success'
        artifact_count = 0
        jobs = @($Jobs | ForEach-Object {
            [pscustomobject][ordered]@{
                name = $_
                status = 'completed'
                conclusion = 'success'
                head_oid = $HeadOid
            }
        })
    }
}

function New-MergeReadyAutonomousPr {
    [pscustomobject][ordered]@{
        number = 112
        url = 'https://github.com/nonononull/inputcodex/pull/112'
        base_ref = 'main'
        head_oid = 'bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb'
        author_login = 'nonononull'
        head_owner_login = 'nonononull'
        is_draft = $false
        merge_state = 'CLEAN'
        review_thread_count = 0
        evidence = [pscustomobject][ordered]@{
            valid = $true
            task_kind = 'refactor'
            tracking_issue_ref = 'https://github.com/nonononull/inputcodex/issues/111'
            standing_authorization_ref = 'https://github.com/nonononull/inputcodex/issues/111'
            policy_sha256 = $script:AutonomousPolicySha256
            scope_count = 12
            scope_hash = 'sha256:5d1f609ca2a5913e4e5df21f0fd04d6de2c6731cdd71d641812fbee80b5ad713'
            final_head = 'bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb'
            independent_review_status = 'passed'
            independent_review_ref = 'https://github.com/nonononull/inputcodex/pull/112#issuecomment-1'
        }
        review_attestation = [pscustomobject][ordered]@{
            valid = $true
            final_head = 'bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb'
            status = 'passed'
            ref = 'https://github.com/nonononull/inputcodex/pull/112#issuecomment-1'
        }
        workflow_runs = @(
            (New-SuccessfulAutonomousWorkflowEvidence -Name 'CI' -RunId 1001 -Jobs @('classify', 'governance', 'release-audit', 'linux-quality', 'windows', 'macos', 'required')),
            (New-SuccessfulAutonomousWorkflowEvidence -Name 'Performance Baseline' -RunId 1002 -Jobs @('contract', 'windows', 'macos', 'required'))
        )
    }
}

function New-MergeReadyAutonomousStateSnapshot {
    $snapshot = Copy-AutonomousStateSnapshot (New-ValidAutonomousStateSnapshot)
    $snapshot.active_issues = @([pscustomobject]@{
        number = 111
        url = 'https://github.com/nonononull/inputcodex/issues/111'
        author_login = 'nonononull'
        planning_evidence = [pscustomobject][ordered]@{
            valid = $true
            task_kind = 'refactor'
            scope_count = 12
            scope_hash = 'sha256:5d1f609ca2a5913e4e5df21f0fd04d6de2c6731cdd71d641812fbee80b5ad713'
            ref = 'https://github.com/nonononull/inputcodex/issues/111#issuecomment-5139215915'
        }
    })
    $snapshot.active_prs = @(New-MergeReadyAutonomousPr)
    return $snapshot
}

function New-PostMergeAutonomousStateSnapshot {
    $snapshot = New-MergeReadyAutonomousStateSnapshot
    $sourcePr = New-MergeReadyAutonomousPr
    $snapshot.active_prs = @()
    $snapshot.observed_origin_main = 'cccccccccccccccccccccccccccccccccccccccc'
    $snapshot.observed_remote_main = 'cccccccccccccccccccccccccccccccccccccccc'
    $snapshot.merged_prs = @([pscustomobject][ordered]@{
        number = 112
        url = 'https://github.com/nonononull/inputcodex/pull/112'
        head_oid = 'bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb'
        merge_commit_oid = 'cccccccccccccccccccccccccccccccccccccccc'
        author_login = 'nonononull'
        head_owner_login = 'nonononull'
        evidence = $sourcePr.evidence
        review_attestation = $sourcePr.review_attestation
        post_merge = [pscustomobject][ordered]@{
            parent_count = 1
            merge_tree_oid = 'dddddddddddddddddddddddddddddddddddddddd'
            head_tree_oid = 'dddddddddddddddddddddddddddddddddddddddd'
            signature_valid = $true
        }
        workflow_runs = @(
            (New-SuccessfulAutonomousWorkflowEvidence -Name 'CI' -RunId 2001 -HeadOid 'cccccccccccccccccccccccccccccccccccccccc' -Event push -PullRequestNumber 0 -Jobs @('classify', 'governance', 'release-audit', 'linux-quality', 'windows', 'macos', 'required')),
            (New-SuccessfulAutonomousWorkflowEvidence -Name 'Performance Baseline' -RunId 2002 -HeadOid 'cccccccccccccccccccccccccccccccccccccccc' -Event push -PullRequestNumber 0 -Jobs @('contract', 'windows', 'macos', 'required'))
        )
    })
    return $snapshot
}

function Copy-AutonomousStateSnapshot {
    param([Parameter(Mandatory)]$Snapshot)

    return ($Snapshot | ConvertTo-Json -Depth 30 | ConvertFrom-Json -Depth 30)
}

function Invoke-AutonomousStateCase {
    param(
        [Parameter(Mandatory)][string]$Name,
        [Parameter(Mandatory)]$Snapshot
    )

    $caseRoot = Join-Path $testRoot ("autonomous-state-{0}" -f $Name)
    New-Item -ItemType Directory -Path $caseRoot -Force | Out-Null
    $snapshotPath = Join-Path $caseRoot 'snapshot.json'
    Write-JsonFile -Path $snapshotPath -Value $Snapshot
    Invoke-ChildScript -Path $autonomousStateScript -Arguments @(
        '-RepositoryRoot', $repositoryRoot,
        '-PolicyPath', $autonomousPolicyPath,
        '-SnapshotPath', $snapshotPath,
        '-ReportOnly'
    )
}

function Assert-AutonomousState {
    param(
        [Parameter(Mandatory)]$Result,
        [Parameter(Mandatory)][string]$ExpectedState,
        [Parameter(Mandatory)][string]$ExpectedAction
    )

    Assert-Equal -Expected 0 -Actual $Result.ExitCode -Message "自治状态解析应成功，输出=$($Result.Output)"
    Assert-True -Condition ($null -ne $Result.Json) -Message "自治状态解析必须输出 JSON，输出=$($Result.Output)"
    Assert-Equal -Expected $true -Actual $Result.Json.ok -Message '自治状态解析必须标记 ok=true'
    Assert-Equal -Expected $ExpectedState -Actual $Result.Json.state -Message '自治状态分类漂移'
    Assert-Equal -Expected $ExpectedAction -Actual $Result.Json.next_action -Message '自治下一动作漂移'
}

Invoke-ContractTest -Name '无人值守拒绝伪装或缺失类型的状态快照字段' -Body {
    foreach ($case in @(
        [pscustomobject]@{ Name = 'github-bool'; Property = 'github_available'; Value = 'true' },
        [pscustomobject]@{ Name = 'paseo-bool'; Property = 'paseo_available'; Value = 'true' },
        [pscustomobject]@{ Name = 'worktree-bool'; Property = 'worktree_clean'; Value = 'true' },
        [pscustomobject]@{ Name = 'release-audit-string'; Property = 'release_audit'; Value = 1 },
        [pscustomobject]@{ Name = 'branch-string'; Property = 'branch'; Value = $null },
        [pscustomobject]@{ Name = 'issues-array'; Property = 'active_issues'; Value = $null },
        [pscustomobject]@{ Name = 'prs-array'; Property = 'active_prs'; Value = $null }
    )) {
        $snapshot = Copy-AutonomousStateSnapshot (New-ValidAutonomousStateSnapshot)
        $snapshot.($case.Property) = $case.Value
        $result = Invoke-AutonomousStateCase -Name "invalid-type-$($case.Name)" -Snapshot $snapshot
        Assert-Equal -Expected 11 -Actual $result.ExitCode -Message "状态字段类型错误必须拒绝：$($case.Name)，输出=$($result.Output)"
        Assert-Equal -Expected 'AUTONOMOUS_STATE_INVALID_SNAPSHOT' -Actual $result.Json.error_code -Message "状态字段类型错误码必须稳定：$($case.Name)"
    }
}

Invoke-ContractTest -Name '无人值守空闲状态选择下一候选' -Body {
    $snapshot = Copy-AutonomousStateSnapshot (New-ValidAutonomousStateSnapshot)
    $snapshot.branch = 'main'
    $result = Invoke-AutonomousStateCase -Name 'idle' -Snapshot $snapshot
    Assert-AutonomousState -Result $result -ExpectedState 'idle-select-candidate' -ExpectedAction 'select-candidate'
}

Invoke-ContractTest -Name '无人值守单一活动 Issue 恢复规划' -Body {
    $snapshot = Copy-AutonomousStateSnapshot (New-ValidAutonomousStateSnapshot)
    $snapshot.active_issues = @([pscustomobject]@{ number = 111; url = 'https://github.com/nonononull/inputcodex/issues/111'; author_login = 'nonononull' })
    $result = Invoke-AutonomousStateCase -Name 'issue' -Snapshot $snapshot
    Assert-AutonomousState -Result $result -ExpectedState 'active-issue-planning' -ExpectedAction 'resume-issue'
    Assert-Equal -Expected 111 -Actual $result.Json.active_issue.number -Message '活动 Issue 编号必须保留'
}

Invoke-ContractTest -Name '无人值守候选耗尽进入所有者决策终态' -Body {
    $snapshot = Copy-AutonomousStateSnapshot (New-ValidAutonomousStateSnapshot)
    $snapshot.branch = 'main'
    $snapshot.worktree_head = 'aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa'
    $snapshot.actual_scope_count = 0L
    $snapshot.actual_scope_hash = 'sha256:01ba4719c80b6fe911b091a7c05124b64eeece964e09c058ef8f9805daca546b'
    $snapshot.active_issues = @([pscustomobject][ordered]@{
        number = 900L
        url = 'https://github.com/nonononull/inputcodex/issues/900'
        author_login = 'nonononull'
        task_kind = 'candidate-exhausted'
        labels = [object[]]@('gate:5', 'status:needs-owner-decision')
    })

    $result = Invoke-AutonomousStateCase -Name 'candidate-exhausted' -Snapshot $snapshot
    Assert-AutonomousState -Result $result `
        -ExpectedState 'blocked-candidate-exhausted' `
        -ExpectedAction 'await-owner-decision'
    Assert-Equal -Expected 900 -Actual $result.Json.active_issue.number -Message '候选耗尽 Issue 必须保留'
    Assert-Equal -Expected 0 -Actual @($result.Json.reason_codes).Count -Message '合法候选耗尽终态不得携带硬停止原因'
}

Invoke-ContractTest -Name '无人值守候选耗尽漂移必须 fail closed' -Body {
    function New-CandidateExhaustedSnapshot {
        $value = Copy-AutonomousStateSnapshot (New-ValidAutonomousStateSnapshot)
        $value.branch = 'main'
        $value.worktree_head = 'aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa'
        $value.actual_scope_count = 0L
        $value.actual_scope_hash = 'sha256:01ba4719c80b6fe911b091a7c05124b64eeece964e09c058ef8f9805daca546b'
        $value.active_issues = @([pscustomobject][ordered]@{
            number = 900L
            url = 'https://github.com/nonononull/inputcodex/issues/900'
            author_login = 'nonononull'
            task_kind = 'candidate-exhausted'
            labels = [object[]]@('gate:5', 'status:needs-owner-decision')
        })
        return $value
    }

    $missingLabel = New-CandidateExhaustedSnapshot
    $missingLabel.active_issues[0].labels = [object[]]@('gate:5')
    $missingLabelResult = Invoke-AutonomousStateCase -Name 'candidate-exhausted-label' -Snapshot $missingLabel
    Assert-AutonomousState -Result $missingLabelResult -ExpectedState 'blocked-hard-stop' -ExpectedAction 'stop'
    Assert-Contains -Collection @($missingLabelResult.Json.reason_codes) `
        -Expected 'CANDIDATE_EXHAUSTED_LABEL_INVALID' `
        -Message 'required label 缺失必须稳定阻断'

    foreach ($case in @('branch', 'dirty', 'head', 'scope', 'release')) {
        $snapshot = New-CandidateExhaustedSnapshot
        switch ($case) {
            'branch' { $snapshot.branch = 'codex/issue-900-invalid' }
            'dirty' { $snapshot.worktree_clean = $false }
            'head' { $snapshot.worktree_head = 'bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb' }
            'scope' {
                $snapshot.actual_scope_count = 1L
                $snapshot.actual_scope_hash = 'sha256:aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa'
            }
            'release' { $snapshot.release_audit = 'stale-re-audit-required' }
        }
        $result = Invoke-AutonomousStateCase -Name "candidate-exhausted-$case" -Snapshot $snapshot
        Assert-AutonomousState -Result $result -ExpectedState 'blocked-hard-stop' -ExpectedAction 'stop'
        Assert-Contains -Collection @($result.Json.reason_codes) `
            -Expected 'CANDIDATE_EXHAUSTED_REPOSITORY_STATE_INVALID' `
            -Message "候选耗尽仓库漂移必须稳定阻断：$case"
    }

    $delivery = New-CandidateExhaustedSnapshot
    $delivery.active_prs = @([pscustomobject][ordered]@{
        number = 901L
        url = 'https://github.com/nonononull/inputcodex/pull/901'
        base_ref = 'main'
        head_oid = 'aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa'
        author_login = 'nonononull'
        head_owner_login = 'nonononull'
    })
    $deliveryResult = Invoke-AutonomousStateCase -Name 'candidate-exhausted-delivery' -Snapshot $delivery
    Assert-AutonomousState -Result $deliveryResult -ExpectedState 'blocked-hard-stop' -ExpectedAction 'stop'
    Assert-Contains -Collection @($deliveryResult.Json.reason_codes) `
        -Expected 'CANDIDATE_EXHAUSTED_DELIVERY_PRESENT' `
        -Message '候选耗尽 Issue 不得绑定 PR'

    $delivered = New-CandidateExhaustedSnapshot
    $delivered.merged_prs = @([pscustomobject][ordered]@{
        author_login = 'nonononull'
        head_owner_login = 'nonononull'
        merge_commit_oid = 'aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa'
        evidence = [pscustomobject][ordered]@{
            valid = $true
            tracking_issue_ref = 'https://github.com/nonononull/inputcodex/issues/900'
        }
    })
    $deliveredResult = Invoke-AutonomousStateCase -Name 'candidate-exhausted-delivered' -Snapshot $delivered
    Assert-AutonomousState -Result $deliveredResult -ExpectedState 'blocked-hard-stop' -ExpectedAction 'stop'
    Assert-Contains -Collection @($deliveredResult.Json.reason_codes) `
        -Expected 'CANDIDATE_EXHAUSTED_DELIVERY_PRESENT' `
        -Message '候选耗尽 Issue 不得关联已交付 PR'
}

Invoke-ContractTest -Name '无人值守 live 仅接受精确 typed marker' -Body {
    Invoke-Expression (Get-AutonomousStateFunctionSource -Name 'Get-PropertyProjection')
    Invoke-Expression (Get-AutonomousStateFunctionSource -Name 'Test-AutonomousIssueTaskMarkerPresence')
    Invoke-Expression (Get-AutonomousStateFunctionSource -Name 'Get-AutonomousIssueTaskKind')
    $upstreamMarker = '<!-- inputcodex:autonomous-refactor-task-kind:upstream-sync:v1 -->'
    $candidateMarker = '<!-- inputcodex:autonomous-refactor-task-kind:candidate-exhausted:v1 -->'
    $uppercasePrefixMarker = '<!-- INPUTCODEX:AUTONOMOUS-REFACTOR-TASK-KIND:CANDIDATE-EXHAUSTED:V1 -->'
    $codeBlockBody = [string]::Join("`n", @(
        '<!-- inputcodex:autonomous-refactor-task:v1 -->',
        '```md',
        $candidateMarker,
        '```'
    ))
    $lateMarkerBody = [string]::Join("`n", @(
        '<!-- inputcodex:autonomous-refactor-task:v1 -->',
        '正文',
        $candidateMarker
    ))
    $emptyLabels = (Get-PropertyProjection ([pscustomobject]@{ labels = [object[]]@() }) 'labels').value
    $singleLabels = (Get-PropertyProjection ([pscustomobject]@{ labels = [object[]]@('gate:5') }) 'labels').value

    Assert-True -Condition ($emptyLabels -is [System.Array] -and $emptyLabels.Count -eq 0) `
        -Message '空 label 数组不得折叠为 null'
    Assert-True -Condition ($singleLabels -is [System.Array] -and $singleLabels.Count -eq 1) `
        -Message '单 label 数组不得折叠为标量'
    Assert-Equal -Expected $true `
        -Actual (Test-AutonomousIssueTaskMarkerPresence -Body $candidateMarker) `
        -Message '只有 typed marker 的 Issue 也必须进入解析器并 fail-closed'
    Assert-Equal -Expected $true `
        -Actual (Test-AutonomousIssueTaskMarkerPresence -Body '<!-- inputcodex:autonomous-refactor-task-kind:unknown:v1 -->') `
        -Message '未知 typed marker 不得被 live 预筛忽略'
    Assert-Equal -Expected $true `
        -Actual (Test-AutonomousIssueTaskMarkerPresence -Body $uppercasePrefixMarker) `
        -Message '大小写漂移 marker 不得被 live 预筛忽略'
    Assert-Equal -Expected $false `
        -Actual (Test-AutonomousIssueTaskMarkerPresence -Body '普通 Issue 正文') `
        -Message '无自治 marker 的普通 Issue 不得进入状态控制面'

    Assert-Equal -Expected 'refactor' `
        -Actual (Get-AutonomousIssueTaskKind -Body '<!-- inputcodex:autonomous-refactor-task:v1 -->' -UpstreamSyncTaskMarker $upstreamMarker -CandidateExhaustedTaskMarker $candidateMarker) `
        -Message '普通自治 Issue 不得得到 upstream-sync task kind'
    Assert-Equal -Expected 'upstream-sync' `
        -Actual (Get-AutonomousIssueTaskKind -Body "<!-- inputcodex:autonomous-refactor-task:v1 -->`n$upstreamMarker" -UpstreamSyncTaskMarker $upstreamMarker -CandidateExhaustedTaskMarker $candidateMarker) `
        -Message '精确 upstream-sync marker 必须得到对应 task kind'
    Assert-Equal -Expected 'candidate-exhausted' `
        -Actual (Get-AutonomousIssueTaskKind -Body "<!-- inputcodex:autonomous-refactor-task:v1 -->`n$candidateMarker" -UpstreamSyncTaskMarker $upstreamMarker -CandidateExhaustedTaskMarker $candidateMarker) `
        -Message '精确 candidate-exhausted marker 必须得到对应 task kind'
    Assert-Equal -Expected 'invalid' `
        -Actual (Get-AutonomousIssueTaskKind -Body '<!-- inputcodex:autonomous-refactor-task-kind:unknown:v1 -->' -UpstreamSyncTaskMarker $upstreamMarker -CandidateExhaustedTaskMarker $candidateMarker) `
        -Message '未知 typed marker 必须 fail-closed'
    Assert-Equal -Expected 'invalid' `
        -Actual (Get-AutonomousIssueTaskKind -Body '<!-- inputcodex:autonomous-refactor-task-kind:candidate-exhausted:v2 -->' -UpstreamSyncTaskMarker $upstreamMarker -CandidateExhaustedTaskMarker $candidateMarker) `
        -Message '未知 marker 版本必须 fail-closed'
    Assert-Equal -Expected 'invalid' `
        -Actual (Get-AutonomousIssueTaskKind -Body '<!-- inputcodex:autonomous-refactor-task-kind:Candidate-Exhausted:v1 -->' -UpstreamSyncTaskMarker $upstreamMarker -CandidateExhaustedTaskMarker $candidateMarker) `
        -Message 'marker 大小写漂移必须 fail-closed'
    Assert-Equal -Expected 'invalid' `
        -Actual (Get-AutonomousIssueTaskKind -Body $uppercasePrefixMarker -UpstreamSyncTaskMarker $upstreamMarker -CandidateExhaustedTaskMarker $candidateMarker) `
        -Message 'marker 前缀大小写漂移必须 fail-closed'
    Assert-Equal -Expected 'invalid' `
        -Actual (Get-AutonomousIssueTaskKind -Body "$candidateMarker`n$candidateMarker" -UpstreamSyncTaskMarker $upstreamMarker -CandidateExhaustedTaskMarker $candidateMarker) `
        -Message '重复 typed marker 必须 fail-closed'
    Assert-Equal -Expected 'invalid' `
        -Actual (Get-AutonomousIssueTaskKind `
            -Body $codeBlockBody `
            -UpstreamSyncTaskMarker $upstreamMarker `
            -CandidateExhaustedTaskMarker $candidateMarker) `
        -Message 'Markdown 代码块中的 typed marker 必须 fail-closed'
    Assert-Equal -Expected 'invalid' `
        -Actual (Get-AutonomousIssueTaskKind `
            -Body $lateMarkerBody `
            -UpstreamSyncTaskMarker $upstreamMarker `
            -CandidateExhaustedTaskMarker $candidateMarker) `
        -Message '非顶部 header 的 typed marker 必须 fail-closed'

    $liveSource = Get-AutonomousStateFunctionSource -Name 'Get-LiveSnapshot'
    Assert-True -Condition $liveSource.Contains('Test-AutonomousIssueTaskMarkerPresence') `
        -Message 'live Issue 预筛必须把 typed marker 交给严格解析器'
    Assert-True -Condition $liveSource.Contains('Get-AutonomousIssueTaskKind') `
        -Message 'live Issue 投影必须消费 typed task kind helper'
    Assert-True -Condition $liveSource.Contains("Get-PropertyProjection `$issue 'labels'") `
        -Message 'live Issue 投影必须结构化读取 label'
}

Invoke-ContractTest -Name '无人值守 live Workflow Run 必须绑定强身份与关联 PR' -Body {
    Invoke-Expression (Get-AutonomousStateFunctionSource -Name 'Get-PropertyValue')
    Invoke-Expression (Get-AutonomousStateFunctionSource -Name 'Test-GitHubWorkflowPullRequestAssociation')
    Invoke-Expression (Get-AutonomousStateFunctionSource -Name 'Test-GitHubWorkflowRunIdentity')

    $expectation = [pscustomobject][ordered]@{
        name = 'CI'
        workflow_id = 318067078L
        path = '.github/workflows/ci.yml'
    }
    $run = [pscustomobject][ordered]@{
        name = 'CI'
        workflow_id = 318067078L
        path = '.github/workflows/ci.yml'
        event = 'pull_request'
        head_sha = 'bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb'
        pull_requests = @([pscustomobject][ordered]@{
            number = 112L
            head = [pscustomobject]@{ sha = 'bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb' }
            base = [pscustomobject]@{ ref = 'main' }
        })
    }
    Assert-Equal -Expected $true -Actual (Test-GitHubWorkflowRunIdentity `
        -Run $run -Expectation $expectation -HeadOid $run.head_sha -Event pull_request -PullRequestNumber 112) `
        -Message '真实 CI Run 身份必须匹配'

    foreach ($case in @(
        [pscustomobject]@{ Name = 'id'; Property = 'workflow_id'; Value = 999L },
        [pscustomobject]@{ Name = 'path'; Property = 'path'; Value = '.github/workflows/fake.yml' },
        [pscustomobject]@{ Name = 'event'; Property = 'event'; Value = 'push' }
    )) {
        $fake = $run | ConvertTo-Json -Depth 20 | ConvertFrom-Json -Depth 20
        $fake.($case.Property) = $case.Value
        Assert-Equal -Expected $false -Actual (Test-GitHubWorkflowRunIdentity `
            -Run $fake -Expectation $expectation -HeadOid $run.head_sha -Event pull_request -PullRequestNumber 112) `
            -Message "伪 Workflow Run 必须被拒绝：$($case.Name)"
    }
    $wrongPr = $run | ConvertTo-Json -Depth 20 | ConvertFrom-Json -Depth 20
    $wrongPr.pull_requests[0].number = 999L
    Assert-Equal -Expected $false -Actual (Test-GitHubWorkflowRunIdentity `
        -Run $wrongPr -Expectation $expectation -HeadOid $run.head_sha -Event pull_request -PullRequestNumber 112) `
        -Message '错误关联 PR 必须被拒绝'
}

Invoke-ContractTest -Name '无人值守 PR evidence 拒绝数组伪装 task kind' -Body {
    Invoke-Expression (Get-AutonomousStateFunctionSource -Name 'Get-PropertyValue')
    Invoke-Expression (Get-AutonomousStateFunctionSource -Name 'Get-PropertyProjection')
    Invoke-Expression (Get-AutonomousStateFunctionSource -Name 'Get-AutonomousPrBodyEvidence')

    $evidence = [pscustomobject][ordered]@{
        schema_version = 'inputcodex.autonomous-refactor-evidence.v1'
        task_kind = [object[]]@('upstream-sync')
        tracking_issue_ref = 'https://github.com/nonononull/inputcodex/issues/116'
        standing_authorization_ref = 'https://github.com/nonononull/inputcodex/issues/111'
        policy_sha256 = $script:AutonomousPolicySha256
        scope_count = 44L
        scope_hash = 'sha256:d321e4b09e887e7f84ed7047d37ab49c2708a14314813bc20d452615ad4ea8bc'
        final_head = 'bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb'
        independent_review_status = 'passed'
        independent_review_ref = 'https://github.com/nonononull/inputcodex/pull/118#issuecomment-1'
    }
    $json = $evidence | ConvertTo-Json -Depth 10 -Compress
    $body = "<!-- inputcodex:autonomous-refactor-evidence:v1 $json -->"
    Assert-Equal -Expected $false -Actual (Get-AutonomousPrBodyEvidence -Body $body).valid `
        -Message 'PR evidence 的单元素数组不得伪装 task kind 字符串'

    $evidence.task_kind = 'upstream-sync'
    $json = $evidence | ConvertTo-Json -Depth 10 -Compress
    $body = "<!-- inputcodex:autonomous-refactor-evidence:v1 $json -->"
    $valid = Get-AutonomousPrBodyEvidence -Body $body
    Assert-Equal -Expected $true -Actual $valid.valid -Message '标量 upstream-sync PR evidence 必须通过'
    Assert-Equal -Expected 'upstream-sync' -Actual $valid.task_kind -Message 'PR evidence 必须保留标量 task kind'
}

Invoke-ContractTest -Name '无人值守脏工作树恢复执行而不重复建任务' -Body {
    $snapshot = Copy-AutonomousStateSnapshot (New-ValidAutonomousStateSnapshot)
    $snapshot.worktree_clean = $false
    $snapshot.active_issues = @([pscustomobject]@{ number = 111; url = 'https://github.com/nonononull/inputcodex/issues/111'; author_login = 'nonononull' })
    Assert-AutonomousState -Result (Invoke-AutonomousStateCase -Name 'worktree' -Snapshot $snapshot) -ExpectedState 'active-worktree-execution' -ExpectedAction 'resume-worktree'
}

Invoke-ContractTest -Name '无人值守单一活动 PR 恢复 Review CI' -Body {
    $snapshot = Copy-AutonomousStateSnapshot (New-ValidAutonomousStateSnapshot)
    $snapshot.active_issues = @([pscustomobject]@{ number = 111; url = 'https://github.com/nonononull/inputcodex/issues/111'; author_login = 'nonononull' })
    $snapshot.active_prs = @([pscustomobject]@{
        number = 112
        url = 'https://github.com/nonononull/inputcodex/pull/112'
        base_ref = 'main'
        head_oid = 'bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb'
        author_login = 'nonononull'
        head_owner_login = 'nonononull'
    })
    $result = Invoke-AutonomousStateCase -Name 'pr' -Snapshot $snapshot
    Assert-AutonomousState -Result $result -ExpectedState 'active-pr-review-ci' -ExpectedAction 'resume-pr'
    Assert-Equal -Expected 112 -Actual $result.Json.active_pr.number -Message '活动 PR 编号必须保留'
}

Invoke-ContractTest -Name '无人值守完整 Final Head 证据进入精确合并就绪' -Body {
    $result = Invoke-AutonomousStateCase -Name 'merge-ready' -Snapshot (New-MergeReadyAutonomousStateSnapshot)
    Assert-AutonomousState -Result $result -ExpectedState 'merge-ready-exact-head' -ExpectedAction 'squash-merge-exact-head'
    Assert-Equal -Expected 0 -Actual @($result.Json.merge_gate_pending).Count -Message '精确合并就绪不得残留 pending gate'
}

Invoke-ContractTest -Name '无人值守任一 Final Head 合并门失败均只恢复 PR' -Body {
    foreach ($case in @(
        [pscustomobject]@{ Name = 'draft'; Code = 'PR_DRAFT' },
        [pscustomobject]@{ Name = 'merge-state'; Code = 'PR_MERGE_STATE' },
        [pscustomobject]@{ Name = 'review-thread'; Code = 'REVIEW_THREADS' },
        [pscustomobject]@{ Name = 'repository-settings'; Code = 'REPOSITORY_MERGE_SETTINGS' },
        [pscustomobject]@{ Name = 'evidence-head'; Code = 'EVIDENCE_HEAD' },
        [pscustomobject]@{ Name = 'evidence-policy'; Code = 'EVIDENCE_POLICY' },
        [pscustomobject]@{ Name = 'evidence-scope'; Code = 'EVIDENCE_SCOPE' },
        [pscustomobject]@{ Name = 'approved-scope'; Code = 'EVIDENCE_SCOPE' },
        [pscustomobject]@{ Name = 'independent-review'; Code = 'INDEPENDENT_REVIEW' },
        [pscustomobject]@{ Name = 'review-attestation'; Code = 'INDEPENDENT_REVIEW' },
        [pscustomobject]@{ Name = 'ci-artifact'; Code = 'WORKFLOW_CI' },
        [pscustomobject]@{ Name = 'performance-job'; Code = 'WORKFLOW_PERFORMANCE_BASELINE' },
        [pscustomobject]@{ Name = 'ci-workflow-id'; Code = 'WORKFLOW_CI' },
        [pscustomobject]@{ Name = 'ci-workflow-path'; Code = 'WORKFLOW_CI' },
        [pscustomobject]@{ Name = 'ci-workflow-event'; Code = 'WORKFLOW_CI' },
        [pscustomobject]@{ Name = 'ci-workflow-pr'; Code = 'WORKFLOW_CI' }
    )) {
        $snapshot = New-MergeReadyAutonomousStateSnapshot
        switch ($case.Name) {
            'draft' { $snapshot.active_prs[0].is_draft = $true }
            'merge-state' { $snapshot.active_prs[0].merge_state = 'BLOCKED' }
            'review-thread' { $snapshot.active_prs[0].review_thread_count = 1 }
            'repository-settings' { $snapshot.repository_settings.allow_auto_merge = $true }
            'evidence-head' { $snapshot.active_prs[0].evidence.final_head = 'cccccccccccccccccccccccccccccccccccccccc' }
            'evidence-policy' { $snapshot.active_prs[0].evidence.policy_sha256 = 'sha256:aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa' }
            'evidence-scope' { $snapshot.active_prs[0].evidence.scope_count = 11 }
            'approved-scope' { $snapshot.active_issues[0].planning_evidence.scope_count = 11 }
            'independent-review' { $snapshot.active_prs[0].evidence.independent_review_status = 'failed' }
            'review-attestation' { $snapshot.active_prs[0].review_attestation.final_head = 'cccccccccccccccccccccccccccccccccccccccc' }
            'ci-artifact' { $snapshot.active_prs[0].workflow_runs[0].artifact_count = 1 }
            'performance-job' { $snapshot.active_prs[0].workflow_runs[1].jobs[0].conclusion = 'failure' }
            'ci-workflow-id' { $snapshot.active_prs[0].workflow_runs[0].workflow_id = 999 }
            'ci-workflow-path' { $snapshot.active_prs[0].workflow_runs[0].workflow_path = '.github/workflows/fake.yml' }
            'ci-workflow-event' { $snapshot.active_prs[0].workflow_runs[0].event = 'push' }
            'ci-workflow-pr' { $snapshot.active_prs[0].workflow_runs[0].pull_request_number = 999 }
        }
        $result = Invoke-AutonomousStateCase -Name "merge-gate-$($case.Name)" -Snapshot $snapshot
        Assert-AutonomousState -Result $result -ExpectedState 'active-pr-review-ci' -ExpectedAction 'resume-pr'
        Assert-Contains -Collection @($result.Json.merge_gate_pending) -Expected $case.Code -Message "Final Head 合并门失败必须可诊断：$($case.Name)"
    }
}

Invoke-ContractTest -Name '无人值守识别合并后主干验证与收口就绪' -Body {
    $ready = Invoke-AutonomousStateCase -Name 'post-merge-ready' -Snapshot (New-PostMergeAutonomousStateSnapshot)
    Assert-AutonomousState -Result $ready -ExpectedState 'post-merge-verification' -ExpectedAction 'close-issue-and-archive'
    Assert-Equal -Expected 0 -Actual @($ready.Json.post_merge_gate_pending).Count -Message '合并后收口就绪不得残留 pending gate'

    $pendingSnapshot = New-PostMergeAutonomousStateSnapshot
    $pendingSnapshot.merged_prs[0].workflow_runs[1].artifact_count = 1
    $pending = Invoke-AutonomousStateCase -Name 'post-merge-pending' -Snapshot $pendingSnapshot
    Assert-AutonomousState -Result $pending -ExpectedState 'post-merge-verification' -ExpectedAction 'verify-main'
    Assert-Contains -Collection @($pending.Json.post_merge_gate_pending) -Expected 'POST_MERGE_WORKFLOW_PERFORMANCE_BASELINE' -Message '合并后 Performance Artifact 漂移必须保持验证状态'

    $identityPendingSnapshot = New-PostMergeAutonomousStateSnapshot
    $identityPendingSnapshot.merged_prs[0].workflow_runs[0].event = 'pull_request'
    $identityPending = Invoke-AutonomousStateCase -Name 'post-merge-workflow-identity' -Snapshot $identityPendingSnapshot
    Assert-AutonomousState -Result $identityPending -ExpectedState 'post-merge-verification' -ExpectedAction 'verify-main'
    Assert-Contains -Collection @($identityPending.Json.post_merge_gate_pending) -Expected 'POST_MERGE_WORKFLOW_CI' `
        -Message '合并后 CI 必须绑定 push 事件'

    $originPendingSnapshot = New-PostMergeAutonomousStateSnapshot
    $originPendingSnapshot.observed_origin_main = 'aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa'
    $originPending = Invoke-AutonomousStateCase -Name 'post-merge-origin-pending' -Snapshot $originPendingSnapshot
    Assert-AutonomousState -Result $originPending -ExpectedState 'post-merge-verification' -ExpectedAction 'verify-main'
    Assert-Contains -Collection @($originPending.Json.post_merge_gate_pending) -Expected 'POST_MERGE_ORIGIN_MAIN' -Message '合并后 origin/main 未刷新不得关闭 Issue'
}

Invoke-ContractTest -Name '无人值守 PR 存在时脏工作树优先恢复执行' -Body {
    $snapshot = Copy-AutonomousStateSnapshot (New-ValidAutonomousStateSnapshot)
    $snapshot.worktree_clean = $false
    $snapshot.active_issues = @([pscustomobject]@{ number = 111; url = 'https://github.com/nonononull/inputcodex/issues/111'; author_login = 'nonononull' })
    $snapshot.active_prs = @([pscustomobject]@{
        number = 112
        url = 'https://github.com/nonononull/inputcodex/pull/112'
        base_ref = 'main'
        head_oid = 'bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb'
        author_login = 'nonononull'
        head_owner_login = 'nonononull'
    })
    Assert-AutonomousState -Result (Invoke-AutonomousStateCase -Name 'dirty-pr' -Snapshot $snapshot) -ExpectedState 'active-worktree-execution' -ExpectedAction 'resume-worktree'
}

Invoke-ContractTest -Name '无人值守阻断受保护或无关活动分支写入' -Body {
    $protected = Copy-AutonomousStateSnapshot (New-ValidAutonomousStateSnapshot)
    $protected.branch = 'main'
    $protected.worktree_clean = $false
    $protected.active_issues = @([pscustomobject]@{ number = 111; url = 'https://github.com/nonononull/inputcodex/issues/111'; author_login = 'nonononull' })
    $protectedResult = Invoke-AutonomousStateCase -Name 'protected-dirty' -Snapshot $protected
    Assert-AutonomousState -Result $protectedResult -ExpectedState 'blocked-hard-stop' -ExpectedAction 'stop'
    Assert-Contains -Collection @($protectedResult.Json.reason_codes) -Expected 'PROTECTED_BRANCH_DIRTY' -Message 'main 脏树必须阻断'

    $unrelated = Copy-AutonomousStateSnapshot (New-ValidAutonomousStateSnapshot)
    $unrelated.branch = 'codex/unrelated-task'
    $unrelated.active_issues = @([pscustomobject]@{ number = 111; url = 'https://github.com/nonononull/inputcodex/issues/111'; author_login = 'nonononull' })
    $unrelatedResult = Invoke-AutonomousStateCase -Name 'unrelated-branch' -Snapshot $unrelated
    Assert-AutonomousState -Result $unrelatedResult -ExpectedState 'blocked-hard-stop' -ExpectedAction 'stop'
    Assert-Contains -Collection @($unrelatedResult.Json.reason_codes) -Expected 'ISSUE_BRANCH_MISMATCH' -Message '活动 Issue 与分支不一致必须阻断'
}

Invoke-ContractTest -Name '无人值守拒绝多个活动 writer' -Body {
    $snapshot = Copy-AutonomousStateSnapshot (New-ValidAutonomousStateSnapshot)
    $snapshot.active_writer_count = 2
    $result = Invoke-AutonomousStateCase -Name 'writers' -Snapshot $snapshot
    Assert-AutonomousState -Result $result -ExpectedState 'blocked-hard-stop' -ExpectedAction 'stop'
    Assert-Contains -Collection @($result.Json.reason_codes) -Expected 'MULTIPLE_WRITERS' -Message '多个 writer 必须稳定阻断'
}

Invoke-ContractTest -Name '无人值守拒绝重复活动 Issue 或 PR' -Body {
    $snapshot = Copy-AutonomousStateSnapshot (New-ValidAutonomousStateSnapshot)
    $snapshot.active_issues = @(
        [pscustomobject]@{ number = 111; url = 'issue-111'; author_login = 'nonononull' },
        [pscustomobject]@{ number = 113; url = 'issue-113'; author_login = 'nonononull' }
    )
    $snapshot.active_prs = @(
        [pscustomobject]@{ number = 112; url = 'pr-112'; base_ref = 'main'; head_oid = 'b' * 40; author_login = 'nonononull'; head_owner_login = 'nonononull' },
        [pscustomobject]@{ number = 114; url = 'pr-114'; base_ref = 'main'; head_oid = 'c' * 40; author_login = 'nonononull'; head_owner_login = 'nonononull' }
    )
    $result = Invoke-AutonomousStateCase -Name 'duplicates' -Snapshot $snapshot
    Assert-AutonomousState -Result $result -ExpectedState 'blocked-hard-stop' -ExpectedAction 'stop'
    Assert-Contains -Collection @($result.Json.reason_codes) -Expected 'MULTIPLE_ACTIVE_ISSUES' -Message '重复 Issue 必须稳定阻断'
    Assert-Contains -Collection @($result.Json.reason_codes) -Expected 'MULTIPLE_ACTIVE_PRS' -Message '重复 PR 必须稳定阻断'
}

Invoke-ContractTest -Name '无人值守阻断 stale release audit' -Body {
    $snapshot = Copy-AutonomousStateSnapshot (New-ValidAutonomousStateSnapshot)
    $snapshot.release_audit = 'stale-re-audit-required'
    $result = Invoke-AutonomousStateCase -Name 'release-audit' -Snapshot $snapshot
    Assert-AutonomousState -Result $result -ExpectedState 'blocked-hard-stop' -ExpectedAction 'stop'
    Assert-Contains -Collection @($result.Json.reason_codes) -Expected 'RELEASE_AUDIT_STALE' -Message 'stale release audit 必须稳定阻断'
}

Invoke-ContractTest -Name '无人值守仅允许 typed upstream-sync 在 stale 下推进' -Body {
    $snapshot = Copy-AutonomousStateSnapshot (New-ValidAutonomousStateSnapshot)
    $snapshot.release_audit = 'stale-re-audit-required'
    $snapshot.branch = 'codex/issue-116-upstream-v1-2-44-sync'
    $snapshot.worktree_clean = $false
    $snapshot.active_issues = @([pscustomobject]@{
        number = 116
        url = 'https://github.com/nonononull/inputcodex/issues/116'
        author_login = 'nonononull'
        task_kind = 'upstream-sync'
    })
    Assert-AutonomousState -Result (Invoke-AutonomousStateCase -Name 'stale-upstream-sync' -Snapshot $snapshot) `
        -ExpectedState 'active-worktree-execution' -ExpectedAction 'resume-worktree'

    $invalid = Copy-AutonomousStateSnapshot $snapshot
    $invalid.active_issues[0].task_kind = 'unknown'
    $invalidResult = Invoke-AutonomousStateCase -Name 'stale-unknown-task-kind' -Snapshot $invalid
    Assert-AutonomousState -Result $invalidResult -ExpectedState 'blocked-hard-stop' -ExpectedAction 'stop'
    Assert-Contains -Collection @($invalidResult.Json.reason_codes) -Expected 'INVALID_TASK_KIND' `
        -Message '未知 task kind 必须显式 fail-closed'

    $caseDrift = Copy-AutonomousStateSnapshot $snapshot
    $caseDrift.release_audit = 'current'
    $caseDrift.active_issues[0].task_kind = 'UPSTREAM-SYNC'
    $caseDriftResult = Invoke-AutonomousStateCase -Name 'current-case-drift-task-kind' -Snapshot $caseDrift
    Assert-AutonomousState -Result $caseDriftResult -ExpectedState 'blocked-hard-stop' -ExpectedAction 'stop'
    Assert-Contains -Collection @($caseDriftResult.Json.reason_codes) -Expected 'INVALID_TASK_KIND' `
        -Message 'task kind 大小写漂移必须显式 fail-closed'

    $arrayKind = Copy-AutonomousStateSnapshot $snapshot
    $arrayKind.release_audit = 'current'
    $arrayKind.active_issues[0].task_kind = [object[]]@('upstream-sync')
    $arrayKindResult = Invoke-AutonomousStateCase -Name 'current-array-task-kind' -Snapshot $arrayKind
    Assert-AutonomousState -Result $arrayKindResult -ExpectedState 'blocked-hard-stop' -ExpectedAction 'stop'
    Assert-Contains -Collection @($arrayKindResult.Json.reason_codes) -Expected 'INVALID_TASK_KIND' `
        -Message 'task kind 单元素数组必须显式 fail-closed'

    $arrayKindWithPr = New-MergeReadyAutonomousStateSnapshot
    $arrayKindWithPr.active_issues[0] | Add-Member -NotePropertyName task_kind `
        -NotePropertyValue ([object[]]@('refactor'))
    $arrayKindWithPrResult = Invoke-AutonomousStateCase -Name 'current-array-task-kind-with-pr' -Snapshot $arrayKindWithPr
    Assert-AutonomousState -Result $arrayKindWithPrResult -ExpectedState 'blocked-hard-stop' -ExpectedAction 'stop'
    Assert-Contains -Collection @($arrayKindWithPrResult.Json.reason_codes) -Expected 'INVALID_TASK_KIND' `
        -Message '无效 task kind 与活动 PR 并存时仍必须返回结构化硬停止'
}

Invoke-ContractTest -Name '无人值守 stale upstream-sync 仍绑定完整 Final Head 与 release-audit Job' -Body {
    $unbound = New-MergeReadyAutonomousStateSnapshot
    $unbound.release_audit = 'stale-re-audit-required'
    $unbound.active_issues[0] | Add-Member -NotePropertyName task_kind -NotePropertyValue 'upstream-sync'
    $unboundResult = Invoke-AutonomousStateCase -Name 'stale-upstream-sync-unbound-task-kind' -Snapshot $unbound
    Assert-AutonomousState -Result $unboundResult -ExpectedState 'active-pr-review-ci' -ExpectedAction 'resume-pr'
    Assert-Contains -Collection @($unboundResult.Json.merge_gate_pending) -Expected 'EVIDENCE_TASK_KIND' `
        -Message 'upstream-sync task kind 必须同时绑定 Planning 与 PR evidence'

    $ready = New-MergeReadyAutonomousStateSnapshot
    $ready.release_audit = 'stale-re-audit-required'
    $ready.active_issues[0] | Add-Member -NotePropertyName task_kind -NotePropertyValue 'upstream-sync'
    $ready.active_issues[0].planning_evidence.task_kind = 'upstream-sync'
    $ready.active_prs[0].evidence.task_kind = 'upstream-sync'
    Assert-AutonomousState -Result (Invoke-AutonomousStateCase -Name 'stale-upstream-sync-merge-ready' -Snapshot $ready) `
        -ExpectedState 'merge-ready-exact-head' -ExpectedAction 'squash-merge-exact-head'

    $arrayEvidence = Copy-AutonomousStateSnapshot $ready
    $arrayEvidence.active_issues[0].planning_evidence.task_kind = [object[]]@('upstream-sync')
    $arrayEvidenceResult = Invoke-AutonomousStateCase -Name 'stale-upstream-sync-array-evidence' -Snapshot $arrayEvidence
    Assert-AutonomousState -Result $arrayEvidenceResult -ExpectedState 'active-pr-review-ci' -ExpectedAction 'resume-pr'
    Assert-Contains -Collection @($arrayEvidenceResult.Json.merge_gate_pending) -Expected 'EVIDENCE_TASK_KIND' `
        -Message 'Planning task kind 单元素数组不得通过 Final Head 门'

    $failedJob = New-MergeReadyAutonomousStateSnapshot
    $failedJob.release_audit = 'stale-re-audit-required'
    $failedJob.active_issues[0] | Add-Member -NotePropertyName task_kind -NotePropertyValue 'upstream-sync'
    $failedJob.active_issues[0].planning_evidence.task_kind = 'upstream-sync'
    $failedJob.active_prs[0].evidence.task_kind = 'upstream-sync'
    $releaseAuditJob = @($failedJob.active_prs[0].workflow_runs[0].jobs | Where-Object { $_.name -ceq 'release-audit' })[0]
    $releaseAuditJob.conclusion = 'failure'
    $failedResult = Invoke-AutonomousStateCase -Name 'stale-upstream-sync-release-audit-failed' -Snapshot $failedJob
    Assert-AutonomousState -Result $failedResult -ExpectedState 'active-pr-review-ci' -ExpectedAction 'resume-pr'
    Assert-Contains -Collection @($failedResult.Json.merge_gate_pending) -Expected 'WORKFLOW_CI' `
        -Message 'stale upstream-sync 不得绕过失败的 release-audit Job'

    $postMerge = New-PostMergeAutonomousStateSnapshot
    $postMerge.release_audit = 'stale-re-audit-required'
    $postMerge.active_issues[0] | Add-Member -NotePropertyName task_kind -NotePropertyValue 'upstream-sync'
    $postMerge.active_issues[0].planning_evidence.task_kind = 'upstream-sync'
    $postMerge.merged_prs[0].evidence.task_kind = 'upstream-sync'
    Assert-AutonomousState -Result (Invoke-AutonomousStateCase -Name 'stale-upstream-sync-post-merge' -Snapshot $postMerge) `
        -ExpectedState 'post-merge-verification' -ExpectedAction 'close-issue-and-archive'
}

Invoke-ContractTest -Name '无人值守阻断 origin main freshness 漂移' -Body {
    $snapshot = Copy-AutonomousStateSnapshot (New-ValidAutonomousStateSnapshot)
    $snapshot.expected_base = 'cccccccccccccccccccccccccccccccccccccccc'
    $result = Invoke-AutonomousStateCase -Name 'base-drift' -Snapshot $snapshot
    Assert-AutonomousState -Result $result -ExpectedState 'blocked-hard-stop' -ExpectedAction 'stop'
    Assert-Contains -Collection @($result.Json.reason_codes) -Expected 'ORIGIN_MAIN_DRIFT' -Message 'main freshness 漂移必须稳定阻断'
}

Invoke-ContractTest -Name '无人值守外部 GitHub 不可用时有限重试' -Body {
    $snapshot = Copy-AutonomousStateSnapshot (New-ValidAutonomousStateSnapshot)
    $snapshot.github_available = $false
    $snapshot.worktree_clean = $false
    Assert-AutonomousState -Result (Invoke-AutonomousStateCase -Name 'github-down' -Snapshot $snapshot) -ExpectedState 'blocked-external-retry' -ExpectedAction 'retry-external'
}

Invoke-ContractTest -Name '无人值守阻断本地 tracking ref 落后 GitHub main' -Body {
    $snapshot = Copy-AutonomousStateSnapshot (New-ValidAutonomousStateSnapshot)
    $snapshot.observed_remote_main = 'cccccccccccccccccccccccccccccccccccccccc'
    $result = Invoke-AutonomousStateCase -Name 'remote-main-drift' -Snapshot $snapshot
    Assert-AutonomousState -Result $result -ExpectedState 'blocked-hard-stop' -ExpectedAction 'stop'
    Assert-Contains -Collection @($result.Json.reason_codes) -Expected 'REMOTE_MAIN_DRIFT' -Message '远端 main 漂移必须稳定阻断'
}

Invoke-ContractTest -Name '无人值守安全忽略非 owner marker Issue 与 PR' -Body {
    $snapshot = Copy-AutonomousStateSnapshot (New-ValidAutonomousStateSnapshot)
    $snapshot.branch = 'main'
    $snapshot.active_issues = @([pscustomobject]@{ number = 900; url = 'issue-900'; author_login = 'external-user' })
    $snapshot.active_prs = @([pscustomobject]@{
        number = 901
        url = 'pr-901'
        base_ref = 'main'
        head_oid = 'bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb'
        author_login = 'external-user'
        head_owner_login = 'external-user'
    })
    $result = Invoke-AutonomousStateCase -Name 'untrusted-marker' -Snapshot $snapshot
    Assert-AutonomousState -Result $result -ExpectedState 'idle-select-candidate' -ExpectedAction 'select-candidate'
    Assert-Equal -Expected 1 -Actual $result.Json.ignored_untrusted_issue_markers -Message '非 owner Issue marker 必须被计数并忽略'
    Assert-Equal -Expected 1 -Actual $result.Json.ignored_untrusted_pr_markers -Message '非 owner PR marker 必须被计数并忽略'
}

Invoke-ContractTest -Name '无人值守 live 外部列表使用全量分页和严格数组解析' -Body {
    $source = Get-Content -LiteralPath $autonomousStateScript -Raw
    foreach ($required in @('--paginate', '--slurp', 'ConvertFrom-Json -Depth 100 -NoEnumerate')) {
        Assert-True -Condition $source.Contains($required) -Message "live 外部列表缺少严格分页合同：$required"
    }
    Assert-True -Condition (-not $source.Contains('--limit 100')) -Message 'live 外部列表不得截断前 100 项'
}

Invoke-ContractTest -Name '无人值守 live 空范围保持数组身份与确定哈希' -Body {
    Invoke-Expression (Get-AutonomousStateFunctionSource -Name 'Get-AutonomousScopeProjection')

    $projection = Get-AutonomousScopeProjection -Paths @()
    Assert-True -Condition ($projection.paths -is [System.Array]) -Message '空范围路径必须保留数组身份'
    Assert-Equal -Expected 0 -Actual $projection.paths.Count -Message '空范围不得产生伪路径'
    Assert-Equal -Expected 0L -Actual $projection.count -Message '空范围计数必须归一化为 Int64 零'
    Assert-Equal -Expected 'sha256:01ba4719c80b6fe911b091a7c05124b64eeece964e09c058ef8f9805daca546b' `
        -Actual $projection.scope_hash -Message '空范围必须使用 LF 空载荷的确定哈希'

    $single = Get-AutonomousScopeProjection -Paths @('single/path.txt')
    Assert-True -Condition ($single.paths -is [System.Array]) -Message '单路径必须保留数组身份'
    Assert-Equal -Expected 1L -Actual $single.count -Message '单路径计数必须归一化为 Int64 一'
    Assert-Equal -Expected 'single/path.txt' -Actual $single.paths[0] -Message '单路径内容不得漂移'
    Assert-Equal -Expected 'sha256:d2e8ff22b0918f8c8bedc32b99272db8a20c1a036d02135cfd9359c4782119bb' `
        -Actual $single.scope_hash -Message '单路径必须使用 LF 末尾的确定哈希'

    $originalCulture = [Threading.Thread]::CurrentThread.CurrentCulture
    $originalUiCulture = [Threading.Thread]::CurrentThread.CurrentUICulture
    try {
        $mixedPaths = [string[]]@('I.txt', 'i.txt', 'İ.txt', 'ı.txt', 'A.txt', 'a.txt', 'I.txt')
        $expectedPaths = [string[]]@('A.txt', 'I.txt', 'a.txt', 'i.txt', 'İ.txt', 'ı.txt')
        foreach ($cultureName in @('en-US', 'sv-SE', 'tr-TR')) {
            $culture = [Globalization.CultureInfo]::GetCultureInfo($cultureName)
            [Threading.Thread]::CurrentThread.CurrentCulture = $culture
            [Threading.Thread]::CurrentThread.CurrentUICulture = $culture

            $mixed = Get-AutonomousScopeProjection -Paths $mixedPaths
            Assert-True -Condition ($mixed.paths -is [System.Array]) `
                -Message "多路径必须在 $cultureName 保留数组身份"
            Assert-Equal -Expected 6L -Actual $mixed.count `
                -Message "大小写不同路径不得在 $cultureName 被合并"
            Assert-True -Condition ([string]::Equals(
                [string]::Join("`n", $expectedPaths),
                [string]::Join("`n", [string[]]$mixed.paths),
                [StringComparison]::Ordinal
            )) -Message "多路径必须在 $cultureName 使用 Ordinal 顺序"
            Assert-Equal -Expected 'sha256:33d03dd10bc0a0f97b747bdb48f4954834a9f04fc74a242bd69fd62bcc4f7635' `
                -Actual $mixed.scope_hash -Message "多路径 hash 不得受 $cultureName 影响"
        }
    } finally {
        [Threading.Thread]::CurrentThread.CurrentCulture = $originalCulture
        [Threading.Thread]::CurrentThread.CurrentUICulture = $originalUiCulture
    }
}

Invoke-ContractTest -Name '无人值守 live 精确恢复自动关闭的合并后 Issue' -Body {
    Invoke-Expression (Get-AutonomousStateFunctionSource -Name 'Get-PropertyValue')
    Invoke-Expression (Get-AutonomousStateFunctionSource -Name 'Resolve-AutonomousTaskLink')

    function New-TestTaskIssue {
        param(
            [string]$State = 'closed',
            [string]$Url = 'https://github.com/nonononull/inputcodex/issues/113',
            [string]$Author = 'nonononull',
            [string]$TaskKind = 'refactor'
        )
        [pscustomobject][ordered]@{
            number = 113L
            url = $Url
            author_login = $Author
            github_state = $State
            task_kind = $TaskKind
            planning_evidence = [pscustomobject][ordered]@{ valid = $false }
        }
    }

    function New-TestMergedTaskPr {
        param(
            [string]$IssueUrl = 'https://github.com/nonononull/inputcodex/issues/113',
            [string]$Author = 'nonononull'
        )
        [pscustomobject][ordered]@{
            number = 114L
            head_oid = 'bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb'
            merge_commit_oid = 'cccccccccccccccccccccccccccccccccccccccc'
            author_login = $Author
            head_owner_login = 'nonononull'
            evidence = [pscustomobject][ordered]@{
                valid = $true
                tracking_issue_ref = $IssueUrl
            }
        }
    }

    $closedIssue = New-TestTaskIssue -TaskKind 'upstream-sync'
    $mergedPr = New-TestMergedTaskPr
    $exact = Resolve-AutonomousTaskLink -MarkedIssues @($closedIssue) -MergedPullRequests @($mergedPr) `
        -ObservedRemoteMain 'cccccccccccccccccccccccccccccccccccccccc' `
        -WorktreeHead 'bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb'
    Assert-Equal -Expected 1 -Actual $exact.active_issues.Count -Message 'exact closed Issue 必须恢复为 workflow-active'
    Assert-Equal -Expected 113L -Actual $exact.active_issues[0].number -Message '恢复的 Issue 必须保持原编号'
    Assert-Equal -Expected 'upstream-sync' -Actual $exact.active_issues[0].task_kind `
        -Message 'closed Issue 恢复必须保留 live 投影的 task kind'
    Assert-Equal -Expected 1 -Actual $exact.linked_merged_prs.Count -Message 'exact merged PR 必须同步恢复'

    $openIssue = New-TestTaskIssue -State 'open'
    $open = Resolve-AutonomousTaskLink -MarkedIssues @($openIssue) -MergedPullRequests @($mergedPr) `
        -ObservedRemoteMain 'cccccccccccccccccccccccccccccccccccccccc' `
        -WorktreeHead 'bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb'
    Assert-Equal -Expected 1 -Actual $open.active_issues.Count -Message 'open Issue 既有行为不得漂移'
    Assert-Equal -Expected 1 -Actual $open.linked_merged_prs.Count -Message 'open Issue 必须继续关联 merged PR'

    foreach ($blocked in @(
        [pscustomobject]@{
            Name = 'stale-main'
            Issues = @($closedIssue)
            PullRequests = @($mergedPr)
            RemoteMain = 'dddddddddddddddddddddddddddddddddddddddd'
            WorktreeHead = 'bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb'
        },
        [pscustomobject]@{
            Name = 'wrong-head'
            Issues = @($closedIssue)
            PullRequests = @($mergedPr)
            RemoteMain = 'cccccccccccccccccccccccccccccccccccccccc'
            WorktreeHead = 'dddddddddddddddddddddddddddddddddddddddd'
        },
        [pscustomobject]@{
            Name = 'untrusted-pr'
            Issues = @($closedIssue)
            PullRequests = @((New-TestMergedTaskPr -Author 'attacker'))
            RemoteMain = 'cccccccccccccccccccccccccccccccccccccccc'
            WorktreeHead = 'bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb'
        },
        [pscustomobject]@{
            Name = 'duplicate'
            Issues = @($closedIssue, (New-TestTaskIssue -Url 'https://github.com/nonononull/inputcodex/issues/115'))
            PullRequests = @($mergedPr, (New-TestMergedTaskPr -IssueUrl 'https://github.com/nonononull/inputcodex/issues/115'))
            RemoteMain = 'cccccccccccccccccccccccccccccccccccccccc'
            WorktreeHead = 'bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb'
        }
    )) {
        $result = Resolve-AutonomousTaskLink -MarkedIssues $blocked.Issues `
            -MergedPullRequests $blocked.PullRequests -ObservedRemoteMain $blocked.RemoteMain `
            -WorktreeHead $blocked.WorktreeHead
        Assert-Equal -Expected 0 -Actual $result.active_issues.Count `
            -Message "非 exact closed Issue 不得恢复：$($blocked.Name)"
        Assert-Equal -Expected 0 -Actual $result.linked_merged_prs.Count `
            -Message "非 exact merged PR 不得恢复：$($blocked.Name)"
    }

    $source = Get-Content -LiteralPath $autonomousStateScript -Raw
    Assert-True -Condition $source.Contains('issues?state=all&per_page=100') `
        -Message 'closed Issue 恢复必须采集全状态 Issue'
    Assert-True -Condition (-not $source.Contains('issues?state=open&per_page=100')) `
        -Message 'live Issue 采集不得继续截断为 open'
    $liveSource = Get-AutonomousStateFunctionSource -Name 'Get-LiveSnapshot'
    Assert-True -Condition $liveSource.Contains('Resolve-AutonomousTaskLink') `
        -Message 'live 生产采集必须消费精确 task-link helper'
}

Invoke-ContractTest -Name '无人值守 live PR 投影不得覆盖工作树 Head' -Body {
    $source = Get-Content -LiteralPath $autonomousStateScript -Raw
    $tokens = $null
    $parseErrors = $null
    $ast = [System.Management.Automation.Language.Parser]::ParseInput(
        $source,
        [ref]$tokens,
        [ref]$parseErrors
    )
    Assert-Equal -Expected 0 -Actual @($parseErrors).Count -Message 'live 状态脚本必须可供 AST 数据流检查'

    $liveFunctions = @($ast.FindAll({
        param($node)
        $node -is [System.Management.Automation.Language.FunctionDefinitionAst] -and
        $node.Name -ceq 'Get-LiveSnapshot'
    }, $true))
    Assert-Equal -Expected 1 -Actual $liveFunctions.Count -Message '必须存在唯一 Get-LiveSnapshot'

    $headCaptures = @($liveFunctions[0].Body.FindAll({
        param($node)
        $node -is [System.Management.Automation.Language.AssignmentStatementAst] -and
        $node.Left -is [System.Management.Automation.Language.VariableExpressionAst] -and
        $node.Extent.Text.Contains("Invoke-GitRead @('rev-parse', 'HEAD')")
    }, $true))
    Assert-Equal -Expected 1 -Actual $headCaptures.Count -Message 'live 工作树 Head 必须唯一采集'

    $headVariableName = $headCaptures[0].Left.VariablePath.UserPath
    $headWrites = @($liveFunctions[0].Body.FindAll({
        param($node)
        $node -is [System.Management.Automation.Language.AssignmentStatementAst] -and
        $node.Left -is [System.Management.Automation.Language.VariableExpressionAst] -and
        $node.Left.VariablePath.UserPath -ieq $headVariableName
    }, $true))
    Assert-Equal -Expected 1 -Actual $headWrites.Count -Message '工作树 Head 变量在嵌套 PR 投影中不得再次赋值'

    $snapshotBindingPattern = '(?m)^\s*worktree_head\s*=\s*\$' +
        [regex]::Escape($headVariableName) + '\s*$'
    Assert-True -Condition ([regex]::IsMatch($liveFunctions[0].Extent.Text, $snapshotBindingPattern)) `
        -Message 'live 快照必须返回唯一采集的工作树 Head 变量'
}

Invoke-ContractTest -Name '无人值守拒绝未知状态快照 schema' -Body {
    $snapshot = Copy-AutonomousStateSnapshot (New-ValidAutonomousStateSnapshot)
    $snapshot.schema_version = 'unknown'
    $result = Invoke-AutonomousStateCase -Name 'schema' -Snapshot $snapshot
    Assert-Equal -Expected 11 -Actual $result.ExitCode -Message "未知快照 schema 应使用稳定退出码，输出=$($result.Output)"
    Assert-Equal -Expected 'AUTONOMOUS_STATE_INVALID_SNAPSHOT' -Actual $result.Json.error_code -Message '未知快照错误码必须稳定'
}

function New-ReleaseAuditSourceLock {
    param(
        [Parameter(Mandatory)]
        [string]$SnapshotTag,

        [Parameter(Mandatory)]
        [string]$SnapshotCommit,

        [Parameter(Mandatory)]
        [string]$CatalogTag,

        [Parameter(Mandatory)]
        [string]$CatalogCommit,

        [Parameter(Mandatory)]
        [string]$Status,

        [AllowNull()]
        [object]$StaleReason,

        [AllowNull()]
        [object]$ReAuditIssueRef
    )

    [pscustomobject][ordered]@{
        snapshot = [pscustomobject][ordered]@{
            release_tag = $SnapshotTag
            commit = $SnapshotCommit
        }
        release_audit = [pscustomobject][ordered]@{
            schema_version = 'inputcodex.release-audit.v1'
            catalog_release = [pscustomobject][ordered]@{
                tag = $CatalogTag
                commit = $CatalogCommit
            }
            status = $Status
            stale_reason = $StaleReason
            re_audit_issue_ref = $ReAuditIssueRef
        }
    }
}

function New-LegacySourceLock {
    param(
        [Parameter(Mandatory)]
        [string]$SnapshotTag,

        [Parameter(Mandatory)]
        [string]$SnapshotCommit
    )

    [pscustomobject][ordered]@{
        snapshot = [pscustomobject][ordered]@{
            release_tag = $SnapshotTag
            commit = $SnapshotCommit
        }
    }
}

function Invoke-ReleaseAuditGateCase {
    param(
        [Parameter(Mandatory)]
        [string]$Name,

        [Parameter(Mandatory)]
        $BaseSourceLock,

        [Parameter(Mandatory)]
        $HeadSourceLock,

        [Parameter(Mandatory)]
        [AllowEmptyCollection()]
        [object[]]$Changes,

        [scriptblock]$MutateFixture
    )

    $caseRoot = Join-Path $testRoot ("release-audit-{0}" -f $Name)
    $upstreamRoot = Join-Path $caseRoot 'upstream'
    $snapshotRoot = Join-Path $upstreamRoot 'CodexPlusPlus'
    New-Item -ItemType Directory -Path $snapshotRoot -Force | Out-Null

    $fixturePath = Join-Path $snapshotRoot 'README.md'
    [System.IO.File]::WriteAllText($fixturePath, "fixture`n", [Text.UTF8Encoding]::new($false))
    $gitOutput = @(& git -C $caseRoot init --quiet 2>&1 | ForEach-Object { $_.ToString() })
    if ($LASTEXITCODE -ne 0) { throw "Release Audit 夹具 git init 失败：$($gitOutput -join [Environment]::NewLine)" }
    $gitOutput = @(& git -C $caseRoot add -- 'upstream/CodexPlusPlus/README.md' 2>&1 | ForEach-Object { $_.ToString() })
    if ($LASTEXITCODE -ne 0) { throw "Release Audit 夹具 git add 失败：$($gitOutput -join [Environment]::NewLine)" }
    & git -C $caseRoot -c user.name=inputcodex-ci -c user.email=ci@inputcodex.invalid commit --quiet -m fixture
    if ($LASTEXITCODE -ne 0) { throw 'Release Audit 夹具初始提交失败' }

    $headFixture = $HeadSourceLock | ConvertTo-Json -Depth 100 | ConvertFrom-Json -Depth 100
    $fixtureBytes = [System.IO.File]::ReadAllBytes($fixturePath)
    $fixtureSha256 = [Convert]::ToHexString([Security.Cryptography.SHA256]::HashData($fixtureBytes)).ToLowerInvariant()
    $fixtureBlob = (& git -C $caseRoot hash-object -- 'upstream/CodexPlusPlus/README.md').Trim()
    if ($LASTEXITCODE -ne 0 -or $fixtureBlob -notmatch '^[0-9a-f]{40}$') { throw 'Release Audit 夹具 blob 计算失败' }
    $fixtureTree = (& git -C $caseRoot rev-parse 'HEAD:upstream/CodexPlusPlus').Trim()
    if ($LASTEXITCODE -ne 0 -or $fixtureTree -notmatch '^[0-9a-f]{40}$') { throw 'Release Audit 夹具 tree 计算失败' }
    $manifestPayload = "$fixtureSha256  README.md`n"
    $manifestSha256 = [Convert]::ToHexString(
        [Security.Cryptography.SHA256]::HashData([Text.UTF8Encoding]::new($false).GetBytes($manifestPayload))
    ).ToLowerInvariant()
    $headFixture.snapshot | Add-Member -NotePropertyName path -NotePropertyValue 'upstream/CodexPlusPlus' -Force
    $headFixture.snapshot | Add-Member -NotePropertyName commit_tree -NotePropertyValue $fixtureTree -Force
    $headFixture | Add-Member -NotePropertyName schema_version -NotePropertyValue 'inputcodex.source-lock.v1' -Force
    $headFixture | Add-Member -NotePropertyName manifest -NotePropertyValue ([pscustomobject][ordered]@{
        algorithm = 'sha256'
        format = '<sha256><two spaces><posix path><newline>'
        sha256 = $manifestSha256
        file_count = 1
        total_bytes = [long]$fixtureBytes.Length
        largest_file = [pscustomobject][ordered]@{
            path = 'README.md'
            mode = '100644'
            size = [long]$fixtureBytes.Length
            git_blob_sha1 = $fixtureBlob
            sha256 = $fixtureSha256
        }
    }) -Force
    $headFixture | Add-Member -NotePropertyName tree -NotePropertyValue ([pscustomobject][ordered]@{
        sha = $fixtureTree
        entry_count = 1
        directory_count = 0
        file_count = 1
        submodule_count = 0
    }) -Force
    $headFixture | Add-Member -NotePropertyName files -NotePropertyValue @([pscustomobject][ordered]@{
        path = 'README.md'
        mode = '100644'
        size = [long]$fixtureBytes.Length
        git_blob_sha1 = $fixtureBlob
        sha256 = $fixtureSha256
    }) -Force

    if ($null -ne $MutateFixture) {
        & $MutateFixture $caseRoot $headFixture
        & git -C $caseRoot diff --cached --quiet
        $cachedDiffExitCode = $LASTEXITCODE
        if ($cachedDiffExitCode -eq 1) {
            & git -C $caseRoot -c user.name=inputcodex-ci -c user.email=ci@inputcodex.invalid commit --quiet -m mutation
            if ($LASTEXITCODE -ne 0) { throw 'Release Audit 夹具漂移提交失败' }
        } elseif ($cachedDiffExitCode -ne 0) {
            throw 'Release Audit 夹具 staged diff 检查失败'
        }
    }

    $baseSourceLockPath = Join-Path $caseRoot 'base-source-lock.json'
    $headSourceLockPath = Join-Path $upstreamRoot 'source-lock.json'
    $changesPath = Join-Path $caseRoot 'changes.json'
    Write-JsonFile -Path $baseSourceLockPath -Value $BaseSourceLock
    Write-JsonFile -Path $headSourceLockPath -Value $headFixture
    Write-JsonFile -Path $changesPath -Value $Changes

    Invoke-ChildScript -Path $releaseAuditGateScript -Arguments @(
        '-RepositoryRoot', $caseRoot,
        '-InputFile', $changesPath,
        '-BaseSourceLockPath', $baseSourceLockPath
    )
}

function Assert-ReleaseAuditSuccess {
    param(
        [Parameter(Mandatory)]
        $Result,

        [Parameter(Mandatory)]
        [string]$ExpectedStatus,

        [Parameter(Mandatory)]
        [bool]$ExpectedReaudit
    )

    Assert-Equal -Expected 0 -Actual $Result.ExitCode -Message "Release 审计门应通过，输出=$($Result.Output)"
    Assert-True -Condition ($null -ne $Result.Json) -Message "Release 审计门必须输出 JSON，输出=$($Result.Output)"
    Assert-Equal -Expected $true -Actual $Result.Json.ok -Message 'Release 审计门通过时必须标记 ok'
    Assert-Equal -Expected $ExpectedStatus -Actual $Result.Json.status -Message 'Release 审计状态必须可诊断'
    Assert-Equal -Expected $ExpectedReaudit -Actual $Result.Json.requires_reaudit -Message '重新审计要求必须可诊断'
}

function Assert-ReleaseAuditFailureCode {
    param(
        [Parameter(Mandatory)]
        $Result,

        [Parameter(Mandatory)]
        [string]$Code
    )

    Assert-True -Condition ($Result.ExitCode -ne 0) -Message 'Release 审计门拒绝路径时必须返回非零退出码'
    Assert-True -Condition ($null -ne $Result.Json) -Message "Release 审计门失败时必须输出 JSON，输出=$($Result.Output)"
    Assert-Contains -Collection @($Result.Json.errors.code) -Expected $Code -Message 'Release 审计门必须返回稳定错误码'
}

function Invoke-ClassifierCase {
    param(
        [Parameter(Mandatory)]
        [string]$Name,

        [Parameter(Mandatory)]
        [AllowEmptyCollection()]
        [object[]]$Changes
    )

    $inputPath = Join-Path $testRoot ("classify-{0}.json" -f $Name)
    Write-JsonFile -Path $inputPath -Value $Changes
    Invoke-ChildScript -Path $classifierScript -Arguments @('-InputFile', $inputPath)
}

function Assert-ClassifierSuccess {
    param(
        [Parameter(Mandatory)]
        $Result
    )

    Assert-Equal -Expected 0 -Actual $Result.ExitCode -Message "路径分类脚本应成功，输出=$($Result.Output)"
    Assert-True -Condition ($null -ne $Result.Json) -Message "路径分类脚本必须输出 JSON，输出=$($Result.Output)"
}

function Assert-ClassifierFailureCode {
    param(
        [Parameter(Mandatory)]
        $Result,

        [Parameter(Mandatory)]
        [string]$Code
    )

    Assert-True -Condition ($Result.ExitCode -ne 0) -Message '非法路径输入必须返回非零退出码'
    Assert-True -Condition ($null -ne $Result.Json) -Message "非法路径输入必须输出 JSON 错误，输出=$($Result.Output)"
    Assert-Contains -Collection @($Result.Json.errors.code) -Expected $Code -Message '非法路径输入必须返回稳定错误码'
}

Invoke-ContractTest -Name '空 diff 返回确定的空分类' -Body {
    $result = Invoke-ClassifierCase -Name 'empty' -Changes @()
    Assert-ClassifierSuccess -Result $result
    Assert-Equal -Expected $false -Actual $result.Json.has_changes -Message '空 diff 不应标记为有变化'
    Assert-Equal -Expected $false -Actual $result.Json.docs_only -Message '空 diff 不应伪装为文档变更'
    Assert-Equal -Expected $false -Actual $result.Json.heavy -Message '空 diff 不应触发重型任务'
    Assert-Equal -Expected 0 -Actual $result.Json.change_count -Message '空 diff 的记录数必须为零'
}

Invoke-ContractTest -Name '纯文档 diff 不触发重型任务' -Body {
    $changes = @(
        [pscustomobject]@{ status = 'M'; path = 'docs/guide.md' }
        [pscustomobject]@{ status = 'A'; path = 'README.md' }
    )
    $result = Invoke-ClassifierCase -Name 'docs-only' -Changes $changes
    Assert-ClassifierSuccess -Result $result
    Assert-Equal -Expected $true -Actual $result.Json.has_changes -Message '文档 diff 应标记为有变化'
    Assert-Equal -Expected $true -Actual $result.Json.docs_only -Message '纯文档 diff 应被识别'
    Assert-Equal -Expected $false -Actual $result.Json.heavy -Message '纯文档 diff 不应触发重型任务'
    Assert-Contains -Collection @($result.Json.docs_paths) -Expected 'README.md' -Message '文档路径必须进入 docs_paths'
}

Invoke-ContractTest -Name 'Rust 源码 diff 触发重型任务' -Body {
    $changes = @(
        [pscustomobject]@{ status = 'M'; path = 'crates/inputcodex-domain/src/lib.rs' }
    )
    $result = Invoke-ClassifierCase -Name 'heavy-rust' -Changes $changes
    Assert-ClassifierSuccess -Result $result
    Assert-Equal -Expected $false -Actual $result.Json.docs_only -Message 'Rust 源码不能归为纯文档'
    Assert-Equal -Expected $true -Actual $result.Json.heavy -Message 'Rust 源码必须触发重型任务'
    Assert-Contains -Collection @($result.Json.heavy_paths) -Expected 'crates/inputcodex-domain/src/lib.rs' -Message '重型路径必须可审计'
}

Invoke-ContractTest -Name '删除记录参与分类' -Body {
    $changes = @(
        [pscustomobject]@{ status = 'D'; path = 'Cargo.lock' }
    )
    $result = Invoke-ClassifierCase -Name 'deleted-lock' -Changes $changes
    Assert-ClassifierSuccess -Result $result
    Assert-Equal -Expected $true -Actual $result.Json.heavy -Message '删除 Cargo.lock 必须触发重型任务'
    Assert-Contains -Collection @($result.Json.changed_paths) -Expected 'Cargo.lock' -Message '删除路径必须保留在输出中'
}

Invoke-ContractTest -Name '重命名同时审计新旧路径' -Body {
    $changes = @(
        [pscustomobject]@{ status = 'R'; old_path = 'docs/old.md'; path = 'docs/new.md' }
    )
    $result = Invoke-ClassifierCase -Name 'renamed-doc' -Changes $changes
    Assert-ClassifierSuccess -Result $result
    Assert-Equal -Expected $true -Actual $result.Json.docs_only -Message '文档间重命名仍应归为纯文档'
    Assert-Contains -Collection @($result.Json.changed_paths) -Expected 'docs/old.md' -Message '重命名旧路径必须保留'
    Assert-Contains -Collection @($result.Json.changed_paths) -Expected 'docs/new.md' -Message '重命名新路径必须保留'
}

$invalidPathCases = @(
    [pscustomobject]@{ name = 'path-traversal'; path = '../Cargo.toml' }
    [pscustomobject]@{ name = 'absolute-posix'; path = '/tmp/file.rs' }
    [pscustomobject]@{ name = 'absolute-windows'; path = 'C:/repo/file.rs' }
    [pscustomobject]@{ name = 'backslash'; path = 'docs\guide.md' }
    [pscustomobject]@{ name = 'control-character'; path = "docs/$([char]1)guide.md" }
)

foreach ($invalidPathCase in $invalidPathCases) {
    Invoke-ContractTest -Name "拒绝非法路径 $($invalidPathCase.name)" -Body {
        $changes = @(
            [pscustomobject]@{ status = 'M'; path = $invalidPathCase.path }
        )
        $result = Invoke-ClassifierCase -Name $invalidPathCase.name -Changes $changes
        Assert-ClassifierFailureCode -Result $result -Code 'INVALID_PATH'
    }
}

Invoke-ContractTest -Name '重命名缺失旧路径时失败' -Body {
    $changes = @(
        [pscustomobject]@{ status = 'R'; path = 'docs/new.md' }
    )
    $result = Invoke-ClassifierCase -Name 'rename-without-old-path' -Changes $changes
    Assert-ClassifierFailureCode -Result $result -Code 'OLD_PATH_REQUIRED'
}

Invoke-ContractTest -Name 'Release 审计门区分 current 与 stale 并阻断产品路径' -Body {
    $current = New-ReleaseAuditSourceLock `
        -SnapshotTag 'v1.2.41' `
        -SnapshotCommit '3dafffcafb2566a1e8bce4b35671656d6adb3eda' `
        -CatalogTag 'v1.2.41' `
        -CatalogCommit '3dafffcafb2566a1e8bce4b35671656d6adb3eda' `
        -Status 'current' `
        -StaleReason $null `
        -ReAuditIssueRef $null
    $stale = New-ReleaseAuditSourceLock `
        -SnapshotTag 'v1.2.42' `
        -SnapshotCommit '657cd33e009ad02515d30db6492cd4e669b06318' `
        -CatalogTag 'v1.2.41' `
        -CatalogCommit '3dafffcafb2566a1e8bce4b35671656d6adb3eda' `
        -Status 'stale-re-audit-required' `
        -StaleReason '上游 v1.2.42 已缓存，功能目录尚未完成复审' `
        -ReAuditIssueRef 'https://github.com/nonononull/inputcodex/issues/34'

    $result = Invoke-ReleaseAuditGateCase `
        -Name 'current-product-change' `
        -BaseSourceLock $current `
        -HeadSourceLock $current `
        -Changes @([pscustomobject]@{ status = 'M'; path = 'apps/inputcodex-desktop/src/main.rs' })
    Assert-ReleaseAuditSuccess -Result $result -ExpectedStatus 'current' -ExpectedReaudit $false

    $result = Invoke-ReleaseAuditGateCase `
        -Name 'current-empty-change-set' `
        -BaseSourceLock $current `
        -HeadSourceLock $current `
        -Changes @()
    Assert-ReleaseAuditSuccess -Result $result -ExpectedStatus 'current' -ExpectedReaudit $false

    $legacyBase = New-LegacySourceLock `
        -SnapshotTag 'v1.2.41' `
        -SnapshotCommit '3dafffcafb2566a1e8bce4b35671656d6adb3eda'
    $result = Invoke-ReleaseAuditGateCase `
        -Name 'current-legacy-base' `
        -BaseSourceLock $legacyBase `
        -HeadSourceLock $current `
        -Changes @([pscustomobject]@{ status = 'M'; path = 'upstream/source-lock.json' })
    Assert-ReleaseAuditSuccess -Result $result -ExpectedStatus 'current' -ExpectedReaudit $false

    $result = Invoke-ReleaseAuditGateCase `
        -Name 'stale-reaudit-only' `
        -BaseSourceLock $current `
        -HeadSourceLock $stale `
        -Changes @(
            [pscustomobject]@{ status = 'M'; path = 'upstream/source-lock.json' }
            [pscustomobject]@{ status = 'M'; path = 'upstream/CodexPlusPlus/README.md' }
            [pscustomobject]@{ status = 'M'; path = 'docs/reports/2026-07-22-upstream-v1.2.42-sync.md' }
            [pscustomobject]@{ status = 'M'; path = 'parity/features/source-index.yml' }
            [pscustomobject]@{ status = 'M'; path = 'crates/inputcodex-parity/src/validation.rs' }
        )
    Assert-ReleaseAuditSuccess -Result $result -ExpectedStatus 'stale-re-audit-required' -ExpectedReaudit $true

    foreach ($blockedPath in @(
        'benchmarks/cold-start.rs'
        'apps/inputcodex-desktop/src/main.rs'
        'crates/inputcodex-domain/src/lib.rs'
        'Cargo.toml'
        'Cargo.lock'
    )) {
        $result = Invoke-ReleaseAuditGateCase `
            -Name ("stale-blocked-{0}" -f ($blockedPath -replace '[^A-Za-z0-9]+', '-')) `
            -BaseSourceLock $current `
            -HeadSourceLock $stale `
            -Changes @([pscustomobject]@{ status = 'M'; path = $blockedPath })
        Assert-ReleaseAuditFailureCode -Result $result -Code 'RELEASE_AUDIT_REAUDIT_REQUIRED'
    }

    $renewedCurrent = New-ReleaseAuditSourceLock `
        -SnapshotTag 'v1.2.42' `
        -SnapshotCommit '657cd33e009ad02515d30db6492cd4e669b06318' `
        -CatalogTag 'v1.2.42' `
        -CatalogCommit '657cd33e009ad02515d30db6492cd4e669b06318' `
        -Status 'current' `
        -StaleReason $null `
        -ReAuditIssueRef $null
    $result = Invoke-ReleaseAuditGateCase `
        -Name 'audit-changed-with-product' `
        -BaseSourceLock $current `
        -HeadSourceLock $renewedCurrent `
        -Changes @(
            [pscustomobject]@{ status = 'M'; path = 'upstream/source-lock.json' }
            [pscustomobject]@{ status = 'M'; path = 'apps/inputcodex-desktop/src/main.rs' }
        )
    Assert-ReleaseAuditFailureCode -Result $result -Code 'RELEASE_AUDIT_CHANGED_WITH_BLOCKED_PATH'

    $invalidStale = New-ReleaseAuditSourceLock `
        -SnapshotTag 'v1.2.42' `
        -SnapshotCommit '657cd33e009ad02515d30db6492cd4e669b06318' `
        -CatalogTag 'v1.2.41' `
        -CatalogCommit '3dafffcafb2566a1e8bce4b35671656d6adb3eda' `
        -Status 'stale-re-audit-required' `
        -StaleReason '' `
        -ReAuditIssueRef 'https://github.com/nonononull/inputcodex/issues/34'
    $result = Invoke-ReleaseAuditGateCase `
        -Name 'invalid-stale-reason' `
        -BaseSourceLock $current `
        -HeadSourceLock $invalidStale `
        -Changes @([pscustomobject]@{ status = 'M'; path = 'parity/features/source-index.yml' })
    Assert-ReleaseAuditFailureCode -Result $result -Code 'RELEASE_AUDIT_INVALID'
}

Invoke-ContractTest -Name 'Release 审计门验证完整上游快照' -Body {
    $current = New-ReleaseAuditSourceLock `
        -SnapshotTag 'v1.2.41' `
        -SnapshotCommit '3dafffcafb2566a1e8bce4b35671656d6adb3eda' `
        -CatalogTag 'v1.2.41' `
        -CatalogCommit '3dafffcafb2566a1e8bce4b35671656d6adb3eda' `
        -Status 'current' `
        -StaleReason $null `
        -ReAuditIssueRef $null

    $valid = Invoke-ReleaseAuditGateCase -Name 'snapshot-valid' -BaseSourceLock $current -HeadSourceLock $current -Changes @()
    Assert-ReleaseAuditSuccess -Result $valid -ExpectedStatus 'current' -ExpectedReaudit $false

    $cases = @(
        [pscustomobject]@{
            Name = 'content'
            Mutate = {
                param($root, $lock)
                [System.IO.File]::WriteAllText((Join-Path $root 'upstream/CodexPlusPlus/README.md'), "tampered`n", [Text.UTF8Encoding]::new($false))
                & git -C $root add -- 'upstream/CodexPlusPlus/README.md' | Out-Null
                if ($LASTEXITCODE -ne 0) { throw 'Release Audit 内容漂移夹具 git add 失败' }
            }
        },
        [pscustomobject]@{
            Name = 'mode'
            Mutate = { param($root, $lock) $lock.files[0].mode = '100755' }
        },
        [pscustomobject]@{
            Name = 'manifest'
            Mutate = { param($root, $lock) $lock.manifest.sha256 = ('0' * 64) }
        },
        [pscustomobject]@{
            Name = 'path-set'
            Mutate = {
                param($root, $lock)
                [System.IO.File]::WriteAllText((Join-Path $root 'upstream/CodexPlusPlus/EXTRA.txt'), "extra`n", [Text.UTF8Encoding]::new($false))
                & git -C $root add -- 'upstream/CodexPlusPlus/EXTRA.txt' | Out-Null
                if ($LASTEXITCODE -ne 0) { throw 'Release Audit 额外路径夹具 git add 失败' }
            }
        }
    )
    foreach ($case in $cases) {
        $result = Invoke-ReleaseAuditGateCase `
            -Name "snapshot-$($case.Name)" `
            -BaseSourceLock $current `
            -HeadSourceLock $current `
            -Changes @() `
            -MutateFixture $case.Mutate
        Assert-ReleaseAuditFailureCode -Result $result -Code 'UPSTREAM_SNAPSHOT_INTEGRITY_INVALID'
    }
}

Invoke-ContractTest -Name '冷构建指标同时写入日志与摘要' -Body {
    Assert-True -Condition (Test-Path -LiteralPath $workflowPath -PathType Leaf) -Message 'CI Workflow 必须存在'
    $workflow = Get-Content -LiteralPath $workflowPath -Raw

    Assert-Equal -Expected 3 -Actual ([regex]::Matches($workflow, [regex]::Escape('$metrics = Get-Content')).Count) -Message '三个平台都必须显式读取冷构建指标'
    Assert-Equal -Expected 3 -Actual ([regex]::Matches($workflow, [regex]::Escape('$metrics | Write-Output')).Count) -Message '三个平台都必须把冷构建指标写入控制台日志'
    Assert-Equal -Expected 3 -Actual ([regex]::Matches($workflow, [regex]::Escape('$metrics | Add-Content -LiteralPath $env:GITHUB_STEP_SUMMARY')).Count) -Message '三个平台都必须把冷构建指标写入 Step Summary'
}

Invoke-ContractTest -Name 'Release 审计门接入 PR 与 required 汇总' -Body {
    Assert-True -Condition (Test-Path -LiteralPath $workflowPath -PathType Leaf) -Message 'CI Workflow 必须存在'
    $workflow = Get-Content -LiteralPath $workflowPath -Raw
    $workflowVariants = [ordered]@{
        LF = [regex]::Replace($workflow, '\r?\n', "`n")
        CRLF = [regex]::Replace($workflow, '\r?\n', "`r`n")
    }

    foreach ($variant in $workflowVariants.GetEnumerator()) {
        Assert-True -Condition ($variant.Value -match '(?m)^  release-audit:\r?$') -Message "$($variant.Key) Workflow 必须存在独立 release-audit Job"
        Assert-True -Condition ($variant.Value -match '(?ms)^  release-audit:.*?ref: \$\{\{ github\.event\.pull_request\.head\.sha \|\| github\.sha \}\}.*?^  governance:') `
            -Message "$($variant.Key) release-audit Job 必须检出精确 PR Head 或 main push SHA"
        Assert-True -Condition ($variant.Value -match 'Verify-ReleaseAuditGate\.ps1') -Message "$($variant.Key) release-audit Job 必须执行审计门脚本"
        Assert-True -Condition ($variant.Value -match '(?s)required:.*?needs:.*?- release-audit') -Message "$($variant.Key) required Job 必须依赖 release-audit Job"
    }
}

Invoke-ContractTest -Name '性能基线 Workflow 固定治理与测量合同' -Body {
    Assert-True -Condition (Test-Path -LiteralPath $performanceWorkflowPath -PathType Leaf) -Message '性能基线 Workflow 必须存在'
    Assert-True -Condition (Test-Path -LiteralPath $performanceInvokeScript -PathType Leaf) -Message '性能基线测量脚本必须存在'
    Assert-True -Condition (Test-Path -LiteralPath $performanceTestScript -PathType Leaf) -Message '性能基线验证脚本必须存在'
    Assert-True -Condition (Test-Path -LiteralPath $performanceObserverScript -PathType Leaf) -Message '性能预算观察器必须存在'

    $workflow = Get-Content -LiteralPath $performanceWorkflowPath -Raw -Encoding utf8
    Assert-Equal -Expected 2 -Actual ([regex]::Matches($workflow, "(?m)^      - 'upstream/source-lock\.json'\r?$").Count) `
        -Message 'Performance Baseline 的 PR 与 main push 必须都由 source-lock 触发'
    $manualModeInputPattern = '(?ms)^  workflow_dispatch:\r?\n    inputs:\r?\n      mode:\r?\n        description: .+\r?\n        required: true\r?\n        default: evidence\r?\n        type: choice\r?\n        options:\r?\n          - evidence\r?\n          - measure\r?\n          - observation\r?$'
    Assert-True -Condition ($workflow -match $manualModeInputPattern) -Message '性能基线手工触发必须声明默认 evidence 的受约束 mode 输入'
    Assert-True -Condition ($workflow.Contains('$eventName = ''${{ github.event_name }}''')) -Message '性能基线必须显式区分手工 dispatch 与自动事件'
    Assert-True -Condition ($workflow.Contains('$manualMode = ''${{ github.event.inputs.mode }}''')) -Message '性能基线必须读取手工 mode 输入'
    Assert-True -Condition ($workflow.Contains('if ($eventName -eq ''workflow_dispatch'') {')) -Message '性能基线必须把手工 dispatch 与自动事件分流'
    Assert-True -Condition ($workflow.Contains('if ($manualMode -eq ''measure'') {')) -Message '性能基线只有显式 measure 输入才能进入测量路径'
    Assert-True -Condition ($workflow.Contains('elseif ($manualMode -eq ''evidence'') {')) -Message '性能基线必须保留手工 evidence 分支'
    Assert-True -Condition ($workflow.Contains('elseif ($manualMode -eq ''observation'') {')) -Message '性能基线必须保留手工 observation 分支'
    Assert-True -Condition ($workflow.Contains('手工 evidence 模式要求三份已入库性能证据同时存在')) -Message '手工 evidence 缺失证据时必须明确失败'
    Assert-True -Condition ($workflow -match '(?ms)^          else \{\r?\n            \$mode = ''observation''\r?\n          \}\r?$') -Message '自动 PR/push 必须固定进入 observation 模式'
    Assert-True -Condition ($workflow -match '(?m)^permissions:\r?\n  contents: read\r?$') -Message '性能基线 Workflow 只能读取仓库内容'
    Assert-True -Condition ($workflow -match '(?m)^concurrency:\r?$') -Message '性能基线 Workflow 必须限制重复运行'
    Assert-True -Condition ($workflow -match '(?m)^  cancel-in-progress: true\r?$') -Message '性能基线 Workflow 必须取消同组旧运行'
    Assert-Equal -Expected 1 -Actual ([regex]::Matches($workflow, '(?m)^    runs-on: windows-latest\r?$').Count) -Message '性能基线必须且只能包含一个 Windows hosted Job'
    Assert-Equal -Expected 1 -Actual ([regex]::Matches($workflow, '(?m)^    runs-on: macos-latest\r?$').Count) -Message '性能基线必须且只能包含一个 macOS hosted Job'
    Assert-True -Condition ([regex]::Matches($workflow, '(?m)^    timeout-minutes: \d+\r?$').Count -ge 4) -Message '所有性能 Job 都必须设置超时'
    Assert-True -Condition ($workflow.Contains('scripts/performance/Invoke-InputcodexBaseline.ps1')) -Message '两平台测量必须调用统一测量脚本'
    Assert-True -Condition ($workflow.Contains('scripts/performance/Test-InputcodexBaseline.ps1')) -Message 'Workflow 必须调用统一证据验证脚本'
    Assert-Equal -Expected 2 -Actual ([regex]::Matches($workflow, 'scripts/performance/Invoke-InputcodexBudgetObservation\.ps1').Count) -Message 'Windows/macOS observation 必须各调用一次预算观察器'
    Assert-Equal -Expected 2 -Actual ([regex]::Matches($workflow, [regex]::Escape('$observationOutput | Tee-Object')).Count) -Message '双平台 observation JSON 必须写入日志'
    Assert-Equal -Expected 2 -Actual ([regex]::Matches($workflow, [regex]::Escape('$observationJson[0] | Add-Content -LiteralPath $env:GITHUB_STEP_SUMMARY')).Count) -Message '双平台 observation JSON 必须写入 Step Summary'
    Assert-Equal -Expected 2 -Actual ([regex]::Matches($workflow, "needs.contract.outputs.mode == 'measure' && success\(\)").Count) -Message '只有显式 measure 成功时才能上传临时成功 Artifact'
    Assert-True -Condition ($workflow -match '(?m)^          retention-days: 1\r?$') -Message '成功临时 Artifact 必须只保留 1 天'
    Assert-True -Condition ($workflow -match '(?m)^          retention-days: 7\r?$') -Message '失败诊断 Artifact 最长保留 7 天'
    Assert-True -Condition ($workflow -match '(?ms)^  required:.*?needs:.*?- windows.*?- macos') -Message 'required Job 必须依赖 Windows 与 macOS'
    Assert-True -Condition ($workflow -notmatch '(?im)uses:\s*actions/cache|^\s*cache:') -Message '性能基线 Workflow 禁止 Cache'
    Assert-True -Condition ($workflow -notmatch '(?im)path:.*target|^\s*target[/\\]') -Message '性能基线 Workflow 禁止上传 target'
    Assert-True -Condition ($workflow -notmatch '(?im)uses:\s*[^\s@]+@(?![0-9a-f]{40}\b)') -Message '性能基线 Workflow 的 Action 必须固定到 40 位 SHA'
}

Invoke-ContractTest -Name '性能 Evidence 对 Git 换行转换保持稳定' -Body {
    $validator = Get-Content -LiteralPath $performanceTestScript -Raw -Encoding utf8
    $normalizedResultHashPattern = '\$manifest\.results\.(windows|macos)\.sha256 -eq \(Get-NormalizedTextHash -Path \$(windows|macos)Path\)'

    Assert-Equal -Expected 2 -Actual ([regex]::Matches($validator, $normalizedResultHashPattern).Count) -Message 'Windows/macOS 结果哈希必须统一使用换行归一化文本哈希'
    Assert-True -Condition ($validator -notmatch '(?m)^function Get-RawFileHash\s*\{') -Message '验证器不得保留会受 core.autocrlf 影响的原始工作树文本哈希入口'
}

function Write-Utf8File {
    param(
        [Parameter(Mandatory)]
        [string]$Path,

        [Parameter(Mandatory)]
        [string]$Content
    )

    $parent = Split-Path -Parent $Path
    if (-not [string]::IsNullOrWhiteSpace($parent)) {
        New-Item -ItemType Directory -Path $parent -Force | Out-Null
    }

    Set-Content -LiteralPath $Path -Value $Content -Encoding utf8NoBOM
}

function Copy-PathToPerformanceFixture {
    param(
        [Parameter(Mandatory)]
        [string]$DestinationRoot,

        [Parameter(Mandatory)]
        [string]$RelativePath
    )

    $sourcePath = Join-Path $repositoryRoot $RelativePath
    $destinationPath = Join-Path $DestinationRoot $RelativePath
    $destinationParent = Split-Path -Parent $destinationPath
    New-Item -ItemType Directory -Path $destinationParent -Force | Out-Null
    Copy-Item -LiteralPath $sourcePath -Destination $destinationPath -Recurse -Force
}

function New-PerformanceContractFixture {
    param(
        [Parameter(Mandatory)]
        [string]$Path
    )

    New-Item -ItemType Directory -Path $Path -Force | Out-Null
    foreach ($relativePath in @(
        '.github/workflows/performance-baseline.yml',
        'Cargo.lock',
        'Cargo.toml',
        'rust-toolchain.toml',
        'benchmarks/README.md',
        'benchmarks/config/issue-32-baseline.json',
        'benchmarks/inputcodex-baseline',
        'apps/inputcodex-desktop',
        'crates/inputcodex-domain',
        'crates/inputcodex-application',
        'crates/inputcodex-infrastructure',
        'crates/inputcodex-platform',
        'crates/inputcodex-presentation',
        'crates/inputcodex-parity',
        'parity',
        'upstream/source-lock.json',
        'scripts/ci/Test-CiScripts.ps1',
        'scripts/performance/Invoke-InputcodexBaseline.ps1',
        'scripts/performance/Test-InputcodexBaseline.ps1'
    )) {
        Copy-PathToPerformanceFixture -DestinationRoot $Path -RelativePath $relativePath
    }
}

Invoke-ContractTest -Name '性能实现哈希只绑定被测生产面' -Body {
    $fixtureRoot = Join-Path $testRoot 'performance-contract-boundary'
    New-PerformanceContractFixture -Path $fixtureRoot
    $fixtureValidator = Join-Path $fixtureRoot 'scripts/performance/Test-InputcodexBaseline.ps1'

    $before = Invoke-ChildScript -Path $fixtureValidator -Arguments @('-RepositoryRoot', $fixtureRoot, '-Mode', 'Contract')
    Assert-Equal -Expected 0 -Actual $before.ExitCode -Message "初始性能 Contract 应通过，输出=$($before.Output)"
    Assert-True -Condition ($null -ne $before.Json) -Message "初始性能 Contract 必须输出 JSON，输出=$($before.Output)"

    Write-Utf8File -Path (Join-Path $fixtureRoot 'crates/inputcodex-parity/tests/non-performance-contract.rs') -Content 'fn ordinary_contract_test() {}'
    $afterTestChange = Invoke-ChildScript -Path $fixtureValidator -Arguments @('-RepositoryRoot', $fixtureRoot, '-Mode', 'Contract')
    Assert-Equal -Expected 0 -Actual $afterTestChange.ExitCode -Message "普通测试变化后 Contract 应通过，输出=$($afterTestChange.Output)"

    Add-Content -LiteralPath (Join-Path $fixtureRoot 'crates/inputcodex-parity/src/lib.rs') -Value "`npub const IMPLEMENTATION_HASH_SENTINEL: bool = true;" -Encoding utf8NoBOM
    $afterSourceChange = Invoke-ChildScript -Path $fixtureValidator -Arguments @('-RepositoryRoot', $fixtureRoot, '-Mode', 'Contract')
    Assert-Equal -Expected 0 -Actual $afterSourceChange.ExitCode -Message "产品源码变化后 Contract 应通过，输出=$($afterSourceChange.Output)"

    Assert-Equal -Expected $before.Json.implementation_sha256 -Actual $afterTestChange.Json.implementation_sha256 -Message '普通 crate tests 变化不得改变性能实现哈希'
    Assert-True -Condition ($before.Json.implementation_sha256 -ne $afterSourceChange.Json.implementation_sha256) -Message '产品 src 变化必须改变性能实现哈希'
}

function New-ValidRepositoryFixture {
    param(
        [Parameter(Mandatory)]
        [string]$Path
    )

    New-Item -ItemType Directory -Path $Path -Force | Out-Null

    $workspaceManifest = @"
[workspace]
resolver = "2"
members = [
    "apps/inputcodex-desktop",
    "crates/inputcodex-domain",
    "crates/inputcodex-application",
    "crates/inputcodex-infrastructure",
    "crates/inputcodex-platform",
    "crates/inputcodex-presentation",
    "crates/inputcodex-parity"
]

[workspace.package]
version = "0.1.0"
edition = "2024"
rust-version = "1.97"
license = "AGPL-3.0-only"
"@
    Write-Utf8File -Path (Join-Path $Path 'Cargo.toml') -Content $workspaceManifest

    $domainManifest = @"
[package]
name = "inputcodex-domain"
version.workspace = true
edition.workspace = true
rust-version.workspace = true
license.workspace = true
"@
    Write-Utf8File -Path (Join-Path $Path 'crates/inputcodex-domain/Cargo.toml') -Content $domainManifest
    Write-Utf8File -Path (Join-Path $Path 'crates/inputcodex-domain/src/lib.rs') -Content 'pub struct DomainMarker;'

    $applicationManifest = @"
[package]
name = "inputcodex-application"
version.workspace = true
edition.workspace = true
rust-version.workspace = true
license.workspace = true

[dependencies]
inputcodex-domain = { path = "../inputcodex-domain" }
"@
    Write-Utf8File -Path (Join-Path $Path 'crates/inputcodex-application/Cargo.toml') -Content $applicationManifest
    Write-Utf8File -Path (Join-Path $Path 'crates/inputcodex-application/src/lib.rs') -Content 'pub struct ApplicationMarker;'

    $infrastructureManifest = @"
[package]
name = "inputcodex-infrastructure"
version.workspace = true
edition.workspace = true
rust-version.workspace = true
license.workspace = true

[dependencies]
inputcodex-application = { path = "../inputcodex-application" }
"@
    Write-Utf8File -Path (Join-Path $Path 'crates/inputcodex-infrastructure/Cargo.toml') -Content $infrastructureManifest
    Write-Utf8File -Path (Join-Path $Path 'crates/inputcodex-infrastructure/src/lib.rs') -Content 'pub struct InfrastructureMarker;'

    $platformManifest = @"
[package]
name = "inputcodex-platform"
version.workspace = true
edition.workspace = true
rust-version.workspace = true
license.workspace = true

[dependencies]
inputcodex-application = { path = "../inputcodex-application" }
"@
    Write-Utf8File -Path (Join-Path $Path 'crates/inputcodex-platform/Cargo.toml') -Content $platformManifest
    Write-Utf8File -Path (Join-Path $Path 'crates/inputcodex-platform/src/lib.rs') -Content 'pub struct PlatformMarker;'

    $presentationManifest = @"
[package]
name = "inputcodex-presentation"
version.workspace = true
edition.workspace = true
rust-version.workspace = true
license.workspace = true

[dependencies]
inputcodex-application = { path = "../inputcodex-application" }
iced = "0.14.0"
"@
    Write-Utf8File -Path (Join-Path $Path 'crates/inputcodex-presentation/Cargo.toml') -Content $presentationManifest
    Write-Utf8File -Path (Join-Path $Path 'crates/inputcodex-presentation/src/lib.rs') -Content 'pub struct PresentationMarker;'

    $parityManifest = @"
[package]
name = "inputcodex-parity"
version.workspace = true
edition.workspace = true
rust-version.workspace = true
license.workspace = true

[dependencies]
inputcodex-domain = { path = "../inputcodex-domain" }
inputcodex-application = { path = "../inputcodex-application" }
"@
    Write-Utf8File -Path (Join-Path $Path 'crates/inputcodex-parity/Cargo.toml') -Content $parityManifest
    Write-Utf8File -Path (Join-Path $Path 'crates/inputcodex-parity/src/lib.rs') -Content 'pub struct ParityMarker;'

    $desktopManifest = @"
[package]
name = "inputcodex-desktop"
version.workspace = true
edition.workspace = true
rust-version.workspace = true
license.workspace = true

[dependencies]
inputcodex-application = { path = "../../crates/inputcodex-application" }
inputcodex-infrastructure = { path = "../../crates/inputcodex-infrastructure" }
inputcodex-platform = { path = "../../crates/inputcodex-platform" }
inputcodex-presentation = { path = "../../crates/inputcodex-presentation" }
"@
    Write-Utf8File -Path (Join-Path $Path 'apps/inputcodex-desktop/Cargo.toml') -Content $desktopManifest
    $desktopSource = 'pub const UPDATE_SOURCE: &str = "https://github.com/nonononull/inputcodex/releases/latest";'
    Write-Utf8File -Path (Join-Path $Path 'apps/inputcodex-desktop/src/main.rs') -Content $desktopSource
}

function Copy-RepositoryFixture {
    param(
        [Parameter(Mandatory)]
        [string]$Source,

        [Parameter(Mandatory)]
        [string]$Name
    )

    $destination = Join-Path $testRoot $Name
    Copy-Item -LiteralPath $Source -Destination $destination -Recurse
    $destination
}

function Invoke-PolicyCase {
    param(
        [Parameter(Mandatory)]
        [string]$RepositoryRoot
    )

    Invoke-ChildScript -Path $policyScript -Arguments @('-RepositoryRoot', $RepositoryRoot)
}

function Assert-PolicyFailureCode {
    param(
        [Parameter(Mandatory)]
        $Result,

        [Parameter(Mandatory)]
        [string]$Code
    )

    Assert-True -Condition ($Result.ExitCode -ne 0) -Message "违规仓库必须返回非零退出码，输出=$($Result.Output)"
    Assert-True -Condition ($null -ne $Result.Json) -Message "仓库政策脚本必须输出 JSON，输出=$($Result.Output)"
    Assert-Contains -Collection @($Result.Json.violations.code) -Expected $Code -Message '仓库政策脚本必须返回稳定违规码'
}

$validRepository = Join-Path $testRoot 'repository-valid'
New-ValidRepositoryFixture -Path $validRepository

Invoke-ContractTest -Name '合法七成员 Workspace 通过仓库政策' -Body {
    $result = Invoke-PolicyCase -RepositoryRoot $validRepository
    Assert-Equal -Expected 0 -Actual $result.ExitCode -Message "合法仓库应通过，输出=$($result.Output)"
    Assert-True -Condition ($null -ne $result.Json) -Message "仓库政策脚本必须输出 JSON，输出=$($result.Output)"
    Assert-Equal -Expected $true -Actual $result.Json.ok -Message '合法仓库的 ok 必须为 true'
    Assert-Equal -Expected 0 -Actual $result.Json.violation_count -Message '合法仓库不应含违规项'
}

Invoke-ContractTest -Name '拒绝非 AGPL-3.0-only Workspace 许可证' -Body {
    $repository = Copy-RepositoryFixture -Source $validRepository -Name 'repository-license'
    $manifestPath = Join-Path $repository 'Cargo.toml'
    $manifest = Get-Content -LiteralPath $manifestPath -Raw -Encoding utf8
    $manifest = $manifest.Replace('license = "AGPL-3.0-only"', 'license = "MIT"')
    Set-Content -LiteralPath $manifestPath -Value $manifest -Encoding utf8NoBOM
    $result = Invoke-PolicyCase -RepositoryRoot $repository
    Assert-PolicyFailureCode -Result $result -Code 'WORKSPACE_LICENSE_INVALID'
}

Invoke-ContractTest -Name '拒绝 Iced 越过展示层' -Body {
    $repository = Copy-RepositoryFixture -Source $validRepository -Name 'repository-iced-layer'
    Add-Content -LiteralPath (Join-Path $repository 'crates/inputcodex-domain/Cargo.toml') -Value "`n[dependencies]`niced = `"0.14.0`"" -Encoding utf8NoBOM
    $result = Invoke-PolicyCase -RepositoryRoot $repository
    Assert-PolicyFailureCode -Result $result -Code 'ICED_LAYER_VIOLATION'
}

Invoke-ContractTest -Name '拒绝 upstream 加入 Workspace' -Body {
    $repository = Copy-RepositoryFixture -Source $validRepository -Name 'repository-upstream-member'
    $manifestPath = Join-Path $repository 'Cargo.toml'
    $manifest = Get-Content -LiteralPath $manifestPath -Raw
    $replacement = @(
        '    "crates/inputcodex-parity",'
        '    "upstream/CodexPlusPlus"'
    ) -join [Environment]::NewLine
    $manifest = $manifest.Replace('    "crates/inputcodex-parity"', $replacement)
    Set-Content -LiteralPath $manifestPath -Value $manifest -Encoding utf8NoBOM
    $result = Invoke-PolicyCase -RepositoryRoot $repository
    Assert-PolicyFailureCode -Result $result -Code 'UPSTREAM_WORKSPACE_MEMBER'
}

Invoke-ContractTest -Name '拒绝生产目录 TypeScript 文件' -Body {
    $repository = Copy-RepositoryFixture -Source $validRepository -Name 'repository-typescript'
    Write-Utf8File -Path (Join-Path $repository 'apps/inputcodex-desktop/src/main.ts') -Content 'export const forbidden = true;'
    $result = Invoke-PolicyCase -RepositoryRoot $repository
    Assert-PolicyFailureCode -Result $result -Code 'SCRIPT_LANGUAGE_FORBIDDEN'
}

Invoke-ContractTest -Name '拒绝 Tauri 运行时依赖' -Body {
    $repository = Copy-RepositoryFixture -Source $validRepository -Name 'repository-tauri'
    Add-Content -LiteralPath (Join-Path $repository 'apps/inputcodex-desktop/Cargo.toml') -Value 'tauri = "2.0.0"' -Encoding utf8NoBOM
    $result = Invoke-PolicyCase -RepositoryRoot $repository
    Assert-PolicyFailureCode -Result $result -Code 'WEB_RUNTIME_DEPENDENCY_FORBIDDEN'
}

Invoke-ContractTest -Name '拒绝 TOML 表形式的 Tauri 别名依赖' -Body {
    $repository = Copy-RepositoryFixture -Source $validRepository -Name 'repository-tauri-table'
    $tableDependency = "`n[dependencies.desktop-runtime]`npackage = `"tauri`"`nversion = `"2.0.0`""
    Add-Content -LiteralPath (Join-Path $repository 'apps/inputcodex-desktop/Cargo.toml') -Value $tableDependency -Encoding utf8NoBOM
    $result = Invoke-PolicyCase -RepositoryRoot $repository
    Assert-PolicyFailureCode -Result $result -Code 'WEB_RUNTIME_DEPENDENCY_FORBIDDEN'
}

Invoke-ContractTest -Name '拒绝 WebView 运行时依赖' -Body {
    $repository = Copy-RepositoryFixture -Source $validRepository -Name 'repository-webview'
    Add-Content -LiteralPath (Join-Path $repository 'apps/inputcodex-desktop/Cargo.toml') -Value 'wry = "0.53.0"' -Encoding utf8NoBOM
    $result = Invoke-PolicyCase -RepositoryRoot $repository
    Assert-PolicyFailureCode -Result $result -Code 'WEB_RUNTIME_DEPENDENCY_FORBIDDEN'
}

Invoke-ContractTest -Name '拒绝广告依赖' -Body {
    $repository = Copy-RepositoryFixture -Source $validRepository -Name 'repository-advertising'
    Add-Content -LiteralPath (Join-Path $repository 'apps/inputcodex-desktop/Cargo.toml') -Value 'admob = "0.3.0"' -Encoding utf8NoBOM
    $result = Invoke-PolicyCase -RepositoryRoot $repository
    Assert-PolicyFailureCode -Result $result -Code 'AD_TELEMETRY_DEPENDENCY_FORBIDDEN'
}

Invoke-ContractTest -Name '拒绝远程遥测依赖' -Body {
    $repository = Copy-RepositoryFixture -Source $validRepository -Name 'repository-telemetry'
    Add-Content -LiteralPath (Join-Path $repository 'apps/inputcodex-desktop/Cargo.toml') -Value 'sentry = "0.40.0"' -Encoding utf8NoBOM
    $result = Invoke-PolicyCase -RepositoryRoot $repository
    Assert-PolicyFailureCode -Result $result -Code 'AD_TELEMETRY_DEPENDENCY_FORBIDDEN'
}

Invoke-ContractTest -Name '拒绝非本仓 Release 或更新源' -Body {
    $repository = Copy-RepositoryFixture -Source $validRepository -Name 'repository-update-source'
    $sourcePath = Join-Path $repository 'apps/inputcodex-desktop/src/main.rs'
    $source = (Get-Content -LiteralPath $sourcePath -Raw).Replace('nonononull/inputcodex', 'BigPizzaV3/CodexPlusPlus')
    Set-Content -LiteralPath $sourcePath -Value $source -Encoding utf8NoBOM
    $result = Invoke-PolicyCase -RepositoryRoot $repository
    Assert-PolicyFailureCode -Result $result -Code 'UPDATE_SOURCE_FORBIDDEN'
}

Invoke-ContractTest -Name '拒绝 Workspace 依赖方向反转' -Body {
    $repository = Copy-RepositoryFixture -Source $validRepository -Name 'repository-dependency-direction'
    Add-Content -LiteralPath (Join-Path $repository 'crates/inputcodex-domain/Cargo.toml') -Value "`n[dependencies]`ninputcodex-presentation = { path = `"../inputcodex-presentation`" }" -Encoding utf8NoBOM
    $result = Invoke-PolicyCase -RepositoryRoot $repository
    Assert-PolicyFailureCode -Result $result -Code 'DEPENDENCY_DIRECTION_INVALID'
}

$forbiddenDirectDependencyCases = @(
    [pscustomobject]@{
        name = 'infrastructure-domain'
        manifest = 'crates/inputcodex-infrastructure/Cargo.toml'
        dependency = 'inputcodex-domain = { path = "../inputcodex-domain" }'
    }
    [pscustomobject]@{
        name = 'platform-domain'
        manifest = 'crates/inputcodex-platform/Cargo.toml'
        dependency = 'inputcodex-domain = { path = "../inputcodex-domain" }'
    }
    [pscustomobject]@{
        name = 'presentation-domain'
        manifest = 'crates/inputcodex-presentation/Cargo.toml'
        dependency = 'inputcodex-domain = { path = "../inputcodex-domain" }'
    }
    [pscustomobject]@{
        name = 'parity-platform'
        manifest = 'crates/inputcodex-parity/Cargo.toml'
        dependency = 'inputcodex-platform = { path = "../inputcodex-platform" }'
    }
)

foreach ($forbiddenDirectDependencyCase in $forbiddenDirectDependencyCases) {
    Invoke-ContractTest -Name "拒绝越过批准箭头 $($forbiddenDirectDependencyCase.name)" -Body {
        $repository = Copy-RepositoryFixture -Source $validRepository -Name "repository-$($forbiddenDirectDependencyCase.name)"
        Add-Content -LiteralPath (Join-Path $repository $forbiddenDirectDependencyCase.manifest) -Value $forbiddenDirectDependencyCase.dependency -Encoding utf8NoBOM
        $result = Invoke-PolicyCase -RepositoryRoot $repository
        Assert-PolicyFailureCode -Result $result -Code 'DEPENDENCY_DIRECTION_INVALID'
    }
}

Invoke-ContractTest -Name '拒绝 TOML 表形式的依赖方向反转' -Body {
    $repository = Copy-RepositoryFixture -Source $validRepository -Name 'repository-dependency-table-direction'
    $tableDependency = "`n[dependencies.ui-layer]`npackage = `"inputcodex-presentation`"`npath = `"../inputcodex-presentation`""
    Add-Content -LiteralPath (Join-Path $repository 'crates/inputcodex-domain/Cargo.toml') -Value $tableDependency -Encoding utf8NoBOM
    $result = Invoke-PolicyCase -RepositoryRoot $repository
    Assert-PolicyFailureCode -Result $result -Code 'DEPENDENCY_DIRECTION_INVALID'
}

function Invoke-TestGit {
    param(
        [Parameter(Mandatory)]
        [string]$RepositoryRoot,

        [Parameter(Mandatory)]
        [string[]]$Arguments
    )

    $output = @(& git -C $RepositoryRoot @Arguments 2>&1 | ForEach-Object { $_.ToString() })
    if ($LASTEXITCODE -ne 0) {
        throw "测试 Git 命令失败：git -C $RepositoryRoot $($Arguments -join ' ')；输出=$($output -join [Environment]::NewLine)"
    }

    ,$output
}

Invoke-ContractTest -Name 'Git 变更收集器保留新增修改删除和重命名' -Body {
    Assert-True -Condition (Test-Path -LiteralPath $collectorScript -PathType Leaf) -Message 'CI_COLLECTOR_RED_MISSING_IMPLEMENTATION'

    $repository = Join-Path $testRoot 'collector-repository'
    New-Item -ItemType Directory -Path $repository -Force | Out-Null
    Invoke-TestGit -RepositoryRoot $repository -Arguments @('init', '--quiet') | Out-Null
    Invoke-TestGit -RepositoryRoot $repository -Arguments @('config', 'user.name', 'inputcodex-ci-test') | Out-Null
    Invoke-TestGit -RepositoryRoot $repository -Arguments @('config', 'user.email', 'ci-test@inputcodex.invalid') | Out-Null

    Write-Utf8File -Path (Join-Path $repository 'README.md') -Content 'initial'
    Write-Utf8File -Path (Join-Path $repository 'Cargo.lock') -Content 'lock'
    Write-Utf8File -Path (Join-Path $repository 'docs/old.md') -Content 'rename-me'
    Invoke-TestGit -RepositoryRoot $repository -Arguments @('add', '--all') | Out-Null
    Invoke-TestGit -RepositoryRoot $repository -Arguments @('commit', '--quiet', '-m', 'initial') | Out-Null
    $base = (Invoke-TestGit -RepositoryRoot $repository -Arguments @('rev-parse', 'HEAD'))[0].Trim()

    Write-Utf8File -Path (Join-Path $repository 'README.md') -Content 'changed'
    Remove-Item -LiteralPath (Join-Path $repository 'Cargo.lock') -Force
    Invoke-TestGit -RepositoryRoot $repository -Arguments @('mv', 'docs/old.md', 'docs/new.md') | Out-Null
    Write-Utf8File -Path (Join-Path $repository 'crates/inputcodex-domain/src/lib.rs') -Content '#![forbid(unsafe_code)]'
    Invoke-TestGit -RepositoryRoot $repository -Arguments @('add', '--all') | Out-Null
    Invoke-TestGit -RepositoryRoot $repository -Arguments @('commit', '--quiet', '-m', 'changes') | Out-Null
    $head = (Invoke-TestGit -RepositoryRoot $repository -Arguments @('rev-parse', 'HEAD'))[0].Trim()

    $outputFile = Join-Path $testRoot 'collector-output.json'
    $result = Invoke-ChildScript -Path $collectorScript -Arguments @(
        '-RepositoryRoot', $repository,
        '-Base', $base,
        '-Head', $head,
        '-OutputFile', $outputFile
    )
    Assert-Equal -Expected 0 -Actual $result.ExitCode -Message "变更收集器应成功，输出=$($result.Output)"
    Assert-True -Condition (Test-Path -LiteralPath $outputFile -PathType Leaf) -Message '变更收集器必须写出 JSON 文件'

    $changes = @(Get-Content -LiteralPath $outputFile -Raw -Encoding utf8 | ConvertFrom-Json -Depth 20)
    Assert-Equal -Expected 4 -Actual $changes.Count -Message '变更收集器记录数必须稳定'
    Assert-Equal -Expected 1 -Actual @($changes | Where-Object { $_.status -eq 'M' -and $_.path -eq 'README.md' }).Count -Message '必须保留修改记录'
    Assert-Equal -Expected 1 -Actual @($changes | Where-Object { $_.status -eq 'D' -and $_.path -eq 'Cargo.lock' }).Count -Message '必须保留删除记录'
    Assert-Equal -Expected 1 -Actual @($changes | Where-Object { $_.status -eq 'A' -and $_.path -eq 'crates/inputcodex-domain/src/lib.rs' }).Count -Message '必须保留新增记录'
    Assert-Equal -Expected 1 -Actual @($changes | Where-Object { $_.status -eq 'R' -and $_.old_path -eq 'docs/old.md' -and $_.path -eq 'docs/new.md' }).Count -Message '必须保留重命名新旧路径'
}

$resolvedTestRoot = [System.IO.Path]::GetFullPath($testRoot)
$resolvedTemporaryRoot = [System.IO.Path]::GetFullPath([System.IO.Path]::GetTempPath())
$isSafeTestRoot = $resolvedTestRoot.StartsWith($resolvedTemporaryRoot, [System.StringComparison]::OrdinalIgnoreCase) -and
    ([System.IO.Path]::GetFileName($resolvedTestRoot) -like 'inputcodex-ci-contract-*')

if ($isSafeTestRoot -and (Test-Path -LiteralPath $resolvedTestRoot)) {
    Remove-Item -LiteralPath $resolvedTestRoot -Recurse -Force
}

if ($script:Failures.Count -gt 0) {
    [Console]::Error.WriteLine('CI_CONTRACT_TEST_FAILURE')
    foreach ($failure in $script:Failures) {
        [Console]::Error.WriteLine($failure)
    }

    exit 1
}

Write-Host "CI_CONTRACT_GREEN passed=$script:PassedCount"
exit 0
