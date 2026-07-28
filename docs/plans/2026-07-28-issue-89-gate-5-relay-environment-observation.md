# Issue #89 Gate 5 Relay 环境只读观察设计

## 文档状态

- `status`: `IMPLEMENTATION_AUTHORIZED`
- `tracking_issue_ref`: `https://github.com/nonononull/inputcodex/issues/89`
- `approved_decision_ref`: `https://github.com/nonononull/inputcodex/issues/88`
- `baseline_ref`: `origin/main@db0c09b9df272887deb9407a5e344cf87a59dda8`
- `planning_scope_hash`: `sha256:0a301df75edda05c8d3d1c01c91221dd9ac8ff11aeeca39fb4c26a293b0543b0`
- `candidate_scope_hash`: `sha256:0adc20d0ed4d73ae645a5ffb23d7208f7aaabfea92c4d6fd62e0da3a120e8f77`

## 目标

以纯 Rust 迁移第五个 Gate 5 产品切片：只读观察影响 Relay 使用的本地环境事实，并返回明确覆盖、明确损坏且不泄露敏感值的稳定领域结果。

本切片只实现 Issue `#88` 批准的
`feature.provider-network.relay-environment-observation`。原
`feature.provider-network.network-environment` 必须继续保持 `unassessed`，不得把只读观察冒充成系统代理探测、网络连通性检查或完整 Relay 管理。

## 已批准边界

1. 新子能力接管 `core-module:relay_environment` 与 `tauri-command:check_relay_environment`；
2. `core-module:proxy` 继续归原总功能；
3. 只观察代理环境变量名称与来源、Codex `.env` 存在状态、Clash Verge TUN 配置状态；
4. 禁止返回环境变量值或读取 `.env` 内容；
5. Windows/macOS 返回同一领域结构，但不得伪造相同来源覆盖；
6. 配置不存在、禁用、启用、不可读取和格式无效必须可区分；
7. 不联网、不写入、不修改环境、不启动子进程或线程；
8. 非 Windows/macOS 明确失败；
9. 不创建 UI、后台轮询或递归 Closeout；
10. 实现、PR 与 Squash Merge 继续使用独立授权门。

## 上游根因

上游入口把进程/Windows 持久化代理环境、Codex `.env` 和多个 Clash Verge 配置候选聚合在一起，但会把文件读取失败、非法配置和 Windows 注册表读取失败静默折叠成“未启用/无变量”。功能目录还把该入口与 `core-module:proxy` 统一标为 `network-read`，而本切片实际不发起网络请求。

新架构保留有效观察能力，同时拒绝静默失败和错误副作用语义。

## 功能身份

### 新子能力

- ID：`feature.provider-network.relay-environment-observation`
- 实现后状态：`implemented`
- 来源：`core-module:relay_environment`、`tauri-command:check_relay_environment`
- 副作用：`environment-read`、`filesystem-read`
- 持久化：`none`

### 原总功能

- ID：`feature.provider-network.network-environment`
- 状态：继续 `unassessed`
- 剩余来源：`core-module:proxy`
- 未实现：系统代理探测、平台命令和网络诊断扩展

## 领域模型

### 代理环境

`ProxyEnvironmentVariableName` 使用封闭枚举表示 `HTTP_PROXY`、`HTTPS_PROXY`、`ALL_PROXY`、`NO_PROXY` 与 `FTP_PROXY`。双平台均按 ASCII 大小写不敏感匹配，输出固定大写规范名，不接受任意字符串，也不保存变量值。

`ProxyEnvironmentSource` 固定为 `RuntimeProcess`、`PersistentUser`、`PersistentSystem`。同一变量可来自多个来源，结果稳定排序并去重。

`ObservationCoverageStatus` 固定为 `Observed`、`NotObserved`、`Unavailable`。`Unavailable` 表示平台具备该来源但读取失败，不得用空列表冒充成功；macOS 当前不扫描 shell profile 或系统数据库，持久化来源明确为 `NotObserved`。

### Codex `.env`

`CodexDotenvStatus` 固定为 `Absent`、`Present`、`Unavailable`。只做元数据级存在性检查，路径仅在平台适配器内部使用，不进入领域结果、诊断或 `Debug` 输出。

### Clash Verge TUN

`ClashConfigSource` 使用 `PlatformData`、`PlatformConfig`、`HomeAppConfig`、`HomeLegacyConfig` 四个固定逻辑来源，不返回实际路径。

每个来源产生 `ClashTunCandidateStatus`：`Absent`、`Disabled`、`Enabled`、`Unreadable`、`Invalid`。读取上限固定为 `64 KiB`；超限、非 UTF-8、顶层字段或布尔值非法均为 `Invalid`，I/O 失败为 `Unreadable`，禁止静默跳过。

### 聚合

`RelayEnvironmentObservation` 包含代理变量与来源、三类来源覆盖、`.env` 状态、固定顺序 Clash 候选状态及纯派生风险标记。即使没有风险，仍返回 `Ready` 报告，不使用 `LoadCompletion::Empty`。

## 应用层

- 请求：`RelayEnvironmentObservationRequest`，零敏感输入；
- Port：`RelayEnvironmentObservationPort::observe`；
- 用例：`ObserveRelayEnvironment<P>`；
- 成功：`LoadCompletion::Ready`；
- 平台硬失败：`LoadCompletion::Failed`；
- 请求身份、超时、取消、过期结果和隔离复用现有加载合同；
- 同步 Port 不启动后台线程，不自行等待或重试。

## 平台层

### 共享

- 从 `platform_paths` 提取 crate 内窄 `CODEX_HOME` 解析函数；
- 禁止复用完整 `PlatformPathsPort`，避免应用安装和状态根成为隐藏前置条件；
- 当前进程环境只扫描一次；
- Clash 候选固定、有序、去重并有界读取；
- `.env` 使用可报告 I/O 错误的元数据检查，禁止 `is_file` 静默吞错。

### Windows

- 当前进程使用 `std::env::vars_os`；
- 用户级和系统级持久化环境使用 Windows target 专用安全依赖 `windows-registry = 0.6.1`；
- 依赖最低 Rust `1.82`，许可证为 `MIT OR Apache-2.0`，与项目 Rust `1.97.1` 兼容；
- 项目源码继续禁止 `unsafe`；
- 单个注册表来源失败标记为 `Unavailable`，其他成功观察保留；
- 禁止 `reg.exe`、PowerShell、WMI 或其他子进程。

### macOS

- 当前进程使用 `std::env::vars_os`；
- 持久化用户级和系统级来源固定 `NotObserved`；
- 不读 shell profile，不调用 `launchctl`、`scutil` 或其他子进程；
- 使用用户目录和平台目录规则构造 Clash 候选。

### 其他平台

返回 `RELAY_ENVIRONMENT_OBSERVATION_UNSUPPORTED`，不得回退或伪造空报告。

## 稳定错误与部分结果

- `RELAY_ENVIRONMENT_OBSERVATION_UNSUPPORTED`
- `USER_HOME_UNAVAILABLE`
- `CODEX_HOME_INVALID`
- `RELAY_ENVIRONMENT_NAME_UNREPRESENTABLE`
- `RELAY_ENVIRONMENT_OBSERVATION_FAILED`

Windows 持久化来源不可用、`.env` 元数据不可用、Clash 配置不可读或无效优先作为领域状态返回，不因局部问题丢弃其他成功观察。

## 性能与隐私

- 进程环境单次 `O(n)` 扫描，只匹配五个固定名称；
- Clash 候选固定，每个最多读取 `64 KiB`；
- Windows 只读两个固定注册表键；
- 不联网、不重试、不轮询、不缓存、不启动线程；
- 不引入 SQLite、异步运行时、HTTP 客户端或目录遍历；
- 产品结果不含变量值和实际路径；
- 诊断只记录请求标识、耗时、结果类别、覆盖/状态计数和稳定诊断码。

## Parity 设计

- `provider-network.yml` 新增只读子能力，原总功能只保留 `core-module:proxy`；
- 新合同只允许 `environment-read`、`filesystem-read`，fixture policy 为 `none`；
- `source-index.yml` 只修正 `core-module:relay_environment` 与 `tauri-command:check_relay_environment` 的归属和副作用；
- `upstream/` 保持只读不变。

## TDD 验收

### Domain

- 五个名称规范化、拒绝其他名称；
- 多来源排序、去重和覆盖；
- `.env` 三态；
- Clash 四来源、五状态、超限与风险派生；
- 空事实仍形成报告；
- `Debug` 不含值或路径。

### Application

- 成功与局部不可用进入 `Ready`；
- 平台硬失败进入 `Failed`；
- 取消和过期结果遵守现有合同。

### Platform

- 双平台进程变量匹配一致；
- Windows 注册表成功、空键和访问失败；
- macOS 持久化来源为 `NotObserved`；
- `.env` 不读内容；
- Clash 缺失、禁用、启用、不可读、非法、超限和候选去重；
- `CODEX_HOME` 与平台路径语义一致；
- 非目标平台稳定 unsupported；
- Linux all-targets 无目标平台死代码。

### Parity 与政策

- 新子能力 `implemented`，原总功能 `unassessed`；
- 只修改批准的两入口映射；
- 无网络、写入、子进程、线程、UI、注入、遥测和 `unsafe`；
- Cargo 只增加批准的 Windows target 依赖；
- Windows/macOS Hosted CI 与 Performance observation 通过。

## 候选完整实施范围

```text
AGENTS.md
CONTEXT.md
Cargo.lock
Cargo.toml
README.md
build.md
crates/inputcodex-application/src/lib.rs
crates/inputcodex-application/src/relay_environment_observation.rs
crates/inputcodex-application/tests/relay_environment_observation.rs
crates/inputcodex-domain/src/lib.rs
crates/inputcodex-domain/src/relay_environment_observation.rs
crates/inputcodex-domain/tests/relay_environment_observation.rs
crates/inputcodex-parity/tests/catalog_repository.rs
crates/inputcodex-platform/Cargo.toml
crates/inputcodex-platform/src/lib.rs
crates/inputcodex-platform/src/platform_paths.rs
crates/inputcodex-platform/src/relay_environment_observation.rs
crates/inputcodex-platform/src/relay_environment_observation/macos.rs
crates/inputcodex-platform/src/relay_environment_observation/windows.rs
crates/inputcodex-platform/tests/relay_environment_observation.rs
docs/plans/2026-07-28-issue-89-gate-5-relay-environment-observation.md
docs/plans/PROJECT-MASTER-PLAN.md
docs/plans/sessions/2026-07-28-issue-89-gate-5-relay-environment-observation.md
docs/reports/issue-89-gate-5-relay-environment-observation.md
docs/workflows/2026-07-28-issue-89-gate-5-relay-environment-observation-runtime.md
err.md
parity/README.md
parity/contracts/provider-network.yml
parity/features/provider-network.yml
parity/features/source-index.yml
```

规范化后共 `30` 路径，哈希为
`sha256:0adc20d0ed4d73ae645a5ffb23d7208f7aaabfea92c4d6fd62e0da3a120e8f77`。

规划控制面为设计稿、Session Plan、Runtime Workflow 三路径，哈希为
`sha256:0a301df75edda05c8d3d1c01c91221dd9ac8ff11aeeca39fb4c26a293b0543b0`。

任何路径变化必须重新计算并取得批准。实现可只修改必要子集；`err.md` 只有出现新根因时才修改。

## 非目标

- 系统代理或网络连通性测试；
- Relay 配置保存、切换、应用；
- 环境、`.env` 或 Clash 配置修改；
- shell profile、`launchctl`、`scutil`、`reg.exe`；
- UI、后台轮询、Watcher、线程或缓存；
- 设置、会话数据库、更新、插件、脚本和注入；
- 其他目录错误修正；
- Release、预算、Workflow、Ruleset、`upstream/` 或 AGOS；
- 递归 Closeout。

## 审批门

1. Issue `#88` 决策已完成；
2. Issue `#89`、隔离分支和工作树已建立；
3. 三份规划控制面落盘并轻量验证；
4. 项目所有者已批准 `30` 路径与 `candidate_scope_hash`；
5. 已允许 TDD、实现、Cargo 更新和 Git checkpoint；
6. 本地轻量验证后普通推送并创建非 Draft PR；
7. Review 根因闭环且 Hosted CI 全绿后停在独立 Squash Merge 授权门；
8. 不创建递归 Closeout。

## 实施状态

- Planning checkpoint：`d4805d3f81801d4d685a510d31566738bc9d3ff6`；
- Domain checkpoint：`fba0e2e3e7ebb692b417b4e3a80800388f711a1f`；
- Application checkpoint：`13e1f349b8a4d2556864e67f1066884b0e4832a4`；
- 共享 Platform checkpoint：`1d9b774a661af44f765cb8e9cb3d7223a56e594d`；
- Windows/macOS checkpoint：`1dae9843618bdffc95f16069d54b7e7440d21db8`；
- Parity checkpoint：`749d02e23f1c1fa8f042d598d8f5bb5e28a18638`；
- 当前阶段：项目控制面已收口，等待最终本地轻量验证、文档 checkpoint、普通推送与非 Draft PR；
- 最终 Squash Merge：未授权，继续保留单独授权门。

## 自审

- [x] 与 Issue `#88` 一致；
- [x] 原总功能继续 `unassessed`；
- [x] 不联网、不写入、不启动进程或线程；
- [x] 明确双平台覆盖差异；
- [x] 不静默吞错；
- [x] 不泄露值、内容或路径；
- [x] 新依赖仅限 Windows target 安全注册表 API；
- [x] 本地负担有界，全量验证继续使用 GitHub-hosted runners；
- [x] 实现前保留精确范围批准门。
