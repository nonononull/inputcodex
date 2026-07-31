# Issue #111 Session Plan：无人值守重构控制面

task_id: issue-111-autonomous-refactor-control-plane
task_class: Standard
task_type: governance
decision_status: approved
delivery_contract: agos.issue-pr-merge.v1
tracking_issue_ref: https://github.com/nonononull/inputcodex/issues/111
session_plan_ref: docs/plans/sessions/2026-07-31-issue-111-autonomous-refactor-control-plane.md
approved_decision_ref: https://github.com/nonononull/inputcodex/issues/111
baseline_ref: origin/main@86a7dd837652f63198c7682b84d82180b8558e3a
branch_ref: codex/issue-111-autonomous-refactor-control-plane
mutation_intent: governance
allowed_operations:
  - issue-111-planning-and-control-docs
  - offline-policy-schema-and-validator
  - read-only-state-resolution
  - ci-contract-tests
  - git-checkpoints-and-normal-push
  - non-draft-pr-review-ci-and-exact-head-squash
  - post-merge-verification-and-paseo-activation
executor_enforcement: fail-closed-on-scope-policy-or-merge-gate-drift

## Session Decision

项目所有者明确要求后续重构全自动执行，不再逐项征求意见。Issue #111 将该指令收敛为 bounded
standing authorization：它只覆盖既有重构目标和现有不变量内的常规 Issue/PR/精确 Head Squash，
不覆盖付费资源、密钥/签名、正式 Release、许可证例外、force push、删除 main 或 Ruleset 绕过。

推荐 GitHub + Paseo 混合自治。GitHub 保存长期事实，Paseo 保持单写者和中断恢复；GitHub 原生
auto-merge 继续关闭，合并由自治执行器在精确 Head 门禁全绿后完成。

## Local Knowledge Lookup

local_knowledge_lookup:
  query: inputcodex Gate 5 issue PR merge gate performance parity autonomous interruption recovery
  memory_refs:
    - C:/Users/dashuai/.codex/memories/MEMORY.md
  project_refs:
    - README.md
    - AGENTS.md
    - build.md
    - err.md
    - docs/plans/PROJECT-MASTER-PLAN.md
    - docs/plans/sessions/2026-07-30-issue-109-gate-5-markdown-generation.md
    - docs/workflows/2026-07-30-issue-109-gate-5-markdown-generation-runtime.md
    - parity/features/*.yml
  github_refs:
    - https://github.com/nonononull/inputcodex/issues/111
    - https://github.com/nonononull/inputcodex/pull/110
    - https://github.com/nonononull/inputcodex/actions/runs/30559880826
    - https://github.com/nonononull/inputcodex/actions/runs/30559880720
  rules_refs:
    - project AGENTS.md issue-pr-squash and owner-decision contract
    - main Ruleset 19395456
  missing_coverage:
    - Paseo provider registry has no Gemini provider, but local Gemini CLI exists
    - repository has no required status checks, so GitHub native auto-merge is unsafe

AGOS 没有参与当前事实裁决。项目原生控制面充足，Issue #111 又禁止修改 AGOS；因此只记录边界，
不让 AGOS 未登记或 needs-input 阻塞本任务。

## Superpowers Method Discipline

- brainstorming：比较长会话、Actions 内 AI、GitHub + Paseo 三种路线后选择第三种。
- writing-plans：使用项目原生 Plan、Session Plan、Runtime Workflow 和 Report。
- using-git-worktrees：Paseo 从 origin/main 建立隔离 worktree。
- test-driven-development：验证器和状态解析器都先 RED 后 GREEN。
- verification-before-completion：任何通过、PR 或合并结论必须有当前命令证据。
- finishing-a-development-branch：PR 完成后由 Paseo 所有权归档工作区。

## Scope Freeze

planning_scope_count: 7
planning_scope_hash: sha256:4c33c7597fcf2afbcc084a8e32560a3ea1801db4d984ccff5b7338c30ef10431
candidate_scope_count: 12
candidate_scope_hash: sha256:5d1f609ca2a5913e4e5df21f0fd04d6de2c6731cdd71d641812fbee80b5ad713

Planning allowlist:

    AGENTS.md
    build.md
    docs/plans/2026-07-31-issue-111-autonomous-refactor-control-plane.md
    docs/plans/PROJECT-MASTER-PLAN.md
    docs/plans/sessions/2026-07-31-issue-111-autonomous-refactor-control-plane.md
    docs/reports/issue-111-autonomous-refactor-control-plane.md
    docs/workflows/2026-07-31-issue-111-autonomous-refactor-control-plane-runtime.md

Candidate implementation allowlist:

    .github/autonomous-refactor-policy.json
    AGENTS.md
    build.md
    docs/plans/2026-07-31-issue-111-autonomous-refactor-control-plane.md
    docs/plans/PROJECT-MASTER-PLAN.md
    docs/plans/sessions/2026-07-31-issue-111-autonomous-refactor-control-plane.md
    docs/reports/issue-111-autonomous-refactor-control-plane.md
    docs/workflows/2026-07-31-issue-111-autonomous-refactor-control-plane-runtime.md
    err.md
    scripts/automation/Get-AutonomousRefactorState.ps1
    scripts/ci/Test-CiScripts.ps1
    scripts/ci/Verify-AutonomousRefactorPolicy.ps1

err.md 只有在出现新可复用根因时才进入实际范围；否则实际范围应为十一条，并计算独立 actual hash。

## Authorization Contract

standing_authorization_ref: https://github.com/nonononull/inputcodex/issues/111
authorization_mode: bounded-standing-v1
exact_head_binding: required
policy_hash_binding: required
authorization_refresh_on_head_change: required
github_native_auto_merge: forbidden
merge_method: squash-only

每个自动合并 PR 必须在 Final Head 上回写：

- owner authorization ref
- policy schema version 与规范化 hash
- exact Final Head
- scope count 与 scope hash
- independent review 结论
- CI/Performance Run、Job 数与 Artifact 数
- Review threads 数
- origin/main freshness

## Runtime State Model

允许状态：

1. idle-select-candidate
2. active-issue-planning
3. active-worktree-execution
4. active-pr-review-ci
5. merge-ready-exact-head
6. post-merge-verification
7. blocked-hard-stop

任何时候最多一个写入者、一个 active 产品 Issue 和一个 active 产品 PR。多个 active 对象、未知状态、
release audit stale 或 main freshness 漂移必须 fail-closed。

## Agent Lifecycle

- writer：一个，拥有当前 worktree 写入权。
- research/reviewer/verifier：可并行，但必须只读并返回引用证据。
- completed、idle 且无 pending 子任务的 agent 在下一写入批次前归档。
- UI 只调用 Gemini；Gemini 不可用时标记 blocked-ui 并继续非 UI 队列。
- 不使用 subagent 代替精确 Head、CI 或 GitHub API 事实验证。

## Execution Batches

1. planning-freeze
2. policy-red
3. policy-green
4. state-red
5. state-green
6. local-closeout-and-review
7. pr-review-ci
8. exact-head-squash-and-main-verification
9. paseo-loop-activation-and-recovery-proof

每个批次结束必须提交普通 Git checkpoint；禁止 amend、rebase、force push 和 main 直接写入。

## Stop Conditions

- 路径越过候选 allowlist 或 scope hash 漂移。
- 修改产品代码、Cargo、Workflow、Runner、Release、upstream、Ruleset 或 AGOS。
- 尝试启用 GitHub 原生 auto-merge。
- 付费/self-hosted Runner、密钥/签名、正式 Release 或许可证例外。
- 自动合并缺少任一 exact Head、review、CI、Artifact、thread、release audit 或 freshness 证据。
- 无法调用 Gemini 且当前只剩 UI 候选。
- 所有剩余候选均因无法形成安全语义被隔离。
