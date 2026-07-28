# Issue #92 Runtime Workflow：设置只读观察

## Workflow Metadata

- `workflow_id`: `inputcodex.issue-92.settings-observation.v1`
- `tracking_issue_ref`: `https://github.com/nonononull/inputcodex/issues/92`
- `approved_decision_ref`: `https://github.com/nonononull/inputcodex/issues/91`
- `approved_design_ref`: `https://github.com/nonononull/inputcodex/issues/92#issuecomment-5107059878`
- `owner_planning_approval_ref`: `https://github.com/nonononull/inputcodex/issues/92#issuecomment-5107673827`
- `owner_scope_approval_ref`: `https://github.com/nonononull/inputcodex/issues/92#issuecomment-5108221117`
- `scope_request_ref`: `https://github.com/nonononull/inputcodex/issues/92#issuecomment-5107913027`
- `baseline_ref`: `origin/main@a5559f4a873a81d91ed09b571503523a78a45118`
- `branch_ref`: `codex/issue-92-gate-5-settings-observation`
- `worktree_ref`: `.worktrees/issue-92-gate-5-settings-observation`
- `planning_scope_hash`: `sha256:dccd142c0c926433ce01adda37524895db2f4369917a6455fcfc393941a10cc2`
- `candidate_scope_hash`: `sha256:ca252684075d32de7aaf2ca066f12822ce48a5b01d1b0fcf67df146ea792baf1`
- `current_node`: `issue-92-local-verified`
- `terminal_node`: `issue-92-completed`

## 状态机

```text
issue-91-decision-completed
  → issue-92-created
  → issue-92-discovery-completed
  → issue-92-design-approved
  → issue-92-isolated-worktree-ready
  → issue-92-planning-controls-ready
  → issue-92-planning-verified
  → issue-92-scope-approval-pending
  → issue-92-scope-approved
  → issue-92-domain-green
  → issue-92-application-green
  → issue-92-platform-green
  → issue-92-parity-green
  → issue-92-local-verified
  → issue-92-pr-open
  → issue-92-review-ci-green
  → issue-92-final-merge-authorization-pending
  → issue-92-squash-merged
  → issue-92-post-merge-verified
  → issue-92-completed
```

失败返回最近已验证 checkpoint；禁止跳过 RED、范围批准、Review 根因闭环、Hosted CI 或
独立 Squash Merge 授权。

## Approval Gates

### Gate A：设计与规划

- 状态：`PASSED`
- 证据：`owner_planning_approval_ref`
- 允许：分支/worktree、四份规划控制面、精确范围、候选哈希和规划验证。

### Gate B：精确范围

- 状态：`PASSED`
- 待批准：`27` 路径和
  `sha256:ca252684075d32de7aaf2ca066f12822ce48a5b01d1b0fcf67df146ea792baf1`
- 通过后才允许：产品/Cargo/Parity/稳定文档写入、TDD、本地验证和 Git checkpoint。

### Gate C：远端交付

- 状态：`READY`
- 实施、本地验证和稳定文档已经形成可推送稳定面；允许按批准范围普通提交、推送和创建非 Draft PR。
- 禁止 force push。

### Gate D：最终合并

- 状态：`NOT_REACHED`
- 必须绑定具体 PR 与 Final Head；Review/CI 全部闭环后等待项目所有者单独授权。

## Preflight

每次恢复先执行：

```powershell
Get-Date
git status --short --branch
git rev-parse HEAD
git rev-parse origin/main
gh issue view 92 --repo nonononull/inputcodex --json state,stateReason,labels,url
```

硬断言：当前分支必须正确；HEAD 必须源自批准基线；Issue `#92` 必须 OPEN；主目录不得被本
任务写入；当前阶段 diff 只能是四份规划文件。基线漂移时停止重新评估。

## Scope Hash Algorithm

```powershell
function Get-ScopeHash([string[]]$Paths) {
  $sorted = [string[]]$Paths.Clone()
  [Array]::Sort($sorted, [StringComparer]::Ordinal)
  $payload = ($sorted -join "`n") + "`n"
  $bytes = [Text.Encoding]::UTF8.GetBytes($payload)
  $hash = [Security.Cryptography.SHA256]::HashData($bytes)
  'sha256:' + [Convert]::ToHexString($hash).ToLowerInvariant()
}
```

禁止区域排序、反斜杠路径、CRLF 拼接或缺少末尾 LF。

## Planning Allowlist

```text
docs/plans/2026-07-28-issue-92-gate-5-settings-observation.md
docs/plans/sessions/2026-07-28-issue-92-gate-5-settings-observation.md
docs/reports/issue-92-gate-5-settings-observation.md
docs/workflows/2026-07-28-issue-92-gate-5-settings-observation-runtime.md
```

- `count`: `4`
- `hash`: `sha256:dccd142c0c926433ce01adda37524895db2f4369917a6455fcfc393941a10cc2`

## Candidate Implementation Allowlist

```text
AGENTS.md
CONTEXT.md
Cargo.lock
Cargo.toml
README.md
build.md
crates/inputcodex-application/src/lib.rs
crates/inputcodex-application/src/settings_observation.rs
crates/inputcodex-application/tests/settings_observation.rs
crates/inputcodex-domain/src/lib.rs
crates/inputcodex-domain/src/settings_observation.rs
crates/inputcodex-domain/tests/settings_observation.rs
crates/inputcodex-parity/tests/catalog_repository.rs
crates/inputcodex-platform/Cargo.toml
crates/inputcodex-platform/src/lib.rs
crates/inputcodex-platform/src/settings_observation.rs
crates/inputcodex-platform/tests/settings_observation.rs
docs/plans/2026-07-28-issue-92-gate-5-settings-observation.md
docs/plans/PROJECT-MASTER-PLAN.md
docs/plans/sessions/2026-07-28-issue-92-gate-5-settings-observation.md
docs/reports/issue-92-gate-5-settings-observation.md
docs/workflows/2026-07-28-issue-92-gate-5-settings-observation-runtime.md
err.md
parity/README.md
parity/contracts/foundation-platform.yml
parity/features/foundation-platform.yml
parity/features/source-index.yml
```

- `count`: `27`
- `hash`: `sha256:ca252684075d32de7aaf2ca066f12822ce48a5b01d1b0fcf67df146ea792baf1`

## Batch 0：规划冻结

### 输入

Issue `#91/#92`、批准基线、项目控制面、上游只读缓存、Issue `#86/#89` 既有切片模式。

### 操作

1. 落盘四份 planning allowlist 文件。
2. 复算 planning/candidate hash。
3. 检查功能 ID、错误码、依赖版本、计数和路径一致。
4. 检查占位符、受保护路径、空白和 Git diff。
5. 可选执行 AGOS `ReportOnly`；失败、不可用或 `needs-input` 时记录并绕过。
6. 回写 Issue 候选范围和证据，然后停在 Gate B。

### 输出

四份规划控制面、`27` 路径 allowlist、两个可复算哈希；产品代码、Cargo、Parity、提交、
推送和 PR 均为 `0`。

## Batch 1：Domain TDD

仅在 Gate B 通过后：

1. 写 `crates/inputcodex-domain/tests/settings_observation.rs` RED。
2. 最小新增 `SettingsDocumentObservation`，固定零/非零数量和隐私 Debug。
3. 从 `src/lib.rs` 导出类型。
4. 运行定向 tests/Clippy、范围检查，建立 `issue-92-domain-green` checkpoint。

## Batch 2：Application TDD

1. 写 `SettingsObservationRequest`、`SettingsObservationPort`、`ObserveSettings` RED。
2. 固定 Some→Ready、None→Empty、Err→Failed；零条目 Some 仍为 Ready。
3. 验证 completion 可交给 `LoadCoordinator`。
4. 最小实现并导出；运行 tests/Clippy，建立 checkpoint。

## Batch 3：Platform 与 Cargo TDD

1. 用模块私有 `SettingsFileProbe` 覆盖缺失、文件类型、超限、I/O、JSON 和错误根。
2. 集成测试固定公开 `SystemSettingsObservation` 实现 Port 和非目标平台错误。
3. Workspace 增加 `serde_json = "=1.0.149"`，Platform 接入 workspace 依赖。
4. 通过 Cargo 更新锁文件并证明无关包漂移为 `0`。
5. 实现 `SystemPlatformPaths` 入口、`symlink_metadata` 和
   `Read::take(256 * 1024 + 1)` 双重门禁。
6. 解析为 `Value::Object` 后只返回 `object.len()`。
7. 运行 tests/Clippy/fmt/offline，建立 `issue-92-platform-green` checkpoint。

## Batch 4：Parity TDD

1. 在 `catalog_repository.rs` 先写 feature/contract/source mapping RED。
2. 新增 settings-observation feature 和无 fixture 合同。
3. 只移动 `tauri-command:load_settings` 并改为 `filesystem-read`。
4. 原设置管理保留三入口、读写副作用和 `unassessed`。
5. 固定 feature/contract `39`、source `133`、fixture manifest `11`。
6. 扫描公开任意路径、敏感输出、网络、写入、线程、子进程、UI、注入和 `unsafe`。
7. 运行 tests/Clippy，建立 `issue-92-parity-green` checkpoint。

## Batch 5：项目控制面与本地验证

1. 只回写稳定产品能力、Gate 事实、构建命令和静态证据。
2. README 禁止写 Head、Run、Artifact 或合并流水账。
3. `err.md` 没有新根因则保持不变。
4. 候选 Report 更新实际路径、TDD、依赖和验证结果。
5. 执行四 crate tests/Clippy、fmt、范围、计数、隐私和禁止面检查。
6. 执行 Git snapshot governance；AGOS 不可用按项目规则绕过。
7. 建立 `issue-92-local-verified` checkpoint。

## Batch 6：PR、Review 与 Hosted CI

1. 只允许普通提交和普通 push。
2. 创建非 Draft PR 并关联 Issue `#91/#92`。
3. 核对实际路径为批准 allowlist 子集。
4. Review 对话记录根因、处理和验证后解决。
5. Final Head 变化后重跑适用门禁。
6. 标准 CI 与 Performance Baseline 成功 Run 的 Artifact 必须为 `0`。
7. 全绿后停在独立 Squash Merge 授权门；未授权不得合并或删分支。

## Planning Verification

```powershell
$expected = @(
  'docs/plans/2026-07-28-issue-92-gate-5-settings-observation.md',
  'docs/plans/sessions/2026-07-28-issue-92-gate-5-settings-observation.md',
  'docs/reports/issue-92-gate-5-settings-observation.md',
  'docs/workflows/2026-07-28-issue-92-gate-5-settings-observation-runtime.md'
)
$actual = @(
  git diff --name-only
  git ls-files --others --exclude-standard
) | Where-Object { $_ } | Sort-Object -Unique
$expectedSorted = [string[]]$expected.Clone()
[Array]::Sort($expectedSorted, [StringComparer]::Ordinal)
$actualSorted = [string[]]$actual.Clone()
[Array]::Sort($actualSorted, [StringComparer]::Ordinal)
if (($actualSorted -join "`n") -ne ($expectedSorted -join "`n")) {
  throw '规划差异不等于四路径 allowlist。'
}
git diff --check
```

## Implementation Verification

Gate B 通过后，以 `build.md` 最终写入命令为准，最低包含：

```powershell
cargo test -p inputcodex-domain --offline
cargo test -p inputcodex-application --offline
cargo test -p inputcodex-platform --all-targets --offline
cargo test -p inputcodex-parity --offline
cargo clippy -p inputcodex-domain --all-targets --offline -- -D warnings
cargo clippy -p inputcodex-application --all-targets --offline -- -D warnings
cargo clippy -p inputcodex-platform --all-targets --offline -- -D warnings
cargo clippy -p inputcodex-parity --all-targets --offline -- -D warnings
cargo fmt --all -- --check
```

本地禁止默认执行 Workspace 全量冷编译；双平台全量和发布构建交给 GitHub-hosted runners。

## Error Recovery

- 先查 `err.md`，重复根因直接引用。
- Cargo 失败先证明缓存、索引或锁文件根因，禁止手写锁项。
- 需要额外 crate、额外文件或公开任意路径 API 时立即停止并重新批准。
- Review/CI 失败必须确定根因并以测试或可复核证据关闭。
- AGOS 不可用不阻塞项目，禁止跨仓修复 AGOS。

## Current Runtime State

```yaml
status: local-verified-pr-pending
baseline_ref: origin/main@a5559f4a873a81d91ed09b571503523a78a45118
branch_ref: codex/issue-92-gate-5-settings-observation
planning_scope_count: 4
planning_scope_hash: sha256:dccd142c0c926433ce01adda37524895db2f4369917a6455fcfc393941a10cc2
candidate_scope_count: 27
candidate_scope_hash: sha256:ca252684075d32de7aaf2ca066f12822ce48a5b01d1b0fcf67df146ea792baf1
planning_validation: passed
actual_scope_count: 26
err_md_changed: false
domain_checkpoint: c752d54506e9307528324c3ce5c2ccecfe23f9c7
application_checkpoint: 10cb5fd0f9f0093b7b9f07e5537e15bb8a84b822
platform_checkpoint: f77d7edd8a00ec6d40a62808fc50a75d1ec70df4
parity_checkpoint: 7e55bd038d7b0e5e7dbbb12a781df510c920aed6
local_verification: passed
feature_contract_counts: 39/39
source_entry_count: 133
fixture_manifest_count: 11
dependency_delta: serde_json-1.0.149,zmij-1.0.23
forbidden_capability_matches: 0
privacy_matches: 0
local_knowledge_lookup: project-native-doc-and-code-query-completed
agos_report_only: bypassed-known-unregistered-contract-err-md-2026-07-21
scope_request_ref: https://github.com/nonononull/inputcodex/issues/92#issuecomment-5107913027
owner_scope_approval_ref: https://github.com/nonononull/inputcodex/issues/92#issuecomment-5108221117
product_changes_authorized: true
git_checkpoint_authorized: true
commit_authorized: true
push_authorized: true
pr_authorized: true
review_ci: not-reached
squash_merge_authorized: false
```

## Stop Conditions

- Gate B 未通过、基线/范围漂移、diff 越界或需要第 `28` 条路径。
- 需要公开任意路径、网络、写入、线程、子进程、UI、注入或 `unsafe`。
- 无法保护路径、键、值、原始 JSON 和凭据，或无法区分缺失/空对象/损坏/错误根。
- Release 审计不为 current；Review/CI 根因未闭环；未获独立 Squash Merge 授权。
