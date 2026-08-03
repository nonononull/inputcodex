# Issue #143 Session Plan：Watcher 偏好固定文件 mutation

schema_version: agos.session-plan.v1
architecture_contract_version: agos.brainstorming-gate.v1
task_id: issue-143-gate-5-watcher-preference-mutation
work_class: standard
decision_status: approved
approved_decision_ref: https://github.com/nonononull/inputcodex/issues/140#issuecomment-5159072214
tracking_issue_ref: https://github.com/nonononull/inputcodex/issues/143
delivery_contract: agos.issue-pr-merge.v1
mutation_intent: source
execution_profile: project-native-v1
brainstorming_method: owner-approved-committee-decision
execution_contract: agos.execution-contract.v1
command_source: build.md
implicit_tool_preconditions: forbidden
scope_hash: sha256:f96f1a979eba89bc4de9744b3267bfd72fd81a828c9491cbfd3b95723b088ab9
selected_business_path: gate5-fixed-file-mutation-tranche-v1/batch-2

## Approved Decision

- Decision: 只实现固定 `watcher.disabled` 的 typed mutation。
- Reason: #140 已明确缩窄为 cooperative-same-user 威胁模型，并保留可见链接/reparse、普通竞态、提交点与 receipt 门。
- Scope boundary: 二十四路径；两条 command；一个产品 feature；无新依赖。
- Rejected options: 完整 Watcher、SQLite、环境/诊断批量写入、通用文件 mutation、句柄/VFS/FFI。

## Local Knowledge Lookup

```yaml
local_knowledge_lookup:
  gbrain_queries:
    - unavailable-current-executor-no-local_knowledge_lookup-tool
  vault_refs:
    - D:/Android_source/ai-growth-os/components/vault/07-Workflows/Core/AI-Growth-OS-Runtime-Workflow-And-ErrMd-Correction.md
  rules_refs:
    - AGENTS.md
    - D:/Android_source/ai-growth-os/components/rules/rules/workflows/ai-growth-os-auto-application.md
  project_refs:
    - README.md
    - build.md
    - err.md
    - CONTEXT.md
    - docs/plans/PROJECT-MASTER-PLAN.md
    - docs/plans/2026-08-01-issue-128-gate-5-watcher-preference-observation.md
    - upstream/CodexPlusPlus/crates/codex-plus-core/src/watcher.rs
    - upstream/CodexPlusPlus/apps/codex-plus-manager/src-tauri/src/commands.rs
  github_refs:
    - https://github.com/nonononull/inputcodex/issues/136
    - https://github.com/nonononull/inputcodex/issues/140
    - https://github.com/nonononull/inputcodex/issues/143
  missing_coverage:
    - AGOS default entry returned needs-input/session-plan-bootstrap; bypassed under project rules
    - two Paseo read-only planning agents timed out during model availability probe and created no agent
```

## Change Contract

```yaml
change_contract:
  mutation_intent: source
  target_contract:
    owner: feature.foundation-platform.watcher-preference-mutation
    expected_behavior: fixed expected-to-desired marker mutation with commit-aware receipt
  preserved_invariants:
    - name: watcher-preference-observation-remains-read-only
      owner: feature.foundation-platform.watcher-preference-observation
    - name: full-watcher-remains-unassessed
      owner: feature.foundation-platform.watcher
    - name: no-path-or-content-disclosure
      owner: inputcodex privacy boundary
    - name: windows-macos-domain-parity
      owner: AGENTS product constraints
  adjacent_surfaces:
    - name: SystemPlatformPaths
      risk: absolute PathBuf is not a bound filesystem handle
    - name: LoadCoordinator
      risk: cancelled completion becomes Stale and must not be reused
    - name: watcher-preference-observation
      risk: mutation must not weaken read-only semantics
  regression_checks:
    - surface: four affected crates
      command_or_evidence_ref: build.md#Issue-143
      expected_result: tests-clippy-fmt-green
    - surface: parity repository
      command_or_evidence_ref: cargo test -p inputcodex-parity
      expected_result: exact-counts-and-two-source-move
  sibling_regression_guard:
    status: passed-local
    closeout_rule: passed-or-blocked-before-done
```

## Scope And Ownership

- planning_scope_count: 8
- planning_scope_hash: sha256:7011b4faa5d331e668b2eb837dbbc42b716541f9f88431c9a773ae830c0dbe11
- candidate_scope_count: 24
- candidate_scope_hash: sha256:f96f1a979eba89bc4de9744b3267bfd72fd81a828c9491cbfd3b95723b088ab9
- active_writers_max: 1
- current_writer: parent coordinator in fresh #143 worktree
- protected_worktree: issue-132-gate-5-zed-remote-project-observation is read-only and out of scope

## Independent Verification Policy

```yaml
independent_verification_policy:
  activation: after-last-mutation-before-closeout
  work_class: standard
  primary_verifier_count: 2
  reserve_slot_count: 1
  lane_freshness_rule: whole-final-head
  report_authority: advisory-only
  required_result: 0-Critical-0-Important
```

## Execution Batches

- Planning Freeze: completed
- Domain TDD: completed
- Application TDD: completed
- Platform TDD: completed
- Parity TDD: completed
- Local closeout: completed
- Review / CI / exact Head Squash: pending
- Reopen #140 and stop: pending

## Execution Evidence

- test: 四 crate all-targets、Clippy 与 fmt 全绿；Domain `4/4`、Application `8/8`、Platform 内部 `13/13` 与全包、Parity `32/32` 已通过
- build: standard GitHub-hosted Windows/macOS CI；result pending
- review: two independent Final Head reviewers；result pending
- verification: CI contract `82/82`、Repository Policy 0、Release Audit current、Cargo metadata、禁止能力与 `24 / sha256:f96f1a...` 全部通过
- closeout: exact Head Squash、main dual workflows、Artifact 0、reopen #140；result pending

## Stop Conditions

范围、基线、Release、威胁模型、双平台终态或 #140 授权任一漂移即停止。禁止修改 #132/#133、AGOS、
Cargo、Workflow/Runner/Ruleset、Release、upstream、UI，禁止新增依赖、`unsafe`、FFI/VFS、网络或进程控制。
