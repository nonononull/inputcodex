# Issue #98 Runtime Workflow：Relay 认证与配置状态只读观察

## Runtime Metadata

- `task_id`: `issue-98-gate-5-relay-status-observation`
- `tracking_issue_ref`: `https://github.com/nonononull/inputcodex/issues/98`
- `approved_decision_ref`: `https://github.com/nonononull/inputcodex/issues/97#issuecomment-5115176838`
- `session_plan_ref`: `docs/plans/sessions/2026-07-29-issue-98-gate-5-relay-status-observation.md`
- `design_plan_ref`: `docs/plans/2026-07-29-issue-98-gate-5-relay-status-observation.md`
- `report_ref`: `docs/reports/issue-98-gate-5-relay-status-observation.md`
- `baseline_ref`: `origin/main@b7c4174671caba806162a42e82b7bc0b20f73bf5`
- `branch_ref`: `codex/issue-98-gate-5-relay-status-observation`
- `worktree_ref`: `.worktrees/issue-98-gate-5-relay-status-observation`
- `planning_scope_hash`: `sha256:ec6c88d4a96c351fee85d6c416b04c95b27050893ccbe55b4ad55edfd8d95051`
- `candidate_scope_hash`: `sha256:b1dda60cda57d4be9344b3fa0c74a49b6087b9bdf03fceb5a772ec7e893d63a5`
- `mutation_intent`: `source`
- `current_gate`: `PLANNING_VERIFIED_AWAITING_IMPLEMENTATION_APPROVAL`

## 不变量

1. 当前只写四份 planning 文件；二十七路径批准前不写产品、Cargo、Parity 或稳定项目文档。
2. 两份固定文件均缺失才返回 Empty；至少一份存在时必须返回带文档状态的 Ready。
3. 单文件最多读取 `256 KiB + 1` 个判界字节，总产品读取上限为 `512 KiB`。
4. 文档损坏、超限或不可读是可观察事实，不冒充缺失、未认证、未配置或 `false`。
5. 返回类型不包含字符串、路径、原文、字段、账号、Token、Provider ID 或 Base URL。
6. 禁止任意路径输入、写入、网络、子进程、线程/Watcher、UI、注入、遥测和 `unsafe`。
7. 原 Relay 总功能与非 `relay_status` 来源继续 `unassessed`。
8. Git 时间使用 Windows 本机 `Get-Date`；禁止覆写 author/committer date。
9. 只允许普通 push；永久禁止 force push 和删除 `main`。
10. 最终 Squash Merge 始终需要独立授权。

## Phase 0：恢复与基线

在 Issue worktree 执行：

```powershell
$ErrorActionPreference = 'Stop'
Get-Date -Format 'yyyy-MM-dd HH:mm:ss zzz'
git status --short --branch
git rev-parse HEAD
git rev-parse origin/main
git merge-base HEAD origin/main
```

成功条件：三个提交均为 `b7c4174671caba806162a42e82b7bc0b20f73bf5`，工作树在写入前干净。

## Phase 1：本地知识查询

读取：

```text
AGENTS.md
README.md
build.md
err.md
CONTEXT.md
docs/plans/PROJECT-MASTER-PLAN.md
upstream/CodexPlusPlus/crates/codex-plus-core/src/relay_config.rs
crates/inputcodex-platform/src/platform_paths.rs
crates/inputcodex-platform/src/settings_observation.rs
crates/inputcodex-platform/src/diagnostic_log_observation.rs
```

若根目录没有 `.codegraph/`，不得初始化。GBrain 无结果时记录事实并继续使用项目原生控制面，不把
外部知识缺口伪造成阻塞。

## Phase 2：Planning Freeze

### Planning Allowlist

```text
docs/plans/2026-07-29-issue-98-gate-5-relay-status-observation.md
docs/plans/sessions/2026-07-29-issue-98-gate-5-relay-status-observation.md
docs/reports/issue-98-gate-5-relay-status-observation.md
docs/workflows/2026-07-29-issue-98-gate-5-relay-status-observation-runtime.md
```

### Candidate Allowlist

```text
AGENTS.md
CONTEXT.md
Cargo.lock
Cargo.toml
README.md
build.md
crates/inputcodex-application/src/lib.rs
crates/inputcodex-application/src/relay_status_observation.rs
crates/inputcodex-application/tests/relay_status_observation.rs
crates/inputcodex-domain/src/lib.rs
crates/inputcodex-domain/src/relay_status_observation.rs
crates/inputcodex-domain/tests/relay_status_observation.rs
crates/inputcodex-parity/tests/catalog_repository.rs
crates/inputcodex-platform/Cargo.toml
crates/inputcodex-platform/src/lib.rs
crates/inputcodex-platform/src/relay_status_observation.rs
crates/inputcodex-platform/tests/relay_status_observation.rs
docs/plans/2026-07-29-issue-98-gate-5-relay-status-observation.md
docs/plans/PROJECT-MASTER-PLAN.md
docs/plans/sessions/2026-07-29-issue-98-gate-5-relay-status-observation.md
docs/reports/issue-98-gate-5-relay-status-observation.md
docs/workflows/2026-07-29-issue-98-gate-5-relay-status-observation-runtime.md
err.md
parity/README.md
parity/contracts/provider-network.yml
parity/features/provider-network.yml
parity/features/source-index.yml
```

### 范围收集

必须合并已跟踪差异和未跟踪文件：

```powershell
$changed = @(
  git diff --name-only --relative
  git diff --cached --name-only --relative
  git ls-files --others --exclude-standard
) | Where-Object { $_ } | Sort-Object -Unique
$changed
```

规划阶段 `$changed` 必须精确等于四路径。实现阶段 `$changed` 必须是二十七路径子集；任何新路径
立即停止并请求重新批准。

### Scope Hash 复算

```powershell
function Get-ScopeHash([string[]]$Paths) {
  $copy = [string[]]$Paths.Clone()
  [Array]::Sort($copy, [StringComparer]::Ordinal)
  $payload = [string]::Join("`n", $copy) + "`n"
  $bytes = [Text.Encoding]::UTF8.GetBytes($payload)
  $hash = [Security.Cryptography.SHA256]::HashData($bytes)
  'sha256:' + [Convert]::ToHexString($hash).ToLowerInvariant()
}
```

必须得到：

```text
planning  = sha256:ec6c88d4a96c351fee85d6c416b04c95b27050893ccbe55b4ad55edfd8d95051
candidate = sha256:b1dda60cda57d4be9344b3fa0c74a49b6087b9bdf03fceb5a772ec7e893d63a5
```

## Phase 3：AGOS ReportOnly

AGOS 只作为外部只读辅助：

```powershell
powershell -ExecutionPolicy Bypass -File `
  D:\Android_source\ai-growth-os\components\rules\scripts\invoke-agos-default-entry.ps1 `
  -Root D:\Android_source\ai-growth-os `
  -ProjectRoot (Get-Location).Path `
  -TaskId issue-98-gate-5-relay-status-observation `
  -SelectedBusinessPath gate-5/relay-status-observation `
  -SessionPlanRef docs/plans/sessions/2026-07-29-issue-98-gate-5-relay-status-observation.md `
  -ApprovedDecisionRef https://github.com/nonononull/inputcodex/issues/97#issuecomment-5115176838 `
  -ReportOnly
```

若结果为未登记、`needs-input`、接口不兼容或异常：

1. 把结果写入本任务 report 和 Issue `#98`；
2. 立即绕过并继续项目原生验证；
3. 禁止修改 AGOS 脚本、Registry、Workflow、Rules 或 Vault。

## Phase 4：Planning 验证与检查点

```powershell
pwsh -NoProfile -File scripts/ci/Test-CiScripts.ps1
if ($LASTEXITCODE -ne 0) { exit $LASTEXITCODE }

pwsh -NoProfile -File scripts/ci/Verify-RepositoryPolicy.ps1 -RepositoryRoot .
if ($LASTEXITCODE -ne 0) { exit $LASTEXITCODE }

cargo metadata --locked --offline --no-deps
if ($LASTEXITCODE -ne 0) { exit $LASTEXITCODE }

$placeholderPattern = @(('TO' + 'DO'), ('TB' + 'D'), ('PLACE' + 'HOLDER'), ('待' + '补'), ('稍后' + '补')) -join '|'
rg -n $placeholderPattern `
  docs/plans/2026-07-29-issue-98-gate-5-relay-status-observation.md `
  docs/plans/sessions/2026-07-29-issue-98-gate-5-relay-status-observation.md `
  docs/reports/issue-98-gate-5-relay-status-observation.md `
  docs/workflows/2026-07-29-issue-98-gate-5-relay-status-observation-runtime.md
if ($LASTEXITCODE -eq 0) { throw '发现未清理占位符' }
if ($LASTEXITCODE -gt 1) { exit $LASTEXITCODE }

git diff --check
if ($LASTEXITCODE -ne 0) { exit $LASTEXITCODE }
```

验证全绿后：

```powershell
Get-Date -Format 'yyyy-MM-dd HH:mm:ss zzz'
git status --short
git add -- `
  docs/plans/2026-07-29-issue-98-gate-5-relay-status-observation.md `
  docs/plans/sessions/2026-07-29-issue-98-gate-5-relay-status-observation.md `
  docs/reports/issue-98-gate-5-relay-status-observation.md `
  docs/workflows/2026-07-29-issue-98-gate-5-relay-status-observation-runtime.md
git diff --cached --check
git commit -m 'docs: 冻结 Issue #98 Relay 状态观察计划'
```

不设置 Git 日期，不 amend，不 push。将本地提交哈希回写 Issue `#98`，随后停止等待独立实施批准。

## Phase 5：批准后的 Domain TDD

前置：Issue `#98` 存在项目所有者对二十七路径和 candidate hash 的明确批准评论。

1. RED：新增 `crates/inputcodex-domain/tests/relay_status_observation.rs`，引用尚不存在的类型。
2. GREEN：新增领域模块和 `lib.rs` 导出。
3. 覆盖五种文档状态、三种凭据状态、四种配置状态、聚合 getter 和 Debug 最小披露。
4. 验证：

```powershell
cargo test -p inputcodex-domain --all-targets --offline
cargo clippy -p inputcodex-domain --all-targets --offline -- -D warnings
cargo fmt --all -- --check
```

建立 `issue-98-domain-green` checkpoint。

## Phase 6：批准后的 Application TDD

1. RED：新增零字段 Request、Port、UseCase 与完成态测试。
2. GREEN：实现 `ObserveRelayStatus<P>` 的 Some/None/Err 固定映射。
3. 保持旧请求类型无法调用本 UseCase。
4. 验证：

```powershell
cargo test -p inputcodex-application --all-targets --offline
cargo clippy -p inputcodex-application --all-targets --offline -- -D warnings
cargo fmt --all -- --check
```

建立 `issue-98-application-green` checkpoint。

## Phase 7：批准后的 Platform TDD

### RED 矩阵

- 两文件均缺失、仅 auth 缺失、仅 config 缺失。
- 合法 JSON object、JSON 损坏、非 object 根、非法 UTF-8。
- 合法 TOML、TOML 损坏、空 Provider、缺 Provider table、字段类型错误。
- ChatGPT token 三选一存在、空字符串、错误认证模式。
- `OPENAI_API_KEY` 存在、空字符串、错误类型。
- Bearer Token 完成路径、API Key 完成路径、认证文档不可观察路径。
- 元数据超限、读取增长越界、符号链接、非普通文件、元数据/读取失败。
- 结果 Debug 与错误码不包含路径、字段或内容。

### GREEN 算法

1. `SystemPlatformPaths.resolve` 取得 `codex_home()`。
2. 派生两个固定文件名，不接受调用方路径。
3. 每份执行普通文件门禁和双重 `256 KiB` 上限。
4. JSON 使用 `serde_json::Value`；TOML 使用 `toml_edit::DocumentMut`。
5. 解析结果立即压缩为内部布尔结构，原始字节与节点不跨出函数。
6. 聚合为五个领域枚举；两份均 Missing 时返回 `None`。

### 验证

```powershell
cargo test -p inputcodex-platform --all-targets --offline
cargo clippy -p inputcodex-platform --all-targets --offline -- -D warnings
cargo fmt --all -- --check
```

建立 `issue-98-platform-green` checkpoint。

## Phase 8：批准后的 Parity TDD

1. RED：新增 feature/contract/source mapping 断言并证明当前不存在。
2. GREEN：新增 observation 子能力，只移动 `tauri-command:relay_status`。
3. 断言原 Relay 总功能仍 `unassessed`，其余来源未移动。
4. 验证：

```powershell
cargo test -p inputcodex-parity --all-targets --offline
cargo clippy -p inputcodex-parity --all-targets --offline -- -D warnings
cargo fmt --all -- --check
pwsh -NoProfile -File scripts/parity/Invoke-ReleaseAudit.ps1 -RepositoryRoot .
```

建立 `issue-98-parity-green` checkpoint。

## Phase 9：批准后的本地收口

执行四 crate tests/Clippy、格式、CI 合同、仓库政策、Release Audit、实际范围、scope hash、隐私与
禁止能力扫描。完整 Workspace 与双平台构建不占用本机，交给 GitHub-hosted CI。

隐私扫描至少覆盖新增 Rust、Parity 与任务报告，禁止出现真实凭据样本、账号、URL、绝对路径和
原始配置正文。测试数据只允许明显虚构且不可用的最小结构值。

## Phase 10：远端交付

仅在实现批准和本地收口通过后：

1. 普通 push 当前分支；禁止 force push。
2. 创建关联 Issue `#98` 的非 Draft PR。
3. Review 对话逐条记录根因、处理与验证证据后解决。
4. 核验标准 CI、Performance observation 和 Artifact 数量/保留合同。
5. 绑定 Final Head 请求独立 Squash Merge 授权。

## 错误恢复

- 遇到错误先查 `err.md`，重复根因只引用既有条目。
- `apply_patch` 包装器拒绝执行时，使用 `err.md` 记录的 npm Codex 原生
  `--codex-run-as-apply-patch` 字面量补丁入口，不改写项目文件生成流程。
- 离线 `cargo tree` 因未缓存无关依赖失败时，用 `Cargo.lock` 与本地 registry 源码核验版本/API，
  不联网、不把缓存边界误判为代码错误。
- 任何测试暴露新语义分支时先回到 Issue；禁止在实现中临时发明领域状态。
- AGOS 异常只记录并绕过，绝不在本 Issue 修复 AGOS。
