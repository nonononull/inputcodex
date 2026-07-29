# Issue #104 Runtime Workflow：本地会话目录只读观察

## Runtime Metadata

- `task_id`: `issue-104-gate-5-local-session-directory-observation`
- `tracking_issue_ref`: `https://github.com/nonononull/inputcodex/issues/104`
- `session_plan_ref`: `docs/plans/sessions/2026-07-29-issue-104-gate-5-local-session-directory-observation.md`
- `approved_decision_ref`: `https://github.com/nonononull/inputcodex/issues/103#issuecomment-5121451656`
- `implementation_scope_approval_ref`: `https://github.com/nonononull/inputcodex/issues/104#issuecomment-5121508761`
- `selected_business_path`: `gate-5/local-session-directory-observation`
- `baseline_ref`: `origin/main@4032b051f0f18be71d344eded2d6e79595233b65`
- `branch_ref`: `codex/issue-104-gate-5-local-session-directory-observation`
- `worktree_ref`: `.worktrees/issue-104-gate-5-local-session-directory-observation`
- `work_class`: `standard`
- `mutation_intent`: `source`
- `candidate_scope_hash`: `sha256:47dcb2c181daa61a8df073e7f3ada069bf8e3d9b95df0c57f7709bcb6cde211d`
- `executor`: `codex-desktop-main-thread`
- `subagents`: `not-authorized`
- `current_node`: `planning-checkpoint`

## Current Gate

允许：

- 冻结七路径 planning 控制面。
- 运行只读源码/文档/依赖查询、AGOS report-only 与本地基线测试。
- planning 验证通过后按 Domain → Application → Platform → Parity TDD 顺序写入。
- 每个 batch 建立 Git checkpoint。

禁止：

- 在 `main` 上写入、force push、删除 main。
- 超出二十九路径、修改其他 feature、AGOS、Ruleset、CI 或上游缓存。
- SQLite 写入、修复、迁移、删除、备份、恢复或任意路径输入。
- 返回路径、Provider、正文、Token、模型、账号或凭据。
- 未观察 RED 就写生产实现。
- 未绑定 Final Head 获得所有者授权就 Squash Merge。

## Node Order

### Node 0：Startup Baseline

1. 核验本地 main 与 `origin/main` 同为 `4032b051f0f18be71d344eded2d6e79595233b65`。
2. 创建 Issue #104、隔离分支和 worktree。
3. 运行 AGOS Git snapshot `-Checkpoint -RequireClean -ReportOnly`。
4. 运行四 crate 基线定向测试、仓库政策与 Release Audit。

状态：`completed`。

### Node 1：Decision Evidence

1. 核验 Issue #103 为 `CLOSED / COMPLETED`。
2. 核验项目所有者选择 A、批准 A1、授权自主细节并下达开工指令。
3. 将二十九路径与 hash 写入 Issue #104 评论。

状态：`completed`。

### Node 2：Planning Freeze

1. 写入 Plan、Session Plan、Runtime Workflow。
2. 回写 AGENTS、CONTEXT、Master Plan 和 build 入口。
3. 验证 planning 七路径与 `sha256:8b9295414fde23c7ea9c9c53a47acfb7a70b1f1bcd3255f81cb011ba5b624a51`。
4. 运行 AGOS default-entry `-ReportOnly` 和项目原生 planning 验证。
5. 建立 `issue-104-planning-freeze` Git checkpoint。

状态：`ready-for-checkpoint`。planning 七路径/hash、CI 合同、仓库政策、Release Audit、Cargo metadata、Markdown 链接和 diff check 已通过；AGOS default-entry 因任务未注册和缺少 owner scope manifest 返回 blocked，已按项目规则绕过。

### Node 3：Domain TDD

1. 写领域测试并运行 RED。
2. 实现标题安全值、条目、分页和来源覆盖。
3. 验证长度、空白、控制字符、截断、不变量和脱敏 Debug。
4. 运行 domain tests/Clippy/fmt。
5. 建立 `issue-104-domain` checkpoint。

状态：`pending`。

### Node 4：Application TDD

1. 写 Request、取消、Port 和 UseCase 测试并运行 RED。
2. 实现默认/显式分页校验、取消标记和完成态映射。
3. 运行 application tests/Clippy/fmt。
4. 建立 `issue-104-application` checkpoint。

状态：`pending`。

### Node 5：Platform TDD

1. 写现场合成 SQLite 测试并运行 RED。
2. 锁定 `rusqlite = 0.40.1`，仅启用 `bundled` 与 `hooks`。
3. 实现固定根、候选门禁、只读连接、schema 查询、多库排序/去重/分页。
4. 实现 busy timeout、deadline、progress cancellation 和错误脱敏。
5. 证明 fixture、WAL/SHM 和目录内容没有业务写入。
6. 运行 platform tests/Clippy/fmt。
7. 建立 `issue-104-platform` checkpoint。

状态：`pending`。

### Node 6：Parity TDD

1. 扩展 catalog 测试并运行 RED。
2. 新增 feature/contract/fixture，重映射 `list_local_sessions`。
3. 保持原管理总功能 `unassessed`，删除/备份/恢复入口不迁移。
4. 运行 parity tests。
5. 建立 `issue-104-parity` checkpoint。

状态：`pending`。

### Node 7：Local Closeout

1. 更新 README 和 Issue #104 报告。
2. 新根因去重后决定是否更新 `err.md`。
3. 运行 `build.md` Issue #104 全门禁、安全检查和依赖审查。
4. 验证二十九路径；未改 err.md 时验证二十八路径 hash。
5. 运行知识图谱刷新；无有效 CodeGraph 索引时只记录跳过，不初始化。
6. 建立 `issue-104-local-verified` checkpoint。

状态：`pending`。

### Node 8：Remote Delivery

1. 普通 push 当前分支。
2. 创建关联 Issue #104 的非 Draft PR。
3. 获取并处理 Review；每条对话记录根因、处理与验证证据。
4. 核验 CI、Performance Baseline、Artifact 和 Final Head。
5. 请求项目所有者对 Final Head 的独立 Squash Merge 授权。

状态：`pending`。

## Scope Enforcement

候选范围固定为二十九路径：

```text
AGENTS.md
build.md
Cargo.lock
Cargo.toml
CONTEXT.md
crates/inputcodex-application/src/lib.rs
crates/inputcodex-application/src/local_session_directory_observation.rs
crates/inputcodex-application/tests/local_session_directory_observation.rs
crates/inputcodex-domain/src/lib.rs
crates/inputcodex-domain/src/local_session_directory_observation.rs
crates/inputcodex-domain/tests/local_session_directory_observation.rs
crates/inputcodex-parity/tests/catalog_repository.rs
crates/inputcodex-platform/Cargo.toml
crates/inputcodex-platform/src/lib.rs
crates/inputcodex-platform/src/local_session_directory_observation.rs
crates/inputcodex-platform/tests/local_session_directory_observation.rs
docs/plans/2026-07-29-issue-104-gate-5-local-session-directory-observation.md
docs/plans/PROJECT-MASTER-PLAN.md
docs/plans/sessions/2026-07-29-issue-104-gate-5-local-session-directory-observation.md
docs/reports/issue-104-gate-5-local-session-directory-observation.md
docs/workflows/2026-07-29-issue-104-gate-5-local-session-directory-observation-runtime.md
err.md
parity/README.md
parity/contracts/session-data.yml
parity/features/session-data.yml
parity/features/source-index.yml
parity/fixtures/feature.session-data.local-session-directory-observation/baseline.yml
parity/fixtures/feature.session-data.local-session-directory-observation/manifest.yml
README.md
```

```powershell
$paths = @(
  'AGENTS.md',
  'build.md',
  'Cargo.lock',
  'Cargo.toml',
  'CONTEXT.md',
  'crates/inputcodex-application/src/lib.rs',
  'crates/inputcodex-application/src/local_session_directory_observation.rs',
  'crates/inputcodex-application/tests/local_session_directory_observation.rs',
  'crates/inputcodex-domain/src/lib.rs',
  'crates/inputcodex-domain/src/local_session_directory_observation.rs',
  'crates/inputcodex-domain/tests/local_session_directory_observation.rs',
  'crates/inputcodex-parity/tests/catalog_repository.rs',
  'crates/inputcodex-platform/Cargo.toml',
  'crates/inputcodex-platform/src/lib.rs',
  'crates/inputcodex-platform/src/local_session_directory_observation.rs',
  'crates/inputcodex-platform/tests/local_session_directory_observation.rs',
  'docs/plans/2026-07-29-issue-104-gate-5-local-session-directory-observation.md',
  'docs/plans/PROJECT-MASTER-PLAN.md',
  'docs/plans/sessions/2026-07-29-issue-104-gate-5-local-session-directory-observation.md',
  'docs/reports/issue-104-gate-5-local-session-directory-observation.md',
  'docs/workflows/2026-07-29-issue-104-gate-5-local-session-directory-observation-runtime.md',
  'err.md',
  'parity/README.md',
  'parity/contracts/session-data.yml',
  'parity/features/session-data.yml',
  'parity/features/source-index.yml',
  'parity/fixtures/feature.session-data.local-session-directory-observation/baseline.yml',
  'parity/fixtures/feature.session-data.local-session-directory-observation/manifest.yml',
  'README.md'
) | Sort-Object
$payload = ($paths -join "`n") + "`n"
$hash = [Convert]::ToHexString(
  [Security.Cryptography.SHA256]::HashData(
    [Text.UTF8Encoding]::new($false).GetBytes($payload)
  )
).ToLowerInvariant()
if ($paths.Count -ne 29) { throw "Issue #104 路径数量漂移：$($paths.Count)" }
if ($hash -ne '47dcb2c181daa61a8df073e7f3ada069bf8e3d9b95df0c57f7709bcb6cde211d') {
  throw "Issue #104 scope_hash 漂移：sha256:$hash"
}
```

## Error Watchlist

- PowerShell 哈希固定使用本机 `Sort-Object`、UTF-8 无 BOM、LF 和末尾换行。
- Git 实际范围合并 `origin/main...HEAD`、工作区 diff 和未跟踪文件。
- 多行 GitHub 正文使用临时文件与 `--body-file`。
- Cargo 本地验证固定 `--locked --offline`；新增依赖解析必须单独、可解释并更新 Cargo.lock。
- SQLite 测试只使用现场合成路径；路径必须脱敏且清理失败不能污染后续测试。
- SQLite raw error 可能包含路径或 SQL，禁止直接进入 ApplicationError。
- rusqlite progress handler 需要 `hooks` feature；中断错误必须按取消标记/截止时间分类。
- Windows symlink 测试可能受权限影响；生产门禁仍需测试抽象覆盖，不得把平台权限失败误判为产品成功。
- CodeGraph 当前没有有效索引，不得运行 `codegraph init`。
- 所有原生命令后显式检查 `$LASTEXITCODE`。

## Verification Gates

### Planning Gate

- 实际修改仅为 planning 七路径。
- planning hash 精确为 `sha256:8b9295414fde23c7ea9c9c53a47acfb7a70b1f1bcd3255f81cb011ba5b624a51`。
- Issue #103/#104 决策和范围评论可访问。
- Release Audit `current`，基线测试、仓库政策、Cargo metadata 和 diff 检查通过。
- AGOS report-only 结果已记录；任何 `needs-input/unregistered` 按项目规则绕过。

### Implementation Gate

- 每个批次先保存预期 RED 输出，再写最小 GREEN。
- dependency 只允许 `rusqlite 0.40.1` 与 `bundled/hooks`。
- 生产代码不得包含任意 SQL、路径回显、日志、写模式或禁用 query_only 的分支。
- 兄弟观察能力定向回归全绿。

### Delivery Gate

- 实际路径为批准范围子集；若无 err.md 新根因，精确匹配二十八路径 hash。
- 安全清单、许可证、测试、Clippy、fmt、治理、Release Audit 与知识图谱状态完成。
- 所有 Review 对话闭环，Hosted CI/Performance/Artifact 通过。
- Squash Merge 仅在项目所有者对 Final Head 单独授权后执行。

## AGOS Boundary

- 只运行 `D:\Android_source\ai-growth-os` 的 report-only、git snapshot 和 rollout 记录入口。
- AGOS 输出只能补充证据，不替代 inputcodex 的 Issue、Plan、Session Plan、Runtime Workflow、build.md、err.md、测试和 GitHub CI。
- 本次 default-entry 实际返回 `blocked/unregistered` 与 `missing-owner-scope-manifest`；根据项目规则已记录并绕过。后续不得为修复该外部状态修改 AGOS。
- Issue #104 禁止修改 AGOS Registry、脚本、规则、Workflow 或 Vault。

## Rollout Draft

- `workflow_family`: `gate-5-read-only-observation`
- `reusable_path`: 严格平台根 → 受控候选发现 → 只读 SQLite → 有界查询/取消 → 最小领域投影 → Parity 拆分
- `skill_usage`: brainstorming、writing-plans、using-git-worktrees、TDD、karpathy-guidelines、domain-modeling、security-review、knowledge-graph-auto-update
- `failure_recovery`: err.md 去重、AGOS 绕过、scope hash 停止门、SQLite 错误脱敏、Review 根因闭环
- `record_after_closeout`: 合并后若形成可复用增量，调用 `record-workflow-rollout.ps1`；单次 rollout 不生成 workflow candidate
