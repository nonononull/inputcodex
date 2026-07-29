# Issue #101 Session Plan：上下文能力只读目录观察

## Session Metadata

- `task_id`: `issue-101-gate-5-context-entry-observation`
- `tracking_issue_ref`: `https://github.com/nonononull/inputcodex/issues/101`
- `approved_decision_ref`: `https://github.com/nonononull/inputcodex/issues/100#issuecomment-5118455158`
- `implementation_scope_approval_ref`: `https://github.com/nonononull/inputcodex/issues/101#issuecomment-5119498422`
- `selected_business_path`: `gate-5/context-entry-observation`
- `baseline_ref`: `origin/main@52320c2c02e19d9ffae11ccb6742a0f0fc4b71b9`
- `branch_ref`: `codex/issue-101-gate-5-context-entry-observation`
- `worktree_ref`: `.worktrees/issue-101-gate-5-context-entry-observation`
- `planning_scope_hash`: `sha256:0393705157d30192e317a8158686baf6c2a79483abab1e5a7a5b109d30923dbd`
- `candidate_scope_hash`: `sha256:5b96235eb1fa7832e5710f7343917a5c2512bc50a46198ed584323366dd34372`
- `planning_validation`: `PASSED`
- `execution_state`: `PARITY_GREEN`
- `agos_report_only`: `needs-input-unregistered-bypassed`

## Mutation Intent

- `mutation_intent`: `source`
- 当前阶段实际允许写入：七路径 planning 控制面。
- 获得实施批准后的目标写入：二十四路径内的 Domain、Application、Platform、Parity、测试、
  稳定文档和任务报告。
- 禁止把上下文管理写入、完整 TOML、网络、SQLite、UI、线程或 AGOS 改动带入本切片。

## Executor Enforcement

- 当前阶段：`planning-freeze`
- 当前允许操作：Issue 写入、隔离分支/worktree、七路径控制面写入、范围哈希复算、只读验证。
- 当前禁止操作：产品 TDD、Rust 实现、Parity 状态修改、依赖修改、实现提交、push、PR、Review/CI、
  Squash Merge、force push、`main` 写入和 AGOS 控制面修改。
- 产品实现前置门：项目所有者批准二十四路径与 `candidate_scope_hash`。
- 最终合并前置门：具体 PR Final Head、全部 Review 根因闭环、Hosted CI 全绿、Artifact 合同和
  独立 Squash Merge 授权。

## Local Knowledge Lookup

```yaml
local_knowledge_lookup:
  gbrain_queries:
    - "inputcodex Gate 5 只读功能切片 本地会话 SQLite 性能 隐私" -> no-results
    - "CodexPlusPlus 上下文条目 Zed Remote Token 用量 副作用 语义" -> no-results
  vault_refs:
    - none: AI Growth OS vault 未命中 inputcodex 专项记录
  rules_refs:
    - D:/Android_source/ai-growth-os/components/rules/rules/workflows/ai-growth-os-auto-application.md
    - D:/Android_source/ai-growth-os/components/rules/rules/workflows/ai-growth-os-brainstorming-gate.md
    - AGENTS.md
  project_refs:
    - README.md
    - build.md
    - err.md
    - CONTEXT.md
    - docs/plans/PROJECT-MASTER-PLAN.md
    - parity/features/provider-network.yml
    - parity/features/source-index.yml
    - upstream/CodexPlusPlus/apps/codex-plus-manager/src-tauri/src/commands.rs
    - upstream/CodexPlusPlus/crates/codex-plus-core/src/relay_config.rs
  missing_coverage:
    - GBrain 与 vault 无项目专项知识；项目原生控制面、锁定上游快照和 GitHub 事实作为当前真源
```

仓库没有 `.codegraph/`，未擅自初始化索引。AGOS 只允许 ReportOnly；未登记、`needs-input`、
接口不兼容或异常时记录后绕过，不修改 AGOS。

## Approved Semantics

- 新功能：`feature.provider-network.context-entry-observation`
- 唯一来源：`tauri-command:read_live_context_entries`
- 固定文档：平台路径定位的 `CODEX_HOME/config.toml`
- 单文件上限：`256 KiB`
- 返回：ID、`McpServer / Skill / Plugin`、启用状态和分类计数
- 文件缺失：`Empty`
- 合法零条目：`Ready(0)`
- 文件或结构错误：稳定 `Failed`
- 禁止返回正文、摘要、命令、参数、环境、Header、URL、Token、账号和实际路径
- 原总功能和五个剩余入口继续 `unassessed`

## Change Contract

- `target_contract`: 固定 `config.toml` 的只读、有界、最小披露目录观察。
- `owner`: Issue `#101`，决策证据为 Issue `#100`。
- `preserved_invariants`:
  - `release_audit=current`
  - Windows/macOS 同一领域与错误语义
  - Iced 只在展示层
  - 无广告、遥测、注入、WebView、TypeScript 或 JavaScript 业务代码
  - 原上下文管理总功能继续未评估
  - 第八切片 Relay 状态观察行为不变
- `adjacent_surfaces`:
  - `SystemPlatformPaths`
  - `relay_status_observation`
  - 设置与 Relay 配置 TOML 解析边界
  - Provider/Network Parity source mapping
- `historical_state_refs`:
  - Issue `#26` Gate 4 来源目录
  - Issue `#65` v1.2.43 重新审计
  - Issue `#98` / PR `#99` Relay 状态观察
- `stale_verdict_invalidation_refs`:
  - Master Plan 顶部仍指向 Issue `#89`
  - Gate 5 正文尚未把 Issue `#98` / PR `#99` 标记完成
- `regression_checks`:
  - 四个受影响 crate 定向 tests/Clippy
  - `cargo fmt --all -- --check`
  - Release Audit、CI 合同、仓库政策、范围和隐私扫描
- `sibling_regression_guard`: `pending-implementation`

## Planning Allowlist

```text
AGENTS.md
build.md
CONTEXT.md
docs/plans/2026-07-29-issue-101-gate-5-context-entry-observation.md
docs/plans/PROJECT-MASTER-PLAN.md
docs/plans/sessions/2026-07-29-issue-101-gate-5-context-entry-observation.md
docs/workflows/2026-07-29-issue-101-gate-5-context-entry-observation-runtime.md
```

- `count`: `7`
- `hash`: `sha256:0393705157d30192e317a8158686baf6c2a79483abab1e5a7a5b109d30923dbd`

## Candidate Implementation Allowlist

```text
AGENTS.md
build.md
CONTEXT.md
crates/inputcodex-application/src/context_entry_observation.rs
crates/inputcodex-application/src/lib.rs
crates/inputcodex-application/tests/context_entry_observation.rs
crates/inputcodex-domain/src/context_entry_observation.rs
crates/inputcodex-domain/src/lib.rs
crates/inputcodex-domain/tests/context_entry_observation.rs
crates/inputcodex-parity/tests/catalog_repository.rs
crates/inputcodex-platform/src/context_entry_observation.rs
crates/inputcodex-platform/src/lib.rs
crates/inputcodex-platform/tests/context_entry_observation.rs
docs/plans/2026-07-29-issue-101-gate-5-context-entry-observation.md
docs/plans/PROJECT-MASTER-PLAN.md
docs/plans/sessions/2026-07-29-issue-101-gate-5-context-entry-observation.md
docs/reports/issue-101-gate-5-context-entry-observation.md
docs/workflows/2026-07-29-issue-101-gate-5-context-entry-observation-runtime.md
err.md
parity/contracts/provider-network.yml
parity/features/provider-network.yml
parity/features/source-index.yml
parity/README.md
README.md
```

- `count`: `24`
- `hash`: `sha256:5b96235eb1fa7832e5710f7343917a5c2512bc50a46198ed584323366dd34372`
- 哈希合同：Windows PowerShell `Sort-Object`、UTF-8 无 BOM、LF 拼接并保留末尾 LF。

## Execution Batches

### Batch 0：Planning Freeze

1. 写入七路径 planning 控制面。
2. 复算 planning/candidate 两个 hash。
3. 验证实际路径集精确等于 Planning allowlist。
4. 运行 Release Audit、Parity baseline、Cargo metadata、占位符和 diff 检查。
5. 运行 AGOS ReportOnly；不可用时记录并绕过。
6. 回写 Issue `#101` 后停止，等待实施批准。

状态：`completed`。七路径实际集合、planning hash、二十四路径 candidate hash、Release Audit、
CI 脚本合同、仓库政策、Cargo metadata、Parity baseline 与 diff 检查均已通过。

### Batch 1：Domain RED → GREEN

- 测试种类、条目、汇总、目录和计数不变量。
- 实现纯领域类型与脱敏 Debug。
- 状态：`completed`；RED 为目标类型未导出，GREEN 后 4 个新增测试与 Domain 全目标测试、Clippy、fmt 通过。

### Batch 2：Application RED → GREEN

- 测试零字段 Request、Port、UseCase、Some/None/Err 映射和旧请求隔离。
- 实现 `ObserveContextEntries<P>`。
- 状态：`completed`；RED 为目标应用类型未导出，GREEN 后 6 个新增测试与 Application 全目标测试、Clippy、fmt 通过。

### Batch 3：Platform RED → GREEN

- 测试固定路径、普通文件门禁、双重上限、严格 TOML、条目顺序、启用语义和最小披露。
- 实现 `SystemContextEntryObservation`，不触碰既有 Relay 状态实现。
- 状态：`completed`；完整 RED 矩阵转绿，Platform 全目标测试、Clippy、fmt 通过；新增 `DocumentMut` span 根因已写入 `err.md`。

### Batch 4：Parity RED → GREEN

- 新增 observation feature/contract/source mapping。
- 只移动 `read_live_context_entries`，其余入口保持原归属。
- 状态：`completed`；新目录测试由缺失条目 RED 转绿，Parity 目录测试 21/21、全目标测试、Clippy、fmt 与 Release Audit `current` 通过。

### Batch 5：Local Closeout

- 更新稳定文档与报告；先查 `err.md`，只有形成新可复用根因时才写入，否则实际范围固定排除该路径。
- 运行本地轻量全门禁并建立 named checkpoint。

### Batch 6：Review / CI

- 普通 push、非 Draft PR、Review 根因闭环、Hosted CI、Performance observation、Artifact 核验。
- 绑定 Final Head 请求独立 Squash Merge 授权。

## Agent Lifecycle

- `agent_strategy`: parent-only
- `open_agent_count_before_dispatch`: `0`
- `reclaim_before_spawn`: `not-required`
- `claimed_file_owners`: 当前父任务独占二十四路径；未授权任何 subagent 写入。
- `completion_status`: `no-subagents-used`
- `closed_agent_refs`: `none`

## Checkpoint Rules

- startup baseline：`52320c2c02e19d9ffae11ccb6742a0f0fc4b71b9`
- planning checkpoint：七路径验证通过且所有者批准前可建立本地 checkpoint，但不得 push
- implementation checkpoints：Domain、Application、Platform、Parity、local-verified
- 每个 checkpoint 前核对 `git status --short`、allowlist、scope hash 和 `git diff --check`
- 时间仅使用本机 `Get-Date`；禁止设置 `GIT_AUTHOR_DATE` 或 `GIT_COMMITTER_DATE`
- 禁止 amend、rebase、force push 或修改 `main`

## Stop Conditions

- 路径或语义新增、依赖新增、第二文件读取、网络、写入、子进程、线程、UI 或原始配置返回。
- `release_audit` 不为 `current`。
- 实际路径超出二十四路径，或无新根因时未精确使用排除 `err.md` 的二十三路径，或对应 hash 漂移。
- 测试失败根因未确定、Review 对话未闭环或 Hosted CI/Artifact 合同未通过。
- AGOS 异常只触发绕过记录，不得转成跨仓修复任务。

## Next Gate

Domain、Application、Platform 与 Parity TDD 已完成；下一合法节点为稳定文档、本地全门禁与 local-verified checkpoint。
