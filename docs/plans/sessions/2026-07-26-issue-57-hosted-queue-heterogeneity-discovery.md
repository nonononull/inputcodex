# Issue #57 Hosted 队列异构性 Discovery Session Plan

## 1. 会话合同

```yaml
task_id: inputcodex-issue-57-hosted-queue-heterogeneity-discovery
tracking_issue_ref: https://github.com/nonononull/inputcodex/issues/57
task_kind: docs-only-hosted-queue-feasibility-discovery
approved_decision_ref: user-message:按你推荐来-不用我二次批准-直接安排后续；Windows 本机记录时间 2026-07-26 10:19:27 +08:00
decision_status: approved-for-discovery-delivery-only-no-substantive-path-selected
scope_approval_status: approved-for-exact-eight-path-discovery-only
scope_hash: sha256:26cc8ba51b7926c0898be56f1cec23c623963b2e944295d14d3d46bf650cd953
mutation_intent: record-hosted-queue-heterogeneity-evidence-and-owner-decision-material
executor_enforcement: exact-path-set-no-performance-run-no-budget-no-adr-or-runner-change-no-force-push
base_ref: main@325bb2419548bc076502065dc583f54f4fddd582
branch_ref: codex/issue-57-hosted-queue-heterogeneity-discovery
```

## 2. 允许操作

1. 只读检查 Issue `#54` 分支、GitHub Issue/Run 元数据、ADR `0004`、`README.md`、`build.md`、`err.md` 与项目总计划。
2. 只写八条批准路径，记录事实、三条路径、推荐理由、停止条件和所有者决策点。
3. 执行本地轻量验证、范围审计、普通提交、普通 push、非 Draft PR、Review、CI 和满足门禁后的 Squash Merge。
4. PR 使用 `Refs #57`，不得使用 `Closes #57`；Discovery 合并不等同于选择后续路径，Issue `#57` 继续保持开放。

## 3. 禁止操作

- 不触发 `Performance Baseline`，不创建 `run-09`，不写预算数值、预算 JSON 或预算 CI。
- 不改 `.github/workflows/`、`scripts/`、`benchmarks/`、Rust/Cargo、产品源码、上游缓存、Ruleset、Release 或 AGOS。
- 不将不同 CPU 的 Windows 样本混为同一可比队列，不删除、重写或伪造 Issue `#54` 结果。
- 不使用 Force Push、Rebase Merge、Merge Commit 或删除 `main`。

## 4. AGOS 与本地知识检查

`local_knowledge_lookup` 没有作为当前会话的直接工具暴露。已执行 AGOS `ReportOnly`：`LOCAL_KNOWLEDGE_LOOKUP_STATUS=ready`，但任务登记为 `unregistered`、总体为 `needs-input`，并阻止 AGOS 直接文档写入。依照项目规则记录后绕过，不创建 task-backlog、不修改 AGOS；项目原生控制面是唯一写入真源。

```powershell
pwsh -NoProfile -File D:\Android_source\ai-growth-os\components\rules\scripts\invoke-agos-default-entry.ps1 `
  -Root D:\Android_source\ai-growth-os `
  -ProjectRoot . `
  -TaskId inputcodex-issue-57-hosted-queue-heterogeneity-discovery `
  -SelectedBusinessPath performance-budget-hosted-queue-discovery `
  -SessionPlanRef docs/plans/sessions/2026-07-26-issue-57-hosted-queue-heterogeneity-discovery.md `
  -ApprovedDecisionRef user-message:按你推荐来-不用我二次批准-直接安排后续 `
  -ReportOnly
```

## 5. 验证门

```powershell
pwsh -NoProfile -File scripts/performance/Test-InputcodexBaseline.ps1 -RepositoryRoot . -Mode Evidence
pwsh -NoProfile -File scripts/ci/Test-CiScripts.ps1
pwsh -NoProfile -File scripts/ci/Verify-RepositoryPolicy.ps1 -RepositoryRoot .
git diff --check
```

范围审计按下列确定性算法执行：以 `main...HEAD`、未暂存、已暂存和未跟踪路径的并集与八路径集合比较；将 Ordinal 排序后的路径以 UTF-8 无 BOM、末尾单一换行符计算 SHA-256，必须等于 `26cc8ba51b7926c0898be56f1cec23c623963b2e944295d14d3d46bf650cd953`。

## 6. 交付与停止

本 Issue 的成功交付是可审计的 Discovery PR，而不是预算批准。所有验证和 Review 通过后，Discovery 文档可以合并；但在项目所有者明确选择后续路径前，不建立新的采样、语义修订或 Runner 工作。
