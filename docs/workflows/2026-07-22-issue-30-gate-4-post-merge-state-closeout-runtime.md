# Runtime Workflow：Issue #30 Gate 4 合并后稳定状态收口

workflow_status: immutable-execution-contract
tracking_issue_ref: https://github.com/nonononull/inputcodex/issues/30
branch_ref: codex/issue-30-gate-4-post-merge-state-closeout
baseline_ref: c07da0cad33e09b5c54e528a8a6728a048c88c0b
approved_decision_ref: user-message:approve-issue-30-eight-path-scope-2026-07-22
scope_hash: sha256:e724713b647c77b0b9269435c82e68101f2a48c49e17cfff726160ad8259c11d
allowed_operations: project-doc-write, ordinary-commit, ordinary-push, issue-comment, pull-request-create, review-ci-evidence-read
mutation_intent: 将 Gate 4 已合并的 Closeout 证据固化为稳定状态，不触发任何性能或产品实现。
executor_enforcement: 分支、基线、八路径、Fresh 事实、仓库政策和验证结果任一异常即停止。
agos_status: bypassed-needs-input-unregistered
anti_recursion_contract: 持久文档不跟踪本 Issue 的动态 PR 状态；动态证据仅使用 Issue/PR 评论。

## Phase 0：启动与基线

1. 确认工作树分支为 `codex/issue-30-gate-4-post-merge-state-closeout`，基线为 `c07da0cad33e09b5c54e528a8a6728a048c88c0b`，主工作树保持在干净的 `main`。
2. 读取 `AGENTS.md`、README、`build.md`、`err.md`、Master Plan、Issue `#28` 报告和 Issue `#30`。
3. Fresh 核对 PR `#29` Squash、主干 CI `29948874307`、Issue `#28` 关闭、Ruleset、Review 和维护者数量。
4. 运行 AGOS `ReportOnly`；若结果为 `needs-input`、`unregistered`、接口不兼容或异常，记录为绕过并继续项目原生流程，不修改 AGOS。

## Phase 1：范围与稳定状态

1. 计算八路径 `scope_hash`，仅允许：

```text
AGENTS.md
README.md
docs/plans/2026-07-22-issue-30-gate-4-post-merge-state-closeout.md
docs/plans/PROJECT-MASTER-PLAN.md
docs/plans/sessions/2026-07-22-issue-30-gate-4-post-merge-state-closeout.md
docs/reports/issue-28-gate-4-feature-catalog-closeout.md
docs/reports/issue-30-gate-4-post-merge-state-closeout.md
docs/workflows/2026-07-22-issue-30-gate-4-post-merge-state-closeout-runtime.md
```

2. 将既有状态页更新为“Gate 4 功能目录与独立 Closeout 已完成；独立性能基线为下一合法 Gate；Gate 5 锁定”。
3. 创建 Issue `#30` 的四份控制面，所有动态 PR/CI/合并字段只描述处理规则，不能作为实时状态断言。

## Phase 2：轻量验证与 PR

1. 使用 `git diff --name-only c07da0cad33e09b5c54e528a8a6728a048c88c0b` 与 `git ls-files --others --exclude-standard` 的并集验证八路径。
2. 复算 `scope_hash`，扫描非法控制字节、行尾空白和“Issue #28 当前/待授权”旧文案。
3. 运行 `pwsh -NoProfile -File scripts/ci/Verify-RepositoryPolicy.ps1 -RepositoryRoot .` 与 `git diff --check`；本任务不运行本地 Rust 全量编译。
4. 普通提交、普通推送并创建非 Draft PR；正文说明稳定状态模型、八路径、哈希、禁止面和反递归边界。

## Phase 3：Review、CI 与合并

1. 只接受最终 PR Head 的 GitHub-hosted CI 作为合并证据；Review 对话必须逐条写明根因、处理方式和验证证据。
2. 合并前 Fresh 核对最终 Head、八路径、Ruleset、维护者数量、自动合并关闭、Review 对话和项目所有者针对最终 Head 的 Squash 授权。
3. 合并后在 Issue/PR 评论回写 Squash、`main` CI、Issue 关闭和源分支状态；不得为这些动态证据修改本 PR 或再创建同类状态收口 Issue。

## 停止条件

- 需要创建 `benchmarks/`、测量脚本、性能预算、优化、产品迁移或 Gate 5 功能。
- 需要修改 AGOS、上游、CI、Ruleset、Release 或删除任何分支。
- 已验证事实、范围、Ruleset、维护者数量、Review 或 CI 漂移。
