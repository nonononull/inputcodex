# Issue #128 Gate 5 Watcher 偏好状态只读观察报告

## 当前状态

- `state`: `LOCAL_VERIFIED_PR_PENDING`
- `tracking_issue_ref`: `https://github.com/nonononull/inputcodex/issues/128`
- `approved_decision_ref`: `https://github.com/nonononull/inputcodex/issues/127#issuecomment-5151021881`
- `implementation_scope_approval_ref`: `https://github.com/nonononull/inputcodex/issues/128#issuecomment-5151032720`
- `baseline_ref`: `origin/main@43ace17de1505f251812e4ead3035ef3274a8455`
- `planning_checkpoint_ref`: `a9c6d08c34c08da95389c3b55334855a03693926`
- `domain_checkpoint_ref`: `40fadd148a1966c1fec82e08c4cfafd9e079a45c`
- `application_checkpoint_ref`: `174bc6fc0a6ab0627b97005b814e786fcce0cb47`
- `platform_checkpoint_ref`: `7d9d6dadbd75343c9fa0d0ef83ac59e63735daa5`
- `parity_checkpoint_ref`: `f4bb3ef48207352cdb8bf3ecee8ff0e6a0d1789c`

## 交付能力

- Domain：`WatcherPreference::{EnabledByDefault, ExplicitlyDisabled}` 与单字段脱敏观察值。
- Application：零字段 Request、同步只读 Port 和只产生 `Ready/Failed` 的 UseCase。
- Platform：平台路径定位状态根后，只对固定 `watcher.disabled` 执行一次 `symlink_metadata`。
- Parity：新增 `feature.foundation-platform.watcher-preference-observation`，只接管
  `tauri-command:load_watcher_state`；完整 Watcher 管理继续 `unassessed`。

## 稳定语义

- 标记缺失：`Ready(EnabledByDefault)`，不是 `Empty`，不声明 Watcher 正在运行。
- 普通文件：`Ready(ExplicitlyDisabled)`，不读取文件内容。
- symlink、目录或其他类型：`WATCHER_PREFERENCE_INVALID`。
- NotFound 之外的元数据 I/O 错误：`WATCHER_PREFERENCE_UNREADABLE`。
- 非 Windows/macOS：`WATCHER_PREFERENCE_UNSUPPORTED`。
- 不返回实际路径、文件内容、进程、debug port、安装状态或运行状态。
- 不写文件、不联网、不调用子进程、不启动线程或真实 Watcher，不新增依赖。

## TDD 证据

- Domain RED：`E0432`，只缺 `WatcherPreference` 与 `WatcherPreferenceObservation`；GREEN 目标测试 `2/2`。
- Application RED：`E0432`，只缺 Request、Port 与 UseCase；GREEN 目标测试 `3/3`。
- Platform RED：私有矩阵只缺 Probe/分类/helper，公开测试只缺 System adapter；GREEN 私有 `4/4`、公开 `1/1`。
- Parity RED：新 feature 条目缺失；第二轮 RED 暴露 YAML ID 前缀误匹配，收紧为完整 ID 行后专项 GREEN。
- Parity all-targets：目录测试 `31/31`；计数固定为 `135/45/45/12/11/3/0`。

## 范围冻结

- 路径数：`24`
- Ordinal hash：`sha256:0be3dd45ed7e91d1cb7f633da369bb2b9052cefc512852d80362775da7e81699`
- 不包含 Cargo、Workflow、Ruleset、Runner、Release、上游快照、UI 或 AGOS 写入。

## 本地门禁

```yaml
four_crate_tests_all_targets: passed
four_crate_clippy_all_targets: passed
rustfmt: passed
ci_script_contract: passed-76
autonomous_policy: passed-sha256-3531b5dafe6f396fa986928dcdc16c0dc20a03678c662b7aa5df39bc9f1cd5d6
release_audit: current
repository_policy: passed-zero-violations
cargo_metadata: passed-locked-offline-no-deps
candidate_scope: passed-24-paths-sha256-0be3dd45ed7e91d1cb7f633da369bb2b9052cefc512852d80362775da7e81699
privacy_and_forbidden_capability_scan: passed-zero-matches
symlink_metadata_system_call_count: passed-one
git_diff_check: passed
remote_push: pending
pull_request: pending
hosted_ci: pending-pr
```

## 下一门

本地门禁通过后建立 `issue-128-local-verified` checkpoint，再普通 push、创建关联 Issue `#128` 的
非 Draft PR。Final Head 必须完成独立只读复审、Review thread、CI、Performance、Artifact 与精确
Head Squash Merge 门禁；动态证据只保留在 GitHub。
