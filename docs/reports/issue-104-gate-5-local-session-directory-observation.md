# Issue #104 Gate 5 本地会话目录只读观察报告

## 当前状态

- `state`: `LOCAL_VERIFIED_PR_PENDING`
- `tracking_issue_ref`: `https://github.com/nonononull/inputcodex/issues/104`
- `approved_decision_ref`: `https://github.com/nonononull/inputcodex/issues/103#issuecomment-5121451656`
- `implementation_scope_approval_ref`: `https://github.com/nonononull/inputcodex/issues/104#issuecomment-5121508761`
- `baseline_ref`: `origin/main@4032b051f0f18be71d344eded2d6e79595233b65`
- `branch_ref`: `codex/issue-104-gate-5-local-session-directory-observation`
- `planning_checkpoint_ref`: `7d11a6ff904b7d9bc2ca74bbaf52c122fc31feb9`
- `domain_checkpoint_ref`: `c684f4bc115ef8d7b406a343876419edb8f2d290`
- `application_checkpoint_ref`: `bd03cd3a8ba8b6df222c191d75bd672881c32d6a`
- `platform_checkpoint_ref`: `129e7d57f644ba32fdd46fb669f3e86774c8ae9a`
- `parity_checkpoint_ref`: `8896eb5174a7e37539f35c6ac5d1ef09f93ae9e8`
- `local_verified_checkpoint_ref`: `pending-self-checkpoint`
- `remote_delivery_started`: `false`

## 交付结果

### 新能力

- `feature.session-data.local-session-directory-observation`
- 唯一接管 `tauri-command:list_local_sessions`
- 默认分页 `offset=0 / limit=50`，单页最大 `100`
- 最多观察 `32` 个受控直接普通 SQLite 候选

原 `feature.session-data.local-session-management` 继续 `unassessed`，保留多数据库删除、备份、恢复、
schema 修复和 grouped undo；本任务没有迁移这些写入能力。

### 返回事实

每条记录只包含：

- `session_id`
- `display_title: Option<String>`
- `title_truncated`
- `archived`
- `updated_at_ms: Option<i64>`

标题会规范化空白、移除控制字符并限制为 `256` 个 Unicode 字符。结果按更新时间倒序、缺失时间
靠后、相同时间按会话 ID 倒序；跨库合并、排序、去重后分页，完全相同时当前数据库优先。

### 完成态与来源覆盖

- 无来源或全部可读来源零条目：`LoadCompletion::Empty`
- 全部来源成功且有条目：`LoadCompletion::Ready / Complete`
- 至少一个来源产生条目且其他来源失败：`LoadCompletion::Ready / Partial`
- 可读空来源与失败来源并存，或所有来源失败：`LoadCompletion::Failed`
- 超时与取消分别使用稳定 `Timeout` 与 `Cancelled` 错误，旧请求结果不得发布

## SQLite 与安全边界

- 默认根来自 `SystemPlatformPaths` 解析的 `CODEX_HOME`；合法非空 `CODEX_SQLITE_HOME` 可覆盖，
  非法显式值明确失败且不回退。
- 只扫描 `<root>/sqlite/` 直接普通 `.db/.sqlite/.sqlite3` 文件和 `<root>/state_5.sqlite`；不递归、
  不跟随符号链接，不接受调用方任意路径。
- `rusqlite 0.40.1` 只启用 `bundled/hooks`，连接使用 `SQLITE_OPEN_READ_ONLY`、`query_only`、短
  busy timeout 和 progress handler；SQL 只由固定模板、白名单表列和参数组成。
- 主数据库与 WAL 字节、目录清单保持不变；SHM 长度保持不变，仅允许 SQLite WAL reader
  协调区字节变化，不产生业务数据写入。
- 错误和 `Debug` 不返回数据库路径/文件名、标题、会话 ID、Provider、正文、模型、账号、凭据
  或 rusqlite 原始错误文本。
- 不联网、不调用子进程、不启动线程或 Watcher、不打开 UI、不注入、不使用 `unsafe`。

## TDD 与排错证据

- Domain RED：缺少领域 API，`E0432 / 101`；GREEN：定向 `7 passed`。
- Application RED：缺少 Request、Cancellation、Port 与 UseCase，`E0432 / 101`；GREEN：定向
  `8 passed`。
- Platform RED：缺少实现文件，退出码 `101`；GREEN：合成 SQLite 定向 `15 passed`。
- Parity RED：缺少独立 observation feature，退出码 `101`；GREEN：仓库目录 `22 passed`，总计
  `43` 个 feature、`43` 份合同和 `12` 个 fixture manifest。
- `automation_runs` 的单行时间回退使用 `COALESCE(updated_at, created_at)`。
- `cargo tree` 使用 `--prefix none` 后做精确 `rusqlite v0.40.1` 断言。
- WAL reader 的 SHM 协调区语义与 `cargo tree` 前缀误报均已作为可复用根因记录到 `err.md`。

## 范围冻结

- 批准路径数：`29`
- 批准哈希：`sha256:47dcb2c181daa61a8df073e7f3ada069bf8e3d9b95df0c57f7709bcb6cde211d`
- 实际范围：包含 `err.md` 的完整二十九路径
- 新依赖：`rusqlite 0.40.1`、`libsqlite3-sys 0.38.1` 及其锁定的必要传递依赖
- UI、Workflow、Ruleset、Release、`upstream/`、`benchmarks/`、AGOS 修改：`none`

## 安全与许可证审查

- 新增直接/必要传递依赖 `rusqlite`、`libsqlite3-sys`、`fallible-iterator`、
  `fallible-streaming-iterator` 与 `vcpkg` 的 SPDX 均为 MIT 或 MIT/Apache-2.0。
- 输入只允许有界分页；路径只来自平台 `CODEX_HOME` 与严格环境覆盖，SQL 只拼装白名单固定表达式，
  行数使用参数 `?1`，没有用户值拼接。
- fresh `cargo audit` 命中 `origin/main` 已存在的 `quick-xml 0.39.4` 两条高危公告；该依赖经
  Iced/Wayland 展示链进入，Issue #104 的 Cargo.lock 新增集合不含它。风险已隔离登记 Issue `#105`，
  当前 PR 不越界升级展示依赖。
- `paste 1.0.15` 与 `ttf-parser 0.25.1` 的未维护警告同样属于既有 Iced/wgpu 依赖树，随 Issue
  `#105` 评估，不与本次新增依赖混写。

## 当前验证证据

```yaml
planning_validation: passed
implementation_scope_approval: passed
domain_tdd: passed-red-green
application_tdd: passed-red-green
platform_tdd: passed-red-green
parity_tdd: passed-red-green
platform_tests_all_targets: passed
platform_clippy_all_targets: passed
parity_tests_all_targets: passed
parity_clippy_all_targets: passed
release_audit: current
repository_policy: passed-zero-violations
dependency_tree: passed-rusqlite-0.40.1-only
readonly_security_gate: passed
wal_business_readonly_evidence: passed
four_crate_tests_all_targets: passed
four_crate_clippy_all_targets: passed
rustfmt: passed
ci_script_contract: passed-35-of-35
cargo_metadata: passed-locked-offline-no-deps
dependency_license_review: passed-mit-or-mit-apache-2.0
security_review: passed-fixed-path-parameterized-readonly-redacted
cargo_audit: baseline-advisories-tracked-in-issue-105
actual_scope: passed-29-paths
actual_scope_hash: sha256:47dcb2c181daa61a8df073e7f3ada069bf8e3d9b95df0c57f7709bcb6cde211d
privacy_matches: 0
forbidden_capability_matches: 0
git_diff_check: passed
knowledge_graph: codegraph-not-initialized-skipped-without-init
agos_workflow_rollout: dry-run-blocked-unregistered-bypassed-no-write
remote_push: pending
pull_request: pending
hosted_ci: pending-pr
```

## 下一门

1. 建立 `issue-104-local-verified` checkpoint 并普通 push。
2. 创建关联 Issue `#104` 的非 Draft PR，进入 Review/CI 与 Artifact 核验。
3. Final Head 全绿后请求独立 Squash Merge 授权；当前禁止自行合并。
