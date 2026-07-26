# Session Plan：Issue #61 性能预算数值 Closeout

schema_version: inputcodex.session-plan.v1
task_id: issue-61-performance-budget-closeout
task_summary: 固化 Issue #59 / PR #60 的正式合并证据，清除待合并状态，并把下一合法工作稳定指向独立预算 CI 观察 Issue。
task_class: Standard
decision_status: approved
execution_status: implementation-complete-pending-pr
tracking_issue_ref: https://github.com/nonononull/inputcodex/issues/61
baseline_ref: e225144831a0928bfa3aaa0d169a054779005812
branch_ref: codex/issue-61-performance-budget-closeout
approved_decision_ref: user-message:批准-Issue-61-八路径范围与-scope_hash-允许实施提交推送PRReviewCI-2026-07-26
approved_scope_ref: https://github.com/nonononull/inputcodex/issues/61#issuecomment-5082819125
scope_hash: sha256:dafe55bfc38c38782558c1577215d227ac8c83b7110735c4ddd58b48d66264b5
allowed_operations: eight-path-doc-write, local-lightweight-validation, ordinary-commit, ordinary-push, pull-request-create, issue-pr-comment, review-ci-evidence-read
mutation_intent: 将性能预算数值已合并事实写成稳定终态，不修改预算、CI、优化或产品实现。
executor_enforcement: 分支、基线、八路径、scope hash、稳定事实、反递归合同、验证或禁止面任一异常即停止。
agos_status: bypassed-report-only-unregistered-needs-input-owner-direct-write-blocked-no-cross-repo-mutation
anti_recursion_contract: 本任务自身的动态 PR Head、CI、Review、授权与合并证据只保留在 GitHub 评论，永久文档不把 Issue #61 标记为待合并活动任务。
task_plan_ref: docs/plans/2026-07-26-issue-61-performance-budget-closeout.md
session_plan_ref: docs/plans/sessions/2026-07-26-issue-61-performance-budget-closeout.md
runtime_workflow_ref: docs/workflows/2026-07-26-issue-61-performance-budget-closeout-runtime.md
report_ref: docs/reports/issue-61-performance-budget-closeout.md
pr_ref: github-comment-only-after-creation
review_ref: github-comment-only
ci_ref: github-comment-only
merge_ref: owner-authorization-required-for-final-head

## local_knowledge_lookup

- `AGENTS.md`、README、`build.md`、`err.md` 与 `docs/plans/PROJECT-MASTER-PLAN.md`。
- Issue `#59` / PR `#60`、Squash 提交和两套合并后主干 Run。
- `docs/plans/2026-07-25-issue-52-performance-budget-closeout.md` 及其 Session、Runtime、报告反递归模板。
- AGOS 默认入口返回 `needs-input/unregistered`；项目 Git 与入口文档基础为 ready，按项目规则记录并绕过，没有修改 AGOS。

## 业务路径

1. Fresh 读取 PR `#60`、Issue `#59`、Squash 提交、PR/主干 CI、Performance Run 与 Artifact 证据。
2. 将来源事实写入稳定 Closeout 报告和长期入口。
3. 清除 Master Plan 的旧 `active_*`、`tracking_issue_ref`、`next_legal_gate` 与 `decision_status`。
4. 使用 `build.md` 的可重复命令验证范围、事实、禁止面和仓库政策。
5. 创建非 Draft PR；动态交付证据只写入 GitHub，不改写本任务文件。

## 精确路径

```text
AGENTS.md
build.md
docs/plans/2026-07-26-issue-61-performance-budget-closeout.md
docs/plans/PROJECT-MASTER-PLAN.md
docs/plans/sessions/2026-07-26-issue-61-performance-budget-closeout.md
docs/reports/issue-61-performance-budget-closeout.md
docs/workflows/2026-07-26-issue-61-performance-budget-closeout-runtime.md
README.md
```

## 稳定事实

- PR `#60` Final Head：`61c088d74d61a329fbe67e14b8280dfa9701c6b2`。
- Squash 提交：`e225144831a0928bfa3aaa0d169a054779005812`。
- tree：`56eb1e8d95dfce22726c1aef1bdde1c353af055e`。
- 提交结构与签名：单父、GitHub `valid`。
- PR CI / Performance：Run `30194465259` 为 `7/7`、Run `30194465231` 为 `4/4`，Artifact 均为 `0`。
- 主干 CI / Performance：Run `30194897171` 为 `7/7`、Run `30194897166` 为 `4/4`，Artifact 均为 `0`。
- Issue `#59`：`CLOSED / COMPLETED`。
- Windows/macOS 同队列样本为 `5/12`，预算 JSON 哈希固定，预算 CI 与 Gate 5 继续锁定。

## 批次

### Batch 1：control-plane

- 创建四份任务控制面。
- 冻结八路径与 scope hash。
- 记录 AGOS ReportOnly 状态；任何 AGOS 改动都禁止进入本任务。

### Batch 2：stable-state

- 更新 `AGENTS.md` 与 README 的长期项目状态。
- 更新 `build.md` 当前状态和 Issue `#61` 轻量验证入口。
- 将 Master Plan 转为“等待独立预算 CI 观察 Issue”的稳定终态。

### Batch 3：verification-and-pr

- 运行八路径、scope hash、旧状态、占位符、Evidence、预算复算、CI 合同、仓库政策和空白检查。
- 普通提交、普通推送并创建非 Draft PR。
- Fresh 核对 Review 对话和 GitHub-hosted CI，最终合并等待项目所有者单独授权。

## 禁止面

- Cargo、Rust、产品代码、测试实现、`benchmarks/`、Workflow、Ruleset、Release、`upstream/` 和 AGOS 零差异。
- 不修改预算数值，不运行新测量，不实施优化，不启动 Gate 5。
- 不 Force Push，不删除任何分支或工作树。
- 不把 Issue `#61` 的动态状态写入长期控制面。

## 完成定义

- 八路径与 scope hash 通过。
- 稳定事实与反递归合同通过。
- 本地轻量验证通过。
- 非 Draft PR 已创建并完成适用 Review/CI。
- 最终 Squash Merge 未经项目所有者针对最终 Head 明确授权不得执行。
