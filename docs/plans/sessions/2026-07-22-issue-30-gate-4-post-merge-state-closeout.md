# Session Plan：Issue #30 Gate 4 合并后稳定状态收口

session_status: approved-scope-frozen
tracking_issue_ref: https://github.com/nonononull/inputcodex/issues/30
branch_ref: codex/issue-30-gate-4-post-merge-state-closeout
baseline_ref: c07da0cad33e09b5c54e528a8a6728a048c88c0b
approved_decision_ref: user-message:approve-issue-30-eight-path-scope-2026-07-22
scope_hash: sha256:e724713b647c77b0b9269435c82e68101f2a48c49e17cfff726160ad8259c11d
allowed_operations: project-doc-write, ordinary-commit, ordinary-push, issue-comment, pull-request-create, review-ci-evidence-read
mutation_intent: 对已完成的 Gate 4 Closeout 建立稳定状态，不触发性能基线或产品实现。
executor_enforcement: 每批次前后检查工作树、基线、允许路径和验证结果；任何漂移立即停止。
agos_status: bypassed-needs-input-unregistered
anti_recursion_contract: 状态页只描述稳定终态；Issue #30 的动态执行与合并证据只保留在 Issue/PR 评论。

## 一、输入与决定

- 项目所有者于 `2026-07-22` 批准方案 A：先完成 Gate 4 合并后稳定状态收口，再进入独立性能基线设计与执行。
- 项目所有者随后批准八路径范围与 `scope_hash`，批准证据为 `user-message:approve-issue-30-eight-path-scope-2026-07-22`，Issue 评论为 `5050458742`。
- 本机 `main` 已通过已验证的 SSH 路径 fast-forward 到 `c07da0cad33e09b5c54e528a8a6728a048c88c0b`；本会话使用独立工作树，不修改 `main`。
- AGOS `ReportOnly` 返回 `needs-input`、`unregistered` 与 `doctor=blocked`；按项目规则绕过，不登记、不修复、不优化 AGOS。

## 二、范围锁定

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

路径升序、LF 分隔、末尾 LF 的 SHA-256 为 `e724713b647c77b0b9269435c82e68101f2a48c49e17cfff726160ad8259c11d`。

## 三、稳定状态模型

- `AGENTS.md`、README 和 Master Plan 只声明：Gate 4 功能目录与独立 Closeout 已完成；独立性能基线是下一合法 Gate；Gate 5 锁定。
- Issue `#28` 报告补齐终态证据，但不假装源分支已删除。
- Issue `#30` 的 Head、CI、Review、Squash 和任何分支状态由 Issue/PR 评论承载；这些动态字段不写入项目“当前任务”文案。

## 四、验证合同

1. 以 `git diff --name-only c07da0cad33e09b5c54e528a8a6728a048c88c0b` 与 `git ls-files --others --exclude-standard` 的并集核对八路径。
2. 重算 `scope_hash`，扫描非法控制字节与旧的“Issue #28 当前/待授权”状态文案。
3. 运行 `pwsh -NoProfile -File scripts/ci/Verify-RepositoryPolicy.ps1 -RepositoryRoot .` 与 `git diff --check`。
4. 本任务没有 Rust 或产品行为改动；只运行 `build.md` 定义的文档治理轻量验证，完整跨平台验证交由 PR CI。

## 五、PR 与合并边界

- PR 正文列明八路径、哈希、稳定状态模型、禁止面和本地验证；自动合并保持关闭。
- 所有 Review 对话必须写明根因、处理方式和验证证据后才能解决。
- 合并前必须 Fresh 核对最终 Head、CI、Review、Ruleset、维护者数量和项目所有者的独立 Squash 授权。
- 合并后只在 Issue/PR 回写动态证据；不得为本状态收口再创建同类 Closeout Issue。
