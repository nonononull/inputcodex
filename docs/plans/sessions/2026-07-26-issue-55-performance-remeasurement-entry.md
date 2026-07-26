# Session Plan：Issue #55 显式性能复测入口

```yaml
task_id: issue-55-performance-remeasurement-entry
tracking_issue_ref: https://github.com/nonononull/inputcodex/issues/55
branch: codex/issue-55-performance-remeasurement-entry
base_commit: 0678d03981ac0aef2051eb2d3711221ac2a50d29
approved_decision_ref: user-message:按你推荐来-不用我二次批准-直接安排后续-2026-07-26
owner_scope_approval_ref: https://github.com/nonononull/inputcodex/issues/55#issuecomment-5080888679
scope_hash: sha256:372c8c3942d492a9372603f5bc6bbae42ae8013c7603a092c294d24be4edb1be
allowed_operations: workflow-dispatch-mode-entry, ci-contract-test, explicit-hosted-measure, issue-32-evidence-refresh-from-success-artifacts, native-control-documents, local-light-validation, normal-commit-push-pr-review-ci
mutation_intent: add-explicit-nonblocking-manual-remeasurement-entry-and-refresh-hash-bound-evidence; preserve-existing-automatic-evidence-contract
executor_enforcement: exact-fourteen-path-set, no-subagents, local-light-validation, github-hosted-measure-validation-only, normal-push-only, squash-merge-only, no-force-push, review-root-cause-closure-required
```

## 一、批准状态

- Issue `#55` 已记录项目所有者预授权；该预授权仅允许最小 Workflow 复测入口及其静态合同、项目原生控制面与 hosted 验收。
- 本 Session 不批准预算数值、预算 CI、性能优化、Gate 5、结果 schema 或基线配置的修改；仅允许以本 Issue 显式 `measure` Run 的双平台成功 Artifact 刷新 Issue `#32` 的三份哈希绑定 Evidence。
- 最终 Squash Merge 仍需项目所有者对最终 PR Head 的单独明确授权。

## 二、Fresh 事实

- 根工作树 `main` 与 `origin/main` 已在本机时间 `2026-07-26 06:12:23 +08:00` 同步到 `0678d03981ac0aef2051eb2d3711221ac2a50d29`；HTTPS 连续五次、SSH 一次、fetch、fast-forward 与对象连通性均已验证。
- Issue `#54` 是后续五次独立采样与预算数值批准任务；它因当前复测入口缺失而等待本 Issue。
- 当前三份 Issue `#32` 证据完整存在；原 Workflow 因此手工 dispatch 时只能选择 `evidence`。
- Workflow 与 CI 合同变更属于 `implementation_sha256` 输入，旧三份 Evidence 对本 Issue Head 正确报告 `HASH_MISMATCH`；未修改 `main` 的同一 Evidence 命令为 `ok=true`，因此刷新同一实现哈希的 Evidence 是合并前置条件，而不是放宽合同。

## 三、知识与外部治理探测

- `local_knowledge_lookup` 未直接暴露为当前会话工具；AGOS ReportOnly 返回 `LOCAL_KNOWLEDGE_LOOKUP_STATUS=ready`，项目内 ADR、计划、报告、脚本、Workflow 与 GitHub Issue 查询已作为本任务的本地证据输入。
- AGOS 默认入口已以 `ReportOnly` 执行，真实状态为 `AGOS_DEFAULT_ENTRY_STATUS=needs-input`、任务登记 `unregistered`、doctor `blocked`、所有者直接写入 `blocked`；同一输出确认项目 Git 基础、入口文档与 source edit admission 均为 `ready`。按项目规则立即绕过，不创建 task-backlog 记录，不修改 AGOS 的 Registry、规则、Workflow、Vault 或脚本。

## 四、候选批准写集合

```text
.github/workflows/performance-baseline.yml
AGENTS.md
benchmarks/results/issue-32/macos.json
benchmarks/results/issue-32/manifest.json
benchmarks/results/issue-32/windows.json
build.md
docs/plans/2026-07-26-issue-55-performance-remeasurement-entry.md
docs/plans/PROJECT-MASTER-PLAN.md
docs/plans/sessions/2026-07-26-issue-55-performance-remeasurement-entry.md
docs/reports/issue-55-performance-remeasurement-entry.md
docs/workflows/2026-07-26-issue-55-performance-remeasurement-entry-runtime.md
err.md
README.md
scripts/ci/Test-CiScripts.ps1
```

## 五、执行批次

1. 启动基线：确认分支、工作树、`main` 基线、Issue 与 scope hash；只读执行 AGOS ReportOnly。
2. RED：通过一次性断言证明现有 Workflow 缺少手工模式输入；不写仓库。
3. 实施：仅修改 Workflow 的手工 mode 分流与 CI 静态合同断言；同步十四路径控制面。
4. GitHub Evidence 刷新：在已推送 Head 上显式 `mode=measure`，只用双平台成功 Artifact 刷新 Issue `#32` 三份结果与 manifest。
5. 本地验证：运行 Evidence、CI Contract、Repository Policy、scope audit、`git diff --check`。
6. GitHub 验收：普通提交、普通 SSH push、非 Draft PR、CI、Review 对话根因闭环；最终等待单独 Squash Merge 授权。

## 六、停止条件

- scope hash 或十四条路径发生变化。
- 需要触及采集器、验证器、结果 schema、基线配置、预算数值、预算 CI、Ruleset、上游、Release、优化或 Gate 5，或修改三条许可路径以外的已入库 Evidence。
- 默认 Evidence 语义、自动触发语义、Artifact 合同或 hosted Runner 合同不能保持。
- 验证、CI、Review 或 hosted Run 失败但根因未闭环。
