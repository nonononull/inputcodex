# Issue #105 Session Plan：quick-xml RustSec 高危公告修复

## Session Metadata

- `schema_version`: `agos.session-plan.v1`
- `architecture_contract_version`: `agos.brainstorming-gate.v1`
- `task_id`: `issue-105-quick-xml-rustsec-remediation`
- `tracking_issue_ref`: `https://github.com/nonononull/inputcodex/issues/105`
- `approved_decision_ref`: `https://github.com/nonononull/inputcodex/issues/105#issuecomment-5129028445`
- `approval_source`: `direct-user-and-handoff-approved`
- `decision_status`: `approved`
- `selected_business_path`: `maintenance/dependency-security/quick-xml-rustsec-remediation`
- `baseline_ref`: `origin/main@060ca045d2c134f8be3c9adc8cdb038842fc3243`
- `branch_ref`: `codex/issue-105-quick-xml-rustsec-remediation`
- `worktree_ref`: `.worktrees/issue-105-quick-xml-rustsec-remediation`
- `work_class`: `standard`
- `delivery_contract`: `agos.issue-pr-merge.v1`
- `planning_scope_hash`: `sha256:25337a6cb90386439878af2bd8be7d00af0276e102663ab6add7e3d3b4621a09`
- `candidate_scope_hash`: `sha256:0c90d018e06aa640d33a4c65c75aea45c89eb0e365b91fadee803b0426c8c58f`
- `execution_state`: `LOCAL_VERIFIED_PR_PENDING`
- `planning_checkpoint_ref`: `c266cab2570ce477bc6079b951cd9e79f5abe4a0`
- `security_checkpoint_ref`: `7c890e3137a503cac334e2802bd2441feae41052`
- `local_verified_checkpoint_ref`: `pending`
- `agos_report_only`: `blocked-unregistered-missing-owner-scope-manifest-bypassed`
- `review_strategy`: `single-main-thread-fresh-local-verification-and-github-review`
- `ci_expectation`: `standard-hosted-ci-and-performance-no-success-artifacts`
- `merge_policy`: `squash-only-final-head-owner-authorization`

## Approved Decision

项目所有者批准安全优先路线并下达“ok，开始吧”：先处理 Issue #105，在其完整收口前不继续
Gate 5 产品功能。实现只允许将 `wayland-scanner` 精确更新到 `0.31.11`，由其把
`quick-xml` 提升到 `0.41.0`；不升级 Iced、不改产品功能、不改 Workflow/Ruleset，最终
Squash Merge 仍需绑定 Final Head 的独立授权。

## Mutation Intent

- `mutation_intent`: `config`
- `mutation_target`: `Cargo.lock 中 wayland-scanner/quick-xml 两个 package block及九路径项目控制面`
- `requested_operations`: `plan, lockfile-update, verify, commit, push, pr, review-response`
- `allowed_operations`: 只允许九路径内的锁文件精确更新、项目文档、验证与 Git 交付。
- `forbidden_operations`: `main-write, force-push, merge-without-final-head-approval, scope-expansion, product-edit, ci-edit, AGOS-edit`

## Executor Enforcement

- 当前阶段为 `planning-freeze`；planning checkpoint 与 AGOS report-only 完成后才执行锁文件更新。
- TDD 证据使用 security audit RED/GREEN：先保留两条漏洞与非零退出码，再做最小锁文件修复。
- 错误先查 `err.md`；本次 advisory-db 直连失败是新可复用根因，进入同一批准范围。
- 不派发 subagent：任务路径单一且用户要求本轮不使用 subagent；独立信号来自 fresh audit、
  本地门禁、GitHub Review 和三平台 Hosted CI。
- 不创建 `docs/superpowers/*` 第二控制面，不修改外部 AGOS。
- 每个稳定批次运行 Git snapshot checkpoint，并使用本机默认 Git 时间建立命名提交。

## Local Knowledge Lookup

```yaml
local_knowledge_lookup:
  gbrain_queries:
    - quick-xml RustSec cargo audit
    - Rust dependency security lockfile transitive dependency
    - Issue 105 inputcodex
  vault_refs:
    - no-task-specific-hit
  rules_refs:
    - components/rules/rules/workflows/ai-growth-os-auto-application.md
    - components/rules/rules/workflows/ai-growth-os-brainstorming-gate.md
  project_refs:
    - AGENTS.md
    - README.md
    - build.md
    - err.md
    - docs/plans/PROJECT-MASTER-PLAN.md
    - Issue #105 and Issue #104/PR #106 evidence
  missing_coverage:
    - GBrain and vault contain no task-specific dependency-remediation entry
    - repository has no .codegraph index and must not initialize one implicitly
```

三个 GBrain 查询和 rules/vault 关键词扫描均无 task-specific 命中。执行因此复用项目既有
Issue/PR、范围 hash、TDD、Hosted CI 与 Review 合同；缺口记为本任务 audit-only 证据，不阻塞
项目原生流程。

## Brainstorming Gate

- `superpowers_skill`: `superpowers:brainstorming`
- `proposal_mode`: `not-required`
- `fallback_reason`: `clear-low-risk-single-lockfile-route-and-owner-approved-no-subagent`
- `actual_agent_count`: `0`
- `approved_decision_ref`: `https://github.com/nonononull/inputcodex/issues/105#issuecomment-5129028445`
- `scope_hash`: `sha256:0c90d018e06aa640d33a4c65c75aea45c89eb0e365b91fadee803b0426c8c58f`
- `allowed_operations`: `nine-path-lockfile-remediation-and-delivery`
- `mutation_intent`: `config`
- `executor_enforcement`: `exact-path-and-exact-package-diff-hard-stop`

候选比较已经在主线程完成：推荐最小上游 patch，拒绝叶子强锁、Iced 整链升级和仅记录不可达性。
用户批准后范围与风险没有变化，因此不重复询问同一决定。

## Change Contract

- `target_contract`: fresh RustSec 扫描从两条 high 漏洞转为 `vulnerabilities.count=0`。
- `preserved_invariants`: manifests、产品语义、Iced/winit/smithay 版本、三平台 CI、Release Audit、
  Ruleset、Artifact 与上游缓存不变。
- `adjacent_surfaces`: Linux Wayland 代码生成、Windows/macOS 依赖解析、锁文件 checksum、许可证、
  MSRV 与十个既有 Gate 5 产品切片。
- `historical_state_refs`: 基线 Cargo.lock、Issue #105 RED 评论、Issue #104/PR #106 稳定事实。
- `stale_verdict_invalidation_refs`: 本地 GREEN 不代表 main 已修复；只有 Squash Merge 后才能解除
  主干安全暂停。
- `regression_checks`: package-level lock diff、fresh cargo audit、metadata、fmt、CI contract、
  repository policy、release audit、diff check、Hosted CI/Performance/Artifact。
- `sibling_regression_guard`: `pending-final-head-ci-review-artifact`

## Scope Boundary

Planning 七路径：

```text
AGENTS.md
build.md
docs/plans/2026-07-30-issue-105-quick-xml-rustsec-remediation.md
docs/plans/PROJECT-MASTER-PLAN.md
docs/plans/sessions/2026-07-30-issue-105-quick-xml-rustsec-remediation.md
docs/workflows/2026-07-30-issue-105-quick-xml-rustsec-remediation-runtime.md
err.md
```

完整九路径在实施计划中定义；新增的 `Cargo.lock` 与 report 只能在 planning checkpoint 后进入。

## Execution Batches

### Batch 0：Startup 与范围冻结

状态：`completed`。独立 worktree 基于 `origin/main@060ca045...` 创建，工作树干净；Issue 评论
冻结九路径与 hash；fresh audit 真实 RED 已保存。

### Batch 1：Planning Freeze

状态：`completed`。七路径 planning hash、CI contract、Repository Policy、Release Audit、metadata、
Markdown 链接和 diff check 通过，checkpoint 为 `c266cab2570ce477bc6079b951cd9e79f5abe4a0`；
干净 Git snapshot 为 `GIT_SNAPSHOT_READY`。AGOS default-entry 返回
`needs-input/unregistered/missing-owner-scope-manifest`，按项目规则绕过且未修改 AGOS。

### Batch 2：Security TDD

状态：`completed`。checkpoint 为 `7c890e3137a503cac334e2802bd2441feae41052`；
`Cargo.lock` 只发生两个 package block 的 `4/4` 版本/checksum 变化。fresh audit 扫描 `350`
个依赖后退出 `0`、漏洞 `0`；`paste` 与 `ttf-parser` 两条 unmaintained warning 单独保留。

### Batch 3：Local Closeout

状态：`completed`。本机完整门禁输出 `ISSUE_105_LOCAL_GREEN`；audit 漏洞 `0`、warning `2`、
CI contract `35/35`、Repository Policy `0` 违规、Release Audit `current`，九路径与 scope hash
精确匹配。报告已创建，local-verified checkpoint 正在建立。

### Batch 4：Review / CI

状态：`pending`。普通 push、非 Draft PR、Review 根因闭环、CI/Performance/Artifact 核验，形成
稳定 Final Head 并停在独立 Squash Merge 授权门。

## Agent Lifecycle

```yaml
agent_lifecycle:
  budget:
    max_total_agents: 0
    max_new_agents_per_round: 0
    actual_agent_count: 0
  spawn_preconditions:
    reclaim_before_spawn: not-needed-zero-open
    open_agent_count_before_dispatch: 0
  active_agent_refs:
    - none
  completion_status:
    completed:
      - none
    idle:
      - none
    timeout:
      - none
    failed:
      - none
  closed_agent_refs:
    - none
  owner_exception_ref: current-thread-no-subagent-direction
```

## Skill Tree

- `superpowers:using-superpowers`
- `superpowers:brainstorming`
- `superpowers:writing-plans`
- `superpowers:executing-plans`
- `superpowers:test-driven-development`
- `superpowers:systematic-debugging`
- `karpathy-guidelines`
- `security-review`
- 完成前使用 `superpowers:verification-before-completion`、`requesting-code-review` 和
  `finishing-a-development-branch`。

## Checkpoint And Stop Rules

- startup baseline、planning、local-verified、push/PR 前与 handoff 前都运行 Git snapshot checkpoint。
- 每次 commit/push/PR 前重新检查 Git diff、九路径与 package-level 锁文件差异。
- Git 时间仅来自本机 `Get-Date`；不设置 `GIT_AUTHOR_DATE` 或 `GIT_COMMITTER_DATE`。
- 永久禁止 force push 或删除 `main`。
- 范围/hash 漂移、第三个 package 变化、许可证/MSRV/三平台失败、新漏洞、Release Audit stale、
  AGOS 写入需求或 Final Head 合并未授权均为硬停止条件。

## Next Gate

Planning checkpoint 后直接进入 Security GREEN，不再逐项请求批准。只有触发停止条件或形成可合并
Final Head 时重新请求项目所有者决策。
