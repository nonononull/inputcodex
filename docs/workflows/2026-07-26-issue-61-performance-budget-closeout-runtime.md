# Runtime Workflow：Issue #61 性能预算数值 Closeout

workflow_status: immutable-execution-contract
tracking_issue_ref: https://github.com/nonononull/inputcodex/issues/61
branch_ref: codex/issue-61-performance-budget-closeout
baseline_ref: e225144831a0928bfa3aaa0d169a054779005812
approved_decision_ref: user-message:批准-Issue-61-八路径范围与-scope_hash-允许实施提交推送PRReviewCI-2026-07-26
approved_scope_ref: https://github.com/nonononull/inputcodex/issues/61#issuecomment-5082819125
scope_hash: sha256:dafe55bfc38c38782558c1577215d227ac8c83b7110735c4ddd58b48d66264b5
allowed_operations: eight-path-doc-write, local-lightweight-validation, ordinary-commit, ordinary-push, pull-request-create, issue-pr-comment, review-ci-evidence-read
mutation_intent: 固化 Issue #59 / PR #60 已合并证据并转入等待独立预算 CI 观察 Issue 的稳定状态。
executor_enforcement: 分支、基线、范围、哈希、稳定事实、反递归、验证或禁止面异常即停止。
agos_status: bypassed-report-only-unregistered-needs-input-owner-direct-write-blocked-no-cross-repo-mutation
anti_recursion_contract: Issue #61 自身动态证据仅写 GitHub 评论，永久文档不得把本任务写成新的待合并活动状态。

## Phase 0：启动与证据

1. 确认分支为 `codex/issue-61-performance-budget-closeout`，基线为 `e225144831a0928bfa3aaa0d169a054779005812`。
2. 读取 `AGENTS.md`、README、`build.md`、`err.md`、Master Plan、Issue `#59` / PR `#60` 和 Issue `#61`。
3. Fresh 核对 Final Head、Squash、单父、tree、签名、PR/主干 CI、Performance Run、Artifact、Issue 关闭和来源分支状态。
4. 运行 AGOS `ReportOnly`；返回 `needs-input/unregistered` 与直接写入阻断后记录并绕过，不修改 AGOS。

## Phase 1：范围与反递归

1. 只允许八路径：

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

2. scope hash 必须为 `sha256:dafe55bfc38c38782558c1577215d227ac8c83b7110735c4ddd58b48d66264b5`。
3. 永久文档只记录 Issue `#59` / PR `#60` 的稳定终态，不记录本任务未来 PR 的动态状态。
4. 任何新增、删除、重命名或越界路径立即停止。

## Phase 2：稳定状态写入

1. 更新 `AGENTS.md`：写入 PR `#60` Squash、tree、签名、双套主干 CI、Issue 关闭和下一合法工作。
2. 更新 README：修正项目摘要、当前阶段、下一步和文档入口。
3. 更新 `build.md`：修正当前状态并新增 Issue `#61` 可重复轻量验证。
4. 更新 Master Plan：清除 Issue `#59` 的旧活动字段，转为等待独立预算 CI 观察 Issue。
5. 新报告只陈述来源稳定事实、产品语义、下一合法工作和反递归边界。

## Phase 3：本地轻量验证

1. 执行 `build.md` 的 Issue `#61` PowerShell 代码块。
2. 验证八路径、scope hash、旧活动状态、占位符、Performance Evidence、预算复算、CI 合同和 Repository Policy。
3. 运行未暂存与已暂存 `git diff --check`。
4. 不运行完整 Workspace、桌面 Release 或真实性能采集。

## Phase 4：提交、PR 与 Review

1. 仅暂存八路径，使用普通 Git 提交和普通推送。
2. 创建关联 Issue `#61` 的非 Draft PR，正文说明范围、哈希、稳定事实、反递归和禁止面。
3. Fresh 核对最终 Head、适用 CI、Artifact、Review 对话和合并状态。
4. Review 反馈必须先确定根因、处理并重验证；所有对话解决后才能请求最终合并授权。
5. 最终 Squash Merge 必须由项目所有者针对最终 Head 单独授权；禁止自动合并、Force Push 或删除分支。

## 停止条件

- 需要修改八路径之外的文件。
- 需要修改预算数值、运行新测量、实施优化或启动 Gate 5。
- 需要修改代码、Cargo、Workflow、Ruleset、Release、`upstream/` 或 AGOS。
- 来源事实、scope hash、Review、CI 或项目所有者授权发生漂移。
