# Issue #69：性能实现哈希路径边界 Session Plan

```yaml
schema_version: agos.session-plan.v1
architecture_contract_version: agos.brainstorming-gate.v1
task_id: issue-69
work_class: standard
task_summary: 修复普通 crate tests 被错误纳入性能 implementation_sha256，并通过真实 hosted 重新测量迁移 Evidence
project_root: C:/Users/dashuai/Documents/inputcodex
trigger_source: PR #68 Performance Baseline Run 30203435572
tracking_issue_ref: https://github.com/nonononull/inputcodex/issues/69
baseline_ref: a2e5e5a6728200739a4acb85042ba7831ac6295b
branch: codex/issue-69-performance-hash-boundary
implementation_plan_ref: docs/plans/2026-07-26-issue-69-performance-implementation-hash-boundary.md
runtime_workflow_ref: docs/workflows/2026-07-26-issue-69-performance-implementation-hash-boundary-runtime.md
decision_status: approved
approval_source: direct-user
approved_decision_ref: user-message:批准-独立性能Evidence合同方案A-2026-07-26
scope_approval_status: approved
scope_approval_ref: https://github.com/nonononull/inputcodex/issues/69#issuecomment-5083808758
scope_hash: sha256:6392a1b4150f2aae0c34c285e83b6870f47e3bdc57def6308d0a26ddd158911d
allowed_operations: exact-ten-path-tdd, local-light-validation, normal-commit, normal-push, github-hosted-measure, evidence-refresh, non-draft-pr, review-ci
mutation_intent: narrow-performance-implementation-hash-to-measured-production-surface-and-refresh-evidence-from-exact-hosted-head
executor_enforcement: exact-ten-path-set, no-pr68-mutation, no-manual-evidence-rehash, github-hosted-measure-only, normal-push-only, squash-merge-only
delivery_contract: agos.issue-pr-merge.v1
review_strategy: final-head changed-surface manual review plus all GitHub review conversations resolved with root-cause evidence
ci_expectation: standard GitHub-hosted CI and Performance Baseline must be green on final PR Head
merge_policy: squash-only, no-force-push, never-delete-main
scope_boundary: approved ten paths only; final squash merge requires separate owner authorization
local_time: 2026-07-26T21:26:28+08:00
closeout_ref: pending
agos_status: bypassed-report-only-needs-input-unregistered
```

## Local Knowledge Lookup

```yaml
local_knowledge_lookup:
  codegraph:
    - 仓库根不存在 .codegraph；按 AGENTS.md 禁止擅自初始化。
    - GitNexus list_repos 返回空列表，没有可查询的 inputcodex 图谱。
  project_refs:
    - AGENTS.md
    - README.md
    - build.md
    - err.md
    - docs/plans/PROJECT-MASTER-PLAN.md
    - docs/plans/issue-32-performance-baseline.md
    - docs/reports/issue-32-performance-baseline.md
    - docs/workflows/issue-32-performance-baseline-runtime.md
    - docs/workflows/2026-07-26-issue-55-performance-remeasurement-entry-runtime.md
  external_governance:
    - AGOS Default Entry 已只运行一次 ReportOnly，返回 needs-input、unregistered、doctor=blocked；项目 Git 与入口文档为 ready，按项目规则立即绕过且不修改外部仓库。
  missing_coverage:
    - 当前没有独立知识卡覆盖普通测试路径误入性能实现哈希；以脚本源码、实际哈希计算、Issue #32 既有先例和 GitHub Run 为权威证据。
```

## Superpowers Method Discipline

```yaml
superpowers_method_discipline:
  using_superpowers: superpowers:using-superpowers
  brainstorming:
    skill: superpowers:brainstorming
    decision: 初始方案 A 已获批准；Discovery 发现必须重新 measure 的新增副作用，实施前重新请求精确十路径批准。
  systematic_debugging:
    skill: superpowers:systematic-debugging
    evidence: 读取失败输出、稳定复现、追踪 implementationPaths、计算旧/新集合哈希并查证历史先例。
  worktree_isolation:
    skill: superpowers:using-git-worktrees
    evidence: C:/Users/dashuai/Documents/inputcodex-worktrees/issue-69-performance-hash-boundary 从 origin/main 隔离创建。
  planning:
    skill: superpowers:writing-plans
    evidence: 项目原生 Implementation Plan、Session Plan 与 Runtime Workflow 已落盘。
  test_driven_development:
    skill: superpowers:test-driven-development
    cycle: 新行为合同 RED -> 最小动态路径修复 GREEN -> hosted Evidence 刷新 -> PR #68 回放
  verification_before_completion:
    skill: superpowers:verification-before-completion
    evidence: 每次提交、推送、PR 和完成声明前运行 Runtime Workflow 中的新鲜验证。
  code_review:
    request_skill: superpowers:requesting-code-review
    receive_skill: superpowers:receiving-code-review
    evidence: 不启用未获用户请求的 subagent；当前执行器完成 changed-surface 自审并闭环全部 GitHub Review 对话。
```

## Brainstorming Gate

<a id="decision"></a>

### 问题定义

PR `#68` 只修改普通测试，却使 Performance Baseline 的三份 Evidence 全部 `HASH_MISMATCH`。修复不能削弱真实实现绑定，也不能伪造历史样本。

### 方案比较

| 方案 | 内容 | 结论 |
| --- | --- | --- |
| A2（推荐） | 收窄动态路径到 `Cargo.toml + src/**/*.rs`；由修复后精确 Head 重新 measure 并刷新三份 Evidence | 最小、符合既有严格语义和 Issue #32 先例 |
| B | 引入版本化兼容验证，从 `measurement_commit` 重建历史路径集合 | 依赖 Git 历史与 checkout 深度，复杂且扩大 CI/Workflow 面，不采用 |
| C | 直接替换三份 Evidence 的哈希或忽略实现哈希 | 伪造历史绑定或削弱门禁，禁止 |

### 新增副作用与停止点

初始方案认为只排除普通 tests 即可保留旧 Evidence。实际计算证明新集合哈希为 `sha256:39fcc5593ada18c4b42daf2e94556a97dda1e90385f146184178418a79b38a7e`，不等于旧 Evidence；验证器和 CI 合同又属于固定实现输入。因此 A2 必须增加三份 hosted Evidence 刷新路径，当前停在精确范围审批门，不擅自扩大原七路径提案。

## 精确范围

```text
benchmarks/results/issue-32/macos.json
benchmarks/results/issue-32/manifest.json
benchmarks/results/issue-32/windows.json
docs/plans/2026-07-26-issue-69-performance-implementation-hash-boundary.md
docs/plans/sessions/2026-07-26-issue-69-performance-implementation-hash-boundary.md
docs/reports/issue-69-performance-implementation-hash-boundary.md
docs/workflows/2026-07-26-issue-69-performance-implementation-hash-boundary-runtime.md
err.md
scripts/ci/Test-CiScripts.ps1
scripts/performance/Test-InputcodexBaseline.ps1
```

```text
scope_hash: sha256:6392a1b4150f2aae0c34c285e83b6870f47e3bdc57def6308d0a26ddd158911d
```

## Protected Feature Replay

```yaml
protected_feature_replay:
  status: implementation-checkpoint-ready
  features:
    - feature: 普通 crate tests 不属于性能被测实现
      baseline_evidence_ref: PR #68 old-algorithm hash sha256:174f8aec273a5b7490aa833e7554e26edda996a649105ce916d9cc4873ea8bd7
      post_change_replay_plan_ref: 临时 Contract 夹具增加 tests 文件后 implementation_sha256 保持不变
      expected_result: test-only 变化不使 Evidence 失效
      actual_result: CI 合同“性能实现哈希只绑定被测生产面”已通过；普通 tests 变化前后哈希相等。
    - feature: 产品 src 与 Cargo 合同仍绑定性能 Evidence
      baseline_evidence_ref: Issue #32 strict implementation_sha256 contract
      post_change_replay_plan_ref: 临时 Contract 夹具修改 src/lib.rs 后 implementation_sha256 必须变化
      expected_result: production-source 变化稳定触发新哈希
      actual_result: 同一合同已通过；产品 src 变化后 implementation_sha256 与基线不同。
    - feature: PR #68 不改变 Final Head
      baseline_evidence_ref: PR #68 Head 159abbf45dfdc29a277cb152af7368868f2f618d
      post_change_replay_plan_ref: Issue #69 合并后重新运行 PR #68 merge-ref checks
      expected_result: Head 不变，标准 CI 与 Performance Baseline 全绿
  forbidden_ops_until_replay:
    - claim-done
    - squash-merge
```
