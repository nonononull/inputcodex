# Issue #78：Gate 5 应用概览只读事实能力设计

## 控制状态

```yaml
status: local-verification-passed-control-checkpoint-pending
issue_ref: https://github.com/nonononull/inputcodex/issues/78
parity_exception_ref: https://github.com/nonononull/inputcodex/issues/77
parity_exception_decision_ref: https://github.com/nonononull/inputcodex/issues/77#issuecomment-5093148603
branch_ref: codex/issue-78-gate-5-application-overview
baseline_ref: a06a97fd59ce125306a13202c8f1a07656c797a0
baseline_tree: b669aa6610e976542a74f404ff4f87b36864816b
upstream_release: v1.2.43
upstream_commit: 5036ff056b5c629f19356396b17d6eeb70da664c
feature_id: feature.foundation-platform.application-overview
owner_decision_ref: codex-session-user-message-approved-issue-77-scheme-a-2026-07-27
owner_decision_local_time: 2026-07-27 23:16:01 +08:00
planning_authorization_ref: codex-session-user-message-approved-issue-78-written-design-planning-scope-2026-07-27
planning_authorization_local_time: 2026-07-27 23:34:39 +08:00
planning_scope_count: 3
planning_scope_hash: sha256:3f81a54c18c07b6889ad8219b0c1605e4b989f997117141fc2d4baae46ebbeb3
candidate_scope_count: 29
candidate_scope_hash: sha256:b46a940ff7dbf4bbc9bfdb69d04d755468e12409d9618837d8ff310490eb5ae4
candidate_scope_approval_ref: https://github.com/nonononull/inputcodex/issues/78#issuecomment-5093844784
candidate_scope_approval_status: approved
implementation_authorization: authorized
commit_push_pr_authorization: authorized
final_merge_authorization: pending-separate-gate
domain_checkpoint: f9db364b0fe21c105af427c878d00063e3f76886
application_checkpoint: a1c09e59b7552a5f6dac9ba79e0025301cde39f6
platform_checkpoint: 26751c5a72009008652ac48ef6d0af7a7753c332
parity_checkpoint: a4012ddbf7ab929d136e0ab5bc7ac1ae61284f7d
local_verification: passed
```

本文件是 Issue `#78` 的项目原生书面设计。项目所有者已经批准 Issue `#77` 方案 A，并通过 Issue `#78` 评论明确批准候选二十九路径、固定哈希、TDD 实施、本地轻量验证、Git checkpoint、普通提交、普通推送、非 Draft PR 与 Review/CI。当前实现只剩最终本地门禁、提交/推送和 Hosted Review/CI；最终 Squash Merge 仍须项目所有者针对 Final Head 单独授权。

Issue `#77` 最初评论中的 PowerShell 变量插值曾被错误写成字面量，已在 GitHub 原评论修正为本机时间 `2026-07-27 23:16:01 +08:00`；本设计只引用修正后的稳定证据，不复制错误文本。

## 目标

在纯 Rust 分层架构中实现 `feature.foundation-platform.application-overview` 的最小只读产品切片：

1. 检测受支持 Codex 应用是否已安装。
2. 已安装时返回现有 `CodexInstallation` 受保护安装引用。
3. 以有界、只读、可诊断方式读取已安装版本；无法安全取得版本时明确返回 `Unknown`，不得把已确认安装伪装为失败或未安装。
4. 返回 `inputcodex` 当前构建版本与本次采集时间。
5. 实时进程状态固定为 `NotObserved`，直到未来独立进程观察或生命周期 feature 落地。
6. Windows 与 macOS 使用同一领域快照、应用端口、加载状态和错误语义。

本切片必须直接消费 Issue `#75` 已建立的平台安装发现基础，但不得复用会同时解析 `CODEX_HOME` 与 `inputcodex` 状态根的完整 `PlatformPathsPort`。安装检测与无关状态目录绑定，会使合法应用概览因无关配置失败，这是本设计明确消除的根因耦合。

## 已批准语义

### 应用概览

`ApplicationOverview` 是一次只读事实采集结果，只包含安装状态、受保护安装引用、已安装版本或明确未知原因、`inputcodex` 当前构建版本、实时进程观察状态和采集时间。

### 历史记录与实时状态

- `LaunchHistoryRecord` 表示持久化的历史启动记录，不是实时进程事实。
- 本切片不读取 `latest-status.json`，也不新增 `LaunchHistoryRecord` 运行路径。
- `LiveProcessState` 当前只允许 `NotObserved`；不得根据历史文件、快捷方式、debug port、旧 PID 或上次启动结果推断 `Running`、`Stopped` 或等价状态。
- 未来历史记录 feature 若读取损坏文件，必须返回稳定失败；损坏内容不得静默视为“无历史记录”。该规则由 Issue `#77` 固定，但不借本切片提前实现历史读取。

### 聚合边界

快捷方式、更新、设置、诊断、应用生命周期、实时进程观察、Watcher、Provider、会话、插件、脚本和注入继续属于独立 feature，不进入本切片，也不得成为隐藏依赖。

## 领域合同

`inputcodex-domain` 新增独立模块 `application_overview.rs`，只保存跨平台语义，不读取环境、文件或时间。

```rust
pub const MAX_APPLICATION_VERSION_BYTES: usize = 128;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ApplicationVersionError {
    Empty,
    TooLong,
    ControlCharacter,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ApplicationVersion(String);

impl ApplicationVersion {
    pub fn new(value: String) -> Result<Self, ApplicationVersionError>;
    pub fn as_str(&self) -> &str;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InstalledVersionUnknownReason {
    MetadataMissing,
    MetadataUnreadable,
    MetadataInvalid,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InstalledVersion {
    Known(ApplicationVersion),
    Unknown(InstalledVersionUnknownReason),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InstallationState {
    Installed {
        installation: CodexInstallation,
        version: InstalledVersion,
    },
    NotInstalled,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LiveProcessState {
    NotObserved,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CollectedAtUnixMs(u64);

impl CollectedAtUnixMs {
    pub const fn new(value: u64) -> Self;
    pub const fn value(self) -> u64;
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ApplicationOverview {
    installation: InstallationState,
    inputcodex_version: ApplicationVersion,
    live_process_state: LiveProcessState,
    collected_at: CollectedAtUnixMs,
}

impl ApplicationOverview {
    pub const fn new(
        installation: InstallationState,
        inputcodex_version: ApplicationVersion,
        live_process_state: LiveProcessState,
        collected_at: CollectedAtUnixMs,
    ) -> Self;
    pub const fn installation(&self) -> &InstallationState;
    pub const fn inputcodex_version(&self) -> &ApplicationVersion;
    pub const fn live_process_state(&self) -> LiveProcessState;
    pub const fn collected_at(&self) -> CollectedAtUnixMs;
}
```

### 版本值规则

- 去除首尾空白后不能为空。
- UTF-8 字节长度不得超过 `128`。
- 不得包含 Unicode 控制字符。
- 已知版本保留来源文本，不进行跨来源数值重排或版本大小比较。
- 已知版本可以显示；绝对路径继续由 `PrivatePath` 固定输出 `PrivatePath(<redacted>)`。

## 应用合同

`inputcodex-application` 新增独立请求、端口和同步用例；不修改既有 `LoadCoordinator` 状态机。

```rust
#[derive(Clone, Default, PartialEq, Eq)]
pub struct ApplicationOverviewRequest {
    explicit_application_path: Option<PathBuf>,
}

impl ApplicationOverviewRequest {
    pub const fn new(explicit_application_path: Option<PathBuf>) -> Self;
    pub fn explicit_application_path(&self) -> Option<&Path>;
}

pub trait ApplicationOverviewPort {
    fn load(
        &self,
        request: &ApplicationOverviewRequest,
    ) -> Result<ApplicationOverview, ApplicationError>;
}

#[derive(Clone)]
pub struct LoadApplicationOverview<P> {
    port: P,
}

impl<P> LoadApplicationOverview<P> {
    pub const fn new(port: P) -> Self;
    pub const fn port(&self) -> &P;
}

impl<P: ApplicationOverviewPort> LoadApplicationOverview<P> {
    pub fn execute(
        &self,
        request: &ApplicationOverviewRequest,
    ) -> LoadCompletion<ApplicationOverview>;
}
```

`ApplicationOverviewRequest` 的自定义 `Debug` 只能把显式路径显示为 `Some("<redacted>")` 或 `None`。

| 场景 | 外部结果 |
| --- | --- |
| 已安装且版本已知 | `Ready` + `Installed` + `Known` |
| 已安装但版本无法安全读取 | `Ready` + `Installed` + `Unknown` |
| 未发现受支持安装 | `Ready` + `NotInstalled` |
| 未执行实时进程观察 | 任一成功快照均为 `NotObserved` |
| 平台或安装发现失败 | `Failed(ApplicationError)` |

本 feature 不使用 `LoadCompletion::Empty`。未安装是一个已完成、可审计的事实快照，不是缺失响应；成功快照内部也不再嵌套第二套 `Failed` 状态。

请求标识、取消、过期结果与状态回退继续使用现有 `RequestId`、`LoadCoordinator<ApplicationOverview>`、`TransitionOutcome`。本切片不启动后台线程；调用方只能围绕有界同步采集应用现有状态机，取消后的旧结果不得覆盖新请求。

## 平台架构

### 安装发现专用入口

`inputcodex-platform` 将现有平台候选逻辑暴露为 crate 内部安装发现入口：

```rust
pub(crate) fn resolve_installation_system(
    explicit_application_path: Option<&Path>,
    probe: &impl PathProbe,
) -> Result<Option<CodexInstallation>, ApplicationError>;
```

Windows 与 macOS 各自实现同名内部函数；`PathProbe`、`SystemPathProbe` 和两个平台子模块只扩大到 `pub(crate)`，不得成为公开 API。完整 `SystemPlatformPaths` 与新的 `SystemApplicationOverview` 都调用该入口，每次请求只执行一次安装发现。

该重构必须保持 Issue `#75` 的候选顺序、安全路径规则和错误结果不变：

- Windows 固定查询三个包家族，并按数值版本、身份顺序、根路径稳定排序；随后最多检查三个 standalone 根。
- macOS 固定检查系统与用户两个根、每根四个应用名，系统根优先。
- 显式路径优先，非法显式路径返回 `EXPLICIT_CODEX_PATH_INVALID`，不得自动回退。
- 禁止管理器自身路径、相对路径和任意可执行文件结构。

新的概览入口不得调用完整 `PlatformPathsPort`，不得读取或验证 `CODEX_HOME`、`inputcodex` 状态根、设置路径、历史状态路径或日志路径。

### 平台适配器

`inputcodex-platform` 对外新增：

```rust
#[derive(Debug, Clone, Copy, Default)]
pub struct SystemApplicationOverview;

impl ApplicationOverviewPort for SystemApplicationOverview {
    fn load(
        &self,
        request: &ApplicationOverviewRequest,
    ) -> Result<ApplicationOverview, ApplicationError>;
}
```

不支持的平台返回 `APPLICATION_OVERVIEW_UNSUPPORTED`。Windows/macOS 适配器完成安装发现、版本解析、构建版本校验和时间采集后，一次性构造领域快照。

## Windows 版本合同

Windows 版本解析按以下固定顺序执行，命中即停止：

1. 只检查 `application_root` 与最多一个结构父目录；当末级为 `app` 或 `bin` 时，元数据根固定为其父目录，否则为 `application_root`。
2. 从受控包目录名解析 `OpenAI.Codex_`、`OpenAI.CodexBeta_` 或 `OpenAI.ChatGPT-Desktop_` 后的首个版本段。
3. 若元数据根目录名本身是至少两段的纯数字点分版本，则直接使用该名称。
4. 仍未命中时只读取一次 `<metadata_root>/version`，最多 `256` 字节。

禁止 `canonicalize`、递归扫描目录、搜索 PATH/注册表/任意文件名、执行二进制或 shell、读取第二个版本文件，以及因版本问题否定已确认安装。

文件不存在或无受支持元数据映射为 `MetadataMissing`；权限或其他 I/O 错误映射为 `MetadataUnreadable`；超长、空白、控制字符或非法结构映射为 `MetadataInvalid`。

## macOS 版本合同

macOS 只读取一次：

```text
<application_root>/Contents/Info.plist
```

硬上限为 `65536` 字节；实现必须使用有界读取，并在读取 `65537` 字节时立即判定超限。只支持 UTF-8 XML 文本：

1. `CFBundleShortVersionString` 优先；
2. 缺失时回退 `CFBundleVersion`；
3. 取得的字符串继续经过统一 `ApplicationVersion` 校验。

文件不存在或两个键都缺失映射为 `MetadataMissing`；权限、I/O 或非 UTF-8/binary plist 映射为 `MetadataUnreadable`；超限、空值、控制字符或不能形成安全文本版本映射为 `MetadataInvalid`。版本未知仍返回已安装成功快照。

禁止调用 `defaults`、`plutil`、shell、Objective-C runtime、外部 XML 库或后台任务。

## 时间与构建版本

- `inputcodex` 版本来自编译期 `env!("CARGO_PKG_VERSION")`，并经过同一 `ApplicationVersion` 校验。
- 每次成功采集只调用一次 `SystemTime::now()`，转换为 Unix epoch 毫秒并检查 `u64` 范围。
- 系统时间早于 epoch 或超出范围返回 `APPLICATION_OVERVIEW_TIME_UNAVAILABLE`，不得填入 `0` 或伪造时间。
- 编译期版本意外不满足领域规则时返回 `APPLICATION_OVERVIEW_BUILD_VERSION_INVALID`，不得 panic。

## 稳定错误

| 诊断码 | `ErrorKind` | 语义 |
| --- | --- | --- |
| `APPLICATION_OVERVIEW_UNSUPPORTED` | `Unsupported` | 当前目标平台不是 Windows 或 macOS。 |
| `EXPLICIT_CODEX_PATH_INVALID` | `Unavailable` | 调用方给出的显式安装路径不满足 Issue `#75` 安全合同。 |
| `APPLICATION_OVERVIEW_DISCOVERY_FAILED` | `Internal` | 平台安装发现 API 或必要发现根发生不可恢复失败。 |
| `APPLICATION_OVERVIEW_TIME_UNAVAILABLE` | `Internal` | 无法生成合法 Unix 毫秒采集时间。 |
| `APPLICATION_OVERVIEW_BUILD_VERSION_INVALID` | `Internal` | 当前构建版本违反领域版本值规则。 |

版本元数据的缺失、不可读和非法不进入 `ApplicationError`，而进入 `InstalledVersion::Unknown`；因为安装事实已确认，版本质量问题必须可见但不得扩大为整个概览失败。

所有错误和 `Debug` 输出禁止包含绝对路径、用户名、用户目录、环境变量值或文件内容。

## 性能与 I/O 上限

| 项目 | 硬上限 |
| --- | --- |
| 单次安装发现 | `1` 次 |
| Windows 包家族 | 固定 `3` 个 |
| Windows standalone 根 | 固定 `3` 个 |
| macOS 应用候选 | 固定 `2 × 4 = 8` 个 |
| Windows 版本文件 | 最多 `1` 个、`256` 字节 |
| macOS `Info.plist` | 最多 `1` 个、`65536` 字节 |
| 系统时间读取 | `1` 次 |
| 网络、写入、缓存、后台线程、shell、`unsafe` | `0` |

任何实现若需要突破上述上限，必须停止并建立新的书面决策；不得用“兼容更多安装形态”为理由扩大扫描面。

## Parity 合同调整

实施后只允许对 `feature.foundation-platform.application-overview` 做以下修正：

1. 状态从 `unassessed` 更新为 `implemented`。
2. Windows/macOS 语义改为“只读安装事实、版本、构建版本、采集时间与 `not-observed` 实时状态”。
3. 决策引用增加 Issue `#77` 与 Issue `#78`。
4. 基线合同固定 `Ready(Installed)`、`Ready(NotInstalled)`、`InstalledVersion::Unknown`、`LiveProcessState::NotObserved`、五个稳定错误和有界 I/O。
5. `core-module:status` 不再归属于应用概览；其真实副作用固定为 `filesystem-read` 与 `filesystem-write`，归入未实现的 `feature.foundation-platform.application-lifecycle`，防止历史状态成为本切片隐藏依赖。
6. `tauri-command:load_overview` 继续作为上游聚合证据，但只迁移本设计批准的只读子集；快捷方式、更新、设置、诊断和生命周期继续由各自 feature 承担。

不得借目录更新改变其他 feature 状态、预算、Release Audit、Ruleset 或上游缓存。

## TDD 验收矩阵

### 领域

- 空版本、超长版本和控制字符被拒绝。
- 已知/未知版本、已安装/未安装和 `NotObserved` 模型可稳定比较。
- `ApplicationOverview` 的 `Debug` 不泄露 `PrivatePath`。

### 应用

- 已安装、未安装和版本未知均映射为 `LoadCompletion::Ready`。
- 端口错误映射为 `LoadCompletion::Failed`。
- 本 feature 永不返回 `LoadCompletion::Empty`。
- 请求 `Debug` 对显式路径脱敏。
- 取消和过期结果继续遵守既有 `LoadCoordinator` 语义。

### 平台

- Windows 包目录、数字目录和单个 `version` 文件按固定优先级解析。
- Windows 版本文件缺失、不可读、超长或非法时保持已安装并返回对应 `Unknown`。
- macOS 短版本优先、构建版本回退；缺失、binary/非 UTF-8、超限和非法内容映射为对应 `Unknown`。
- 平台安装发现只执行一次，不读取 `CODEX_HOME`、状态根或 `latest-status.json`。
- 不支持平台和安装发现错误使用固定诊断码。

### Parity 与治理

- 应用概览状态、决策引用、源归属和副作用合同固定。
- `release_audit=current`、`requires_reaudit=false`。
- 仓库政策违规数为 `0`。
- 精确范围为批准路径集合，哈希复算一致。
- 改动不包含私人绝对路径、广告、远程推荐、遥测、网络、写入、shell、`unsafe` 或 UI。

## 候选完整实施范围

以下 `29` 路径按 `StringComparer.Ordinal` 升序，以 UTF-8 无 BOM、LF 分隔并保留末尾 LF 后计算，候选哈希为 `sha256:b46a940ff7dbf4bbc9bfdb69d04d755468e12409d9618837d8ff310490eb5ae4`：

```text
AGENTS.md
CONTEXT.md
README.md
build.md
crates/inputcodex-application/src/application_overview.rs
crates/inputcodex-application/src/lib.rs
crates/inputcodex-application/tests/application_overview.rs
crates/inputcodex-domain/src/application_overview.rs
crates/inputcodex-domain/src/lib.rs
crates/inputcodex-domain/tests/application_overview.rs
crates/inputcodex-parity/tests/catalog_repository.rs
crates/inputcodex-platform/src/application_overview.rs
crates/inputcodex-platform/src/application_overview/macos.rs
crates/inputcodex-platform/src/application_overview/windows.rs
crates/inputcodex-platform/src/lib.rs
crates/inputcodex-platform/src/platform_paths.rs
crates/inputcodex-platform/src/platform_paths/macos.rs
crates/inputcodex-platform/src/platform_paths/windows.rs
crates/inputcodex-platform/tests/application_overview.rs
docs/plans/2026-07-27-issue-78-gate-5-application-overview.md
docs/plans/PROJECT-MASTER-PLAN.md
docs/plans/sessions/2026-07-27-issue-78-gate-5-application-overview.md
docs/reports/issue-78-gate-5-application-overview.md
docs/workflows/2026-07-27-issue-78-gate-5-application-overview-runtime.md
err.md
parity/README.md
parity/contracts/foundation-platform.yml
parity/features/foundation-platform.yml
parity/features/source-index.yml
```

候选范围不包含 `Cargo.toml` 或 `Cargo.lock`，因为设计禁止新增依赖家族；也不包含 `upstream/`、`.github/workflows/`、Ruleset、预算、Release、Iced 或 AGOS 路径。

## 当前规划写入范围

本轮实际只允许以下 `3` 路径，规划哈希为 `sha256:3f81a54c18c07b6889ad8219b0c1605e4b989f997117141fc2d4baae46ebbeb3`：

```text
docs/plans/2026-07-27-issue-78-gate-5-application-overview.md
docs/plans/sessions/2026-07-27-issue-78-gate-5-application-overview.md
docs/workflows/2026-07-27-issue-78-gate-5-application-overview-runtime.md
```

## 工具与外部治理边界

- 仓库不存在 `.codegraph/`，本任务不得擅自初始化。
- 当前 GitNexus 未索引任何仓库；代码理解使用项目原生文档、`rg` 和直接读取完成。
- 当前会话没有可用的 `local_knowledge_lookup` 调用面；已通过项目计划、报告、Issue、Git 历史和上游只读快照完成等价本地查询。
- AGOS 仅为可选外部辅助。本任务不修改、修复或优化 AGOS，也不把 `docs/superpowers/*` 建为活动控制面；外部工具缺失不阻塞本项目原生 Issue/计划/验证链。

## 停止条件

出现任一条件立即停止，不得提交或推送：

- 上游最新正式 Release 不再是 `v1.2.43`，或提交不再是 `5036ff056b5c629f19356396b17d6eeb70da664c`。
- `origin/main` 不再是基线 `a06a97fd59ce125306a13202c8f1a07656c797a0`，且尚未重新审计影响。
- `release_audit` 不再为 `current` 或 `requires_reaudit` 不再为 `false`。
- 需要读取历史状态、枚举进程、观察 PID/debug port、写文件、联网、缓存或启动后台线程。
- 需要新增依赖、shell、`unsafe`、UI、Iced 或平台间不同应用合同。
- 实际路径集合或任一哈希漂移。
- 测试、Review 或 CI 失败根因未确定并解决。
- 请求 force push、删除/改写 `main`、修改 Ruleset、预算、Release、`upstream/` 或 AGOS。

## 下一控制门

候选范围和远端写入已获批准。下一步按 Runtime Workflow 完成最终本地轻量门禁、形成 Git checkpoint、普通推送并创建非 Draft PR；随后完成 Final Head Review、标准 CI、Windows/macOS 真实编译、非 required Performance observation 与 Artifact `0` 复核。全部根因和 Review 对话闭环后必须停止，等待项目所有者单独授权 Squash Merge。
