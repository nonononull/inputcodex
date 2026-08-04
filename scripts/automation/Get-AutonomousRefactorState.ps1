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

function Get-PropertyProjection {
    param(
        [AllowNull()]$Value,
        [Parameter(Mandatory)][string]$Name
    )

    $property = if ($null -eq $Value) { $null } else { $Value.PSObject.Properties[$Name] }
    $projection = [pscustomobject][ordered]@{
        exists = ($null -ne $property)
        value = $null
    }
    if ($null -ne $property) {
        $projection.value = $property.Value
    }
    return $projection
}

function Get-ReleaseAuditStatusProjection {
    param([AllowNull()]$SourceLock)

    if ($SourceLock -isnot [System.Management.Automation.PSCustomObject]) {
        return 'invalid'
    }
    $audit = Get-PropertyProjection $SourceLock 'release_audit'
    if (-not $audit.exists -or
        $audit.value -isnot [System.Management.Automation.PSCustomObject]) {
        return 'invalid'
    }
    $schema = Get-PropertyProjection $audit.value 'schema_version'
    $status = Get-PropertyProjection $audit.value 'status'
    if (-not $schema.exists -or
        $schema.value -isnot [string] -or
        -not [string]::Equals(
            $schema.value,
            'inputcodex.release-audit.v1',
            [StringComparison]::Ordinal
        ) -or
        -not $status.exists -or
        $status.value -isnot [string]) {
        return 'invalid'
    }
    if ([string]::Equals($status.value, 'current', [StringComparison]::Ordinal)) {
        return 'current'
    }
    if ([string]::Equals($status.value, 'stale-re-audit-required', [StringComparison]::Ordinal)) {
        return 'stale-re-audit-required'
    }
    return 'invalid'
}

function ConvertFrom-StrictJsonArrayOutput {
    param(
        [AllowEmptyCollection()][object[]]$Output,
        [Parameter(Mandatory)][string]$Label
    )

    $raw = [string]::Join([Environment]::NewLine, @($Output | ForEach-Object { $_.ToString() }))
    if ([string]::IsNullOrWhiteSpace($raw)) {
        throw "$Label returned empty output"
    }
    try {
        $value = $raw | ConvertFrom-Json -Depth 100 -NoEnumerate
    } catch {
        throw "$Label returned invalid JSON"
    }
    if ($value -isnot [System.Array]) {
        throw "$Label must return a JSON array"
    }
    return [pscustomobject]@{ Items = $value }
}

function ConvertFrom-StrictJsonObjectOutput {
    param(
        [AllowEmptyCollection()][object[]]$Output,
        [Parameter(Mandatory)][string]$Label
    )

    $raw = [string]::Join([Environment]::NewLine, @($Output | ForEach-Object { $_.ToString() }))
    if ([string]::IsNullOrWhiteSpace($raw)) {
        throw "$Label returned empty output"
    }
    try {
        $value = $raw | ConvertFrom-Json -Depth 100 -NoEnumerate
    } catch {
        throw "$Label returned invalid JSON"
    }
    if ($value -isnot [System.Management.Automation.PSCustomObject]) {
        throw "$Label must return a JSON object"
    }
    return [pscustomobject]@{ Value = $value }
}

function ConvertFrom-StrictPagedArrayOutput {
    param(
        [AllowEmptyCollection()][object[]]$Output,
        [Parameter(Mandatory)][string]$Label
    )

    $pages = ConvertFrom-StrictJsonArrayOutput -Output $Output -Label $Label
    $items = [System.Collections.Generic.List[object]]::new()
    foreach ($page in $pages.Items) {
        if ($page -isnot [System.Array]) {
            throw "$Label page must be a JSON array"
        }
        foreach ($item in $page) {
            $items.Add($item) | Out-Null
        }
    }
    return [pscustomobject]@{ Items = $items.ToArray() }
}

function ConvertFrom-StrictPagedObjectOutput {
    param(
        [AllowEmptyCollection()][object[]]$Output,
        [Parameter(Mandatory)][string]$Label
    )

    $pages = ConvertFrom-StrictJsonArrayOutput -Output $Output -Label $Label
    foreach ($page in $pages.Items) {
        if ($page -isnot [pscustomobject]) {
            throw "$Label page must be a JSON object"
        }
    }
    return [pscustomobject]@{ Pages = $pages.Items }
}

function Test-AutonomousIssueTaskMarkerPresence {
    param([AllowNull()]$Body)

    if ($Body -isnot [string]) {
        return $false
    }
    $options = [Text.RegularExpressions.RegexOptions]::CultureInvariant -bor
        [Text.RegularExpressions.RegexOptions]::IgnoreCase
    return [regex]::IsMatch(
        $Body,
        '<!--\s*inputcodex:autonomous-refactor-(?:bootstrap|task):v1\s*-->',
        $options
    ) -or [regex]::IsMatch(
        $Body,
        '<!--\s*inputcodex:autonomous-refactor-task-kind:[^>]*-->',
        $options
    )
}

function Get-AutonomousIssueTaskKind {
    param(
        [AllowNull()]$Body,
        [Parameter(Mandatory)][string]$UpstreamSyncTaskMarker,
        [Parameter(Mandatory)][string]$CandidateExhaustedTaskMarker
    )

    if ($Body -isnot [string]) {
        return 'refactor'
    }
    $matches = [regex]::Matches(
        $Body,
        '<!--\s*inputcodex:autonomous-refactor-task-kind:[^>]*-->',
        [Text.RegularExpressions.RegexOptions]::CultureInvariant -bor
            [Text.RegularExpressions.RegexOptions]::IgnoreCase
    )
    if ($matches.Count -eq 0) {
        return 'refactor'
    }
    if ($matches.Count -ne 1) {
        return 'invalid'
    }
    $taskKind = if ($matches[0].Value -ceq $UpstreamSyncTaskMarker) {
        'upstream-sync'
    } elseif ($matches[0].Value -ceq $CandidateExhaustedTaskMarker) {
        'candidate-exhausted'
    } else {
        return 'invalid'
    }
    $normalizedBody = $Body.Replace("`r`n", "`n").Replace("`r", "`n")
    $expectedHeader = "<!-- inputcodex:autonomous-refactor-task:v1 -->`n$($matches[0].Value)"
    if ($normalizedBody -cne $expectedHeader -and
        -not $normalizedBody.StartsWith("$expectedHeader`n", [StringComparison]::Ordinal)) {
        return 'invalid'
    }
    return $taskKind
}

function Get-AutonomousPrBodyEvidence {
    param([AllowNull()]$Body)

    $invalid = [pscustomobject][ordered]@{ valid = $false }
    if ($Body -isnot [string]) {
        return $invalid
    }
    $match = [regex]::Match(
        $Body,
        '(?s)<!--\s*inputcodex:autonomous-refactor-evidence:v1\s*(\{.*?\})\s*-->'
    )
    if (-not $match.Success) {
        return $invalid
    }
    try {
        $evidence = $match.Groups[1].Value | ConvertFrom-Json -Depth 30 -NoEnumerate
    } catch {
        return $invalid
    }
    $taskKindProjection = Get-PropertyProjection $evidence 'task_kind'
    $taskKind = $taskKindProjection.value
    if (-not $taskKindProjection.exists) {
        $taskKind = 'refactor'
    } elseif ($taskKind -isnot [string] -or $taskKind -cnotin @('refactor', 'upstream-sync')) {
        return $invalid
    }
    if ($evidence -isnot [pscustomobject] -or
        (Get-PropertyValue $evidence 'schema_version') -cne 'inputcodex.autonomous-refactor-evidence.v1' -or
        (Get-PropertyValue $evidence 'tracking_issue_ref') -isnot [string] -or
        (Get-PropertyValue $evidence 'standing_authorization_ref') -isnot [string] -or
        [string](Get-PropertyValue $evidence 'policy_sha256') -cnotmatch '^sha256:[0-9a-f]{64}$' -or
        (Get-PropertyValue $evidence 'scope_count') -isnot [long] -or
        (Get-PropertyValue $evidence 'scope_count') -lt 0 -or
        [string](Get-PropertyValue $evidence 'scope_hash') -cnotmatch '^sha256:[0-9a-f]{64}$' -or
        [string](Get-PropertyValue $evidence 'final_head') -cnotmatch '^[0-9a-f]{40}$' -or
        (Get-PropertyValue $evidence 'independent_review_status') -isnot [string] -or
        (Get-PropertyValue $evidence 'independent_review_ref') -isnot [string]) {
        return $invalid
    }
    return [pscustomobject][ordered]@{
        valid = $true
        task_kind = $taskKind
        tracking_issue_ref = Get-PropertyValue $evidence 'tracking_issue_ref'
        standing_authorization_ref = Get-PropertyValue $evidence 'standing_authorization_ref'
        policy_sha256 = Get-PropertyValue $evidence 'policy_sha256'
        scope_count = Get-PropertyValue $evidence 'scope_count'
        scope_hash = Get-PropertyValue $evidence 'scope_hash'
        final_head = Get-PropertyValue $evidence 'final_head'
        independent_review_status = Get-PropertyValue $evidence 'independent_review_status'
        independent_review_ref = Get-PropertyValue $evidence 'independent_review_ref'
    }
}

function Get-GitHubComments {
    param(
        [Parameter(Mandatory)][string]$GhExecutable,
        [Parameter(Mandatory)][long]$Number,
        [Parameter(Mandatory)][string]$Label
    )

    $output = @(
        & $GhExecutable api --paginate --slurp "repos/nonononull/inputcodex/issues/$Number/comments?per_page=100" 2>$null
    )
    if ($LASTEXITCODE -ne 0) { throw "$Label failed" }
    $comments = (ConvertFrom-StrictPagedArrayOutput -Output $output -Label $Label).Items
    foreach ($comment in $comments) {
        $user = Get-PropertyValue $comment 'user'
        $body = Get-PropertyValue $comment 'body'
        if ((Get-PropertyValue $comment 'id') -isnot [long] -or
            (Get-PropertyValue $comment 'html_url') -isnot [string] -or
            (Get-PropertyValue $user 'login') -isnot [string] -or
            ($null -ne $body -and $body -isnot [string])) {
            throw "$Label item schema invalid"
        }
    }
    return [pscustomobject]@{ Comments = $comments }
}

function Get-GitHubPlanningEvidence {
    param(
        [Parameter(Mandatory)][string]$GhExecutable,
        [Parameter(Mandatory)][long]$IssueNumber
    )

    $comments = (Get-GitHubComments -GhExecutable $GhExecutable -Number $IssueNumber -Label 'gh planning comments').Comments
    $marker = "<!-- inputcodex:issue-$IssueNumber`:planning-freeze:v1 -->"
    $matches = @($comments | Where-Object {
        (Get-PropertyValue (Get-PropertyValue $_ 'user') 'login') -ceq 'nonononull' -and
        [string](Get-PropertyValue $_ 'body') -match [regex]::Escape($marker)
    } | Sort-Object { Get-PropertyValue $_ 'id' } -Descending)
    foreach ($comment in $matches) {
        $body = [string](Get-PropertyValue $comment 'body')
        $countMatch = [regex]::Match($body, '(?m)^- candidate_scope:\s*([0-9]+)\s*$')
        $hashMatch = [regex]::Match($body, '(?m)^- candidate_scope_hash:\s*(sha256:[0-9a-f]{64})\s*$')
        $taskKindMatches = [regex]::Matches($body, '(?m)^- task_kind:\s*(\S+)\s*$')
        $taskKind = 'refactor'
        if ($taskKindMatches.Count -gt 1) {
            continue
        }
        if ($taskKindMatches.Count -eq 1) {
            $taskKind = $taskKindMatches[0].Groups[1].Value
            if ($taskKind -cnotin @('refactor', 'upstream-sync')) {
                continue
            }
        }
        $scopeCount = 0L
        if ($countMatch.Success -and $hashMatch.Success -and
            [long]::TryParse($countMatch.Groups[1].Value, [ref]$scopeCount)) {
            return [pscustomobject][ordered]@{
                valid = $true
                task_kind = $taskKind
                scope_count = $scopeCount
                scope_hash = $hashMatch.Groups[1].Value
                ref = Get-PropertyValue $comment 'html_url'
            }
        }
    }
    return [pscustomobject][ordered]@{ valid = $false }
}

function Get-GitHubReviewAttestation {
    param(
        [Parameter(Mandatory)][string]$GhExecutable,
        [Parameter(Mandatory)][long]$PullRequestNumber
    )

    $comments = (Get-GitHubComments -GhExecutable $GhExecutable -Number $PullRequestNumber -Label 'gh review comments').Comments
    $matches = @($comments | Where-Object {
        (Get-PropertyValue (Get-PropertyValue $_ 'user') 'login') -ceq 'nonononull' -and
        [string](Get-PropertyValue $_ 'body') -match
            '<!--\s*inputcodex:autonomous-refactor-review:v1\s*-->'
    } | Sort-Object { Get-PropertyValue $_ 'id' } -Descending)
    foreach ($comment in $matches) {
        $body = [string](Get-PropertyValue $comment 'body')
        $headMatch = [regex]::Match($body, '(?m)^- final_head:\s*`?([0-9a-f]{40})`?\s*$')
        $statusMatch = [regex]::Match($body, '(?im)^- result:\s*(PASSED|FAILED)\s*$')
        if ($headMatch.Success -and $statusMatch.Success) {
            return [pscustomobject][ordered]@{
                valid = $true
                final_head = $headMatch.Groups[1].Value
                status = $statusMatch.Groups[1].Value.ToLowerInvariant()
                ref = Get-PropertyValue $comment 'html_url'
            }
        }
    }
    return [pscustomobject][ordered]@{ valid = $false }
}

function Get-GitHubPullRequestReviewState {
    param(
        [Parameter(Mandatory)][string]$GhExecutable,
        [Parameter(Mandatory)][long]$Number
    )

    $query = @'
query($owner:String!,$name:String!,$number:Int!,$endCursor:String){
  repository(owner:$owner,name:$name){
    pullRequest(number:$number){
      isDraft
      mergeStateStatus
      headRefOid
      baseRefName
      reviewThreads(first:100,after:$endCursor){
        nodes{isResolved}
        pageInfo{hasNextPage endCursor}
      }
    }
  }
}
'@
    $output = @(
        & $GhExecutable api graphql --paginate --slurp `
            -F owner=nonononull -F name=inputcodex -F number=$Number -f query=$query 2>$null
    )
    if ($LASTEXITCODE -ne 0) { throw 'gh pr review state failed' }
    $pages = (ConvertFrom-StrictPagedObjectOutput -Output $output -Label 'gh pr review state').Pages
    if ($pages.Count -lt 1) { throw 'gh pr review state empty' }

    $unresolvedThreads = 0L
    $firstPullRequest = $null
    foreach ($page in $pages) {
        $data = Get-PropertyValue $page 'data'
        $repository = Get-PropertyValue $data 'repository'
        $pullRequest = Get-PropertyValue $repository 'pullRequest'
        if ($pullRequest -isnot [pscustomobject] -or
            (Get-PropertyValue $pullRequest 'isDraft') -isnot [bool] -or
            (Get-PropertyValue $pullRequest 'mergeStateStatus') -isnot [string] -or
            [string](Get-PropertyValue $pullRequest 'headRefOid') -cnotmatch '^[0-9a-f]{40}$' -or
            (Get-PropertyValue $pullRequest 'baseRefName') -isnot [string]) {
            throw 'gh pr review state schema invalid'
        }
        if ($null -eq $firstPullRequest) {
            $firstPullRequest = $pullRequest
        } elseif ((Get-PropertyValue $pullRequest 'headRefOid') -cne (Get-PropertyValue $firstPullRequest 'headRefOid')) {
            throw 'gh pr head changed during pagination'
        }
        $reviewThreads = Get-PropertyValue $pullRequest 'reviewThreads'
        $nodesProperty = $reviewThreads.PSObject.Properties['nodes']
        if ($null -eq $nodesProperty -or $nodesProperty.Value -isnot [System.Array]) {
            throw 'gh review thread nodes schema invalid'
        }
        foreach ($node in $nodesProperty.Value) {
            if ((Get-PropertyValue $node 'isResolved') -isnot [bool]) {
                throw 'gh review thread schema invalid'
            }
            if ((Get-PropertyValue $node 'isResolved') -ne $true) {
                $unresolvedThreads += 1
            }
        }
    }
    return [pscustomobject][ordered]@{
        is_draft = Get-PropertyValue $firstPullRequest 'isDraft'
        merge_state = Get-PropertyValue $firstPullRequest 'mergeStateStatus'
        head_oid = Get-PropertyValue $firstPullRequest 'headRefOid'
        base_ref = Get-PropertyValue $firstPullRequest 'baseRefName'
        review_thread_count = [long]$unresolvedThreads
    }
}

function Test-GitHubWorkflowPullRequestAssociation {
    param(
        [Parameter(Mandatory)]$Run,
        [Parameter(Mandatory)][long]$PullRequestNumber,
        [Parameter(Mandatory)][string]$HeadOid
    )

    $pullRequestsProperty = $Run.PSObject.Properties['pull_requests']
    if ($null -eq $pullRequestsProperty -or $pullRequestsProperty.Value -isnot [System.Array]) {
        throw 'gh workflow pull_requests schema invalid'
    }
    $matches = @($pullRequestsProperty.Value | Where-Object {
        $head = Get-PropertyValue $_ 'head'
        $base = Get-PropertyValue $_ 'base'
        (Get-PropertyValue $_ 'number') -is [long] -and
        (Get-PropertyValue $_ 'number') -eq $PullRequestNumber -and
        (Get-PropertyValue $head 'sha') -ceq $HeadOid -and
        (Get-PropertyValue $base 'ref') -ceq 'main'
    })
    return $matches.Count -eq 1
}

function Test-GitHubWorkflowRunIdentity {
    param(
        [Parameter(Mandatory)]$Run,
        [Parameter(Mandatory)]$Expectation,
        [Parameter(Mandatory)][string]$HeadOid,
        [Parameter(Mandatory)][ValidateSet('pull_request', 'push')][string]$Event,
        [Parameter(Mandatory)][long]$PullRequestNumber
    )

    if (($Event -ceq 'pull_request' -and $PullRequestNumber -lt 1) -or
        ($Event -ceq 'push' -and $PullRequestNumber -ne 0)) {
        return $false
    }
    if ((Get-PropertyValue $Run 'name') -cne (Get-PropertyValue $Expectation 'name') -or
        (Get-PropertyValue $Run 'workflow_id') -isnot [long] -or
        (Get-PropertyValue $Run 'workflow_id') -ne (Get-PropertyValue $Expectation 'workflow_id') -or
        (Get-PropertyValue $Run 'path') -cne (Get-PropertyValue $Expectation 'path') -or
        (Get-PropertyValue $Run 'event') -cne $Event -or
        (Get-PropertyValue $Run 'head_sha') -cne $HeadOid) {
        return $false
    }
    return ($Event -ceq 'push' -or
        (Test-GitHubWorkflowPullRequestAssociation `
            -Run $Run `
            -PullRequestNumber $PullRequestNumber `
            -HeadOid $HeadOid))
}

function Get-GitHubWorkflowEvidence {
    param(
        [Parameter(Mandatory)][string]$GhExecutable,
        [Parameter(Mandatory)][string]$HeadOid,
        [Parameter(Mandatory)][System.Array]$RequiredWorkflows,
        [ValidateSet('pull_request', 'push')][string]$Event = 'pull_request',
        [long]$PullRequestNumber = 0
    )

    if (($Event -ceq 'pull_request' -and $PullRequestNumber -lt 1) -or
        ($Event -ceq 'push' -and $PullRequestNumber -ne 0)) {
        throw 'gh workflow event binding invalid'
    }

    $runsEndpoint = "repos/nonononull/inputcodex/actions/runs?head_sha=$HeadOid&event=$Event&per_page=100"
    $runOutput = @(& $GhExecutable api --paginate --slurp $runsEndpoint 2>$null)
    if ($LASTEXITCODE -ne 0) { throw 'gh workflow run list failed' }
    $runPages = (ConvertFrom-StrictPagedObjectOutput -Output $runOutput -Label 'gh workflow run list').Pages
    $allRuns = [System.Collections.Generic.List[object]]::new()
    foreach ($page in $runPages) {
        $runsProperty = $page.PSObject.Properties['workflow_runs']
        if ($null -eq $runsProperty -or $runsProperty.Value -isnot [System.Array]) {
            throw 'gh workflow runs schema invalid'
        }
        foreach ($run in $runsProperty.Value) {
            if ((Get-PropertyValue $run 'id') -isnot [long] -or
                (Get-PropertyValue $run 'name') -isnot [string] -or
                (Get-PropertyValue $run 'workflow_id') -isnot [long] -or
                (Get-PropertyValue $run 'path') -isnot [string] -or
                (Get-PropertyValue $run 'event') -isnot [string] -or
                [string](Get-PropertyValue $run 'head_sha') -cnotmatch '^[0-9a-f]{40}$' -or
                (Get-PropertyValue $run 'status') -isnot [string] -or
                ($null -ne (Get-PropertyValue $run 'conclusion') -and
                    (Get-PropertyValue $run 'conclusion') -isnot [string])) {
                throw 'gh workflow run schema invalid'
            }
            $allRuns.Add($run) | Out-Null
        }
    }

    $evidence = [System.Collections.Generic.List[object]]::new()
    foreach ($expectation in $RequiredWorkflows) {
        $workflowName = Get-PropertyValue $expectation 'name'
        $workflowId = Get-PropertyValue $expectation 'workflow_id'
        $workflowPath = Get-PropertyValue $expectation 'path'
        $expectedEvents = @(Get-PropertyValue $expectation 'events')
        if ($workflowName -isnot [string] -or
            $workflowId -isnot [long] -or
            $workflowPath -isnot [string] -or
            $Event -notin $expectedEvents) {
            throw 'required workflow expectation invalid'
        }
        $matches = @($allRuns | Where-Object {
            Test-GitHubWorkflowRunIdentity `
                -Run $_ `
                -Expectation $expectation `
                -HeadOid $HeadOid `
                -Event $Event `
                -PullRequestNumber $PullRequestNumber
        } | Sort-Object { Get-PropertyValue $_ 'id' } -Descending)
        if ($matches.Count -eq 0) {
            $evidence.Add([pscustomobject][ordered]@{
                name = $workflowName
                workflow_id = [long]$workflowId
                workflow_path = $workflowPath
                event = $Event
                pull_request_number = [long]$PullRequestNumber
                run_id = 0L
                head_oid = $HeadOid
                status = 'missing'
                conclusion = 'missing'
                artifact_count = -1L
                jobs = @()
            }) | Out-Null
            continue
        }

        $run = $matches[0]
        $runId = [long](Get-PropertyValue $run 'id')
        $jobsOutput = @(
            & $GhExecutable api --paginate --slurp "repos/nonononull/inputcodex/actions/runs/$runId/jobs?per_page=100" 2>$null
        )
        if ($LASTEXITCODE -ne 0) { throw 'gh workflow jobs failed' }
        $jobPages = (ConvertFrom-StrictPagedObjectOutput -Output $jobsOutput -Label 'gh workflow jobs').Pages
        $jobs = [System.Collections.Generic.List[object]]::new()
        foreach ($page in $jobPages) {
            $jobsProperty = $page.PSObject.Properties['jobs']
            if ($null -eq $jobsProperty -or $jobsProperty.Value -isnot [System.Array]) {
                throw 'gh workflow jobs schema invalid'
            }
            foreach ($job in $jobsProperty.Value) {
                if ((Get-PropertyValue $job 'name') -isnot [string] -or
                    (Get-PropertyValue $job 'status') -isnot [string] -or
                    ($null -ne (Get-PropertyValue $job 'conclusion') -and
                        (Get-PropertyValue $job 'conclusion') -isnot [string]) -or
                    [string](Get-PropertyValue $job 'head_sha') -cnotmatch '^[0-9a-f]{40}$') {
                    throw 'gh workflow job item schema invalid'
                }
                $jobs.Add([pscustomobject][ordered]@{
                    name = Get-PropertyValue $job 'name'
                    status = Get-PropertyValue $job 'status'
                    conclusion = Get-PropertyValue $job 'conclusion'
                    head_oid = Get-PropertyValue $job 'head_sha'
                }) | Out-Null
            }
        }

        $artifactOutput = @(
            & $GhExecutable api --paginate --slurp "repos/nonononull/inputcodex/actions/runs/$runId/artifacts?per_page=100" 2>$null
        )
        if ($LASTEXITCODE -ne 0) { throw 'gh workflow artifacts failed' }
        $artifactPages = (ConvertFrom-StrictPagedObjectOutput -Output $artifactOutput -Label 'gh workflow artifacts').Pages
        $artifactCount = 0L
        foreach ($page in $artifactPages) {
            $artifactsProperty = $page.PSObject.Properties['artifacts']
            if ($null -eq $artifactsProperty -or $artifactsProperty.Value -isnot [System.Array]) {
                throw 'gh workflow artifacts schema invalid'
            }
            $artifactCount += [long]$artifactsProperty.Value.Count
        }

        $evidence.Add([pscustomobject][ordered]@{
            name = $workflowName
            workflow_id = [long](Get-PropertyValue $run 'workflow_id')
            workflow_path = Get-PropertyValue $run 'path'
            event = Get-PropertyValue $run 'event'
            pull_request_number = [long]$PullRequestNumber
            run_id = $runId
            head_oid = Get-PropertyValue $run 'head_sha'
            status = Get-PropertyValue $run 'status'
            conclusion = Get-PropertyValue $run 'conclusion'
            artifact_count = [long]$artifactCount
            jobs = $jobs.ToArray()
        }) | Out-Null
    }
    return [pscustomobject]@{ Runs = $evidence.ToArray() }
}

function Get-GitHubPostMergeEvidence {
    param(
        [Parameter(Mandatory)][string]$GhExecutable,
        [Parameter(Mandatory)][string]$HeadOid,
        [Parameter(Mandatory)][string]$MergeCommitOid
    )

    $headOutput = @(& $GhExecutable api "repos/nonononull/inputcodex/commits/$HeadOid" 2>$null)
    if ($LASTEXITCODE -ne 0) { throw 'gh head commit failed' }
    $mergeOutput = @(& $GhExecutable api "repos/nonononull/inputcodex/commits/$MergeCommitOid" 2>$null)
    if ($LASTEXITCODE -ne 0) { throw 'gh merge commit failed' }
    $headCommit = (ConvertFrom-StrictJsonObjectOutput -Output $headOutput -Label 'gh head commit').Value
    $mergeCommit = (ConvertFrom-StrictJsonObjectOutput -Output $mergeOutput -Label 'gh merge commit').Value

    $headGitCommit = Get-PropertyValue $headCommit 'commit'
    $mergeGitCommit = Get-PropertyValue $mergeCommit 'commit'
    $headTree = Get-PropertyValue (Get-PropertyValue $headGitCommit 'tree') 'sha'
    $mergeTree = Get-PropertyValue (Get-PropertyValue $mergeGitCommit 'tree') 'sha'
    $parentsProperty = $mergeCommit.PSObject.Properties['parents']
    $verification = Get-PropertyValue $mergeGitCommit 'verification'
    if ((Get-PropertyValue $headCommit 'sha') -cne $HeadOid -or
        (Get-PropertyValue $mergeCommit 'sha') -cne $MergeCommitOid -or
        [string]$headTree -cnotmatch '^[0-9a-f]{40}$' -or
        [string]$mergeTree -cnotmatch '^[0-9a-f]{40}$' -or
        $null -eq $parentsProperty -or
        $parentsProperty.Value -isnot [System.Array] -or
        (Get-PropertyValue $verification 'verified') -isnot [bool]) {
        throw 'gh post-merge commit schema invalid'
    }
    return [pscustomobject][ordered]@{
        parent_count = [long]$parentsProperty.Value.Count
        merge_tree_oid = $mergeTree
        head_tree_oid = $headTree
        signature_valid = Get-PropertyValue $verification 'verified'
    }
}

function Test-ExactStringSet {
    param(
        [AllowNull()]$Actual,
        [Parameter(Mandatory)][string[]]$Expected
    )

    $actualValues = @($Actual)
    if ($actualValues.Count -ne $Expected.Count -or
        @($actualValues | Where-Object { $_ -isnot [string] }).Count -ne 0) {
        return $false
    }
    $actualSorted = @($actualValues | Sort-Object -Unique)
    $expectedSorted = @($Expected | Sort-Object -Unique)
    return ($actualSorted.Count -eq $Expected.Count -and
        [string]::Join([char]10, $actualSorted) -ceq [string]::Join([char]10, $expectedSorted))
}

function Get-FixedFileMutationTrancheProjection {
    param([AllowNull()]$Value)

    $invalid = [pscustomobject][ordered]@{ valid = $false }
    if ($Value -isnot [pscustomobject] -or
        -not (Test-ExactStringSet `
            -Actual @($Value.PSObject.Properties.Name) `
            -Expected @(
                'schema_version',
                'decision_id',
                'owner_decision_ref',
                'retry_resume_ref',
                'standing_authorization_ref',
                'repository_batches_max',
                'product_deliveries_max',
                'candidate_features',
                'expected_source_delta',
                'terminal'
            ))) {
        return $invalid
    }

    $candidateFeaturesProjection = Get-PropertyProjection $Value 'candidate_features'
    $candidateFeatures = $candidateFeaturesProjection.value
    $schemaVersion = $Value.PSObject.Properties['schema_version'].Value
    $decisionId = $Value.PSObject.Properties['decision_id'].Value
    $ownerDecisionRef = $Value.PSObject.Properties['owner_decision_ref'].Value
    $retryResumeRef = $Value.PSObject.Properties['retry_resume_ref'].Value
    $standingAuthorizationRef = $Value.PSObject.Properties['standing_authorization_ref'].Value
    $repositoryBatchesMaxProperty = $Value.PSObject.Properties['repository_batches_max']
    $repositoryBatchesMax = $repositoryBatchesMaxProperty.Value
    $productDeliveriesMaxProperty = $Value.PSObject.Properties['product_deliveries_max']
    $productDeliveriesMax = $productDeliveriesMaxProperty.Value
    $expectedSourceDelta = $Value.PSObject.Properties['expected_source_delta'].Value
    $terminal = $Value.PSObject.Properties['terminal'].Value
    if (-not $candidateFeaturesProjection.exists -or
        $candidateFeatures -isnot [System.Array] -or
        $candidateFeatures.Count -ne 1 -or
        $candidateFeatures[0] -isnot [string] -or
        $expectedSourceDelta -isnot [pscustomobject] -or
        -not (Test-ExactStringSet `
            -Actual @($expectedSourceDelta.PSObject.Properties.Name) `
            -Expected @('implemented', 'unassessed')) -or
        $terminal -isnot [pscustomobject] -or
        -not (Test-ExactStringSet `
            -Actual @($terminal.PSObject.Properties.Name) `
            -Expected @('owner_issue_ref', 'reopen_on', 'action', 'state', 'next_action'))) {
        return $invalid
    }

    $terminalOwnerIssueRef = $terminal.PSObject.Properties['owner_issue_ref'].Value
    $terminalAction = $terminal.PSObject.Properties['action'].Value
    $terminalState = $terminal.PSObject.Properties['state'].Value
    $terminalNextAction = $terminal.PSObject.Properties['next_action'].Value
    $reopenOnProjection = Get-PropertyProjection $terminal 'reopen_on'
    $reopenOn = $reopenOnProjection.value
    $stringFieldsValid =
        $schemaVersion -is [string] -and
        $schemaVersion -ceq 'inputcodex.fixed-file-mutation-tranche.v1' -and
        $decisionId -is [string] -and
        $decisionId -ceq 'gate5-fixed-file-mutation-tranche-v1' -and
        $ownerDecisionRef -is [string] -and
        $ownerDecisionRef -ceq 'https://github.com/nonononull/inputcodex/issues/140#issuecomment-5159072214' -and
        $retryResumeRef -is [string] -and
        $retryResumeRef -ceq 'https://github.com/nonononull/inputcodex/issues/140#issuecomment-5159471091' -and
        $standingAuthorizationRef -is [string] -and
        $standingAuthorizationRef -ceq 'https://github.com/nonononull/inputcodex/issues/111' -and
        $terminalOwnerIssueRef -is [string] -and
        $terminalOwnerIssueRef -ceq 'https://github.com/nonononull/inputcodex/issues/140' -and
        $terminalAction -is [string] -and
        $terminalAction -ceq 'reopen-owner-decision-issue' -and
        $terminalState -is [string] -and
        $terminalState -ceq 'blocked-candidate-exhausted' -and
        $terminalNextAction -is [string] -and
        $terminalNextAction -ceq 'await-owner-decision'
    $expectedImplementedProperty = $expectedSourceDelta.PSObject.Properties['implemented']
    $expectedImplemented = $expectedImplementedProperty.Value
    $expectedUnassessedProperty = $expectedSourceDelta.PSObject.Properties['unassessed']
    $expectedUnassessed = $expectedUnassessedProperty.Value
    if (-not $reopenOnProjection.exists -or
        $reopenOn -isnot [System.Array] -or
        $reopenOn.Count -ne 2 -or
        $reopenOn[0] -isnot [string] -or
        $reopenOn[1] -isnot [string] -or
        -not $stringFieldsValid -or
        $repositoryBatchesMax -isnot [long] -or
        $repositoryBatchesMax -ne 2L -or
        $productDeliveriesMax -isnot [long] -or
        $productDeliveriesMax -ne 1L -or
        $candidateFeatures[0] -cne 'feature.foundation-platform.watcher-preference-mutation' -or
        $expectedImplemented -isnot [long] -or
        $expectedImplemented -ne 2L -or
        $expectedUnassessed -isnot [long] -or
        $expectedUnassessed -ne -2L -or
        $reopenOn[0] -cne 'completed' -or
        $reopenOn[1] -cne 'hard-stop') {
        return $invalid
    }

    return [pscustomobject][ordered]@{
        valid = $true
        schema_version = $schemaVersion
        decision_id = $decisionId
        owner_decision_ref = $ownerDecisionRef
        retry_resume_ref = $retryResumeRef
        standing_authorization_ref = $standingAuthorizationRef
        repository_batches_max = $repositoryBatchesMax
        product_deliveries_max = $productDeliveriesMax
        candidate = $candidateFeatures[0]
        expected_source_delta = [pscustomobject][ordered]@{
            implemented = $expectedImplemented
            unassessed = $expectedUnassessed
        }
        terminal = [pscustomobject][ordered]@{
            owner_issue_ref = $terminalOwnerIssueRef
            reopen_on = @($reopenOn)
            action = $terminalAction
            state = $terminalState
            next_action = $terminalNextAction
        }
    }
}

function Test-AutonomousWorkflowRunEvidence {
    param(
        [Parameter(Mandatory)][AllowEmptyCollection()][System.Array]$WorkflowRuns,
        [Parameter(Mandatory)]$WorkflowExpectation,
        [Parameter(Mandatory)][string]$HeadOid,
        [Parameter(Mandatory)][ValidateSet('pull_request', 'push')][string]$ExpectedEvent,
        [Parameter(Mandatory)][long]$PullRequestNumber
    )

    $workflowName = Get-PropertyValue $WorkflowExpectation 'name'
    $workflowId = Get-PropertyValue $WorkflowExpectation 'workflow_id'
    $workflowPath = Get-PropertyValue $WorkflowExpectation 'path'
    $expectedJobs = @(Get-PropertyValue $WorkflowExpectation 'jobs')
    $matches = @($WorkflowRuns | Where-Object { (Get-PropertyValue $_ 'name') -ceq $WorkflowName })
    if ($matches.Count -ne 1) {
        return $false
    }
    $run = $matches[0]
    $jobsProperty = $run.PSObject.Properties['jobs']
    if ($null -eq $jobsProperty -or $jobsProperty.Value -isnot [System.Array]) {
        return $false
    }
    $jobs = @($jobsProperty.Value)
    if (-not (Test-ExactStringSet -Actual @($jobs | ForEach-Object { Get-PropertyValue $_ 'name' }) -Expected $ExpectedJobs)) {
        return $false
    }
    foreach ($job in $jobs) {
        if ((Get-PropertyValue $job 'status') -cne 'completed' -or
            (Get-PropertyValue $job 'conclusion') -cne 'success' -or
            (Get-PropertyValue $job 'head_oid') -cne $HeadOid) {
            return $false
        }
    }
    return ((Get-PropertyValue $run 'run_id') -is [long] -and
        (Get-PropertyValue $run 'run_id') -gt 0 -and
        (Get-PropertyValue $run 'workflow_id') -is [long] -and
        (Get-PropertyValue $run 'workflow_id') -eq $workflowId -and
        (Get-PropertyValue $run 'workflow_path') -ceq $workflowPath -and
        (Get-PropertyValue $run 'event') -ceq $ExpectedEvent -and
        (Get-PropertyValue $run 'pull_request_number') -is [long] -and
        (Get-PropertyValue $run 'pull_request_number') -eq $PullRequestNumber -and
        (Get-PropertyValue $run 'head_oid') -ceq $HeadOid -and
        (Get-PropertyValue $run 'status') -ceq 'completed' -and
        (Get-PropertyValue $run 'conclusion') -ceq 'success' -and
        (Get-PropertyValue $run 'artifact_count') -is [long] -and
        (Get-PropertyValue $run 'artifact_count') -eq 0)
}

function Get-AutonomousPostMergeGateEvaluation {
    param(
        [Parameter(Mandatory)]$MergedPullRequest,
        [Parameter(Mandatory)]$Issue,
        [Parameter(Mandatory)]$Snapshot,
        [Parameter(Mandatory)][string]$PolicySha256,
        [Parameter(Mandatory)][string]$TaskKind,
        [Parameter(Mandatory)][System.Array]$RequiredWorkflows
    )

    $pending = [System.Collections.Generic.List[string]]::new()
    function Add-PostMergePendingCode {
        param([Parameter(Mandatory)][string]$Code)
        if (-not $pending.Contains($Code)) {
            $pending.Add($Code) | Out-Null
        }
    }

    $mergeCommitOid = Get-PropertyValue $MergedPullRequest 'merge_commit_oid'
    $evidence = Get-PropertyValue $MergedPullRequest 'evidence'
    $planningEvidence = Get-PropertyValue $Issue 'planning_evidence'
    $evidenceTaskKind = (Get-PropertyProjection $evidence 'task_kind').value
    $planningTaskKind = (Get-PropertyProjection $planningEvidence 'task_kind').value
    if ([string]$mergeCommitOid -cnotmatch '^[0-9a-f]{40}$' -or
        (Get-PropertyValue $Snapshot 'observed_remote_main') -cne $mergeCommitOid -or
        $null -eq $evidence -or
        (Get-PropertyValue $evidence 'valid') -ne $true -or
        (Get-PropertyValue $evidence 'tracking_issue_ref') -cne (Get-PropertyValue $Issue 'url') -or
        (Get-PropertyValue $evidence 'standing_authorization_ref') -cne 'https://github.com/nonononull/inputcodex/issues/111' -or
        (Get-PropertyValue $evidence 'policy_sha256') -cne $PolicySha256 -or
        (Get-PropertyValue $evidence 'final_head') -cne (Get-PropertyValue $MergedPullRequest 'head_oid') -or
        $null -eq $planningEvidence -or
        (Get-PropertyValue $planningEvidence 'valid') -ne $true -or
        (Get-PropertyValue $planningEvidence 'scope_count') -ne (Get-PropertyValue $Snapshot 'actual_scope_count') -or
        (Get-PropertyValue $planningEvidence 'scope_hash') -cne (Get-PropertyValue $Snapshot 'actual_scope_hash') -or
        (Get-PropertyValue $evidence 'scope_count') -ne (Get-PropertyValue $Snapshot 'actual_scope_count') -or
        (Get-PropertyValue $evidence 'scope_hash') -cne (Get-PropertyValue $Snapshot 'actual_scope_hash') -or
        $evidenceTaskKind -isnot [string] -or
        $evidenceTaskKind -cne $TaskKind -or
        $planningTaskKind -isnot [string] -or
        $planningTaskKind -cne $TaskKind) {
        Add-PostMergePendingCode 'POST_MERGE_EVIDENCE'
    }
    if ((Get-PropertyValue $Snapshot 'observed_origin_main') -cne $mergeCommitOid) {
        Add-PostMergePendingCode 'POST_MERGE_ORIGIN_MAIN'
    }

    $review = Get-PropertyValue $MergedPullRequest 'review_attestation'
    if ($null -eq $review -or
        (Get-PropertyValue $review 'valid') -ne $true -or
        (Get-PropertyValue $review 'final_head') -cne (Get-PropertyValue $MergedPullRequest 'head_oid') -or
        (Get-PropertyValue $review 'status') -cne 'passed' -or
        (Get-PropertyValue $review 'ref') -cne (Get-PropertyValue $evidence 'independent_review_ref')) {
        Add-PostMergePendingCode 'POST_MERGE_REVIEW'
    }

    $postMerge = Get-PropertyValue $MergedPullRequest 'post_merge'
    if ($null -eq $postMerge -or
        (Get-PropertyValue $postMerge 'parent_count') -isnot [long] -or
        (Get-PropertyValue $postMerge 'parent_count') -ne 1 -or
        [string](Get-PropertyValue $postMerge 'merge_tree_oid') -cnotmatch '^[0-9a-f]{40}$' -or
        (Get-PropertyValue $postMerge 'merge_tree_oid') -cne (Get-PropertyValue $postMerge 'head_tree_oid') -or
        (Get-PropertyValue $postMerge 'signature_valid') -isnot [bool] -or
        (Get-PropertyValue $postMerge 'signature_valid') -ne $true) {
        Add-PostMergePendingCode 'POST_MERGE_STRUCTURE'
    }

    $repositorySettings = Get-PropertyValue $Snapshot 'repository_settings'
    if ((Get-PropertyValue $repositorySettings 'allow_auto_merge') -ne $false -or
        (Get-PropertyValue $repositorySettings 'allow_squash_merge') -ne $true -or
        (Get-PropertyValue $repositorySettings 'allow_merge_commit') -ne $false -or
        (Get-PropertyValue $repositorySettings 'allow_rebase_merge') -ne $false -or
        (Get-PropertyValue $repositorySettings 'default_branch') -cne 'main') {
        Add-PostMergePendingCode 'POST_MERGE_REPOSITORY_SETTINGS'
    }

    $workflowProperty = $MergedPullRequest.PSObject.Properties['workflow_runs']
    $workflowRuns = @()
    if ($null -ne $workflowProperty -and $workflowProperty.Value -is [System.Array]) {
        $workflowRuns = @($workflowProperty.Value)
    }
    foreach ($workflowExpectation in $RequiredWorkflows) {
        $workflowName = Get-PropertyValue $workflowExpectation 'name'
        $code = if ($workflowName -ceq 'CI') {
            'POST_MERGE_WORKFLOW_CI'
        } else {
            'POST_MERGE_WORKFLOW_PERFORMANCE_BASELINE'
        }
        if (-not (Test-AutonomousWorkflowRunEvidence `
            -WorkflowRuns $workflowRuns `
            -WorkflowExpectation $workflowExpectation `
            -HeadOid $mergeCommitOid `
            -ExpectedEvent push `
            -PullRequestNumber 0)) {
            Add-PostMergePendingCode $code
        }
    }
    if ($workflowRuns.Count -ne $RequiredWorkflows.Count) {
        Add-PostMergePendingCode 'POST_MERGE_WORKFLOW_SET'
    }
    return [pscustomobject]@{ Pending = $pending.ToArray() }
}

function Get-AutonomousMergeGateEvaluation {
    param(
        [Parameter(Mandatory)]$PullRequest,
        [Parameter(Mandatory)]$Issue,
        [Parameter(Mandatory)]$Snapshot,
        [Parameter(Mandatory)][string]$PolicySha256,
        [Parameter(Mandatory)][string]$TaskKind,
        [Parameter(Mandatory)][System.Array]$RequiredWorkflows
    )

    $pending = [System.Collections.Generic.List[string]]::new()
    function Add-PendingCode {
        param([Parameter(Mandatory)][string]$Code)
        if (-not $pending.Contains($Code)) {
            $pending.Add($Code) | Out-Null
        }
    }

    if ((Get-PropertyValue $PullRequest 'is_draft') -isnot [bool] -or
        (Get-PropertyValue $PullRequest 'is_draft') -ne $false) {
        Add-PendingCode 'PR_DRAFT'
    }
    if ((Get-PropertyValue $PullRequest 'merge_state') -cne 'CLEAN') {
        Add-PendingCode 'PR_MERGE_STATE'
    }
    $reviewThreadCount = Get-PropertyValue $PullRequest 'review_thread_count'
    if ($reviewThreadCount -isnot [long] -or $reviewThreadCount -ne 0) {
        Add-PendingCode 'REVIEW_THREADS'
    }

    $repositorySettings = Get-PropertyValue $Snapshot 'repository_settings'
    if ($null -eq $repositorySettings -or
        (Get-PropertyValue $repositorySettings 'allow_auto_merge') -isnot [bool] -or
        (Get-PropertyValue $repositorySettings 'allow_auto_merge') -ne $false -or
        (Get-PropertyValue $repositorySettings 'allow_squash_merge') -isnot [bool] -or
        (Get-PropertyValue $repositorySettings 'allow_squash_merge') -ne $true -or
        (Get-PropertyValue $repositorySettings 'allow_merge_commit') -isnot [bool] -or
        (Get-PropertyValue $repositorySettings 'allow_merge_commit') -ne $false -or
        (Get-PropertyValue $repositorySettings 'allow_rebase_merge') -isnot [bool] -or
        (Get-PropertyValue $repositorySettings 'allow_rebase_merge') -ne $false -or
        (Get-PropertyValue $repositorySettings 'default_branch') -cne 'main') {
        Add-PendingCode 'REPOSITORY_MERGE_SETTINGS'
    }

    $evidence = Get-PropertyValue $PullRequest 'evidence'
    if ($null -eq $evidence -or
        (Get-PropertyValue $evidence 'valid') -isnot [bool] -or
        (Get-PropertyValue $evidence 'valid') -ne $true -or
        (Get-PropertyValue $evidence 'tracking_issue_ref') -cne (Get-PropertyValue $Issue 'url') -or
        (Get-PropertyValue $evidence 'standing_authorization_ref') -cne 'https://github.com/nonononull/inputcodex/issues/111') {
        Add-PendingCode 'EVIDENCE_AUTHORIZATION'
    }
    if ($null -eq $evidence -or
        (Get-PropertyValue $evidence 'final_head') -cne (Get-PropertyValue $PullRequest 'head_oid') -or
        (Get-PropertyValue $evidence 'final_head') -cne (Get-PropertyValue $Snapshot 'worktree_head')) {
        Add-PendingCode 'EVIDENCE_HEAD'
    }
    if ($null -eq $evidence -or
        (Get-PropertyValue $evidence 'policy_sha256') -cne $PolicySha256) {
        Add-PendingCode 'EVIDENCE_POLICY'
    }
    $actualScopeCount = Get-PropertyValue $Snapshot 'actual_scope_count'
    $actualScopeHash = Get-PropertyValue $Snapshot 'actual_scope_hash'
    $planningEvidence = Get-PropertyValue $Issue 'planning_evidence'
    if ($null -eq $evidence -or
        $null -eq $planningEvidence -or
        (Get-PropertyValue $planningEvidence 'valid') -isnot [bool] -or
        (Get-PropertyValue $planningEvidence 'valid') -ne $true -or
        (Get-PropertyValue $planningEvidence 'scope_count') -isnot [long] -or
        (Get-PropertyValue $planningEvidence 'scope_count') -ne $actualScopeCount -or
        (Get-PropertyValue $planningEvidence 'scope_hash') -cne $actualScopeHash -or
        [string](Get-PropertyValue $planningEvidence 'ref') -cnotmatch
            '^https://github\.com/nonononull/inputcodex/issues/[0-9]+#issuecomment-[0-9]+$' -or
        $actualScopeCount -isnot [long] -or
        (Get-PropertyValue $evidence 'scope_count') -isnot [long] -or
        (Get-PropertyValue $evidence 'scope_count') -ne $actualScopeCount -or
        (Get-PropertyValue $evidence 'scope_hash') -cne $actualScopeHash) {
        Add-PendingCode 'EVIDENCE_SCOPE'
    }
    $evidenceTaskKind = (Get-PropertyProjection $evidence 'task_kind').value
    $planningTaskKind = (Get-PropertyProjection $planningEvidence 'task_kind').value
    if ($evidenceTaskKind -isnot [string] -or
        $evidenceTaskKind -cne $TaskKind -or
        $planningTaskKind -isnot [string] -or
        $planningTaskKind -cne $TaskKind) {
        Add-PendingCode 'EVIDENCE_TASK_KIND'
    }
    $reviewAttestation = Get-PropertyValue $PullRequest 'review_attestation'
    if ($null -eq $evidence -or
        $null -eq $reviewAttestation -or
        (Get-PropertyValue $reviewAttestation 'valid') -isnot [bool] -or
        (Get-PropertyValue $reviewAttestation 'valid') -ne $true -or
        (Get-PropertyValue $reviewAttestation 'final_head') -cne (Get-PropertyValue $PullRequest 'head_oid') -or
        (Get-PropertyValue $reviewAttestation 'status') -cne 'passed' -or
        (Get-PropertyValue $reviewAttestation 'ref') -cne (Get-PropertyValue $evidence 'independent_review_ref') -or
        (Get-PropertyValue $evidence 'independent_review_status') -cne 'passed' -or
        [string](Get-PropertyValue $evidence 'independent_review_ref') -cnotmatch
            '^https://github\.com/nonononull/inputcodex/(pull|issues)/[0-9]+#issuecomment-[0-9]+$') {
        Add-PendingCode 'INDEPENDENT_REVIEW'
    }

    $workflowProperty = $PullRequest.PSObject.Properties['workflow_runs']
    $workflowRuns = @()
    if ($null -ne $workflowProperty -and $workflowProperty.Value -is [System.Array]) {
        $workflowRuns = @($workflowProperty.Value)
    }
    foreach ($workflowExpectation in $RequiredWorkflows) {
        $workflowName = Get-PropertyValue $workflowExpectation 'name'
        $code = if ($workflowName -ceq 'CI') { 'WORKFLOW_CI' } else { 'WORKFLOW_PERFORMANCE_BASELINE' }
        if (-not (Test-AutonomousWorkflowRunEvidence `
            -WorkflowRuns $workflowRuns `
            -WorkflowExpectation $workflowExpectation `
            -HeadOid ([string](Get-PropertyValue $PullRequest 'head_oid')) `
            -ExpectedEvent pull_request `
            -PullRequestNumber ([long](Get-PropertyValue $PullRequest 'number')))) {
            Add-PendingCode $code
        }
    }
    if ($workflowRuns.Count -ne $RequiredWorkflows.Count) {
        Add-PendingCode 'WORKFLOW_SET'
    }

    return [pscustomobject]@{ Pending = $pending.ToArray() }
}

function Get-AutonomousScopeProjection {
    param([AllowEmptyCollection()][string[]]$Paths)

    $pathSet = [Collections.Generic.SortedSet[string]]::new([StringComparer]::Ordinal)
    foreach ($path in $Paths) {
        if (-not [string]::IsNullOrWhiteSpace($path)) {
            [void]$pathSet.Add($path)
        }
    }
    $normalizedPaths = [string[]]@($pathSet)
    $payload = [string]::Join([char]10, [string[]]$normalizedPaths) + [char]10
    $hash = [Convert]::ToHexString(
        [Security.Cryptography.SHA256]::HashData(
            [Text.UTF8Encoding]::new($false).GetBytes($payload)
        )
    ).ToLowerInvariant()

    return [pscustomobject][ordered]@{
        paths = [string[]]$normalizedPaths
        count = [long]$normalizedPaths.Count
        scope_hash = "sha256:$hash"
    }
}

function Resolve-AutonomousTaskLink {
    param(
        [AllowEmptyCollection()][object[]]$MarkedIssues,
        [AllowEmptyCollection()][object[]]$MergedPullRequests,
        [Parameter(Mandatory)][string]$ObservedRemoteMain,
        [Parameter(Mandatory)][string]$WorktreeHead
    )

    $openIssues = @($MarkedIssues | Where-Object {
        (Get-PropertyValue $_ 'github_state') -ceq 'open'
    })
    $trustedOpenIssues = @($openIssues | Where-Object {
        (Get-PropertyValue $_ 'author_login') -ceq 'nonononull'
    })
    $trustedMergedPullRequests = @($MergedPullRequests | Where-Object {
        (Get-PropertyValue $_ 'author_login') -ceq 'nonononull' -and
        (Get-PropertyValue $_ 'head_owner_login') -ceq 'nonononull'
    })

    if ($trustedOpenIssues.Count -eq 1) {
        $linkedMergedPullRequests = @($trustedMergedPullRequests | Where-Object {
            $evidence = Get-PropertyValue $_ 'evidence'
            $null -ne $evidence -and
            (Get-PropertyValue $evidence 'valid') -eq $true -and
            (Get-PropertyValue $evidence 'tracking_issue_ref') -ceq
                (Get-PropertyValue $trustedOpenIssues[0] 'url')
        })
        return [pscustomobject][ordered]@{
            active_issues = [object[]]$openIssues
            linked_merged_prs = [object[]]$linkedMergedPullRequests
        }
    }

    if ($trustedOpenIssues.Count -eq 0) {
        $trustedClosedIssues = @($MarkedIssues | Where-Object {
            (Get-PropertyValue $_ 'github_state') -ceq 'closed' -and
            (Get-PropertyValue $_ 'author_login') -ceq 'nonononull'
        })
        $recoveryCandidates = @(
            foreach ($issue in $trustedClosedIssues) {
                foreach ($pullRequest in $trustedMergedPullRequests) {
                    $evidence = Get-PropertyValue $pullRequest 'evidence'
                    if ($null -ne $evidence -and
                        (Get-PropertyValue $evidence 'valid') -eq $true -and
                        (Get-PropertyValue $evidence 'tracking_issue_ref') -ceq (Get-PropertyValue $issue 'url') -and
                        (Get-PropertyValue $pullRequest 'merge_commit_oid') -ceq $ObservedRemoteMain -and
                        (Get-PropertyValue $pullRequest 'head_oid') -ceq $WorktreeHead) {
                        [pscustomobject][ordered]@{
                            issue = $issue
                            pull_request = $pullRequest
                        }
                    }
                }
            }
        )
        if ($recoveryCandidates.Count -eq 1) {
            return [pscustomobject][ordered]@{
                active_issues = [object[]]@($openIssues + $recoveryCandidates[0].issue)
                linked_merged_prs = [object[]]@($recoveryCandidates[0].pull_request)
            }
        }
    }

    return [pscustomobject][ordered]@{
        active_issues = [object[]]$openIssues
        linked_merged_prs = [object[]]@()
    }
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
    param(
        [Parameter(Mandatory)][string]$UpstreamSyncTaskMarker,
        [Parameter(Mandatory)][string]$CandidateExhaustedTaskMarker,
        [Parameter(Mandatory)][System.Array]$RequiredWorkflows
    )

    $originMain = ((Invoke-GitRead @('rev-parse', 'origin/main')) -join '').Trim()
    $worktreeHead = ((Invoke-GitRead @('rev-parse', 'HEAD')) -join '').Trim()
    $branch = ((Invoke-GitRead @('branch', '--show-current')) -join '').Trim()
    $expectedBase = ((Invoke-GitRead @('merge-base', 'HEAD', 'origin/main')) -join '').Trim()
    $worktreeClean = @((Invoke-GitRead @('status', '--porcelain=v1'))).Count -eq 0
    $actualScope = Get-AutonomousScopeProjection -Paths @(
        Invoke-GitRead @('diff', '--no-renames', '--name-only', 'origin/main...HEAD')
    )

    $sourceLockPath = Join-Path $script:Root 'upstream/source-lock.json'
    $releaseAudit = 'missing'
    if (Test-Path -LiteralPath $sourceLockPath -PathType Leaf) {
        try {
            $sourceLock = [System.IO.File]::ReadAllText($sourceLockPath, [Text.UTF8Encoding]::new($false)) |
                ConvertFrom-Json -Depth 100 -NoEnumerate
            $releaseAudit = Get-ReleaseAuditStatusProjection -SourceLock $sourceLock
        } catch {
            $releaseAudit = 'invalid'
        }
    }

    $githubAvailable = $false
    $remoteMain = $originMain
    $repositorySettings = [pscustomobject][ordered]@{
        allow_auto_merge = $false
        allow_squash_merge = $true
        allow_merge_commit = $false
        allow_rebase_merge = $false
        default_branch = 'main'
    }
    $activeIssues = @()
    $activePrs = @()
    $mergedPrs = @()
    $ghCommand = Get-Command gh -ErrorAction SilentlyContinue
    if ($null -ne $ghCommand) {
        try {
            $repositoryOutput = @(& $ghCommand.Source api repos/nonononull/inputcodex 2>$null)
            if ($LASTEXITCODE -ne 0) { throw 'gh repository settings failed' }
            $repository = (ConvertFrom-StrictJsonObjectOutput -Output $repositoryOutput -Label 'gh repository settings').Value
            foreach ($propertyName in @(
                'allow_auto_merge',
                'allow_squash_merge',
                'allow_merge_commit',
                'allow_rebase_merge'
            )) {
                if ((Get-PropertyValue $repository $propertyName) -isnot [bool]) {
                    throw 'gh repository settings schema invalid'
                }
            }
            if ((Get-PropertyValue $repository 'default_branch') -isnot [string]) {
                throw 'gh repository default branch schema invalid'
            }
            $repositorySettings = [pscustomobject][ordered]@{
                allow_auto_merge = Get-PropertyValue $repository 'allow_auto_merge'
                allow_squash_merge = Get-PropertyValue $repository 'allow_squash_merge'
                allow_merge_commit = Get-PropertyValue $repository 'allow_merge_commit'
                allow_rebase_merge = Get-PropertyValue $repository 'allow_rebase_merge'
                default_branch = Get-PropertyValue $repository 'default_branch'
            }

            $remoteMainOutput = @(& $ghCommand.Source api repos/nonononull/inputcodex/git/ref/heads/main --jq '.object.sha' 2>$null)
            if ($LASTEXITCODE -ne 0) { throw 'gh main ref failed' }
            $remoteMainCandidate = ($remoteMainOutput -join '').Trim()
            if ($remoteMainCandidate -cnotmatch '^[0-9a-f]{40}$') { throw 'gh main ref invalid' }
            $remoteMain = $remoteMainCandidate

            $issueOutput = @(
                & $ghCommand.Source api --paginate --slurp 'repos/nonononull/inputcodex/issues?state=all&per_page=100' 2>$null
            )
            if ($LASTEXITCODE -ne 0) { throw 'gh issue list failed' }
            $issues = (ConvertFrom-StrictPagedArrayOutput -Output $issueOutput -Label 'gh issue list').Items
            foreach ($issue in $issues) {
                $issueUser = Get-PropertyValue $issue 'user'
                $issueBody = Get-PropertyValue $issue 'body'
                $issueLabelsProjection = Get-PropertyProjection $issue 'labels'
                $issueLabels = $issueLabelsProjection.value
                if ((Get-PropertyValue $issue 'number') -isnot [long] -or
                    (Get-PropertyValue $issue 'number') -lt 1 -or
                    (Get-PropertyValue $issue 'html_url') -isnot [string] -or
                    (Get-PropertyValue $issueUser 'login') -isnot [string] -or
                    (Get-PropertyValue $issue 'state') -isnot [string] -or
                    ($null -ne $issueBody -and $issueBody -isnot [string]) -or
                    -not $issueLabelsProjection.exists -or
                    $issueLabels -isnot [System.Array] -or
                    @($issueLabels | Where-Object {
                        $_ -isnot [pscustomobject] -or
                        (Get-PropertyValue $_ 'name') -isnot [string]
                    }).Count -ne 0) {
                    throw 'gh issue item schema invalid'
                }
            }
            $markedIssues = @($issues |
                Where-Object {
                    $null -eq $_.PSObject.Properties['pull_request'] -and
                    (Test-AutonomousIssueTaskMarkerPresence -Body (Get-PropertyValue $_ 'body'))
                } |
                ForEach-Object {
                    $issueLabels = (Get-PropertyProjection $_ 'labels').value
                    [pscustomobject][ordered]@{
                        number = Get-PropertyValue $_ 'number'
                        url = Get-PropertyValue $_ 'html_url'
                        author_login = Get-PropertyValue (Get-PropertyValue $_ 'user') 'login'
                        github_state = Get-PropertyValue $_ 'state'
                        task_kind = Get-AutonomousIssueTaskKind `
                            -Body (Get-PropertyValue $_ 'body') `
                            -UpstreamSyncTaskMarker $UpstreamSyncTaskMarker `
                            -CandidateExhaustedTaskMarker $CandidateExhaustedTaskMarker
                        labels = [object[]]@($issueLabels | ForEach-Object {
                            Get-PropertyValue $_ 'name'
                        })
                        planning_evidence = [pscustomobject][ordered]@{ valid = $false }
                    }
                })

            $prOutput = @(
                & $ghCommand.Source api --paginate --slurp 'repos/nonononull/inputcodex/pulls?state=all&per_page=100' 2>$null
            )
            if ($LASTEXITCODE -ne 0) { throw 'gh pr list failed' }
            $prs = (ConvertFrom-StrictPagedArrayOutput -Output $prOutput -Label 'gh pr list').Items
            foreach ($pr in $prs) {
                $prUser = Get-PropertyValue $pr 'user'
                $prBase = Get-PropertyValue $pr 'base'
                $prHead = Get-PropertyValue $pr 'head'
                $prBody = Get-PropertyValue $pr 'body'
                if ((Get-PropertyValue $pr 'number') -isnot [long] -or
                    (Get-PropertyValue $pr 'number') -lt 1 -or
                    (Get-PropertyValue $pr 'html_url') -isnot [string] -or
                    (Get-PropertyValue $prUser 'login') -isnot [string] -or
                    (Get-PropertyValue $prBase 'ref') -isnot [string] -or
                    [string](Get-PropertyValue $prHead 'sha') -cnotmatch '^[0-9a-f]{40}$' -or
                    (Get-PropertyValue $pr 'draft') -isnot [bool] -or
                    (Get-PropertyValue $pr 'state') -isnot [string] -or
                    ($null -ne (Get-PropertyValue $pr 'merged_at') -and
                        (Get-PropertyValue $pr 'merged_at') -isnot [string] -and
                        (Get-PropertyValue $pr 'merged_at') -isnot [datetime]) -or
                    ($null -ne (Get-PropertyValue $pr 'merge_commit_sha') -and
                        [string](Get-PropertyValue $pr 'merge_commit_sha') -cnotmatch '^[0-9a-f]{40}$') -or
                    ($null -ne $prBody -and $prBody -isnot [string])) {
                    throw 'gh pr item schema invalid'
                }
            }
            $markedPrs = @($prs |
                Where-Object {
                    [string](Get-PropertyValue $_ 'body') -match
                        'inputcodex:autonomous-refactor-pr:v1'
                } |
                ForEach-Object {
                    $head = Get-PropertyValue $_ 'head'
                    $headRepository = Get-PropertyValue $head 'repo'
                    $bodyEvidence = Get-AutonomousPrBodyEvidence -Body (Get-PropertyValue $_ 'body')
                    $mergedAt = Get-PropertyValue $_ 'merged_at'
                    if ($mergedAt -is [datetime]) {
                        $mergedAt = $mergedAt.ToUniversalTime().ToString('o')
                    }
                    [pscustomobject][ordered]@{
                        number = Get-PropertyValue $_ 'number'
                        url = Get-PropertyValue $_ 'html_url'
                        base_ref = Get-PropertyValue (Get-PropertyValue $_ 'base') 'ref'
                        head_oid = Get-PropertyValue $head 'sha'
                        author_login = Get-PropertyValue (Get-PropertyValue $_ 'user') 'login'
                        head_owner_login = Get-PropertyValue (Get-PropertyValue $headRepository 'owner') 'login'
                        is_draft = Get-PropertyValue $_ 'draft'
                        merge_state = 'UNKNOWN'
                        review_thread_count = -1L
                        state = Get-PropertyValue $_ 'state'
                        merged_at = $mergedAt
                        merge_commit_oid = Get-PropertyValue $_ 'merge_commit_sha'
                        evidence = $bodyEvidence
                        review_attestation = [pscustomobject][ordered]@{ valid = $false }
                        post_merge = [pscustomobject][ordered]@{}
                        workflow_runs = @()
                    }
                })

            $activePrs = @($markedPrs | Where-Object {
                (Get-PropertyValue $_ 'state') -ceq 'open'
            })
            $mergedPrs = @($markedPrs | Where-Object {
                (Get-PropertyValue $_ 'state') -ceq 'closed' -and
                (Get-PropertyValue $_ 'merged_at') -is [string] -and
                [string](Get-PropertyValue $_ 'merge_commit_oid') -cmatch '^[0-9a-f]{40}$'
            })

            $trustedPrs = @($activePrs | Where-Object {
                (Get-PropertyValue $_ 'author_login') -ceq 'nonononull' -and
                (Get-PropertyValue $_ 'head_owner_login') -ceq 'nonononull'
            })
            if ($trustedPrs.Count -eq 1) {
                $trustedPr = $trustedPrs[0]
                $reviewState = Get-GitHubPullRequestReviewState `
                    -GhExecutable $ghCommand.Source `
                    -Number ([long](Get-PropertyValue $trustedPr 'number'))
                if ((Get-PropertyValue $reviewState 'head_oid') -cne (Get-PropertyValue $trustedPr 'head_oid') -or
                    (Get-PropertyValue $reviewState 'base_ref') -cne (Get-PropertyValue $trustedPr 'base_ref')) {
                    throw 'gh pr changed during collection'
                }
                $trustedPr.is_draft = Get-PropertyValue $reviewState 'is_draft'
                $trustedPr.merge_state = Get-PropertyValue $reviewState 'merge_state'
                $trustedPr.review_thread_count = Get-PropertyValue $reviewState 'review_thread_count'
                $trustedPr.review_attestation = Get-GitHubReviewAttestation `
                    -GhExecutable $ghCommand.Source `
                    -PullRequestNumber ([long](Get-PropertyValue $trustedPr 'number'))
                $workflowEvidence = Get-GitHubWorkflowEvidence `
                    -GhExecutable $ghCommand.Source `
                    -HeadOid ([string](Get-PropertyValue $trustedPr 'head_oid')) `
                    -RequiredWorkflows $RequiredWorkflows `
                    -PullRequestNumber ([long](Get-PropertyValue $trustedPr 'number'))
                $trustedPr.workflow_runs = $workflowEvidence.Runs
            }

            $taskLink = Resolve-AutonomousTaskLink -MarkedIssues $markedIssues `
                -MergedPullRequests $mergedPrs -ObservedRemoteMain $remoteMain -WorktreeHead $worktreeHead
            $activeIssues = @($taskLink.active_issues)
            $linkedMergedPrs = @($taskLink.linked_merged_prs)
            $trustedIssuesForEvidence = @($activeIssues | Where-Object {
                (Get-PropertyValue $_ 'author_login') -ceq 'nonononull'
            })
            if ($trustedIssuesForEvidence.Count -eq 1) {
                $trustedIssue = $trustedIssuesForEvidence[0]
                $trustedIssue.planning_evidence = Get-GitHubPlanningEvidence `
                    -GhExecutable $ghCommand.Source `
                    -IssueNumber ([long](Get-PropertyValue $trustedIssue 'number'))
            }
            if ($linkedMergedPrs.Count -eq 1) {
                $mergedPr = $linkedMergedPrs[0]
                $mergedPr.review_attestation = Get-GitHubReviewAttestation `
                    -GhExecutable $ghCommand.Source `
                    -PullRequestNumber ([long](Get-PropertyValue $mergedPr 'number'))
                $mergedPr.post_merge = Get-GitHubPostMergeEvidence `
                    -GhExecutable $ghCommand.Source `
                    -HeadOid ([string](Get-PropertyValue $mergedPr 'head_oid')) `
                    -MergeCommitOid ([string](Get-PropertyValue $mergedPr 'merge_commit_oid'))
                $mainWorkflowEvidence = Get-GitHubWorkflowEvidence `
                    -GhExecutable $ghCommand.Source `
                    -HeadOid ([string](Get-PropertyValue $mergedPr 'merge_commit_oid')) `
                    -RequiredWorkflows $RequiredWorkflows `
                    -Event push
                $mergedPr.workflow_runs = $mainWorkflowEvidence.Runs
            }
            $githubAvailable = $true
        } catch {
            $githubAvailable = $false
            $activeIssues = @()
            $activePrs = @()
            $mergedPrs = @()
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
            $writers = (ConvertFrom-StrictJsonArrayOutput -Output $writerOutput -Label 'paseo writer list').Items
            foreach ($writer in $writers) {
                if ((Get-PropertyValue $writer 'id') -isnot [string] -or
                    (Get-PropertyValue $writer 'status') -isnot [string] -or
                    (Get-PropertyValue $writer 'cwd') -isnot [string]) {
                    throw 'paseo writer item schema invalid'
                }
            }
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
        observed_remote_main = $remoteMain
        expected_base = $expectedBase
        worktree_head = $worktreeHead
        branch = $branch
        worktree_clean = $worktreeClean
        active_writer_count = [long]$activeWriterCount
        repository_settings = $repositorySettings
        actual_scope_count = $actualScope.count
        actual_scope_hash = $actualScope.scope_hash
        active_issues = @($activeIssues)
        active_prs = @($activePrs)
        merged_prs = @($mergedPrs)
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
$requiredWorkflowsProperty = $policyResult.PSObject.Properties['required_workflows']
if ($null -eq $requiredWorkflowsProperty -or $requiredWorkflowsProperty.Value -isnot [System.Array]) {
    Write-Result -ExitCode 12 -Value ([pscustomobject][ordered]@{
        schema_version = 1
        ok = $false
        error_code = 'AUTONOMOUS_STATE_POLICY_INVALID'
    })
}
$requiredWorkflows = @($requiredWorkflowsProperty.Value)
$fixedFileMutationTranche = Get-FixedFileMutationTrancheProjection `
    (Get-PropertyValue $policyResult 'fixed_file_mutation_tranche')
if ((Get-PropertyValue $fixedFileMutationTranche 'valid') -ne $true) {
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
        $snapshot = Get-LiveSnapshot `
            -UpstreamSyncTaskMarker ([string](Get-PropertyValue (Get-PropertyValue $policyResult 'upstream_sync') 'task_marker')) `
            -CandidateExhaustedTaskMarker ([string](Get-PropertyValue (Get-PropertyValue $policyResult 'candidate_exhaustion') 'task_marker')) `
            -RequiredWorkflows $requiredWorkflows
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
    'observed_remote_main',
    'expected_base',
    'worktree_head',
    'branch',
    'worktree_clean',
    'active_writer_count',
    'repository_settings',
    'actual_scope_count',
    'actual_scope_hash',
    'active_issues',
    'active_prs',
    'merged_prs'
)
$missingProperties = @($requiredProperties | Where-Object { $null -eq $snapshot.PSObject.Properties[$_] })
$shaPattern = '^[0-9a-f]{40}$'
$githubAvailable = Get-PropertyValue $snapshot 'github_available'
$paseoAvailable = Get-PropertyValue $snapshot 'paseo_available'
$releaseAudit = Get-PropertyValue $snapshot 'release_audit'
$branchValue = Get-PropertyValue $snapshot 'branch'
$worktreeClean = Get-PropertyValue $snapshot 'worktree_clean'
$repositorySettingsValue = Get-PropertyValue $snapshot 'repository_settings'
$actualScopeCount = Get-PropertyValue $snapshot 'actual_scope_count'
$actualScopeHash = Get-PropertyValue $snapshot 'actual_scope_hash'
$activeIssuesValue = $null
$activePrsValue = $null
$mergedPrsValue = $null
if ($null -ne $snapshot.PSObject.Properties['active_issues']) {
    $activeIssuesValue = $snapshot.PSObject.Properties['active_issues'].Value
}
if ($null -ne $snapshot.PSObject.Properties['active_prs']) {
    $activePrsValue = $snapshot.PSObject.Properties['active_prs'].Value
}
if ($null -ne $snapshot.PSObject.Properties['merged_prs']) {
    $mergedPrsValue = $snapshot.PSObject.Properties['merged_prs'].Value
}
if ((Get-PropertyValue $snapshot 'schema_version') -cne 'inputcodex.autonomous-refactor-state-snapshot.v1' -or
    $missingProperties.Count -ne 0 -or
    $githubAvailable -isnot [bool] -or
    $paseoAvailable -isnot [bool] -or
    $releaseAudit -isnot [string] -or
    $branchValue -isnot [string] -or
    [string]::IsNullOrWhiteSpace($branchValue) -or
    $worktreeClean -isnot [bool] -or
    $repositorySettingsValue -isnot [pscustomobject] -or
    (Get-PropertyValue $repositorySettingsValue 'allow_auto_merge') -isnot [bool] -or
    (Get-PropertyValue $repositorySettingsValue 'allow_squash_merge') -isnot [bool] -or
    (Get-PropertyValue $repositorySettingsValue 'allow_merge_commit') -isnot [bool] -or
    (Get-PropertyValue $repositorySettingsValue 'allow_rebase_merge') -isnot [bool] -or
    (Get-PropertyValue $repositorySettingsValue 'default_branch') -isnot [string] -or
    $actualScopeCount -isnot [long] -or
    $actualScopeCount -lt 0 -or
    [string]$actualScopeHash -cnotmatch '^sha256:[0-9a-f]{64}$' -or
    $activeIssuesValue -isnot [System.Array] -or
    $activePrsValue -isnot [System.Array] -or
    $mergedPrsValue -isnot [System.Array] -or
    [string](Get-PropertyValue $snapshot 'observed_origin_main') -cnotmatch $shaPattern -or
    [string](Get-PropertyValue $snapshot 'observed_remote_main') -cnotmatch $shaPattern -or
    [string](Get-PropertyValue $snapshot 'expected_base') -cnotmatch $shaPattern -or
    [string](Get-PropertyValue $snapshot 'worktree_head') -cnotmatch $shaPattern) {
    Write-Result -ExitCode 11 -Value ([pscustomobject][ordered]@{
        schema_version = 1
        ok = $false
        error_code = 'AUTONOMOUS_STATE_INVALID_SNAPSHOT'
    })
}

$markerIssues = @($activeIssuesValue)
$markerPrs = @($activePrsValue)
$markerMergedPrs = @($mergedPrsValue)
$activeIssues = @($markerIssues | Where-Object {
    (Get-PropertyValue $_ 'author_login') -ceq 'nonononull'
})
$activePrs = @($markerPrs | Where-Object {
    (Get-PropertyValue $_ 'author_login') -ceq 'nonononull' -and
    (Get-PropertyValue $_ 'head_owner_login') -ceq 'nonononull'
})
$trustedMergedPrs = @($markerMergedPrs | Where-Object {
    (Get-PropertyValue $_ 'author_login') -ceq 'nonononull' -and
    (Get-PropertyValue $_ 'head_owner_login') -ceq 'nonononull'
})
$linkedMergedPrs = @()
if ($activeIssues.Count -eq 1) {
    $linkedMergedPrs = @($trustedMergedPrs | Where-Object {
        $mergedEvidence = Get-PropertyValue $_ 'evidence'
        $null -ne $mergedEvidence -and
        (Get-PropertyValue $mergedEvidence 'valid') -eq $true -and
        (Get-PropertyValue $mergedEvidence 'tracking_issue_ref') -ceq (Get-PropertyValue $activeIssues[0] 'url')
    })
}
$ignoredUntrustedIssueMarkers = $markerIssues.Count - $activeIssues.Count
$ignoredUntrustedPrMarkers = $markerPrs.Count - $activePrs.Count
$ignoredUntrustedMergedPrMarkers = $markerMergedPrs.Count - $trustedMergedPrs.Count
$activeWriterCount = Get-PropertyValue $snapshot 'active_writer_count'
$reasonCodes = [System.Collections.Generic.List[string]]::new()
$activeTaskKind = $null
if ($activeIssues.Count -eq 1) {
    $activeTaskKindProjection = Get-PropertyProjection $activeIssues[0] 'task_kind'
    $activeTaskKind = $activeTaskKindProjection.value
    if (-not $activeTaskKindProjection.exists) {
        $activeTaskKind = 'refactor'
    } elseif ($activeTaskKind -isnot [string] -or
        $activeTaskKind -cnotin @('refactor', 'upstream-sync', 'candidate-exhausted')) {
        $reasonCodes.Add('INVALID_TASK_KIND') | Out-Null
    }
}
$candidateExhaustionPolicy = Get-PropertyValue $policyResult 'candidate_exhaustion'
$isCandidateExhaustedTask = $activeIssues.Count -eq 1 -and
    $activeTaskKind -ceq (Get-PropertyValue $candidateExhaustionPolicy 'task_kind')

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
if ($linkedMergedPrs.Count -gt 1) {
    $reasonCodes.Add('MULTIPLE_MERGED_PRS_FOR_ISSUE') | Out-Null
}
if ($activePrs.Count -gt 0 -and $linkedMergedPrs.Count -gt 0) {
    $reasonCodes.Add('OPEN_AND_MERGED_PR_FOR_ISSUE') | Out-Null
}
$isPostMergeTransition = $linkedMergedPrs.Count -eq 1 -and
    $activePrs.Count -eq 0 -and
    (Get-PropertyValue $linkedMergedPrs[0] 'merge_commit_oid') -ceq
        (Get-PropertyValue $snapshot 'observed_remote_main')
if ($linkedMergedPrs.Count -eq 1 -and -not $isPostMergeTransition) {
    $reasonCodes.Add('MERGED_PR_MAIN_DRIFT') | Out-Null
}
$upstreamSyncPolicy = Get-PropertyValue $policyResult 'upstream_sync'
$isAllowedUpstreamSyncStale = $activeIssues.Count -eq 1 -and
    $activeTaskKind -ceq (Get-PropertyValue $upstreamSyncPolicy 'task_kind') -and
    (Get-PropertyValue $snapshot 'release_audit') -ceq
        (Get-PropertyValue $upstreamSyncPolicy 'allowed_release_audit')
if ((Get-PropertyValue $snapshot 'release_audit') -cne 'current' -and
    -not $isAllowedUpstreamSyncStale) {
    $reasonCodes.Add('RELEASE_AUDIT_STALE') | Out-Null
}
if (-not $isPostMergeTransition -and
    (Get-PropertyValue $snapshot 'expected_base') -cne (Get-PropertyValue $snapshot 'observed_origin_main')) {
    $reasonCodes.Add('ORIGIN_MAIN_DRIFT') | Out-Null
}
if (-not $isPostMergeTransition -and (
    (Get-PropertyValue $snapshot 'observed_origin_main') -cne (Get-PropertyValue $snapshot 'observed_remote_main') -or
    (Get-PropertyValue $snapshot 'expected_base') -cne (Get-PropertyValue $snapshot 'observed_remote_main'))) {
    $reasonCodes.Add('REMOTE_MAIN_DRIFT') | Out-Null
}
if ($branchValue -in @('main', 'master') -and $worktreeClean -ne $true) {
    $reasonCodes.Add('PROTECTED_BRANCH_DIRTY') | Out-Null
}
if ($githubAvailable -eq $true -and $activeIssues.Count -eq 1 -and $branchValue -notin @('main', 'master')) {
    $expectedIssueBranchPrefix = "codex/issue-$((Get-PropertyValue $activeIssues[0] 'number'))-"
    if (-not $branchValue.StartsWith($expectedIssueBranchPrefix, [StringComparison]::Ordinal)) {
        $reasonCodes.Add('ISSUE_BRANCH_MISMATCH') | Out-Null
    }
}
if ($githubAvailable -eq $true -and
    $activeIssues.Count -eq 0 -and
    $activePrs.Count -eq 0 -and
    $branchValue -notin @('main', 'master')) {
    $reasonCodes.Add('ORPHANED_TASK_BRANCH') | Out-Null
}
if ($activePrs.Count -eq 1) {
    if ($branchValue -in @('main', 'master')) {
        $reasonCodes.Add('PR_BRANCH_INVALID') | Out-Null
    }
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
if ((Get-PropertyValue $snapshot 'github_available') -eq $true -and
    $activeIssues.Count -eq 0 -and
    (Get-PropertyValue $snapshot 'worktree_clean') -ne $true) {
    $reasonCodes.Add('ORPHANED_DIRTY_WORKTREE') | Out-Null
}
if ($isCandidateExhaustedTask) {
    $candidateLabelsProjection = Get-PropertyProjection $activeIssues[0] 'labels'
    $candidateLabels = $candidateLabelsProjection.value
    $requiredCandidateLabel = Get-PropertyValue $candidateExhaustionPolicy 'required_label'
    if (-not $candidateLabelsProjection.exists -or
        $candidateLabels -isnot [System.Array] -or
        @($candidateLabels | Where-Object { $_ -isnot [string] }).Count -ne 0 -or
        @($candidateLabels | Where-Object { $_ -ceq $requiredCandidateLabel }).Count -ne 1) {
        $reasonCodes.Add('CANDIDATE_EXHAUSTED_LABEL_INVALID') | Out-Null
    }

    $emptyScopeHash = 'sha256:01ba4719c80b6fe911b091a7c05124b64eeece964e09c058ef8f9805daca546b'
    if ($branchValue -cne 'main' -or
        $worktreeClean -ne $true -or
        (Get-PropertyValue $snapshot 'worktree_head') -cne (Get-PropertyValue $snapshot 'observed_origin_main') -or
        (Get-PropertyValue $snapshot 'worktree_head') -cne (Get-PropertyValue $snapshot 'observed_remote_main') -or
        (Get-PropertyValue $snapshot 'worktree_head') -cne (Get-PropertyValue $snapshot 'expected_base') -or
        $actualScopeCount -ne 0L -or
        $actualScopeHash -cne $emptyScopeHash -or
        $releaseAudit -cne 'current') {
        $reasonCodes.Add('CANDIDATE_EXHAUSTED_REPOSITORY_STATE_INVALID') | Out-Null
    }
    if ($activePrs.Count -ne 0 -or $linkedMergedPrs.Count -ne 0) {
        $reasonCodes.Add('CANDIDATE_EXHAUSTED_DELIVERY_PRESENT') | Out-Null
    }
}

$externalReasons = [System.Collections.Generic.List[string]]::new()
if ((Get-PropertyValue $snapshot 'github_available') -ne $true) {
    $externalReasons.Add('GITHUB_UNAVAILABLE') | Out-Null
}
if ((Get-PropertyValue $snapshot 'paseo_available') -ne $true) {
    $externalReasons.Add('PASEO_UNAVAILABLE') | Out-Null
}

$mergeGatePending = @()
if ($reasonCodes.Count -eq 0 -and $activeIssues.Count -eq 1 -and $activePrs.Count -eq 1) {
    $mergeGateEvaluation = Get-AutonomousMergeGateEvaluation `
        -PullRequest $activePrs[0] `
        -Issue $activeIssues[0] `
        -Snapshot $snapshot `
        -PolicySha256 ([string](Get-PropertyValue $policyResult 'policy_sha256')) `
        -TaskKind $activeTaskKind `
        -RequiredWorkflows $requiredWorkflows
    $mergeGatePending = @($mergeGateEvaluation.Pending)
}
$postMergeGatePending = @()
if ($reasonCodes.Count -eq 0 -and $isPostMergeTransition) {
    $postMergeGateEvaluation = Get-AutonomousPostMergeGateEvaluation `
        -MergedPullRequest $linkedMergedPrs[0] `
        -Issue $activeIssues[0] `
        -Snapshot $snapshot `
        -PolicySha256 ([string](Get-PropertyValue $policyResult 'policy_sha256')) `
        -TaskKind $activeTaskKind `
        -RequiredWorkflows $requiredWorkflows
    $postMergeGatePending = @($postMergeGateEvaluation.Pending)
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
} elseif ($isCandidateExhaustedTask) {
    $state = Get-PropertyValue $candidateExhaustionPolicy 'state'
    $nextAction = Get-PropertyValue $candidateExhaustionPolicy 'next_action'
    $allReasons = @()
} elseif ($activeIssues.Count -eq 1 -and (Get-PropertyValue $snapshot 'worktree_clean') -ne $true) {
    $state = 'active-worktree-execution'
    $nextAction = 'resume-worktree'
    $allReasons = @()
} elseif ($isPostMergeTransition) {
    $state = 'post-merge-verification'
    $nextAction = if ($postMergeGatePending.Count -eq 0) { 'close-issue-and-archive' } else { 'verify-main' }
    $allReasons = @()
} elseif ($activePrs.Count -eq 1 -and $mergeGatePending.Count -eq 0) {
    $state = 'merge-ready-exact-head'
    $nextAction = 'squash-merge-exact-head'
    $allReasons = @()
} elseif ($activePrs.Count -eq 1) {
    $state = 'active-pr-review-ci'
    $nextAction = 'resume-pr'
    $allReasons = @()
} elseif ($activeIssues.Count -eq 1) {
    $state = 'active-issue-planning'
    $nextAction = 'resume-issue'
    $allReasons = @()
} else {
    $state = 'idle-select-candidate'
    $nextAction = 'select-fixed-file-mutation-candidate'
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
    merge_gate_pending = @($mergeGatePending)
    post_merge_gate_pending = @($postMergeGatePending)
    selected_candidate = if ($state -ceq 'idle-select-candidate') {
        Get-PropertyValue $fixedFileMutationTranche 'candidate'
    } else {
        $null
    }
    fixed_file_mutation_tranche = $fixedFileMutationTranche
    active_issue = if ($activeIssues.Count -eq 1) { $activeIssues[0] } else { $null }
    active_pr = if ($activePrs.Count -eq 1) { $activePrs[0] } else { $null }
    merged_pr = if ($linkedMergedPrs.Count -eq 1) { $linkedMergedPrs[0] } else { $null }
    expected_base = Get-PropertyValue $snapshot 'expected_base'
    observed_origin_main = Get-PropertyValue $snapshot 'observed_origin_main'
    observed_remote_main = Get-PropertyValue $snapshot 'observed_remote_main'
    observed_head = Get-PropertyValue $snapshot 'worktree_head'
    branch = Get-PropertyValue $snapshot 'branch'
    worktree_clean = Get-PropertyValue $snapshot 'worktree_clean'
    active_writer_count = $activeWriterCount
    ignored_untrusted_issue_markers = $ignoredUntrustedIssueMarkers
    ignored_untrusted_pr_markers = $ignoredUntrustedPrMarkers
    ignored_untrusted_merged_pr_markers = $ignoredUntrustedMergedPrMarkers
})
