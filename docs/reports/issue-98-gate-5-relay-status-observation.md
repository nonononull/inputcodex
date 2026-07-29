# Issue #98 Gate 5 Relay 认证与配置状态只读观察报告

## 当前状态

- `state`: `IMPLEMENTATION_APPROVED_TDD_IN_PROGRESS`
- `tracking_issue_ref`: `https://github.com/nonononull/inputcodex/issues/98`
- `approved_decision_ref`: `https://github.com/nonononull/inputcodex/issues/97#issuecomment-5115176838`
- `baseline_ref`: `origin/main@b7c4174671caba806162a42e82b7bc0b20f73bf5`
- `branch_ref`: `codex/issue-98-gate-5-relay-status-observation`
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
product_implementation: forbidden
remote_push: forbidden
pull_request: forbidden
hosted_ci: forbidden
```

## 下一门

1. 完成四路径、两个 hash、占位符、CI 合同、仓库政策、Cargo metadata 与 diff 验证。
2. 运行 AGOS ReportOnly，异常或 `needs-input` 按项目规则记录后绕过。
3. 建立本地 planning checkpoint，并把提交证据回写 Issue `#98`。
4. 请求二十七路径独立实施批准；未获批准前不得修改任何产品或依赖文件。
