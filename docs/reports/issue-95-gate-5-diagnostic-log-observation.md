# Issue #95 诊断日志只读结构观察报告

## 状态

- `phase`: `LOCAL_VERIFIED_PR_PENDING`
- `tracking_issue_ref`: `https://github.com/nonononull/inputcodex/issues/95`
- `approved_decision_ref`: `https://github.com/nonononull/inputcodex/issues/94#issuecomment-5109582183`
- `owner_scope_approval_ref`: `https://github.com/nonononull/inputcodex/issues/95#issuecomment-5113647548`
- `baseline_ref`: `9587549c3f1bb334507075499f806485d83fce6a`
- `planning_validation`: `PASSED`
- `implementation`: `LOCAL_VERIFIED`
- `pr_ref`: `NOT_REACHED`
- `squash_merge`: `NOT_AUTHORIZED`

## 决策结果

Issue `#94` 方案 1 已固定：设置管理继续 `unassessed`，不复制上游单体设置结构，也不提前建立
无人使用的通用写入抽象。第七个 Gate 5 产品切片改为独立的诊断日志只读结构观察。

本 Issue 只接管 `tauri-command:read_latest_logs`；原诊断总功能及清理、复制报告、写事件能力继续
未评估，不进入当前架构。

## Discovery 证据

### 上游事实

- `crates/codex-plus-core/src/diagnostic_log.rs` 负责诊断事件追加与日志文件路径。
- `commands.rs` 的 `read_latest_logs` 返回实际路径和原始文本。
- `clear_logs` 会写入/清理日志；`write_diagnostic_event` 会追加事件。
- `copy_diagnostics` 会聚合设置、安装/日志路径与状态，属于高披露副作用。
- 上游尾部读取使用 `2 MiB` 和 lossy UTF-8，无法保留严格损坏语义。

### 本项目复用点

- `SystemPlatformPaths` 已拥有私有诊断日志文件定位能力。
- Domain/Application 的只读观察模式已由前序 Gate 5 切片验证。
- `serde_json = "=1.0.149"` 已是 Workspace 依赖，本切片不需要新增依赖。
- 当前 Parity 基线为 feature/contract `39/39`、source `133`、fixture manifest `11`。

## 设计结果

### 返回事实

- `file_size_bytes`
- `sampled_record_count`
- `valid_object_record_count`
- `malformed_record_count`
- `truncated`
- `partial_record_discarded`

计数不变量为：

```text
sampled_record_count = valid_object_record_count + malformed_record_count
```

### 状态语义

- 文件不存在：`LoadCompletion::Empty`；Parity 稳定语义名为 `NoDiagnosticLog`。
- 合法空普通文件：`Ready`，计数为 `0`，标志为 `false`。
- 有内容普通文件：`Ready`，只返回结构计数与截断事实。
- 非支持平台：`DIAGNOSTIC_LOG_OBSERVATION_UNSUPPORTED`。
- 元数据、打开、Seek 或读取失败：`DIAGNOSTIC_LOG_OBSERVATION_UNAVAILABLE`。
- 符号链接或非普通文件：`DIAGNOSTIC_LOG_OBSERVATION_INVALID_FILE_TYPE`。

### 读取合同

- 固定尾部窗口 `256 KiB`。
- 大文件从尾部 Seek，首个可能残缺片段被丢弃并留证。
- 完整非空记录只有严格 UTF-8 且 JSON 根为 object 才计为合法。
- 空行、非法 UTF-8、损坏 JSON、数组/标量/null 等非 object 根计为 malformed。
- 单条损坏是成功观察中的事实，不把整个文件伪造成不可用。

### 隐私与副作用

禁止返回或持久化日志正文、JSON 字段、事件、detail、PID、时间戳、路径、用户名、机器名、
设置或凭据。禁止网络、写入、清理、复制报告、子进程、线程、Watcher、UI、注入和 `unsafe`。

## Parity 预期

- 新功能：`feature.foundation-platform.diagnostic-log-observation`
- 新功能状态：实现完成后 `implemented`
- 新来源映射：仅 `tauri-command:read_latest_logs`
- 新副作用：`[filesystem-read]`
- 原 `feature.foundation-platform.diagnostics`：继续 `unassessed`
- 预期 feature/contract：`40/40`
- source：`133`
- fixture manifest：`11`

## 范围冻结

### 当前 Planning

- 路径数：`4`
- 哈希：`sha256:14d78bf1a92f5b8db58650b501fb0cebee59a329823ff49e7b8ff3e93e0b7231`

### 候选实现

- 路径数：`24`
- 哈希：`sha256:8d407c269436c655e12ff94035183de6aa50dc7759fbc75f9cb7b6f9b0349d38`
- 不包含 Cargo、UI、Workflow、Ruleset、Release、`upstream/` 或 AGOS 写入。

## 本地实施 Checkpoints

- Planning：`9ae2447619bd26e6818968b9c3f1cf8df3e61cc6`。
- Domain GREEN：`b6b05b4e0e2ac21086f24927d2106fd63ee7b048`。
- Application GREEN：`0b2093044f6f9598f7ff56fbbc73c5c5a0469162`。
- Platform GREEN：`d647fadd264084d6376a6b3f64c8819e7f698552`。
- Parity GREEN：`4ea54fdf4b81cb8eee76d36f9917047098769613`。

## 当前验证证据

```yaml
planning_files_written: passed-4-paths
planning_allowlist: passed-exact
planning_scope_hash: verified-by-independent-recalculation
candidate_scope_hash: verified-by-independent-recalculation
ci_script_contract: passed-35
repository_policy: passed-zero-violations
placeholder_scan: passed
git_diff_check: passed
local_knowledge_lookup: project-native-doc-and-code-query-completed
agos_report_only: needs-input-known-unregistered-bypassed
domain_checkpoint: green
application_checkpoint: green
platform_checkpoint: green
parity_checkpoint: green
four_crate_tests_all_targets: passed
four_crate_clippy_all_targets: passed
rustfmt: passed
release_audit: current
candidate_scope: passed-24-paths
actual_scope: passed-23-paths
err_md: unchanged
cargo_delta: none
feature_contract_counts: 40/40
source_entry_count: 133
fixture_manifest_count: 11
privacy_matches: 0
forbidden_capability_matches: 0
validation_contract_fix: scan-added-lines-and-untracked-files
reused_err_root_cause: 2026-07-21-validator-self-scan-false-positive
full_local_verification: passed
review_ci: not-reached
```

## 下一门

1. 建立最终本地验证 checkpoint。
2. 普通推送并创建关联 Issue `#95` 的非 Draft PR。
3. Review/CI 全绿后绑定 Final Head，请求独立 Squash Merge 授权。
