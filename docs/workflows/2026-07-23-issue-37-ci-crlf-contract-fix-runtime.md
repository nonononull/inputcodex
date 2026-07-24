# Issue #37：CI 合同 CRLF 兼容性修复 Runtime Workflow

```yaml
workflow_id: inputcodex.issue-37.ci-crlf-contract-fix.v1
tracking_issue_ref: https://github.com/nonononull/inputcodex/issues/37
baseline_ref: d7438a0f2c43b7fbd2b159b3759aacea4ef1999e
branch: codex/issue-37-ci-crlf-contract-fix
session_plan_ref: docs/plans/sessions/2026-07-23-issue-37-ci-crlf-contract-fix.md
owner_execution_intent_ref: user-message:complete-and-squash-merge-issue-37-then-enter-issue-34-cache-sync-2026-07-23
scope_approval_status: approved
scope_approval_ref: https://github.com/nonononull/inputcodex/issues/37#issuecomment-5062599616
scope_hash: sha256:dea8803625e26050281443ed0d0a5021c58c272d1e8887b55578f588b3def2a3
```

## 1. `startup-baseline`

1. 用 GitHub API 与本地 Git 同时确认权威 `main` 为 `d7438a0f2c43b7fbd2b159b3759aacea4ef1999e`。
2. 确认当前分支和隔离工作树，读取 `AGENTS.md`、`README.md`、`build.md`、`err.md` 与 Issue `#37`。
3. 用 Windows `Get-Date` 记录本地审计时间；禁止设置 `GIT_AUTHOR_DATE` 或 `GIT_COMMITTER_DATE`。
4. 若 `local_knowledge_lookup` 未暴露或 AGOS 返回 `needs-input/unregistered`，记录后按项目规则绕过；禁止修改 AGOS，也不得把外部状态说成项目阻塞。

## 2. `owner-scope-stop`

1. 路径必须严格等于计划中的六条。
2. `scope_hash` 必须为 `sha256:dea8803625e26050281443ed0d0a5021c58c272d1e8887b55578f588b3def2a3`。
3. 项目所有者已通过批准引用确认六路径；若路径或 `scope_hash` 漂移，立即恢复为停止状态并重新批准。

## 3. `red-reproduction`

1. 读取真实 Workflow 文本并转换为确定性的 CRLF 文本，不修改 `.github/workflows/ci.yml`。
2. 对 CRLF 文本执行旧正则 `(?m)^  release-audit:$`，必须稳定返回 false。
3. 同时验证文本真实包含 `release-audit`、门脚本与 `required.needs`，证明失败只来自 EOL 假设。
4. 正常 Windows 工作树中的旧 `Test-CiScripts.ps1` 必须复现同一失败。

## 4. `minimal-green`

1. 将 Job 行断言改为 `(?m)^  release-audit:\r?$`。
2. 在同一 Contract Test 内构造 CRLF Workflow，并验证 Job、门脚本与 `required.needs` 三项合同。
3. 不改变 Workflow、不删除断言、不减少 Contract Test 数量、不规范化整个文件。

## 5. `local-verification`

依次执行：

```powershell
pwsh -NoProfile -File scripts/ci/Test-CiScripts.ps1
pwsh -NoProfile -File scripts/ci/Verify-ReleaseAuditGate.ps1 -RepositoryRoot .
pwsh -NoProfile -File scripts/ci/Verify-RepositoryPolicy.ps1 -RepositoryRoot .
git diff --check
```

要求：

- `CI_CONTRACT_GREEN passed=32`；
- Release Audit Gate 输出 `ok=true`、`status=current`、`requires_reaudit=false`；
- 仓库政策退出码为 `0`；
- 变更路径严格等于六路径，UTF-8、控制字符和空白检查通过。

## 6. `commit-push-pr`

1. 使用普通 Git 提交；不覆写本机时间。
2. 优先正常推送；若 Git smart-HTTP 复现 `Recv failure: Connection was reset`，引用 `err.md` 的既有根因，使用 GitHub Git Database API 保持 blob/tree/parent 一致并以 `force:false` 创建或快进远端功能分支。
3. 创建非 Draft PR，正文必须包含 `Closes #37`、根因、RED/GREEN、范围哈希和验证命令。

## 7. `review-ci-merge`

1. 等待最终 PR Head 的全部标准 GitHub-hosted Checks。
2. 获取 Review 对话；未解决项必须写明根因、处理方式和验证证据。
3. 核验 PR 非 Draft、可合并、Head 未漂移、全部 Checks 成功且所有者授权仍适用于最终 Head。
4. 只执行 Squash Merge；禁止 Merge Commit、Rebase Merge、force push 或删除 `main`。
5. 合并后核验 merge commit、tree、Issue 关闭状态和 `main` CI，再进入 Issue `#34`。

## 8. `failure-recovery`

| 失败条件 | 恢复动作 |
| --- | --- |
| RED 不再复现 | 停止并重新审计根因，不先改正则 |
| 需要修改 Workflow | 停止并申请新范围，不扩大本 Issue |
| Git smart-HTTP 失败 | 使用 `err.md` 已验证的 API 回退；禁止 force push |
| CI 或 Review 失败 | 定位根因、最小修复、重新验证；不得带病合并 |
| scope 漂移 | 重新计算哈希并取得项目所有者批准 |
| Issue #34 来源事实漂移 | 停在 discovery，不写缓存 |
