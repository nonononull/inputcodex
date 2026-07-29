# Issue #95 Runtime Workflow：诊断日志只读结构观察

## Workflow Metadata

- `workflow_id`: `inputcodex.issue-95.diagnostic-log-observation.v1`
- `tracking_issue_ref`: `https://github.com/nonononull/inputcodex/issues/95`
- `approved_decision_ref`: `https://github.com/nonononull/inputcodex/issues/94#issuecomment-5109582183`
- `owner_scope_approval_ref`: `https://github.com/nonononull/inputcodex/issues/95#issuecomment-5113647548`
- `baseline_ref`: `origin/main@9587549c3f1bb334507075499f806485d83fce6a`
- `branch_ref`: `codex/issue-95-gate-5-diagnostic-log-observation`
- `worktree_ref`: `.worktrees/issue-95-gate-5-diagnostic-log-observation`
- `planning_scope_hash`: `sha256:14d78bf1a92f5b8db58650b501fb0cebee59a329823ff49e7b8ff3e93e0b7231`
- `candidate_scope_hash`: `sha256:8d407c269436c655e12ff94035183de6aa50dc7759fbc75f9cb7b6f9b0349d38`
- `planning_checkpoint_ref`: `9ae2447619bd26e6818968b9c3f1cf8df3e61cc6`
- `domain_checkpoint_ref`: `b6b05b4e0e2ac21086f24927d2106fd63ee7b048`
- `application_checkpoint_ref`: `0b2093044f6f9598f7ff56fbbc73c5c5a0469162`
- `platform_checkpoint_ref`: `d647fadd264084d6376a6b3f64c8819e7f698552`
- `parity_checkpoint_ref`: `4ea54fdf4b81cb8eee76d36f9917047098769613`
- `current_node`: `issue-95-local-verified`
- `terminal_node`: `issue-95-completed`

## 状态机

```text
issue-94-option-1-approved
  → issue-94-completed
  → issue-95-created
  → issue-95-discovery-completed
  → issue-95-planning-controls-ready
  → issue-95-planning-verified
  → issue-95-planning-checkpoint-pushed
  → issue-95-scope-approval-pending
  → issue-95-scope-approved
  → issue-95-domain-green
  → issue-95-application-green
  → issue-95-platform-green
  → issue-95-parity-green
  → issue-95-local-verified
  → issue-95-pr-open
  → issue-95-review-ci-green
  → issue-95-final-merge-authorization-pending
  → issue-95-squash-merged
  → issue-95-post-merge-verified
  → issue-95-completed
```

失败返回最近已验证 checkpoint；禁止跳过精确范围批准、TDD、Review 根因闭环、Hosted CI 或
独立 Squash Merge 授权。

## Approval Gates

### Gate A：Discovery 与规划

- 状态：`PLANNING_VERIFIED`
- 证据：Issue `#94` 方案 1 决策与 Issue `#95` 当前批准边界
- 允许：隔离分支/worktree、四份规划控制面、范围哈希、规划验证、planning checkpoint、普通 push、Issue 回写
- 禁止：Rust/Parity/稳定控制面实现、PR 与合并
- AGOS：ReportOnly 返回既有 `needs-input`/未登记状态，按 `err.md` 绕过，不阻塞项目原生流程

### Gate B：二十四路径实现范围

- 状态：`PASSED`
- 已批准：`24` 路径与
  `sha256:8d407c269436c655e12ff94035183de6aa50dc7759fbc75f9cb7b6f9b0349d38`
- 当前允许：产品 TDD、稳定文档、轻量验证、Git checkpoint、普通 push、非 Draft PR、Review/CI

### Gate C：远端交付

- 状态：`READY`
- 前置：Gate B 通过且本地验证稳定
- 要求：禁止 force push；PR 必须关联 Issue `#95`

### Gate D：最终合并

- 状态：`NOT_REACHED`
- 前置：绑定具体 PR 与 Final Head、所有 Review 对话根因闭环、Hosted CI 与 Artifact 合同通过
- 要求：项目所有者单独授权 Squash Merge

## Preflight

每次恢复先执行：

```powershell
Get-Date
git status --short --branch
git rev-parse HEAD
git rev-parse origin/main
gh issue view 95 --repo nonononull/inputcodex --json state,stateReason,labels,url
```

硬断言：

- 当前分支必须为 `codex/issue-95-gate-5-diagnostic-log-observation`。
- 基线必须源自 `9587549c3f1bb334507075499f806485d83fce6a`。
- Issue `#95` 必须 OPEN；主目录 `main` 不得被本任务写入。
- 规划批准前实际差异只能是 Planning Allowlist 四路径。
- Git 时间使用项目所有者 Windows 本机默认时间；禁止设置 `GIT_AUTHOR_DATE` 或 `GIT_COMMITTER_DATE`。

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
docs/plans/2026-07-28-issue-95-gate-5-diagnostic-log-observation.md
docs/plans/sessions/2026-07-28-issue-95-gate-5-diagnostic-log-observation.md
docs/reports/issue-95-gate-5-diagnostic-log-observation.md
docs/workflows/2026-07-28-issue-95-gate-5-diagnostic-log-observation-runtime.md
```

- `count`: `4`
- `hash`: `sha256:14d78bf1a92f5b8db58650b501fb0cebee59a329823ff49e7b8ff3e93e0b7231`

## Candidate Implementation Allowlist

```text
AGENTS.md
CONTEXT.md
README.md
build.md
crates/inputcodex-application/src/diagnostic_log_observation.rs
crates/inputcodex-application/src/lib.rs
crates/inputcodex-application/tests/diagnostic_log_observation.rs
crates/inputcodex-domain/src/diagnostic_log_observation.rs
crates/inputcodex-domain/src/lib.rs
crates/inputcodex-domain/tests/diagnostic_log_observation.rs
crates/inputcodex-parity/tests/catalog_repository.rs
crates/inputcodex-platform/src/diagnostic_log_observation.rs
crates/inputcodex-platform/src/lib.rs
crates/inputcodex-platform/tests/diagnostic_log_observation.rs
docs/plans/2026-07-28-issue-95-gate-5-diagnostic-log-observation.md
docs/plans/PROJECT-MASTER-PLAN.md
docs/plans/sessions/2026-07-28-issue-95-gate-5-diagnostic-log-observation.md
docs/reports/issue-95-gate-5-diagnostic-log-observation.md
docs/workflows/2026-07-28-issue-95-gate-5-diagnostic-log-observation-runtime.md
err.md
parity/README.md
parity/contracts/foundation-platform.yml
parity/features/foundation-platform.yml
parity/features/source-index.yml
```

- `count`: `24`
- `hash`: `sha256:8d407c269436c655e12ff94035183de6aa50dc7759fbc75f9cb7b6f9b0349d38`

## 分批执行

### Batch 0：Planning

- 四份规划文件必须一次形成完整静态合同。
- 执行 CI 脚本合同、仓库政策、allowlist、两个哈希、占位符和 `git diff --check`。
- 验证通过后建立 planning checkpoint 并普通 push。
- 回写 Issue 后必须停止，等待 Gate B 明确批准。

### Batch 1：Domain

- RED：新领域类型、六字段、不变量和隐私断言不存在。
- GREEN：实现 `DiagnosticLogObservation` 私有字段、构造、getter 与 `NoDiagnosticLog` 语义。
- 验证：Domain 全目标测试、Clippy、格式检查。
- checkpoint：`issue-95-domain-green @ b6b05b4e0e2ac21086f24927d2106fd63ee7b048`。

### Batch 2：Application

- RED：Request/Port/UseCase 和完成态映射不存在。
- GREEN：实现 `ObserveDiagnosticLog<P>`，固定 Some/None/Err 映射。
- 验证：Application 全目标测试、Clippy、LoadCoordinator 兼容测试。
- checkpoint：`issue-95-application-green @ 0b2093044f6f9598f7ff56fbbc73c5c5a0469162`。

### Batch 3：Platform

- RED：固定路径、有界尾读、记录分类和错误测试先失败。
- GREEN：实现 `SystemDiagnosticLogObservation` 与模块私有文件探针。
- 算法：`symlink_metadata` → 普通文件门禁 → 计算窗口 → Seek → 有界读取 → 首片段丢弃 → 逐行严格解析。
- 验证：Platform 全目标测试、Clippy、格式、隐私与禁止能力扫描。
- checkpoint：`issue-95-platform-green @ d647fadd264084d6376a6b3f64c8819e7f698552`。

### Batch 4：Parity

- RED：新 feature/contract/source mapping 和计数断言先失败。
- GREEN：只移动 `read_latest_logs`，原诊断总功能保留四入口且 `unassessed`。
- 预期：feature/contract `40/40`、source `133`、fixture manifest `11`。
- checkpoint：`issue-95-parity-green @ 4ea54fdf4b81cb8eee76d36f9917047098769613`。

### Batch 5：Project Closeout

- 更新稳定项目入口；README 只写用户稳定能力，不写动态 GitHub 流水账。
- `err.md` 仅记录新根因；重复问题引用既有条目。
- 执行全部本地轻量验证、范围、哈希、隐私和禁止面检查。
- checkpoint：`issue-95-local-verified`。

### Batch 6：PR / Review / CI

- 普通 push；禁止 force push。
- 创建非 Draft PR，关联 Issue `#95`。
- Review 每条对话都写根因、处理与验证证据后解决。
- 核验标准 CI、Performance observation 与 Artifact 合同。
- Final Head 全绿后只请求独立 Squash Merge 授权。

## 写入强制

每批前后执行：

1. `git status --short`；
2. 合并 `git diff --name-only` 与未跟踪文件形成实际路径集；
3. 使用 `StringComparer.Ordinal` 排序并验证 allowlist；
4. 复算候选或实际 scope hash；
5. `git diff --check`；
6. 需要新路径时立即停止并重新批准。

规划阶段实际差异必须精确等于四份规划文件。

## 禁止能力与隐私扫描

必须拒绝：

- 网络客户端、socket、下载、远程推荐或遥测；
- `Command::new`、线程、Watcher、轮询、后台任务；
- 写文件、清理日志、写诊断事件、复制诊断、剪贴板或设置序列化；
- Tauri、WebView、JavaScript、注入、广告或 UI；
- 任意路径公共入口、目录扫描、完整文件读取；
- 路径、用户名、机器名、正文、JSON 字段、事件、detail、PID、时间戳或凭据输出；
- 误迁移 `core-module:diagnostic_log`、clear/copy/write 三个命令；
- 把原 `feature.foundation-platform.diagnostics` 标记为 implemented。

## 本地验证合同

规划阶段：

```powershell
pwsh -NoProfile -File scripts/ci/Test-CiScripts.ps1
pwsh -NoProfile -File scripts/ci/Verify-RepositoryPolicy.ps1 -RepositoryRoot .
git diff --check
```

批准后的实现阶段最低执行：

```powershell
cargo test -p inputcodex-domain --all-targets --offline
cargo test -p inputcodex-application --all-targets --offline
cargo test -p inputcodex-platform --all-targets --offline
cargo test -p inputcodex-parity --all-targets --offline
cargo clippy -p inputcodex-domain --all-targets --offline -- -D warnings
cargo clippy -p inputcodex-application --all-targets --offline -- -D warnings
cargo clippy -p inputcodex-platform --all-targets --offline -- -D warnings
cargo clippy -p inputcodex-parity --all-targets --offline -- -D warnings
cargo fmt --all -- --check
```

Workspace 全量、Windows/macOS 编译和 Performance observation 交给 GitHub-hosted runners。

## 错误恢复

- 先查 `err.md`；重复 Patch、PowerShell、CRLF、GitHub 或 Cargo 问题引用既有条目。
- `apply_patch` 桌面包装器不可执行时，允许使用已安装 npm Codex 原生二进制的
  `--codex-run-as-apply-patch`，但仍必须走 `apply_patch` 协议。
- I/O 场景难以稳定复现时使用模块私有探针，禁止公开任意路径产品 API。
- 需要正文、搜索、导出、清理或写入时，回到新的功能或一致性例外 Issue。
- AGOS 不可用或不兼容时记录并绕过；禁止修改 AGOS 仓库或控制面。
