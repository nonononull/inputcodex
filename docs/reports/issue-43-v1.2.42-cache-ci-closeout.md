# Issue #43：`v1.2.42` 缓存与 CI 合同状态收口报告

report_status: static-closeout-evidence
tracking_issue_ref: https://github.com/nonononull/inputcodex/issues/43
branch_ref: codex/issue-43-v1.2.42-cache-ci-closeout
baseline_ref: 353391424db5514d022473ba97f601486a190869
approved_decision_ref: user-message:批准-Issue-43-十路径与-scope_hash-允许实施轻量验证提交推送PRReviewCI-2026-07-25
scope_hash: sha256:39a26c9301f8ca792498be73117320b1d26c3028fdc1c558703ee7cefb96061e
allowed_operations: project-doc-write, lightweight-verification, ordinary-commit, ordinary-push, pull-request-create, review-ci-evidence-read
mutation_intent: docs
executor_enforcement: exact-ten-path-set, stable-fact-only, normal-push-only, squash-merge-only
agos_status: optional-reportonly-checkpoint-ready; default-entry-bypassed-needs-input-unregistered
anti_recursion_contract: 本报告不预写 Issue #43 自身的 PR、Review、CI、Squash 或合并后主干事实；这些动态证据必须在 GitHub Issue/PR 中记录，永久状态漂移只能由新的独立 Issue 修正。

## 一、已收口的稳定事实

- Issue `#35` / PR `#36` 已解耦活动 Release 快照与功能目录审计基线，Squash 提交为 `d7438a0f2c43b7fbd2b159b3759aacea4ef1999e`。
- Issue `#34` / PR `#40` 已将上游正式 Release `v1.2.42` / `657cd33e009ad02515d30db6492cd4e669b06318` 同步为只读审计缓存；PR `#40` Squash 提交为 `353391424db5514d022473ba97f601486a190869`，Issue 已关闭，合并后主干 Run `30147841226` 七 Job 全绿。
- Issue `#41` / PR `#42` 已修复固定 Release 对象与合法 stale 状态的错误断言；PR `#42` Squash 提交为 `8aa1d4c96b0543e766b477b1b8e9652968b55f92`，Issue 已关闭，合并后主干 Run `30147071062` 七 Job 全绿。
- `upstream/source-lock.json` 的 `release_audit.status` 保持 `stale-re-audit-required`；功能目录仍固定在 `v1.2.41`，只有 Issue `#38` 的独立重新审计能恢复 `current`。

## 二、PR `#40` 基线重跑根因与处理

PR `#40` 的 Head `86d48ad261669daaf14666556372a12f9b908726` 在 PR `#42` 合并使 `main` 前进后，没有自动收到新的 `pull_request` Run。根因是基线分支推进本身不触发该既有 PR 的新工作流事件，不是上游缓存、`source-lock`、测试、Runner 或 CI 合同回退。

项目所有者明确授权后，仅关闭并立即重新打开 PR `#40`；Head、文件和提交均未改变，没有 force push、rebase 或分支删除。由重开事件生成的 Run `30147559602` 七 Job 全绿；随后才完成 Squash Merge，并由 `main` Run `30147841226` 再次验证。

## 三、范围与禁止面

本 Issue 只允许十条 Markdown 路径，范围哈希为 `sha256:39a26c9301f8ca792498be73117320b1d26c3028fdc1c558703ee7cefb96061e`。它不修改历史计划快照、`upstream/`、`source-lock.json`、Rust、Cargo、测试、Workflow、Ruleset、Release、UI 或 AGOS。

所有本地时间来自项目所有者 Windows `Get-Date`；GitHub 事件时间只按服务端原值记录。提交不用 `GIT_AUTHOR_DATE` 或 `GIT_COMMITTER_DATE` 覆写默认本机时间。

## 四、验证合同

- 十路径以 Windows `Sort-Object` 排序、UTF-8/LF/末尾 LF 重新计算后，必须仍为获批 `scope_hash`。
- `verify-session-plan.ps1`、`Verify-RepositoryPolicy.ps1`、`Verify-ReleaseAuditGate.ps1`、状态字段静态检查、`git diff --check` 和 `git diff --cached --check` 必须通过。
- 最终 PR Head 仍须完成两个只读 reviewer 的变更面审查、全部 Review 对话解决、GitHub-hosted CI 全绿和项目所有者单独 Squash Merge 授权。

## 五、本地轻量验证证据

项目所有者 Windows 本机时间 `2026-07-25 15:35:55 +08:00` 已完成以下验证：

- 十路径的“未暂存差异 + 已暂存差异 + 未跟踪文件”并集为精确 `10` 条；按 UTF-8、LF 和末尾 LF 重算仍为 `sha256:39a26c9301f8ca792498be73117320b1d26c3028fdc1c558703ee7cefb96061e`。
- `verify-session-plan.ps1` 输出 `SESSION_PLAN_VERIFY_OK`，任务为 `issue-43`、工作等级为 `standard`。
- `Verify-ReleaseAuditGate.ps1` 输出 `ok=true`、`status=stale-re-audit-required`、`requires_reaudit=true`，没有错误或 blocked path。
- `Verify-RepositoryPolicy.ps1` 输出 `ok=true`、`violation_count=0`。
- 长期状态字段审计已覆盖 `AGENTS.md`、README、Master Plan、两份来源报告和 `err.md`；`git diff --check` 通过。
- 首轮范围审计曾只使用 `git diff --name-only`，未列出四个未跟踪的新文档；已复现并将本任务的审计口径修正为三类路径并集，未修改任何项目文件来掩盖该问题。

## 六、后续合法工作

下一项可启动工作是 Issue `#32` 的独立性能基线发现：重新冻结测量对象、参考来源与许可证、可比环境、范围哈希和所有者批准。Issue `#38` 的目录重新审计、性能预算、性能优化、产品迁移与 Gate 5 继续保持彼此独立的 Issue/PR 边界。
