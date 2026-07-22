# Issue #30：Gate 4 合并后稳定状态收口实施计划

plan_status: approved-scope-frozen
tracking_issue_ref: https://github.com/nonononull/inputcodex/issues/30
branch_ref: codex/issue-30-gate-4-post-merge-state-closeout
baseline_ref: c07da0cad33e09b5c54e528a8a6728a048c88c0b
approved_decision_ref: user-message:approve-issue-30-eight-path-scope-2026-07-22
scope_hash: sha256:e724713b647c77b0b9269435c82e68101f2a48c49e17cfff726160ad8259c11d
allowed_operations: project-doc-write, ordinary-commit, ordinary-push, issue-comment, pull-request-create, review-ci-evidence-read
mutation_intent: 将已合并 Gate 4 Closeout 回写为稳定终态；不创建性能基线、预算、优化或产品行为。
executor_enforcement: 仅允许八路径；范围外写入、事实漂移、未解决 Review、CI 失败或缺少最终 owner 授权均停止推进。
agos_status: bypassed-needs-input-unregistered
anti_recursion_contract: 永久控制面不得把 Issue #30、其分支或 PR 标为当前待合并任务；动态证据只写 Issue/PR 评论。

## 一、目标

将 Issue `#28` / PR `#29` 的已完成事实写入稳定项目状态：Gate 4 功能目录和独立 Closeout 已完成，独立性能基线是下一项可启动工作，Gate 5 仍保持锁定。

## 二、已验证基线

- PR `#29` 最终 Head 为 `7ee316c6bf4d9ca44f3475283ae1aee9c83f8577`，于 `2026-07-22T18:56:38Z` 以单父 Squash 提交 `c07da0cad33e09b5c54e528a8a6728a048c88c0b` 合入 `main`。
- Squash tree 为 `02ab8a3d8497ebb7b990e4078122b9bf916ef454`，GitHub 签名有效；Issue `#28` 于 `2026-07-22T18:56:39Z` 关闭。
- PR CI `29948253910` 成功、Review 对话为 `0`、Artifact 为 `0`；合并后 main CI `29948874307` 六个 Job 全成功、Artifact 为 `0`。
- AGOS `ReportOnly` 对任务 `inputcodex-issue-30-gate-4-post-merge-state-closeout` 返回 `needs-input` 与 `unregistered`；按项目规则绕过，不改动任何 AGOS 跨仓表面。

## 三、范围与文件职责

允许路径按升序、LF 分隔并保留末尾 LF 后计算得到本计划顶部的 `scope_hash`：

1. `AGENTS.md`：写入 Gate 4 的稳定完成态与独立性能基线前置条件。
2. `README.md`：更新项目摘要、当前阶段、禁止面与下一步。
3. `docs/plans/2026-07-22-issue-30-gate-4-post-merge-state-closeout.md`：记录不可递归的执行合同。
4. `docs/plans/PROJECT-MASTER-PLAN.md`：将活动任务清空，记录已完成 Closeout 与下一合法 Gate。
5. `docs/plans/sessions/2026-07-22-issue-30-gate-4-post-merge-state-closeout.md`：记录会话输入、范围、批准和 AGOS 绕过。
6. `docs/reports/issue-28-gate-4-feature-catalog-closeout.md`：补齐 PR `#29`、Squash、主干 CI、Issue 关闭和分支保留的最终证据。
7. `docs/reports/issue-30-gate-4-post-merge-state-closeout.md`：记录稳定状态目标与外部动态证据边界。
8. `docs/workflows/2026-07-22-issue-30-gate-4-post-merge-state-closeout-runtime.md`：定义范围验证、PR 收口和反递归流程。

禁止修改 Rust、Cargo、`parity/`、`benchmarks/`、`.github/`、`scripts/ci/`、`upstream/`、Ruleset、Release、AGOS 和任何分支删除表面。

## 四、执行步骤

1. 将四份 Issue `#30` 控制面创建为不可递归合同：它们可记录批准、范围、基线和验证命令，但不宣称自身 PR/CI/合并状态。
2. 将 `AGENTS.md`、README、Master Plan 和 Issue `#28` 报告更新为已验证的稳定终态；不把 Issue `#30` 写入这些文档的“当前任务”字段。
3. 使用基线差异路径与未跟踪文件并集验证八路径、复算 `scope_hash`，再运行仓库政策、文本控制字节和 Git 空白检查。
4. 普通提交、普通推送并创建非 Draft PR；将 Head、Review、CI、项目所有者 Squash 决策和最终合并证据仅回写 Issue/PR 评论。

## 五、完成标准

1. 持久控制面不再把 Issue `#28` / PR `#29` 记为当前或待授权。
2. 持久控制面不把 Issue `#30`、其分支或 PR 记为当前待合并任务。
3. PR `#29` 的 Squash、主干 CI、Issue 关闭和源分支保留状态均有可复核证据。
4. 净变更精确等于八路径，仓库政策、控制字节和 `git diff --check` 通过。

## 六、停止条件

- 需要修改八路径之外的任一文件，或需要创建性能基线、预算、优化、产品迁移、上游同步或一致性例外。
- PR `#29`、`main`、Ruleset、维护者数量、已验证 CI 或 Issue `#28` 状态发生物质漂移。
- 需要删除源分支但没有项目所有者的单独明确授权。
