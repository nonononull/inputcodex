# Issue #81 Session Plan：版本与启动意图

## 会话控制

```yaml
task_id: issue-81-gate-5-version-startup
tracking_issue_ref: https://github.com/nonononull/inputcodex/issues/81
approved_decision_ref: https://github.com/nonononull/inputcodex/issues/80#issuecomment-5095090077
approved_scope_ref: https://github.com/nonononull/inputcodex/issues/81#issuecomment-5100612634
branch: codex/issue-81-gate-5-version-startup
baseline_main: ef69494d92c7c461b0cb858e95f6838404ae1a61
candidate_scope_count: 23
candidate_scope_hash: sha256:c1ef2c00a445dd2bd60dc5f5b375cb27d1e467a3d457d7eb53b7ec82a304aafe
allowed_operations: plan,tdd,local-light-verification,git-checkpoint,commit,push,non-draft-pr,review-ci
mutation_intent: add-version-startup-slice-and-sync-native-control-plane
executor_enforcement: exact-path-scope-and-forbidden-capability-hard-stop
final_merge_authorization: pending-separate-gate
```

## 当前事实

- `origin/main` 与分支基线均为
  `ef69494d92c7c461b0cb858e95f6838404ae1a61`。
- Issue #78 / PR #79 已完成应用概览只读事实迁移。
- Issue #80 已批准 inputcodex 原生启动意图方案 A。
- 上游最新正式功能真源仍为
  `v1.2.43@5036ff056b5c629f19356396b17d6eeb70da664c`。
- `release_audit=current`，允许 Gate 5 产品迁移。
- AGOS 不参与本仓写入；仅使用项目原生控制面。

## 成功标准

1. 领域模型不保存原始参数或环境值；
2. Application 只产生 Ready/Failed，不伪造 Empty；
3. 非法显式环境值稳定映射为 `InvalidInput`；
4. Platform 仅进行进程读取，旧变量不进入生产代码；
5. 功能目录与合同状态为 `implemented`；
6. 23 路径范围、隐私和禁止能力扫描通过；
7. 本地轻量验证与 Hosted Review/CI 全绿；
8. PR 停在独立 Squash Merge 授权门。

## 执行批次

### Batch 0：规划控制面

- [x] 创建 Issue #81 与授权证据；
- [x] 创建隔离分支；
- [x] 创建设计、Session Plan 与 Runtime Workflow；
- [ ] 验证 planning scope 的 3 路径与哈希；
- [ ] 创建规划 Git checkpoint。

验证：

```powershell
$planning = [string[]]@(
  'docs/plans/2026-07-28-issue-81-gate-5-version-startup.md',
  'docs/plans/sessions/2026-07-28-issue-81-gate-5-version-startup.md',
  'docs/workflows/2026-07-28-issue-81-gate-5-version-startup-runtime.md'
)
[Array]::Sort($planning, [StringComparer]::Ordinal)
$text = [string]::Join("`n", $planning) + "`n"
$hash = [Convert]::ToHexString(
  [Security.Cryptography.SHA256]::HashData(
    [Text.UTF8Encoding]::new($false).GetBytes($text)
  )
).ToLowerInvariant()
if ($planning.Count -ne 3) { throw 'Issue #81 planning scope count 漂移。' }
if ($hash -ne '707b8a43199ffb69b71f18a9681e9432b02c94b8533dd7dcbc4cf2b1ad758579') {
  throw "Issue #81 planning scope hash 漂移：sha256:$hash"
}
```

### Batch 1：Domain TDD

- [ ] RED：新增测试引用尚不存在的 `StartupIntent` 和快照；
- [ ] GREEN：新增最小领域类型与 getter；
- [ ] 验证复用 `ApplicationVersion`，不修改既有应用概览模块；
- [ ] 创建 Domain Git checkpoint。

定向命令：

```powershell
cargo test --locked --offline --ignore-rust-version -p inputcodex-domain --test version_startup
```

### Batch 2：Application TDD

- [ ] RED：新增 Port/用例、Ready/Failed、InvalidInput 与过期结果测试；
- [ ] GREEN：实现最小同步用例和共享错误扩展；
- [ ] 验证请求 Debug 不含私密输入；
- [ ] 创建 Application Git checkpoint。

定向命令：

```powershell
cargo test --locked --offline --ignore-rust-version -p inputcodex-application --test version_startup
```

### Batch 3：Platform TDD

- [ ] RED：默认、参数、环境、非法值、非 Unicode、优先级与 unsupported；
- [ ] GREEN：实现纯解析函数与 `SystemVersionStartup`；
- [ ] 验证生产代码不含旧变量，不读取文件或网络；
- [ ] 创建 Platform Git checkpoint。

定向命令：

```powershell
cargo test --locked --offline --ignore-rust-version -p inputcodex-platform --test version_startup
```

### Batch 4：Parity TDD

- [ ] RED：目录状态和合同仍不满足 Issue #80；
- [ ] GREEN：更新 feature、contract、README 与仓库级断言；
- [ ] 验证 source-index 不发生变更；
- [ ] 创建 Parity Git checkpoint。

定向命令：

```powershell
cargo test --locked --offline --ignore-rust-version -p inputcodex-parity --test catalog_repository
```

### Batch 5：控制面收口

- [ ] 更新 AGENTS、CONTEXT、README、build 和 Master Plan；
- [ ] 创建实施报告；
- [ ] 如发生新根因，更新 err.md；重复问题只引用既有记录；
- [ ] 验证 23 路径和 candidate scope hash；
- [ ] 创建控制面 Git checkpoint。

### Batch 6：本地轻量验证

```powershell
cargo test --locked --offline --ignore-rust-version `
  -p inputcodex-domain `
  -p inputcodex-application `
  -p inputcodex-platform `
  -p inputcodex-parity

cargo clippy --locked --offline --ignore-rust-version `
  -p inputcodex-domain `
  -p inputcodex-application `
  -p inputcodex-platform `
  -p inputcodex-parity `
  --all-targets -- -D warnings

cargo fmt --all -- --check
pwsh -NoProfile -File scripts/ci/Test-CiScripts.ps1
pwsh -NoProfile -File scripts/ci/Verify-ReleaseAuditGate.ps1 -RepositoryRoot .
pwsh -NoProfile -File scripts/ci/Verify-RepositoryPolicy.ps1 -RepositoryRoot .
```

继续执行范围、隐私、旧变量、网络、写入、线程、shell、`unsafe` 和
`git diff --check` 扫描。验证失败时必须先查 `err.md` 并完成根因闭环。

### Batch 7：远端交付

- [ ] 最终 Git checkpoint；
- [ ] 使用本机默认时间提交；
- [ ] 普通推送，不 force push；
- [ ] 创建关联 Issue #80/#81 的非 Draft PR；
- [ ] 核验 Review 对话、标准 CI、Performance observation 与 Artifact；
- [ ] 停在独立 Squash Merge 授权门。

## 范围强制

候选范围与哈希以设计文件为真源。任意新增、删除或改名路径都必须：

1. 停止写入；
2. 说明根因；
3. 重新计算规范化 SHA-256；
4. 更新控制面；
5. 取得项目所有者新批准。

## 停止条件

- 上游最新正式 Release 不再是 `v1.2.43`；
- `release_audit` 不为 `current`；
- 需要新依赖、UI、网络、写入、线程或 OS 专属模块；
- 生产代码需要旧变量兼容；
- 任何测试、Review 或 CI 根因未闭环；
- 需要最终 Squash Merge 但尚未获得单独授权。
