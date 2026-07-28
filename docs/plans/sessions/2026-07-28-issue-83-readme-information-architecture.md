# Issue #83 Session Plan：README 信息架构与文档入口

## 会话控制

```yaml
task_id: issue-83-readme-information-architecture
tracking_issue_ref: https://github.com/nonononull/inputcodex/issues/83
approved_decision_ref: https://github.com/nonononull/inputcodex/issues/83
approved_scope_ref: pending-owner-approval
branch: codex/issue-83-readme-information-architecture
worktree: .worktrees/issue-83-readme-information-architecture
baseline_main: da65f7d8402e4de27e2795ee8905be18ad565653
planning_scope_count: 3
planning_scope_hash: sha256:83c915a75626bbfb31d9520a519dba3a5a210adc8b47a535f46fc21412c3a95f
candidate_scope_count: 10
candidate_scope_hash: sha256:d8a404c19b108587a5e17b4ded454444d5e948c92410b759504a7eb7c63bed44
allowed_operations: plan,local-light-verification,git-checkpoint
mutation_intent: separate-stable-readme-from-dynamic-governance-history
executor_enforcement: exact-path-scope-and-no-product-surface-hard-stop
final_merge_authorization: pending-separate-gate
```

## 当前事实

- Issue #83 已由仓库所有者账号创建并保存方案 B 批准证据；
- 分支基线是 PR #82 合并后的 `main@da65f7d8402e4de27e2795ee8905be18ad565653`；
- 当前无开放 PR，第四个 Gate 5 产品切片尚未建立；
- README 的动态状态与文档清单占比为 `84.5%`；
- `docs/` 共有 `146` 份文档且没有 `docs/README.md`；
- 基线 CI 合同为 `35/35`，Repository Policy 为 `0` 违规；
- 默认 `apply_patch.bat` 命中既有 WindowsApps 权限根因，已复用 npm 官方
  `bin/codex.js --codex-run-as-apply-patch` 入口；
- `.worktrees/` 已写入本地 `.git/info/exclude`，未修改受版本控制文件；
- AGOS 不参与本仓写入，项目原生控制面足以执行本任务。

## 成功标准

1. README 成为稳定、诚实的公开项目首页；
2. README 约 `70–100` 行，不再承担动态 Issue/PR/CI 流水账；
3. 文档门户按主题导航，不逐文件枚举历史文档；
4. 文档职责进入 AGENTS，防止后续任务继续向 README 追加流水账；
5. Master Plan 与 build.md 的 PR #82 前状态被最小纠正；
6. 新 worktree ignore 查询根因进入 err.md；
7. 十路径、链接、政策、格式和纯文档边界验证通过；
8. PR 停在独立 Squash Merge 授权门。

## 执行批次

### Batch 0：规划控制面

- [x] 创建 Issue #83；
- [x] 从最新 `main` 创建隔离 worktree 与分支；
- [x] 运行 CI 合同和 Repository Policy 基线；
- [x] 计算 planning/candidate scope；
- [x] 创建设计、Session Plan 与 Runtime Workflow；
- [ ] 验证 planning scope 并创建 Git checkpoint；
- [ ] 将十路径和 candidate scope hash 回写 Issue #83；
- [ ] 获得项目所有者精确范围批准。

Planning scope 验证：

```powershell
$planning = [string[]]@(
  'docs/plans/2026-07-28-issue-83-readme-information-architecture.md',
  'docs/plans/sessions/2026-07-28-issue-83-readme-information-architecture.md',
  'docs/workflows/2026-07-28-issue-83-readme-information-architecture-runtime.md'
)
[Array]::Sort($planning, [StringComparer]::Ordinal)
$text = [string]::Join("`n", $planning) + "`n"
$hash = [Convert]::ToHexString(
  [Security.Cryptography.SHA256]::HashData(
    [Text.UTF8Encoding]::new($false).GetBytes($text)
  )
).ToLowerInvariant()
if ($planning.Count -ne 3) { throw 'Issue #83 planning scope count 漂移。' }
if ($hash -ne '83c915a75626bbfb31d9520a519dba3a5a210adc8b47a535f46fc21412c3a95f') {
  throw "Issue #83 planning scope hash 漂移：sha256:$hash"
}
```

### Batch 1：稳定 README

- [ ] 删除逐 Issue 历史、完整 SHA、Run 与文档流水账；
- [ ] 增加开发阶段警告和三个已迁移能力；
- [ ] 增加七成员 Workspace 架构表；
- [ ] 增加构建、上游、一致性、贡献和许可证入口；
- [ ] 验证不存在产品成熟度夸大或上游运行面混入。

### Batch 2：文档门户

- [ ] 新增 `docs/README.md`；
- [ ] 分类控制面、ADR、Gate 5、上游一致性、性能与历史目录；
- [ ] 精选长期入口，避免复制全部 `146` 份文档；
- [ ] 验证全部相对链接存在。

### Batch 3：职责与稳定状态

- [ ] 在 AGENTS 固定 README、docs portal、Master Plan、build、err 和 GitHub 证据职责；
- [ ] Master Plan 记录 PR #82 稳定完成，下一产品切片尚未选择；
- [ ] build.md 删除 Issue #81 待推送/待 PR 的顶部陈述；
- [ ] 不重排或重写 Master Plan/build.md 的历史主体。

### Batch 4：排错与报告

- [ ] err.md 新增 `git check-ignore` 对不存在目录本身需使用尾斜杠或子路径的根因；
- [ ] 既有 PowerShell Markdown 与 apply-patch 根因只引用，不重复新增；
- [ ] 创建 `docs/reports/issue-83-readme-information-architecture.md`；
- [ ] 回写实际行数、链接数、路径数和轻量验证结果。

### Batch 5：本地轻量验证

- [ ] candidate scope count/hash 精确匹配；
- [ ] README 动态内容与长度守卫通过；
- [ ] README 和 docs portal 相对链接全部存在；
- [ ] CI 合同 `35/35`；
- [ ] Repository Policy `0` 违规；
- [ ] `git diff --check` 通过；
- [ ] 实际差异不进入产品、CI、上游、性能或第四功能路径。

不运行本地 Cargo 全量编译或桌面构建。本任务是纯文档治理，Hosted CI 根据现有分类合同跳过
重型 Job，并由 required 汇总验证文档 PR 语义。

### Batch 6：远端交付

- [ ] 使用 Windows 本机默认时间提交；
- [ ] 普通推送，不 force push；
- [ ] 创建关联 Issue #83 的非 Draft PR；
- [ ] 核验 Review 对话、CI 分类、required 与 Artifact；
- [ ] 根因全部闭环后停在独立 Squash Merge 授权门；
- [ ] 不删除分支，不创建递归 Closeout。

## 范围强制

候选范围与哈希以设计文件为真源。精确范围批准前，只允许修改三份 planning 文档。
批准后如需新增、删除或改名路径，必须：

1. 停止写入；
2. 说明根因；
3. 重新计算规范化 SHA-256；
4. 更新三份控制面；
5. 在 Issue #83 回写新范围；
6. 取得项目所有者新批准。

## 停止条件

- README 需要展示完整动态状态或逐 Issue 历史；
- 需要新增脚本、Workflow、Ruleset 或 required check；
- 需要修改 Rust、UI、Cargo、Parity、upstream 或性能数据；
- 需要删除历史文档；
- 实际路径不在获批十路径内；
- Markdown 链接、政策、Review 或 CI 根因未闭环；
- 需要最终 Squash Merge 但尚未获得单独授权。
