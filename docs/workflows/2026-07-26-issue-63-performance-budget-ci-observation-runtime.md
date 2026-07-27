# Issue #63：预算 CI Observation Runtime Workflow

```yaml
schema_version: inputcodex.runtime-workflow.v1
task_id: issue-63
tracking_issue_ref: https://github.com/nonononull/inputcodex/issues/63
baseline_ref: 15e91708b41548f523e26ede4c7ca4de41badf77
branch: codex/issue-63-performance-budget-ci-observation
scope_hash: sha256:d5eb57c1b93dc2b7acc47ba78c8f514af2a2c98e8661df389774713a7b47d8dc
scope_status: approved
scope_approval_ref: https://github.com/nonononull/inputcodex/issues/63#issuecomment-5087488361
local_time_source: Windows Get-Date
pr_ref: pending
ci_ref: pending
merge_ref: pending-separate-owner-authorization
```

## 节点

1. `startup-baseline`：核对 Issue、批准评论、基线、十三路径与 Git 快照。
2. `control-plane`：落盘 Plan、Session Plan、Runtime Workflow 与报告；AGOS 仅 ReportOnly，失败即绕过。
3. `red-observer`：创建观察器测试，确认实现缺失时真实失败。
4. `green-contract`：实现预算/结果/平台/单位/指标/样本合同/checksum 验证。
5. `green-classification`：实现四种非阻断分类和结构化 JSON。
6. `workflow-observation`：自动事件选择 observation，Windows/macOS 测量后观察，成功不上传 Artifact。
7. `ci-contract`：更新 `Test-CiScripts.ps1` 固定 mode、Runner、权限、超时、Action SHA 和 Artifact 边界。
8. `control-plane-sync`：更新 AGENTS、README、build、err、Master Plan 与本报告。
9. `verify`：运行新测试、CI 合同、仓库政策、范围哈希和空白检查。
10. `delivery`：普通提交/推送，创建非 Draft PR；不得 `Closes #63`，Issue 等待 current 主干 observation。
11. `hosted-observation`：PR Head Windows/macOS 真实 observation 成功，成功 Artifact 为零。
12. `review-ci`：全部 Review 对话根因闭环，标准 CI 与 Performance Baseline 全绿。
13. `merge-gate`：报告最终 Head，等待项目所有者单独授权 Squash Merge。

## 精确写入范围

```text
.github/workflows/performance-baseline.yml
AGENTS.md
build.md
docs/plans/2026-07-26-issue-63-performance-budget-ci-observation.md
docs/plans/PROJECT-MASTER-PLAN.md
docs/plans/sessions/2026-07-26-issue-63-performance-budget-ci-observation.md
docs/reports/issue-63-performance-budget-ci-observation.md
docs/workflows/2026-07-26-issue-63-performance-budget-ci-observation-runtime.md
err.md
README.md
scripts/ci/Test-CiScripts.ps1
scripts/performance/Invoke-InputcodexBudgetObservation.ps1
scripts/performance/Test-InputcodexBudgetObservation.ps1
```

## 验证命令

```powershell
pwsh -NoProfile -File scripts/performance/Test-InputcodexBudgetObservation.ps1 -RepositoryRoot .
pwsh -NoProfile -File scripts/ci/Test-CiScripts.ps1
pwsh -NoProfile -File scripts/ci/Verify-RepositoryPolicy.ps1 -RepositoryRoot .
git diff --check
```

## 证据

```yaml
evidence:
  scope_hash: sha256:d5eb57c1b93dc2b7acc47ba78c8f514af2a2c98e8661df389774713a7b47d8dc
  red_test: observer implementation missing at Windows local time 2026-07-27 13:21:15 +08:00; expected failure confirmed
  observer_test: pending
  ci_contract: pending
  repository_policy: pending
  pr_ref: pending
  hosted_windows: pending
  hosted_macos: pending
  review_threads: pending
  artifacts: pending
  merge_authorization: pending
```

## 停止门

- 十三路径或 `scope_hash` 漂移。
- 预算 JSON、历史 Evidence、预算数值/公式、Ruleset、required checks、产品、Gate 5、`upstream/` 或 AGOS 需要变化。
- 阈值分类返回失败，或环境不匹配被伪造成可比。
- 失败未确定根因、Review 对话未闭环或 Squash Merge 未单独授权。
