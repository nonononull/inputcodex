# Issue #123：Local Storage 模型后缀清理一致性例外 Session Plan

```yaml
schema_version: agos.session-plan.v1
architecture_contract_version: agos.brainstorming-gate.v1
task_id: issue-123
work_class: standard
task_summary: 纠正 codex_local_storage 的错误只读建模并隔离其 CDP Local Storage 写入副作用
project_root: C:/Users/dashuai/.paseo/worktrees/1takg4n7/issue-123-local-storage-sanitization-exception
tracking_issue_ref: https://github.com/nonononull/inputcodex/issues/123
standing_authorization_ref: https://github.com/nonononull/inputcodex/issues/111
planning_freeze_ref: https://github.com/nonononull/inputcodex/issues/123#issuecomment-5150729887
baseline_ref: 29f5317d66b9f16cf64797420bf2fd7e2aec45f7
branch: codex/issue-123-local-storage-sanitization-exception
implementation_plan_ref: docs/plans/2026-08-01-issue-123-local-storage-sanitization-exception.md
runtime_workflow_ref: docs/workflows/2026-08-01-issue-123-local-storage-sanitization-exception-runtime.md
report_ref: docs/reports/issue-123-local-storage-sanitization-exception.md
decision_status: approved-via-bounded-standing-authorization
approved_decision_ref: https://github.com/nonononull/inputcodex/issues/111
scope_approval_ref: https://github.com/nonononull/inputcodex/issues/123#issuecomment-5150729887
scope_hash: sha256:aa27e2551cfa743248ef7a2ab53fad5f1a1954b369ae40bf3485ada2099f7bdc
allowed_operations: exact-sixteen-path-parity-correction, project-native-tdd, local-lightweight-verification, git-checkpoint, normal-push, non-draft-pr, review-ci, exact-head-squash-under-standing-authorization
mutation_intent: control-plane
business_mutation_intent: parity-exception-catalog-correction-only
executor_enforcement: exact-path-set, no-product-cargo-workflow-ruleset-upstream-readme-ui-release-agos-mutation, normal-push-only, squash-only
delivery_contract: agos.issue-pr-merge.v1
```

## 决策记录

```yaml
decision:
  outcome: approved
  selected_architecture: rename-and-isolate-as-local-storage-model-suffix-sanitization-exception-pending
  rejected_options:
    - 保留 token-usage-history 名称，会继续把 CDP 写入链伪装成只读能力
    - 直接实现上游行为，会违反无 WebView/JavaScript/注入和副作用评审边界
    - 同时迁移 rollout Token 历史，会混合两个来源、两套资源边界和不同安全语义
  deferred_capability: codex-plus-data rollout thread usage history requires separate bounded read-only Discovery
  expected_counts: sources=135, features=44, contracts=44, fixtures=12, exceptions=11, excluded=3, gaps=0
```

## 本地知识与控制面

```yaml
local_knowledge_lookup:
  memory_ref: inputcodex autonomous refactor interrupt recovery
  project_refs:
    - AGENTS.md
    - README.md
    - build.md
    - err.md
    - docs/plans/PROJECT-MASTER-PLAN.md
    - Issue #111 autonomous control plane
    - Issue #115 / PR #122 v1.2.44 catalog re-audit
  codegraph_status: not-initialized
  graph_policy: 不擅自初始化 CodeGraph
  agos_status: optional-bypassed
  legacy_tool_canary: project-native-v1; use project-local control plane
```

## Change Contract

```yaml
change_contract:
  target_contract: codex_local_storage 诚实登记为 CDP Local Storage 写入例外
  preserved_invariants:
    - 产品运行面零差异
    - Windows 与 macOS 同一 exception-pending 语义
    - source/feature/contract/fixture/exception/excluded/gap=135/44/44/12/11/3/0
    - 无广告、遥测、WebView、JavaScript 业务代码或注入迁移
  adjacent_surfaces:
    - rollout Token 历史保留为独立 Discovery
    - upstream/CodexPlusPlus 只读快照不可修改
  regression_checks:
    - cargo test -p inputcodex-parity --test catalog_repository --offline
    - scripts/ci/Test-CiScripts.ps1
    - scripts/ci/Verify-ReleaseAuditGate.ps1
    - scripts/ci/Verify-RepositoryPolicy.ps1
    - exact 16 path scope, rustfmt and git diff --check
```

## 证据

- Fresh 基线：目录 `28/28`、CI 合同 `76/76`、Repository Policy `ok=true / 0`、
  Release Audit `current`。
- RED：提交 `e4b134953d802e6968511b241e669b1fb67d4ed9`，目录 `26/29`；
  三个失败均命中预期旧语义。
- GREEN：Windows 本机时间 `2026-08-01 17:17:44.717 +08:00`，目录 `29/29`、CI 合同
  `76/76`、Release Audit `current`、Repository Policy `0`、rustfmt、whitespace 与 16 路径/hash
  均通过。
- 完整本地门、PR、Final Head 与合并后证据由 Runtime Workflow 持续回写到 GitHub。

## 停止门

- 实际路径或 hash 漂移。
- 需要迁移上游 CDP/JavaScript/注入/Local Storage 写入或 rollout Token 历史。
- 需要修改产品、Cargo、Workflow、Ruleset、upstream、README、UI、Release 或 AGOS。
- Final Head Review、Review thread、CI/Performance、Artifact、release audit 或 origin/main
  freshness 未闭环。
