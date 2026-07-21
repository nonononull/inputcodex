# Runtime Workflow：Issue #4 Gate 1 合并证据 closeout

```yaml
task_id: 2026-07-21-issue-4-gate-1-closeout
session_plan_ref: docs/plans/sessions/2026-07-21-issue-4-gate-1-closeout.md
approved_decision_ref: session-plan:2026-07-21-issue-4-gate-1-closeout#decision
selected_business_path: architecture-governance
workflow_lookup_mode: dynamic-generated
static_workflow_refs:
  - D:/Android_source/ai-growth-os/components/rules/rules/workflows/ai-growth-os-auto-application.md
  - D:/Android_source/ai-growth-os/components/rules/rules/workflows/ai-growth-os-runtime-workflow.md
  - D:/Android_source/ai-growth-os/components/rules/rules/workflows/git-snapshot-governance.md
dynamic_workflow_gap_summary: AGOS registry 未登记 inputcodex task 与 architecture-governance 路径；本任务以 Issue #4、专用分支和项目原生控制文档在 warning mode 执行。
task_scope_boundary: 只回写 Issue #2 / PR #3 已发生的 closeout 事实并创建 Issue #4 开放 PR；不导入源码、不创建 Actions、不修改 Ruleset、不发布、不合并当前 PR。
task_current_state: Issue #4 OPEN，分支 codex/issue-4-gate-1-closeout 基于 main 合并提交 0e113759；closeout 文档正在写入，关联 PR 尚未创建。
task_owner: nonononull
task_follow_up_required: Fresh 验证、提交、正常推送、创建包含 Closes #4 的开放 PR，回写真实 PR URL 后停止等待项目所有者 Review。
task_validation_attribution: GitHub Issue/PR/GraphQL/Ruleset API、Git commit/tree 对象、本地 refs、项目验证脚本与 diff 检查。
task_closeout_ref: pending:issue-4-pr-merge-closeout

allowed_operations:
  - docs-write
  - git-status
  - git-diff
  - git-add-approved-docs
  - git-commit-current-branch
  - git-push-current-branch
  - github-create-linked-pr
  - github-read-issue-pr-ruleset
forbidden_operations:
  - source-import
  - source-code-write
  - application-scaffold
  - github-actions-write
  - github-ruleset-write
  - required-check-write
  - release-publish
  - force-push
  - direct-main-write
  - merge-current-pr
  - cross-repo-agos-registry-write

workflow_nodes:
  - startup
  - knowledge-prep
  - plan
  - execute
  - verify
  - sync
  - pr
  - handoff
node_order:
  - 读取 AGENTS.md、README.md、build.md、err.md、Master Plan 和 Issue #2 控制文档
  - 运行 GBrain 查询并读取 AI Growth OS Vault/Rules；记录无命中与 warning-mode 缺口
  - 核验 Issue #4、Issue #2、PR #3、Review 对话、Checks、合并提交、tree、分支和 Ruleset
  - 写入 Issue #4 计划、Session Plan、Runtime Workflow 与 Issue #2 closeout 报告
  - 更新 Master Plan、README、总架构、Issue #2 Session Plan/Runtime、build.md 与 err.md
  - 运行 AGOS 默认入口 ReportOnly、Session Plan、Master Plan、protected replay 和 Git snapshot 验证
  - 运行 rollout recorder dry-run；未登记外部 task 时只记录 repair-required，不跨仓写 registry
  - 精确暂存允许文件，运行 cached diff 检查并创建命名提交
  - 正常推送当前分支并创建包含 Closes #4 的非 Draft PR
  - 回写真实 PR URL，重验证、提交、推送并复核 PR OPEN/CLEAN/Review 状态
  - 停止在项目所有者 Review，不自动合并或删除分支

subagent_roles:
  - none-owner-did-not-request-subagents
skill_tree_nodes:
  - superpowers:using-superpowers
  - superpowers:brainstorming
  - superpowers:using-git-worktrees
  - superpowers:executing-plans
  - superpowers:systematic-debugging
  - superpowers:verification-before-completion
  - superpowers:requesting-code-review
  - superpowers:finishing-a-development-branch
  - karpathy-guidelines
  - knowledge-graph-auto-update

err_md_correction_watchlist:
  - apply_patch.bat 在 Codex Desktop 中可能 Access is denied；复用 err.md 的官方 Codex apply-patch 入口
  - PowerShell 脚本错误不能只看 LASTEXITCODE；原生 git/gh quiet 命令必须显式检查 LASTEXITCODE
  - Git HTTPS fetch/push 与 gh api 可出现不同网络结果；禁止把 API 成功等同于 Git transport 成功
  - 重建 GitHub 签名提交时必须完整保留 verification.payload、gpgsig 缩进与签名末尾 continuation
  - 0 Checks 表示未配置 CI，不表示 CI 已通过
  - Ruleset 与仓库级合并开关作用域不同，main 的有效规则以 Ruleset 为准

git_progress_checkpoints:
  - startup-baseline: 已在 clean 分支 0e113759 上运行 verify-git-snapshot-governance.ps1 -Checkpoint -ReportOnly
  - after-docs: 文档批次完成后运行并停止扩大范围
  - pre-verification: Fresh 重验证前运行
  - pre-commit: 暂存前后核对 status、diff 与 cached diff
  - handoff: PR 创建与 URL 回写后运行
git_commit_discipline_gate:
  - verify-git-snapshot-governance.ps1 -CommitDiscipline -RequireFeatureBranchForMutableWork -ReportOnly
  - 当前分支必须为 codex/issue-4-gate-1-closeout
  - 首个提交主题使用 docs: 回写 Gate 1 合并 closeout 证据
  - PR URL 回写提交主题使用 docs: 记录 Issue 4 closeout PR
  - 禁止 Force Push；所有后续修正使用正常追加提交
  - PR 最终只允许 Squash Merge，但本任务不执行合并

project_state_gates:
  - Gate 0 已验证
  - Gate 1 架构治理 Issue #2 / PR #3 已 Squash Merge并关闭
  - Gate 1 closeout Issue #4 当前 active
  - Gate 1 模板与标签仍待独立 Issue/PR
  - Gate 2 及以后保持 locked

task_interruption_packets:
  - pause: 记录分支、HEAD、git status、最后通过命令和下一步
  - github-fact-change: 停止写入并重新抓取 Issue/PR/Ruleset/commit 证据
  - verification-failure: 先查 err.md，使用 systematic-debugging 确定根因
  - scope-change: 将 decision_status 改为 needs-user，禁止继续 mutation

verification_gates:
  - verify-project-git-foundation.ps1 -ProjectRoot C:/Users/dashuai/Documents/inputcodex -RequireGit -ReportOnly
  - verify-project-entry-doc-foundation.ps1 -ProjectRoot C:/Users/dashuai/Documents/inputcodex -ReportOnly
  - verify-session-plan.ps1 -Path docs/plans/sessions/2026-07-21-issue-2-architecture-governance.md
  - verify-session-plan.ps1 -Path docs/plans/sessions/2026-07-21-issue-4-gate-1-closeout.md
  - verify-master-plan-index.ps1 -Path docs/plans/PROJECT-MASTER-PLAN.md
  - verify-protected-feature-replay.ps1 -Path docs/plans/sessions/2026-07-21-issue-4-gate-1-closeout.md -RequireProtectedReplay -ReportOnly
  - verify-git-snapshot-governance.ps1 -ProjectRoot C:/Users/dashuai/Documents/inputcodex -TaskId 2026-07-21-issue-4-gate-1-closeout -WorkflowNode verify -Checkpoint -ReportOnly
  - git diff --check
  - git diff --cached --check
  - gh issue view 2 --repo nonononull/inputcodex
  - gh issue view 4 --repo nonononull/inputcodex
  - gh pr view 3 --repo nonononull/inputcodex
  - gh api graphql reviewThreads
  - gh api repos/nonononull/inputcodex/rulesets/19395456
  - gh api repos/nonononull/inputcodex/rules/branches/main
  - git rev-list --parents -n 1 0e11375997ff10fdc0c233b31c8468af2d9a4f44
  - git show -s --format=%T 0e11375997ff10fdc0c233b31c8468af2d9a4f44
  - git show -s --format=%T 6b090ba5aa479c714c9e231aa07787724d6a8190

strict_runtime_validator_status: blocked-by-unregistered-external-task-and-business-path
strict_runtime_validator_claimed: false
strict_runtime_validator_recovery: 若未来要纳入 AGOS 正式 registry，必须在 AI Growth OS 仓库另建 Issue/PR，登记 task/business path 后重放；本仓不越权修复。

delivery_evidence:
  tracking_issue_ref: https://github.com/nonononull/inputcodex/issues/4
  branch_ref: codex/issue-4-gate-1-closeout
  review_ref: pending:project-owner-review-on-issue-4-pr
  pr_ref: pending:issue-4-closeout-pr
  ci_ref: not-configured:repository-has-no-actions-or-required-checks
  merge_ref: none:issue-4-pr-must-remain-open

rollout_record:
  reusable_path: 外部 GitHub 项目在 Squash Merge 后通过独立 Issue/PR 回写 closeout、Master Plan 与 delivery evidence
  recorder_mode: dry-run
  current_status: repair-required-unregistered-source-task
  source_task: 2026-07-21-issue-2-architecture-governance
  suggested_task_id: agos-p2-2026-07-21-issue-2-architecture-governance
  recorder_write_status: blocked
  task_intake_draft_status: ready-for-main-thread-review
  task_intake_draft_write_status: stdout-only
  cross_repo_write_performed: false
  candidate_rule: 单次 rollout 不能生成 workflow candidate
```
