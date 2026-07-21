# Issue #2 / PR #3 架构治理 closeout 报告

## 结论

2026 年 7 月 21 日，Issue `#2` 对应的架构与发布治理文档已通过 PR `#3` Squash Merge 到 `main`。Issue 已自动关闭，旧功能分支已删除，合并后 `main-protection` Ruleset 未发生漂移。

本报告只证明 Issue `#2` 的交付闭环，不宣称 Gate 1 全部完成。Issue/PR 模板与标签仍需后续独立 Issue/PR；Gate 2 仍处于锁定状态。

## 交付引用

- `review_ref`：
  - `https://github.com/nonononull/inputcodex/pull/3#issuecomment-5033315525`
  - `https://github.com/nonononull/inputcodex/pull/3#issuecomment-5033444325`
  - `https://github.com/nonononull/inputcodex/pull/3#issuecomment-5034419368`
  - `https://github.com/nonononull/inputcodex/pull/3#issuecomment-5034505181`
- `pr_ref`：`https://github.com/nonononull/inputcodex/pull/3`。
- `ci_ref`：`not-configured`；合并时 `statusCheckRollup` 为空，Checks 数量为 `0`。
- `merge_ref`：`https://github.com/nonononull/inputcodex/commit/0e11375997ff10fdc0c233b31c8468af2d9a4f44`。
- 本次 closeout 跟踪：`https://github.com/nonononull/inputcodex/issues/4`。

## GitHub 最终状态

- PR `#3`：`MERGED`、非 Draft。
- 目标分支：`main`。
- PR Head：`6b090ba5aa479c714c9e231aa07787724d6a8190`。
- 合并提交：`0e11375997ff10fdc0c233b31c8468af2d9a4f44`。
- 合并时间：`2026-07-21T13:15:51Z`，即北京时间 `2026-07-21 21:15:51`。
- 合并人：`nonononull`。
- Issue `#2`：`CLOSED`，关闭时间 `2026-07-21T13:15:52Z`。
- Review 对话：总数 `0`，未解决数 `0`。
- Checks：`0`；当前 Gate 1 未创建 GitHub Actions，也未配置 required status checks。
- 远端分支 `docs/issue-2-architecture-governance`：不存在。
- 本地分支 `docs/issue-2-architecture-governance`：不存在。

## Squash Merge 证明

- `0e11375997ff10fdc0c233b31c8468af2d9a4f44` 只有一个父提交：`09564740b8d00a4b09630c024607cc5292d0381f`。
- 合并提交 tree：`0730422eb3fa738fe2d05a51e5191832fbfec0fe`。
- PR Head `6b090ba5aa479c714c9e231aa07787724d6a8190` 的 tree：`0730422eb3fa738fe2d05a51e5191832fbfec0fe`。
- 两个 tree 完全一致，证明 Squash 结果完整包含 PR 最终内容，没有额外文件差异。
- 本地 `main`、`origin/main` 和本 closeout 分支基线均指向该合并提交。

## Review 与 CI 语义

- 单人维护阶段平台 required approvals 为 `0`，但 PR 评论保存了项目所有者对架构、Review 根因闭环、Ruleset 和 CI 策略的明确决策证据。
- PR 合并时不存在 Review 对话，因此未解决数量为 `0`；这不放宽未来“先确定根因、完成处理、回写验证证据再 Resolve”的硬约束。
- `ci_ref: not-configured` 是事实边界，不是“CI 通过”。Gate 1 明确禁止在 Issue `#2` 中提前创建 Actions 或 required checks。

## Ruleset 合并后复核

- Ruleset：`main-protection`，ID `19395456`。
- `enforcement`：`active`。
- 范围：仅 `refs/heads/main`，无 exclude。
- `bypass_actors`：空；当前用户也不能 bypass。
- 有效规则：`deletion`、`non_fast_forward`、`pull_request`。
- required approvals：`0`。
- required Review 对话解决：`true`。
- allowed merge methods：仅 `squash`。
- 仓库级 Merge/Rebase 开关保持原状；Gate 1 只通过目标 Ruleset 约束 `main`，没有扩大到其他分支。

## 保持不变的边界

- 仓库仍无应用源码、Cargo Workspace、Rust/Iced 文件或 `.github/workflows/`。
- 未导入 `BigPizzaV3/CodexPlusPlus` 或 `zsr131550/CodexPlusPlus` 源码。
- 未创建 required status checks，未修改 Ruleset，未发布版本或安装包。
- 上游最新正式 Release 仍为 `v1.2.41`，标签仍解析到 `3dafffcafb2566a1e8bce4b35671656d6adb3eda`。

## AGOS warning-mode 边界

- `invoke-agos-default-entry.ps1 -ReportOnly` 对 Issue `#4` 返回 `AGOS_DEFAULT_ENTRY_STATUS=needs-input`，原因是全局 registry 未登记 inputcodex task/business path；项目 Git foundation、入口文档和本地知识查询均为 `ready`。
- `record-workflow-rollout.ps1` dry-run 返回 `WORKFLOW_ROLLOUT_REPAIR_REQUIRED`，建议任务 ID 为 `agos-p2-2026-07-21-issue-2-architecture-governance`；未写 task intake draft，也未修改 AI Growth OS 仓库。
- 本项目只记录上述真实缺口，不宣称 strict runtime 或正式 rollout 已通过；如需接入，必须在 AI Growth OS 仓库另建 Issue/PR。

## 可重复验证

```powershell
gh issue view 2 --repo nonononull/inputcodex --json number,state,closedAt,url
gh pr view 3 --repo nonononull/inputcodex --json state,isDraft,mergedAt,mergedBy,mergeCommit,headRefOid,statusCheckRollup,url
gh api repos/nonononull/inputcodex/rulesets/19395456
gh api repos/nonononull/inputcodex/rules/branches/main
git rev-list --parents -n 1 0e11375997ff10fdc0c233b31c8468af2d9a4f44
git show -s --format=%T 0e11375997ff10fdc0c233b31c8468af2d9a4f44
git show -s --format=%T 6b090ba5aa479c714c9e231aa07787724d6a8190
git for-each-ref --format='%(refname):%(objectname)' refs/heads refs/remotes/origin
```

Review 对话通过 GitHub GraphQL `reviewThreads(first:100)` 核验；远端旧分支通过 branches API 核验并预期返回不存在。

## 后续

1. 通过 Issue `#4` 的独立 PR 合并本 closeout 证据；该 PR 在项目所有者再次授权前保持开放。
2. 另建 Issue/PR 补齐 Issue 模板、PR 模板和标签，完成 Gate 1 剩余治理工作。
3. Gate 1 完成后才能建立 Gate 2 的 `upstream-sync` Issue，导入 `v1.2.41` 完整快照并创建每 6 小时上游监控。
