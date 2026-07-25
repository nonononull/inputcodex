# Runtime Workflow：Issue #52 性能预算 Discovery Closeout

workflow_status: immutable-execution-contract
tracking_issue_ref: https://github.com/nonononull/inputcodex/issues/52
branch_ref: codex/issue-52-performance-budget-closeout
baseline_ref: fea8824c652665df710a7e6ef941854060eb6e1f
approved_decision_ref: user-message:建立独立-closeout-issue-pr-更新-master-plan-2026-07-25
approved_scope_ref: https://github.com/nonononull/inputcodex/issues/52#issuecomment-5080685026
scope_hash: sha256:af1cfffe1e72b847b212874ab6348bb6f375c54a43564cc702abb24145efb513
allowed_operations: eight-path-doc-write, local-lightweight-validation, ordinary-commit, ordinary-push, pull-request-create, issue-pr-comment, review-ci-evidence-read
mutation_intent: 固化 Issue #50 / PR #51 已合并证据并转入等待独立性能复测与预算数值批准的稳定状态。
executor_enforcement: 分支、基线、范围、哈希、稳定事实、反递归、验证或禁止面异常即停止。
agos_status: bypassed-unregistered-needs-input-owner-direct-write-blocked
anti_recursion_contract: Issue #52 自身动态证据仅写 GitHub 评论，永久文档不得把本任务写成新的待合并活动状态。

## Phase 0：启动与证据

1. 确认分支为 `codex/issue-52-performance-budget-closeout`，基线为 `fea8824c652665df710a7e6ef941854060eb6e1f`。
2. 读取 `AGENTS.md`、README、`build.md`、`err.md`、Master Plan、Issue `#50` / PR `#51` 和 Issue `#52`。
3. Fresh 核对 Final Head、Squash、单父、tree、签名、PR CI、主干 CI、Artifact、Issue 关闭和来源分支状态。
4. 运行 AGOS `ReportOnly`；若返回 `needs-input`、`unregistered`、接口异常或直接写入阻断，记录后绕过，不修改 AGOS。

## Phase 1：范围与反递归

1. 只允许八路径：

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

2. scope hash 必须为 `sha256:af1cfffe1e72b847b212874ab6348bb6f375c54a43564cc702abb24145efb513`。
3. 永久文档只记录 Issue `#50` / PR `#51` 的稳定终态，不记录本任务未来 PR 的动态状态。
4. 任何新增、删除、重命名或越界路径立即停止。

## Phase 2：稳定状态写入

1. 更新 `AGENTS.md`：写入 PR `#51` Squash、tree、签名、CI、Issue 关闭和下一合法工作。
2. 更新 README：修正项目摘要、当前阶段、下一步和文档入口。
3. 更新 `build.md`：修正当前状态并新增 Issue `#52` 可重复轻量验证。
4. 更新 Master Plan：清除 Issue `#50` 的旧活动字段，转为等待独立性能复测与数值批准 Issue。
5. 新报告只陈述来源稳定事实、产品语义、下一合法工作和反递归边界。

## Phase 3：本地轻量验证

1. 执行 `build.md` 的 Issue `#52` PowerShell 代码块。
2. 验证八路径、scope hash、旧活动状态、占位符、Performance Evidence、CI 合同和 Repository Policy。
3. 运行未暂存与已暂存 `git diff --check`。
4. 不运行完整 Workspace、桌面 Release 或真实性能采集。

## Phase 4：提交、PR 与 Review

1. 仅暂存八路径，使用普通 Git 提交和普通推送。
2. 创建关联 Issue `#52` 的非 Draft PR，正文说明范围、哈希、稳定事实、反递归和禁止面。
3. Fresh 核对最终 Head、适用 CI、Artifact、Review 对话和合并状态。
4. Review 反馈必须先确定根因、处理并重验证；所有对话解决后才能请求最终合并授权。
5. 最终 Squash Merge 必须由项目所有者针对最终 Head 单独授权；禁止自动合并、Force Push 或删除分支。

## 停止条件

- 需要修改八路径之外的文件。
- 需要填写预算数值、运行新测量、实施优化或启动 Gate 5。
- 需要修改代码、Cargo、Workflow、Ruleset、Release、`upstream/` 或 AGOS。
- 来源事实、scope hash、Review、CI 或项目所有者授权发生漂移。
