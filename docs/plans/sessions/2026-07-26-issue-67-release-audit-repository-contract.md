# Issue #67：解耦固定目录证据与活动快照状态 Session Plan

```yaml
schema_version: agos.session-plan.v1
architecture_contract_version: agos.brainstorming-gate.v1
task_id: issue-67
work_class: standard
task_summary: 修复仓库实例测试把 v1.2.42 历史 current 状态错误提升为永久不变量的问题
project_root: C:/Users/dashuai/Documents/inputcodex
trigger_source: PR #66 CI Run 30202056781 三平台同构失败
tracking_issue_ref: https://github.com/nonononull/inputcodex/issues/67
baseline_ref: a2e5e5a6728200739a4acb85042ba7831ac6295b
branch: codex/issue-67-release-audit-repository-contract
implementation_plan_ref: docs/plans/2026-07-26-issue-67-release-audit-repository-contract.md
runtime_workflow_ref: docs/workflows/2026-07-26-issue-67-release-audit-repository-contract-runtime.md
decision_status: approved
approval_source: direct-user
approved_decision_ref: session-plan:issue-67#decision
scope_approval_status: approved
scope_approval_ref: https://github.com/nonononull/inputcodex/issues/67#issuecomment-5083523560
scope_hash: sha256:b6295dabd39f0cba7c4f13bd3d35ff8b0433e1fb95de98e6fc5f2cf0c1eb6b9f
allowed_operations: exact-six-path-edit, local-red-green, temporary-pr66-overlay, normal-commit, normal-push, non-draft-pr, review-ci
mutation_intent: control-plane
executor_enforcement: exact-six-path-set, no-pr66-mutation, fresh-verification, normal-push-only, squash-merge-only
delivery_contract: agos.issue-pr-merge.v1
review_strategy: final-head changed-surface manual review plus all GitHub review conversations resolved with root-cause evidence
ci_expectation: standard GitHub-hosted Linux/Windows/macOS CI and release-audit must be green on final PR Head
merge_policy: squash-only, no-force-push, never-delete-main
scope_boundary: approved six paths only; final squash merge requires separate owner authorization
closeout_ref: pending
```

## Local Knowledge Lookup

```yaml
local_knowledge_lookup:
  gbrain_queries:
    - GitNexus resources 已暴露但 indexed repositories=[]，没有可查询的 inputcodex 图谱；按 AGENTS.md 禁止擅自初始化 CodeGraph。
  vault_refs:
    - D:/Android_source/ai-growth-os/components/vault/07-Workflows/Core/AI-Growth-OS-Brainstorming-Gate-And-Session-Plan.md
    - D:/Android_source/ai-growth-os/components/vault/08-Skills/AI-Growth-OS.md
  rules_refs:
    - D:/Android_source/ai-growth-os/components/rules/rules/workflows/ai-growth-os-auto-application.md
    - D:/Android_source/ai-growth-os/components/rules/rules/workflows/ai-growth-os-brainstorming-gate.md
    - D:/Android_source/ai-growth-os/components/rules/rules/quality/testing.md
  project_refs:
    - AGENTS.md
    - README.md
    - build.md
    - err.md
    - docs/plans/PROJECT-MASTER-PLAN.md
    - docs/plans/2026-07-24-issue-41-ci-contract-decoupling.md
  missing_coverage:
    - 没有独立知识卡覆盖“目录重新审计测试在下一 Release 后再次固定 current”；以 CI RED、Git blame、Issue #41 历史和现有状态机测试为权威证据。
```

## Superpowers Method Discipline

```yaml
superpowers_method_discipline:
  upstream_superpowers_ref: https://github.com/obra/superpowers
  local_superpowers_state: loaded-from-C:/Users/dashuai/.codex/superpowers
  using_superpowers: superpowers:using-superpowers
  brainstorming: superpowers:brainstorming
  worktree_isolation:
    skill: superpowers:using-git-worktrees
    evidence: C:/Users/dashuai/Documents/inputcodex-worktrees/issue-67-release-audit-repository-contract 从 origin/main 隔离创建。
  planning_execution:
    writing_skill: superpowers:writing-plans
    executing_skill: superpowers:executing-plans
    plan_control_plane: project-native AGOS control docs
  test_driven_development:
    skill: superpowers:test-driven-development
    cycle: PR #66 RED -> 最小测试合同 GREEN -> main/current 与 PR #66/stale 双回放
  verification_before_completion:
    skill: superpowers:verification-before-completion
    evidence: 提交、推送、PR 与完成声明前运行本计划列出的新鲜验证。
  systematic_debugging:
    skill: superpowers:systematic-debugging
    trigger: PR #66 三平台同构测试失败
  code_review:
    request_skill: superpowers:requesting-code-review
    receive_skill: superpowers:receiving-code-review
    evidence: 不启用未获用户请求的 subagent；由当前执行器完成 changed-surface 自审并处理全部 GitHub Review 对话。
  finishing_branch:
    skill: superpowers:finishing-a-development-branch
    evidence: 最终 Head 的验证、Review 与 CI 全绿后才请求 Squash Merge 授权。
  evidence_writeback:
    target: session plan, runtime workflow, err.md, closeout report
    docs_superpowers_boundary: docs/superpowers remains archive-only, not the active control plane
delivery_evidence:
  tracking_issue_ref: https://github.com/nonononull/inputcodex/issues/67
  review_ref: 提交前 changed-surface 自审无发现；最终 PR Head 的 GitHub Review 对话检查仍待执行
  pr_ref: pending
  ci_ref: pending
  merge_ref: pending
master_plan:
  path: docs/plans/PROJECT-MASTER-PLAN.md
  update_required: false
  update_summary: 本修复只解除 PR #66 阻塞；合并后证据使用独立 closeout，不扩大本次六路径。
```

## Approved Decision

- Decision：采用方案 A，只解除仓库实例测试对活动快照状态的错误固定，保留目录证据和专项状态机合同。
- Reason：这是唯一同时适用于 `main current` 与 `PR #66 stale`、不重复生产逻辑且不扩大缓存同步范围的最小根因修复。
- Scope boundary：仅六路径；不修改 PR `#66`、`upstream/`、生产 crate、Workflow、Cargo、预算、Ruleset、Gate 5 或 AGOS。
- Rejected options：动态重复解析状态、固定升级到 `v1.2.43`、把测试修复混入 PR `#66`。

## Brainstorming

```yaml
brainstorming:
  superpowers_skill: superpowers:brainstorming
  user_decision: 项目所有者明确回复“批准”，批准 Issue #67 六路径、scope_hash 与方案 A，允许实施、验证、提交、推送、PR、Review/CI；最终 Squash Merge 保留单独授权门。
  selected_business_path: inputcodex.release-audit-repository-contract
  approved_decision_evidence:
    - user-message:批准-2026-07-26
    - https://github.com/nonononull/inputcodex/issues/67#issuecomment-5083523560
  options_considered:
    - 方案 A：目录实例测试只固定目录证据，采用。
    - 方案 B：动态重复推导 source-lock 状态，拒绝。
    - 方案 C：固定改为 v1.2.43 或混入 PR #66，拒绝。
verification_commands:
  - cargo test --locked --offline --ignore-rust-version -p inputcodex-parity --test catalog_repository
  - pwsh -NoProfile -File scripts/ci/Test-CiScripts.ps1
  - pwsh -NoProfile -File scripts/ci/Verify-ReleaseAuditGate.ps1 -RepositoryRoot .
  - pwsh -NoProfile -File scripts/ci/Verify-RepositoryPolicy.ps1 -RepositoryRoot .
  - cargo fmt --all -- --check
  - git diff --check
```

## 执行批次

1. `startup-baseline`：确认基线、隔离工作树、本机时间和六路径范围。
2. `control-plane`：写入计划与 Runtime Workflow，运行 Session Plan verifier 和一次 AGOS ReportOnly。
3. `red`：保留 PR `#66` Head 上旧测试退出码 `101` 的同构失败证据。
4. `green`：仅修改仓库实例测试，复验 main/current 与临时 PR `#66`/stale 两种状态。
5. `verify`：运行目录测试、CI 合同、Release Audit Gate、仓库政策、fmt、scope hash 与 diff check。
6. `delivery`：六路径提交、普通推送、非 Draft PR、最终 Head Review/CI；Squash Merge 另行授权。

## Change Contract

```yaml
change_contract:
  mutation_intent: control-plane
  target_contract:
    owner: crates/inputcodex-parity/tests/catalog_repository.rs
    expected_behavior: 仓库实例测试固定 v1.2.42 目录证据，但允许活动快照处于合法 current 或 stale-re-audit-required 状态
    evidence_refs:
      - crates/inputcodex-parity/tests/catalog_repository.rs:305
      - crates/inputcodex-parity/tests/catalog_repository.rs:393
      - https://github.com/nonononull/inputcodex/actions/runs/30202056781
  preserved_invariants:
    - name: release-audit 状态机合法性
      baseline_ref: release_audit_显式解耦快照与功能目录审计基线
      regression_ref: 完整 catalog_repository 测试
    - name: v1.2.42 目录证据完整性
      baseline_ref: 仓库v1_2_42目录重新审计恢复current
      regression_ref: 改名后的目录证据测试与仓库总体验证
  adjacent_surfaces:
    - name: PR #66 纯缓存同步范围
      why_adjacent: 必须让合法 v1.2.43 stale 输入通过，但不得修改其 35 路径或 Final Head
    - name: Issue #65 功能目录重新审计
      why_adjacent: stale 必须继续保留重新审计引用，不能被测试修复伪装成 current
  historical_state_refs:
    - Issue #41 / PR #42 的快照值与状态语义解耦
    - Issue #38 / PR #45 的 v1.2.42 目录重新审计
  stale_verdict_invalidation_refs:
    - 任何最后写入后的旧本地测试、旧 Review 或旧 CI 结果均失效
  regression_checks:
    - surface: main current 仓库实例
      command_or_evidence_ref: cargo test -p inputcodex-parity --test catalog_repository
      expected_result: 12/12 通过且 current 专项语义保持 false
    - surface: PR #66 stale 临时叠加
      command_or_evidence_ref: 临时 detached 工作树叠加 4e419586fc89b1bbdd79d20b7179f017070052fb
      expected_result: 12/12 通过且 stale 专项语义保持 true
    - surface: PR #66 范围不变
      command_or_evidence_ref: gh pr view 66 与 git diff 路径对账
      expected_result: Final Head 与 35 路径不变
  sibling_regression_guard:
    status: passed
    closeout_rule: passed-or-blocked-before-done
  protected_feature_replay:
    status: passed
    known_good_features:
      - feature: current Release Audit
        owner: crates/inputcodex-parity
        baseline_evidence_ref: release_audit 专项测试 current 分支
        post_change_replay_plan_ref: 本分支完整 catalog_repository 测试
        post_change_replay_ref: 本分支 main/current 完整 catalog_repository 12/12
        expected_result: snapshot 与 catalog 相同时 requires_reaudit=false
        actual_result: snapshot 与 catalog 相同时 requires_reaudit=false；v1.2.42 main/current 完整仓库验证 12/12 通过。
        owner_visible_status: passed
        regression_status: passed
      - feature: stale Release Audit 与重新审计引用
        owner: crates/inputcodex-parity
        baseline_evidence_ref: PR #66 Run 30202056781 的合法 stale 输入
        post_change_replay_plan_ref: 临时叠加 PR #66 Head 的完整 catalog_repository 测试
        post_change_replay_ref: 临时 detached 工作树叠加 PR #66 Head 后完整 catalog_repository 12/12
        expected_result: snapshot 新于 catalog 时 requires_reaudit=true 且仓库验证成功
        actual_result: snapshot 新于 catalog 时 requires_reaudit=true 且仓库验证成功；v1.2.43 stale 与 Issue #65 引用完成 12/12 回放。
        owner_visible_status: passed
        regression_status: passed
    forbidden_ops_until_replay:
      - commit
      - push
      - pr
      - claim-done
```
