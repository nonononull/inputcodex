# Issue #92 Gate 5 设置只读观察设计与实施计划

## 文档状态

- `status`: `LOCAL_VERIFIED_PR_PENDING`
- `tracking_issue_ref`: `https://github.com/nonononull/inputcodex/issues/92`
- `approved_decision_ref`: `https://github.com/nonononull/inputcodex/issues/91`
- `approved_design_ref`: `https://github.com/nonononull/inputcodex/issues/92#issuecomment-5107059878`
- `owner_planning_approval_ref`: `https://github.com/nonononull/inputcodex/issues/92#issuecomment-5107673827`
- `owner_scope_approval_ref`: `https://github.com/nonononull/inputcodex/issues/92#issuecomment-5108221117`
- `baseline_ref`: `origin/main@a5559f4a873a81d91ed09b571503523a78a45118`
- `branch_ref`: `codex/issue-92-gate-5-settings-observation`
- `worktree_ref`: `.worktrees/issue-92-gate-5-settings-observation`
- `planning_scope_hash`: `sha256:dccd142c0c926433ce01adda37524895db2f4369917a6455fcfc393941a10cc2`
- `candidate_scope_hash`: `sha256:ca252684075d32de7aaf2ca066f12822ce48a5b01d1b0fcf67df146ea792baf1`

## 目标

以纯 Rust 迁移第六个 Gate 5 产品切片：只读观察既有平台路径能力解析出的
`inputcodex` 设置文件，只返回顶层 JSON 对象条目数量。

本切片只实现 Issue `#91` 批准的
`feature.foundation-platform.settings-observation`。原
`feature.foundation-platform.settings-management` 必须继续保持 `unassessed`。

## 已批准边界

1. 新子能力只接管 `tauri-command:load_settings`。
2. `core-module:settings`、`save_settings`、`reset_settings` 继续归原设置管理总功能。
3. 请求为零输入，不接受任意路径；设置路径只由 `SystemPlatformPaths` 解析。
4. 文件不存在返回 `LoadCompletion::Empty`，语义为 `NotConfigured`。
5. 合法空对象 `{}` 返回 `Ready` 且 `top_level_entry_count = 0`。
6. 禁止返回键、值、原始 JSON、绝对路径、凭据和脚本内容。
7. 单文件上限固定 `256 KiB`，实际最多读取 `256 KiB + 1`。
8. 只接受 JSON object 根；损坏、非 UTF-8 和错误根明确失败。
9. Windows/macOS 领域结构和错误语义一致；非目标平台稳定失败。
10. 禁止写文件、网络、子进程、线程、Watcher、UI/Iced、注入和目录扫描。
11. 唯一新增直接依赖为 `serde_json = "=1.0.149"`。
12. 产品实现、提交、推送、PR、Review/CI 和 Squash Merge 仍需后续独立授权。

## 上游根因

- `SettingsStore::load` 把文件不存在静默转换为 `BackendSettings::default()`。
- 损坏 JSON 使用 `serde_json::from_str(...).unwrap_or_default()` 静默回退。
- `load_settings` 返回完整设置、私人路径和用户脚本清单。
- 读取与保存、重置、皮肤、Provider、Relay 和脚本副作用耦合。

新架构保留安全、可诊断的结构观察，拒绝默认值伪装和敏感数据扩散。

## 功能身份

### 新子能力

- ID：`feature.foundation-platform.settings-observation`
- 实现后状态：`implemented`
- 来源：`tauri-command:load_settings`
- 副作用：`filesystem-read`
- 持久化：`none`

### 原总功能

- ID：`feature.foundation-platform.settings-management`
- 状态：继续 `unassessed`
- 剩余来源：`core-module:settings`、`save_settings`、`reset_settings`
- 副作用：继续包含 `filesystem-read`、`filesystem-write`

## 分层设计

### Domain

新增 `SettingsDocumentObservation`，唯一字段为私有
`top_level_entry_count: usize`，提供构造函数和只读 getter。类型不保存路径、键、值、
字节内容或底层解析错误；`0` 是合法 Ready 事实。

### Application

新增零字段 `SettingsObservationRequest`、`SettingsObservationPort` 与
`ObserveSettings<P>`。Port 返回
`Result<Option<SettingsDocumentObservation>, ApplicationError>`，映射固定为：

- `Some` → `LoadCompletion::Ready`；
- `None` → `LoadCompletion::Empty`；
- `Err` → `LoadCompletion::Failed`。

请求身份、取消、超时、过期结果和隔离继续复用 `LoadCoordinator`；不新增线程或异步运行时。

### Platform

新增 `SystemSettingsObservation`：

1. 非 Windows/macOS 返回 `SETTINGS_OBSERVATION_UNSUPPORTED`。
2. 支持平台调用 `SystemPlatformPaths.resolve(PlatformPathsRequest::default())`。
3. 只取 `PlatformPathsSnapshot::settings_file()`；既有路径错误原样传播，禁止回退。
4. 使用 `fs::symlink_metadata`；NotFound 返回 `None`。
5. 符号链接或非普通文件返回 `SETTINGS_OBSERVATION_INVALID_FILE_TYPE`。
6. 元数据超过 `256 * 1024` 返回 `SETTINGS_OBSERVATION_TOO_LARGE`。
7. 通过 `File::open` 和 `Read::take(256 * 1024 + 1)` 有界读取并复核上限。
8. I/O 失败返回 `SETTINGS_OBSERVATION_UNAVAILABLE`，不暴露底层消息或路径。
9. `serde_json::from_slice::<Value>` 失败返回 `SETTINGS_OBSERVATION_INVALID_JSON`。
10. 根不是 `Value::Object` 返回 `SETTINGS_OBSERVATION_INVALID_ROOT`。
11. 成功后只保留 `object.len()`。

文件探针和任意路径入口保持模块私有；不得为了测试向产品公开任意路径读取 API。

## 稳定错误

| 场景 | `ErrorKind` | `DiagnosticCode` |
| --- | --- | --- |
| 非支持平台 | `Unsupported` | `SETTINGS_OBSERVATION_UNSUPPORTED` |
| 文件不可读取 | `Unavailable` | `SETTINGS_OBSERVATION_UNAVAILABLE` |
| 符号链接或非普通文件 | `InvalidInput` | `SETTINGS_OBSERVATION_INVALID_FILE_TYPE` |
| 超过 `256 KiB` | `InvalidInput` | `SETTINGS_OBSERVATION_TOO_LARGE` |
| JSON 损坏或非 UTF-8 | `InvalidInput` | `SETTINGS_OBSERVATION_INVALID_JSON` |
| JSON 根不是 object | `InvalidInput` | `SETTINGS_OBSERVATION_INVALID_ROOT` |

错误只暴露稳定 kind/code；禁止记录解析消息、私人路径和文件内容。

## 性能与隐私

- 单次请求只解析一次平台路径并读取一个固定文件。
- 时间复杂度 `O(n)`，输入硬上限 `256 KiB`。
- 不扫描目录、脚本、Provider、Relay、皮肤或其他候选文件。
- 不联网、不重试、不轮询、不缓存、不启动线程。
- 观测证据只允许请求标识、耗时、结果类别、稳定错误码和条目数量。

## Parity 设计

1. 新增 `feature.foundation-platform.settings-observation` 和无 fixture 合同。
2. 只把 `tauri-command:load_settings` 移入新子能力并改为 `[filesystem-read]`。
3. 原设置管理移除 `load_settings`，其余三入口和 `unassessed` 不变。
4. feature/contract 从 `38` 增至 `39`；source 保持 `133`；覆盖缺口保持 `0`。
5. fixture manifest 保持 `11`；`upstream/` 不变。

## TDD 验收

- Domain：零/非零数量、私有字段、隐私 Debug。
- Application：Some→Ready、None→Empty、Err→Failed，合法空对象不得变 Empty。
- Platform：缺失、合法对象、损坏、非 UTF-8、错误根、非普通文件、超限、I/O 失败、unsupported。
- Parity：新能力 implemented、原总功能 unassessed、只移动 load_settings、`39/39`、source `133`、fixture `11`。
- 政策：无网络、写入、子进程、线程、UI、注入、遥测、公开任意路径和 `unsafe`。

## 当前规划写入范围

```text
docs/plans/2026-07-28-issue-92-gate-5-settings-observation.md
docs/plans/sessions/2026-07-28-issue-92-gate-5-settings-observation.md
docs/reports/issue-92-gate-5-settings-observation.md
docs/workflows/2026-07-28-issue-92-gate-5-settings-observation-runtime.md
```

按 `StringComparer.Ordinal` 排序、LF 拼接且末尾保留 LF：

- 路径数：`4`
- 哈希：`sha256:dccd142c0c926433ce01adda37524895db2f4369917a6455fcfc393941a10cc2`

## 候选完整实施范围

```text
AGENTS.md
CONTEXT.md
Cargo.lock
Cargo.toml
README.md
build.md
crates/inputcodex-application/src/lib.rs
crates/inputcodex-application/src/settings_observation.rs
crates/inputcodex-application/tests/settings_observation.rs
crates/inputcodex-domain/src/lib.rs
crates/inputcodex-domain/src/settings_observation.rs
crates/inputcodex-domain/tests/settings_observation.rs
crates/inputcodex-parity/tests/catalog_repository.rs
crates/inputcodex-platform/Cargo.toml
crates/inputcodex-platform/src/lib.rs
crates/inputcodex-platform/src/settings_observation.rs
crates/inputcodex-platform/tests/settings_observation.rs
docs/plans/2026-07-28-issue-92-gate-5-settings-observation.md
docs/plans/PROJECT-MASTER-PLAN.md
docs/plans/sessions/2026-07-28-issue-92-gate-5-settings-observation.md
docs/reports/issue-92-gate-5-settings-observation.md
docs/workflows/2026-07-28-issue-92-gate-5-settings-observation-runtime.md
err.md
parity/README.md
parity/contracts/foundation-platform.yml
parity/features/foundation-platform.yml
parity/features/source-index.yml
```

- 路径数：`27`
- 哈希：`sha256:ca252684075d32de7aaf2ca066f12822ce48a5b01d1b0fcf67df146ea792baf1`

任何路径变化必须先同步四份规划文件、复算哈希并重新批准。实际实现可少改，不能越界；
`err.md` 只有出现新可复用根因时才修改。

## 实施任务

### Task 0：规划冻结

- [x] 决策 Issue `#91`、实现 Issue `#92` 和书面设计完成。
- [x] 项目所有者批准方案 A 和规划写入。
- [x] 精确基线、隔离分支/worktree 和四份规划控制面就绪。
- [x] 完成规划范围、哈希、占位符、受保护路径和 diff 检查。
- [x] 回写 Issue 候选范围与验证证据。
- [x] 取得 `27` 路径与哈希批准。

### Task 1-4：产品 TDD

- [x] Domain RED→GREEN：数量事实和隐私边界。
- [x] Application RED→GREEN：Request/Port/UseCase 与 Ready/Empty/Failed。
- [x] Platform RED→GREEN：依赖、路径、文件类型、双重上限、解析和稳定错误。
- [x] Parity RED→GREEN：feature/contract/source mapping 与仓库级断言。
- [x] 每批验证后建立 named Git checkpoint。

### Task 5：控制面收口

- [x] 更新 README、CONTEXT、AGENTS、Master Plan、parity README 和 build.md 的稳定事实。
- [x] 新根因才更新 `err.md`；本次复用既有离线依赖缓存根因，`err.md` 保持不变。
- [x] 不写递归 Closeout 或动态合并证据。

### Task 6：验证与远端交付

- [x] 四 crate tests/Clippy、rustfmt、范围、隐私、依赖和禁止能力验证。
- [ ] 普通提交、普通推送、非 Draft PR、Review 根因闭环和 Hosted CI。
- [ ] Final Head 全绿后等待独立 Squash Merge 授权。

## 本地实施证据

- Planning checkpoint：`3848372d5bd75b175901df59a853aa62d03c8e73`。
- Domain checkpoint：`c752d54506e9307528324c3ce5c2ccecfe23f9c7`。
- Application checkpoint：`10cb5fd0f9f0093b7b9f07e5537e15bb8a84b822`。
- Platform checkpoint：`f77d7edd8a00ec6d40a62808fc50a75d1ec70df4`。
- Parity checkpoint：`7e55bd038d7b0e5e7dbbb12a781df510c920aed6`。
- 候选 allowlist 保持 `27` 路径和批准哈希；实际使用 `26` 路径，`err.md` 未修改。
- Cargo 只新增 `serde_json 1.0.149` 与必要锁包 `zmij 1.0.23`；许可证分别为
  `MIT OR Apache-2.0` 与 `MIT`。
- feature/contract 为 `39/39`，source 为 `133`，fixture manifest 为 `11`；禁止能力和隐私扫描命中均为 `0`。

## 当前授权

项目所有者已批准 `27` 路径与候选哈希，允许在 allowlist 内执行 TDD、产品实现、本地轻量验证、
named Git checkpoint、普通提交、普通推送、非 Draft PR 与 Review/Hosted CI 根因闭环。

最终 Squash Merge 继续保留单独授权门；任何第 `28` 条路径、依赖变化或语义扩大必须重新批准。

## 停止条件

- 精确范围或哈希未获批准；基线漂移；需要第 `28` 条路径。
- 需要公开任意路径读取、网络、写入、子进程、线程、Watcher、UI、注入或 `unsafe`。
- `serde_json` 版本、许可证或锁文件漂移超出批准边界。
- 无法区分 NotConfigured、损坏、错误根和合法空对象。
- 无法保护路径、键、值、原始 JSON 和凭据。
- `release_audit` 不为 `current`；Review/CI 根因未闭环；未获独立 Squash Merge 授权。
