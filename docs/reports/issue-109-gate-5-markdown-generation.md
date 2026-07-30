# Issue #109 Gate 5 受控会话 Markdown 生成报告

## 当前状态

- `state`: `LOCAL_VERIFIED_PR_PENDING`
- `tracking_issue_ref`: `https://github.com/nonononull/inputcodex/issues/109`
- `approved_decision_ref`: `https://github.com/nonononull/inputcodex/issues/108#issuecomment-5130112866`
- `implementation_scope_approval_ref`: `https://github.com/nonononull/inputcodex/issues/109#issuecomment-5130373022`
- `baseline_ref`: `origin/main@88eaf6301f1897cadaf4da830db998078fb06e97`
- `branch_ref`: `codex/issue-109-gate-5-markdown-generation`
- `planning_checkpoint_ref`: `bc05e31ef3b4d9fda79fbd4170cae5bd52f24db4`
- `domain_checkpoint_ref`: `16d0816`
- `application_checkpoint_ref`: `60da5b1`
- `platform_checkpoint_ref`: `b4a66b5`
- `parity_checkpoint_ref`: `338359f`
- `local_verified_checkpoint_ref`: `pending-self-checkpoint`
- `remote_delivery_started`: `false`

## 交付结果

### 新能力

- `feature.session-data.markdown-generation`
- 输入只接受既有本地会话目录中的 `session_id`
- 输出只包含有界建议文件名、内存 Markdown 与消息计数
- `LoadCompletion::Ready / Empty / Failed` 与取消、旧结果隔离使用项目既有协调语义

本切片不保存文件。保存对话框、覆盖确认、取消保存、原子写入和展示层交互继续后置独立评审；
上游 `saveMarkdown` 仍归 `feature.plugin-script.renderer-enhancements=exception-pending`。

### 确定性 Markdown

- 标题缺失时使用稳定英文回退 `Untitled session`。
- 建议文件名固定为 `session-<clean-title>.md`，最大 `160 UTF-8 bytes`，不含完整会话 ID。
- 只保留 `user / assistant` 的 `input_text / output_text`。
- 图片统一为 `> Image attachment omitted`，不返回 data URL、远程 URL 或字节。
- 合法 RFC 3339 时间戳规范化为 UTC；非法或缺失时间戳省略，不读取本机时区。
- 换行固定 LF 且文档固定以换行结尾；Windows 与 macOS 使用相同 Domain/Application 合同。

## 数据源与资源边界

- SQLite 候选最多 `32`，复用平台 `CODEX_HOME` 与合法非空 `CODEX_SQLITE_HOME` 语义。
- 连接只使用 `SQLITE_OPEN_READ_ONLY | SQLITE_OPEN_NO_MUTEX`、`query_only=1`、`50 ms` busy
  timeout 和 `1000` progress interval；SQL 只由固定白名单表达式与参数 `?1` 组成。
- 任一候选数据库损坏、schema 不支持或查询失败都会阻断生成，较旧重复记录不得冒充权威结果。
- rollout 只允许位于普通目录 `CODEX_HOME/sessions` 或 `archived_sessions`；相对路径、父目录、
  根越界、符号链接、非普通文件、非 JSONL 和会话元数据不匹配均失败。
- rollout 深度 `4`、枚举项 `8192`、候选 `4096`、metadata 前缀 `8 KiB`、发现累计
  `32 MiB`、单文件 `16 MiB`、非空记录 `100000`、消息 `20000`、Markdown `16 MiB`。
- 整体 deadline 为 `2 s`；取消与超时均使用稳定脱敏错误，JSONL 全程经 `BufReader` 流式解析。

## TDD 证据

- Domain RED：公开 API 缺失，`E0432`；GREEN：专项 `7 passed`。
- Application RED：Request、Cancellation、Port 与 UseCase 缺失，`E0432`；GREEN：专项
  `6 passed`。
- Platform RED：目标模块文件缺失；首轮 GREEN `15 passed`。安全复核再以三条失败测试证明并修复
  CODEX_HOME 符号链接、显式 rollout 会话错配和非法 UTF-8 分类，最终专项 `17 passed`。
- Parity RED：缺少 `feature.session-data.markdown-generation`；GREEN：目录测试通过，总数保持
  `43 features / 43 contracts / 12 fixtures`。

## 安全审查

- 无新依赖、Cargo 或锁文件改动；继续使用已锁定 `rusqlite 0.40.1` 与 `serde_json 1.0.149`。
- 请求不能携带 SQLite、rollout、输出路径或 Markdown 正文；会话 ID 不拼接进 SQL。
- 生产代码不包含普通 `Connection::open`、整文件字符串读取、文件写入、网络、子进程、线程、
  Watcher、Iced、注入或 `unsafe`。
- 错误与自定义 `Debug` 不包含会话 ID、标题、正文、URL、建议文件名、数据库名、实际路径或底层
  rusqlite、I/O、JSON 错误文本。
- 测试全部使用现场合成 SQLite/JSONL；未读取、复制或提交真实用户会话。
- WAL 验证证明主数据库与 WAL 字节、目录清单不变；SHM 只允许既有 reader 协调语义。

## 范围冻结

- 候选路径：`29`
- 候选哈希：`sha256:b113da5d41514f50e36cef7d4eb9ade89e2562cbcfe8d392a5173d38fd0ebaac`
- 实际路径：无新根因，排除 `err.md` 后精确 `28`
- 实际哈希：`sha256:3d19b05918c8294f050f3f0c83f9118453705afd8aad016827fafb477e74a50b`
- UI、Workflow、Ruleset、Release、`upstream/`、`benchmarks/`、Cargo 与 AGOS 修改：`none`

## 当前验证证据

```yaml
planning_validation: passed
implementation_scope_approval: passed
domain_tdd: passed-red-green
application_tdd: passed-red-green
platform_tdd: passed-red-green-17-tests
parity_tdd: passed-red-green
four_crate_tests_all_targets: passed
four_crate_clippy_all_targets: passed
rustfmt: passed
ci_script_contract: passed-35-of-35
repository_policy: passed-zero-violations
release_audit: current
cargo_metadata: passed-locked-offline-no-deps
readonly_security_gate: passed
wal_business_readonly_evidence: passed
security_review: passed-fixed-sql-readonly-bounded-redacted
actual_scope: passed-28-paths-without-err-md
actual_scope_hash: sha256:3d19b05918c8294f050f3f0c83f9118453705afd8aad016827fafb477e74a50b
privacy_matches: 0
forbidden_capability_matches: 0
git_diff_check: passed
knowledge_graph: codegraph-not-initialized-skipped-without-init
agos_default_entry: blocked-needs-input-unregistered-bypassed-no-write
remote_push: pending
pull_request: pending
hosted_ci: pending-pr
```

## 下一门

1. 建立 `issue-109-local-verified` checkpoint 并普通 push。
2. 创建关联 Issue `#109` 的非 Draft PR，进入 Review/CI、Performance Baseline 与 Artifact 核验。
3. Final Head 全绿后请求项目所有者独立 Squash Merge 授权；当前禁止自行合并。
