# Runtime Workflow：Issue #2 重构与发布治理冻结

```yaml
task_id: 2026-07-21-issue-2-architecture-governance
session_plan_ref: docs/plans/sessions/2026-07-21-issue-2-architecture-governance.md
approved_decision_ref: session-plan:2026-07-21-issue-2-architecture-governance#decision
selected_business_path: architecture-governance
workflow_lookup_mode: dynamic-generated
static_workflow_refs:
  - D:/Android_source/ai-growth-os/components/rules/rules/workflows/ai-growth-os-auto-application.md
  - D:/Android_source/ai-growth-os/components/rules/rules/workflows/ai-growth-os-runtime-workflow.md
  - D:/Android_source/ai-growth-os/components/rules/rules/workflows/git-snapshot-governance.md
dynamic_workflow_gap_summary: AGOS 全局 registry 尚无 inputcodex 专属 task 与 architecture-governance business path；当前以项目原生控制面和 GitHub Issue #2 在外部项目 warning mode 执行。
task_scope_boundary: 仅修改文档与 Git 元数据；不导入源码、不创建应用或 Actions、不发布、不合并。
task_current_state: 文档与治理方案已通过 Fresh 本地验证，提交 4acb76a 已推送，PR #3 为 OPEN、非 Draft、mergeStateStatus=CLEAN，等待项目所有者 Review。
task_owner: nonononull
task_follow_up_required: 项目所有者在 PR 中审阅硬约束、范围和上游基线，批准后才能合并。
task_validation_attribution: 本地 Fresh 命令输出、提交 4acb76a、GitHub PR #3 元数据和待补项目所有者 Review。
task_closeout_ref: pending:docs/reports/issue-2-architecture-governance-closeout.md

allowed_operations:
  - docs-write
  - git-status
  - git-diff
  - git-add
  - git-commit
  - git-push-current-branch
  - github-create-linked-pr
forbidden_operations:
  - source-import
  - source-code-write
  - application-scaffold
  - github-actions-write
  - release-publish
  - merge-pr
  - cross-repo-agos-registry-write

workflow_nodes:
  - startup
  - knowledge-prep
  - plan
  - execute
  - verify
  - sync
node_order:
  - 读取项目规则、构建入口、排错记录和已批准方案
  - 完成 GBrain、Vault、Rules 和项目文档查询
  - 核验 Issue #2、上游最新正式 Release 和 v1.2.41 标签提交
  - 固化总方案、项目语境和 ADR
  - 写入 Major Session Plan 与本 Runtime Workflow
  - 更新 Master Plan、build.md 和 err.md
  - 自审占位符、矛盾、失效链接和越界内容
  - 运行 Session Plan、Master Plan、Git diff 和 Git snapshot 验证
  - 暂存并运行 cached diff 验证
  - 提交并推送当前分支
  - 创建包含 Closes #2 的待审 PR
  - 等待项目所有者 Review，不自动合并

subagent_roles:
  - none-owner-did-not-request-subagents
skill_tree_nodes:
  - superpowers:using-superpowers
  - superpowers:brainstorming
  - superpowers:writing-plans
  - superpowers:executing-plans
  - superpowers:systematic-debugging
  - superpowers:verification-before-completion
  - superpowers:finishing-a-development-branch
  - karpathy-guidelines

code_authoring_rules:
  - rules/domain/agent-generated-code.md
  - rules/quality/comments.md
  - rules/quality/complexity-file-size.md
  - rules/quality/testing.md
code_authoring_gates:
  - non-obvious logic must have boundary comments；本任务没有产品逻辑，文档中的非显然约束必须写明理由和停止条件
  - trivial-code no-comment decision；本任务不新增代码，不为显然事实堆砌注释或解释
  - ownership, dependency, risk, and verification boundaries 必须在 Session Plan、Runtime Workflow 与总方案中可追溯
  - 文档只写已批准决策，不擅自增加功能、依赖、性能数值或发布承诺
  - 第三方仓库事实必须带仓库、版本和提交证据

model_drift_guards:
  - 软件名称始终为 inputcodex
  - 性能优先不能解释为静默删除有效功能
  - 功能真源是上游最新正式 Release，main 只作变化预警
  - 禁止 TypeScript、JavaScript 业务代码和 WebView
  - Iced 只允许存在于展示层
  - Windows 与 macOS 从首版起功能一致
  - 广告、推广、导流和隐蔽遥测不得进入最终运行面
  - 上游完整快照与新产品构建必须隔离
  - 上游 Tauri/React 管理界面、现有注入脚本和远程推荐列表只能用于审计，不得直接进入新架构或最终运行面
  - 上游同步 PR 与功能迁移 PR 永远分离
  - 所有 PR 合并到 main 只允许 Squash Merge，禁止 Merge Commit 和 Rebase Merge
  - main 永久禁止 --force 和 --force-with-lease；错误历史与紧急修复只能通过 revert 和关联 Issue/PR 处理
  - 单人维护阶段 required approvals 为 0 但必须有项目所有者决策证据；第二名具备合并权限的人类维护者加入后在下一次合并前提升为 1
  - 客户端更新和资产只指向 nonononull/inputcodex
  - 争议功能必须走 parity-exception Issue

err_md_correction_watchlist:
  - AGOS 默认入口对未登记外部任务返回 needs-input
  - AGOS 严格 Runtime Workflow 校验依赖已登记 business path
  - Git snapshot 在关键文档未提交时返回 blocked
  - Windows 单条补丁命令长度上限
  - Codex Desktop apply_patch 包装器拒绝访问
  - GitHub Release 或标签提交在验证期间变化
  - PR 正文遗漏 Closes #2

stop_gates:
  - 用户修改当前已批准架构、版本、同步或发布规则
  - 上游最新正式 Release 不再是 v1.2.41
  - 上游 v1.2.41 标签不再解析到 3dafffcafb2566a1e8bce4b35671656d6adb3eda
  - 任一文档要求导入源码、搭建 Rust/Iced、创建 Actions 或发布资产
  - 任一改动需要跨仓修改 D:/Android_source/ai-growth-os
  - 验证脚本、git diff --check 或 cached diff 检查失败
  - 当前分支不是 docs/issue-2-architecture-governance
  - PR 不能关联 Issue #2

git_progress_checkpoints:
  - startup-baseline: verify-git-snapshot-governance.ps1 -Checkpoint -ReportOnly；记录现有草案未提交状态
  - after-control-docs: 写完 Session Plan、Runtime Workflow、Master Plan、build.md 和 err.md 后再次执行
  - pre-verification: 重验证前执行并停止扩大范围
  - pre-commit: 暂存前后分别执行 Git 状态和 diff 检查
  - handoff: 创建 PR 后记录分支、提交和 PR URL
git_commit_discipline_gate:
  - verify-git-snapshot-governance.ps1 -CommitDiscipline -RequireFeatureBranchForMutableWork -ReportOnly
  - 当前分支必须为 docs/issue-2-architecture-governance
  - 提交主题使用 docs: 固化重构与发布治理方案
  - PR 最终只能 Squash Merge，使一个 Issue 在 main 上对应一条可回滚提交
  - 不允许对 main 使用 force push；不允许绕过 PR 直接修改 main
project_git_foundation_gate:
  - verify-project-git-foundation.ps1 -ProjectRoot C:/Users/dashuai/Documents/inputcodex -RequireGit -ReportOnly
project_git_foundation_status: ready
project_git_foundation_next_action: 在现有 Issue #2 文档分支完成验证、提交与 PR。
project_git_foundation_forbidden_ops: direct-main-write,force-push,merge-without-review
project_entry_doc_foundation_gate:
  - verify-project-entry-doc-foundation.ps1 -ProjectRoot C:/Users/dashuai/Documents/inputcodex -ReportOnly
project_entry_doc_foundation_status: ready
project_entry_doc_foundation_next_action: 保持 README.md、AGENTS.md、build.md、err.md 与当前任务计划同步。
project_entry_doc_foundation_forbidden_ops: claim-entry-docs-ready-without-fresh-check

post_implementation_review_gate:
  - verify-post-implementation-review.ps1 -ProjectRoot C:/Users/dashuai/Documents/inputcodex -ReportOnly
  - 当前 docs-only 任务由主线程自审并在 GitHub PR 等待项目所有者 Review
  - 未取得 owner review_ref 前禁止合并或宣称治理闭环完成
protected_feature_replay_gate:
  - verify-protected-feature-replay.ps1 -SessionPlanPath docs/plans/sessions/2026-07-21-issue-2-architecture-governance.md -ReportOnly
  - 回放 LICENSE 不变、仓库无应用源码、Gate 0 历史证据仍可访问
  - 未完成回放前禁止 claim-done 或 merge
code_understanding_gate:
  - verify-code-understanding-tool-use.ps1 -ProjectRoot C:/Users/dashuai/Documents/inputcodex -ReportOnly
  - 当前仓库无应用源码且 .codegraph 不存在，因此代码理解门为 not-required
protocol_state_evidence_gate:
  - verify-protocol-state-evidence-contract.ps1 -ProjectRoot C:/Users/dashuai/Documents/inputcodex -ReportOnly
  - 本任务不涉及协议状态，必须明确 not-required，禁止伪造协议证据
protocol_sync_closure_gate:
  - 当前不执行协议同步；状态为 not-required
delegated_deliberation_gate:
  - 当前未启动子 agent；状态为 not-required
  - 若后续用户要求 reviewer，必须先声明 bounded reviewer 范围和只读文件所有权

task_interruption_packets:
  - pause: 记录当前分支、git status、最后通过命令和下一条命令
  - upstream-change: 记录新 Release/标签并冻结提交，返回项目所有者确认是否更新基线
  - verification-failure: 先查 err.md，使用 systematic-debugging 定位根因
  - scope-change: 将 decision_status 改为 needs-user，禁止继续写入
task_loop_policy_gate:
  - 当前任务是一次性 docs-only 交付，不启动重复执行循环
  - 任何重试最多围绕同一失败验证，不扩大改动范围
misjudgment_recovery_gate:
  - 发现事实错误时立即停止提交，回滚当前错误文档行并记录 err.md
  - 发现架构误判时返回 Issue #2 讨论，不静默改写批准决策
project_state_gates:
  - Gate 0 已验证并保留历史 closeout
  - Gate 1 当前 active，仅允许方案与治理冻结
  - Gate 2 及以后 locked，必须新 Issue、Session Plan 和用户批准
long_task_states:
  - discovery-complete
  - plan-approved
  - docs-execution-active
  - verification-pending
  - pr-review-pending
execution_windows:
  - window-1: 已批准方案、CONTEXT.md 与 ADR
  - window-2: Session Plan、Runtime Workflow、Master Plan、build.md、err.md
  - window-3: Fresh 验证、Git snapshot、提交、推送、PR
execution_ownership_contract:
  - 主线程独占本任务全部写入文件
  - 不启动未经用户要求的写入型子 agent
  - PR 创建后不在本会话自动合并或删除分支

verification_gates:
  - verify-project-git-foundation.ps1 -ProjectRoot C:/Users/dashuai/Documents/inputcodex -RequireGit -ReportOnly
  - verify-project-entry-doc-foundation.ps1 -ProjectRoot C:/Users/dashuai/Documents/inputcodex -ReportOnly
  - verify-code-understanding-tool-use.ps1 -ProjectRoot C:/Users/dashuai/Documents/inputcodex -ReportOnly
  - verify-post-implementation-review.ps1 -ProjectRoot C:/Users/dashuai/Documents/inputcodex -ReportOnly
  - verify-protected-feature-replay.ps1 -SessionPlanPath docs/plans/sessions/2026-07-21-issue-2-architecture-governance.md -ReportOnly
  - verify-protocol-state-evidence-contract.ps1 -ProjectRoot C:/Users/dashuai/Documents/inputcodex -ReportOnly
  - verify-session-plan.ps1 -Path docs/plans/sessions/2026-07-21-issue-2-architecture-governance.md
  - verify-master-plan-index.ps1 -Path docs/plans/PROJECT-MASTER-PLAN.md
  - verify-git-snapshot-governance.ps1 -ProjectRoot C:/Users/dashuai/Documents/inputcodex -TaskId 2026-07-21-issue-2-architecture-governance -WorkflowNode verify -Checkpoint -ReportOnly
  - git diff --check
  - git diff --cached --check
  - gh issue view 2 --repo nonononull/inputcodex
  - gh api repos/BigPizzaV3/CodexPlusPlus/releases/latest
  - gh api repos/BigPizzaV3/CodexPlusPlus/git/ref/tags/v1.2.41

strict_runtime_validator_status: blocked-by-unregistered-external-task-and-business-path
strict_runtime_validator_claimed: false
strict_runtime_validator_recovery: 若未来将 inputcodex 纳入 AGOS 全局 registry，先创建独立跨仓治理 Issue/PR，再映射 task 与 business path 并运行 verify-runtime-workflow.ps1。

rollout_draft:
  reusable_path: GitHub Issue 驱动的外部项目架构治理文档冻结
  record_at_closeout: true
  closeout_boundary: PR 合并并补齐 review_ref、ci_ref、merge_ref 后
  current_status: deferred-until-pr-3-merge-closeout
  candidate_rule: 单次 rollout 不能生成 workflow candidate
```
