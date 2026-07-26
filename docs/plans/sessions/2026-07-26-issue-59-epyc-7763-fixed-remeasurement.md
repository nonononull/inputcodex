# Issue #59 Session Plan：EPYC 7763 四次固定串行复测

schema_version: inputcodex.session-plan.v1
task_id: issue-59-epyc-7763-fixed-remeasurement
tracking_issue_ref: https://github.com/nonononull/inputcodex/issues/59
decision_status: approved
approved_decision_ref: user-message:批准方案-A-EPYC-7763-四次固定串行复测-2026-07-26
selected_business_path: performance-budget-fixed-remeasurement
project_root: C:\Users\dashuai\Documents\inputcodex-worktrees\issue-59-epyc-7763-fixed-remeasurement
baseline_ref: d9d1ed77b9796ac6a99e250d1547217a39426aa9
branch_ref: codex/issue-59-epyc-7763-fixed-remeasurement
plan_ref: docs/plans/2026-07-26-issue-59-epyc-7763-fixed-remeasurement.md
runtime_workflow_ref: docs/workflows/2026-07-26-issue-59-epyc-7763-fixed-remeasurement-runtime.md
report_ref: docs/reports/issue-59-epyc-7763-fixed-remeasurement.md
scope_hash: sha256:d0577e546d2209d10373eccdf335bbcf3cd4caad7906163838c88b461da0b570
allowed_operations: import-immutable-issue-54-evidence, create-four-issue-59-slots, github-hosted-measurement, artifact-download-and-delete, offline-budget-derivation-if-qualified, local-light-validation, normal-commit-push, non-draft-pr, review-ci
mutation_intent: 缓存旧复测证据并执行四次新的固定串行双平台测量；达到五份目标队列样本时生成可复算预算数值，否则硬停止。
executor_enforcement: exact-thirty-eight-path-set, four-slots-only, target-windows-processor-exact-match, historical-labels-immutable, github-hosted-only, no-budget-ci, no-product-source, no-agos-mutation, no-force-push, no-main-delete, squash-only, review-root-cause-closure-required
time_source: Windows Get-Date for local operations; GitHub service timestamps remain separate evidence
agos_status: bypassed-report-only-unregistered-needs-input-no-cross-repo-mutation

## local_knowledge_lookup

- `AGENTS.md`、`README.md`、`build.md`、`err.md` 与 `docs/plans/PROJECT-MASTER-PLAN.md`。
- `docs/adr/0004-performance-budget-policy.md`。
- `docs/reports/issue-57-hosted-queue-heterogeneity-discovery.md`。
- Issue `#54` worktree 的 `benchmarks/results/issue-54/manifest.json`、十六份原始结果与停止报告。
- AGOS 默认入口在本任务未登记时返回 `needs-input`；依项目规则记录后绕过，不修改 AGOS Registry、Workflow、Rules 或 Vault。

## 固定参数

- `target_windows_processor=AMD EPYC 7763 64-Core Processor`
- `max_serial_runs=4`
- `required_target_cohort_runs=5`
- `fixed_slots=run-01,run-02,run-03,run-04`
- `historical_target_slots=issue-54/run-03,issue-54/run-04,issue-54/run-05,issue-54/run-08`
- `hard_stop_after_run_04=true`
- `budget_ci_enabled=false`

## 执行检查点

1. **startup-baseline**：确认分支、干净工作树、本机时间、Issue、主干提交和 Git 远端；运行 Git snapshot ReportOnly。
2. **control-plane**：写入三十八路径、scope hash、历史证据和四槽位合同；轻量验证后形成第一条普通提交并 push。
3. **run-01 至 run-04**：每个槽位执行、等待、下载、核验、分类、入库、轻量验证、普通提交和 push；任一槽位未终态不得触发下一槽位。
4. **final-decision**：四次后计算 AMD EPYC 7763 总数；达标生成预算控制面，不达标记录硬停止。
5. **pr-closeout**：完成 Fresh 轻量验证、非 Draft PR、Review/CI 与全部对话闭环；最终等待项目所有者针对 Final Head 的 Squash Merge 授权。

## 停止条件

- 需要创建 `run-05`、恢复 Issue `#54` 或创建其 `run-09`。
- 需要降低 CPU 队列语义、修改 ADR/Workflow、使用收费/Larger/self-hosted Runner。
- 历史文件与 Issue `#54` manifest 哈希不一致，或任何新 Artifact 无法证明 source 与 Run 关系。
- 实际路径超出三十八路径或 scope hash 漂移。
- Review、CI、Git 远端或根因闭环无法完成。
