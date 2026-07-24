# Issue #37：CI 合同 CRLF 兼容性修复报告

## 元数据

- Issue：`https://github.com/nonononull/inputcodex/issues/37`
- 基线：`d7438a0f2c43b7fbd2b159b3759aacea4ef1999e`
- 分支：`codex/issue-37-ci-crlf-contract-fix`
- PR：`https://github.com/nonononull/inputcodex/pull/39`
- 范围批准：`https://github.com/nonononull/inputcodex/issues/37#issuecomment-5062599616`
- `scope_hash`：`sha256:dea8803625e26050281443ed0d0a5021c58c272d1e8887b55578f588b3def2a3`
- 本地完整验证时间：`2026-07-24 12:28:45 +08:00`，来自 Windows `Get-Date`
- 当前阶段：PR `#39` 已创建，本地六路径门禁已通过，等待最终 Head 的 GitHub-hosted CI 与 Review

## 根因

`scripts/ci/Test-CiScripts.ps1` 使用 `(?m)^  release-audit:$` 检查 Workflow Job。Windows 系统级 `core.autocrlf=true` 把 `.github/workflows/ci.yml` 签出为 CRLF 后，Job 行在冒号与换行之间带有 `\r`，导致 `$` 断言失败。

Workflow 实际包含 `release-audit` Job、`Verify-ReleaseAuditGate.ps1` 和 `required.needs -> release-audit`。因此失败属于测试的 EOL 错误假设，不是 CI 拓扑缺失。

## RED 证据

在现有 Contract Test 内将同一 Workflow 确定性构造为 LF 与 CRLF 两个变体，并保持旧正则：

```text
CI_CONTRACT_TEST_FAILURE
FAIL Release 审计门接入 PR 与 required 汇总 :: CRLF Workflow 必须存在独立 release-audit Job
RED_EXIT=1
```

同一 CRLF 文本中的门脚本与 `required.needs` 断言均成立，失败精确落在 Job 行的 `\r`。

## 最小修复

- Job 行断言从 `(?m)^  release-audit:$` 改为 `(?m)^  release-audit:\r?$`。
- 同一 Contract Test 始终测试 LF 与 CRLF 两个 Workflow 变体。
- 两个变体都验证 Job、门脚本与 `required.needs` 三项合同。
- 不修改 `.github/workflows/ci.yml`，不全局规范化 Workflow，不减少或跳过既有合同。

## 本地 GREEN 证据

```text
pwsh -NoProfile -File scripts/ci/Test-CiScripts.ps1
CI_CONTRACT_GREEN passed=32

pwsh -NoProfile -File scripts/ci/Verify-ReleaseAuditGate.ps1 -RepositoryRoot .
ok=true status=current requires_reaudit=false

pwsh -NoProfile -File scripts/ci/Verify-RepositoryPolicy.ps1 -RepositoryRoot .
ok=true violation_count=0

git diff --cached --check
exit=0
```

Windows 工作树使用系统级 `core.autocrlf=true`。空白门禁必须使用正常 Git 过滤器与精确暂存后的 `git diff --cached --check`；若强制 `core.autocrlf=false` 读取 CRLF 工作树，会把既有 `\r` 误报为整文件尾随空白。

## 精确范围

```text
docs/plans/2026-07-23-issue-37-ci-crlf-contract-fix.md
docs/plans/sessions/2026-07-23-issue-37-ci-crlf-contract-fix.md
docs/reports/issue-37-ci-crlf-contract-fix.md
docs/workflows/2026-07-23-issue-37-ci-crlf-contract-fix-runtime.md
err.md
scripts/ci/Test-CiScripts.ps1
```

当前没有修改 Workflow、Rust、Cargo、上游缓存、`source-lock`、功能目录、性能基线、Ruleset、Release 或 AGOS。

## 未完成门禁

- 最终 PR Head 的 GitHub-hosted CI；
- Review 对话、mergeability 和最终 Head 所有者授权复核；
- Squash Merge 与合并后 `main` 验证。
