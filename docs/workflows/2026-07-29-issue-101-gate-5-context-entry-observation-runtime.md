# Issue #101 Runtime Workflow：上下文能力只读目录观察

## Runtime Metadata

- `task_id`: `issue-101-gate-5-context-entry-observation`
- `tracking_issue_ref`: https://github.com/nonononull/inputcodex/issues/101
- `session_plan_ref`: `docs/plans/sessions/2026-07-29-issue-101-gate-5-context-entry-observation.md`
- `approved_decision_ref`: https://github.com/nonononull/inputcodex/issues/100#issuecomment-5118455158
- `implementation_scope_approval_ref`: https://github.com/nonononull/inputcodex/issues/101#issuecomment-5119498422
- `selected_business_path`: `gate-5/context-entry-observation`
- `baseline_ref`: `52320c2c02e19d9ffae11ccb6742a0f0fc4b71b9`
- `branch_ref`: `codex/issue-101-gate-5-context-entry-observation`
- `candidate_scope_hash`: `sha256:5b96235eb1fa7832e5710f7343917a5c2512bc50a46198ed584323366dd34372`
- `runtime_state`: `domain-green`
- `agos_status`: `needs-input-unregistered-bypassed`

## Current Gate

```text
Issue #100 owner decision approved
  -> Issue #101 planning scope approved
  -> Domain RED / GREEN / VERIFY completed
  -> NEXT: Application RED / GREEN
```

当前 `ALLOWED_OPS`：

- 二十四路径批准范围内的 TDD、文档回写和本地验证。
- Git checkpoint、普通 push、非 Draft PR 与 Review/CI。

当前 `FORBIDDEN_OPS`：

- 批准范围外文件、依赖、Cargo、CI、应用、上游缓存、脚本或性能预算修改。
- 第二文件读取、写入、联网、子进程、线程、Watcher、UI 或原始配置返回。
- Squash Merge、force push 或 `main` 写入。

## Node Order

### Node 0：Startup Baseline

1. `Get-Date` 记录本机时间。
2. 确认 `main == origin/main == 52320c2c...`。
3. 确认隔离 worktree 基于相同提交且仅跟踪 Issue `#101`。
4. 运行 Cargo metadata、Release Audit 和 Parity baseline。

状态：`completed`。

### Node 1：Decision Evidence

1. Issue `#100` 固化方案 A、错误语义和最小披露。
2. 回写项目所有者原话“批准方案 A”。
3. 按 `COMPLETED` 关闭 Issue `#100`。

状态：`completed`。

### Node 2：Planning Freeze

1. 创建 Issue `#101`、分支和 worktree。
2. 写书面计划、Session Plan、Runtime Workflow、术语和 Master Plan 状态。
3. `build.md` 固化二十四路径和验证器。
4. 复算 planning/candidate hash。
5. 回写 Issue `#101` 并停止。

状态：`completed`。Planning 实际路径、两个 hash、Release Audit、CI 脚本合同、仓库政策、
Cargo metadata、Parity baseline 和 diff 检查均已通过；下一节点保持所有者批准阻断。

### Node 3：Domain TDD

前置：`implementation_scope_approval_ref != pending`。

1. RED：领域测试引用不存在的类型与不变量。
2. GREEN：实现种类、条目、汇总、目录和脱敏 Debug。
3. VERIFY：Domain tests、Clippy、fmt。
4. CHECKPOINT：`issue-101-domain-green`。

状态：`completed`。RED 精确命中目标类型缺失；GREEN 后 Domain 全目标测试、Clippy 与 fmt 通过。

### Node 4：Application TDD

1. RED：Request、Port、UseCase、Some/None/Err 与旧请求隔离。
2. GREEN：实现 `ObserveContextEntries<P>`。
3. VERIFY：Application tests、Clippy、fmt。
4. CHECKPOINT：`issue-101-application-green`。

状态：`ready`。

### Node 5：Platform TDD

1. RED：固定路径、有界读取、严格 TOML、类型错误、顺序、启用状态和隐私矩阵。
2. GREEN：实现 `SystemContextEntryObservation`。
3. VERIFY：Platform tests、Clippy、fmt。
4. CHECKPOINT：`issue-101-platform-green`。

状态：`blocked-by-node-4`。

### Node 6：Parity TDD

1. RED：新 feature/contract/source mapping 当前不存在。
2. GREEN：只移动 `read_live_context_entries`。
3. VERIFY：Parity tests、Release Audit、Clippy、fmt。
4. CHECKPOINT：`issue-101-parity-green`。

状态：`blocked-by-node-5`。

### Node 7：Local Closeout

1. 更新稳定 README、AGENTS、CONTEXT、Master Plan、Parity README 和任务报告。
2. 先查 `err.md`；只有形成新可复用根因时才写入，否则最终实际范围必须排除该路径。
3. 运行四 crate 定向 tests/Clippy、fmt、CI 合同、仓库政策、Release Audit、Cargo metadata、
   scope hash、禁止能力、隐私和 `git diff --check`。
4. 建立 `issue-101-local-verified` named checkpoint。

状态：`blocked-by-node-6`。

### Node 8：Remote Delivery

1. 普通 push 当前分支。
2. 创建关联 Issue `#101` 的非 Draft PR。
3. Review 对话逐条记录根因、处理和验证证据后解决。
4. 核验标准 CI、Performance observation 和 Artifact 合同。
5. 绑定 Final Head 请求独立 Squash Merge 授权。

状态：`blocked-by-node-7`。

## Scope Enforcement

候选范围固定为二十四路径；完整清单以 Session Plan 和 `build.md` 为准。

```powershell
$paths = @(
  'AGENTS.md',
  'build.md',
  'CONTEXT.md',
  'crates/inputcodex-application/src/context_entry_observation.rs',
  'crates/inputcodex-application/src/lib.rs',
  'crates/inputcodex-application/tests/context_entry_observation.rs',
  'crates/inputcodex-domain/src/context_entry_observation.rs',
  'crates/inputcodex-domain/src/lib.rs',
  'crates/inputcodex-domain/tests/context_entry_observation.rs',
  'crates/inputcodex-parity/tests/catalog_repository.rs',
  'crates/inputcodex-platform/src/context_entry_observation.rs',
  'crates/inputcodex-platform/src/lib.rs',
  'crates/inputcodex-platform/tests/context_entry_observation.rs',
  'docs/plans/2026-07-29-issue-101-gate-5-context-entry-observation.md',
  'docs/plans/PROJECT-MASTER-PLAN.md',
  'docs/plans/sessions/2026-07-29-issue-101-gate-5-context-entry-observation.md',
  'docs/reports/issue-101-gate-5-context-entry-observation.md',
  'docs/workflows/2026-07-29-issue-101-gate-5-context-entry-observation-runtime.md',
  'err.md',
  'parity/contracts/provider-network.yml',
  'parity/features/provider-network.yml',
  'parity/features/source-index.yml',
  'parity/README.md',
  'README.md'
) | Sort-Object
$payload = ($paths -join "`n") + "`n"
$hash = [Convert]::ToHexString(
  [Security.Cryptography.SHA256]::HashData(
    [Text.UTF8Encoding]::new($false).GetBytes($payload)
  )
).ToLowerInvariant()
if ($paths.Count -ne 24) { throw "Issue #101 路径数量漂移：$($paths.Count)" }
if ($hash -ne '5b96235eb1fa7832e5710f7343917a5c2512bc50a46198ed584323366dd34372') {
  throw "Issue #101 scope_hash 漂移：sha256:$hash"
}
```

## Error Watchlist

- PowerShell 路径哈希必须使用本机 `Sort-Object`、UTF-8 无 BOM、LF 与末尾换行。
- Git 范围必须合并 `origin/main...HEAD`、工作区 diff 和未跟踪文件。
- `rg` 在 Windows 不接受 shell 风格路径通配符，必须使用 `--glob` 或显式路径。
- 多行 GitHub 正文使用临时文件与 `--body-file`，禁止把 `string[]` 直接传参。
- `apply_patch.bat` 返回 `Access is denied` 时，使用 `codex.ps1 --codex-run-as-apply-patch`；
  不修改包装器或系统 ACL。
- 所有原生命令后显式检查 `$LASTEXITCODE`。

## Verification Gates

### Planning Gate

- planning 实际路径精确为七路径。
- planning hash 为 `sha256:0393705157d30192e317a8158686baf6c2a79483abab1e5a7a5b109d30923dbd`。
- candidate 范围为二十四路径，hash 为
  `sha256:5b96235eb1fa7832e5710f7343917a5c2512bc50a46198ed584323366dd34372`。
- 无新根因时最终实际范围为排除 `err.md` 的二十三路径，hash 为
  `sha256:08b223934a07a66d91e5cf2e1b340a243ea460d6c4edc266f58d30101c478d47`。
- Release Audit `current`，Parity baseline、Cargo metadata 和 diff 检查通过。

### Implementation Gate

- 仅在 Issue `#101` 存在项目所有者对二十四路径和 hash 的明确批准评论后开启。
- 每个 TDD 批次必须先证明 RED 根因，再写最小 GREEN。
- 任何新增依赖、第二文件、写入、网络、子进程、线程或 UI 立即停止。

### Delivery Gate

- 实际路径是批准范围子集且实际 hash 等于批准 hash。
- 所有 Review 对话根因闭环。
- Hosted CI、Performance observation 和 Artifact 合同通过。
- Squash Merge 仅在项目所有者针对 Final Head 单独授权后执行。

## AGOS Boundary

AGOS ReportOnly 返回 `needs-input/unregistered`，并要求跨仓 task-backlog 注册。根据 inputcodex
项目规则，该状态已记录并绕过；不得在 Issue `#101` 修改 AGOS Registry、脚本、规则、Workflow
或 Vault，也不得让其阻塞项目原生 Issue/PR 流程。

## Rollout Draft

- `workflow_family`: `gate-5-read-only-observation`
- `reusable_path`: 固定平台路径 → 普通文件门禁 → 双重大小上限 → 严格解析 → 最小领域投影
- `skill_usage`: brainstorming、writing-plans、using-git-worktrees、karpathy-guidelines、domain-modeling
- `failure_recovery`: err.md 复用、AGOS 绕过、scope hash 重新批准、Review 根因闭环
- `record_after_closeout`: 仅在合并后形成可复用增量时调用项目允许的 rollout 记录入口
