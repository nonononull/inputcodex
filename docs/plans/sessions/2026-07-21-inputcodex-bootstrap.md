# Session Plan：inputcodex 仓库筹备

schema_version: agos.session-plan.v1
architecture_contract_version: agos.brainstorming-gate.v1
task_id: 2026-07-21-inputcodex-bootstrap
work_class: standard
task_status: verified
task_summary: 建立 inputcodex 公开仓库与项目治理基线，不导入应用源码。
project_root: C:/Users/dashuai/Documents/inputcodex
trigger_source: 用户要求先完成准备工作再讨论重构方案
decision_status: approved
approval_source: inherited-user-instruction
approved_decision_ref: session-plan:2026-07-21-inputcodex-bootstrap#decision
mutation_intent: docs
allowed_operations:
  - 创建项目治理文档
  - 关联本地仓库与 GitHub 公开仓库
  - 创建筹备 Issue
  - 提交并推送准备阶段材料
scope_boundary: 仅完成仓库、许可证、文档和跟踪入口，不导入或实现应用源码。
selected_business_path: project-bootstrap
verification_commands:
  - git diff --check
  - git status --short --branch
  - gh repo view nonononull/inputcodex --json nameWithOwner,visibility,url,defaultBranchRef,licenseInfo
  - verify-session-plan.ps1
  - verify-git-snapshot-governance.ps1 -Checkpoint -ReportOnly
closeout_ref: docs/reports/2026-07-21-bootstrap-closeout.md

## Approved Decision

- Decision: 先完成 `inputcodex` 本地与公开仓库准备，再讨论架构。
- Reason: 用户明确要求准备工作先行，并在范围未变化后回复“继续”。
- Scope boundary: 不导入两份参考仓库源码，不实现 UI、功能或性能优化。
- Rejected options: 未审计源码前直接重写；直接把半成品整体搬入新仓库。

## Brainstorming

```yaml
level: standard
proposal_mode: not-required
fallback_reason: 准备范围由用户直接指定且不包含架构实现
superpowers_skill: superpowers:brainstorming
user_decision: 用户批准创建公开仓库、项目治理骨架和筹备 Issue，随后再讨论方案。
decision_reason: 先建立可逆、可审计的准备基线。
rejected_options:
  - 当前阶段导入参考源码
  - 当前阶段选定最终技术栈
```

## Change Contract

```yaml
change_contract:
  mutation_intent: docs
  target_contract:
    owner: 项目所有者
    expected_behavior: 建立公开仓库和可审计的准备文档，同时保持应用源码为空。
    evidence_refs:
      - https://github.com/nonononull/inputcodex/issues/1
  preserved_invariants:
    - name: 空应用源码基线
      owner: 项目所有者
      baseline_ref: git:bd3b01a4
      regression_ref: git diff --name-only origin/main...HEAD
  adjacent_surfaces:
    - name: GNU AGPLv3 许可证
      why_adjacent: 两份参考项目均采用该许可证，后续可能形成衍生作品。
      risk: 许可证不一致会产生合规风险。
      owner: 项目所有者
  historical_state_refs:
    - BigPizzaV3/CodexPlusPlus@main
    - zsr131550/CodexPlusPlus@main
  stale_verdict_invalidation_refs:
    - 任一参考仓库许可证或默认分支变化时重新核验
  regression_checks:
    - surface: 文档格式
      command_or_evidence_ref: git diff --check
      expected_result: 退出码为 0
    - surface: 空应用源码基线
      command_or_evidence_ref: rg --files
      expected_result: 仅出现许可证和筹备文档
    - surface: GitHub 仓库公开性
      command_or_evidence_ref: gh repo view nonononull/inputcodex
      expected_result: visibility 为 PUBLIC
  sibling_regression_guard:
    status: passed
    closeout_rule: passed-or-blocked-before-done
    exception_ref: none
  protected_feature_replay:
    status: passed
    not_applicable_reason: 当前没有产品功能，使用仓库许可证和空源码状态作为受保护基线。
    known_good_features:
      - feature: GitHub 初始许可证与空源码状态
        owner: 项目所有者
        baseline_evidence_ref: git:bd3b01a4
        post_change_replay_plan_ref: 提交前检查 LICENSE 无差异且仓库没有应用源码
        post_change_replay_ref: docs/reports/2026-07-21-bootstrap-closeout.md
        expected_result: LICENSE 无差异，新增文件全部为筹备文档
        actual_result: LICENSE 无差异，仓库仅包含许可证和筹备文档。
        owner_visible_status: passed
        regression_status: passed
    forbidden_ops_until_replay:
      - claim-done
```

## Local Knowledge Lookup

```yaml
local_knowledge_lookup:
  gbrain_queries:
    - inputcodex CodexPlusPlus 桌面应用重构 性能 卡顿 功能加载 广告
  vault_refs:
    - D:/Android_source/ai-growth-os/components/vault/08-Skills/AI-Growth-OS.md
    - D:/Android_source/ai-growth-os/components/vault/07-Workflows/Core/AI-Growth-OS-Brainstorming-Gate-And-Session-Plan.md
  rules_refs:
    - D:/Android_source/ai-growth-os/components/rules/rules/workflows/ai-growth-os-auto-application.md
    - D:/Android_source/ai-growth-os/components/rules/rules/workflows/ai-growth-os-brainstorming-gate.md
  project_refs:
    - 启动时仓库仅有 .git 和远端初始 LICENSE
    - https://github.com/BigPizzaV3/CodexPlusPlus
    - https://github.com/zsr131550/CodexPlusPlus
  missing_coverage:
    - 本地知识库未命中 CodexPlusPlus 专项架构与性能资料，下一阶段以源码审计和实测补齐。
```

## Superpowers Method Discipline

```yaml
superpowers_method_discipline:
  upstream_superpowers_ref: https://github.com/obra/superpowers
  local_superpowers_state: unknown
  using_superpowers: superpowers:using-superpowers
  brainstorming: superpowers:brainstorming
  worktree_isolation:
    skill: superpowers:using-git-worktrees
    evidence: 新建空仓准备阶段不需要额外 worktree
  test_driven_development:
    skill: superpowers:test-driven-development
    cycle: RED/GREEN/REFACTOR
    evidence: 当前仅文档和 Git 准备，不适用源码 TDD
  verification_before_completion:
    skill: superpowers:verification-before-completion
    evidence: 完成前运行 Git、GitHub、会话计划与快照验证
  systematic_debugging:
    skill: superpowers:systematic-debugging
    trigger: 任一验证失败或出现异常行为
  code_review:
    request_skill: superpowers:requesting-code-review
    receive_skill: superpowers:receiving-code-review
    evidence: 当前准备阶段采用本地文档检查
  planning_execution:
    writing_skill: superpowers:writing-plans
    executing_skill: superpowers:executing-plans
    subagent_skill: superpowers:subagent-driven-development
    plan_control_plane: project-native AGOS control docs
  finishing_branch:
    skill: superpowers:finishing-a-development-branch
    evidence: 直接初始化 main，不建立功能分支
  evidence_writeback:
    target: build.md, session plan, runtime workflow, closeout report
    docs_superpowers_boundary: docs/superpowers remains archive-only, not the active control plane
```

## Master Plan

```yaml
path: docs/plans/PROJECT-MASTER-PLAN.md
update_required: true
update_summary: 建立 Gate 0 至 Gate 4 的项目级阶段边界。
```

## Runtime Workflow

```yaml
path: docs/workflows/2026-07-21-inputcodex-bootstrap-runtime.md
session_plan_ref: docs/plans/sessions/2026-07-21-inputcodex-bootstrap.md
approved_decision_ref: session-plan:2026-07-21-inputcodex-bootstrap#decision
selected_business_path: project-bootstrap
workflow_nodes:
  - 规则与知识查询
  - 参考仓库元数据核验
  - GitHub 公开仓库与 Issue 创建
  - 项目原生文档写入
  - 提交前验证与推送
subagent_roles:
  - none
skill_tree_nodes:
  - superpowers:using-superpowers
  - superpowers:brainstorming
  - superpowers:writing-plans
  - superpowers:systematic-debugging
  - superpowers:verification-before-completion
  - karpathy-guidelines
stop_gates:
  - 用户改变准备范围
  - 参考仓库许可证核验失败
verification_commands:
  - git diff --check
  - verify-session-plan.ps1
  - verify-git-snapshot-governance.ps1 -Checkpoint -ReportOnly
```

## Delivery Governance

```yaml
tracking_issue_ref: https://github.com/nonononull/inputcodex/issues/1
review_strategy: 本地机器校验与人工差异审阅
ci_expectation: 当前无可构建源码，不启用 CI
merge_policy: 准备基线直接提交并推送 main
```
