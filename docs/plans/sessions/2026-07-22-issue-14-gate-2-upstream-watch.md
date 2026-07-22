# Issue #14 Session Plan：Gate 2 上游变化监控

session_status: approved-for-execution
task_id: issue-14-gate-2-upstream-watch
tracking_issue_ref: https://github.com/nonononull/inputcodex/issues/14
base_ref: main@5e64015075ddf2adef4bf685f50977b47b7f72e7
branch_ref: codex/issue-14-gate-2-upstream-watch
session_plan_ref: docs/plans/sessions/2026-07-22-issue-14-gate-2-upstream-watch.md
approved_decision_ref: user-message:create-upstream-watch-full-delivery-2026-07-22
scope_hash: 3d8764a4844a1d4c06eba2c7998766b2fa8f88da5f2232169fd24e997e25daef
mutation_intent: 创建只管理 GitHub Issue 的每 6 小时上游变化监控，并完成 PR、Review、CI 与 Squash Merge 收口
executor_enforcement: 仅允许本文件列出的路径；越界、上游基线变化、Review 未闭环或 Fresh 验证失败时立即停止

## 允许操作

- 读取公开 GitHub API、当前仓库控制面和 `upstream/source-lock.json`。
- 创建和修改本任务允许路径内的 Python、Workflow 与 Markdown 文件。
- 运行无网络单元测试、Workflow 静态验证和 Git/GitHub Fresh 验证。
- 普通提交、普通 push、创建 PR、运行/观察 GitHub Actions、处理 Review 对话。
- 在全部门禁满足后，使用项目所有者已记录的条件式授权执行 Squash Merge，并关闭 Issue。

## 允许路径

1. `.github/scripts/tests/test_upstream_watch.py`
2. `.github/scripts/upstream_watch.py`
3. `.github/workflows/upstream-watch.yml`
4. `README.md`
5. `build.md`
6. `docs/plans/2026-07-22-issue-14-gate-2-upstream-watch.md`
7. `docs/plans/PROJECT-MASTER-PLAN.md`
8. `docs/plans/sessions/2026-07-22-issue-14-gate-2-upstream-watch.md`
9. `docs/reports/issue-14-gate-2-upstream-watch.md`
10. `docs/workflows/2026-07-22-issue-14-gate-2-upstream-watch-runtime.md`
11. `err.md`

上述路径按 POSIX 形式排序并以 LF 结尾计算 SHA-256，结果为本文件的 `scope_hash`。

## 明确禁止

- 修改 `upstream/`、`upstream/source-lock.json`、Cargo/Rust/Iced 产品源码、UI、Release、Artifact、Ruleset、仓库合并开关或 AGOS。
- 使用 TypeScript、JavaScript 业务代码、WebView、本地 Runner、self-hosted Runner、Larger Runner 或付费 CI。
- 自动同步、自动创建功能 PR、自动合并上游变化、Force Push 或删除 `main`。
- 更新没有 inputcodex 精确机器标记的 Issue，或在发现多个相同机器标记时任意选择一个 Issue。

## Fresh 上游基线

- 最新正式 Release：`v1.2.41`。
- Release 发布时间：`2026-07-20T01:48:40Z`。
- Release 标签提交：`3dafffcafb2566a1e8bce4b35671656d6adb3eda`。
- 上游 `main`：`6fa0a57decbb3382771a981247e6922799e97f5d`，仅作预警源。

## 外部治理处理

AGOS 只作为可选外部辅助。本任务已有真实 Issue、项目原生计划、允许路径和项目所有者批准，当前不调用或修改 AGOS；该绕过不阻塞 inputcodex 原生交付链。

## 停止条件

- `upstream/source-lock.json`、最新正式 Release 或其标签提交在实现期间发生未解释变化。
- diff 出现允许路径之外的文件，或需要修改 Ruleset、Release、产品源码、快照或 AGOS。
- PR 验证 Job 获得写权限，或定时/手动监控 Job 超出 `contents: read` 与 `issues: write`。
- 单元测试、真实 Actions 运行、Review 对话、Ruleset或合并前 Fresh 状态无法闭环。
