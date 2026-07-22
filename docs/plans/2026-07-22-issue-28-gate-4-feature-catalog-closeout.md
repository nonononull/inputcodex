# Issue #28：Gate 4 功能目录执行合并证据收口实施计划

plan_status: approved-control-plane-in-progress
tracking_issue_ref: https://github.com/nonononull/inputcodex/issues/28
source_issue_ref: https://github.com/nonononull/inputcodex/issues/26
source_pr_ref: https://github.com/nonononull/inputcodex/pull/27
branch_ref: codex/issue-28-gate-4-feature-catalog-closeout
approved_decision_ref: user-message:approve-gate-4-closeout-issue-2026-07-22
scope_hash: sha256:91cd1bd908b61e32c573706f26a4bb5d09c6cf5371382ebc0d14d87ae7a4fc29
baseline_ref: a9b20f00ae069aedd42c8124d2789b230187258c
allowed_operations: project-doc-write, ordinary-commit, ordinary-push, issue-comment, pull-request-create, review-ci-evidence-read
mutation_intent: 仅回写 Gate 4 功能目录执行已经发生的 GitHub 合并事实；不改变产品、性能、上游或 CI 行为。
executor_enforcement: 每个执行批次前后检查工作树、分支、允许路径与空白错误；范围外写入、证据漂移或 GitHub 状态不一致立即停止。
agos_status: bypassed-needs-input-unregistered

## 一、目标

将 Gate 4 功能目录、行为合同与脱敏夹具执行从来源 PR 的动态状态，收口为项目原生控制面中的可复核事实。来源 Issue `#26` 已关闭，来源 PR `#27` 已 Squash Merge；本任务不实现新功能，也不建立性能基线。

## 二、冻结基线

- `main` 基线与来源 PR 合并提交均为 `a9b20f00ae069aedd42c8124d2789b230187258c`。
- Issue `#26` 于 `2026-07-22T17:41:13Z` 关闭。
- PR `#27` 于 `2026-07-22T17:41:11Z` 合并；最终 Head 为 `1d1bf32cdc4edc45e2d28f1047604222ebdb51e4`。
- PR `#27` 的 `classify`、`governance`、`linux-quality`、`windows`、`macos` 与 `required` 均成功；合并后 `main` CI 运行 `29943399832` 成功。
- 项目所有者批准引用为 `user-message:approve-gate-4-closeout-issue-2026-07-22`。

## 三、范围与文件职责

本任务只允许以下七条路径，路径升序、LF 分隔并保留末尾 LF 后得到本计划顶部的 `scope_hash`：

1. `README.md`：将当前阶段描述从“PR 待创建”更新为已合并的 Gate 4 功能目录状态，并明确下一步是独立 Closeout 后的性能基线 Issue。
2. `docs/plans/2026-07-22-issue-28-gate-4-feature-catalog-closeout.md`：记录本实施计划、范围、步骤与停止条件。
3. `docs/plans/PROJECT-MASTER-PLAN.md`：回写活动任务、Gate 4 清单和后续性能基线前置条件。
4. `docs/plans/sessions/2026-07-22-issue-28-gate-4-feature-catalog-closeout.md`：记录会话决策、范围、AGOS 绕过原因和执行检查点。
5. `docs/reports/issue-26-gate-4-feature-catalog.md`：将来源 PR、CI、Review、Squash 合并与分支清理字段回写为真实证据。
6. `docs/reports/issue-28-gate-4-feature-catalog-closeout.md`：形成 Closeout 证据报告并记录本 PR 的动态收口状态。
7. `docs/workflows/2026-07-22-issue-28-gate-4-feature-catalog-closeout-runtime.md`：定义逐批执行、停机与验证流程。

禁止改动 `Cargo.toml`、`Cargo.lock`、Rust、`parity/`、`benchmarks/`、`.github/`、`scripts/ci/`、`upstream/`、Ruleset、Release、Issue `#16/#20` 与任何 AGOS 跨仓文件。

## 四、执行任务

### Task 1：建立 Closeout 任务控制面

- [ ] 新建本计划、Session Plan、Runtime Workflow 与初始 Closeout 报告。
- [ ] 在四份新文档中统一记录 Issue `#28`、来源 Issue/PR、批准引用、基线、七路径范围、`scope_hash`、`allowed_operations`、`mutation_intent` 与 `executor_enforcement`。
- [ ] 记录 AGOS `-ReportOnly` 返回 `needs-input` 与 `unregistered`；按项目规则绕过，不修改 AGOS、Registry、Workflow 或 Vault。
- [ ] 运行本计划的范围自检：工作树只包含四条新增路径，且范围哈希等于 `sha256:91cd1bd908b61e32c573706f26a4bb5d09c6cf5371382ebc0d14d87ae7a4fc29`。
- [ ] 创建普通 Git checkpoint、推送分支，并在 Issue `#28` 回写 checkpoint 提交、计划引用与范围哈希。

### Task 2：回写来源执行与项目状态

- [ ] 通过 GitHub CLI Fresh 查询 Issue `#26`、PR `#27`、最终 Head、Squash 提交、Review 对话、PR CI、合并后 `main` CI 与远端分支状态。
- [ ] 将 `README.md`、Master Plan 和 Issue `#26` 报告中的“PR 待创建 / CI 待运行 / 待合并”状态替换为上述真实证据。
- [ ] 在 Issue `#28` Closeout 报告中说明：来源 PR 合并后不能改写来源提交，因此必须通过独立 PR 回写控制面；不得把该事实当作功能迁移或性能基线授权。
- [ ] 保留 Gate 4 性能基线为下一张独立 Issue，且禁止在本任务创建 `benchmarks/`、预算、样本或优化代码。

### Task 3：执行轻量验证与 PR 准备

- [ ] 检查工作树路径集合与七路径允许集合完全一致。
- [ ] 使用 `gh issue view 26`、`gh pr view 27`、`gh run view 29942593564` 与 `gh run view 29943399832` 复核来源事实；若任一状态、Head、CI 或 Review 对话漂移，停止并在 Issue `#28` 记录。
- [ ] 运行 `pwsh -NoProfile -File scripts/ci/Verify-RepositoryPolicy.ps1 -RepositoryRoot .`、文本控制字节扫描与 `git diff --check`；本任务不触发本地 Rust 全量编译。
- [ ] 普通提交、普通推送并创建关联 PR；PR 非 Draft、自动合并关闭、无未解决 Review 对话后，等待 GitHub-hosted CI。

### Task 4：Review、CI 与合并边界

- [ ] PR 的每条 Review 对话必须回写根因、处理方式与验证证据；不成立的反馈必须留存可复核证据与 reviewer 或项目所有者确认。
- [ ] 最终 Head 的 CI、允许路径、Ruleset、维护者人数、Review 对话和项目所有者授权必须 Fresh 一致。
- [ ] 未获得针对本 Closeout PR 的明确项目所有者 Squash Merge 授权前，禁止合并。

## 五、完成标准

1. 项目原生控制面不再把 Issue `#26` / PR `#27` 标记为待创建、待 CI 或待合并。
2. 来源 Issue、PR、Head、Squash、Review、PR CI、合并后 CI 和分支清理均有可复核证据。
3. 工作树只触及七条允许路径，仓库政策、文本控制字节扫描与 `git diff --check` 通过。
4. Closeout PR 通过 Review/CI 并在项目所有者明确授权后 Squash Merge。

## 六、停止条件

- `main`、Issue `#26`、PR `#27`、Ruleset、维护者数量、来源 CI 或批准引用发生物质漂移。
- 需要改动七路径以外任一文件，或需要创建性能基线、产品功能、优化、预算或一致性例外。
- AGOS 的外部状态诱导本仓修改其脚本、Registry、Workflow、Vault 或任何跨仓控制面。
- Fresh 验证失败、存在未解决 Review 对话，或没有本 Closeout PR 的项目所有者合并授权。
