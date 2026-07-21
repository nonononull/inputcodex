# Session Plan：Issue #2 重构与发布治理冻结

schema_version: agos.session-plan.v1
architecture_contract_version: agos.brainstorming-gate.v1
task_id: 2026-07-21-issue-2-architecture-governance
work_class: major
task_status: review-pending
task_summary: 将 inputcodex 重构、上游同步、版本发布与 GitHub 治理决策固化为项目单一真源，并通过 Issue #2 关联 PR 交付。
project_root: C:/Users/dashuai/Documents/inputcodex
trigger_source: 用户批准架构方案并要求立即落盘、持续维护单一真源
decision_status: approved
approval_source: inherited-user-instruction
approved_decision_ref: session-plan:2026-07-21-issue-2-architecture-governance#decision
scope_hash: sha256:1055a1f60198728b6f3ea489efda840cc72262828ce60180420e22326c6b5ec4
scope_hash_source: docs-only|issue-2|architecture-governance|no-source-import|no-actions|no-app-scaffold|no-release
mutation_intent: docs
executor_enforcement: project-native-docs-plus-github-issue-pr
allowed_operations:
  - 新增和修改项目治理文档、ADR、会话计划与运行工作流
  - 核验 GitHub Issue #2 与上游正式 Release 元数据
  - 更新项目总计划、构建验证入口和排错记录
  - 在验证通过后提交、推送并创建关联 Issue #2 的 PR
scope_boundary: 仅冻结方案与治理合同；不导入上游源码，不创建 Rust/Iced 工程，不实现 GitHub Actions，不发布安装包，不合并 PR。
selected_business_path: architecture-governance
delivery_contract: agos.issue-pr-merge.v1
tracking_issue_ref: https://github.com/nonononull/inputcodex/issues/2
review_strategy: 本地结构与语义自审、Fresh 验证证据、GitHub PR 由项目所有者审阅
ci_expectation: 当前仅文档变更；本地执行文档、Git 与 GitHub 元数据检查，后续仓库 CI 建立后再升级为 required checks
merge_policy: 项目所有者审阅通过且检查成功后只允许 Squash Merge；禁止 Merge Commit 和 Rebase Merge
closeout_ref: pending:merge-and-closeout-after-https://github.com/nonononull/inputcodex/pull/3

## Approved Decision

- Decision: 采用纯 Rust 与 Iced 重构 `inputcodex`，以性能优先、有效功能一致、Windows/macOS 同步交付为长期硬约束。
- Decision: 以 `BigPizzaV3/CodexPlusPlus` 最新正式 Release 为功能真源；当前起始基线为 `v1.2.41`，标签提交为 `3dafffcafb2566a1e8bce4b35671656d6adb3eda`。
- Decision: 上游完整源码从 `v1.2.41` 快照开始进入 `upstream/CodexPlusPlus/`，不导入上游 Git 历史，且不参与新产品构建。
- Decision: 上游 Tauri/React 管理界面、现有注入脚本和远程推荐列表只作快照审计，不直接进入新架构或最终运行面；需要保留其有效能力时另建功能或一致性例外 Issue。
- Decision: GitHub Actions 每 6 小时监控上游，只创建或更新 Issue，不自动同步、实现、合并或发布。
- Decision: 所有 PR 合并到 `main` 只允许 Squash Merge，禁止 Merge Commit 和 Rebase Merge；每个 Issue 在 `main` 上只保留一条可追踪、可回滚提交。
- Decision: 单人维护阶段平台 required approvals 为 `0`，但必须保留项目所有者决策证据；第二名具备合并权限的人类维护者加入后，在下一次 PR 合并前提升为 `1`，自动化账号不计入人数。
- Decision: 版本采用 `v<上游版本>-inputcodex.<修订号>`，安装包、更新清单、签名与下载地址全部属于 `nonononull/inputcodex`。
- Decision: 无效功能、有害副作用或错误语义争议必须建立 `parity-exception` Issue，由项目所有者决定。
- Reason: 用户明确拒绝 TypeScript/WebView 架构及其卡顿和异常加载问题，并批准纯 Rust、Iced、完整上游缓存、自主发布线与 Issue/PR 治理。
- Scope boundary: 本 Issue 只冻结文档和治理，不提前执行 Gate 2 及其后的源码、工作流、构建或发布任务。
- Rejected options: 继续维护半成品作为底座；直接复刻上游 Tauri/React 架构；只删除广告但保留原架构；以删除有效功能伪造性能提升；直接跟随上游 `main` 发布。

## Brainstorming

```yaml
level: major
proposal_mode: simulated-roles
fallback_reason: 开发者工具规则不允许在用户未要求时启动子 agent；主线程依据已完成的逐项澄清与用户批准，分别记录架构、操作体验和验证视角
superpowers_skill: superpowers:brainstorming
actual_agent_count: 0
agent_result_refs:
  - none
agent_budget_guard:
  initial_review_agents: 0
  escalation_agents: 0
  divergence: low
  idle_agent_cleanup: not-needed-zero-open
  timeout_policy: blocked-main-thread-rereview
  model_downgrade: forbidden
agent_model_contract:
  model_inheritance_policy: inherit-parent
  reasoning_effort_inheritance_policy: inherit-parent
  model_override_allowed: false
  override_approval_ref: none
  override_reason: none
agent_lifecycle:
  budget:
    max_total_agents: 0
    max_new_agents_per_round: 0
    actual_agent_count: 0
  spawn_preconditions:
    dispatch_plan_ref: not-applicable
    reclaim_before_spawn: not-needed-zero-open
    open_agent_count_before_dispatch: 0
  active_agent_refs:
    - none
  completion_status:
    completed:
      - none
    idle:
      - none
    timeout:
      - none
    failed:
      - none
  closed_agent_refs:
    - none
  timeout_handling: blocked-main-thread-rereview
  closeout_rule: no-agent-lifecycle-to-close
  owner_exception_ref: user-approved-main-thread-only
agent_proposals:
  - role: architecture-reviewer
    recommendation: 采用纯 Rust 与 Iced 分层重构，完整上游快照只作审计与映射，不参与产品构建。
    risks: 首期建设成本更高，功能迁移前必须先建立稳定端口、状态模型和错误边界。
    required_changes: 建立 Gate、依赖方向、快照隔离、合同测试和自主发布线。
    reject_if: 方案保留 Tauri/React/WebView 运行面，或让 Iced 类型泄漏到非展示层。
  - role: operator-experience-reviewer
    recommendation: 保持有效功能、双平台和版本节奏一致；上游每 6 小时只做 Issue 预警，任何争议功能进入 parity-exception。
    risks: 自动同步、静默删功能或平台长期残缺会破坏可预期性和用户信任。
    required_changes: 建立功能矩阵、显式加载状态、超时取消、错误隔离和项目所有者决策入口。
    reject_if: 性能优化依赖删除有效功能，或 Windows 与 macOS 长期不一致。
  - role: verification-reviewer
    recommendation: 所有架构、同步、迁移和发布决策都必须映射到可重复命令、性能数据、PR 与 Release 证据。
    risks: 只写原则而没有 Fresh 验证，会形成无法执行的治理装饰。
    required_changes: 固化 Session Plan、Runtime Workflow、Master Plan、Git checkpoint、PR 证据和后续性能预算门。
    reject_if: 宣称严格 AGOS、CI、性能或发布能力已完成，但没有对应命令输出和托管证据。
user_decision: 批准纯 Rust/Iced 重构、完整上游快照、每 6 小时云端监控、自主版本发布与 Issue/PR 合并治理。
decision_reason: 性能和稳定性必须从架构根因解决，同时保留有效功能并持续跟踪上游。
rejected_options:
  - 原架构小修小补
  - 以 zsr131550/CodexPlusPlus 作为新代码底座
  - 客户端直接消费上游发布资产或远程列表
deliberation:
  mode: proposal-debate
  same_question_ref: conversation:2026-07-21-inputcodex-architecture
  required_agent_count: 0
  returned_agent_count: 0
  reject_if_hits:
    - none
  conflict_matrix:
    - topic: 性能优先与功能一致冲突
      positions:
        - 不允许静默删有效功能
        - 争议项必须由 parity-exception Issue 决策
      divergence: low
      resolution_required: false
  parent_resolution:
    status: resolved
    allowed_ops:
      - docs
      - git
      - github-pr
    forbidden_ops:
      - source-import
      - application-scaffold
      - github-actions-implementation
      - release-publish
      - merge-without-owner-review
    implementation_freeze_status: released-for-docs-only
    freeze_reason: Gate 2 及以后仍未获本会话授权
    owner_decision_needed: none
    accepted_points:
      - 性能优先但不能伪造功能削减
      - 功能真源与自主发布线必须分离
      - 上游同步 PR 与功能迁移 PR 永远分离
    rejected_points:
      - 沿用上游运行时架构
      - 自动同步或自动合并上游变化
    modified_plan: none
    decision_rationale: 用户已逐项批准，且范围保持不变后连续要求继续落盘。
    unresolved_risks:
      - 具体性能预算必须在 Gate 4 通过实测 Issue 批准
      - 具体上游功能有效性必须由功能矩阵和例外 Issue 判定
```

## Change Contract

```yaml
change_contract:
  mutation_intent: docs
  target_contract:
    owner: 项目所有者
    expected_behavior: 仓库存在唯一、可追溯、可通过 PR 修改的重构和发布治理真源。
    evidence_refs:
      - docs/plans/2026-07-21-architecture-governance.md
      - https://github.com/nonononull/inputcodex/issues/2
  preserved_invariants:
    - name: 当前不包含应用源码
      owner: 项目所有者
      baseline_ref: git:09564740b8d00a4b09630c024607cc5292d0381f
      regression_ref: rg --files
    - name: GNU AGPLv3 许可证保持不变
      owner: 项目所有者
      baseline_ref: LICENSE@09564740b8d00a4b09630c024607cc5292d0381f
      regression_ref: git diff -- LICENSE
    - name: 软件名称固定为 inputcodex
      owner: 项目所有者
      baseline_ref: AGENTS.md
      regression_ref: rg -n "CodexPlusPlus" README.md CONTEXT.md AGENTS.md docs
  adjacent_surfaces:
    - name: 旧筹备计划
      why_adjacent: 旧计划仍是历史证据，但不得覆盖新单一真源。
      risk: 主计划继续引用旧 Gate 会造成执行分叉。
      owner: 项目所有者
    - name: build.md 与 err.md
      why_adjacent: 当前无源码时，文档验证和治理异常必须从这里进入。
      risk: 命令过期会产生虚假完成声明。
      owner: 项目所有者
  historical_state_refs:
    - docs/plans/2026-07-21-bootstrap.md
    - docs/reports/2026-07-21-bootstrap-closeout.md
    - git:09564740b8d00a4b09630c024607cc5292d0381f
  stale_verdict_invalidation_refs:
    - BigPizzaV3/CodexPlusPlus 最新正式 Release 变化时重新核验基线
    - 项目所有者修改纯 Rust、Iced、双平台或功能一致硬约束时重新进入 Brainstorming Gate
  regression_checks:
    - surface: 文档格式
      command_or_evidence_ref: git diff --check
      expected_result: 退出码为 0
    - surface: Session Plan 结构
      command_or_evidence_ref: verify-session-plan.ps1
      expected_result: SESSION_PLAN_VERIFY_OK
    - surface: Master Plan 索引
      command_or_evidence_ref: verify-master-plan-index.ps1
      expected_result: MASTER_PLAN_INDEX_VERIFY_OK
    - surface: GitHub 任务关联
      command_or_evidence_ref: gh issue view 2 --repo nonononull/inputcodex
      expected_result: Issue #2 为 OPEN 且内容覆盖本任务边界
    - surface: 上游正式基线
      command_or_evidence_ref: gh api repos/BigPizzaV3/CodexPlusPlus/releases/latest
      expected_result: 最新正式 Release 为 v1.2.41
  sibling_regression_guard:
    status: passed
    closeout_rule: passed-or-blocked-before-done
    exception_ref: none
  protected_feature_replay:
    status: passed
    not_applicable_reason: 当前无产品功能；使用许可证、空源码和筹备文档链接作为受保护基线。
    known_good_features:
      - feature: Gate 0 仓库筹备基线
        owner: 项目所有者
        baseline_evidence_ref: docs/reports/2026-07-21-bootstrap-closeout.md
        post_change_replay_plan_ref: 提交前核对 LICENSE、仓库公开性、历史计划与主计划链接
        post_change_replay_ref: pr:3@4acb76a08c2f1c74b3f7672fdc9d5f96ecdc5a84
        expected_result: Gate 0 证据保留，当前控制面切换到 Gate 1
        actual_result: LICENSE 未变化，仓库仍无应用源码，Gate 0 计划与 closeout 可访问，当前控制面已切换到 Gate 1。
        owner_visible_status: pending
        regression_status: passed
    forbidden_ops_until_replay:
      - claim-done
      - merge
```

## Local Knowledge Lookup

```yaml
local_knowledge_lookup:
  gbrain_queries:
    - inputcodex CodexPlusPlus 纯 Rust Iced 桌面重构 上游快照 发布治理 性能优先 功能一致
  gbrain_result: no-results
  vault_refs:
    - D:/Android_source/ai-growth-os/components/vault/08-Skills/AI-Growth-OS.md
    - D:/Android_source/ai-growth-os/components/vault/07-Workflows/Core/AI-Growth-OS-Brainstorming-Gate-And-Session-Plan.md
  rules_refs:
    - D:/Android_source/ai-growth-os/components/rules/rules/workflows/ai-growth-os-auto-application.md
    - D:/Android_source/ai-growth-os/components/rules/rules/workflows/ai-growth-os-brainstorming-gate.md
    - D:/Android_source/ai-growth-os/components/rules/rules/workflows/ai-growth-os-runtime-workflow.md
    - D:/Android_source/ai-growth-os/components/rules/rules/workflows/git-snapshot-governance.md
  project_refs:
    - README.md
    - AGENTS.md
    - build.md
    - err.md
    - CONTEXT.md
    - docs/plans/PROJECT-MASTER-PLAN.md
    - docs/plans/2026-07-21-architecture-governance.md
  remote_refs:
    - https://github.com/nonononull/inputcodex/issues/2
    - https://github.com/BigPizzaV3/CodexPlusPlus/releases/tag/v1.2.41
    - https://github.com/zsr131550/CodexPlusPlus
  missing_coverage:
    - 本地知识库未命中 CodexPlusPlus 专项资料，架构事实仍以仓库审计、上游快照和实测为准。
    - AGOS 全局 registry 没有 inputcodex 专属 task 与 business path；本外部项目按 warning mode 记录，不跨仓修改 AI Growth OS 控制面。
```

## Superpowers Method Discipline

```yaml
superpowers_method_discipline:
  upstream_superpowers_ref: https://github.com/obra/superpowers
  local_superpowers_state: available
  using_superpowers: superpowers:using-superpowers
  brainstorming: superpowers:brainstorming
  worktree_isolation:
    skill: superpowers:using-git-worktrees
    evidence: 当前在专用分支 docs/issue-2-architecture-governance；未创建额外 worktree
  planning_execution:
    writing_skill: superpowers:writing-plans
    executing_skill: superpowers:executing-plans
    subagent_skill: superpowers:subagent-driven-development
    plan_control_plane: project-native AGOS control docs
  test_driven_development:
    skill: superpowers:test-driven-development
    cycle: RED/GREEN/REFACTOR
    evidence: 纯文档任务不写产品行为代码；以验证脚本的失败到通过作为等价证据
  verification_before_completion:
    skill: superpowers:verification-before-completion
    evidence: 提交、推送和创建 PR 前执行全量 Fresh 验证
  systematic_debugging:
    skill: superpowers:systematic-debugging
    trigger: 任一验证失败、GitHub 元数据不一致或补丁工具异常
  code_review:
    request_skill: superpowers:requesting-code-review
    receive_skill: superpowers:receiving-code-review
    evidence: PR 由项目所有者审阅；当前不使用未经授权的子 agent
  finishing_branch:
    skill: superpowers:finishing-a-development-branch
    evidence: 用户已选择推送并创建 PR；不在本会话自动合并
  evidence_writeback:
    target: build.md, session plan, runtime workflow, PR body
    docs_superpowers_boundary: docs/superpowers remains archive-only, not the active control plane
```

## AGOS Entry Status

```yaml
agos_default_entry:
  command_date: 2026-07-21
  status: needs-input
  task_registration_status: unregistered
  project_git_foundation_status: ready
  project_entry_doc_foundation_status: ready
  local_knowledge_lookup_status: ready
  strict_runtime_validation_claimed: false
  handling: 外部项目 warning mode；以 GitHub Issue #2、当前分支和项目所有者批准作为本仓正式任务证据，不修改 D:/Android_source/ai-growth-os registry。
  source_implementation_admission: forbidden
```

## Master Plan

```yaml
path: docs/plans/PROJECT-MASTER-PLAN.md
update_required: true
update_summary: 将 active_task 切换为 Issue #2，将 Gate 1 标记为当前阶段，并链接总方案、ADR、Session Plan 与 Runtime Workflow。
```

## Runtime Workflow

```yaml
path: docs/workflows/2026-07-21-issue-2-architecture-governance-runtime.md
session_plan_ref: docs/plans/sessions/2026-07-21-issue-2-architecture-governance.md
approved_decision_ref: session-plan:2026-07-21-issue-2-architecture-governance#decision
selected_business_path: architecture-governance
workflow_nodes:
  - startup
  - knowledge-prep
  - plan
  - execute
  - verify
  - sync
subagent_roles:
  - none
skill_tree_nodes:
  - superpowers:using-superpowers
  - superpowers:brainstorming
  - superpowers:writing-plans
  - superpowers:executing-plans
  - superpowers:systematic-debugging
  - superpowers:verification-before-completion
  - superpowers:finishing-a-development-branch
  - karpathy-guidelines
stop_gates:
  - 用户改变已批准架构或当前 docs-only 范围
  - 上游最新正式 Release 不再是 v1.2.41
  - 需要导入源码、创建 Actions、搭建 Iced 或发布资产
  - 需要修改 AI Growth OS 跨仓控制面
verification_commands:
  - verify-session-plan.ps1
  - verify-master-plan-index.ps1
  - git diff --check
  - git diff --cached --check
  - verify-git-snapshot-governance.ps1 -Checkpoint -ReportOnly
```

## Delivery Governance

```yaml
delivery_contract: agos.issue-pr-merge.v1
tracking_issue_ref: https://github.com/nonononull/inputcodex/issues/2
branch_ref: docs/issue-2-architecture-governance
review_ref: pending:project-owner-review
pr_ref: https://github.com/nonononull/inputcodex/pull/3
ci_ref: not-configured:statusCheckRollup-empty-2026-07-21
merge_ref: pending:owner-approved-merge
review_strategy: Fresh 本地验证后创建非 Draft PR，由项目所有者逐项核对硬约束、范围和上游基线。
ci_expectation: 文档校验必须通过；若仓库尚无 Actions，则 PR 明确记录本地命令和结果，不伪造 CI。
merge_policy: PR 正文必须包含 Closes #2；单人阶段 required approvals 为 0 但必须有项目所有者决策证据，多人阶段 required approvals 为 1；批准后只允许 Squash Merge，禁止 Merge Commit 和 Rebase Merge。
```

## Completion Criteria

- `docs/plans/2026-07-21-architecture-governance.md` 成为明确标注的单一真源。
- `CONTEXT.md`、`AGENTS.md`、两份 ADR、Master Plan、Session Plan 与 Runtime Workflow 互不矛盾。
- `build.md` 给出当前文档任务可重复执行的验证命令，`err.md` 记录本次 AGOS 与补丁工具异常。
- Fresh 验证已通过，提交 `4acb76a08c2f1c74b3f7672fdc9d5f96ecdc5a84` 已推送到 `docs/issue-2-architecture-governance`。
- 已创建包含 `Closes #2` 的 PR `#3`，当前状态为 `OPEN`、非 Draft、`mergeStateStatus=CLEAN`，等待项目所有者 Review；本会话不自动合并。
