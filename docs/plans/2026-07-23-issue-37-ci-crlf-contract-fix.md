# Issue #37：CI 合同 CRLF 兼容性修复计划

## 任务元数据

- Issue：`https://github.com/nonononull/inputcodex/issues/37`
- 基线：`d7438a0f2c43b7fbd2b159b3759aacea4ef1999e`
- 分支：`codex/issue-37-ci-crlf-contract-fix`
- 本机计划时间：`2026-07-23 13:40:40 +08:00`，来自 Windows `Get-Date`
- 所有者执行意图：`user-message:complete-and-squash-merge-issue-37-then-enter-issue-34-cache-sync-2026-07-23`
- 精确范围批准：`https://github.com/nonononull/inputcodex/issues/37#issuecomment-5062599616`

## 根因

`scripts/ci/Test-CiScripts.ps1` 在读取 `.github/workflows/ci.yml` 后，以 `(?m)^  release-audit:$` 识别独立 Job。Windows 系统级 `core.autocrlf=true` 将工作树签出为 CRLF，行尾 `\r` 使该正则产生假失败；Workflow 的 `release-audit` Job 与 `required.needs` 拓扑本身没有缺失。

临时 LF 检出只证明错误位于测试的 EOL 假设，不是永久修复。Issue `#37` 必须在所有者正常 Windows 工作树与标准 GitHub-hosted CI 中通过。

## 方案比较

### 方案 A：只让 Job 行断言容忍 CRLF（采用）

- 将断言改为 `(?m)^  release-audit:\r?$`。
- 在同一 Contract Test 内构造确定性的 CRLF Workflow 文本，验证 `release-audit`、门脚本和 `required.needs` 三项合同。
- 优点：根因处最小修复，不改变其他 YAML 检查语义，不修改 Workflow。
- 风险：未来若新增其他严格依赖行尾的正则，仍需各自建立回归合同。

### 方案 B：读取后统一规范化全部 Workflow 换行

- 把 CRLF 全量转换为 LF，再执行现有全部正则。
- 优点：统一输入。
- 风险：扩大当前单点缺陷的行为面，可能掩盖其他需要独立诊断的原始文本问题。

### 方案 C：强制 Windows 工作树使用 LF（拒绝）

- 通过临时 Git 配置或签出策略绕过 CRLF。
- 拒绝原因：没有修复脚本的错误跨平台假设，且违反 Issue `#37` 的验收边界。

## 精确写入范围

按字典序固定为以下六条 POSIX 路径：

```text
docs/plans/2026-07-23-issue-37-ci-crlf-contract-fix.md
docs/plans/sessions/2026-07-23-issue-37-ci-crlf-contract-fix.md
docs/reports/issue-37-ci-crlf-contract-fix.md
docs/workflows/2026-07-23-issue-37-ci-crlf-contract-fix-runtime.md
err.md
scripts/ci/Test-CiScripts.ps1
```

哈希字节输入为上述路径以 LF 连接并保留一个末尾 LF：

```text
scope_hash: sha256:dea8803625e26050281443ed0d0a5021c58c272d1e8887b55578f588b3def2a3
```

任何新增、删除、重命名路径都使批准失效，必须重新计算并取得项目所有者批准。

## 明确禁止

- 不修改 `.github/workflows/ci.yml`、`Verify-ReleaseAuditGate.ps1`、Rust、Cargo、上游缓存、`source-lock`、功能目录、性能基线、Ruleset、Release 或 AGOS。
- 不降低 `release-audit` Job、门脚本或 `required.needs` 的检查强度。
- 不使用临时 LF 检出作为合并证据。
- 不 force push，不删除 `main`，不带未解决 Review 对话合并。

## 外部治理辅助状态

- 当前会话未暴露 `local_knowledge_lookup`，MCP resources 与 resource templates 均为空；本计划只记录不可用，不伪造查询结果。
- AGOS `invoke-agos-default-entry.ps1 -ReportOnly` 返回 `needs-input`、`unregistered`，并将继续执行限制在 intake；按照 `inputcodex` 项目规则立即绕过，不修改或优化 AGOS Registry、Workflow、Rules 或 Vault。
- 后续执行以本计划、Session Plan、Runtime Workflow、Issue、Git 与 CI 证据为准；AGOS 输出不构成本项目阻塞或执行授权。

## RED → GREEN

1. **RED：** 在未修改脚本的基线上将 Workflow 文本确定性转换为 CRLF，证明旧正则不能识别真实存在的 `release-audit` Job；同时正常 Windows 工作树的 `Test-CiScripts.ps1` 继续稳定报告同一失败。
2. **GREEN：** 将 Job 行断言改为 EOL 无关正则，并加入 CRLF 文本的 `release-audit`、`Verify-ReleaseAuditGate.ps1` 与 `required.needs` 三项回归断言。
3. **回归：** 正常运行 `pwsh -NoProfile -File scripts/ci/Test-CiScripts.ps1`，要求输出 `CI_CONTRACT_GREEN passed=32`；不得通过删除或跳过现有合同获得绿色。

## 验证命令

```powershell
pwsh -NoProfile -File scripts/ci/Test-CiScripts.ps1
pwsh -NoProfile -File scripts/ci/Verify-ReleaseAuditGate.ps1 -RepositoryRoot .
pwsh -NoProfile -File scripts/ci/Verify-RepositoryPolicy.ps1 -RepositoryRoot .
git diff --check
```

GitHub-hosted CI 必须在 PR 最终 Head 上全部通过；所有 Review 对话必须解决并记录根因、处理与验证证据。项目所有者已经授权在这些门禁满足后对 Issue `#37` 的 PR 执行 Squash Merge。

## 与 Issue #34 的顺序

Issue `#37` 合并且 `main` 重新 Fresh 后，才能以已批准的二十路径 `scope_hash` `sha256:fc2ff14a2011f54c3014daa63e3d7658d1a47bd68bd49ab3a84d9601c8d3d76c` 进入 Issue `#34` 的纯缓存同步。Issue `#34` 的最终 PR 合并不由本计划预授权。
