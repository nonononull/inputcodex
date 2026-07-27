# Issue #63：预算 CI Observation Session Plan

```yaml
schema_version: agos.session-plan.v1
architecture_contract_version: agos.brainstorming-gate.v1
task_id: issue-63
work_class: standard
task_summary: 在现有 Performance Baseline 增加非 required approved-observation 双平台观察模式
project_root: C:/Users/dashuai/Documents/inputcodex-worktrees/issue-63-performance-budget-ci-observation
trigger_source: PR #72 暴露旧 Evidence input_sha256 对合法 parity 变化的确定性阻断
tracking_issue_ref: https://github.com/nonononull/inputcodex/issues/63
baseline_ref: 15e91708b41548f523e26ede4c7ca4de41badf77
branch: codex/issue-63-performance-budget-ci-observation
implementation_plan_ref: docs/plans/2026-07-26-issue-63-performance-budget-ci-observation.md
runtime_workflow_ref: docs/workflows/2026-07-26-issue-63-performance-budget-ci-observation-runtime.md
decision_status: approved
approval_source: direct-user
approved_decision_ref: session-plan:issue-63#decision
scope_approval_status: approved
scope_approval_ref: https://github.com/nonononull/inputcodex/issues/63#issuecomment-5087488361
scope_hash: sha256:d5eb57c1b93dc2b7acc47ba78c8f514af2a2c98e8661df389774713a7b47d8dc
allowed_operations: exact-thirteen-path-observation-ci, project-native-tdd, local-lightweight-verification, git-checkpoint, normal-commit, normal-push, non-draft-pr, review-ci, hosted-windows-macos-observation
mutation_intent: config
business_mutation_intent: approved-observation-reporting-only
executor_enforcement: no-budget-value-formula-ruleset-required-check-product-gate5-upstream-agos-mutation, normal-push-only, squash-merge-only
delivery_contract: agos.issue-pr-merge.v1
review_strategy: final-head changed-surface review plus all GitHub review conversations resolved with root-cause evidence
ci_expectation: standard CI green and Performance Baseline observation completes on Windows/macos without success artifacts
merge_policy: squash-only, no-force-push, never-delete-main
scope_boundary: approved thirteen paths only; final squash merge requires separate owner authorization
closeout_ref: docs/reports/issue-63-performance-budget-ci-observation.md
```

## Brainstorming

```yaml
brainstorming:
  superpowers_skill: superpowers:brainstorming
  selected_business_path: inputcodex.performance-budget-ci-observation
  selected_option: 方案 A，扩展现有 Performance Baseline 为第三种 observation 模式
  user_decision: 项目所有者批准十三路径、精确 scope_hash、TDD、普通推送、非 Draft PR、Review/CI 与双平台真实 observation
  approved_decision_evidence:
    - https://github.com/nonononull/inputcodex/issues/63
    - https://github.com/nonononull/inputcodex/issues/63#issuecomment-5087488361
  rejected_options:
    - 新建重复双平台 Workflow
    - 把预算观察加入主 CI 或 Ruleset
  verification_commands:
    - pwsh -NoProfile -File scripts/performance/Test-InputcodexBudgetObservation.ps1 -RepositoryRoot .
    - pwsh -NoProfile -File scripts/ci/Test-CiScripts.ps1
    - pwsh -NoProfile -File scripts/ci/Verify-RepositoryPolicy.ps1 -RepositoryRoot .
    - git diff --check
  final_merge_authorization: pending
```

## Local Knowledge Lookup

```yaml
local_knowledge_lookup:
  gbrain_queries:
    - 查询 approved-observation 预算、Performance Baseline Evidence/measure 合同、Issue #69 哈希边界与 PR #72 input_sha256 失败证据；无可用 inputcodex 图索引，使用项目原生文件、Git 历史与 GitHub Issue/PR 证据
  vault_refs:
    - D:/Android_source/ai-growth-os/components/vault/07-Workflows/Core/AI-Growth-OS-Brainstorming-Gate-And-Session-Plan.md
    - D:/Android_source/ai-growth-os/components/vault/08-Skills/AI-Growth-OS.md
  rules_refs:
    - D:/Android_source/ai-growth-os/components/rules/rules/workflows/ai-growth-os-auto-application.md
    - D:/Android_source/ai-growth-os/components/rules/rules/workflows/ai-growth-os-brainstorming-gate.md
  project_refs:
    - AGENTS.md
    - README.md
    - build.md
    - err.md
    - docs/plans/PROJECT-MASTER-PLAN.md
    - benchmarks/budgets/issue-59-approved-observation.json
    - scripts/performance/Build-InputcodexBudgetApproval.ps1
    - scripts/performance/Test-InputcodexBudgetApproval.ps1
    - scripts/performance/Invoke-InputcodexBaseline.ps1
    - scripts/performance/Test-InputcodexBaseline.ps1
  github_refs:
    - Issue #59 / PR #60
    - Issue #63
    - Issue #65 / PR #72
    - Issue #69 / PR #70
  graph_status: no-inputcodex-index, project-native-evidence-used
  agos_status: bypassed-needs-input-unregistered
  external_boundary: AGOS 仅可只读报告，不得修改或优化
  agos_evidence: Default Entry 于本机返回 needs-input/unregistered；项目 Git 与入口文档 ready，按 inputcodex 规则绕过
  missing_coverage:
    - 没有已索引的 inputcodex GitNexus/CodeGraph；以预算 JSON、性能脚本、Issue #59/#63/#65/#69、PR #72 与 Git 历史为权威证据
  external_gap:
    - AGOS 未登记 issue-63 且 protected-feature-replay 处于外部 intake 状态；按项目规则绕过，不修改 AGOS
```

## Change Contract

```yaml
change_contract:
  mutation_intent: config
  target_contract:
    expected_behavior: 自动 PR/push 双平台生成临时结果并只读输出 approved-observation 分类，数值越界不阻断，证据错误阻断
    evidence_refs:
      - https://github.com/nonononull/inputcodex/issues/63
      - https://github.com/nonononull/inputcodex/pull/72#issuecomment-5087464590
      - benchmarks/budgets/issue-59-approved-observation.json
  preserved_invariants:
    - name: 批准预算内容保持只读
      baseline_ref: benchmarks/budgets/issue-59-approved-observation.json sha256:be07138908cd411925db963718b71062060f4fd4a50b910ab5d5f25f88d4ebe5
      regression_ref: 提交前后归一化 SHA-256 必须一致，budget_ci_enabled=false 且 gate_5_unlocked=false
    - name: 产品与 Ruleset 零变化
      baseline_ref: 15e91708b41548f523e26ede4c7ca4de41badf77
      regression_ref: git diff 只允许十三路径且不含产品、预算、Ruleset、upstream 或 Gate 5
  adjacent_surfaces:
    - name: PR #72 功能目录重新审计
      why_adjacent: #63 合并后需重触发同一 #72 Final Head，使自动 observation 替代确定性失败的旧 Evidence
    - name: Issue #32 历史性能 Evidence
      why_adjacent: 历史结果继续只读保存，不得为当前 parity 输入改写 input_sha256
    - name: Issue #59 approved-observation 预算
      why_adjacent: 只读消费已批准阈值，不得重新计算数值或公式
  historical_state_refs:
    - Issue #32 / PR #49 的双平台性能基线与历史 Evidence
    - Issue #59 / PR #60 的 approved-observation 数值
    - Issue #69 / PR #70 的 implementation_sha256 边界修复
  stale_verdict_invalidation_refs:
    - Workflow、观察器或测试最后写入后的旧本地验证、旧 Review 与旧 CI 均失效
    - PR #72 Head、预算 JSON 哈希或十三路径发生变化时当前结论失效
  regression_checks:
    - surface: 预算观察器行为
      command_or_evidence_ref: pwsh -NoProfile -File scripts/performance/Test-InputcodexBudgetObservation.ps1 -RepositoryRoot .
      expected_result: 四种分类通过且合同错误被拒绝
    - surface: CI Workflow 合同
      command_or_evidence_ref: pwsh -NoProfile -File scripts/ci/Test-CiScripts.ps1
      expected_result: 自动 observation、非 required、双 hosted Runner、成功零 Artifact 合同通过
    - surface: 仓库政策与精确范围
      command_or_evidence_ref: Verify-RepositoryPolicy.ps1、scope_hash 复算与 git diff --check
      expected_result: 零违规且只含十三路径
  sibling_regression_guard:
    status: pending
    closeout_rule: passed-or-blocked-before-done
  protected_feature_replay:
    status: pending
    known_good_features:
      - feature: 手工 evidence 与 measure 模式
        owner: .github/workflows/performance-baseline.yml
        baseline_evidence_ref: Issue #55 与现有 Workflow 合同
        post_change_replay_plan_ref: scripts/ci/Test-CiScripts.ps1 与 PR Head Performance Baseline
        post_change_replay_ref: CI_CONTRACT_GREEN passed=35 at commit b465c660d0401ff4bff37673671147aa6b513e1a
        expected_result: 手工 evidence/measure 继续可选且默认仍为 evidence
        owner_visible_status: pending
        regression_status: pending
      - feature: approved-observation 预算只读与 Gate 5 锁定
        owner: benchmarks/budgets/issue-59-approved-observation.json
        baseline_evidence_ref: Issue #59 / PR #60
        post_change_replay_plan_ref: 预算哈希复算、观察器测试与仓库政策
        post_change_replay_ref: budget hash unchanged; BUDGET_OBSERVATION_GREEN passed=12; repository policy violation_count=0
        expected_result: 预算字节、数值、公式、budget_ci_enabled=false 与 gate_5_unlocked=false 均不变
        owner_visible_status: pending
        regression_status: pending
    forbidden_ops_until_replay:
      - 修改预算 JSON、历史 Evidence、预算公式或阈值
      - 修改 Ruleset、required checks、产品行为、Gate 5、upstream 或 AGOS
      - force push
      - squash merge without separate owner authorization
      - claim-done
```

## 基线

- 本机时间：`2026-07-27 13:12:19 +08:00`，来自 Windows `Get-Date`。
- 基线：`origin/main=15e91708b41548f523e26ede4c7ca4de41badf77`。
- Issue `#63` 批准评论已回读，无控制字符。
- 范围复算：十三路径，`scope_hash=sha256:d5eb57c1b93dc2b7acc47ba78c8f514af2a2c98e8661df389774713a7b47d8dc`。
