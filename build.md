# inputcodex 构建与验证说明

## 当前状态

截至 2026 年 7 月 21 日，Gate 1 已通过 PR `#7` 完成最终治理收口。Issue `#8` 是一次性的 Gate 1→2 控制面过渡任务，Issue `#9` 是持续开放的 Gate 2 upstream-sync 活动任务。

仓库当前仍没有应用源码，因此没有 Cargo、Iced、安装包或发布构建命令。本文件提供两个检查点：

1. Issue `#8` 过渡 PR 合并前验证。
2. 过渡 PR Squash Merge 后的 Gate 2 最终状态验证。

当前禁止：

- 创建或修改 `upstream/`、`source-lock.json`。
- 创建 `Cargo.toml`、Rust/Iced 源码、临时 UI 或 WebView。
- 创建 `.github/workflows/`、Release、安装包、签名或更新资产。
- 修改 Ruleset、required checks 或仓库级合并开关。
- 修改或优化外部 AGOS。

## 环境要求

- PowerShell 7。
- Git。
- GitHub CLI `gh`，已登录 `nonononull`。
- Python 3 与 PyYAML。

```powershell
Set-Location 'C:\Users\dashuai\Documents\inputcodex'
$ErrorActionPreference = 'Stop'
Set-StrictMode -Version Latest
```

原生 `git`、`gh`、`python` 命令后必须立即检查 `$LASTEXITCODE`。只有一行输出时使用 `@(...)` 归一化，禁止把空 stdout 当成成功证据。

## Gate 2 本地 Fresh 验证

```powershell
$expectedFiles = @(
  'AGENTS.md',
  'README.md',
  'build.md',
  'err.md',
  'docs/plans/PROJECT-MASTER-PLAN.md',
  'docs/plans/2026-07-21-architecture-governance.md',
  'docs/plans/2026-07-21-issue-8-gate-2-transition.md',
  'docs/plans/2026-07-21-issue-9-gate-2-upstream-baseline.md',
  'docs/plans/sessions/2026-07-21-issue-8-gate-2-transition.md',
  'docs/plans/sessions/2026-07-21-issue-9-gate-2-upstream-baseline.md',
  'docs/workflows/2026-07-21-issue-8-gate-2-transition-runtime.md',
  'docs/workflows/2026-07-21-issue-9-gate-2-upstream-baseline-runtime.md',
  'docs/reports/issue-6-gate-1-finalization-closeout.md',
  '.github/pull_request_template.md',
  '.github/ISSUE_TEMPLATE/config.yml',
  '.github/ISSUE_TEMPLATE/upstream-watch.yml',
  '.github/ISSUE_TEMPLATE/upstream-sync.yml',
  '.github/ISSUE_TEMPLATE/feature-parity.yml',
  '.github/ISSUE_TEMPLATE/parity-exception.yml',
  '.github/ISSUE_TEMPLATE/performance.yml',
  '.github/ISSUE_TEMPLATE/architecture.yml',
  '.github/ISSUE_TEMPLATE/release.yml',
  '.github/ISSUE_TEMPLATE/bug.yml'
)

foreach ($path in $expectedFiles) {
  if (-not (Test-Path -LiteralPath $path -PathType Leaf)) {
    throw "缺少 Gate 2 控制文件：$path"
  }
}

$forbiddenPaths = @(
  'upstream',
  'source-lock.json',
  'Cargo.toml',
  'Cargo.lock',
  'package.json',
  'package-lock.json',
  'pnpm-lock.yaml',
  'yarn.lock'
)

foreach ($path in $forbiddenPaths) {
  if (Test-Path -LiteralPath $path) {
    throw "Gate 2 规划阶段禁止出现：$path"
  }
}

$rustFiles = @(
  Get-ChildItem -LiteralPath . -Recurse -File -Filter '*.rs' |
  Where-Object { $_.FullName -notmatch '[\\/]\.git[\\/]' }
)
if ($rustFiles.Count -ne 0) {
  throw "Gate 2 规划阶段禁止 Rust 源码，发现 $($rustFiles.Count) 个文件。"
}

if (Test-Path -LiteralPath '.github/workflows') {
  $workflowFiles = @(Get-ChildItem -LiteralPath '.github/workflows' -Recurse -File)
  if ($workflowFiles.Count -ne 0) {
    throw 'Gate 2 规划阶段禁止 GitHub Actions Workflow。'
  }
}

$currentFiles = @(
  'README.md',
  'docs/plans/PROJECT-MASTER-PLAN.md',
  'docs/plans/2026-07-21-architecture-governance.md',
  'docs/plans/2026-07-21-issue-8-gate-2-transition.md',
  'docs/plans/2026-07-21-issue-9-gate-2-upstream-baseline.md',
  'docs/plans/sessions/2026-07-21-issue-9-gate-2-upstream-baseline.md',
  'docs/workflows/2026-07-21-issue-9-gate-2-upstream-baseline-runtime.md'
)

$stalePatterns = @(
  'active_task: 2026-07-21-issue-6-gate-1-finalization',
  'PR `#7`.*OPEN',
  'PR `#7`.*等待项目所有者 Review',
  'Issue `#1`.*保持 OPEN',
  'Issue `#6`.*保持 OPEN'
)

foreach ($pattern in $stalePatterns) {
  $matches = @(Select-String -LiteralPath $currentFiles -Pattern $pattern)
  if ($matches.Count -ne 0) {
    throw "发现过期 Gate 1 状态：$pattern"
  }
}

$requiredStatements = @(
  @{ Path = 'README.md'; Pattern = 'Gate 2 上游基线规划阶段' },
  @{ Path = 'README.md'; Pattern = 'Issue `#9`' },
  @{ Path = 'docs/plans/PROJECT-MASTER-PLAN.md'; Pattern = 'active_task: 2026-07-21-issue-9-gate-2-upstream-baseline' },
  @{ Path = 'docs/plans/PROJECT-MASTER-PLAN.md'; Pattern = 'active_gate: Gate 2' },
  @{ Path = 'docs/plans/PROJECT-MASTER-PLAN.md'; Pattern = 'transition_pr_ref: https://github.com/nonononull/inputcodex/pull/10' },
  @{ Path = 'docs/reports/issue-6-gate-1-finalization-closeout.md'; Pattern = 'c74b66422ba47f96bd3eb2b2385cdfb90541808e' },
  @{ Path = 'docs/plans/2026-07-21-issue-9-gate-2-upstream-baseline.md'; Pattern = '尚未批准快照写入' }
)

foreach ($statement in $requiredStatements) {
  if (-not (Select-String -LiteralPath $statement.Path -SimpleMatch $statement.Pattern -Quiet)) {
    throw "缺少 Gate 2 硬约束：$($statement.Path) -> $($statement.Pattern)"
  }
}

$branch = git branch --show-current
if ($LASTEXITCODE -ne 0 -or $branch -ne 'codex/issue-8-gate-2-transition') {
  throw "当前过渡分支不正确：$branch"
}

git diff --check
if ($LASTEXITCODE -ne 0) {
  throw 'git diff --check 失败。'
}

Write-Output 'GATE2_LOCAL_CONTROL_PLANE_VERIFY_OK'
```

## Issue Forms YAML 验证

```powershell
$python = @"
from pathlib import Path
import yaml

root = Path('.github/ISSUE_TEMPLATE')
expected = {
    'upstream-watch.yml': {'type:upstream-watch'},
    'upstream-sync.yml': {'type:upstream-sync', 'gate:2'},
    'feature-parity.yml': {'type:feature-parity'},
    'parity-exception.yml': {'type:parity-exception', 'status:needs-owner-decision'},
    'performance.yml': {'type:performance'},
    'architecture.yml': {'type:architecture'},
    'release.yml': {'type:release', 'gate:6'},
    'bug.yml': {'type:bug'},
}

for filename, required_labels in expected.items():
    data = yaml.safe_load((root / filename).read_text(encoding='utf-8'))
    assert isinstance(data, dict), filename
    assert data.get('name') and data.get('description') and data.get('title'), filename
    assert required_labels <= set(data.get('labels') or []), filename
    body = data.get('body')
    assert isinstance(body, list) and body, filename
    ids = [item.get('id') for item in body if isinstance(item, dict) and item.get('id')]
    assert len(ids) == len(set(ids)), (filename, ids)

config = yaml.safe_load((root / 'config.yml').read_text(encoding='utf-8'))
assert config == {'blank_issues_enabled': False, 'contact_links': []}
print('ISSUE_FORMS_YAML_VERIFY_OK')
"@

$python | python -
if ($LASTEXITCODE -ne 0) {
  throw 'Issue Forms YAML 验证失败。'
}
```

## PR #7 与 Gate 1 closeout 核验

```powershell
$repo = 'nonononull/inputcodex'

$issue1 = gh issue view 1 --repo $repo --json state,stateReason,closedAt | ConvertFrom-Json
if ($LASTEXITCODE -ne 0) { throw '读取 Issue #1 失败。' }
$issue6 = gh issue view 6 --repo $repo --json state,closedAt | ConvertFrom-Json
if ($LASTEXITCODE -ne 0) { throw '读取 Issue #6 失败。' }
$pr7 = gh pr view 7 --repo $repo --json state,mergedAt,mergeCommit,headRefOid,statusCheckRollup | ConvertFrom-Json
if ($LASTEXITCODE -ne 0) { throw '读取 PR #7 失败。' }

if ($issue1.state -ne 'CLOSED' -or $issue1.stateReason -ne 'COMPLETED') {
  throw 'Issue #1 未以 completed 关闭。'
}
if ($issue6.state -ne 'CLOSED') {
  throw 'Issue #6 未关闭。'
}
if ($pr7.state -ne 'MERGED' -or
    $pr7.mergeCommit.oid -ne 'c74b66422ba47f96bd3eb2b2385cdfb90541808e' -or
    $pr7.headRefOid -ne 'e8b8631685e1b2f4361897016250b525f6d7c813' -or
    @($pr7.statusCheckRollup).Count -ne 0) {
  throw 'PR #7 合并证据变化。'
}

$commit = gh api repos/$repo/git/commits/c74b66422ba47f96bd3eb2b2385cdfb90541808e | ConvertFrom-Json
if ($LASTEXITCODE -ne 0) { throw '读取 PR #7 merge commit 失败。' }
if (-not $commit.verification.verified -or
    $commit.verification.reason -ne 'valid' -or
    @($commit.parents).Count -ne 1 -or
    $commit.parents[0].sha -ne 'b7404b0c63f2d2ba65474c077182c42a01cc9a64' -or
    $commit.tree.sha -ne '00f0f7fe0e408a1e6f218ee8e1be0d8442ed1e65') {
  throw 'PR #7 签名、parent 或 tree 证据变化。'
}

$query = 'query($owner:String!,$name:String!,$number:Int!){repository(owner:$owner,name:$name){pullRequest(number:$number){reviewThreads(first:100){nodes{isResolved}}}}}'
$review = gh api graphql -f query=$query -F owner=nonononull -F name=inputcodex -F number=7 | ConvertFrom-Json
if ($LASTEXITCODE -ne 0) { throw '读取 PR #7 Review 对话失败。' }
$unresolved = @($review.data.repository.pullRequest.reviewThreads.nodes | Where-Object { -not $_.isResolved })
if ($unresolved.Count -ne 0) {
  throw "PR #7 仍有 $($unresolved.Count) 个未解决 Review 对话。"
}

gh api repos/$repo/git/ref/heads/codex/issue-6-gate-1-finalization --silent 2>$null
if ($LASTEXITCODE -ne 1) {
  throw 'PR #7 远端旧分支仍存在或查询异常。'
}

Write-Output 'PR7_GATE1_CLOSEOUT_VERIFY_OK'
```

## Gate 2 Issues、Ruleset 与上游基线核验

```powershell
$repo = 'nonononull/inputcodex'

$issue8 = gh issue view 8 --repo $repo --json state,labels,url | ConvertFrom-Json
if ($LASTEXITCODE -ne 0) { throw '读取 Issue #8 失败。' }
$issue9 = gh issue view 9 --repo $repo --json state,labels,url | ConvertFrom-Json
if ($LASTEXITCODE -ne 0) { throw '读取 Issue #9 失败。' }
if ($issue8.state -ne 'OPEN' -or $issue9.state -ne 'OPEN') {
  throw '过渡 PR 合并前 Issue #8/#9 都必须保持 OPEN。'
}
foreach ($label in @('type:architecture', 'gate:1', 'gate:2')) {
  if ($label -notin @($issue8.labels.name)) { throw "Issue #8 缺少标签：$label" }
}
foreach ($label in @('type:upstream-sync', 'gate:2')) {
  if ($label -notin @($issue9.labels.name)) { throw "Issue #9 缺少标签：$label" }
}

$ruleset = gh api repos/$repo/rulesets/19395456 | ConvertFrom-Json
if ($LASTEXITCODE -ne 0) { throw '读取 Ruleset 失败。' }
$pullRequestRule = @($ruleset.rules | Where-Object { $_.type -eq 'pull_request' })
$allowedMethods = (@($pullRequestRule[0].parameters.allowed_merge_methods) -join ',')
if ($ruleset.enforcement -ne 'active' -or
    @($ruleset.bypass_actors).Count -ne 0 -or
    @($ruleset.rules | Where-Object { $_.type -eq 'deletion' }).Count -ne 1 -or
    @($ruleset.rules | Where-Object { $_.type -eq 'non_fast_forward' }).Count -ne 1 -or
    $pullRequestRule.Count -ne 1 -or
    $pullRequestRule[0].parameters.required_approving_review_count -ne 0 -or
    -not $pullRequestRule[0].parameters.required_review_thread_resolution -or
    $allowedMethods -ne 'squash') {
  throw 'main-protection Ruleset 不符合批准值。'
}

$release = gh api repos/BigPizzaV3/CodexPlusPlus/releases/latest | ConvertFrom-Json
if ($LASTEXITCODE -ne 0 -or $release.tag_name -ne 'v1.2.41') {
  throw '上游最新正式 Release 已变化。'
}
$upstreamCommit = gh api repos/BigPizzaV3/CodexPlusPlus/commits/v1.2.41 | ConvertFrom-Json
if ($LASTEXITCODE -ne 0 -or $upstreamCommit.sha -ne '3dafffcafb2566a1e8bce4b35671656d6adb3eda') {
  throw '上游 v1.2.41 提交已变化。'
}

$workflows = gh api repos/$repo/actions/workflows | ConvertFrom-Json
if ($LASTEXITCODE -ne 0 -or $workflows.total_count -ne 0) {
  throw 'Gate 2 规划阶段不允许 Actions Workflow。'
}
$releases = @(gh api repos/$repo/releases | ConvertFrom-Json)
if ($LASTEXITCODE -ne 0 -or $releases.Count -ne 0) {
  throw 'Gate 2 规划阶段不允许项目 Release。'
}

Write-Output 'GATE2_ISSUES_RULESET_UPSTREAM_VERIFY_OK'
```

## Issue #8 过渡 PR 合并前复核

本节在过渡 PR 创建并回写真实 URL 后执行：

```powershell
$repo = 'nonononull/inputcodex'
$branch = 'codex/issue-8-gate-2-transition'

$pullRequests = @(
  gh pr list --repo $repo --head $branch --state open --json number,state,isDraft,mergeStateStatus,headRefOid,body,url |
  ConvertFrom-Json
)
if ($LASTEXITCODE -ne 0) { throw '读取 Issue #8 过渡 PR 失败。' }
if ($pullRequests.Count -ne 1 -or
    $pullRequests[0].number -ne 10 -or
    $pullRequests[0].state -ne 'OPEN' -or
    $pullRequests[0].isDraft -or
    $pullRequests[0].mergeStateStatus -ne 'CLEAN' -or
    $pullRequests[0].body -notmatch 'Closes\s+#8') {
  throw 'Issue #8 过渡 PR 状态不符合授权合并条件。'
}

$localHead = git rev-parse HEAD
if ($LASTEXITCODE -ne 0) { throw '读取本地 HEAD 失败。' }
$trackingHead = git rev-parse refs/remotes/origin/codex/issue-8-gate-2-transition
if ($LASTEXITCODE -ne 0) { throw '读取过渡分支远端跟踪 HEAD 失败。' }
if ($localHead -ne $trackingHead -or $localHead -ne $pullRequests[0].headRefOid) {
  throw '本地、远端跟踪和过渡 PR Head 不一致。'
}

$query = 'query($owner:String!,$name:String!,$number:Int!){repository(owner:$owner,name:$name){pullRequest(number:$number){reviewThreads(first:100){nodes{isResolved}} autoMergeRequest{enabledAt}}}}'
$review = gh api graphql -f query=$query -F owner=nonononull -F name=inputcodex -F number=10 | ConvertFrom-Json
if ($LASTEXITCODE -ne 0) { throw '读取过渡 PR Review 对话失败。' }
$unresolved = @($review.data.repository.pullRequest.reviewThreads.nodes | Where-Object { -not $_.isResolved })
if ($unresolved.Count -ne 0 -or $null -ne $review.data.repository.pullRequest.autoMergeRequest) {
  throw '过渡 PR 存在未解决 Review 对话或启用了自动合并。'
}

Write-Output 'ISSUE8_TRANSITION_PR_PREMERGE_VERIFY_OK'
```

## Gate 2 过渡合并后最终核验

本节只在 PR `#10` Squash Merge 后执行：

```powershell
$repo = 'nonononull/inputcodex'

$issue8 = gh issue view 8 --repo $repo --json state,closedAt | ConvertFrom-Json
if ($LASTEXITCODE -ne 0 -or $issue8.state -ne 'CLOSED') {
  throw 'Issue #8 未关闭。'
}
$issue9 = gh issue view 9 --repo $repo --json state | ConvertFrom-Json
if ($LASTEXITCODE -ne 0 -or $issue9.state -ne 'OPEN') {
  throw 'Issue #9 应保持 OPEN。'
}
$pr10 = gh pr view 10 --repo $repo --json state,mergeCommit,statusCheckRollup | ConvertFrom-Json
if ($LASTEXITCODE -ne 0 -or $pr10.state -ne 'MERGED' -or @($pr10.statusCheckRollup).Count -ne 0) {
  throw 'PR #10 未完成预期 Squash Merge。'
}
$main = (gh api repos/$repo/git/ref/heads/main | ConvertFrom-Json).object.sha
if ($LASTEXITCODE -ne 0 -or $main -ne $pr10.mergeCommit.oid) {
  throw 'main 未指向 PR #10 merge commit。'
}
gh api repos/$repo/git/ref/heads/codex/issue-8-gate-2-transition --silent 2>$null
if ($LASTEXITCODE -ne 1) {
  throw 'PR #10 远端过渡分支仍存在或查询异常。'
}

Write-Output 'GATE2_TRANSITION_FINAL_VERIFY_OK'
```

## Git 快照与提交前验证

```powershell
$branch = git branch --show-current
if ($LASTEXITCODE -ne 0 -or $branch -ne 'codex/issue-8-gate-2-transition') {
  throw "当前分支不正确：$branch"
}

$head = git rev-parse HEAD
if ($LASTEXITCODE -ne 0) { throw '读取 HEAD 失败。' }

git status --short --branch
if ($LASTEXITCODE -ne 0) { throw '读取 Git 状态失败。' }
git diff --check
if ($LASTEXITCODE -ne 0) { throw 'git diff --check 失败。' }
git diff --stat
if ($LASTEXITCODE -ne 0) { throw '读取 diff 统计失败。' }

Write-Output "GIT_SNAPSHOT_BRANCH=$branch"
Write-Output "GIT_SNAPSHOT_HEAD=$head"
```

暂存后执行：

```powershell
git diff --cached --check
if ($LASTEXITCODE -ne 0) { throw 'cached diff 检查失败。' }
git diff --cached --stat
if ($LASTEXITCODE -ne 0) { throw '读取 cached diff 统计失败。' }
git status --short --branch
```

## 外部 AGOS 使用边界

本任务使用项目原生控制面，不运行 AGOS。AGOS 不属于环境要求或合并门禁；不得在本过渡 PR 中修改、修复或优化其 Registry、脚本、规则、Workflow 或 Vault。

## 后续维护规则

- Issue `#9` 获得快照写入批准后，在同一 PR 更新本文件，加入 `source-lock.json`、快照校验和许可证验证命令。
- 建立首个 Cargo Workspace 时再加入 Rust 构建、测试、基准和三平台 CI 命令。
- 任何错误先查 `err.md`，重复问题优先复用既有根因与处理方案。
