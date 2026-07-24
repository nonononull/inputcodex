# Issue #37：CI 合同 CRLF 兼容性修复 Session Plan

```yaml
task_id: issue-37
tracking_issue_ref: https://github.com/nonononull/inputcodex/issues/37
baseline_ref: d7438a0f2c43b7fbd2b159b3759aacea4ef1999e
branch: codex/issue-37-ci-crlf-contract-fix
implementation_plan_ref: docs/plans/2026-07-23-issue-37-ci-crlf-contract-fix.md
runtime_workflow_ref: docs/workflows/2026-07-23-issue-37-ci-crlf-contract-fix-runtime.md
owner_execution_intent_ref: user-message:complete-and-squash-merge-issue-37-then-enter-issue-34-cache-sync-2026-07-23
scope_approval_status: approved
scope_approval_ref: https://github.com/nonononull/inputcodex/issues/37#issuecomment-5062599616
pr_ref: https://github.com/nonononull/inputcodex/pull/39
scope_hash: sha256:dea8803625e26050281443ed0d0a5021c58c272d1e8887b55578f588b3def2a3
mutation_intent: ci-contract-bugfix-with-regression-evidence
executor_enforcement: exact-path-set-and-no-force-push
```

## 当前事实

- GitHub 权威 `main` 与本地 `main` 均为 `d7438a0f2c43b7fbd2b159b3759aacea4ef1999e`。
- Issue `#37` 已复现 Windows CRLF 假失败；临时 LF 检出通过 `32` 个合同。
- `.github/workflows/ci.yml` 的 `release-audit` Job 和 `required.needs` 真实存在，不属于写入范围。
- `err.md` 已记录 Git smart-HTTP 连接重置与 API 安全回退；本轮不得重复发明或优化外部传输机制。
- 当前会话没有暴露 `local_knowledge_lookup`；AGOS 只读入口返回 `needs-input/unregistered`。两者均已记录并按项目规则绕过，不得阻塞 Issue `#37`。

## 允许操作

项目所有者已确认六路径 `scope_hash`，允许：

1. 修改精确六路径；
2. 执行本地轻量 RED/GREEN 与政策验证；
3. 普通提交、正常推送或按 `err.md` 的既有 GitHub API 回退建立远端分支；
4. 创建关联 Issue `#37` 的非 Draft PR；
5. 处理 Review、等待标准 GitHub-hosted CI；
6. 在全部门禁满足后按当前所有者授权执行 Squash Merge，并关闭 Issue `#37`。

## 执行批次

### 批次 1：范围与基线门

- 回写 Issue `#37` 的所有者执行授权、六路径和 `scope_hash`。
- 复核分支基线、Git 状态、Windows EOL 与现有失败特征。
- 精确范围批准已回写；执行前仍需确认路径与 `scope_hash` 未漂移。

### 批次 2：TDD RED

- 用明确的 CRLF Workflow 文本执行旧 `release-audit` Job 正则。
- 要求失败仅指向 `\r` 行尾，而不是 Job 或 `required.needs` 缺失。
- 记录命令、退出码和失败文本到报告。

### 批次 3：最小 GREEN

- 只修改 `Test-CiScripts.ps1` 的相关 Contract Test。
- 使用 `\r?` 兼容 LF/CRLF，不对整个 Workflow 做隐式全局规范化。
- 同一 Contract Test 增加确定性 CRLF 文本的三项合同断言，保持 Contract Test 总数 `32`。

### 批次 4：证据与本地验证

- 将错误根因、处理和证据补入 `err.md` 与报告。
- 运行 CI 合同、Release Audit Gate、仓库政策与空白检查。
- 校验变更路径严格等于六路径，重新计算 `scope_hash`。

### 批次 5：PR、Review、CI 与合并

- 普通提交并推送；禁止 force push。
- 创建关联 `Closes #37` 的非 Draft PR。
- 所有 Review 对话必须解决；CI 最终 Head 全绿后，核验 Head、Review、Checks、mergeability 和所有者授权。
- 只允许 Squash Merge；合并后验证 `main` 与合并提交，并关闭 Issue `#37`。

### 批次 6：切换 Issue #34

- 从合并后的权威 `main` 建立新的 Issue `#34` 纯缓存同步工作树。
- Fresh 复核 Release、tag、commit、tree、tarball、manifest、许可证与二十路径。
- 不把本 Issue 的计划、脚本或报告混入缓存同步 PR。

## 停止条件

- 六路径或 `scope_hash` 未获项目所有者明确批准；
- RED 不能稳定复现、失败根因发生变化，或修复需要修改 Workflow；
- 任何本地验证、CI、Review、mergeability 或路径门禁失败；
- 需要 force push、删除 `main`、扩大到 Rust/Cargo/上游缓存/AGOS；
- Issue `#34` 的 Fresh 来源事实与既有发现不一致。
