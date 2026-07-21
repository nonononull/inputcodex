# Session Plan：Issue #6 Gate 1 仓库治理基线最终收口

schema_version: inputcodex.session-plan.v1
task_id: 2026-07-21-issue-6-gate-1-finalization
work_class: standard
task_status: implementation
task_summary: 补齐仓库模板、标签、closeout 证据和项目原生验证，完成 Gate 1 最终治理收口。
project_root: C:/Users/dashuai/Documents/inputcodex
tracking_issue_ref: https://github.com/nonononull/inputcodex/issues/6
branch_ref: codex/issue-6-gate-1-finalization
baseline_ref: b7404b0c63f2d2ba65474c077182c42a01cc9a64
decision_status: approved
approved_decision_ref: user-message:2026-07-21-start-gate-1-governance-closeout
session_plan_ref: docs/plans/sessions/2026-07-21-issue-6-gate-1-finalization.md
implementation_plan_ref: docs/plans/2026-07-21-issue-6-gate-1-finalization.md
runtime_workflow_ref: docs/workflows/2026-07-21-issue-6-gate-1-finalization-runtime.md
scope_hash: sha256:bd271f22bfb20eb32a78d99ae89e9554cc2de17c8486d1f4386b8980068659bd
mutation_intent: 修改 inputcodex 仓库治理文档与 GitHub 元数据，不修改产品源码、CI、Ruleset、Release 或外部仓库。
executor_enforcement: 只允许在 codex/issue-6-gate-1-finalization 分支写入；提交前执行 build.md Fresh 验证；禁止 Force Push 与自动合并。
external_agos_policy: available-optional-unavailable-bypass-no-optimization
external_agos_execution: skipped-by-approved-project-boundary

## 会话目标

- 把 Issue `#4` / PR `#5` 的最终事实写回项目控制面。
- 建立八类 Issue Forms、PR 模板与批准标签。
- 固化“所有 Review 对话必须完成根因、处理和验证证据闭环”的贡献入口。
- 证明 Gate 1 仍不包含源码、Cargo、Actions 或 Release。
- 创建 Issue `#6` 的关联 PR 后停止在项目所有者 Review 前。

## 允许写入

- `README.md`
- `build.md`
- `docs/plans/PROJECT-MASTER-PLAN.md`
- `docs/plans/2026-07-21-architecture-governance.md`
- `docs/plans/2026-07-21-issue-6-gate-1-finalization.md`
- `docs/plans/sessions/2026-07-21-issue-6-gate-1-finalization.md`
- `docs/workflows/2026-07-21-issue-6-gate-1-finalization-runtime.md`
- `docs/reports/issue-4-gate-1-closeout.md`
- `.github/ISSUE_TEMPLATE/*.yml`
- `.github/pull_request_template.md`
- GitHub 项目标签、Issue `#6` 标签与 Issue `#1` 追踪评论。
- PR `#5` 已合并旧分支的安全清理。

## 禁止写入

- `upstream/` 快照内容。
- Rust、Iced、Cargo、TypeScript、JavaScript 或 WebView 代码。
- `.github/workflows/`。
- GitHub Ruleset、required checks、仓库级合并开关和 Release。
- `main` 分支历史。
- AGOS 或任何外部仓库。

## 关键判断

- 性能优先、功能一致、双平台一致、Rust + Iced 和无广告均为不可变硬约束。
- `BigPizzaV3/CodexPlusPlus` 最新正式 Release 是功能真源；`main` 只作变化预警。
- 上游 UI、注入脚本与远程推荐列表不进入最终运行面。
- Issue Form 只建立治理入口，不代表相应 Gate 已解锁。
- 单人阶段 required approvals 为 `0`，但必须保留项目所有者决策证据；多人阶段再通过独立 Issue/PR 升级为 `1`。
- Checks 数量为 `0` 只表示尚未配置 CI，不得表述为 CI 已通过。

## 执行检查点

- `startup-baseline`：确认 `main` 与 GitHub `main` 同为 `b7404b0c...`，工作树干净。
- `issue-and-branch`：Issue `#6` 已创建，当前分支为 `codex/issue-6-gate-1-finalization`。
- `after-docs`：运行 `git diff --check`、文件清单和禁止表面检查。
- `after-metadata`：核验标签全集、Issue `#6` 标签和默认标签保留。
- `pre-verification`：重新抓取 Issue、PR、Ruleset、上游 Release 与旧分支状态。
- `pre-commit`：核对 staged diff 只包含批准范围。
- `handoff`：核对本地 HEAD、远端分支与 PR Head 一致，PR 非 Draft、保持开放、无未解决 Review 对话。

## 异常处理

- 任何失败先查 `err.md`，复用已有根因与处理流程。
- GitHub 事实变化时停止写入，重新抓取权威证据后再判断。
- YAML 解析失败时只修复对应模板，不扩大到产品实现。
- Git HTTPS 传输失败时先区分 Git transport 与 GitHub API，不盲目连续重试。
- 发现需要修改 AGOS 时记录外部缺口并绕过，不在本任务修复。

## 交付证据

tracking_issue_ref: https://github.com/nonononull/inputcodex/issues/6
review_ref: pending:project-owner-review
pr_ref: pending:issue-6-pull-request
ci_ref: not-configured:gate-1-workflows-0
merge_ref: none:owner-authorization-required
closeout_ref: pending:issue-6-pr-merge
