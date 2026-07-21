# 构建与验证说明

## 当前状态

截至 2026 年 7 月 21 日，仓库仍不包含应用源码，因此没有 Cargo、Iced 或安装包构建命令。当前可执行工作是 Gate 1 的文档、治理与 `main` Ruleset 验证；任何源码、GitHub Actions 或发布命令都属于后续独立 Issue。

## 环境要求

- Git。
- GitHub CLI `gh`，已登录 `nonononull` 账号。
- Windows PowerShell 5.1 或 PowerShell 7。
- 当前机器的 AI Growth OS 规则根目录：`D:\Android_source\ai-growth-os\components\rules`。

## Issue #2 文档验证

在仓库根目录执行：

```powershell
$rules = 'D:\Android_source\ai-growth-os\components\rules'
$sessionPlan = 'docs\plans\sessions\2026-07-21-issue-2-architecture-governance.md'
$masterPlan = 'docs\plans\PROJECT-MASTER-PLAN.md'

& "$rules\scripts\verify-project-git-foundation.ps1" `
  -ProjectRoot (Get-Location).Path `
  -RequireGit `
  -ReportOnly

& "$rules\scripts\verify-project-entry-doc-foundation.ps1" `
  -ProjectRoot (Get-Location).Path `
  -ReportOnly

& "$rules\scripts\verify-session-plan.ps1" `
  -Path $sessionPlan

& "$rules\scripts\verify-master-plan-index.ps1" `
  -Path $masterPlan

git diff --check
git status --short --branch
```

预期结果：

- Git foundation 与入口文档 foundation 状态为 `ready`。
- Session Plan 输出 `SESSION_PLAN_VERIFY_OK`。
- Master Plan 输出 `MASTER_PLAN_INDEX_VERIFY_OK`。
- `git diff --check` 无输出且退出码为 `0`。
- 当前分支为 `docs/issue-2-architecture-governance`。

## GitHub 与上游基线核验

```powershell
$issue = gh issue view 2 `
  --repo nonononull/inputcodex `
  --json number,title,state,url | ConvertFrom-Json

if ($issue.number -ne 2 -or $issue.state -ne 'OPEN') {
  throw 'GitHub Issue #2 不存在或不再处于 OPEN 状态。'
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

预期结果：命令无异常退出，Issue #2 仍为开放状态，上游最新正式 Release 仍是 `v1.2.41`，标签仍解析到批准提交。

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
      reviewThreads(first:100){nodes{isResolved}}
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

if ($unresolved.Count -ne 0) {
  throw "PR #3 仍有 $($unresolved.Count) 个未解决 Review 对话。"
}
```

预期结果：未解决 Review 对话数量为 `0`；若未来出现对话，必须先完成根因、处理与验证闭环。

## Git 快照检查

写入批次完成、重验证前、暂存前和交接前分别执行：

```powershell
$rules = 'D:\Android_source\ai-growth-os\components\rules'

& "$rules\scripts\verify-git-snapshot-governance.ps1" `
  -ProjectRoot (Get-Location).Path `
  -TaskId '2026-07-21-issue-2-architecture-governance' `
  -WorkflowNode 'verify' `
  -CheckpointReason 'Issue #2 architecture governance checkpoint' `
  -Checkpoint `
  -ReportOnly
```

未提交的关键文档会使检查输出 `blocked`；这表示必须停止扩大范围并完成验证、暂存和命名 Git 快照，不表示可以忽略。

## 提交前验证

暂存本 Issue 的文件后执行：

```powershell
git diff --cached --check
git diff --cached --stat
git status --short --branch
```

预期结果：cached diff 检查退出码为 `0`，暂存内容只覆盖 Issue #2 批准的文档与治理文件。

## Runtime Workflow 校验边界

当前 AI Growth OS 全局 `registry/task-backlog.yml` 和 `registry/business-paths.yml` 尚未登记 `inputcodex` 的本任务与 `architecture-governance` 路径，因此本项目不宣称 `verify-runtime-workflow.ps1` 严格校验通过。若未来需要接入严格模式，必须在 AI Growth OS 仓库另建 Issue/PR 完成跨仓登记，再执行该校验；不得在本项目 PR 中越权修改外部控制面。

## 后续维护规则

- 加入首个可构建 Rust Workspace 时，在同一 PR 补齐 Rust 版本、系统依赖、开发构建、发布构建、测试、基准和产物位置。
- 新增可独立构建的子项目时，在子项目根目录新增独立 `build.md` 和 `err.md`。
- 构建或验证失败先查阅并更新 `err.md`。
- 验证命令必须能在 Windows 与 macOS 对应环境复现；平台专用命令需明确标注。
