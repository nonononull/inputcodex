# Issue #138 Session Plan：候选前沿耗尽自治终态

task_id: issue-138-autonomous-candidate-exhaustion-state
task_class: Standard
task_type: governance-corrective
decision_status: approved
delivery_contract: agos.issue-pr-merge.v1
tracking_issue_ref: https://github.com/nonononull/inputcodex/issues/138
session_plan_ref: docs/plans/sessions/2026-08-02-issue-138-autonomous-candidate-exhaustion-state.md
approved_decision_ref: https://github.com/nonononull/inputcodex/issues/137#issuecomment-5157787716
baseline_ref: origin/main@da035b3a6e8ddab9b7c6948ef115ed8b561aa1f4
branch_ref: codex/issue-138-autonomous-candidate-exhaustion-state
mutation_intent: governance
allowed_operations:
  - issue-138-task-local-control-docs
  - autonomous-policy-contract
  - read-only-state-resolution
  - powershell-contract-tests
  - stable-gate-documentation
  - git-checkpoints-and-normal-push
  - non-draft-pr-review-ci-and-exact-head-squash
  - post-merge-live-terminal-state-proof
executor_enforcement: fail-closed-on-scope-marker-label-head-or-delivery-drift

## Session Decision

Issue #137 的所有者决策选择保守阶段拆分：确认当前基线下只读候选前沿饱和，不降低安全合同，
不宣称 Gate 5 产品完成，也不解锁 Gate 6。本任务只修复状态控制面无法表达该事实的根因。

## Local Knowledge Lookup

local_knowledge_lookup:
  query: inputcodex Gate 5 candidate exhaustion autonomous state owner decision recovery
  memory_refs:
    - C:/Users/dashuai/.codex/memories/MEMORY.md
  project_refs:
    - AGENTS.md
    - build.md
    - err.md
    - docs/plans/PROJECT-MASTER-PLAN.md
    - docs/plans/sessions/2026-07-31-issue-111-autonomous-refactor-control-plane.md
    - scripts/automation/Get-AutonomousRefactorState.ps1
    - scripts/ci/Test-CiScripts.ps1
  github_refs:
    - https://github.com/nonononull/inputcodex/issues/111
    - https://github.com/nonononull/inputcodex/issues/137
    - https://github.com/nonononull/inputcodex/issues/138
  result: project-native-control-plane-sufficient

## AGOS Boundary

AGOS default entry 对 `issue-138` 返回 `unregistered / needs-input`，但项目 Git 与入口文档基础均为 ready。
按 inputcodex 项目规则立即绕过，继续项目原生流程；禁止修改 AGOS Registry、规则、脚本、Workflow 或 Vault。

当前会话没有 `superpowers:*` skill。架构比较已由 #137 committee、独立复审和所有者决策完成；TDD、
review 与 verification 使用项目原生 PowerShell 合同和 `karpathy-guidelines`。

## Scope Freeze

planning_scope_count: 7
planning_scope_hash: sha256:2b264858d4a3bcde53885c7121e2ef65e3c9b04cf7f685bf1095a583de32604a
candidate_scope_count: 12
candidate_scope_hash: sha256:aac82afc513192fa9adbe1ed6b85f81c56fa3bd8cd3ea20e718d1e31c794f417

候选范围只包含策略、三份 PowerShell、四份 task-local 文档、AGENTS、build、Master Plan 和 err。
产品、Cargo、Parity、Workflow、Ruleset、Release、upstream、UI 与 AGOS 全部禁止。

## Domain Language

- 候选前沿饱和：当前精确 Release 与目录基线下，没有符合 standing authorization 的下一安全候选。
- Gate 5 产品完成：产品范围和所有阶段条件已由所有者批准完成；本任务不成立该事实。
- 候选耗尽终态：只读控制面停止选择候选并等待所有者决策的机器状态，不是产品状态。

## Execution Batches

1. planning-freeze
2. policy-marker-red
3. policy-marker-green
4. terminal-state-red
5. terminal-state-green
6. local-closeout-and-review
7. pr-review-ci
8. exact-head-squash-and-main-verification
9. live-terminal-state-proof

每批结束运行 Git snapshot governance ReportOnly；RED/GREEN 与稳定验证面分别建立普通提交，禁止 amend、
rebase、force push 或修改 main。

## Local Green Evidence

- candidate exhaustion 策略、typed marker、required label、专用 state/action 与 hard stop 已形成单一严格合同。
- 合法 terminal snapshot 返回 `blocked-candidate-exhausted / await-owner-decision`；label、Release、仓库状态、
  活动或已交付 PR 任一漂移均返回稳定 hard stop reason。
- PowerShell 空数组与单元素数组使用属性投影保留身份，禁止把数组伪装成标量。
- 完整 CI 脚本合同为 `80/80`，真实 #138 live 预检仍返回 `active-worktree-execution / resume-worktree`。
- 规范化策略哈希为 `sha256:c907410a535020e9276fd8de5f448fca38a91ebd84aa26e8768a51818f300d53`。
- 首轮 Final Head 独立复审为 `0 Critical / 2 Important / FAIL`，暴露 typed marker 前缀大小写与 hard-stop
  大小写检查未 fail-closed；同族变异继续覆盖 candidate 对象数组/额外字段和 live typed-only 预筛。
- 纠正 RED 精确为 `77 PASS / 3 FAIL`，最小修复后恢复 `80/80`，并再次通过真实 #138 live 预检。
