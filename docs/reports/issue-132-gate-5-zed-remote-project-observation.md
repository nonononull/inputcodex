# Issue #132 Gate 5 Zed 远程项目只读观察报告

## 当前状态

- `state`: `LOCAL_VERIFIED_PR_PENDING`
- `tracking_issue_ref`: `https://github.com/nonononull/inputcodex/issues/132`
- `approved_decision_ref`: `https://github.com/nonononull/inputcodex/issues/131#issuecomment-5152838531`
- `implementation_scope_approval_ref`: `https://github.com/nonononull/inputcodex/issues/132#issuecomment-5152853748`
- `baseline_ref`: `origin/main@da035b3a6e8ddab9b7c6948ef115ed8b561aa1f4`
- `planning_checkpoint_ref`: `ab8473eea13893961ad6cd1884e015928162f40f`
- `domain_checkpoint_ref`: `d5f7b4f`
- `application_checkpoint_ref`: `726d30b`
- `platform_checkpoint_ref`: `2873f94`
- `parity_checkpoint_ref`: `bda6a81`

## 交付能力

- Domain：固定格式的完整 SHA-256 稳定假名、三种 origin、两种选择提示、来源覆盖和有界项目集合；Debug 不记录稳定假名。
- Application：零字段 Request、功能专用 Cancellation、Port、UseCase 与既有 `LoadCoordinator`；Partial 保持 `Ready` 内的 coverage。
- Platform：固定读取 `CODEX_HOME/.codex-global-state.json` 和受控 SQLite `threads.cwd`，不读取 legacy Recent registry。
- Parity：新增 `feature.remote-install.zed-remote-project-observation`，只接管 `tauri-command:list_zed_remote_projects`；旧 core/open/forget 与旧 fixture 保持 `unassessed`。

## 稳定语义

- 输出只含 `zed-remote-project:v1:sha256:<64 lowercase hex>`、origin、`SelectedHostHint / NotObserved` 和来源计数；稳定假名不是匿名标识。
- `SelectedHostHint` 只表示全局状态选择提示，不表示 Zed 运行、项目打开或远端可达。
- SHA-256 输入使用固定域和长度前缀；None 端口与显式 `22` 不同；不同身份摘要碰撞时整体失败。
- 全局状态上限 `1 MiB`；SQLite 最多 `32` 个、单库 `1 GiB`、每库 `80` 个 cwd、单值 `4 KiB`；内部候选 `512`、最终项目 `256`。
- JSON 使用 no-follow 同句柄限读和打开前后身份复验；SQLite 使用 `READ_ONLY | NOFOLLOW | query_only`、`50 ms` busy timeout 与 progress handler。
- 三类来源全部缺失或合法但无项目返回 `Empty`；有项目且无失败返回 `Ready(Complete)`；有项目且存在来源失败返回 `Ready(Partial)`；无项目且存在失败返回 `Failed`。
- 不返回 label、SSH user/host/port、remote path、URL、hostId、时间戳、实际路径、内容、凭据或原始错误；不写入、不联网、不调用子进程、不启动 Zed、不使用 `unsafe`。

## TDD 证据

- Domain RED：`E0432`，只缺目标领域 API；GREEN 专项 `5/5`，全目标测试和 Clippy 通过。
- Application RED：`E0432`，只缺 Request/Cancellation/Port/UseCase；GREEN 专项 `5/5`，全目标测试和 Clippy 通过。
- Platform RED：目标模块与适配器缺失；GREEN 专项 `12/12`，覆盖稳定向量、碰撞、资源、Partial/Failed、取消/超时、WAL 只读和源码禁止能力；全目标测试与 Clippy 通过。
- Parity RED：新 feature 条目缺失；GREEN 目录专项 `32/32`，计数固定为 `135/46/46/13/11/3/0`，Release Audit 为 `current`。

## 依赖与范围

- 唯一新增直接依赖：`sha2 = 0.10.9`。
- 锁文件新增闭包：`sha2`、`digest`、`crypto-common`、`block-buffer`、`generic-array`、`typenum`、`cpufeatures`。
- 新增闭包许可证全部为 `MIT` 或 `MIT OR Apache-2.0`。
- 路径数：`28`。
- Ordinal hash：`sha256:7ee5d47dca72d0d2f1ec683cc45e4bbac0e3ce40af1e57417d39608c8c0c26bb`。
- 不包含 Workflow、Ruleset、Runner、Release、上游快照、UI 或 AGOS 修改。

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
candidate_scope: passed-28-paths-sha256-7ee5d47dca72d0d2f1ec683cc45e4bbac0e3ce40af1e57417d39608c8c0c26bb
dependency_and_license_audit: passed
privacy_and_forbidden_capability_scan: passed-zero-matches
git_diff_check: passed
remote_push: pending
pull_request: pending
hosted_ci: pending-pr
```

## 下一门

Issue #132 二十八路径本地总门禁已经通过；下一门仅为普通 push、创建非 Draft PR。Final Head 必须完成独立只读复审、Review thread、CI、Performance、Artifact 与精确 Head Squash Merge 门禁，动态证据只保留在 GitHub。
