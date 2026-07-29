# Issue #95 Session Plan：诊断日志只读结构观察

## Session Metadata

- `task_id`: `issue-95-gate-5-diagnostic-log-observation`
- `tracking_issue_ref`: `https://github.com/nonononull/inputcodex/issues/95`
- `approved_decision_ref`: `https://github.com/nonononull/inputcodex/issues/94#issuecomment-5109582183`
- `owner_scope_approval_ref`: `https://github.com/nonononull/inputcodex/issues/95#issuecomment-5113647548`
- `selected_business_path`: `gate-5/diagnostic-log-observation`
- `baseline_ref`: `origin/main@9587549c3f1bb334507075499f806485d83fce6a`
- `branch_ref`: `codex/issue-95-gate-5-diagnostic-log-observation`
- `worktree_ref`: `.worktrees/issue-95-gate-5-diagnostic-log-observation`
- `planning_scope_hash`: `sha256:14d78bf1a92f5b8db58650b501fb0cebee59a329823ff49e7b8ff3e93e0b7231`
- `candidate_scope_hash`: `sha256:8d407c269436c655e12ff94035183de6aa50dc7759fbc75f9cb7b6f9b0349d38`
- `planning_validation`: `PASSED`
- `agos_report_only`: `needs-input-known-unregistered-bypassed`

## Mutation Intent

将上游诊断总功能中的 `tauri-command:read_latest_logs` 拆成独立、只读、最小披露的结构观察能力。
实现只允许读取现有平台路径定位的诊断日志尾部，输出大小、记录分类和截断事实；原始文本、路径、
设置、凭据和其他诊断副作用不得进入新架构。

## Executor Enforcement

- 当前阶段：`implementation-authorized`
- 当前写入 allowlist：四份 task-local 规划文件
- 当前允许操作：二十四路径内 TDD、稳定文档、本地轻量验证、Git checkpoint、普通 push、非 Draft PR、Review/CI
- 当前禁止操作：范围外写入、依赖变更、合并、force push、`main` 写入
- 产品实现前置门：`PASSED`，证据为 `owner_scope_approval_ref`
- 最终合并前置门：具体 PR Final Head、Review 根因闭环、Hosted CI 全绿和独立 Squash Merge 授权

## 本地知识与来源

已读取并比对：

- `README.md`、`build.md`、`err.md`、`AGENTS.md`、`docs/plans/PROJECT-MASTER-PLAN.md`
- 既有设置观察的 Domain/Application/Platform/Parity 分层模式
- `upstream/CodexPlusPlus/crates/codex-plus-core/src/diagnostic_log.rs`
- 上游 `commands.rs` 中 `read_latest_logs`、`clear_logs`、`copy_diagnostics`、
  `write_diagnostic_event` 和尾部读取实现
- 当前 `SystemPlatformPaths` 私有诊断日志路径能力

仓库没有 `.codegraph/`，未初始化新索引；本次使用项目原生文档、代码查询和 Git/GitHub 事实完成
`local_knowledge_lookup`。AGOS 只允许 ReportOnly；若仍命中 `err.md` 已记录的未登记/不兼容合同，
立即按项目规则绕过，禁止修改 AGOS。

## 已批准语义

- 新功能：`feature.foundation-platform.diagnostic-log-observation`
- 唯一来源：`tauri-command:read_latest_logs`
- 缺失文件：`LoadCompletion::Empty`，Parity 稳定语义名为 `NoDiagnosticLog`
- 空普通文件：`Ready` 且六个事实为零/false
- 尾部窗口：最多 `256 KiB`
- 大文件：`truncated = true`，丢弃首个可能残缺片段并留证
- 合法记录：完整非空 UTF-8 行且 JSON 根为 object
- 损坏记录：空行、非法 UTF-8、损坏 JSON、非 object JSON
- 不变量：`sampled = valid + malformed`
- 稳定错误：unsupported、unavailable、invalid file type
- 禁止泄露任何正文、字段、事件、detail、PID、时间戳、路径或凭据

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

## 执行批次

### Batch 0：规划冻结

1. 已落盘四份控制面。
2. 已验证实际 diff 精确等于四路径。
3. 已复算 planning/candidate 两个哈希。
4. 已通过 CI 脚本合同、仓库政策、占位符和 `git diff --check`。
5. 下一步建立 planning checkpoint，普通 push 并回写 Issue `#95`。
6. 随后停在产品实现范围批准门。

### Batch 1：Domain RED → GREEN

1. 先写编译失败/行为失败测试，证明领域类型和六字段不存在。
2. 实现私有字段、构造与 getter，并从合法/损坏计数派生 sampled 计数以强制不变量。
3. 验证零值、混合计数、截断标志和隐私 Debug。
4. 运行 Domain tests/Clippy，建立 named Git checkpoint。

### Batch 2：Application RED → GREEN

1. 先写 Request/Port/UseCase 缺失与完成态映射测试。
2. 实现零字段请求与 `ObserveDiagnosticLog<P>`。
3. 固定 Some/None/Err 到 Ready/Empty/Failed 的映射。
4. 运行 Application tests/Clippy，建立 named Git checkpoint。

### Batch 3：Platform RED → GREEN

1. 用模块私有探针先覆盖缺失、空文件、记录分类、截断和错误。
2. 实现固定路径、普通文件门禁、Seek 尾读、首片段丢弃与逐行严格解析。
3. 禁止公开任意路径 API，禁止保存或返回文本。
4. 运行 Platform tests/Clippy/fmt，建立 named Git checkpoint。

### Batch 4：Parity RED → GREEN

1. 先证明新 feature/contract/source 映射不存在。
2. 新增 observation 子能力与合同，只移动 `read_latest_logs`。
3. 固定原总功能 `unassessed`、feature/contract `40/40`、source `133`、fixture `11`。
4. 运行 Parity tests/Clippy，建立 named Git checkpoint。

### Batch 5：稳定控制面与本地收口

1. 只把稳定用户能力写入 README；动态 Head/Run/Artifact 留在 GitHub。
2. 更新 AGENTS、CONTEXT、Master Plan、Parity README 和 build 命令。
3. `err.md` 仅在出现新且可复用根因时更新，否则保持不变。
4. 执行四 crate 定向 tests/Clippy、fmt、CI 合同、仓库政策、范围和隐私扫描。
5. 建立最终本地验证 checkpoint。

### Batch 6：远端交付

1. 只允许普通 push，创建关联 Issue `#95` 的非 Draft PR。
2. Review 对话逐条记录根因、处理方式和验证证据后解决。
3. 核验 Hosted CI 与 Performance observation 的预期 Job 和 Artifact 合同。
4. 绑定 PR 与 Final Head 请求独立 Squash Merge 授权。

## 验证成功标准

- 当前规划 diff 精确为四路径，哈希复算一致。
- 实现后 Domain/Application/Platform/Parity 定向 tests 与 Clippy 全绿。
- `cargo fmt --all -- --check`、CI 合同和仓库政策全绿。
- feature/contract `40/40`、source `133`、fixture manifest `11`。
- 禁止能力和隐私扫描命中为 `0`。
- 不新增依赖，不改 Cargo、UI、Workflow、Ruleset、Release、`upstream/` 或 AGOS。
- PR Review 根因闭环，Hosted CI 与 Artifact 合同通过。

## 硬停止

出现范围漂移、依赖需求、任意路径入口、正文披露、写入/网络/子进程/线程/UI 需求、基线漂移、
哈希不一致或验证失败时，返回最近稳定 checkpoint 并在 Issue `#95` 说明根因；不得扩大授权解释。
