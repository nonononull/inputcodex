# Issue #50 性能预算 Discovery Runtime Workflow

schema_version: inputcodex.runtime-workflow.v1
workflow_id: inputcodex.issue-50.performance-budget-discovery
tracking_issue_ref: https://github.com/nonononull/inputcodex/issues/50
session_plan_ref: docs/plans/sessions/2026-07-25-issue-50-performance-budget-discovery.md
task_plan_ref: docs/plans/2026-07-25-issue-50-performance-budget-discovery.md
approved_decision_ref: user-message:按A方案开始-2026-07-25
branch_ref: codex/issue-50-performance-budget-discovery
baseline_ref: fd9db9ca1c150b7db34dda8acc09b6f0cc357a17
scope_hash: sha256:af1c248c46d54741f9c77ab3621cd66ccd40e3fa50698d377c788fcb0b93205f
workflow_status: planning-checkpoint-owner-scope-approval-pending
execution_mode: single-executor, no-subagents, documentation-only, local-light-validation
mutation_intent: freeze-budget-method-and-gate-5-preconditions; no-budget-values-or-implementation
agos_status: bypassed-report-only-unregistered-needs-input-no-cross-repo-mutation

## 1. 启动基线

1. 读取 `AGENTS.md`、`README.md`、`build.md`、`err.md`、Master Plan 和 Issue `#32` 性能报告。
2. 确认当前工作树为 `codex/issue-50-performance-budget-discovery`，HEAD 为 `fd9db9ca1c150b7db34dda8acc09b6f0cc357a17`，工作树起点干净。
3. 使用 GitHub API 核对远端 `main`、Issue `#50`、最新正式 Release、Ruleset、维护者数量和现有性能 Run。
4. 读取原始 Windows/macOS JSON、manifest、配置和已知排错记录，不运行上游或半成品。

## 2. 规划检查点

1. 形成任务计划、Session Plan 和本 Runtime Workflow。
2. 固定候选九路径，并按 Windows `Sort-Object`、UTF-8 无 BOM、LF 和末尾 LF 计算 `scope_hash`。
3. 范围批准前只允许三份规划文件存在差异。
4. 将九路径、hash、禁止面和下一批次回写 Issue `#50`，等待项目所有者明确批准。

## 3. AGOS 可选入口

Session Plan 落盘后运行：

```powershell
pwsh -NoProfile -File D:\Android_source\ai-growth-os\components\rules\scripts\invoke-agos-default-entry.ps1 `
  -Root D:\Android_source\ai-growth-os `
  -ProjectRoot . `
  -TaskId issue-50-performance-budget-discovery `
  -SelectedBusinessPath inputcodex.performance-budget-discovery `
  -SessionPlanRef docs/plans/sessions/2026-07-25-issue-50-performance-budget-discovery.md `
  -ApprovedDecisionRef user-message:按A方案开始-2026-07-25 `
  -ReportOnly
```

若返回 `unregistered`、`needs-input`、schema 不兼容或执行异常，记录真实输出后立即按项目原生流程绕过。禁止为通过外部入口而修改 AGOS Registry、规则、Workflow、Vault 或脚本。

本任务实际返回：

```text
DEFAULT_ENTRY_ROUTE_STATUS=needs-input
DEFAULT_ENTRY_TASK_REGISTRATION_STATUS=unregistered
DEFAULT_ENTRY_OWNER_DIRECT_WRITE_ADMISSION_STATUS=blocked
DEFAULT_ENTRY_PROJECT_GIT_FOUNDATION_STATUS=ready
DEFAULT_ENTRY_PROJECT_ENTRY_DOC_FOUNDATION_STATUS=ready
LOCAL_KNOWLEDGE_LOOKUP_STATUS=ready
AGOS_DEFAULT_ENTRY_STATUS=needs-input
```

结论：AGOS 已按 ReportOnly 完成只读尝试并被项目原生绕过；不执行 task-backlog 修复、Registry 登记或任何跨仓写入。

## 4. 范围批准后的 Discovery

1. 创建 ADR `0004`，定义预算对象分类、可比环境、采样统计、异常与失败语义、阶段升级和所有者批准合同。
2. 创建 Issue `#50` 报告，盘点证据并记录方案 A、拒绝方案和后续 Issue 边界。
3. 更新 `AGENTS.md`、README、build 和 Master Plan 的稳定状态。
4. 不修改历史 Issue `#32` 计划、Session Plan、Runtime Workflow、原始报告或样本；这些文件继续保留当时阶段事实。

## 5. Fresh 验证门

```powershell
pwsh -NoProfile -File scripts/performance/Test-InputcodexBaseline.ps1 -RepositoryRoot . -Mode Evidence
pwsh -NoProfile -File scripts/ci/Test-CiScripts.ps1
pwsh -NoProfile -File scripts/ci/Verify-RepositoryPolicy.ps1 -RepositoryRoot .
git diff --check
```

额外验证必须证明：

- 实际差异只包含批准九路径。
- 九路径排序后重算 `scope_hash` 仍为 `sha256:af1c248c46d54741f9c77ab3621cd66ccd40e3fa50698d377c788fcb0b93205f`。
- 根 Cargo、产品、Parity、upstream、benchmarks、Workflow、Ruleset、Release 和 AGOS 零差异。
- ADR 和报告不存在 `TBD`、`TODO`、占位预算或跨平台排名。

范围审计必须分别收集未暂存、已暂存和未跟踪路径后再合并；`git diff --name-only` 不包含未跟踪文件，不能单独作为范围真源：

```powershell
$unstaged = @(git diff --name-only)
$staged = @(git diff --cached --name-only)
$untracked = @(git ls-files --others --exclude-standard)
$actualPaths = @($unstaged + $staged + $untracked | Where-Object { $_ } | Sort-Object -Unique)
```

该写法复用 `err.md` 中既有的 PowerShell 原生命令参数编组与差异并集结论，禁止把多条原生命令直接用逗号写入同一数组表达式。

本地禁止运行完整 Workspace、桌面 Release、真实性能采集、上游或半成品。

## 6. Git 与 PR

1. 使用普通 Git 提交和普通 push，禁止 Force Push。
2. 创建关联 `Closes #50` 的非 Draft PR，自动合并保持关闭。
3. 所有 Review 对话必须记录根因、处理方式和验证证据后解决。
4. 最终 Head 的 Review/CI 全部完成后，仍需项目所有者单独授权 Squash Merge。
5. 未获得删除授权前保留来源分支和工作树。

## 7. 停止条件

- 九路径或 `scope_hash` 未获批准或发生变化。
- 需要新增预算值、修改性能代码/配置/Workflow、运行新 hosted 测量或接入 Ruleset。
- 需要处理上游 Release、版本资产、一致性例外、UI 或 Gate 5 产品功能。
- 外部事实、验证、Review 或 CI 失败且根因未闭环。
