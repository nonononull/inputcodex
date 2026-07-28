# Issue #86 Runtime Workflow：运行时环境冲突只读观察

## 运行元数据

```yaml
status: implementation-authorized-parity-resumed
task_id: issue-86-gate-5-runtime-environment-observation
tracking_issue_ref: https://github.com/nonononull/inputcodex/issues/86
approved_decision_ref: https://github.com/nonononull/inputcodex/issues/85#issuecomment-5102509300
written_review_ref: https://github.com/nonononull/inputcodex/issues/86#issuecomment-5102713590
branch: codex/issue-86-gate-5-runtime-environment-observation
baseline_main: 3f2914cd81ace7afe28e0137c867c20fd346c3f9
upstream_release: v1.2.43
upstream_commit: 5036ff056b5c629f19356396b17d6eeb70da664c
planning_scope_count: 3
planning_scope_hash: sha256:c3d16ff75e79d9fd2866db1bd59f4259089b7398bce46b20e2766fc2bccc6d34
candidate_scope_count: 24
candidate_scope_hash: sha256:dd1d784ffe3149bf130c6bd678050d6aea3059f33a405abee5e2cc3f9735bb59
approved_scope_ref: https://github.com/nonononull/inputcodex/issues/86#issuecomment-5103198917
implementation_authorization: authorized
remote_write_authorization: authorized-normal-push-non-draft-pr-review-ci
final_merge_authorization: pending-separate-gate
local_time_source: Windows Get-Date
agos_status: bypassed-project-native-control-plane
```

## 节点图

```text
ISSUE_85_DECISION_COMPLETED
  -> ISSUE_86_CREATED
  -> ISOLATED_BRANCH_CREATED
  -> WRITTEN_DESIGN_CREATED
  -> WRITTEN_DESIGN_APPROVED
  -> SESSION_PLAN_CREATED
  -> RUNTIME_WORKFLOW_CREATED
  -> PLANNING_SCOPE_VERIFIED
  -> PLANNING_GIT_CHECKPOINT
  -> OWNER_CANDIDATE_SCOPE_APPROVAL_REQUIRED
  -> DOMAIN_RED
  -> DOMAIN_GREEN
  -> APPLICATION_RED
  -> APPLICATION_GREEN
  -> PLATFORM_RED
  -> PLATFORM_GREEN
  -> PARITY_RED
  -> PARITY_GREEN
  -> CONTROL_PLANE_CLOSEOUT
  -> LOCAL_LIGHTWEIGHT_VERIFICATION
  -> FINAL_GIT_CHECKPOINT
  -> NORMAL_PUSH
  -> NON_DRAFT_PR
  -> FINAL_HEAD_REVIEW_CI
  -> OWNER_SQUASH_MERGE_APPROVAL
  -> SQUASH_MERGE
  -> POST_MERGE_VERIFICATION
```

项目所有者已批准 `24` 路径修订范围，可执行到 `FINAL_HEAD_REVIEW_CI`；最终 Squash Merge 仍由
独立授权门锁定。

## 当前规划写入范围

```text
docs/plans/2026-07-28-issue-86-gate-5-runtime-environment-observation.md
docs/plans/sessions/2026-07-28-issue-86-gate-5-runtime-environment-observation.md
docs/workflows/2026-07-28-issue-86-gate-5-runtime-environment-observation-runtime.md
```

规范化规则固定为 `StringComparer.Ordinal` 升序、UTF-8 无 BOM、LF 分隔并以 LF 结尾；哈希为
`sha256:c3d16ff75e79d9fd2866db1bd59f4259089b7398bce46b20e2766fc2bccc6d34`。

## 候选完整实施范围

```text
AGENTS.md
CONTEXT.md
README.md
build.md
crates/inputcodex-application/src/lib.rs
crates/inputcodex-application/src/runtime_environment_observation.rs
crates/inputcodex-application/tests/runtime_environment_observation.rs
crates/inputcodex-domain/src/lib.rs
crates/inputcodex-domain/src/runtime_environment_observation.rs
crates/inputcodex-domain/tests/runtime_environment_observation.rs
crates/inputcodex-parity/tests/catalog_repository.rs
crates/inputcodex-platform/src/lib.rs
crates/inputcodex-platform/src/runtime_environment_observation.rs
crates/inputcodex-platform/tests/runtime_environment_observation.rs
docs/plans/2026-07-28-issue-86-gate-5-runtime-environment-observation.md
docs/plans/PROJECT-MASTER-PLAN.md
docs/plans/sessions/2026-07-28-issue-86-gate-5-runtime-environment-observation.md
docs/reports/issue-86-gate-5-runtime-environment-observation.md
docs/workflows/2026-07-28-issue-86-gate-5-runtime-environment-observation-runtime.md
err.md
parity/README.md
parity/contracts/foundation-platform.yml
parity/features/foundation-platform.yml
parity/features/source-index.yml
```

候选范围固定为 `24` 路径，哈希为
`sha256:dd1d784ffe3149bf130c6bd678050d6aea3059f33a405abee5e2cc3f9735bb59`。实现可以只修改其中必要
子集，不得为了凑数制造无意义差异。

## 阶段授权

| 阶段 | 当前状态 | 允许操作 |
| --- | --- | --- |
| Discovery 与书面设计 | 已完成并批准 | 只读项目、上游快照与 Issue，落盘设计稿 |
| 规划冻结 | 当前已授权 | 只写三份规划文件、轻量文档验证、Git checkpoint 与规划提交 |
| TDD 实施 | 已授权 | 只在 `24` 路径内执行 RED/GREEN、验证和 checkpoint |
| 普通推送、PR、Review/CI | 已授权 | 最终本地门禁通过后普通推送并创建非 Draft PR |
| Squash Merge | 未授权 | Final Head Review/CI 通过后单独授权 |

## 双哈希验证

```powershell
$ErrorActionPreference = 'Stop'

$planning = [string[]]@(
  'docs/plans/2026-07-28-issue-86-gate-5-runtime-environment-observation.md',
  'docs/plans/sessions/2026-07-28-issue-86-gate-5-runtime-environment-observation.md',
  'docs/workflows/2026-07-28-issue-86-gate-5-runtime-environment-observation-runtime.md'
)
[Array]::Sort($planning, [StringComparer]::Ordinal)
$planningText = [string]::Join("`n", $planning) + "`n"
$planningHash = [Convert]::ToHexString(
  [Security.Cryptography.SHA256]::HashData(
    [Text.UTF8Encoding]::new($false).GetBytes($planningText)
  )
).ToLowerInvariant()
if ($planning.Count -ne 3) { throw "Issue #86 规划路径数量漂移：$($planning.Count)" }
if ($planningHash -ne 'c3d16ff75e79d9fd2866db1bd59f4259089b7398bce46b20e2766fc2bccc6d34') {
  throw "Issue #86 planning_scope_hash 漂移：sha256:$planningHash"
}

$candidate = [string[]]@(
  'AGENTS.md',
  'CONTEXT.md',
  'README.md',
  'build.md',
  'crates/inputcodex-application/src/lib.rs',
  'crates/inputcodex-application/src/runtime_environment_observation.rs',
  'crates/inputcodex-application/tests/runtime_environment_observation.rs',
  'crates/inputcodex-domain/src/lib.rs',
  'crates/inputcodex-domain/src/runtime_environment_observation.rs',
  'crates/inputcodex-domain/tests/runtime_environment_observation.rs',
  'crates/inputcodex-parity/tests/catalog_repository.rs',
  'crates/inputcodex-platform/src/lib.rs',
  'crates/inputcodex-platform/src/runtime_environment_observation.rs',
  'crates/inputcodex-platform/tests/runtime_environment_observation.rs',
  'docs/plans/2026-07-28-issue-86-gate-5-runtime-environment-observation.md',
  'docs/plans/PROJECT-MASTER-PLAN.md',
  'docs/plans/sessions/2026-07-28-issue-86-gate-5-runtime-environment-observation.md',
  'docs/reports/issue-86-gate-5-runtime-environment-observation.md',
  'docs/workflows/2026-07-28-issue-86-gate-5-runtime-environment-observation-runtime.md',
  'err.md',
  'parity/README.md',
  'parity/contracts/foundation-platform.yml',
  'parity/features/foundation-platform.yml',
  'parity/features/source-index.yml'
)
[Array]::Sort($candidate, [StringComparer]::Ordinal)
$candidateText = [string]::Join("`n", $candidate) + "`n"
$candidateHash = [Convert]::ToHexString(
  [Security.Cryptography.SHA256]::HashData(
    [Text.UTF8Encoding]::new($false).GetBytes($candidateText)
  )
).ToLowerInvariant()
if ($candidate.Count -ne 24) { throw "Issue #86 候选路径数量漂移：$($candidate.Count)" }
if ($candidateHash -ne 'dd1d784ffe3149bf130c6bd678050d6aea3059f33a405abee5e2cc3f9735bb59') {
  throw "Issue #86 candidate_scope_hash 漂移：sha256:$candidateHash"
}
```

## 当前规划范围验证

```powershell
$changed = @(
  git diff --name-only origin/main...HEAD
  git diff --name-only
  git ls-files --others --exclude-standard
) | Where-Object { $_ } | Sort-Object -Unique

$outside = @($changed | Where-Object { $_ -notin $planning })
if ($outside.Count -ne 0) {
  throw "Issue #86 当前规划越界路径：$($outside -join ', ')"
}
if ($changed.Count -ne 3) {
  throw "Issue #86 当前规划应精确覆盖 3 路径，实际为 $($changed.Count)"
}
Write-Output "ISSUE86_PLANNING_SCOPE_GREEN changed=$($changed.Count) planning_scope_hash=sha256:$planningHash candidate_scope_hash=sha256:$candidateHash"
```

## 输入合同

- 系统入口每个请求只调用一次 `std::env::vars_os()`；
- 只观察当前 `inputcodex` 进程继承的环境，不读取用户级或系统级持久化来源；
- 纯观察函数接收注入的 `(OsString, OsString)` 迭代器，测试不得修改真实进程环境；
- Windows 名称比较大小写不敏感，命中后输出大写规范名；
- macOS 名称比较大小写敏感，命中后保留原始名称；
- 名称不修剪，只有真实前缀 `OPENAI_` 才命中，`CUSTOM_OPENAI_*` 等不得误判；
- 值只允许检查 `OsStr::is_empty`，不得转换、复制、格式化、返回、记录或持久化。

## 输出合同

```text
LoadCompletion::Ready(RuntimeEnvironmentConflictObservation {
  conflicts: sorted-and-deduplicated,
  coverage: {
    runtime_process: Observed,
    persistent_user: NotObserved,
    persistent_system: NotObserved
  },
  scanned_entry_count,
  conflict_count
})

LoadCompletion::Failed(ApplicationError {
  code: RUNTIME_ENVIRONMENT_OBSERVATION_UNSUPPORTED
      | RUNTIME_ENVIRONMENT_OBSERVATION_TIMEOUT
      | RUNTIME_ENVIRONMENT_NAME_UNREPRESENTABLE
})
```

无冲突也是 `Ready`，其中 `conflicts=[]`、`conflict_count=0`。`LoadCompletion::Empty` 对本能力不可达，
不得把 unsupported、超时、取消、名称不可表示或“未观察持久化来源”解释为空结果。

## 请求生命周期状态机

```text
IDLE
  -> begin(request_id): LOADING(current=request_id)

LOADING(current=request_id)
  -> port success before deadline and request still current: READY(observation)
  -> port failure before deadline and request still current: FAILED(stable_error)
  -> caller deadline reached: FAILED(RUNTIME_ENVIRONMENT_OBSERVATION_TIMEOUT)
  -> cancel(request_id): CANCELLED(request_id invalidated)
  -> begin(new_request_id): LOADING(current=new_request_id; old invalidated)

FAILED(timeout) / CANCELLED / superseded
  -> old synchronous result arrives: LATE_RESULT_DROPPED
  -> no second platform call
  -> no cleanup, rollback, retry thread or background work
```

超时只使请求及其迟到结果失效，不通过线程抢占或终止 `std::env::vars_os()`。取消和旧请求结果也不得
覆盖更新请求的状态。

## TDD 执行矩阵

| 批次 | RED 证据 | 最小 GREEN | 回归与 checkpoint |
| --- | --- | --- | --- |
| Domain | 名称前缀、平台规范化、空值状态、覆盖、排序去重和实际值隔离类型缺失 | 新增纯领域类型与构造约束 | Domain 定向测试、全 crate 测试/Clippy、Domain 提交 |
| Application | Port、用例、`Ready(empty)`、超时、取消和迟到结果语义缺失 | 复用现有协调器并增加最小同步用例 | Application 定向测试、Domain+Application 回归、Application 提交 |
| Platform | 双平台名称规则、单次采集、不可表示名称、值不泄露和 unsupported 缺失 | 注入式纯函数与系统适配器 | Platform 定向测试、前三层回归、Platform 提交 |
| Parity | 新子能力和合同缺失，原总功能状态可能被误改 | 新子能力 `implemented`，原总功能保持 `unassessed` | Parity 定向测试、四 crate 回归、Parity 提交 |
| Closeout | 控制面尚未反映真实实现与验证 | 最小更新文档、报告和命令 | 完整本地轻量验证与最终 Git checkpoint |

每个 RED 必须真实失败并保存失败原因；每个 GREEN 只实现当前批次需要的最小行为。失败先查
`err.md`，重复根因引用既有结论，新根因才允许在批准范围内更新 `err.md`。

## 写入强制与保护面

每个 checkpoint 必须先执行双哈希脚本，再执行：

```powershell
$changed = @(
  git diff --name-only origin/main...HEAD
  git diff --name-only
  git ls-files --others --exclude-standard
) | Where-Object { $_ } | Sort-Object -Unique

$outside = @($changed | Where-Object { $_ -notin $candidate })
if ($outside.Count -ne 0) {
  throw "Issue #86 越界路径：$($outside -join ', ')"
}

$protected = @($changed | Where-Object {
  $_ -eq 'Cargo.toml' -or
  $_ -eq 'Cargo.lock' -or
  $_ -like '*/Cargo.toml' -or
  $_ -like '.github/*' -or
  $_ -like 'upstream/*' -or
  $_ -like 'apps/*' -or
  $_ -like 'crates/inputcodex-presentation/*'
})
if ($protected.Count -ne 0) {
  throw "Issue #86 命中受保护路径：$($protected -join ', ')"
}
```

`Ruleset`、预算、Release 和 AGOS 不在本仓写入范围；发现外部缺口只记录，不得跨仓修复。

## 禁止能力与隐私扫描

实现后至少执行：

```powershell
$ErrorActionPreference = 'Stop'

$productFiles = @(
  'crates/inputcodex-domain/src/runtime_environment_observation.rs',
  'crates/inputcodex-application/src/runtime_environment_observation.rs',
  'crates/inputcodex-platform/src/runtime_environment_observation.rs'
)

$forbidden = @(
  rg -n 'std::env::set_var|std::env::remove_var|std::env::vars\(|std::fs|std::process::Command|std::thread|thread::spawn|tokio::|reqwest|ureq|unsafe\s*\{|windows::Win32|reg\.exe|launchctl|scutil' $productFiles 2>$null
)
if ($LASTEXITCODE -eq 0 -and $forbidden.Count -ne 0) {
  throw "Issue #86 命中禁止运行能力：$($forbidden -join '; ')"
}
if ($LASTEXITCODE -notin 0, 1) { throw 'Issue #86 禁止能力扫描执行失败。' }

$varsOsCalls = @(rg -n 'std::env::vars_os\(\)' crates/inputcodex-platform/src/runtime_environment_observation.rs 2>$null)
if ($LASTEXITCODE -ne 0 -or $varsOsCalls.Count -ne 1) {
  throw "Issue #86 系统入口必须精确调用一次 std::env::vars_os()，实际为 $($varsOsCalls.Count)"
}

$valueLeakPatterns = @(
  rg -n 'value\.(to_string|to_string_lossy|into_string)|format!\([^\r\n]*value|dbg!\(|println!\(' $productFiles 2>$null
)
if ($LASTEXITCODE -eq 0 -and $valueLeakPatterns.Count -ne 0) {
  throw "Issue #86 存在环境值格式化或调试泄露风险：$($valueLeakPatterns -join '; ')"
}
if ($LASTEXITCODE -notin 0, 1) { throw 'Issue #86 环境值泄露扫描执行失败。' }

$ownerName = Split-Path -Leaf $HOME
if ($ownerName) {
  $privateLeaks = @(
    rg -n --fixed-strings $ownerName crates parity docs/reports/issue-86-gate-5-runtime-environment-observation.md 2>$null
  )
  if ($LASTEXITCODE -eq 0 -and $privateLeaks.Count -ne 0) {
    throw "Issue #86 产品或证据表面泄露本机用户标识：$($privateLeaks -join '; ')"
  }
  if ($LASTEXITCODE -notin 0, 1) { throw 'Issue #86 本机标识扫描执行失败。' }
}
```

测试必须使用明显的秘密哨兵值，并断言领域结果、错误和生产 `Debug` 输出不包含该值。测试源码中的
哨兵字面量不属于产品泄露，静态扫描不得把测试输入误报为运行时输出。

## Checkpoint 合同

### Planning

- 只允许三份规划路径；
- 双哈希与当前规划范围验证必须通过；
- 设计、Session Plan 和 Runtime Workflow 不得包含占位符、非法控制字符或类型命名漂移；
- 形成规划提交并回写 Issue `#86` 后，停在所有者候选范围批准门。

### Domain

- RED 证明名称、值存在状态、覆盖、冲突和观察类型尚不存在；
- GREEN 后只允许 Domain 三路径与既有规划路径变化；
- 环境变量实际值不得进入任何领域类型或错误；
- 排序去重稳定，同名冲突任一值为 `NonEmpty` 时最终为 `NonEmpty`。

### Application

- RED 证明 Port、用例、`Ready(empty)`、超时、取消或迟到结果隔离缺失；
- GREEN 后成功只产生 `Ready`，失败只产生稳定错误，`Empty` 不可达；
- 每次执行只调用一次 Port，取消和超时不得触发第二次平台调用；
- 旧请求或迟到结果不得覆盖新请求。

### Platform

- RED 覆盖 Windows/macOS 名称规则、不修剪、`CUSTOM_OPENAI_*` 排除、空值、重复项、不可表示名称和 unsupported；
- GREEN 后系统适配器精确调用一次 `std::env::vars_os()`；
- 值只检查是否为空，不转换、不复制、不格式化；
- 不读取持久化来源，不使用环境写入、文件、网络、线程、子进程、FFI 或 `unsafe`。

### Parity

- RED 证明新子能力或合同尚未存在；
- GREEN 后 `feature.foundation-platform.runtime-environment-conflict-observation` 为 `implemented`；
- `feature.foundation-platform.environment-conflicts` 继续为 `unassessed`；
- 合同明确 `Ready(empty)`、三项覆盖、三条稳定错误/状态语义和禁止副作用；
- `parity/features/source-index.yml` 只修正 `check_env_conflicts` 为只读副作用并映射到新子能力；
- `core-module:env_conflicts` 与 `remove_env_conflicts` 的原总功能映射保持不变。

### Closeout

- 更新项目原生控制面和 Issue `#86` 实施报告；
- 运行四 crate 定向测试/Clippy、格式、CI 合同、Release Audit、Repository Policy、范围、保护面、隐私与禁止能力扫描；
- 形成最终本地验证 checkpoint；
- 普通推送并创建非 Draft PR 后，只做 Review/CI 根因闭环；
- 未获单独授权不得 Squash Merge。

## 本地轻量验证合同

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
git diff --check
```

预期：四 crate 全绿、CI 合同 `35/35`、`release_audit=current`、仓库政策 `0` 违规；再执行本文件
双哈希、范围、保护面、隐私和禁止能力脚本。Rust 全 Workspace 和 Windows/macOS 全量编译继续交给
公开仓库 GitHub-hosted runners。

## 错误恢复

1. 先读取 `err.md` 并查重复根因；
2. 固定最小失败命令、退出码、阶段和受影响路径；
3. 只提出一个可证伪根因假设并运行最小验证；
4. 根因确定后只修改批准范围内的必要路径；
5. 新根因写入 `err.md`，重复根因引用既有条目；
6. 修复后先重跑最小失败命令，再重跑所属 checkpoint；
7. 路径或候选哈希变化必须停止并重新取得所有者批准；
8. GitHub Actions 外部事故不得通过修改产品代码绕过。

## 远端交付合同

- 只允许普通推送，禁止 `--force` 和 `--force-with-lease`；
- PR 必须非 Draft，并关联 Issue `#85/#86`；
- 每条 Review 对话必须记录根因、处理方式和验证证据后才能解决；
- Final Head 必须完成标准 CI 与 Performance Baseline 合同核验；
- 成功 Run Artifact 数必须为 `0`；
- Final Head 变化后必须重跑全部适用门禁；
- 最终停在项目所有者单独 Squash Merge 授权门。

## 当前执行进度

```yaml
current_node: PARITY_RED
written_design_checkpoint: 26b3b1c54dd35cc92460879483b0f9d1f9d4793f
planning_documents: 3
planning_scope_hash: sha256:c3d16ff75e79d9fd2866db1bd59f4259089b7398bce46b20e2766fc2bccc6d34
candidate_scope_count: 24
candidate_scope_hash: sha256:dd1d784ffe3149bf130c6bd678050d6aea3059f33a405abee5e2cc3f9735bb59
planning_validation: ci-contract-35-of-35-release-audit-current-policy-zero
domain_checkpoint: 6591882dc23596a502833d38aed08d585b4acc08
application_checkpoint: 55b84b6c2b45d00fdf3f6e42aaa1e86d1635557e
platform_checkpoint: cd41fa8ef739b1481cfbfc491ef42e26369f0b4e
implementation_changes: domain-application-platform-complete
product_code_changes: runtime-environment-observation-complete
parity_changes: none
remote_push_pr_review_ci: authorized-after-local-verification
final_merge_authorization: pending-separate-gate
```

## 停止条件

出现任一情况立即停止并回到 Issue `#86`：

- 三路径规划范围、`24` 路径候选范围或任一哈希漂移；
- 未取得所有者范围批准却开始 Rust、Parity、根控制面、推送或 PR 写入；
- `source-index.yml` 超出批准的 `check_env_conflicts` 单入口修订；
- 需要 Cargo、新依赖、UI、预算、Release、Workflow、Ruleset、`upstream/` 或 AGOS 改动；
- 需要读取或修改持久化环境、文件、网络、缓存、线程、子进程、FFI 或 `unsafe`；
- 环境变量实际值进入领域、错误、诊断、日志、报告或生产 `Debug`；
- 原环境冲突总功能被标记为 `implemented`；
- Windows/macOS 无法保持相同领域与应用合同；
- 测试、Review 或 CI 失败根因未确定并解决；
- Review 对话未全部根因闭环；
- 请求 force push、删除或改写 `main`、Merge Commit 或 Rebase Merge；
- 最终 Squash Merge 未获得单独授权。
