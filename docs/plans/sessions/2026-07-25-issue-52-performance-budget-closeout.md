# Session Plan：Issue #52 性能预算 Discovery Closeout

task_id: issue-52-performance-budget-closeout
task_summary: 固化 Issue #50 / PR #51 的正式合并证据，清除待合并状态，并把下一合法工作稳定指向独立性能复测与数值批准 Issue。
task_class: Standard
decision_status: approved-closeout-implementation-pr
tracking_issue_ref: https://github.com/nonononull/inputcodex/issues/52
baseline_ref: fea8824c652665df710a7e6ef941854060eb6e1f
branch_ref: codex/issue-52-performance-budget-closeout
approved_decision_ref: user-message:建立独立-closeout-issue-pr-更新-master-plan-2026-07-25
approved_scope_ref: https://github.com/nonononull/inputcodex/issues/52#issuecomment-5080685026
scope_hash: sha256:af1cfffe1e72b847b212874ab6348bb6f375c54a43564cc702abb24145efb513
allowed_operations: eight-path-doc-write, local-lightweight-validation, ordinary-commit, ordinary-push, pull-request-create, issue-pr-comment, review-ci-evidence-read
mutation_intent: 将性能预算 Discovery 已合并事实写成稳定终态，不创建预算、优化、CI 或产品实现。
executor_enforcement: 分支、基线、八路径、scope hash、稳定事实、反递归合同、验证或禁止面任一异常即停止。
agos_status: bypassed-unregistered-needs-input-owner-direct-write-blocked
anti_recursion_contract: 本任务自身的动态 PR Head、CI、Review、授权与合并证据只保留在 GitHub 评论，永久文档不把 Issue #52 标记为待合并活动任务。
task_plan_ref: docs/plans/2026-07-25-issue-52-performance-budget-closeout.md
session_plan_ref: docs/plans/sessions/2026-07-25-issue-52-performance-budget-closeout.md
runtime_workflow_ref: docs/workflows/2026-07-25-issue-52-performance-budget-closeout-runtime.md
report_ref: docs/reports/issue-52-performance-budget-closeout.md
pr_ref: github-comment-only-after-creation
review_ref: github-comment-only
ci_ref: github-comment-only
merge_ref: owner-authorization-required-for-final-head

## 业务路径

1. 读取 PR `#51`、Issue `#50`、Squash 提交、主干 CI 与 Artifact 的 Fresh 证据。
2. 将来源事实写入稳定 Closeout 报告和长期入口。
3. 清除 Master Plan 的旧 `active_*`、`tracking_issue_ref`、`next_legal_gate` 与 `decision_status`。
4. 使用 `build.md` 的可重复命令验证范围、事实、禁止面和仓库政策。
5. 创建非 Draft PR；动态交付证据只写入 GitHub，不改写本任务文件。

## 精确路径

```text
AGENTS.md
README.md
build.md
docs/plans/2026-07-25-issue-52-performance-budget-closeout.md
docs/plans/PROJECT-MASTER-PLAN.md
docs/plans/sessions/2026-07-25-issue-52-performance-budget-closeout.md
docs/reports/issue-52-performance-budget-closeout.md
docs/workflows/2026-07-25-issue-52-performance-budget-closeout-runtime.md
```

## 稳定事实

- PR `#51` Final Head：`e0154c61d8b05835db10437c79f029909516eac1`。
- Squash 提交：`fea8824c652665df710a7e6ef941854060eb6e1f`。
- tree：`9fb518cda8b35a9388fb9fce0a1ff6ba976d80cb`。
- 提交结构与签名：单父、GitHub `valid`。
- PR CI：Run `30174131581`，`required` 成功，Artifact `0`。
- 主干 CI：Run `30175592979`，七 Job 全绿，Artifact `0`。
- Issue `#50`：`CLOSED / COMPLETED`。
- `release_audit=current`，预算数值尚未批准，Gate 5 继续锁定。

## 批次

### Batch 1：control-plane

- 创建四份任务控制面。
- 冻结八路径与 scope hash。
- 记录 AGOS ReportOnly 状态；任何 AGOS 改动都禁止进入本任务。

### Batch 2：stable-state

- 更新 `AGENTS.md` 与 README 的长期项目状态。
- 更新 `build.md` 当前状态和 Issue `#52` 轻量验证入口。
- 将 Master Plan 转为“等待独立性能复测与数值批准 Issue”的稳定终态。

### Batch 3：verification-and-pr

- 运行八路径、scope hash、旧状态、占位符、Evidence、CI 合同、仓库政策和空白检查。
- 普通提交、普通推送并创建非 Draft PR。
- Fresh 核对 Review 对话和 GitHub-hosted CI，最终合并等待项目所有者单独授权。

## 禁止面

- Cargo、Rust、产品代码、测试实现、`benchmarks/`、Workflow、Ruleset、Release、`upstream/` 和 AGOS 零差异。
- 不写预算数值，不运行新测量，不实施优化，不启动 Gate 5。
- 不 Force Push，不删除任何分支或工作树。
- 不把 Issue `#52` 的动态状态写入长期控制面。

## 完成定义

- 八路径与 scope hash 通过。
- 稳定事实与反递归合同通过。
- 本地轻量验证通过。
- 非 Draft PR 已创建并完成适用 Review/CI。
- 最终 Squash Merge 未经项目所有者针对最终 Head 明确授权不得执行。
