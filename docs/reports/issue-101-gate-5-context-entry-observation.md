# Issue #101 Gate 5 上下文能力只读目录观察报告

## 当前状态

- `state`: `LOCAL_VERIFIED_PR_PENDING`
- `tracking_issue_ref`: `https://github.com/nonononull/inputcodex/issues/101`
- `approved_decision_ref`: `https://github.com/nonononull/inputcodex/issues/100#issuecomment-5118455158`
- `implementation_scope_approval_ref`: `https://github.com/nonononull/inputcodex/issues/101#issuecomment-5119498422`
- `baseline_ref`: `origin/main@52320c2c02e19d9ffae11ccb6742a0f0fc4b71b9`
- `branch_ref`: `codex/issue-101-gate-5-context-entry-observation`
- `planning_checkpoint_ref`: `f72ec09680980b396897f9363f3fc79c4b179d32`
- `domain_checkpoint_ref`: `325b86a94c19c0eb494ce724e438187fc3bd97b3`
- `application_checkpoint_ref`: `bbb9b88a58ba08e4d7184c6d3b1a93229307f7ed`
- `platform_checkpoint_ref`: `273f3234e36e35e7c09e00a2b9e0ff5ec81564ab`
- `parity_checkpoint_ref`: `40a9dc15f19f80a576622178a05c6511534eae7b`
- `remote_delivery_started`: `false`

## 设计结果

### 新能力

- `feature.provider-network.context-entry-observation`
- 唯一接管 `tauri-command:read_live_context_entries`
- 固定观察平台路径解析的 `CODEX_HOME/config.toml`
- 单文件硬上限 `256 KiB`

### 返回事实

- 条目 ID
- 稳定种类：`McpServer / Skill / Plugin`
- 启用状态
- 三类各自的 `total / enabled / disabled` 计数

分类计数只能由有序条目集合派生，调用方不能注入与条目不一致的摘要。

### 完成态

- 固定文件不存在：`LoadCompletion::Empty`
- 文件存在且合法但零条目：`LoadCompletion::Ready`，总数为 `0`
- 文件存在且包含合法条目：`LoadCompletion::Ready`，保留跨根表原文顺序
- 文件类型、大小、I/O、UTF-8、TOML 或结构错误：`LoadCompletion::Failed`，只携带稳定诊断码

### 启用语义

- `enabled = false`：禁用
- `disabled = true`：禁用
- 两字段均缺失：启用
- 任一字段存在但不是布尔值：整个配置失败
- `enabled=false` 或 `disabled=true` 任一成立即禁用

### 文件与隐私边界

平台层只通过 `SystemPlatformPaths` 定位一个固定文件，使用 `symlink_metadata` 拒绝符号链接和
非普通文件，对元数据及实际读取执行双重 `256 KiB` 上限，并用 `limit + 1` 检测增长越界。

返回值不得包含原始 TOML、摘要、命令、参数、环境变量、Header、URL、Token、账号、实际路径、
用户名、机器名、内容片段或可逆派生值。实现不写文件、不联网、不调用子进程、不启动线程或
Watcher、不打开 UI、不注入、不使用 `unsafe`，也不新增依赖。

## 原文顺序根因

`toml_edit::DocumentMut::from_str` 会在 `Document::into_mut()` 中执行 `despan()`，清除原文 span；
直接按三个根表迭代会把跨表条目错误聚合。本实现先用同一 TOML 解析器的
`Document<String>` 采集子 Item 起始偏移，再转换为 `DocumentMut` 做结构验证和最小领域投影，
最后按原文偏移排序。该根因和恢复方式已记录到 `err.md`，没有引入手写 TOML 解析器，也不保存
或返回正文。

## Parity 结果

- 新功能状态：`implemented`
- 新入口副作用：`[filesystem-read]`
- 原 `feature.provider-network.context-entry-management`：继续 `unassessed`
- 五个上下文增加、删除、同步、提取和完整管理入口：继续归原总功能
- feature 数：`42`
- contract 数：`42`
- source entry 数：`133`
- fixture manifest 数：`11`

## 范围冻结

- 批准路径数：`24`
- 批准哈希：`sha256:5b96235eb1fa7832e5710f7343917a5c2512bc50a46198ed584323366dd34372`
- 实际范围：包含 `err.md` 的完整二十四路径
- 新依赖：`none`
- UI、Workflow、Ruleset、Release、`upstream/`、AGOS 修改：`none`

## 当前验证证据

```yaml
planning_validation: passed
implementation_scope_approval: passed
domain_tdd: passed-red-green
application_tdd: passed-red-green
platform_tdd: passed-red-green
parity_tdd: passed-red-green
parity_tests_all_targets: passed
parity_clippy_all_targets: passed
release_audit: current
documentmut_span_root_cause: recorded-in-err-md
four_crate_tests_all_targets: passed
four_crate_clippy_all_targets: passed
rustfmt: passed
ci_script_contract: passed
repository_policy: passed-zero-violations
cargo_metadata: passed-locked-offline-no-deps
candidate_scope: passed-24-paths
actual_scope: passed-24-paths
actual_scope_hash: sha256:5b96235eb1fa7832e5710f7343917a5c2512bc50a46198ed584323366dd34372
privacy_matches: 0
forbidden_capability_matches: 0
git_diff_check: passed
remote_push: pending
pull_request: pending
hosted_ci: pending-pr
```

## 下一门

1. 建立 `issue-101-local-verified` checkpoint 并普通 push。
2. 创建关联 Issue `#101` 的非 Draft PR，进入 Review/CI 与 Artifact 核验。
3. Final Head 全绿后请求独立 Squash Merge 授权；当前禁止自行合并。
