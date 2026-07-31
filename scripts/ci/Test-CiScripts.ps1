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
                    jobs = @('contract', 'windows', 'macos', 'required')
                }
            )
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
        [string]$HeadOid = 'bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb'
    )

    [pscustomobject][ordered]@{
        name = $Name
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
            tracking_issue_ref = 'https://github.com/nonononull/inputcodex/issues/111'
            standing_authorization_ref = 'https://github.com/nonononull/inputcodex/issues/111'
            policy_sha256 = 'sha256:2cb8467153892fcb1510c86cdcb186cd9dabc3d4f08055ec9c503b823d760275'
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
            (New-SuccessfulAutonomousWorkflowEvidence -Name 'CI' -RunId 2001 -HeadOid 'cccccccccccccccccccccccccccccccccccccccc' -Jobs @('classify', 'governance', 'release-audit', 'linux-quality', 'windows', 'macos', 'required')),
            (New-SuccessfulAutonomousWorkflowEvidence -Name 'Performance Baseline' -RunId 2002 -HeadOid 'cccccccccccccccccccccccccccccccccccccccc' -Jobs @('contract', 'windows', 'macos', 'required'))
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
        [pscustomobject]@{ Name = 'performance-job'; Code = 'WORKFLOW_PERFORMANCE_BASELINE' }
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
        [object[]]$Changes
    )

    $caseRoot = Join-Path $testRoot ("release-audit-{0}" -f $Name)
    $upstreamRoot = Join-Path $caseRoot 'upstream'
    New-Item -ItemType Directory -Path $upstreamRoot -Force | Out-Null

    $baseSourceLockPath = Join-Path $caseRoot 'base-source-lock.json'
    $headSourceLockPath = Join-Path $upstreamRoot 'source-lock.json'
    $changesPath = Join-Path $caseRoot 'changes.json'
    Write-JsonFile -Path $baseSourceLockPath -Value $BaseSourceLock
    Write-JsonFile -Path $headSourceLockPath -Value $HeadSourceLock
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
