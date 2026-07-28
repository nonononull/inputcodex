# Issue #89 Runtime Workflow：Relay 环境只读观察

## 运行元数据

- `tracking_issue_ref`: `https://github.com/nonononull/inputcodex/issues/89`
- `approved_decision_ref`: `https://github.com/nonononull/inputcodex/issues/88`
- `baseline_ref`: `origin/main@db0c09b9df272887deb9407a5e344cf87a59dda8`
- `branch_ref`: `codex/issue-89-gate-5-relay-environment-observation`
- `planning_scope_hash`: `sha256:0a301df75edda05c8d3d1c01c91221dd9ac8ff11aeeca39fb4c26a293b0543b0`
- `candidate_scope_hash`: `sha256:0adc20d0ed4d73ae645a5ffb23d7208f7aaabfea92c4d6fd62e0da3a120e8f77`
- `mutation_intent`: `approved-tdd-implementation`
- `executor_enforcement`: `ordinal-allowlist + git-diff audit + forbidden-capability scan`
- `current_node`: `domain-red-green`

## 节点图

```text
decision-88-completed
  → issue-89-created
  → isolated-worktree-created
  → discovery-completed
  → design-written
  → session-plan-written
  → runtime-workflow-written
  → planning-verification
  → owner-scope-approval
  → domain-red-green
  → application-red-green
  → shared-platform-red-green
  → windows-macos-red-green
  → parity-red-green
  → control-plane-closeout
  → local-final-verification
  → push-and-pr
  → review-and-hosted-ci
  → independent-squash-authorization
```

## 当前规划写入范围

```text
docs/plans/2026-07-28-issue-89-gate-5-relay-environment-observation.md
docs/plans/sessions/2026-07-28-issue-89-gate-5-relay-environment-observation.md
docs/workflows/2026-07-28-issue-89-gate-5-relay-environment-observation-runtime.md
```

共 `3` 路径，哈希：
`sha256:0a301df75edda05c8d3d1c01c91221dd9ac8ff11aeeca39fb4c26a293b0543b0`。

## 候选完整实施范围

```text
AGENTS.md
CONTEXT.md
Cargo.lock
Cargo.toml
README.md
build.md
crates/inputcodex-application/src/lib.rs
crates/inputcodex-application/src/relay_environment_observation.rs
crates/inputcodex-application/tests/relay_environment_observation.rs
crates/inputcodex-domain/src/lib.rs
crates/inputcodex-domain/src/relay_environment_observation.rs
crates/inputcodex-domain/tests/relay_environment_observation.rs
crates/inputcodex-parity/tests/catalog_repository.rs
crates/inputcodex-platform/Cargo.toml
crates/inputcodex-platform/src/lib.rs
crates/inputcodex-platform/src/platform_paths.rs
crates/inputcodex-platform/src/relay_environment_observation.rs
crates/inputcodex-platform/src/relay_environment_observation/macos.rs
crates/inputcodex-platform/src/relay_environment_observation/windows.rs
crates/inputcodex-platform/tests/relay_environment_observation.rs
docs/plans/2026-07-28-issue-89-gate-5-relay-environment-observation.md
docs/plans/PROJECT-MASTER-PLAN.md
docs/plans/sessions/2026-07-28-issue-89-gate-5-relay-environment-observation.md
docs/reports/issue-89-gate-5-relay-environment-observation.md
docs/workflows/2026-07-28-issue-89-gate-5-relay-environment-observation-runtime.md
err.md
parity/README.md
parity/contracts/provider-network.yml
parity/features/provider-network.yml
parity/features/source-index.yml
```

共 `30` 路径，哈希：
`sha256:0adc20d0ed4d73ae645a5ffb23d7208f7aaabfea92c4d6fd62e0da3a120e8f77`。

## 阶段授权

| 阶段 | 状态 | 允许操作 |
| --- | --- | --- |
| 决策与 Discovery | 已完成 | 只读仓库、上游缓存与 GitHub；创建 Issue/worktree |
| 规划冻结 | 当前允许 | 只写三份规划文件并执行轻量验证 |
| TDD 与实现 | 已授权 | 只在 `30` 路径内按 RED→GREEN 执行 |
| Cargo 更新 | 已授权 | 只允许加入 `windows-registry = 0.6.1` 及必要锁文件 |
| Git checkpoint/提交 | 已授权 | 每个稳定层完成后提交 checkpoint |
| 普通推送与 PR | 已授权 | 最终本地门禁后普通推送并创建非 Draft PR |
| Squash Merge | 未授权 | Final Head Review/CI 后单独授权 |

## 双哈希验证

```powershell
$ErrorActionPreference = 'Stop'

$planning = [string[]]@(
  'docs/plans/2026-07-28-issue-89-gate-5-relay-environment-observation.md',
  'docs/plans/sessions/2026-07-28-issue-89-gate-5-relay-environment-observation.md',
  'docs/workflows/2026-07-28-issue-89-gate-5-relay-environment-observation-runtime.md'
)

$candidate = [string[]]@(
  'AGENTS.md',
  'CONTEXT.md',
  'Cargo.lock',
  'Cargo.toml',
  'README.md',
  'build.md',
  'crates/inputcodex-application/src/lib.rs',
  'crates/inputcodex-application/src/relay_environment_observation.rs',
  'crates/inputcodex-application/tests/relay_environment_observation.rs',
  'crates/inputcodex-domain/src/lib.rs',
  'crates/inputcodex-domain/src/relay_environment_observation.rs',
  'crates/inputcodex-domain/tests/relay_environment_observation.rs',
  'crates/inputcodex-parity/tests/catalog_repository.rs',
  'crates/inputcodex-platform/Cargo.toml',
  'crates/inputcodex-platform/src/lib.rs',
  'crates/inputcodex-platform/src/platform_paths.rs',
  'crates/inputcodex-platform/src/relay_environment_observation.rs',
  'crates/inputcodex-platform/src/relay_environment_observation/macos.rs',
  'crates/inputcodex-platform/src/relay_environment_observation/windows.rs',
  'crates/inputcodex-platform/tests/relay_environment_observation.rs',
  'docs/plans/2026-07-28-issue-89-gate-5-relay-environment-observation.md',
  'docs/plans/PROJECT-MASTER-PLAN.md',
  'docs/plans/sessions/2026-07-28-issue-89-gate-5-relay-environment-observation.md',
  'docs/reports/issue-89-gate-5-relay-environment-observation.md',
  'docs/workflows/2026-07-28-issue-89-gate-5-relay-environment-observation-runtime.md',
  'err.md',
  'parity/README.md',
  'parity/contracts/provider-network.yml',
  'parity/features/provider-network.yml',
  'parity/features/source-index.yml'
)

function Get-ScopeHash([string[]]$Paths) {
  [Array]::Sort($Paths, [StringComparer]::Ordinal)
  $text = ($Paths -join "`n") + "`n"
  $bytes = [Text.UTF8Encoding]::new($false).GetBytes($text)
  'sha256:' + [Convert]::ToHexString(
    [Security.Cryptography.SHA256]::HashData($bytes)
  ).ToLowerInvariant()
}

if ((Get-ScopeHash $planning) -ne 'sha256:0a301df75edda05c8d3d1c01c91221dd9ac8ff11aeeca39fb4c26a293b0543b0') {
  throw 'planning scope hash mismatch'
}
if ((Get-ScopeHash $candidate) -ne 'sha256:0adc20d0ed4d73ae645a5ffb23d7208f7aaabfea92c4d6fd62e0da3a120e8f77') {
  throw 'candidate scope hash mismatch'
}
```

## 输入合同

- 当前进程环境只读快照；
- Windows 两个固定持久化环境键；
- 合法用户目录和 `CODEX_HOME`；
- 四个固定 Clash Verge 逻辑候选；
- 零敏感请求参数。

## 输出合同

- 成功始终返回 `Ready(RelayEnvironmentObservation)`；
- 没有风险不是 `Empty`；
- 局部不可用保留在领域状态中；
- 无法形成可信聚合时返回稳定 `ApplicationError`；
- 不输出变量值、`.env` 内容、注册表值或实际路径。

## 请求生命周期

沿用 `Idle → Loading → Ready/Failed` 与 `Loading → Cancelling → Idle`。每次请求使用唯一递增身份；超时、取消或新请求产生后，旧同步结果不得覆盖当前状态。

## TDD 执行矩阵

| 批次 | RED | GREEN | Checkpoint |
| --- | --- | --- | --- |
| Domain | 名称、来源、覆盖、文件状态、隐私 | 最小领域对象与不变量 | `issue-89-domain-green` |
| Application | Port、Ready/Failed、取消/过期 | 最小 Request/UseCase | `issue-89-application-green` |
| Shared Platform | 路径、环境、文件边界 | 共享探针与有界解析 | `issue-89-platform-shared-green` |
| Windows/macOS | 注册表、平台目录、覆盖差异 | 目标平台适配器 | `issue-89-platform-targets-green` |
| Parity | 缺失子能力与错误映射 | feature/contract/source-index | `issue-89-parity-green` |
| Closeout | 文档与最终禁止面 | 控制面和报告 | `issue-89-local-verified` |

## 写入强制

每个执行批次前后都必须：

1. `git status --short`；
2. 取得 `git diff --name-only` 并按 `StringComparer.Ordinal` 排序；
3. 证明所有路径均在批准 allowlist；
4. 检查未跟踪文件；
5. 执行 `git diff --check`；
6. 如需新增路径，立即停止并重新批准哈希。

规划阶段实际差异必须精确等于三份规划文件，不允许提前出现 Cargo、源码、Parity 或项目状态回写。

## 禁止能力与隐私扫描

实现阶段必须扫描并拒绝：

- `reqwest`、`hyper`、`TcpStream`、`UdpSocket` 与 URL 请求；
- `Command::new`、`std::thread`、Watcher、sleep、轮询和后台任务；
- `fs::write`、OpenOptions 写模式、环境修改和注册表写入；
- `unsafe`、Tauri、WebView、JavaScript、注入和远程推荐；
- 环境变量值、`.env` 内容、真实路径或注册表值进入日志/Debug；
- `core-module:proxy` 被误映射到新子能力；
- 原总功能被标记为 `implemented`。

## 本地轻量验证合同

规划阶段：

```powershell
git diff --check
git status --short
rg -n 'TODO|TBD|PLACEHOLDER|待定|待补' docs/plans/2026-07-28-issue-89-gate-5-relay-environment-observation.md docs/plans/sessions/2026-07-28-issue-89-gate-5-relay-environment-observation.md
```

批准后的实现阶段计划执行：

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

Workspace 全量、Windows/macOS 编译和 Performance observation 继续交给 GitHub-hosted runners。

## 错误恢复

- 重复 PowerShell Markdown、原生命令退出码或补丁 ACL 问题引用 `err.md` 既有条目；
- `windows-registry` 锁文件漂移超出自身及既有 windows-rs 依赖时停止；
- Linux all-targets 死代码优先收紧 `cfg`，禁止 `allow(dead_code)`；
- 任何真实功能语义冲突回到独立 Parity Decision Issue；
- 不通过修改 Workflow、Ruleset 或跳过测试绕过失败。

## 远端交付合同

- 仅普通 push，禁止 force push；
- PR 必须非 Draft、关联 Issue `#89`；
- Review 对话必须写明根因、处理和验证后解决；
- CI、Performance observation 与 Artifact 必须按项目合同核验；
- Final Head 全绿后只请求独立 Squash Merge 授权；
- 合并后不创建递归 Closeout。

## 当前进度

- [x] Issue `#88` 决策完成；
- [x] Issue `#89`、分支和 worktree 完成；
- [x] Discovery 与候选依赖核对完成；
- [x] 三份规划控制面落盘；
- [ ] 规划轻量验证与 Issue 回写；
- [x] 项目所有者批准 `30` 路径和哈希；
- [ ] TDD、实现、提交、推送、PR、Review/CI；
- [ ] 独立 Squash Merge 授权。

## 停止条件

- 未取得精确范围批准；
- 实际差异越出 allowlist；
- 需要第 `31` 条路径；
- 需要网络、写入、子进程、线程、UI、注入或 `unsafe`；
- 无法表达真实双平台覆盖或明确损坏状态；
- Release 审计不再 current；
- Review/CI 根因未闭环；
- 未取得独立 Squash Merge 授权。
