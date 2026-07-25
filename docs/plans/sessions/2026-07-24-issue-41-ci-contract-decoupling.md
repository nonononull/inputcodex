# Issue #41：解耦上游快照同步与 CI 基线断言 Session Plan

```yaml
schema_version: agos.session-plan.v1
architecture_contract_version: agos.brainstorming-gate.v1
task_id: issue-41
work_class: standard
tracking_issue_ref: https://github.com/nonononull/inputcodex/issues/41
baseline_ref: 317349a2cee1d2777472c8ccbd55204e570176c4
branch: codex/issue-41-ci-contract-decoupling
implementation_plan_ref: docs/plans/2026-07-24-issue-41-ci-contract-decoupling.md
runtime_workflow_ref: docs/workflows/2026-07-24-issue-41-ci-contract-decoupling-runtime.md
owner_execution_intent_ref: user-message:批准独立-CI-合同-Issue-PR-2026-07-24
decision_status: approved
approved_decision_ref: session-plan:issue-41#decision
scope_approval_status: approved
scope_approval_ref: https://github.com/nonononull/inputcodex/issues/41#issuecomment-5067938336
scope_hash: sha256:ada2baa0a524b2c8f0831d946236197b056513981c30b4530d903114b709c1b8
allowed_operations: exact-seven-path-edit, local-red-green, temporary-pr40-overlay, git-checkpoint, normal-commit, normal-push, non-draft-pr, review-ci
mutation_intent: source
executor_enforcement: exact-seven-path-set, fresh-head-verification, normal-push-only, squash-merge-only
delivery_contract: agos.issue-pr-merge.v1
review_strategy: final-head changed-surface review plus all GitHub review conversations resolved with evidence
ci_expectation: Upstream Watch and standard GitHub-hosted Linux/Windows/macOS CI must be green on the final PR Head
merge_policy: squash-only, no-force-push, never-delete-main
scope_boundary: approved seven paths only; final squash merge requires separate owner authorization
closeout_ref: pending
```

## 当前事实

- GitHub 权威 `main` 与本机基线均为 `317349a2cee1d2777472c8ccbd55204e570176c4`。
- PR `#40` Head 为 `86d48ad261669daaf14666556372a12f9b908726`，其批准范围不包含本任务的两个测试文件；在其工作树的 RED 复现后仍保持干净。
- Python `Baseline` 已在生产加载路径中验证 `source-lock` schema、固定上游仓库、tag、UTC 时间、Release URL 和 40 位 SHA；失败来源是测试比较了可变完整对象。
- Rust stale/current 专项合同已覆盖合法与非法状态；失败来源是总体验证额外拒绝了合法 stale。
- AGOS ReportOnly 入口的任务登记为 `unregistered`、总体为 `needs-input`；项目规则要求记录后绕过，不允许改动外部控制面。
- 实现提交 `af866b6fabb41de3a9ea42b44859ef73d7a1549b` 已于项目所有者本机时间 `2026-07-25 13:04:15 +08:00` 创建；PR `#42` 已以该 Head 创建为非 Draft，首轮 GitHub-hosted CI 全绿。
- 双 reviewer 的首轮审查中，技术审查无发现；治理审查发现报告/计划仍把已完成的暂存、提交、推送和 PR 写成待完成。
- 本次控制面纠正提交只修复上述状态滞后；最终 Head 仍必须重新完成双 reviewer、Review 对话检查和 GitHub-hosted CI。

## Brainstorming

```yaml
brainstorming:
  superpowers_skill: superpowers:brainstorming
  user_decision: 项目所有者先批准建立独立 CI 合同 Issue/PR，后在范围不变时以“继续”继承批准七路径 scope_hash，允许实施、提交、推送、PR、Review/CI；最终 Squash Merge 保留单独授权门。
  selected_business_path: inputcodex.ci-contract-decoupling
  approved_decision_evidence:
    - user-message:批准独立-CI-合同-Issue-PR-2026-07-24
    - https://github.com/nonononull/inputcodex/issues/41
  options_considered:
    - 方案 A：以稳定字段和状态语义替代固定 Release 对象比较，采用。
    - 方案 B：把固定版本更新为 v1.2.42，拒绝。
    - 方案 C：在 PR #40 混入 CI 修复，拒绝。
verification_commands:
  - python -m unittest discover -s .github/scripts/tests -p 'test_upstream_watch.py' -v
  - python .github/scripts/upstream_watch.py --validate-only
  - cargo test --locked --offline --ignore-rust-version -p inputcodex-parity --test catalog_repository
  - pwsh -NoProfile -File scripts/ci/Verify-ReleaseAuditGate.ps1 -RepositoryRoot .
  - pwsh -NoProfile -File scripts/ci/Verify-RepositoryPolicy.ps1 -RepositoryRoot .
  - git diff --check
```

## 当前允许操作

项目所有者已批准七路径和 `scope_hash`，因此当前允许：

1. 修改精确七路径并执行本地轻量 RED/GREEN；
2. 用临时 detached 工作树无提交叠加 PR #40，验证合法 v1.2.42 stale 状态；
3. 普通提交、正常推送、创建关联 Issue #41 的非 Draft PR；
4. 处理 Review、等待标准 GitHub-hosted CI，并保留全部根因与验证证据。

仍禁止扩大七路径、修改 PR #40、force push、删除 `main`、使用 Merge Commit/Rebase Merge，或在最终 Squash Merge 未获单独授权时合并。

## 执行批次

### 批次 1：根因与范围门（已完成）

- 复核 #40 的 GitHub Actions 失败日志、提交范围与本地干净状态。
- 在 #40 复现 Python 退出 `1` 与 Rust 退出 `101`；两者均指向陈旧断言而非缓存输入损坏。
- 计算七路径排序、LF 字节输入和 `scope_hash`。

### 批次 2：项目所有者范围门（已完成）

- Issue #41 已回写方案 A、拒绝方案 B/C、RED 证据、七路径与哈希。
- 项目所有者在范围未变化后回复“继续”，批准引用为 `https://github.com/nonononull/inputcodex/issues/41#issuecomment-5067938336`；授权不包含最终 Squash Merge。

### 批次 3：TDD GREEN（已完成）

- 先保留 RED 输出，再仅修改两个测试文件，使仓库输入跨合法 Release/stale 状态可通过。
- 记录根因、处理和验证到 `err.md` 与执行报告。
- 运行项目定义的本地轻量定向命令；不执行本地 Workspace 全量 Rust 构建。

### 批次 4：合并模拟与交付（进行中）

- 用临时 detached 工作树无提交合入 #40 Head，复验 Python、Rust 与 Release Audit Gate，再完整清理临时状态。
- 实现提交 `af866b6fabb41de3a9ea42b44859ef73d7a1549b` 已普通推送，PR `#42` 已创建为非 Draft；首轮 CI 全绿。
- 首轮治理审查发现控制面阶段滞后；本次更正提交后，必须以最终 Head 重新执行双 reviewer、Review 对话检查与 GitHub-hosted CI。
- 全部 Review 对话写明根因、处理与验证；最终 Head CI 全绿且项目所有者单独授权后，才允许 Squash Merge。
- 合并后重新检查 #40 的自动重跑、关闭 Issue #41 并建立独立 closeout 证据。

## 停止条件

- 七路径或 `scope_hash` 相对批准引用发生变化；
- RED 不再稳定复现，或证据表明需要修改 `upstream/`、Workflow、Cargo、产品代码或 AGOS；
- 最小补丁不能同时保留 `Baseline` 失败关闭和 stale/current 专项语义；
- 本地验证、合并模拟、Review、CI、路径门禁或合并可用性失败；
- 需要 force push、删除 `main`、Merge Commit、Rebase Merge 或带未解决对话合并。

## 变更合同

```yaml
change_contract:
  mutation_intent: source-and-test-contract
  target_contract:
    owner: Issue #41
    expected_behavior: 合法 source-lock Release 更新和合法 stale-re-audit-required 均不应被陈旧总体验证阻断。
    evidence_refs:
      - Issue #41
      - PR #40 CI runs 30075403077 and 30075403007
  preserved_invariants:
    - name: source-lock 失败关闭验证
      owner: .github/scripts/upstream_watch.py
      baseline_ref: Baseline.__post_init__ 与 baseline_from_data
      regression_ref: Python 上游监控全量单测和 --validate-only
    - name: current/stale 目录审计语义
      owner: crates/inputcodex-parity
      baseline_ref: release_audit_显式解耦快照与功能目录审计基线
      regression_ref: catalog_repository 定向测试与 #40 合并模拟
    - name: #40 纯缓存范围
      owner: Issue #34 / PR #40
      baseline_ref: scope_hash sha256:fc2ff14a2011f54c3014daa63e3d7658d1a47bd68bd49ab3a84d9601c8d3d76c
      regression_ref: #40 相对 main 的二十路径差异不变
  adjacent_surfaces:
    - name: Upstream Watch 工作流
      why_adjacent: Python 单测由 workflow 调用
      risk: 误改 Workflow 会扩大到运行时权限与定时触发面
      owner: .github/workflows/upstream-watch.yml
    - name: Release Audit PR 门
      why_adjacent: stale 状态由该门读取
      risk: 错误放宽会允许无审计功能目录变更
      owner: scripts/ci/Verify-ReleaseAuditGate.ps1
  historical_state_refs:
    - d7438a0f2c43b7fbd2b159b3759aacea4ef1999e
    - 86d48ad261669daaf14666556372a12f9b908726
  stale_verdict_invalidation_refs:
    - https://github.com/nonononull/inputcodex/issues/38
  regression_checks:
    - surface: Python source-lock
      command_or_evidence_ref: python -m unittest discover -s .github/scripts/tests -p 'test_upstream_watch.py' -v
      expected_result: 全部通过，仓库 source-lock 不绑定固定 Release
    - surface: Rust 目录审计
      command_or_evidence_ref: cargo test --locked --offline --ignore-rust-version -p inputcodex-parity --test catalog_repository
      expected_result: current 与 stale 专项语义均通过
    - surface: #40 合并结果
      command_or_evidence_ref: 临时 detached 合并模拟
      expected_result: v1.2.42 stale 输入下 Python/Rust/Release Audit Gate 通过
  sibling_regression_guard:
    status: passed
    closeout_rule: passed-or-blocked-before-done
    exception_ref: none
  protected_feature_replay:
    status: passed
    not_applicable_reason: null
    known_good_features:
      - feature: 上游监控失败关闭
        owner: .github/scripts/upstream_watch.py
        baseline_evidence_ref: test_invalid_source_lock_fails_closed
        post_change_replay_plan_ref: Python 全量单测
        post_change_replay_ref: 主干与 PR #40 叠加的 Python 28/28 测试以及 --validate-only
        expected_result: 非法 source-lock 仍抛出 MonitorError
        actual_result: 非法输入测试通过；v1.2.41 与 v1.2.42 均完成动态全对象映射和失败关闭验证。
        owner_visible_status: passed
        regression_status: passed
      - feature: stale 目录审计复审提醒
        owner: crates/inputcodex-parity
        baseline_evidence_ref: release_audit_显式解耦快照与功能目录审计基线
        post_change_replay_plan_ref: Rust 定向测试与 #40 合并模拟
        post_change_replay_ref: 主干与 PR #40 叠加的 catalog_repository 10/10 和 Release Audit Gate
        expected_result: 合法 stale 为 true，非法 stale 为 ReleaseMismatch
        actual_result: current 为 false、PR #40 合法 stale 为 true、非法 stale 专项测试继续通过。
        owner_visible_status: passed
        regression_status: passed
    forbidden_ops_until_replay:
      - commit
      - pr
      - merge
      - claim-done
```

## Post-Implementation Review Policy

```yaml
post_implementation_review_policy:
  review_phase: post-implementation
  same_question_ref: session-plan:issue-41#decision
  required_agent_count: 2
  review_scope: changed-surface
  owner_requested_scope: changed-surface
  freshness_rule: review-after-last-mutation
```

## Local Knowledge Lookup

```yaml
local_knowledge_lookup:
  gbrain_queries:
    - local_knowledge_lookup MCP/CLI 未向当前会话暴露；resources=[]、resourceTemplates=[]，无查询结果可伪造。
  vault_refs:
    - D:/Android_source/ai-growth-os/components/vault/08-Skills/AI-Growth-OS.md
  rules_refs:
    - D:/Android_source/ai-growth-os/components/rules/rules/workflows/ai-growth-os-brainstorming-gate.md
  project_refs:
    - AGENTS.md
    - build.md
    - err.md
    - docs/plans/PROJECT-MASTER-PLAN.md
  missing_coverage:
    - 未发现可复用的 inputcodex CI stale 状态修复知识卡；本任务以本地 RED、Git 历史和项目原生合同审计为唯一证据。
```

## Superpowers Method Discipline

```yaml
superpowers_method_discipline:
  upstream_superpowers_ref: https://github.com/obra/superpowers
  local_superpowers_state: verified-current
  using_superpowers: superpowers:using-superpowers
  brainstorming: superpowers:brainstorming
  worktree_isolation:
    skill: superpowers:using-git-worktrees
    evidence: C:/Users/dashuai/Documents/inputcodex-worktrees/issue-41-ci-contract-discovery 从 origin/main 隔离创建，批准后分支重命名为 codex/issue-41-ci-contract-decoupling。
  test_driven_development:
    skill: superpowers:test-driven-development
    cycle: RED/GREEN/REFACTOR
    evidence: #40 未修改工作树的 Python 退出 1、Rust 退出 101；范围批准后，GREEN 与临时合并模拟已完成，证据见执行报告；每次最后变更仍需新鲜 Review 与最终 Head CI。
  verification_before_completion:
    skill: superpowers:verification-before-completion
    evidence: 提交、PR、Merge 或完成声明前必须取得新鲜本地验证、合并模拟、Review 和最终 Head CI 证据。
  systematic_debugging:
    skill: superpowers:systematic-debugging
    trigger: PR #40 的 Upstream Watch 与三平台 CI 失败。
  code_review:
    request_skill: superpowers:requesting-code-review
    receive_skill: superpowers:receiving-code-review
    evidence: 最后一处实现变更后请求新鲜审查；所有对话逐条写明根因、处理与验证。
  planning_execution:
    writing_skill: superpowers:writing-plans
    executing_skill: superpowers:executing-plans
    subagent_skill: superpowers:subagent-driven-development
    plan_control_plane: project-native AGOS control docs
  finishing_branch:
    skill: superpowers:finishing-a-development-branch
    evidence: 在最终验证、Review 和 CI 均通过后才决定 PR、Squash Merge 与清理。
  evidence_writeback:
    target: build.md, session plan, runtime workflow, closeout report
    docs_superpowers_boundary: docs/superpowers remains archive-only, not the active control plane
delivery_evidence:
  tracking_issue_ref: https://github.com/nonononull/inputcodex/issues/41
  review_ref: first-pass technical=no-findings; governance=control-plane-state-drift-fixed-by-follow-up-doc-commit; final-head-review=pending
  pr_ref: https://github.com/nonononull/inputcodex/pull/42
  ci_ref: https://github.com/nonononull/inputcodex/actions/runs/30145266589 (af866b6 first Head green; final Head rerun pending)
  merge_ref: pending
master_plan:
  path: docs/plans/PROJECT-MASTER-PLAN.md
  update_required: false
  update_summary: 仅在 #41 合并收口后回写，不在发现阶段改动主计划。
```
