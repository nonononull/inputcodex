# inputcodex 构建与验证说明

## 当前状态

截至 2026 年 7 月 21 日，仓库仍不包含应用源码，因此没有 Cargo、Iced、安装包或发布构建命令。当前正式任务是 Issue `#6` 的 Gate 1 仓库治理基线最终收口，只允许文档、Issue Forms、PR 模板、标签和已合并旧分支清理。

Gate 1 明确禁止：

- 导入上游或半成品源码。
- 创建 `Cargo.toml`、Rust/Iced 源码或临时 UI。
- 创建 `.github/workflows/`。
- 创建 Release、安装包、签名或更新资产。
- 修改 GitHub Ruleset、required checks 或仓库级合并开关。
- 修改或优化外部 AGOS。

Rust Workspace、双平台 CI、Cache 和发布构建的批准策略分别记录在：

- `docs/plans/2026-07-21-rust-ci-offload-strategy.md`
- `docs/plans/2026-07-21-rust-ci-offload-implementation-plan.md`

## 环境要求

- PowerShell 7 或兼容 PowerShell。
- Git。
- GitHub CLI `gh`，且已登录 `nonononull`。
- Python 3 与 PyYAML；当前验证环境为 Python `3.13.12`、PyYAML `6.0.3`。

先进入仓库：

```powershell
Set-Location 'C:\Users\dashuai\Documents\inputcodex'
```

所有原生 `git`、`gh` 和 `python` 命令执行后都必须立即检查 `$LASTEXITCODE`。PowerShell 脚本错误使用 `$ErrorActionPreference = 'Stop'`；二者不能互相替代。

## Gate 1 本地 Fresh 验证

在未暂存状态下执行以下命令：

```powershell
Set-StrictMode -Version Latest
$ErrorActionPreference = 'Stop'

$expectedFiles = @(
  'AGENTS.md',
  'README.md',
  'build.md',
  'err.md',
  'docs/plans/PROJECT-MASTER-PLAN.md',
  'docs/plans/2026-07-21-architecture-governance.md',
  'docs/plans/2026-07-21-issue-6-gate-1-finalization.md',
  'docs/plans/sessions/2026-07-21-issue-6-gate-1-finalization.md',
  'docs/workflows/2026-07-21-issue-6-gate-1-finalization-runtime.md',
  'docs/reports/issue-4-gate-1-closeout.md',
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
    throw "缺少 Gate 1 文件：$path"
  }
}

$forbiddenFiles = @(
  'Cargo.toml',
  'Cargo.lock',
  'package.json',
  'package-lock.json',
  'pnpm-lock.yaml',
  'yarn.lock'
)

foreach ($path in $forbiddenFiles) {
  if (Test-Path -LiteralPath $path) {
    throw "当前 Gate 1 禁止出现：$path"
  }
}

$rustFiles = @(
  Get-ChildItem -LiteralPath . -Recurse -File -Filter '*.rs' |
  Where-Object { $_.FullName -notmatch '[\\/]\.git[\\/]' }
)
if ($rustFiles.Count -ne 0) {
  throw "当前 Gate 1 禁止 Rust 源码，发现 $($rustFiles.Count) 个文件。"
}

if (Test-Path -LiteralPath '.github/workflows') {
  $workflowFiles = @(
    Get-ChildItem -LiteralPath '.github/workflows' -Recurse -File -ErrorAction Stop
  )
  if ($workflowFiles.Count -ne 0) {
    throw '当前 Gate 1 不允许创建 GitHub Actions Workflow。'
  }
}

$currentFiles = @(
  'README.md',
  'docs/plans/PROJECT-MASTER-PLAN.md',
  'docs/plans/2026-07-21-architecture-governance.md',
  'docs/plans/2026-07-21-issue-6-gate-1-finalization.md',
  'docs/plans/sessions/2026-07-21-issue-6-gate-1-finalization.md',
  'docs/workflows/2026-07-21-issue-6-gate-1-finalization-runtime.md'
)

$stalePatterns = @(
  'PR `#5`.*OPEN',
  'PR `#5`.*等待项目所有者 Review',
  'Issue `#4`.*当前 active',
  'active_task: 2026-07-21-issue-4-gate-1-closeout'
)

foreach ($pattern in $stalePatterns) {
  $matches = @(Select-String -LiteralPath $currentFiles -Pattern $pattern)
  if ($matches.Count -ne 0) {
    throw "发现过期 Gate 1 状态：$pattern"
  }
}

$requiredStatements = @(
  @{ Path = 'AGENTS.md'; Pattern = '禁止 TypeScript、JavaScript 业务代码和 WebView' },
  @{ Path = 'AGENTS.md'; Pattern = '所有 Review 对话' },
  @{ Path = 'README.md'; Pattern = 'Issue `#6`' },
  @{ Path = 'docs/plans/PROJECT-MASTER-PLAN.md'; Pattern = 'active_task: 2026-07-21-issue-6-gate-1-finalization' },
  @{ Path = 'docs/plans/PROJECT-MASTER-PLAN.md'; Pattern = 'pending:issue-6-pull-request' },
  @{ Path = '.github/pull_request_template.md'; Pattern = '根因' },
  @{ Path = '.github/pull_request_template.md'; Pattern = 'Fresh 验证' },
  @{ Path = '.github/pull_request_template.md'; Pattern = 'Squash Merge' }
)

foreach ($statement in $requiredStatements) {
  if (-not (Select-String -LiteralPath $statement.Path -SimpleMatch $statement.Pattern -Quiet)) {
    throw "缺少硬约束：$($statement.Path) -> $($statement.Pattern)"
  }
}

git diff --check
if ($LASTEXITCODE -ne 0) {
  throw 'git diff --check 失败。'
}

$branch = git branch --show-current
if ($LASTEXITCODE -ne 0 -or $branch -ne 'codex/issue-6-gate-1-finalization') {
  throw "当前分支不正确：$branch"
}

Write-Output 'GATE1_LOCAL_FILES_VERIFY_OK'
```

预期结果：输出 `GATE1_LOCAL_FILES_VERIFY_OK`，且没有 Rust、Cargo、Workflow 或过期 Gate 1 状态。

## Issue Forms YAML 验证

使用 PyYAML Fresh 解析全部模板，并验证关键字段、ID 唯一性和批准标签：

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
    path = root / filename
    data = yaml.safe_load(path.read_text(encoding='utf-8'))
    assert isinstance(data, dict), filename
    assert data.get('name'), filename
    assert data.get('description'), filename
    assert data.get('title'), filename
    labels = set(data.get('labels') or [])
    assert required_labels <= labels, (filename, labels)
    body = data.get('body')
    assert isinstance(body, list) and body, filename
    ids = [item.get('id') for item in body if isinstance(item, dict) and item.get('id')]
    assert len(ids) == len(set(ids)), (filename, ids)
    for item in body:
        assert isinstance(item, dict) and item.get('type'), (filename, item)
        if item.get('type') != 'markdown':
            assert item.get('id'), (filename, item)

config = yaml.safe_load((root / 'config.yml').read_text(encoding='utf-8'))
assert config == {'blank_issues_enabled': False, 'contact_links': []}
print('ISSUE_FORMS_YAML_VERIFY_OK')
"@

$python | python -
if ($LASTEXITCODE -ne 0) {
  throw 'Issue Forms YAML 验证失败。'
}
```

预期结果：输出 `ISSUE_FORMS_YAML_VERIFY_OK`。

## GitHub、Ruleset 与上游基线核验

本节需要网络和已登录的 `gh`：

```powershell
Set-StrictMode -Version Latest
$ErrorActionPreference = 'Stop'

$issue1 = gh issue view 1 --repo nonononull/inputcodex --json number,state,url | ConvertFrom-Json
if ($LASTEXITCODE -ne 0) { throw '读取 Issue #1 失败。' }
$issue4 = gh issue view 4 --repo nonononull/inputcodex --json number,state,closedAt,url | ConvertFrom-Json
if ($LASTEXITCODE -ne 0) { throw '读取 Issue #4 失败。' }
$issue6 = gh issue view 6 --repo nonononull/inputcodex --json number,state,labels,url | ConvertFrom-Json
if ($LASTEXITCODE -ne 0) { throw '读取 Issue #6 失败。' }
$pr5 = gh pr view 5 --repo nonononull/inputcodex --json number,state,isDraft,mergedAt,mergeCommit,headRefOid,statusCheckRollup | ConvertFrom-Json
if ($LASTEXITCODE -ne 0) { throw '读取 PR #5 失败。' }

if ($issue1.state -ne 'OPEN') { throw 'Issue #1 应在 Gate 1 PR 合并前保持 OPEN。' }
if ($issue4.state -ne 'CLOSED') { throw 'Issue #4 未关闭。' }
if ($issue6.state -ne 'OPEN') { throw 'Issue #6 不处于 OPEN。' }
if ($pr5.state -ne 'MERGED' -or $pr5.isDraft) { throw 'PR #5 不是已合并非 Draft PR。' }
if ($pr5.mergeCommit.oid -ne 'b7404b0c63f2d2ba65474c077182c42a01cc9a64') { throw 'PR #5 merge commit 变化。' }
if ($pr5.headRefOid -ne 'ecd34360ae5f6c0d1f2995ccc6724fe39bf95381') { throw 'PR #5 Head 变化。' }
if (@($pr5.statusCheckRollup).Count -ne 0) { throw 'Gate 1 PR #5 Checks 数量不再为 0。' }

$issue6Labels = @($issue6.labels | ForEach-Object { $_.name })
foreach ($label in @('type:architecture', 'gate:1')) {
  if ($label -notin $issue6Labels) { throw "Issue #6 缺少标签：$label" }
}

$reviewQuery = 'query($owner:String!,$name:String!,$number:Int!){repository(owner:$owner,name:$name){pullRequest(number:$number){reviewThreads(first:100){totalCount nodes{isResolved}}}}}'
$review = gh api graphql -f query=$reviewQuery -F owner=nonononull -F name=inputcodex -F number=5 | ConvertFrom-Json
if ($LASTEXITCODE -ne 0) { throw '读取 PR #5 Review 对话失败。' }
$threads = @($review.data.repository.pullRequest.reviewThreads.nodes)
$unresolved = @($threads | Where-Object { -not $_.isResolved })
if ($threads.Count -ne 0 -or $unresolved.Count -ne 0) { throw 'PR #5 Review 对话证据变化。' }

$merge = gh api repos/nonononull/inputcodex/git/commits/b7404b0c63f2d2ba65474c077182c42a01cc9a64 | ConvertFrom-Json
if ($LASTEXITCODE -ne 0) { throw '读取 PR #5 merge commit 失败。' }
$head = gh api repos/nonononull/inputcodex/git/commits/ecd34360ae5f6c0d1f2995ccc6724fe39bf95381 | ConvertFrom-Json
if ($LASTEXITCODE -ne 0) { throw '读取 PR #5 Head commit 失败。' }
if (@($merge.parents).Count -ne 1 -or $merge.tree.sha -ne $head.tree.sha -or $merge.tree.sha -ne 'af186e05673b441a936199e55c7d632cd06ea929') {
  throw 'PR #5 Squash tree 证据不成立。'
}

$ruleset = gh api repos/nonononull/inputcodex/rulesets/19395456 | ConvertFrom-Json
if ($LASTEXITCODE -ne 0) { throw '读取 Ruleset 失败。' }
$pullRequestRule = @($ruleset.rules | Where-Object { $_.type -eq 'pull_request' })
if ($ruleset.enforcement -ne 'active' -or
    @($ruleset.conditions.ref_name.include).Count -ne 1 -or
    $ruleset.conditions.ref_name.include[0] -ne 'refs/heads/main' -or
    @($ruleset.bypass_actors).Count -ne 0 -or
    @($ruleset.rules | Where-Object { $_.type -eq 'deletion' }).Count -ne 1 -or
    @($ruleset.rules | Where-Object { $_.type -eq 'non_fast_forward' }).Count -ne 1 -or
    $pullRequestRule.Count -ne 1 -or
    $pullRequestRule[0].parameters.required_approving_review_count -ne 0 -or
    -not $pullRequestRule[0].parameters.required_review_thread_resolution -or
    (@($pullRequestRule[0].parameters.allowed_merge_methods) -join ',') -ne 'squash') {
  throw 'main-protection Ruleset 不符合批准值。'
}

$workflows = gh api repos/nonononull/inputcodex/actions/workflows | ConvertFrom-Json
if ($LASTEXITCODE -ne 0 -or $workflows.total_count -ne 0) { throw 'Gate 1 不允许存在 Actions Workflow。' }
$releases = @(gh api repos/nonononull/inputcodex/releases | ConvertFrom-Json)
if ($LASTEXITCODE -ne 0 -or $releases.Count -ne 0) { throw 'Gate 1 不允许存在项目 Release。' }

$upstreamRelease = gh api repos/BigPizzaV3/CodexPlusPlus/releases/latest | ConvertFrom-Json
if ($LASTEXITCODE -ne 0 -or $upstreamRelease.tag_name -ne 'v1.2.41') { throw '上游最新正式 Release 已变化。' }
$upstreamCommit = gh api repos/BigPizzaV3/CodexPlusPlus/commits/v1.2.41 | ConvertFrom-Json
if ($LASTEXITCODE -ne 0 -or $upstreamCommit.sha -ne '3dafffcafb2566a1e8bce4b35671656d6adb3eda') { throw '上游 v1.2.41 提交已变化。' }

Write-Output 'GITHUB_RULESET_UPSTREAM_VERIFY_OK'
```

预期结果：输出 `GITHUB_RULESET_UPSTREAM_VERIFY_OK`。如果上游最新正式 Release 变化，停止 Gate 1 收口并重新确认冻结基线，不能静默更新文档。

## 标签与模板元数据核验

```powershell
$requiredLabels = @(
  'type:upstream-watch',
  'type:upstream-sync',
  'type:feature-parity',
  'type:parity-exception',
  'type:performance',
  'type:architecture',
  'type:release',
  'type:bug',
  'gate:1',
  'gate:2',
  'gate:3',
  'gate:4',
  'gate:5',
  'gate:6',
  'platform:windows',
  'platform:macos',
  'status:needs-owner-decision',
  'status:blocked'
)

$defaultLabels = @(
  'bug',
  'documentation',
  'duplicate',
  'enhancement',
  'good first issue',
  'help wanted',
  'invalid',
  'question',
  'wontfix'
)

$labels = @(gh label list --repo nonononull/inputcodex --limit 100 --json name | ConvertFrom-Json | ForEach-Object { $_.name })
if ($LASTEXITCODE -ne 0) { throw '读取标签失败。' }

foreach ($label in $requiredLabels + $defaultLabels) {
  if ($label -notin $labels) { throw "缺少 GitHub 标签：$label" }
}

Write-Output 'GITHUB_LABELS_VERIFY_OK'
```

预期结果：输出 `GITHUB_LABELS_VERIFY_OK`；默认标签必须保留。

## PR #5 旧分支清理核验

```powershell
$branchName = 'codex/issue-4-gate-1-closeout'

gh api "repos/nonononull/inputcodex/git/ref/heads/$branchName" --silent 2>$null
$remoteExit = $LASTEXITCODE
if ($remoteExit -eq 0) { throw 'PR #5 远端旧分支仍存在。' }
if ($remoteExit -ne 1) { throw "远端旧分支查询退出码异常：$remoteExit" }

$localRefs = @(
  git for-each-ref --format='%(refname)' "refs/heads/$branchName" "refs/remotes/origin/$branchName"
)
if ($LASTEXITCODE -ne 0) { throw '读取本地旧分支引用失败。' }
if ($localRefs.Count -ne 0) { throw 'PR #5 本地或跟踪旧分支仍存在。' }

Write-Output 'PR5_BRANCH_CLEANUP_VERIFY_OK'
```

预期结果：输出 `PR5_BRANCH_CLEANUP_VERIFY_OK`。

## Git 快照与提交前验证

每个写入批次、Fresh 重验证前、暂存前和交接前执行：

```powershell
$branch = git branch --show-current
if ($LASTEXITCODE -ne 0 -or $branch -ne 'codex/issue-6-gate-1-finalization') {
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
if ($LASTEXITCODE -ne 0) { throw '读取暂存状态失败。' }
```

暂存内容只能覆盖 Issue `#6` 批准文件。

## Issue #6 PR 创建后复核

```powershell
$pullRequests = @(
  gh pr list `
    --repo nonononull/inputcodex `
    --head codex/issue-6-gate-1-finalization `
    --state open `
    --json number,state,isDraft,headRefOid,body,url |
  ConvertFrom-Json
)
if ($LASTEXITCODE -ne 0) { throw '读取 Issue #6 PR 失败。' }

if ($pullRequests.Count -ne 1 -or $pullRequests[0].state -ne 'OPEN' -or $pullRequests[0].isDraft) {
  throw 'Issue #6 PR 不唯一、不是 OPEN 或仍为 Draft。'
}
if ($pullRequests[0].body -notmatch 'Closes\s+#6') {
  throw 'Issue #6 PR 正文缺少 Closes #6。'
}

$localHead = git rev-parse HEAD
if ($LASTEXITCODE -ne 0) { throw '读取本地 HEAD 失败。' }
$remoteHead = git rev-parse refs/remotes/origin/codex/issue-6-gate-1-finalization
if ($LASTEXITCODE -ne 0) { throw '读取远端跟踪 HEAD 失败。' }
if ($localHead -ne $remoteHead -or $localHead -ne $pullRequests[0].headRefOid) {
  throw '本地、远端跟踪与 PR Head 不一致。'
}

$reviewQuery = 'query($owner:String!,$name:String!,$number:Int!){repository(owner:$owner,name:$name){pullRequest(number:$number){reviewThreads(first:100){nodes{isResolved}} autoMergeRequest{enabledAt}}}}'
$review = gh api graphql -f query=$reviewQuery -F owner=nonononull -F name=inputcodex -F number=$pullRequests[0].number | ConvertFrom-Json
if ($LASTEXITCODE -ne 0) { throw '读取当前 PR Review 对话失败。' }
$unresolved = @($review.data.repository.pullRequest.reviewThreads.nodes | Where-Object { -not $_.isResolved })
if ($unresolved.Count -ne 0) { throw "当前 PR 仍有 $($unresolved.Count) 个未解决 Review 对话。" }
if ($null -ne $review.data.repository.pullRequest.autoMergeRequest) { throw '当前 PR 不得启用自动合并。' }

$issue1 = gh issue view 1 --repo nonononull/inputcodex --json state,comments | ConvertFrom-Json
if ($LASTEXITCODE -ne 0 -or $issue1.state -ne 'OPEN') { throw 'Issue #1 应保持 OPEN。' }
$prUrl = $pullRequests[0].url
$trackingComments = @($issue1.comments | Where-Object { $_.body -like "*$prUrl*" })
if ($trackingComments.Count -eq 0) { throw 'Issue #1 尚未回写当前 PR URL。' }

Write-Output 'ISSUE6_PR_HANDOFF_VERIFY_OK'
```

预期结果：输出 `ISSUE6_PR_HANDOFF_VERIFY_OK`；本命令不执行合并或关闭 Issue `#1`。

## 外部 AGOS 使用边界

AGOS 不属于本项目环境要求或合并门禁。本任务已按项目所有者决策直接使用项目原生控制面，不运行 AGOS，也不修改、修复或优化其 Registry、脚本、规则、Workflow 或 Vault。

## 后续维护规则

- 加入首个可构建 Rust Workspace 时，在同一 PR 补齐 Rust 版本、系统依赖、开发构建、发布构建、测试、基准和产物位置。
- 新增可独立构建的子项目时，在子项目根目录新增独立 `build.md` 和 `err.md`。
- 构建或验证失败先查阅并更新 `err.md`。
- 验证命令必须能在 Windows 与 macOS 对应环境复现；平台专用命令需明确标注。
