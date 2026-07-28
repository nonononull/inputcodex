# Issue #92 设置只读观察实施报告

## 报告状态

- `status`: `LOCAL_VERIFIED_PR_PENDING`
- `tracking_issue_ref`: `https://github.com/nonononull/inputcodex/issues/92`
- `approved_decision_ref`: `https://github.com/nonononull/inputcodex/issues/91`
- `approved_design_ref`: `https://github.com/nonononull/inputcodex/issues/92#issuecomment-5107059878`
- `owner_planning_approval_ref`: `https://github.com/nonononull/inputcodex/issues/92#issuecomment-5107673827`
- `scope_request_ref`: `https://github.com/nonononull/inputcodex/issues/92#issuecomment-5107913027`
- `owner_scope_approval_ref`: `https://github.com/nonononull/inputcodex/issues/92#issuecomment-5108221117`
- `baseline_ref`: `origin/main@a5559f4a873a81d91ed09b571503523a78a45118`
- `branch_ref`: `codex/issue-92-gate-5-settings-observation`
- `planning_scope_hash`: `sha256:dccd142c0c926433ce01adda37524895db2f4369917a6455fcfc393941a10cc2`
- `candidate_scope_hash`: `sha256:ca252684075d32de7aaf2ca066f12822ce48a5b01d1b0fcf67df146ea792baf1`

## 当前结论

Issue `#92` 已在批准的二十七路径 allowlist 内完成 Domain、Application、Platform、Parity、
稳定项目控制面和本地轻量验证。实际使用二十六条路径，`err.md` 因没有新可复用根因而保持不变。

当前只允许普通提交、普通推送、创建非 Draft PR 和进入 Review/Hosted CI。最终 Squash Merge
仍未授权，动态 Head、Run、Artifact、Review 对话和合并证据只保留在 GitHub。

## 根因与迁移结果

### 上游问题

- `SettingsStore::load` 把文件不存在静默转换为默认设置。
- 损坏 JSON 使用 `unwrap_or_default()` 静默转换为默认设置。
- `load_settings` 返回完整设置、私人路径和用户脚本清单。
- 读取与保存、重置、皮肤、Provider、Relay 和脚本副作用耦合。

### 新能力

- 新增 `feature.foundation-platform.settings-observation`，只接管
  `tauri-command:load_settings`。
- 请求为零字段，不接受任意路径；文件位置只来自 `SystemPlatformPaths`。
- 文件不存在返回 `LoadCompletion::Empty`，稳定语义为 `NotConfigured`。
- 合法 JSON object 返回 `LoadCompletion::Ready` 和 `top_level_entry_count`；`{}` 为
  `Ready(0)`，不得冒充未配置。
- 不返回字段名称、字段值、原始 JSON、脚本、凭据或实际路径。
- `core-module:settings`、`save_settings`、`reset_settings` 与原
  `feature.foundation-platform.settings-management` 继续保持 `unassessed`。

## 分层实施

### Domain

- 新增私有字段 `SettingsDocumentObservation::top_level_entry_count`、构造函数和只读 getter。
- 零条目与非零条目均为合法事实；`Debug` 不包含设置内容。
- RED→GREEN checkpoint：`c752d54506e9307528324c3ce5c2ccecfe23f9c7`。

### Application

- 新增 `SettingsObservationRequest`、`SettingsObservationPort` 与 `ObserveSettings<P>`。
- 固定 `Some → Ready`、`None → Empty`、错误 `→ Failed`；`Some(count=0)` 仍为 Ready。
- 取消、超时、旧结果失效和请求身份继续复用 `LoadCoordinator`。
- RED→GREEN checkpoint：`10cb5fd0f9f0093b7b9f07e5537e15bb8a84b822`。

### Platform

- 新增 `SystemSettingsObservation`，支持平台只通过 `SystemPlatformPaths.resolve` 定位文件。
- 使用 `fs::symlink_metadata` 拒绝符号链接和非普通文件。
- 元数据上限为 `256 KiB`，实际读取使用 `File::open` 与
  `Read::take(256 KiB + 1)` 二次门禁。
- 使用 `serde_json::from_slice::<Value>`；只接受 object 根并只保留 `object.len()`。
- 文件探针与任意路径入口保持模块私有；未新增网络、写入、线程、Watcher、子进程、UI 或
  `unsafe`。
- RED→GREEN checkpoint：`f77d7edd8a00ec6d40a62808fc50a75d1ec70df4`。

### Parity

- 新增设置观察 feature 与无 fixture 行为合同。
- `load_settings` 副作用改为 `[filesystem-read]` 并映射到新子能力。
- 原设置管理保留三个入口和 `[filesystem-read, filesystem-write]`，状态仍为 `unassessed`。
- feature/contract 为 `39/39`，source 为 `133`，fixture manifest 为 `11`。
- RED→GREEN checkpoint：`7e55bd038d7b0e5e7dbbb12a781df510c920aed6`。

## 稳定错误

| 场景 | `ErrorKind` | `DiagnosticCode` |
| --- | --- | --- |
| 非支持平台 | `Unsupported` | `SETTINGS_OBSERVATION_UNSUPPORTED` |
| 文件不可读取 | `Unavailable` | `SETTINGS_OBSERVATION_UNAVAILABLE` |
| 符号链接或非普通文件 | `InvalidInput` | `SETTINGS_OBSERVATION_INVALID_FILE_TYPE` |
| 超过 `256 KiB` | `InvalidInput` | `SETTINGS_OBSERVATION_TOO_LARGE` |
| JSON 损坏或非 UTF-8 | `InvalidInput` | `SETTINGS_OBSERVATION_INVALID_JSON` |
| JSON 根不是 object | `InvalidInput` | `SETTINGS_OBSERVATION_INVALID_ROOT` |

错误只公开稳定 kind/code，不包含底层解析消息、私人路径或文件内容。

## 依赖证据

- Workspace 唯一新增直接依赖为 `serde_json = "=1.0.149"`。
- Platform 通过 `serde_json.workspace = true` 复用根依赖。
- 与基线锁文件比较只新增 `serde_json 1.0.149` 和必要锁包 `zmij 1.0.23`；没有包被移除。
- 本地注册表元数据许可证为：`serde_json` 使用 `MIT OR Apache-2.0`，`zmij` 使用 `MIT`。
- 全 Workspace 离线 metadata 会解析未缓存的 `ash`，属于 `err.md` 已记录的离线依赖缓存边界；
  本次复用既有结论，不新增重复排错条目。

## 范围与 checkpoint

- Planning checkpoint：`3848372d5bd75b175901df59a853aa62d03c8e73`。
- Domain checkpoint：`c752d54506e9307528324c3ce5c2ccecfe23f9c7`。
- Application checkpoint：`10cb5fd0f9f0093b7b9f07e5537e15bb8a84b822`。
- Platform checkpoint：`f77d7edd8a00ec6d40a62808fc50a75d1ec70df4`。
- Parity checkpoint：`7e55bd038d7b0e5e7dbbb12a781df510c920aed6`。
- 候选范围：`27` 路径，哈希保持
  `sha256:ca252684075d32de7aaf2ca066f12822ce48a5b01d1b0fcf67df146ea792baf1`。
- 实际范围：`26` 路径；未修改 `err.md`、`upstream/`、Workflow、Ruleset、Release 或 AGOS。

## 本地验证

```yaml
four_crate_tests_all_targets: passed
four_crate_clippy_all_targets: passed
rustfmt: passed
ci_contract: passed
release_audit: current
repository_policy: passed
candidate_scope: passed-27-paths
actual_scope: passed-26-paths
git_diff_check: passed
feature_contract_counts: 39/39
source_entry_count: 133
fixture_manifest_count: 11
dependency_delta: serde_json-1.0.149,zmij-1.0.23
license_check: passed
forbidden_capability_matches: 0
privacy_matches: 0
readme_lines: 100
local_knowledge_lookup: project-native-doc-and-code-query-completed
agos_report_only: bypassed-known-unregistered-contract-err-md-2026-07-21
review_ci: not-reached
squash_merge: not-authorized
```

## 下一门

1. 建立最终本地验证 checkpoint。
2. 普通推送当前分支并创建关联 Issue `#91/#92` 的非 Draft PR。
3. Review 对话逐条确定根因、处理并验证；Hosted CI 与 Performance observation 全绿。
4. 绑定具体 PR 与 Final Head，请求项目所有者单独授权 Squash Merge。
