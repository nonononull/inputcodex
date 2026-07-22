# Issue #35 Runtime Workflow：Release 审计基线解耦

workflow_status: ci-repair-verified-awaiting-github-hosted-ci
task_id: 2026-07-22-issue-35-release-catalog-decoupling
tracking_issue_ref: https://github.com/nonononull/inputcodex/issues/35
branch_ref: codex/issue-35-release-catalog-decoupling
baseline_ref: 939f3454b34e0faa42897be7489b344f2bec1d4c
approved_decision_ref: user-message:approve-issue-35-14-path-scope-2026-07-22
scope_hash: sha256:446444f8cef61de3923d8fe40823ee6b1719a424d9f9e013ee26e70d2f20686a
session_plan_ref: docs/plans/sessions/2026-07-22-issue-35-release-catalog-decoupling.md
implementation_plan_ref: docs/plans/2026-07-22-issue-35-release-catalog-decoupling.md
report_ref: docs/reports/issue-35-release-catalog-decoupling.md
allowed_operations: project-doc-write, rust-validation-write, test-write, ci-script-write, workflow-write, source-lock-metadata-write, ordinary-commit, ordinary-push, issue-comment, pull-request-create, review-ci-evidence-read
mutation_intent: 实现 Release 审计状态和 PR 路径门禁，保持产品运行面与上游快照字节不变。
executor_enforcement: 只允许十四路径；每次执行批次后检查工作树；提交、推送和 PR 前必须有 fresh 本地验证；最终合并仍需要单独的所有者授权。
agos_status: bypassed-needs-input-unregistered

## 执行节点

1. **批准与基线**：写入所有者范围批准，移除待决标签；确认 Issue `#35` 为 `OPEN`、远端 `main` 为 `939f3454b34e0faa42897be7489b344f2bec1d4c`、上游最新正式 Release 为 `v1.2.42` / `657cd33e009ad02515d30db6492cd4e669b06318`、工作树干净且没有既有 PR。
2. **RED**：添加临时仓库夹具，验证 `current`、合法 stale、相同 Release stale、缺少根因和错误 Issue URL；添加 PowerShell 合同，验证 current 放行、stale 放行复审资料、阻断五类产品路径及阻断 audit 与产品路径同 PR。
3. **GREEN**：引入 `release_audit`、`requires_reaudit()`、目录 Release 对齐、`Verify-ReleaseAuditGate.ps1` 和独立 CI Job；将 `required` 依赖该 Job。
4. **定向验证**：依次运行 `Test-CiScripts.ps1`、`cargo test -p inputcodex-parity --test catalog_repository --offline release_audit_显式解耦快照与功能目录审计基线`、仓库政策、格式和差异检查。
5. **PR 交付**：普通提交、普通推送、创建非 Draft PR，PR body 关联 `Closes #35`；等待 GitHub-hosted 全量 CI，解决每条 Review 对话并回写根因、处理和验证证据。
6. **合并边界**：仅在最终 Head、CI、Review 与范围均 fresh 后，由项目所有者明确授权 Squash Merge。未经该授权，不合并、不自动合并、不删除功能分支。
7. **CI 失败修复**：PR `#36` 首轮失败时先下载失败 Artifact 和读取 Clippy 日志，写 RED 合同复现 legacy base，再以最小修复、定向 Clippy、普通提交和普通推送更新同一 PR；禁止 force push、禁止忽略失败或修改 Ruleset。

## 失败恢复

- `release_audit` 结构无效：保持 PR 失败，先修复状态和定向测试；不得借删除字段或降低验证强度绕过。
- stale 阻断产品路径：拆分为上游同步/重新审计 PR 与后续性能或产品 PR；不得在同一 PR 混合提交。
- 上游正式 Release 或 `main` 漂移：停止提交，重新核验设计假设、Issue 状态与范围哈希。
- AGOS 再次返回不可用：将原因补充到本任务证据后直接走项目原生流程，不修改 AGOS。
- Review 发现错误语义或无效功能：建立一致性例外 Issue，等待项目所有者决定，不以“功能一致”名义直接照搬。
