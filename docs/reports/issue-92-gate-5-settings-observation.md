# Issue #92 设置只读观察候选实施报告

## 报告状态

- `status`: `IMPLEMENTATION_AUTHORIZED_PRE_TDD`
- `tracking_issue_ref`: `https://github.com/nonononull/inputcodex/issues/92`
- `approved_decision_ref`: `https://github.com/nonononull/inputcodex/issues/91`
- `approved_design_ref`: `https://github.com/nonononull/inputcodex/issues/92#issuecomment-5107059878`
- `owner_planning_approval_ref`: `https://github.com/nonononull/inputcodex/issues/92#issuecomment-5107673827`
- `owner_scope_approval_ref`: `https://github.com/nonononull/inputcodex/issues/92#issuecomment-5108221117`
- `scope_request_ref`: `https://github.com/nonononull/inputcodex/issues/92#issuecomment-5107913027`
- `baseline_ref`: `origin/main@a5559f4a873a81d91ed09b571503523a78a45118`
- `branch_ref`: `codex/issue-92-gate-5-settings-observation`
- `planning_scope_hash`: `sha256:dccd142c0c926433ce01adda37524895db2f4369917a6455fcfc393941a10cc2`
- `candidate_scope_hash`: `sha256:ca252684075d32de7aaf2ca066f12822ce48a5b01d1b0fcf67df146ea792baf1`

## 当前结论

Issue `#92` 的书面设计方案 A、二十七路径与候选哈希均已获批准，隔离分支/worktree 已从
精确基线创建。当前仍只完成 Discovery、计划、Runtime Workflow、候选范围和哈希；产品 TDD
尚未开始，下一节点为 Domain RED。

## Discovery 证据

### 上游语义

- `SettingsStore::load` 把文件不存在静默转换为默认设置。
- 损坏 JSON 使用 `unwrap_or_default()` 静默转换默认设置。
- `load_settings` 返回完整设置、私人路径和用户脚本清单。
- 保存、重置、皮肤、Provider、Relay 和脚本副作用与读取耦合。

### 可复用架构

- `PlatformPathsSnapshot::settings_file()` 已提供唯一设置路径来源。
- `PrivatePath` 的 `Debug` 已脱敏。
- `LoadCompletion` 已支持 Ready/Empty/Failed。
- `LoadCoordinator` 已提供请求身份、取消、超时、过期结果和错误隔离。
- Domain/Application/Platform 已有只读观察切片模式。
- 不需要 Iced、异步运行时、线程、网络或基础设施存储。

### 依赖

- 唯一新直接依赖候选为 `serde_json = "=1.0.149"`。
- 版本与上游 Release 锁文件一致。
- 当前阶段尚未修改 `Cargo.toml` 或 `Cargo.lock`。
- 实施时必须通过 Cargo 生成锁文件，并证明无关依赖漂移为 `0`。

## 批准语义摘要

- 新能力：`feature.foundation-platform.settings-observation`。
- 唯一来源：`tauri-command:load_settings`。
- 唯一产品事实：`top_level_entry_count`。
- 缺失：`Empty / NotConfigured`；合法 `{}`：`Ready(count=0)`。
- 上限：`256 KiB`，读取门禁 `256 KiB + 1`。
- 只接受 JSON object 根；使用六个稳定设置观察错误码。
- 原设置管理继续 `unassessed`。
- Parity 目标：feature/contract `39/39`、source `133`、fixture manifest `11`。

## 当前规划差异

```text
docs/plans/2026-07-28-issue-92-gate-5-settings-observation.md
docs/plans/sessions/2026-07-28-issue-92-gate-5-settings-observation.md
docs/reports/issue-92-gate-5-settings-observation.md
docs/workflows/2026-07-28-issue-92-gate-5-settings-observation-runtime.md
```

- `planning_scope_count`: `4`
- `planning_scope_hash`: `sha256:dccd142c0c926433ce01adda37524895db2f4369917a6455fcfc393941a10cc2`
- `product_source_changes`: `0`
- `cargo_changes`: `0`
- `parity_changes`: `0`
- `upstream_changes`: `0`

## 候选实施范围

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

- `candidate_scope_count`: `27`
- `candidate_scope_hash`: `sha256:ca252684075d32de7aaf2ca066f12822ce48a5b01d1b0fcf67df146ea792baf1`

## 验证状态

```yaml
planning_diff_check: passed
planning_scope_check: passed-4-of-4
planning_hash_recalculation: passed
candidate_hash_recalculation: passed-27-paths
placeholder_scan: passed-0-matches
protected_path_changes: 0
git_diff_check: passed
line_endings: LF
utf8_bom: 0
extra_blank_line_at_eof: 0
local_knowledge_lookup: project-native-doc-and-code-query-completed
agos_report_only: bypassed-known-unregistered-contract-err-md-2026-07-21
product_implementation: authorized-not-started
git_checkpoint: authorized
commit: authorized
push: authorized
pr: authorized
review_ci: not-reached
squash_merge: not-authorized
```

## 当前实施门

产品实现必须严格限制在 `27` 路径与既定语义内；新增路径、依赖或能力立即停止重新批准。
Review/CI 根因闭环后，最终 Squash Merge 继续等待项目所有者针对具体 PR 与 Final Head 的单独授权。
