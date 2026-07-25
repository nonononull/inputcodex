# Issue #55 显式性能复测入口 Runtime Workflow

```yaml
task_id: issue-55-performance-remeasurement-entry
scope_hash: sha256:89a9a40c76e98d573d4f55ca7d0aa140f325c9eb908e908f9e8731c55aaf03df
execution_mode: single-executor, no-subagents, local-light-validation, github-hosted-manual-measure-validation
mutation_intent: explicit-workflow-dispatch-mode-without-budget-ci-or-performance-implementation-change
agos_status: bypassed-report-only-unregistered-needs-input-owner-scope-required-no-cross-repo-mutation
```

## 1. 启动基线

1. 读取 `AGENTS.md`、`README.md`、`build.md`、`err.md`、主计划、Issue `#55` 和本 Session Plan。
2. 以项目所有者 Windows 本机 `Get-Date` 记录 Git checkpoint；不得设置 `GIT_AUTHOR_DATE` 或 `GIT_COMMITTER_DATE`。
3. 确认工作树位于 `codex/issue-55-performance-remeasurement-entry`，基线为 `0678d03981ac0aef2051eb2d3711221ac2a50d29`，且无未提交差异。

## 2. 合同 RED

运行只读断言，证明当前 Workflow 尚未包含 `workflow_dispatch.inputs.mode`：

```powershell
$workflow = Get-Content -LiteralPath .github/workflows/performance-baseline.yml -Raw -Encoding utf8
if ($workflow -match '(?ms)^  workflow_dispatch:\r?\n    inputs:\r?\n      mode:') {
  throw 'RED 前提失效：当前 Workflow 已存在显式复测输入。'
}
Write-Output 'REMEASUREMENT_ENTRY_RED_CONFIRMED'
```

预期输出：`REMEASUREMENT_ENTRY_RED_CONFIRMED`。若不是，先调查当前 `main` 已有变更，禁止覆盖或重复实现。

## 3. AGOS 可选入口

使用以下只读命令；其结果只能作为补充证据：

```powershell
pwsh -NoProfile -File D:\Android_source\ai-growth-os\components\rules\scripts\invoke-agos-default-entry.ps1 `
  -Root D:\Android_source\ai-growth-os `
  -ProjectRoot C:\Users\dashuai\Documents\inputcodex-worktrees\issue-55-performance-remeasurement-entry `
  -TaskId issue-55-performance-remeasurement-entry `
  -SelectedBusinessPath performance-remeasurement-entry `
  -SessionPlanRef docs/plans/sessions/2026-07-26-issue-55-performance-remeasurement-entry.md `
  -ApprovedDecisionRef user-message:按你推荐来-不用我二次批准-直接安排后续-2026-07-26 `
  -ErrSearchQuery 'performance workflow dispatch remeasurement evidence mode' `
  -ReportOnly
```

真实输出为：`AGOS_DEFAULT_ENTRY_STATUS=needs-input`、`DEFAULT_ENTRY_TASK_REGISTRATION_STATUS=unregistered`、`DEFAULT_ENTRY_DOCTOR_STATUS=blocked`、`DEFAULT_ENTRY_OWNER_DIRECT_WRITE_ADMISSION_STATUS=blocked`；同时 `DEFAULT_ENTRY_PROJECT_GIT_FOUNDATION_STATUS=ready`、`DEFAULT_ENTRY_PROJECT_ENTRY_DOC_FOUNDATION_STATUS=ready`、`DEFAULT_ENTRY_GIT_SOURCE_EDIT_ADMISSION_STATUS=ready` 与 `LOCAL_KNOWLEDGE_LOOKUP_STATUS=ready`。按项目规则记录后绕过；不创建 task-backlog 记录，不修改任何 AGOS 文件。

## 4. 最小实现

1. 在 `workflow_dispatch` 中声明必填 choice `mode`，默认 `evidence`，选项仅 `evidence` 和 `measure`。
2. 在 contract Job 先区分事件：
   - 手工 `measure` 无条件选择既有测量路径；
   - 手工 `evidence` 要求完整的三份入库证据；
   - PR/push 保持原有文件存在性驱动模式。
3. 在 `Test-CiScripts.ps1` 中固定输入定义与事件分流的静态合同，不改变已有权限、并发、Runner、Action SHA、Artifact 保留或 `target/` 禁止规则。
4. 同步十一路径项目原生文档；记录根因、禁止面、验证和 Issue `#54` 的消费关系。

## 5. 本地验证门

```powershell
pwsh -NoProfile -File scripts/performance/Test-InputcodexBaseline.ps1 -RepositoryRoot . -Mode Evidence
pwsh -NoProfile -File scripts/ci/Test-CiScripts.ps1
pwsh -NoProfile -File scripts/ci/Verify-RepositoryPolicy.ps1 -RepositoryRoot .
git diff --check
```

范围审计必须同时纳入已提交、未暂存、已暂存与未跟踪路径：

```powershell
$baseline = '0678d03981ac0aef2051eb2d3711221ac2a50d29'
$approvedPaths = @(
  '.github/workflows/performance-baseline.yml'
  'AGENTS.md'
  'build.md'
  'docs/plans/2026-07-26-issue-55-performance-remeasurement-entry.md'
  'docs/plans/PROJECT-MASTER-PLAN.md'
  'docs/plans/sessions/2026-07-26-issue-55-performance-remeasurement-entry.md'
  'docs/reports/issue-55-performance-remeasurement-entry.md'
  'docs/workflows/2026-07-26-issue-55-performance-remeasurement-entry-runtime.md'
  'err.md'
  'README.md'
  'scripts/ci/Test-CiScripts.ps1'
) | Sort-Object
$committed = @(git diff --name-only "$baseline...HEAD")
$unstaged = @(git diff --name-only)
$staged = @(git diff --cached --name-only)
$untracked = @(git ls-files --others --exclude-standard)
$actualPaths = @($committed + $unstaged + $staged + $untracked | Where-Object { $_ } | Sort-Object -Unique)
$pathDiff = @(Compare-Object -ReferenceObject $approvedPaths -DifferenceObject $actualPaths)
if ($pathDiff.Count -ne 0) { $pathDiff | Format-Table -AutoSize; throw 'Issue #55 实际差异不是批准的十一条路径。' }
$scopeText = ($approvedPaths -join "`n") + "`n"
$scopeBytes = [System.Text.UTF8Encoding]::new($false).GetBytes($scopeText)
$scopeHash = [Convert]::ToHexString([System.Security.Cryptography.SHA256]::HashData($scopeBytes)).ToLowerInvariant()
if ($scopeHash -ne '89a9a40c76e98d573d4f55ca7d0aa140f325c9eb908e908f9e8731c55aaf03df') { throw "Issue #55 scope_hash 漂移：$scopeHash" }
Write-Output 'ISSUE_55_SCOPE_GREEN'
```

## 6. GitHub 验收与收口

1. 普通提交、普通 SSH push，创建 `Closes #55` 的非 Draft PR；禁止 Force Push、Rebase Merge、Merge Commit 与删除 `main`。
2. PR CI 必须通过 Evidence 路径，并保持 Artifact 为 `0`。
3. 对已推送分支执行一次 `gh workflow run "Performance Baseline" --ref codex/issue-55-performance-remeasurement-entry -f mode=measure`；等待 contract、Windows、macOS、required 全部成功。
4. 核验临时成功 Artifact 仅为 Windows/macOS 结果、保留期为 1 天，失败诊断最长 7 天，且不存在 `target/` 上传。
5. 把 Issue、PR、CI、hosted Run、Review 根因闭环和 Issue `#54` 接续证据写入报告与 GitHub 评论；最终 Merge 仅在项目所有者对最终 Head 单独授权后执行 Squash Merge。

## 7. 停止条件

- 十一路径或 scope hash 漂移。
- 需要修改采集器、验证器、schema、配置、已入库证据、预算数值、预算 CI、Ruleset、上游、Release、优化或 Gate 5。
- 本地或 hosted 验证失败且根因未闭环。
