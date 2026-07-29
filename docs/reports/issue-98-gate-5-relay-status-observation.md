# Issue #98 Gate 5 Relay 认证与配置状态只读观察报告

## 当前状态

- `state`: `LOCAL_VERIFIED_PR_PENDING`
- `tracking_issue_ref`: `https://github.com/nonononull/inputcodex/issues/98`
- `approved_decision_ref`: `https://github.com/nonononull/inputcodex/issues/97#issuecomment-5115176838`
- `baseline_ref`: `origin/main@b7c4174671caba806162a42e82b7bc0b20f73bf5`
- `branch_ref`: `codex/issue-98-gate-5-relay-status-observation`
- `planning_checkpoint_ref`: `a6b3cdc5f88cafaf1624da34793bb133d8d05300`
- `domain_checkpoint_ref`: `01d94aba6a65544d42d1da9d18b3affb0afb3227`
- `application_checkpoint_ref`: `b0bdf4eac7c0ee19a3f85b7ae2ba35e4683b7247`
- `platform_checkpoint_ref`: `f6eb1d3ce5f934a09878fe83b7945df2730b3f2e`
- `parity_checkpoint_ref`: `f21d04225e131eb0ed2bbe24cc7827046ceaef1f`
- `implementation_started`: `true`
- `remote_delivery_started`: `false`

## Discovery 证据

### 上游事实

- `relay_status_from_home` 聚合认证、账号、路径与配置布尔值。
- `chatgpt_auth_status_from_home` 会把读取失败、损坏和凭据缺失折叠为未认证。
- `relay_config_status_from_home` 使用 `unwrap_or_default()` 读取两份文档，损坏语义不可见。
- `configured` 依赖 Provider、认证要求、Bearer Token/API Key 和 Base URL，却只返回单一布尔值。
- 原 Relay 总功能还包含完整读取、保存、切换、回填与网络相关副作用。

### 本项目复用点

- `SystemPlatformPaths` 已提供 Windows/macOS 一致的 `codex_home()` 定位。
- 前序设置与诊断观察已验证固定路径、模块私有 probe、有界读取和 LoadCompletion 映射。
- Workspace 已固定 `serde_json`；锁定依赖源码已有 `toml_edit 0.25.13+spec-1.1.0`。
- 当前分支从第七个 Gate 5 切片合并后的权威主干建立。

## 设计结果

### 新能力

- `feature.provider-network.relay-status-observation`
- 唯一接管 `tauri-command:relay_status`
- 固定读取 `CODEX_HOME/auth.json` 与 `CODEX_HOME/config.toml`
- 单文件 `256 KiB`，总读取预算 `512 KiB`

### 返回事实

- `auth_document_status`
- `config_document_status`
- `chatgpt_credentials`
- `openai_api_key`
- `relay_configuration`

返回值只使用封闭枚举，不携带字符串、路径、字段、内容、账号、Token、Provider ID 或 Base URL。

### 完成态

- 两文件均缺失：`LoadCompletion::Empty`
- 至少一份存在：`LoadCompletion::Ready`
- 平台不支持或平台路径解析失败：`LoadCompletion::Failed`
- 单文件损坏、超限或不可读：保留在 Ready 载荷的文档状态中，不覆盖另一文件事实

### 副作用与隐私

只允许固定文件有界读取。禁止写入、网络、环境修改、子进程、线程/Watcher、UI、注入、`unsafe`、
账号标签、Token、Provider ID、Base URL、字段名/内容、认证来源和实际路径披露。

## 范围冻结

### Planning

- 路径数：`4`
- 哈希：`sha256:ec6c88d4a96c351fee85d6c416b04c95b27050893ccbe55b4ad55edfd8d95051`

### Candidate Implementation

- 路径数：`27`
- 哈希：`sha256:b1dda60cda57d4be9344b3fa0c74a49b6087b9bdf03fceb5a772ec7e893d63a5`
- 新增直接依赖只允许 `toml_edit` 的已锁定精确版本与 parse feature。
- 不包含 UI、Workflow、Ruleset、Release、`upstream/` 或 AGOS 写入。

## 当前验证证据

```yaml
planning_files_written: passed-4-paths
planning_allowlist: passed-exact
planning_scope_hash: verified-independent-recalculation
candidate_scope_hash: verified-independent-recalculation
ci_script_contract: passed-35
repository_policy: passed-zero-violations
cargo_metadata: passed-locked-offline-no-deps
document_encoding: passed-utf8-no-bom-lf
placeholder_scan: passed
trailing_whitespace_scan: passed
git_diff_check: passed
local_knowledge_lookup: completed-no-codegraph-gbrain-no-results
agos_report_only: needs-input-unregistered-bypassed
agos_repository_mutation: none
apply_patch_recovery: reused-existing-err-md-native-codex-entry
product_implementation: completed-within-approved-scope
domain_tdd: passed-red-green
application_tdd: passed-red-green
platform_tdd: passed-red-green
platform_tests_all_targets: passed
platform_clippy_all_targets: passed
parity_tdd: passed-red-green
parity_tests_all_targets: passed
parity_clippy_all_targets: passed
release_audit: current
toml_edit_lock: 0.25.13+spec-1.1.0
runtime_release_audit_path_fix: recorded-in-err-md
four_crate_tests_all_targets: passed
four_crate_clippy_all_targets: passed
rustfmt: passed
ci_script_contract: passed-35
repository_policy: passed-zero-violations
cargo_metadata: passed-locked-offline-no-deps
candidate_scope: passed-27-paths
actual_scope: passed-27-paths
actual_scope_hash: sha256:b1dda60cda57d4be9344b3fa0c74a49b6087b9bdf03fceb5a772ec7e893d63a5
feature_contract_counts: 41/41
source_entry_count: 133
fixture_manifest_count: 11
privacy_matches: 0
forbidden_capability_matches: 0
git_diff_check: passed
remote_push: pending
pull_request: pending
hosted_ci: pending-pr
```

## 下一门

1. 按 `build.md` 完成四 crate、格式、CI 合同、Release Audit、仓库政策、Cargo metadata、范围、隐私和禁止能力门禁。
2. 把任务状态更新为 `LOCAL_VERIFIED_PR_PENDING`，建立最终本地 checkpoint 并普通 push。
3. 创建关联 Issue `#98` 的非 Draft PR，进入 Review/CI 与 Artifact 核验。
4. Final Head 全绿后请求独立 Squash Merge 授权；当前禁止自行合并。
