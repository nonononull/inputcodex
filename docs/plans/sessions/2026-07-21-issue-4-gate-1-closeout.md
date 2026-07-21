# Session Plan：Issue #4 Gate 1 合并证据 closeout

schema_version: agos.session-plan.v1
architecture_contract_version: agos.brainstorming-gate.v1
schema_role: optional-external-format-compatibility
governance_authority: inputcodex-project-native
external_agos_policy: use-when-available-bypass-when-unavailable-no-optimization
task_id: 2026-07-21-issue-4-gate-1-closeout
work_class: standard
task_status: pr-review-pending
task_summary: 通过独立 Issue、分支与 PR 回写 Issue #2 / PR #3 的最终合并证据，并把 Master Plan 与项目入口同步到真实状态。
project_root: C:/Users/dashuai/Documents/inputcodex
trigger_source: 用户明确要求现在同步，并建立独立 closeout Issue/PR，回写合并证据和最新 Master Plan
decision_status: approved
approval_source: inherited-user-instruction
approved_decision_ref: session-plan:2026-07-21-issue-4-gate-1-closeout#decision
scope_hash: sha256:91d211b15a79b3795b61048db95ae5490726ceac5a25da98fda60cf693790aa8
scope_hash_source: docs-closeout|issue-4|pr-3-merge-evidence|master-plan|issue-2-session-runtime|build-err|agos-optional-use-or-bypass|no-agos-optimization|no-source|no-actions|no-ruleset-change|no-release|no-merge
mutation_intent: docs
executor_enforcement: project-native-docs-plus-github-issue-pr
allowed_operations:
  - 新增 Issue #4 计划、Session Plan、Runtime Workflow 与 Issue #2 closeout 报告
  - 更新 Master Plan、README、总架构、Issue #2 Session Plan 与 Runtime Workflow
  - 更新 build.md 与 err.md 的验证和排错证据
  - Fresh 验证后提交、正常推送并创建关联 Issue #4 的开放 PR
  - PR 创建后回写真实 PR URL并再次提交推送
scope_boundary: 只写回 Issue #2 / PR #3 的已发生事实和当前 Gate 1 控制面；不导入源码、不创建 Actions、不修改 Ruleset、不发布、不合并本 closeout PR。
selected_business_path: architecture-governance
delivery_contract: agos.issue-pr-merge.v1
delivery_contract_role: optional-format-compatibility-only
tracking_issue_ref: https://github.com/nonononull/inputcodex/issues/4
review_strategy: 主线程完成结构与事实自审，GitHub PR 由项目所有者审阅；所有 Review 对话必须完成根因、处理和验证闭环。
ci_expectation: 仓库当前没有 Actions 或 required checks；以 Fresh 本地验证和 GitHub API 事实为证据，不把 Checks 0 伪装成 CI 通过。
merge_policy: 本 closeout PR 保持 OPEN，只有项目所有者再次明确授权且全部 Review 对话闭环后才允许 Squash Merge；禁止 Merge Commit、Rebase Merge、Force Push 和直接写 main。
closeout_ref: pending:issue-4-pr-merge-closeout

## Approved Decision

- Decision: 使用独立 Issue `#4`、分支 `codex/issue-4-gate-1-closeout` 和关联 PR 回写 Issue `#2` / PR `#3` 的最终交付证据。
- Decision: Master Plan、README、总架构、Issue `#2` Session Plan 与 Runtime Workflow必须移除“PR #3 仍 OPEN”的过期状态。
- Decision: `review_ref`、`ci_ref`、`merge_ref`、Issue 关闭、Squash 单父提交、tree 一致和分支删除必须有可重复核验的证据。
- Decision: 本任务继续保持 docs-only；禁止源码、Cargo、Rust/Iced、UI、Actions、Ruleset、required checks 与发布变更。
- Decision: closeout PR 创建后保持开放，未经项目所有者再次明确授权不得合并。
- Decision: AGOS 可用且适用时只作为可选外部辅助；不可用、未登记、`needs-input` 或异常时记录并绕过，不构成本项目门禁，且禁止在本仓任务中修改或优化 AGOS。
- Reason: PR `#3` 已于 `2026-07-21T13:15:51Z` Squash Merge，Issue `#2` 已关闭，但 `main` 上的控制文档仍保留合并前状态。
- Scope boundary: 只修正 closeout 事实和控制面索引，不进入 Gate 2，也不补做未获批准的 GitHub 配置。

## Brainstorming

```yaml
brainstorming:
  level: standard
  superpowers_skill: superpowers:brainstorming
  user_decision: 用户已明确批准“现在同步”，并指定独立 closeout Issue/PR、合并证据和最新 Master Plan；范围相对既有 closeout 方案未变化。
  proposal_mode: not-required
  actual_agent_count: 0
  fallback_reason: 本任务是已批准事实回写，不产生新架构或产品行为；开发者规则也禁止在用户未要求时启动子 agent。
  allowed_operations:
    - docs
    - git-current-branch
    - github-linked-pr
  forbidden_ops:
    - source-edit
    - github-actions-write
    - ruleset-write
    - release-publish
    - merge-current-pr
  implementation_freeze_status: released-for-docs-only
  decision_rationale: 最小变更即可消除过期状态并建立完整审计链，不需要重新设计 Gate 1 架构。
```

## Change Contract

```yaml
change_contract:
  mutation_intent: docs
  target_contract:
    owner: 项目所有者
    expected_behavior: 项目控制面准确反映 Issue #2 CLOSED、PR #3 MERGED、Squash 证据与 Gate 1 剩余工作，并通过独立 Issue #4 PR 交付。
    evidence_refs:
      - https://github.com/nonononull/inputcodex/issues/4
      - https://github.com/nonononull/inputcodex/pull/3
      - https://github.com/nonononull/inputcodex/commit/0e11375997ff10fdc0c233b31c8468af2d9a4f44
      - docs/reports/issue-2-architecture-governance-closeout.md
  preserved_invariants:
    - name: 仓库仍无应用源码和 Workflow
      owner: 项目所有者
      baseline_ref: git:0e11375997ff10fdc0c233b31c8468af2d9a4f44
      regression_ref: rg --files
    - name: main Ruleset 参数不变
      owner: 项目所有者
      baseline_ref: https://github.com/nonononull/inputcodex/rules/19395456
      regression_ref: gh api repos/nonononull/inputcodex/rulesets/19395456
    - name: 软件名称固定为 inputcodex
      owner: 项目所有者
      baseline_ref: AGENTS.md
      regression_ref: rg -n "inputcodex" README.md AGENTS.md docs/plans docs/workflows docs/reports
  adjacent_surfaces:
    - name: README 与 Master Plan
      why_adjacent: 两者是用户和执行器读取当前状态的入口，过期 PR 状态会直接误导下一任务。
      risk: 误把已完成 Review/Merge 当成待办，或错误进入 Gate 2。
      owner: 项目所有者
    - name: Issue #2 Session Plan 与 Runtime Workflow
      why_adjacent: 原任务 closeout 字段是 delivery contract 的历史证据真源。
      risk: pending 字段会破坏 Issue→PR→Merge 审计链。
      owner: 项目所有者
    - name: build.md 与 err.md
      why_adjacent: closeout 验证和 Git 同步恢复必须可重复执行与查重。
      risk: 继续执行“Issue #2 必须 OPEN”的旧命令会产生假失败。
      owner: 项目所有者
  historical_state_refs:
    - docs/plans/sessions/2026-07-21-issue-2-architecture-governance.md
    - docs/workflows/2026-07-21-issue-2-architecture-governance-runtime.md
    - docs/reports/2026-07-21-main-protection-rollout.md
    - git:0e11375997ff10fdc0c233b31c8468af2d9a4f44
  stale_verdict_invalidation_refs:
    - PR #3、Issue #2、Ruleset 19395456 或 merge commit 的 GitHub API 事实变化时重新核验
    - 用户扩大到源码、Actions、Ruleset、发布或合并本 PR 时重新进入决策门
  regression_checks:
    - surface: Session Plan 结构
      command_or_evidence_ref: verify-session-plan.ps1
      expected_result: Issue #2 与 Issue #4 Session Plan 均输出 SESSION_PLAN_VERIFY_OK
    - surface: Master Plan 索引
      command_or_evidence_ref: verify-master-plan-index.ps1
      expected_result: MASTER_PLAN_INDEX_VERIFY_OK
    - surface: GitHub closeout 事实
      command_or_evidence_ref: gh issue view 2; gh pr view 3; gh api graphql reviewThreads
      expected_result: Issue #2 CLOSED、PR #3 MERGED、Checks 0、Review 对话总数与未解决数均为 0
    - surface: Squash 与 tree 一致
      command_or_evidence_ref: git rev-list --parents -n 1; git show -s --format=%T
      expected_result: merge commit 只有一个父节点，merge tree 与 PR Head tree 相同
    - surface: Ruleset 漂移
      command_or_evidence_ref: gh api repos/nonononull/inputcodex/rulesets/19395456
      expected_result: active、仅 main、无 bypass、required approvals 0、Review 对话必解、只允许 squash
    - surface: 变更范围
      command_or_evidence_ref: git diff --check; git diff --cached --check; rg --files
      expected_result: 仅允许的 Markdown 文件变化，且无源码、Cargo 或 Workflow
  sibling_regression_guard:
    status: passed
    closeout_rule: passed-or-blocked-before-done
    exception_ref: none
  protected_feature_replay:
    status: passed
    not_applicable_reason: 当前无产品功能；回放 Gate 0/1 文档、许可证、空源码和 main Ruleset 不变量。
    known_good_features:
      - feature: Issue #2 / PR #3 已合并治理基线
        owner: 项目所有者
        baseline_evidence_ref: https://github.com/nonononull/inputcodex/commit/0e11375997ff10fdc0c233b31c8468af2d9a4f44
        post_change_replay_plan_ref: 核对 PR、Issue、Review、Checks、tree、分支与 Ruleset，并确认无源码或 Workflow
        post_change_replay_ref: local:2026-07-21:PR3_MERGE_EVIDENCE=passed;pr:5
        expected_result: Issue #2 / PR #3 的已合并事实保持成立，Gate 0/1 约束和 Ruleset 未回退
        actual_result: Issue #2 / PR #3 的已合并事实保持成立，Gate 0/1 约束和 Ruleset 未回退；GitHub API、Git 对象与文件清单预检均符合预期。
        owner_visible_status: passed
        regression_status: passed
    forbidden_ops_until_replay:
      - claim-done
      - create-pr
```

## Local Knowledge Lookup

```yaml
local_knowledge_lookup:
  gbrain_queries:
    - inputcodex GitHub Issue PR Squash Merge closeout 证据 Master Plan Git snapshot governance
  gbrain_result: no-results
  optional_external_vault_refs:
    - D:/Android_source/ai-growth-os/components/vault/08-Skills/AI-Growth-OS.md
    - D:/Android_source/ai-growth-os/components/vault/07-Workflows/Core/AI-Growth-OS-Brainstorming-Gate-And-Session-Plan.md
  optional_external_rules_refs:
    - D:/Android_source/ai-growth-os/components/rules/rules/workflows/ai-growth-os-auto-application.md
    - D:/Android_source/ai-growth-os/components/rules/rules/workflows/ai-growth-os-brainstorming-gate.md
    - D:/Android_source/ai-growth-os/components/rules/rules/workflows/ai-growth-os-runtime-workflow.md
    - D:/Android_source/ai-growth-os/components/rules/rules/workflows/git-snapshot-governance.md
  project_refs:
    - AGENTS.md
    - README.md
    - build.md
    - err.md
    - docs/plans/PROJECT-MASTER-PLAN.md
    - docs/plans/sessions/2026-07-21-issue-2-architecture-governance.md
    - docs/workflows/2026-07-21-issue-2-architecture-governance-runtime.md
  missing_coverage:
    - GBrain 未命中 inputcodex closeout 专项资料，事实以本仓文档、Git 对象和 GitHub API 为准。
    - AGOS 全局 registry 未登记 inputcodex task/business path；该外部状态已记录并绕过，不影响本项目交付，也不触发任何跨仓修复。
    - 仓库没有 .codegraph，按项目规则不擅自初始化。
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
    evidence: 当前为普通仓库检出，但已在专用分支 codex/issue-4-gate-1-closeout；用户已要求继续当前 Git 流程，不额外创建 worktree。
  planning_execution:
    writing_skill: superpowers:writing-plans
    executing_skill: superpowers:executing-plans
    subagent_skill: superpowers:subagent-driven-development
    plan_control_plane: inputcodex project-native docs; AGOS optional external assistance only
  test_driven_development:
    skill: superpowers:test-driven-development
    cycle: not-applicable-docs-only
    evidence: 不写产品行为代码；以事实预检、文档验证和最终 Fresh 重验证代替代码 RED/GREEN/REFACTOR。
  verification_before_completion:
    skill: superpowers:verification-before-completion
    evidence: 提交、推送、创建 PR 和完成声明前都运行 Fresh 验证。
  systematic_debugging:
    skill: superpowers:systematic-debugging
    trigger: GitHub 事实、验证脚本、Git 同步或补丁工具出现异常。
  code_review:
    request_skill: superpowers:requesting-code-review
    receive_skill: superpowers:receiving-code-review
    evidence: 主线程自审后创建开放 PR，由项目所有者 Review；不启动未经要求的子 agent。
  finishing_branch:
    skill: superpowers:finishing-a-development-branch
    evidence: 用户已选择推送并创建 PR；保持分支与 PR 开放，不自动合并或清理。
  evidence_writeback:
    target: build.md, session plan, runtime workflow, closeout report, PR body
    docs_superpowers_boundary: docs/superpowers remains archive-only, not the active control plane
```

## External AGOS Observation

```yaml
external_agos_observation:
  command_date: 2026-07-21
  role: optional-external-assistance
  observed_status: needs-input
  task_registration_status: unregistered
  project_git_foundation_status: ready
  project_entry_doc_foundation_status: ready
  local_knowledge_lookup_status: ready
  project_gate_effect: none
  bypass_status: applied
  project_native_continuation_status: allowed
  strict_runtime_validation_claimed: false
  external_mutation_permission: forbidden
  handling: 保留 needs-input 作为历史外部观测；本任务以 Issue #4、当前分支、项目原生文档、Git/GitHub 事实和项目所有者批准为权威证据，绕过外部门禁，不修改、修复或优化 AGOS。
  source_implementation_admission: forbidden
```

## Master Plan

```yaml
path: docs/plans/PROJECT-MASTER-PLAN.md
update_required: true
update_summary: 将 active_task 切换到 Issue #4，记录 Issue #2 / PR #3 已完成，并保留 Gate 1 模板与标签待办。
```

## Runtime Workflow

```yaml
path: docs/workflows/2026-07-21-issue-4-gate-1-closeout-runtime.md
workflow_nodes:
  - startup
  - knowledge-prep
  - plan
  - execute
  - verify
  - sync
  - pr
verification_commands:
  - verify-session-plan.ps1
  - verify-master-plan-index.ps1
  - verify-protected-feature-replay.ps1 -RequireProtectedReplay
  - verify-git-snapshot-governance.ps1 -Checkpoint -ReportOnly
  - git diff --check
  - git diff --cached --check
  - gh issue view 2
  - gh issue view 4
  - gh pr view 3
  - gh api graphql reviewThreads
  - gh api repos/nonononull/inputcodex/rulesets/19395456
```

## Delivery Governance

```yaml
delivery_contract: agos.issue-pr-merge.v1
tracking_issue_ref: https://github.com/nonononull/inputcodex/issues/4
branch_ref: codex/issue-4-gate-1-closeout
review_ref: pending:project-owner-review-on-pr-5
pr_ref: https://github.com/nonononull/inputcodex/pull/5
ci_ref: not-configured:pr-5-checks-0-2026-07-21
merge_ref: none:issue-4-pr-must-remain-open
review_strategy: Fresh 本地验证与事实自审后创建非 Draft PR；所有 Review 对话必须先完成根因、处理与验证闭环。
ci_expectation: 不创建 CI；PR 正文回写本地验证命令、结果和 Checks 边界。
merge_policy: PR 正文包含 Closes #4；未经项目所有者再次明确授权不得合并，获批后也只能 Squash Merge。
```

## Completion Criteria

- Issue `#2` / PR `#3` closeout 报告、旧 Session Plan 与 Runtime Workflow、Master Plan 和 README 语义一致。
- `build.md` 可重复验证 CLOSED/MERGED、0 Review 对话、0 Checks、单父提交、tree 一致、旧分支删除和 Ruleset 不漂移。
- `err.md` 记录 Git HTTPS 失败后的 GitHub API 签名提交精确重建，以及 quiet 原生命令必须检查 `$LASTEXITCODE`。
- 当前分支已同步到远端并创建包含 `Closes #4` 的开放 PR `#5`；真实 PR URL 已回写，等待最终验证和追加提交。
- PR `#5` 保持 OPEN、非 Draft、`CLEAN`，Checks 与未解决 Review 对话均为 `0`；本任务不自动合并。
