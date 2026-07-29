# Issue #98 Gate 5 Relay 认证与配置状态只读观察设计与实施计划

## 文档状态

- `status`: `LOCAL_VERIFIED_PR_PENDING`
- `tracking_issue_ref`: `https://github.com/nonononull/inputcodex/issues/98`
- `approved_decision_ref`: `https://github.com/nonononull/inputcodex/issues/97#issuecomment-5115176838`
- `design_ref`: `https://github.com/nonononull/inputcodex/issues/98`
- `implementation_scope_approval_ref`: `https://github.com/nonononull/inputcodex/issues/98#issuecomment-5115957943`
- `baseline_ref`: `origin/main@b7c4174671caba806162a42e82b7bc0b20f73bf5`
- `branch_ref`: `codex/issue-98-gate-5-relay-status-observation`
- `worktree_ref`: `.worktrees/issue-98-gate-5-relay-status-observation`
- `planning_scope_hash`: `sha256:ec6c88d4a96c351fee85d6c416b04c95b27050893ccbe55b4ad55edfd8d95051`
- `candidate_scope_hash`: `sha256:b1dda60cda57d4be9344b3fa0c74a49b6087b9bdf03fceb5a772ec7e893d63a5`
- `planning_checkpoint_ref`: `a6b3cdc5f88cafaf1624da34793bb133d8d05300`
- `domain_checkpoint_ref`: `01d94aba6a65544d42d1da9d18b3affb0afb3227`
- `application_checkpoint_ref`: `b0bdf4eac7c0ee19a3f85b7ae2ba35e4683b7247`
- `platform_checkpoint_ref`: `f6eb1d3ce5f934a09878fe83b7945df2730b3f2e`
- `parity_checkpoint_ref`: `f21d04225e131eb0ed2bbe24cc7827046ceaef1f`
- `mutation_intent`: `source`
- `planning_checkpoint_policy`: `completed-and-recorded`

## 目标

以纯 Rust 迁移第八个 Gate 5 产品切片：只读观察既有平台路径能力定位的
`CODEX_HOME/auth.json` 与 `CODEX_HOME/config.toml`，仅返回脱敏后的文档状态、凭据存在事实和
Relay 配置完整性，不返回账号、凭据、Provider 标识、URL、字段、内容或实际路径。

本切片新增 `feature.provider-network.relay-status-observation`，只接管
`tauri-command:relay_status`。原 `feature.provider-network.relay-profile-management`、
`core-module:relay_config` 及其读取完整文件、保存、切换和回填能力继续 `unassessed`。

## 已批准设计边界

1. 请求为零字段；禁止任意路径、文件名、读取上限、Provider ID 或认证来源输入。
2. 只允许从 `SystemPlatformPaths.resolve` 的 `codex_home()` 派生固定 `auth.json` 与
   `config.toml`。
3. 两份文件均不存在时返回 `LoadCompletion::Empty`，稳定语义为 `NotConfigured`。
4. 只要至少一份文件存在，就返回 `Ready(RelayStatusObservation)`，并保留每份文档的明确状态。
5. 单文件上限固定为 `256 KiB`，总读取预算不超过 `512 KiB`。
6. 缺失、损坏、超限、不可读不得静默折叠为未认证、未配置、`false` 或空文档。
7. 只返回 `Present / Absent / NotObserved` 和
   `NotConfigured / Complete / Incomplete / NotObserved` 等封闭状态。
8. 禁止返回账号标签、Token、Provider ID、Base URL、字段名、字段内容、认证来源、实际路径、
   用户名、机器名、摘要、哈希或任何可逆派生值。
9. 不联网、不写文件、不修改环境、不调用子进程、不启动线程/Watcher、不打开 UI、不注入、
   不使用 `unsafe`。
10. Windows 与 macOS 使用相同领域语义、大小上限、解析规则和最小披露合同。
11. Iced 不进入 Domain、Application、Platform 或 Parity；本 Issue 不创建 UI。
12. 产品 TDD、实现、普通推送、PR、Review/CI 与最终 Squash Merge 均受后续独立门禁控制。

## 上游根因

- 上游 `relay_status_from_home` 聚合 `authenticated`、`auth_source`、`account_label`、
  `config_path`、`configured` 等值，直接扩大账号与路径披露面。
- `relay_config_status_from_home` 对两份文件使用 `read_to_string(...).unwrap_or_default()`，
  缺失、不可读和读取失败都会冒充空内容。
- `auth_json_chatgpt_account_label` 将读取失败、JSON 损坏、认证模式不匹配和凭据缺失统一折叠为
  `None`，错误语义会冒充“未登录”。
- 上游 `configured` 将 Provider 选择、认证要求、Bearer Token/API Key 和 Base URL 聚合为单一
  布尔值，无法说明未配置、不完整、不可观察或文档损坏。
- 原 Relay 总功能还包含完整文件读取、保存、切换、回填和网络相关能力，不能为了状态入口整体迁移。

新架构保留必要的只读结构事实，同时把隐私、损坏状态与副作用隔离在明确边界内。

## 功能身份

### 新子能力

- ID：`feature.provider-network.relay-status-observation`
- 实现后状态：`implemented`
- 唯一来源：`tauri-command:relay_status`
- 副作用：`filesystem-read`
- 持久化：`none`

### 原总功能

- ID：`feature.provider-network.relay-profile-management`
- 状态：继续 `unassessed`
- 保留来源：`core-module:relay_config`、`tauri-command:read_relay_files`、
  `tauri-command:save_relay_file`、`tauri-command:switch_relay_profile`、
  `tauri-command:backfill_relay_profile_from_live`
- 完整文件读取、保存、切换、回填、网络测试、代理、模型目录和应用生命周期均不进入本切片。

## 领域模型

### 文档状态

```rust
pub enum RelayDocumentStatus {
    Missing,
    Valid,
    Invalid,
    TooLarge,
    Unreadable,
}
```

- `Missing`：`symlink_metadata` 返回 `NotFound`。
- `Valid`：普通文件、未超限、严格 UTF-8 且解析根结构合法。
- `Invalid`：严格 UTF-8、JSON/TOML 解析或规定根结构不合法。
- `TooLarge`：元数据长度超过上限，或实际有界读取发现第 `256 KiB + 1` 个字节。
- `Unreadable`：符号链接、非普通文件、元数据/打开/读取失败。

### 凭据存在事实

```rust
pub enum CredentialPresence {
    Present,
    Absent,
    NotObserved,
}
```

- 文档 `Valid` 时按固定结构检查，存在非空目标字符串为 `Present`，否则为 `Absent`。
- 文档 `Missing` 时为 `Absent`，因为固定文档不存在且不会自动探测其他来源。
- 文档 `Invalid / TooLarge / Unreadable` 时为 `NotObserved`，禁止推断不存在。

### Relay 配置完整性

```rust
pub enum RelayConfigurationStatus {
    NotConfigured,
    Complete,
    Incomplete,
    NotObserved,
}
```

- `config.toml` 缺失，或合法文档没有非空根 `model_provider`：`NotConfigured`。
- 配置文档不可观察：`NotObserved`。
- 已选择 Provider，但目标表、`requires_openai_auth = true` 或非空 `base_url` 等结构不完整：
  `Incomplete`。
- 基础结构完整，且存在非空 `experimental_bearer_token`：`Complete`，不依赖认证文档状态。
- 基础结构完整、没有 Bearer Token 时：有效 `OPENAI_API_KEY` 为 `Complete`；明确不存在为
  `Incomplete`；认证文档不可观察为 `NotObserved`。
- ChatGPT 登录凭据单独报告，不参与 Relay 配置完整性的判定，保持与上游实际 `configured` 条件一致。

### 聚合值

```rust
pub struct RelayStatusObservation {
    auth_document_status: RelayDocumentStatus,
    config_document_status: RelayDocumentStatus,
    chatgpt_credentials: CredentialPresence,
    openai_api_key: CredentialPresence,
    relay_configuration: RelayConfigurationStatus,
}
```

字段保持私有，只提供构造函数与只读 getter。类型不得保存路径、字节、字符串、JSON/TOML 节点、
Provider ID 或底层 I/O 错误。Debug 输出只能包含上述封闭枚举值。

## 固定结构判定

### `auth.json`

1. 严格 UTF-8 解析为 `serde_json::Value`。
2. 根必须为 object；数组、标量和 `null` 为 `Invalid`。
3. ChatGPT 凭据只有在 `auth_mode` 为大小写不敏感的 `chatgpt`、`tokens` 为 object，且
   `access_token / id_token / refresh_token` 至少一个是非空字符串时为 `Present`。
4. `OPENAI_API_KEY` 只有在根字段为非空字符串时为 `Present`。
5. 不解码 JWT，不读取账号标签，不返回字段名或任何值。

### `config.toml`

1. 严格 UTF-8 后解析为 `toml_edit::DocumentMut`。
2. 根 `model_provider` 必须是非空字符串，才能进入已配置状态判断。
3. 只在对应 `model_providers.<provider>` table 内检查：
   `requires_openai_auth` 是否为 `true`、`base_url` 是否为非空字符串、
   `experimental_bearer_token` 是否为非空字符串。
4. Provider ID 只用于文档内局部索引，绝不保存、返回、记录或写入诊断信息。
5. 其他字段、数组、内联表和文档内容均不进入返回值。

## 分层设计

### Domain

- 新增 `relay_status_observation` 模块及上述三个枚举和一个聚合值。
- 枚举为封闭状态，不承载字符串。
- 聚合值字段私有，测试覆盖组合语义、getter 和 Debug 最小披露。

### Application

```rust
pub struct RelayStatusObservationRequest;

pub trait RelayStatusObservationPort {
    fn observe(
        &self,
        request: &RelayStatusObservationRequest,
    ) -> Result<Option<RelayStatusObservation>, ApplicationError>;
}

pub struct ObserveRelayStatus<P> {
    port: P,
}
```

- Request 为零字段，不能注入路径、凭据或读取策略。
- `Some(observation) -> LoadCompletion::Ready(observation)`。
- `None -> LoadCompletion::Empty`，只用于两份固定文件均缺失。
- `Err(error) -> LoadCompletion::Failed(error)`。
- 旧请求到新 UseCase 的类型隔离测试必须保留。

### Platform

- 新增 `SystemRelayStatusObservation`，Windows/macOS 复用 `SystemPlatformPaths.resolve`。
- 只从 `codex_home()` 派生 `auth.json` 和 `config.toml`；禁止公开任意路径产品 API。
- 模块私有 `RelayStatusFileProbe` 负责可测试的元数据与有界读取。
- 读取顺序固定为：`symlink_metadata` → 文件类型门禁 → 元数据上限 →
  `File::open().take(limit + 1)` → 实际上限复核 → 严格解析。
- 单份文档问题进入 `RelayDocumentStatus`，不得使另一份文档事实丢失。
- 只有平台不支持或 `SystemPlatformPaths` 解析失败才返回 Application 错误；路径错误沿用既有稳定码。
- 非 Windows/macOS 返回 `RELAY_STATUS_OBSERVATION_UNSUPPORTED`。

### 依赖

- 根 `Cargo.toml` 固定 Workspace 依赖：
  `toml_edit = { version = "=0.25.13", default-features = false, features = ["parse"] }`；
  Cargo 锁定结果保持 `0.25.13+spec-1.1.0`，避免在版本约束中写入会被 SemVer 忽略的构建元数据。
- `crates/inputcodex-platform/Cargo.toml` 只引用 Workspace 依赖。
- `Cargo.lock` 纳入允许范围；本地锁定源码已确认该版本最低 Rust 为 `1.85`，支持
  `DocumentMut: FromStr` 及本切片所需只读访问。
- `serde_json` 复用现有 Workspace 固定版本，不新增另一 JSON 解析器。

### Parity

1. 在 provider-network feature/contract 中新增
   `feature.provider-network.relay-status-observation`。
2. 只把 `tauri-command:relay_status` 重映射到新子能力，副作用为 `[filesystem-read]`。
3. 原 `feature.provider-network.relay-profile-management` 与其余来源保持 `unassessed`。
4. 更新目录测试与计数断言；不得改写 Release 真源或 `upstream/` 快照。

## Planning Allowlist

```text
docs/plans/2026-07-29-issue-98-gate-5-relay-status-observation.md
docs/plans/sessions/2026-07-29-issue-98-gate-5-relay-status-observation.md
docs/reports/issue-98-gate-5-relay-status-observation.md
docs/workflows/2026-07-29-issue-98-gate-5-relay-status-observation-runtime.md
```

- `count`: `4`
- `hash`: `sha256:ec6c88d4a96c351fee85d6c416b04c95b27050893ccbe55b4ad55edfd8d95051`

## Candidate Implementation Allowlist

```text
AGENTS.md
CONTEXT.md
Cargo.lock
Cargo.toml
README.md
build.md
crates/inputcodex-application/src/lib.rs
crates/inputcodex-application/src/relay_status_observation.rs
crates/inputcodex-application/tests/relay_status_observation.rs
crates/inputcodex-domain/src/lib.rs
crates/inputcodex-domain/src/relay_status_observation.rs
crates/inputcodex-domain/tests/relay_status_observation.rs
crates/inputcodex-parity/tests/catalog_repository.rs
crates/inputcodex-platform/Cargo.toml
crates/inputcodex-platform/src/lib.rs
crates/inputcodex-platform/src/relay_status_observation.rs
crates/inputcodex-platform/tests/relay_status_observation.rs
docs/plans/2026-07-29-issue-98-gate-5-relay-status-observation.md
docs/plans/PROJECT-MASTER-PLAN.md
docs/plans/sessions/2026-07-29-issue-98-gate-5-relay-status-observation.md
docs/reports/issue-98-gate-5-relay-status-observation.md
docs/workflows/2026-07-29-issue-98-gate-5-relay-status-observation-runtime.md
err.md
parity/README.md
parity/contracts/provider-network.yml
parity/features/provider-network.yml
parity/features/source-index.yml
```

- `count`: `27`
- `hash`: `sha256:b1dda60cda57d4be9344b3fa0c74a49b6087b9bdf03fceb5a772ec7e893d63a5`
- 排序与哈希：`StringComparer.Ordinal`、UTF-8、LF 拼接并保留末尾 LF。

## TDD 执行批次

### Batch 0：规划冻结

1. 落盘四份任务本地控制面。
2. 验证实际 diff 精确等于四路径并复算两个 scope hash。
3. 执行 CI 脚本合同、仓库政策、占位符和 `git diff --check`。
4. 运行 AGOS ReportOnly；不可用、未登记或 `needs-input` 时记录并立即绕过。
5. 建立本地 planning checkpoint，回写 Issue `#98` 后硬停止。

### Batch 1：Domain RED → GREEN

1. 先写枚举、聚合值、组合语义和最小 Debug 的失败测试。
2. 实现私有字段、构造与 getter，不引入字符串载荷。
3. 运行 Domain 全目标 tests/Clippy，建立 named Git checkpoint。

### Batch 2：Application RED → GREEN

1. 先写零字段 Request、Port、UseCase、Ready/Empty/Failed 和旧请求隔离测试。
2. 实现 `ObserveRelayStatus<P>` 的固定映射。
3. 运行 Application 全目标 tests/Clippy，建立 named Git checkpoint。

### Batch 3：Platform RED → GREEN

1. 用模块私有 probe 先覆盖缺失、合法、损坏、超限、非普通文件和读取失败。
2. 覆盖 ChatGPT 凭据、API Key、Provider 结构和跨文档完整性组合。
3. 实现固定路径、有界读取、严格 JSON/TOML 解析和最小披露聚合。
4. 运行 Platform 全目标 tests/Clippy/fmt、隐私与禁止能力扫描，建立 named Git checkpoint。

### Batch 4：Parity RED → GREEN

1. 先证明新 feature/contract/source mapping 不存在。
2. 新增 observation 子能力，只移动 `relay_status`。
3. 固定原 Relay 总功能和其余入口继续 `unassessed`。
4. 运行 Parity tests/Clippy 和 Release Audit，建立 named Git checkpoint。

### Batch 5：稳定控制面与本地收口

1. README 只记录稳定用户能力；动态 Head、CI 和合并证据留在 GitHub。
2. 更新 AGENTS、CONTEXT、Master Plan、Parity README 和 build 验证入口。
3. `err.md` 仅在出现新且可复用根因时更新；重复问题引用既有条目。
4. 执行四 crate 定向 tests/Clippy、fmt、CI 合同、仓库政策、范围和隐私扫描。
5. 建立最终本地验证 checkpoint，普通推送并创建关联 Issue `#98` 的非 Draft PR。

### Batch 6：Review / CI / Merge 门

1. 所有 Review 对话写明根因、处理方式与验证证据后解决。
2. 核验 Hosted 标准 CI、Performance observation 和 Artifact 合同。
3. Final Head 全绿后请求独立 Squash Merge 授权；禁止自行合并。

## 本地实施 Checkpoints

- Planning：`a6b3cdc5f88cafaf1624da34793bb133d8d05300`。
- Domain GREEN：`01d94aba6a65544d42d1da9d18b3affb0afb3227`。
- Application GREEN：`b0bdf4eac7c0ee19a3f85b7ae2ba35e4683b7247`。
- Platform GREEN：`f6eb1d3ce5f934a09878fe83b7945df2730b3f2e`。
- Parity GREEN：`f21d04225e131eb0ed2bbe24cc7827046ceaef1f`。
- 最终内容态验证已通过；本次文档提交将封存 `issue-98-local-verified`，动态 Head 留在 GitHub Issue/PR。

## TDD 验收

### Domain RED → GREEN

- 三个封闭枚举与聚合值最初不存在，RED 由缺失类型稳定触发。
- GREEN 固定五种文档状态、三种凭据状态、四种配置状态、getter 与最小 Debug。

### Application RED → GREEN

- 零字段 Request、Port 与 UseCase 最初不存在。
- GREEN 固定 `Some → Ready`、`None → Empty`、`Err → Failed`，不接收旧请求或任意路径。

### Platform RED → GREEN

- 先覆盖公开 Port 缺失与私有文件矩阵，再实现固定双文档、有界读取和严格 JSON/TOML 解析。
- 两份均缺失返回 `None`；至少一份存在时保留每份文档状态，不泄露字符串、路径或内容。

### Parity RED → GREEN

- RED 证明新 feature/contract/source mapping 不存在。
- GREEN 只移动 `tauri-command:relay_status`，目录固定为 `41/41`、source `133`、fixture `11`，原 Relay 配置管理总功能继续 `unassessed`。

## 本地验证合同

规划阶段：

```powershell
pwsh -NoProfile -File scripts/ci/Test-CiScripts.ps1
pwsh -NoProfile -File scripts/ci/Verify-RepositoryPolicy.ps1 -RepositoryRoot .
cargo metadata --locked --offline --no-deps
git diff --check
```

批准后的实现阶段最低执行：

```powershell
cargo test -p inputcodex-domain --all-targets --offline
cargo test -p inputcodex-application --all-targets --offline
cargo test -p inputcodex-platform --all-targets --offline
cargo test -p inputcodex-parity --all-targets --offline
cargo clippy -p inputcodex-domain --all-targets --offline -- -D warnings
cargo clippy -p inputcodex-application --all-targets --offline -- -D warnings
cargo clippy -p inputcodex-platform --all-targets --offline -- -D warnings
cargo clippy -p inputcodex-parity --all-targets --offline -- -D warnings
cargo fmt --all -- --check
```

完整 Workspace、Windows/macOS 编译和 Performance observation 交给 GitHub-hosted runners。

## 验证成功标准

- 规划 diff 精确为四路径，planning/candidate 哈希复算一致。
- 实现后 Domain/Application/Platform/Parity 定向 tests 与 Clippy 全绿。
- `cargo fmt --all -- --check`、CI 合同、仓库政策和 Release Audit 全绿。
- 两份文件各自状态不被另一份文件错误覆盖。
- 两份均缺失为 Empty；任何一份存在为带明确状态的 Ready。
- 隐私与禁止能力扫描命中为 `0`。
- 实际路径集是候选二十七路径子集；新增路径必须重新审批。
- PR Review 根因闭环，Hosted CI 与 Artifact 合同通过。

## 硬停止

出现范围漂移、需要公开任意路径、需要返回凭据/字段/内容/路径、需要写入/网络/子进程/线程/UI、
需要未批准依赖、基线漂移、哈希不一致、解析语义无法保持跨平台一致或验证失败时，返回最近稳定
checkpoint，并在 Issue `#98` 写明根因；不得扩大授权解释。

## 下一批准门

1. 完成四 crate 定向 tests/Clippy、格式、CI 合同、Release Audit、仓库政策、范围、隐私和禁止能力门禁。
2. 建立最终本地验证 checkpoint，普通 push 并创建关联 Issue `#98` 的非 Draft PR。
3. 逐条闭环 Review，对标准 CI、Performance observation 和 Artifact 合同做 Final Head 核验。
4. 全绿后只请求绑定 Final Head 的独立 Squash Merge 授权；当前禁止自行合并。
