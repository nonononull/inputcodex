# Issue #104 Gate 5 本地会话目录只读观察实施计划

> 当前活跃计划使用项目原生控制面；本计划继承 `superpowers:writing-plans` 的 TDD、精确文件和可重复验证纪律，但不创建新的 `docs/superpowers/*` 活跃计划。

**目标：** 从上游本地会话管理总功能中拆出安全、可分页、可取消、严格只读且最小披露的本地会话清单快照。

**架构：** Domain 固定条目与覆盖语义，Application 固定请求与完成态，Platform 负责严格路径和只读 SQLite，多库排序/去重后投影最小字段，Parity 记录新子能力及合成 fixture。原删除、备份、恢复和 grouped undo 继续未评估。

**技术栈：** Rust 1.97.1、`rusqlite = 0.40.1`（`bundled`、`hooks`）、现有七成员 Workspace、GitHub-hosted Windows/macOS/Linux CI。

## 任务元数据

- `tracking_issue_ref`: `https://github.com/nonononull/inputcodex/issues/104`
- `approved_decision_ref`: `https://github.com/nonononull/inputcodex/issues/103#issuecomment-5121451656`
- `implementation_scope_approval_ref`: `https://github.com/nonononull/inputcodex/issues/104#issuecomment-5121508761`
- `baseline_ref`: `origin/main@4032b051f0f18be71d344eded2d6e79595233b65`
- `branch_ref`: `codex/issue-104-gate-5-local-session-directory-observation`
- `worktree_ref`: `.worktrees/issue-104-gate-5-local-session-directory-observation`
- `planning_scope_hash`: `sha256:8b9295414fde23c7ea9c9c53a47acfb7a70b1f1bcd3255f81cb011ba5b624a51`
- `candidate_scope_hash`: `sha256:47dcb2c181daa61a8df073e7f3ada069bf8e3d9b95df0c57f7709bcb6cde211d`
- `normal_scope_without_err`: `sha256:acee55e9539631f6eca4fb557d27999c7815d20ae15a4b4ea932db243079cb8c`
- `work_class`: `standard`
- `delivery_contract`: `issue-pr-squash-merge`

## 全局约束

- 只接管 `tauri-command:list_local_sessions` 的安全只读事实。
- 新能力固定为 `feature.session-data.local-session-directory-observation`。
- 原 `feature.session-data.local-session-management` 继续 `unassessed`。
- 不返回 `cwd`、Provider、Rollout 路径、数据库路径/文件名、正文、Prompt、Token、模型、账号或凭据。
- 不接受调用方传入数据库路径，不递归、不跟随符号链接，候选最多 `32`。
- SQLite 必须只读打开并启用 `query_only`；禁止建库、修复、迁移、索引、删除、备份、恢复或其他写入。
- Windows 与 macOS 共享相同领域、分页、排序、去重、超时、取消和错误合同。
- 不创建 UI，不修改 Iced、Desktop、CI、Ruleset、上游缓存、性能预算或 AGOS。

## 已批准语义

### 条目投影

每条记录只包含：

- `session_id`
- `display_title: Option<String>`
- `title_truncated`
- `archived`
- `updated_at_ms: Option<i64>`

空白标题转为缺失；空白规范化并移除控制字符；最多保留 `256` 个 Unicode 字符。条目使用脱敏 `Debug`，禁止标题和 ID 进入诊断。

### 分页与顺序

- 默认 `limit=50`，最大 `100`。
- `limit=0`、`limit>100` 或 `offset + limit + 1` 溢出明确返回 `LOCAL_SESSION_DIRECTORY_INVALID_PAGINATION`。
- 跨库合并、排序、去重后再分页。
- 更新时间倒序，缺失时间靠后；相同时间按会话 ID 降序保证稳定。
- 相同 ID 保留最新条目；完全相同时按来源优先级保留当前数据库记录。

### 来源覆盖

- `Complete`：所有已发现来源均成功读取。
- `Partial`：至少一个来源成功产生条目，同时其他来源失败。
- 无来源或全部可读来源零条目：`Empty`。
- 可读空来源与失败来源并存：`Failed`，不得伪造完整空结果。
- 所有来源失败：`Failed`。

### 路径与 SQLite

- 默认复用 `SystemPlatformPaths` 得到 `CODEX_HOME`。
- 合法非空 `CODEX_SQLITE_HOME` 覆盖 SQLite 根；非法显式值失败且不得回退。
- 扫描根目录下 `sqlite/` 的直接普通 SQLite 候选及 `state_5.sqlite`。
- 使用 `rusqlite 0.40.1` 的 `bundled` 与 `hooks`，只读 OpenFlags、`PRAGMA query_only`、短 busy timeout 和 progress handler。
- progress handler 同时检查取消标记与整体 deadline；SQLite interrupt 根据原因映射为 `Cancelled` 或 `Timeout`。

## 架构与接口

### Domain

创建 `local_session_directory_observation.rs`：

- `LocalSessionTitle`
- `LocalSessionDirectoryEntry`
- `LocalSessionSourceCoverage`
- `LocalSessionDirectoryPage`
- 领域构造错误和只读访问器
- 标题规范化、长度边界和脱敏 Debug

### Application

创建 `local_session_directory_observation.rs`：

- `LocalSessionDirectoryRequest::new(offset, limit)`
- `LocalSessionDirectoryCancellation`
- `LocalSessionDirectoryObservationPort`
- `ObserveLocalSessionDirectory`
- `Result<Option<Page>, ApplicationError>` 到 `LoadCompletion` 的稳定映射

### Platform

创建 `local_session_directory_observation.rs`：

- `SystemLocalSessionDirectoryObservation`
- 环境与路径 probe，生产实现只读取当前进程环境和固定根目录
- SQLite candidate 发现、普通文件/符号链接门禁和 `32` 上限
- `threads` / `automation_runs` schema 识别和最小列查询
- 多来源排序、去重、分页、覆盖状态
- read-only、query-only、busy timeout、deadline/cancellation 与脱敏错误

### Parity

- 新增 observation feature、baseline contract 与合成 fixture。
- `tauri-command:list_local_sessions` 重映射到 observation，副作用收敛为 `database-read` 与受控目录枚举。
- 原管理总功能保留删除、备份、恢复和 grouped undo 入口及 `unassessed` 状态。

## TDD 执行批次

### Batch 0：Planning Freeze

- [x] 创建 Session Plan、Runtime Workflow 和本计划。
- [x] 回写 `AGENTS.md`、`CONTEXT.md`、Master Plan 与 `build.md`。
- [x] 验证七路径 planning allowlist 和 hash。
- [x] 运行 AGOS default-entry `-ReportOnly`；实际 `blocked/unregistered` 与缺失 owner scope manifest 已记录并按项目规则绕过。
- [ ] 建立 planning Git checkpoint。

### Batch 1：Domain RED → GREEN

- [ ] 先写领域测试：标题空白、空白规范化、控制字符、256 字符边界、截断、空 ID、分页与覆盖不变量、脱敏 Debug。
- [ ] 运行定向测试并证明因模块/API 缺失而 RED。
- [ ] 写最小领域实现并导出符号。
- [ ] 运行领域测试 GREEN、格式与 Clippy。
- [ ] 建立 domain Git checkpoint。

### Batch 2：Application RED → GREEN

- [ ] 先写应用测试：默认/合法分页、非法分页、Ready/Partial、Empty、Failed、取消标记和旧结果隔离所需接口。
- [ ] 运行定向测试并记录 RED 根因。
- [ ] 写 Request、Cancellation、Port 和 UseCase 的最小实现。
- [ ] 运行应用测试 GREEN、格式与 Clippy。
- [ ] 建立 application Git checkpoint。

### Batch 3：Platform RED → GREEN

- [ ] 先写合成 SQLite 测试：无库、threads、automation_runs、可选列、跨库排序/去重、当前/legacy 优先级、分页、部分失败、全部失败、符号链接/非普通文件、候选超限、非法显式根、只读、超时和取消。
- [ ] 运行平台测试并证明缺少实现/依赖的 RED。
- [ ] 在根 Workspace 与 Platform crate 锁定 `rusqlite = 0.40.1`，仅启用 `bundled`、`hooks`。
- [ ] 写最小候选发现、只读连接、schema 查询和结果聚合实现。
- [ ] 验证真实 fixture 文件没有被写入，且错误/Debug 不含私人路径、标题或 ID。
- [ ] 运行平台测试 GREEN、格式与 Clippy。
- [ ] 建立 platform Git checkpoint。

### Batch 4：Parity RED → GREEN

- [ ] 先扩展目录测试，要求新 feature/contract/fixture、source remap、管理总功能继续未评估和副作用收敛。
- [ ] 运行 `catalog_repository` 并记录 RED。
- [ ] 写最小 YAML、fixture 和说明更新。
- [ ] 运行 Parity 测试 GREEN。
- [ ] 建立 parity Git checkpoint。

### Batch 5：文档与本地收口

- [ ] 更新 README 稳定能力说明与任务报告。
- [ ] 仅在出现新的可复用根因时更新 `err.md`；否则实际范围排除该路径。
- [ ] 运行 `build.md` 的 Issue #104 完整本地轻量验证。
- [ ] 执行安全清单、依赖许可证/锁文件、禁止内容和范围哈希检查。
- [ ] 运行知识图谱刷新；CodeGraph 无有效索引时不得初始化，只记录跳过原因。
- [ ] 建立 local-verified Git checkpoint。

### Batch 6：远端交付

- [ ] 普通 push 当前分支。
- [ ] 创建关联 Issue #104 的非 Draft PR。
- [ ] 处理 Review，对每条反馈记录根因、处理和新验证证据。
- [ ] 核验 CI、Performance Baseline、Artifact 和 Review 对话。
- [ ] 绑定 Final Head 请求独立 Squash Merge 授权。

## 精确路径范围

候选范围固定为二十九路径，完整清单和哈希以 Session Plan、Runtime Workflow、`build.md` 与 Issue #104 范围冻结评论为准。

- `planning_count`: `7`
- `planning_scope_hash`: `sha256:8b9295414fde23c7ea9c9c53a47acfb7a70b1f1bcd3255f81cb011ba5b624a51`
- `candidate_count`: `29`
- `candidate_scope_hash`: `sha256:47dcb2c181daa61a8df073e7f3ada069bf8e3d9b95df0c57f7709bcb6cde211d`
- `normal_count_without_err`: `28`
- `normal_scope_without_err`: `sha256:acee55e9539631f6eca4fb557d27999c7815d20ae15a4b4ea932db243079cb8c`

## 成功标准

- A1 条目、分页、跨库去重和覆盖状态拥有领域、应用和真实 SQLite 合成测试。
- 生产 SQLite 连接只读且不改变数据库、WAL、SHM 或业务文件内容。
- 非法显式根、符号链接、超限、损坏 schema、锁等待、超时和取消均有稳定脱敏失败。
- `feature.session-data.local-session-directory-observation=implemented`。
- `feature.session-data.local-session-management=unassessed`。
- 本地定向测试/Clippy/格式/治理/Release Audit/范围门通过。
- GitHub-hosted CI 与 Performance Baseline 通过且成功运行 Artifact 为 `0`。

## 停止门

出现下列任一情况立即停止：

- 需要修改二十九路径之外的文件。
- 需要返回路径、Provider、正文、Token、模型、账号或凭据。
- 需要数据库写入、删除、修复、迁移、备份或恢复。
- `rusqlite` 许可证、安全或跨平台构建合同不满足。
- 需要 UI、网络、子进程、永久线程、Watcher、Ruleset、收费 CI、上游缓存或 AGOS 改动。
- RED 无法归因于缺失行为，或 GREEN 需要扩大已批准语义。
- Release Audit 不是 `current`，工作树不干净，范围哈希漂移或基线发生变化。
- Final Head 未完成 Review/CI/Artifact 闭环却请求 Squash Merge。
