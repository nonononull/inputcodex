# 构建与验证说明

## 当前状态

截至 2026 年 7 月 21 日，仓库仍不包含应用源码，因此没有 Cargo、Iced 或安装包构建命令。Issue `#2` / PR `#3` 已完成 Squash Merge；Issue `#4` 对应 PR `#5` 已创建并保持开放。当前可执行工作是 Gate 1 closeout 文档、合并证据与 `main` Ruleset 复核；任何源码、GitHub Actions 或发布命令都属于后续独立 Issue。

## Rust CI 职责边界

- 已批准方案：`docs/plans/2026-07-21-rust-ci-offload-strategy.md`。
- 已批准实施计划：`docs/plans/2026-07-21-rust-ci-offload-implementation-plan.md`；当前只保存未来独立 Issue/PR 顺序，不执行 Gate 2/3。
- 当前仍无 Cargo Workspace，不能虚构 Cargo 命令；Gate 3 创建 Workspace 时必须在同一 PR 补齐准确的本地轻量与云端全量命令。
- 本地默认只执行快速、定向检查；全量 Workspace、Windows/macOS 和发布构建由标准 GitHub-hosted runners 承担。
- Issue `#2` / PR `#3` 未创建 `.github/workflows/` 或 required status checks；Issue `#4` closeout 同样不得创建，也不使用 Larger 或 self-hosted runner。

当前设计一致性检查：

```powershell
$ciPlan = 'docs\plans\2026-07-21-rust-ci-offload-strategy.md'
$ciImplementationPlan = 'docs\plans\2026-07-21-rust-ci-offload-implementation-plan.md'

if (-not (Test-Path $ciPlan)) {
  throw '缺少 Rust CI 云端编译卸载方案。'
}

if (-not (Test-Path $ciImplementationPlan)) {
  throw '缺少 Rust CI 云端卸载实施计划。'
}

$requiredRefs = @(
  'AGENTS.md',
  'docs\plans\2026-07-21-architecture-governance.md',
  'docs\plans\PROJECT-MASTER-PLAN.md',
  'docs\plans\sessions\2026-07-21-issue-2-architecture-governance.md',
  'docs\workflows\2026-07-21-issue-2-architecture-governance-runtime.md'
)

foreach ($path in $requiredRefs) {
  if (-not (Select-String -LiteralPath $path -Pattern 'rust-ci-offload-strategy|本地轻量验证|云端全量' -Quiet)) {
    throw "CI 策略未同步到控制文件：$path"
  }
}

$implementationRefs = @(
  'docs\plans\2026-07-21-rust-ci-offload-strategy.md',
  'docs\plans\2026-07-21-architecture-governance.md',
  'docs\plans\PROJECT-MASTER-PLAN.md',
  'docs\plans\sessions\2026-07-21-issue-2-architecture-governance.md',
  'docs\workflows\2026-07-21-issue-2-architecture-governance-runtime.md'
)

foreach ($path in $implementationRefs) {
  if (-not (Select-String -LiteralPath $path -Pattern 'rust-ci-offload-implementation-plan' -Quiet)) {
    throw "CI 实施计划未同步到控制文件：$path"
  }
}

if (Test-Path '.github\workflows') {
  throw '当前 Gate 1 不允许创建 GitHub Actions Workflow。'
}

if ((Test-Path 'Cargo.toml') -or (Test-Path 'Cargo.lock') -or (Get-ChildItem -Recurse -File -Filter '*.rs')) {
  throw '当前 Gate 1 不允许创建 Cargo Workspace 或 Rust 源码。'
}
```

预期结果：策略、实施计划与全部控制面引用一致，当前仓库不存在 `.github/workflows/`、Cargo Workspace 或 Rust 源码。

## 环境要求

- Git。
- GitHub CLI `gh`，已登录 `nonononull` 账号。
- Windows PowerShell 5.1 或 PowerShell 7。
- AGOS 不是必需环境；仅在本机已有、命令可用且适合当前任务时作为可选外部辅助。

## Issue #4 closeout 文档验证（项目原生）

在仓库根目录执行：

```powershell
$requiredFiles = @(
  'AGENTS.md',
  'README.md',
  'build.md',
  'err.md',
  'docs\plans\PROJECT-MASTER-PLAN.md',
  'docs\plans\2026-07-21-issue-4-gate-1-closeout.md',
  'docs\plans\sessions\2026-07-21-issue-4-gate-1-closeout.md',
  'docs\workflows\2026-07-21-issue-4-gate-1-closeout-runtime.md',
  'docs\reports\issue-2-architecture-governance-closeout.md'
)

foreach ($path in $requiredFiles) {
  if (-not (Test-Path -LiteralPath $path)) {
    throw "缺少项目控制文件：$path"
  }
}

$policyChecks = @{
  'AGENTS.md' = 'AGOS 仅作为可选外部治理辅助'
  'docs\plans\PROJECT-MASTER-PLAN.md' = '不构成本项目门禁'
  'docs\plans\sessions\2026-07-21-issue-4-gate-1-closeout.md' = 'optional-external-assistance'
  'docs\workflows\2026-07-21-issue-4-gate-1-closeout-runtime.md' = 'optional-external'
  'docs\reports\issue-2-architecture-governance-closeout.md' = '可选外部辅助'
}

foreach ($entry in $policyChecks.GetEnumerator()) {
  if (-not (Select-String -LiteralPath $entry.Key -SimpleMatch $entry.Value -Quiet)) {
    throw "AGOS 外部辅助边界未同步：$($entry.Key)"
  }
}

if (Test-Path '.github\workflows') {
  throw '当前 Gate 1 不允许创建 GitHub Actions Workflow。'
}

if ((Test-Path 'Cargo.toml') -or (Test-Path 'Cargo.lock') -or (Get-ChildItem -Recurse -File -Filter '*.rs')) {
  throw '当前 Gate 1 不允许创建 Cargo Workspace 或 Rust 源码。'
}

$branch = git branch --show-current
if ($LASTEXITCODE -ne 0 -or $branch -ne 'codex/issue-4-gate-1-closeout') {
  throw "当前分支不正确：$branch"
}

git diff --check
if ($LASTEXITCODE -ne 0) {
  throw 'git diff --check 失败。'
}

git status --short --branch
```

预期结果：

- 所有项目控制文件存在，AGOS 可选辅助边界在规则、Master Plan、Session Plan、Runtime Workflow 与 closeout 报告中一致。
- 当前仍不存在 `.github/workflows/`、Cargo Workspace 或 Rust 源码。
- `git diff --check` 无输出且退出码为 `0`。
- 当前分支为 `codex/issue-4-gate-1-closeout`。

## 可选外部 AGOS 辅助验证

AGOS 不属于本项目环境要求或合并门禁。只有在本机已存在、命令接口可用且当前任务确实受益时，才可运行只读或 `ReportOnly` 辅助检查。出现路径缺失、未登记、`needs-input`、接口漂移或执行异常时，写入警告并立即绕过；不得因此停止本项目流程，也不得在 `inputcodex` 的 Issue/PR 中修改或优化 AGOS。

示例：

```powershell
$rules = 'D:\Android_source\ai-growth-os\components\rules'
$optionalVerifier = Join-Path $rules 'scripts\verify-session-plan.ps1'

if (Test-Path -LiteralPath $optionalVerifier) {
  try {
    & $optionalVerifier -Path 'docs\plans\sessions\2026-07-21-issue-4-gate-1-closeout.md'
  } catch {
    Write-Warning "AGOS 可选辅助验证不可用，已绕过：$($_.Exception.Message)"
  }
} else {
  Write-Host 'AGOS 可选辅助验证不存在，已绕过。'
}
```

无论该可选命令输出成功、`needs-input` 或异常，项目结论都以本文件定义的原生验证、GitHub 事实与项目所有者决策为准。

## GitHub 与上游基线核验

```powershell
$issue2 = gh issue view 2 `
  --repo nonononull/inputcodex `
  --json number,title,state,closedAt,url | ConvertFrom-Json

$issue2ClosedAt = $issue2.closedAt.ToUniversalTime().ToString('yyyy-MM-ddTHH:mm:ssZ')
if ($issue2.number -ne 2 -or $issue2.state -ne 'CLOSED' -or
    $issue2ClosedAt -ne '2026-07-21T13:15:52Z') {
  throw 'GitHub Issue #2 的 CLOSED 状态或关闭时间不符合 closeout 证据。'
}

$issue4 = gh issue view 4 `
  --repo nonononull/inputcodex `
  --json number,title,state,url | ConvertFrom-Json

if ($issue4.number -ne 4 -or $issue4.state -ne 'OPEN') {
  throw 'GitHub Issue #4 不存在或不再处于 OPEN 状态。'
}

$pr3 = gh pr view 3 `
  --repo nonononull/inputcodex `
  --json number,state,isDraft,mergedAt,mergeCommit,headRefOid,statusCheckRollup,url |
  ConvertFrom-Json

$pr3MergedAt = $pr3.mergedAt.ToUniversalTime().ToString('yyyy-MM-ddTHH:mm:ssZ')
if ($pr3.number -ne 3 -or $pr3.state -ne 'MERGED' -or $pr3.isDraft -or
    $pr3MergedAt -ne '2026-07-21T13:15:51Z' -or
    $pr3.mergeCommit.oid -ne '0e11375997ff10fdc0c233b31c8468af2d9a4f44' -or
    $pr3.headRefOid -ne '6b090ba5aa479c714c9e231aa07787724d6a8190' -or
    @($pr3.statusCheckRollup).Count -ne 0) {
  throw 'PR #3 的 MERGED、提交、时间、Head 或 Checks 证据不符合 closeout。'
}

$release = gh api repos/BigPizzaV3/CodexPlusPlus/releases/latest | ConvertFrom-Json
if ($release.tag_name -ne 'v1.2.41' -or $release.draft -or $release.prerelease) {
  throw "上游最新正式 Release 已变化：$($release.tag_name)"
}

$tag = gh api repos/BigPizzaV3/CodexPlusPlus/git/ref/tags/v1.2.41 | ConvertFrom-Json
if ($tag.object.sha -ne '3dafffcafb2566a1e8bce4b35671656d6adb3eda') {
  throw "上游 v1.2.41 标签提交已变化：$($tag.object.sha)"
}
```

预期结果：命令无异常退出；Issue `#2` 为 `CLOSED`、Issue `#4` 为 `OPEN`、PR `#3` 为 `MERGED` 且 Checks 数量为 `0`；上游最新正式 Release 仍是 `v1.2.41`，标签仍解析到批准提交。

## GitHub `main` Ruleset 核验

传统 Branch Protection 接口不是 Ruleset 的状态真源；本项目使用 Ruleset 详情和分支有效规则接口进行核验：

```powershell
$headers = @(
  '-H', 'Accept: application/vnd.github+json',
  '-H', 'X-GitHub-Api-Version: 2026-03-10'
)
$rulesetId = 19395456

$ruleset = gh api @headers `
  "repos/nonononull/inputcodex/rulesets/$rulesetId" |
  ConvertFrom-Json

$effective = @(
  gh api @headers repos/nonononull/inputcodex/rules/branches/main |
  ConvertFrom-Json
)

$pullRequestRule = @(
  $ruleset.rules | Where-Object { $_.type -eq 'pull_request' }
)

if ($ruleset.enforcement -ne 'active') {
  throw "Ruleset 未激活：$($ruleset.enforcement)"
}

if (@($ruleset.conditions.ref_name.include).Count -ne 1 -or
    @($ruleset.conditions.ref_name.include)[0] -ne 'refs/heads/main' -or
    @($ruleset.conditions.ref_name.exclude).Count -ne 0) {
  throw 'Ruleset 范围不是仅 main。'
}

if (@($ruleset.bypass_actors).Count -ne 0) {
  throw 'Ruleset 存在未经批准的 bypass actor。'
}

if (@($ruleset.rules | Where-Object type -eq 'deletion').Count -ne 1 -or
    @($ruleset.rules | Where-Object type -eq 'non_fast_forward').Count -ne 1 -or
    $pullRequestRule.Count -ne 1) {
  throw 'Ruleset 缺少删除、Force Push 或 PR 门禁规则。'
}

$parameters = $pullRequestRule[0].parameters
if ($parameters.required_approving_review_count -ne 0 -or
    -not $parameters.required_review_thread_resolution -or
    @($parameters.allowed_merge_methods).Count -ne 1 -or
    @($parameters.allowed_merge_methods)[0] -ne 'squash') {
  throw 'Ruleset 的审批、Review 对话或合并方式参数不符合批准决策。'
}

if (@($effective | Where-Object ruleset_id -eq $rulesetId).Count -ne 3) {
  throw 'main 的三个有效规则未全部来自目标 Ruleset。'
}
```

预期结果：命令无异常退出；Ruleset `19395456` 为 `active`，只命中 `main`、无 bypass actor，并有效提供删除保护、Force Push 保护和 PR 门禁。

核验当前 PR 是否仍有未解决 Review 对话：

```powershell
$query = @'
query($owner:String!,$name:String!,$number:Int!){
  repository(owner:$owner,name:$name){
    pullRequest(number:$number){
      reviewThreads(first:100){totalCount nodes{isResolved}}
    }
  }
}
'@

$response = gh api graphql `
  -f query=$query `
  -F owner='nonononull' `
  -F name='inputcodex' `
  -F number=3 |
  ConvertFrom-Json

$unresolved = @(
  $response.data.repository.pullRequest.reviewThreads.nodes |
  Where-Object { -not $_.isResolved }
)

if ($response.data.repository.pullRequest.reviewThreads.totalCount -ne 0 -or
    $unresolved.Count -ne 0) {
  throw "PR #3 Review 对话总数或未解决数量不为 0。"
}
```

预期结果：Review 对话总数与未解决数量均为 `0`；若未来出现对话，必须先完成根因、处理与验证闭环。

## Squash 与分支清理核验

```powershell
$mergeSha = '0e11375997ff10fdc0c233b31c8468af2d9a4f44'
$prHeadSha = '6b090ba5aa479c714c9e231aa07787724d6a8190'

$parents = @(git rev-list --parents -n 1 $mergeSha)
if ($LASTEXITCODE -ne 0) {
  throw '读取 merge commit 父节点失败。'
}

$parts = $parents[0] -split ' '
if ($parts.Count -ne 2 -or $parts[0] -ne $mergeSha -or
    $parts[1] -ne '09564740b8d00a4b09630c024607cc5292d0381f') {
  throw 'merge commit 不是预期的单父 Squash 结果。'
}

$mergeTree = git show -s --format='%T' $mergeSha
if ($LASTEXITCODE -ne 0) {
  throw '读取 merge tree 失败。'
}

$prHeadTree = git show -s --format='%T' $prHeadSha
if ($LASTEXITCODE -ne 0) {
  throw '读取 PR Head tree 失败。'
}

if ($mergeTree -ne '0730422eb3fa738fe2d05a51e5191832fbfec0fe' -or
    $mergeTree -ne $prHeadTree) {
  throw 'merge tree 与 PR Head tree 不一致。'
}

gh api repos/nonononull/inputcodex/branches/docs%2Fissue-2-architecture-governance --silent 2>$null
$remoteBranchExit = $LASTEXITCODE
if ($remoteBranchExit -eq 0) {
  throw '远端旧功能分支仍存在。'
}
if ($remoteBranchExit -ne 1) {
  throw "远端旧分支查询出现非预期退出码：$remoteBranchExit"
}

$localBranch = git for-each-ref --format='%(refname)' refs/heads/docs/issue-2-architecture-governance
if ($LASTEXITCODE -ne 0) {
  throw '读取本地分支引用失败。'
}
if (-not [string]::IsNullOrWhiteSpace(($localBranch -join ''))) {
  throw '本地旧功能分支仍存在。'
}
```

预期结果：合并提交只有一个父节点，merge tree 与 PR Head tree 都是 `0730422eb3fa738fe2d05a51e5191832fbfec0fe`，远端与本地旧分支均不存在。

## Git 快照检查（项目原生）

写入批次完成、重验证前、暂存前和交接前分别执行：

```powershell
$branch = git branch --show-current
if ($LASTEXITCODE -ne 0) {
  throw '读取当前分支失败。'
}

$head = git rev-parse HEAD
if ($LASTEXITCODE -ne 0) {
  throw '读取当前 HEAD 失败。'
}

git status --short --branch
if ($LASTEXITCODE -ne 0) {
  throw '读取 Git 状态失败。'
}

git diff --check
if ($LASTEXITCODE -ne 0) {
  throw 'git diff --check 失败。'
}

git diff --stat
if ($LASTEXITCODE -ne 0) {
  throw '读取 Git diff 统计失败。'
}

if ($branch -ne 'codex/issue-4-gate-1-closeout') {
  throw "当前分支不正确：$branch"
}

Write-Output "GIT_SNAPSHOT_BRANCH=$branch"
Write-Output "GIT_SNAPSHOT_HEAD=$head"
```

输出必须能明确识别当前分支、HEAD、未提交文件和 diff；发现无关文件时停止扩大范围并先完成清理或拆分。AGOS Git snapshot 命令如可用可以作为补充，但其缺失或失败不影响本项目原生检查。

## 提交前验证

暂存本 Issue 的文件后执行：

```powershell
git diff --cached --check
git diff --cached --stat
git status --short --branch
```

预期结果：cached diff 检查退出码为 `0`，暂存内容只覆盖 Issue `#4` 批准的 closeout Markdown 文件。

## Issue #4 PR 创建后复核

```powershell
$closeoutPr = @(
  gh pr list `
    --repo nonononull/inputcodex `
    --head codex/issue-4-gate-1-closeout `
    --state open `
    --json number,state,isDraft,mergeStateStatus,url |
  ConvertFrom-Json
)

if ($closeoutPr.Count -ne 1 -or
    $closeoutPr[0].state -ne 'OPEN' -or
    $closeoutPr[0].isDraft) {
  throw 'Issue #4 closeout PR 不唯一、不是 OPEN 或仍为 Draft。'
}

$query = @'
query($owner:String!,$name:String!,$number:Int!){
  repository(owner:$owner,name:$name){
    pullRequest(number:$number){
      reviewThreads(first:100){nodes{isResolved}}
    }
  }
}
'@

$response = gh api graphql `
  -f query=$query `
  -F owner='nonononull' `
  -F name='inputcodex' `
  -F number=$closeoutPr[0].number |
  ConvertFrom-Json

$unresolved = @(
  $response.data.repository.pullRequest.reviewThreads.nodes |
  Where-Object { -not $_.isResolved }
)

if ($unresolved.Count -ne 0) {
  throw "Issue #4 closeout PR 仍有 $($unresolved.Count) 个未解决 Review 对话。"
}
```

预期结果：存在且仅存在一个来自当前分支的开放非 Draft PR，未解决 Review 对话为 `0`；本命令不执行合并。

## 外部 AGOS 使用边界

AGOS 可用且适用时可以提供只读辅助验证；不可用、未登记、返回 `needs-input` 或异常时必须记录并绕过，不构成 Runtime Workflow、PR 或合并阻塞。若发现 AGOS 本身需要修改，只记录外部缺口并停止该外部动作；不得在本项目 PR 中修改、修复或优化其 Registry、脚本、规则、Workflow 或 Vault。

## 后续维护规则

- 加入首个可构建 Rust Workspace 时，在同一 PR 补齐 Rust 版本、系统依赖、开发构建、发布构建、测试、基准和产物位置。
- 新增可独立构建的子项目时，在子项目根目录新增独立 `build.md` 和 `err.md`。
- 构建或验证失败先查阅并更新 `err.md`。
- 验证命令必须能在 Windows 与 macOS 对应环境复现；平台专用命令需明确标注。
