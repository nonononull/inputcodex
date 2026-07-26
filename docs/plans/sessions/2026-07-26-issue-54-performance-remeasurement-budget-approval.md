# Session Plan：Issue #54 五次性能复测与数值批准

```yaml
task_id: issue-54-performance-remeasurement-budget-approval
tracking_issue_ref: https://github.com/nonononull/inputcodex/issues/54
branch: codex/issue-54-performance-remeasurement-budget-approval
base_commit: 325bb2419548bc076502065dc583f54f4fddd582
approved_decision_ref: user-message:按你推荐来-不用我二次批准-直接安排后续-2026-07-26
owner_scope_approval_ref: https://github.com/nonononull/inputcodex/issues/54
scope_hash: sha256:32c818aaf99efe550e9afc45d5871f9dceeef50ba314aaa91b617cd76158a38e
allowed_operations: native-control-documents, eight-slot-serial-hosted-measure, success-artifact-cache, offline-sample-classification, deterministic-budget-generation-and-validation, local-light-validation, normal-commit-push-pr-review-ci
mutation_intent: collect-five-independent-comparable-hosted-samples-per-platform-and-record-preauthorized-warning-blocking-values-without-budget-ci
executor_enforcement: exact-twenty-nine-path-set, eight-run-cap, no-local-real-measurement, github-hosted-serial-measure-only, no-workflow-or-sampler-change, no-budget-ci, normal-push-only, squash-merge-only, no-force-push, review-root-cause-closure-required
```

## 一、批准状态

- Issue `#54` 已记录项目所有者的数值预授权：当 ADR `0004` 的五次独立可比 Run、完整分类和统计证据满足时，执行者可以直接形成并落盘 warning/blocking 数值。
- 预授权不允许修改 Workflow、采集器、基线结果 schema、配置、预算 CI、Ruleset、性能优化、上游、Release 或 Gate 5；也不允许把数值加入 required checks。
- 最终合并前仍必须把本 Session 的精确 Head、数值来源、PR CI、Review 根因闭环和 Squash 决策证据写入 GitHub；不使用 Force Push 或删除 `main`。

## 二、Fresh 事实

- `main` 已在项目所有者本机时间 `2026-07-26 08:07:02 +08:00` 同步到 `325bb2419548bc076502065dc583f54f4fddd582`；Issue `#54` 工作树以纯 fast-forward 同步到同一提交。
- Issue `#55` / PR `#56` 已完成显式 `measure` 入口并以 Squash 提交 `325bb2419548bc076502065dc583f54f4fddd582` 进入 `main`；合并后 `CI` 与 `Performance Baseline` 均全绿。
- Issue `#32` 的当前 Evidence 仍完整且只作种子审计参照；新的 Issue `#54` 样本必须来自本分支显式 `workflow_dispatch mode=measure` 的不同 `run_id`，不能改写 Issue `#32` 三份 Evidence。
- `release_audit=current`，预算阶段仍为 `baseline-only`；本 Session 只产生数值控制面和原始样本，预算 CI 的 `approved-observation` 实施另行立项。

## 三、知识与外部治理边界

- 已读取项目 `AGENTS.md`、`README.md`、`build.md`、`err.md`、ADR `0004`、Issue `#50` Discovery、Issue `#52` Closeout、Issue `#55` 报告、当前结果 schema 和 GitHub Issue `#54`。
- `local_knowledge_lookup` 未暴露为当前会话工具；项目内 ADR、计划、报告、脚本、结果与 GitHub Issue 已作为本地知识证据。
- AGOS 在本项目是可选外部辅助；已有 `needs-input/unregistered` 结论且项目规则要求立即绕过。此 Session 不调用、修改或优化 AGOS 的 Registry、规则、Workflow、Vault 或脚本。

## 四、候选批准写集合

```text
AGENTS.md
benchmarks/budgets/issue-54-approved-observation.json
benchmarks/results/issue-54/manifest.json
benchmarks/results/issue-54/runs/run-01/macos.json
benchmarks/results/issue-54/runs/run-01/windows.json
benchmarks/results/issue-54/runs/run-02/macos.json
benchmarks/results/issue-54/runs/run-02/windows.json
benchmarks/results/issue-54/runs/run-03/macos.json
benchmarks/results/issue-54/runs/run-03/windows.json
benchmarks/results/issue-54/runs/run-04/macos.json
benchmarks/results/issue-54/runs/run-04/windows.json
benchmarks/results/issue-54/runs/run-05/macos.json
benchmarks/results/issue-54/runs/run-05/windows.json
benchmarks/results/issue-54/runs/run-06/macos.json
benchmarks/results/issue-54/runs/run-06/windows.json
benchmarks/results/issue-54/runs/run-07/macos.json
benchmarks/results/issue-54/runs/run-07/windows.json
benchmarks/results/issue-54/runs/run-08/macos.json
benchmarks/results/issue-54/runs/run-08/windows.json
build.md
docs/plans/2026-07-26-issue-54-performance-remeasurement-budget-approval.md
docs/plans/PROJECT-MASTER-PLAN.md
docs/plans/sessions/2026-07-26-issue-54-performance-remeasurement-budget-approval.md
docs/reports/issue-54-performance-remeasurement-budget-approval.md
docs/workflows/2026-07-26-issue-54-performance-remeasurement-budget-approval-runtime.md
err.md
README.md
scripts/performance/Build-InputcodexBudgetApproval.ps1
scripts/performance/Test-InputcodexBudgetApproval.ps1
```

## 五、执行批次

1. 启动：写入计划、Session Plan、Runtime Workflow 和控制面，重算二十九路径 `scope_hash`，普通提交并推送可采样 Head。
2. 采样：每次只触发一个 hosted `mode=measure` Run；等待四 Job 成功、缓存两个 Artifact、记录 run/attempt/Artifact/哈希/环境；最多八次。
3. 分类：对每个平台独立比较队列键与指纹，保留所有原始样本和 IQR 标记；只有 `comparable-valid` 进入五次统计。
4. 数值：先写失败验证合同，再实现离线构建器，生成带单位、公式、中心、MAD、裕量、来源和预授权引用的数值控制面。
5. 验证与 PR：运行样本/预算验证、Issue `#32` Evidence、CI Contract、Repository Policy、范围审计与 `git diff --check`；普通 push、非 Draft PR、Review/CI、Squash Merge。

## 六、停止条件

- 二十九路径或 `scope_hash` 漂移；第九次 Run、动态新增结果路径或任何未列路径都必须先停止。
- 任一平台在八次独立 Run 后仍少于五个同队列 `comparable-valid` 样本；记录每个无效/漂移/事故根因，但不伪造或删除样本。
- 需要改变性能 Workflow、采集器、验证器、结果 schema、预算 CI、Ruleset、产品代码、上游、Release、优化或 Gate 5。
- hosted Run、Artifact、样本分类、统计、数值验证、PR CI 或 Review 无法用可复核证据闭环。
