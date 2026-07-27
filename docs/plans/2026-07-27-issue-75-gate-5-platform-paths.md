# Issue #75：Gate 5 平台路径解析迁移设计

## 控制状态

```yaml
status: written-spec-awaiting-owner-review
issue_ref: https://github.com/nonononull/inputcodex/issues/75
parity_exception_ref: https://github.com/nonononull/inputcodex/issues/74
branch_ref: codex/issue-75-gate-5-platform-paths
baseline_ref: fc1683aabda4afb27ca333387ec954b6a405d2df
baseline_tree: d17a038fcb4fc986565f121283481eb38cdfbc33
upstream_release: v1.2.43
upstream_commit: 5036ff056b5c629f19356396b17d6eeb70da664c
feature_id: feature.foundation-platform.platform-paths
owner_decision_ref: codex-session-user-message-approved-full-platform-paths-design-2026-07-27
candidate_scope_count: 29
candidate_scope_hash: sha256:251f54063fafa368e5f134fd01d8a1b6ff3f1ff6f3b02a07b661aa5c0d6f523b
implementation_authorization: pending-owner-written-spec-review
final_merge_authorization: pending
```

本文件是 Issue `#75` 的书面设计规范。项目所有者已批准总体设计和 Issue `#74` 的路径安全策略，但尚未复核本文件与二十九路径候选范围。在复核完成前，只允许提交和推送本规范，不允许编写实现代码、创建 PR 或修改其他仓库文件。

## 目标

以纯 Rust 分层架构重新实现 `feature.foundation-platform.platform-paths`，为后续应用概览、生命周期、设置和会话能力提供稳定路径基础。Windows 与 macOS 必须通过同一应用层合同解析等价的 Codex 应用路径、Codex 数据根目录和 `inputcodex` 独立状态路径。

上游 `v1.2.43` 仅作为行为证据；不复制 Tauri/React 架构，不把上游源码作为运行时依赖，不夹带 UI、写入或其他功能。

## 统一术语

- **私有路径**：进程内部使用的绝对路径值；不得序列化或公开显示，自定义 `Debug` 只输出脱敏文本。
- **平台路径快照**：单次解析形成的稳定结果；不负责缓存、监听、创建目录或写文件。
- **Codex 数据根目录**：有效 `CODEX_HOME`，或未配置时用户主目录下的 `.codex`。
- **inputcodex 状态根目录**：Windows `%LOCALAPPDATA%\inputcodex`；macOS `$HOME/Library/Application Support/inputcodex`。
- **未发现安装**：解析成功但没有受支持 Codex 应用，是正常空安装结果，不是系统错误。
- **安装来源**：`explicit`、`windows-package`、`windows-standalone`、`macos-system-applications`、`macos-user-applications`。

## 范围

### 包含

1. 受保护绝对路径值对象、安装来源和稳定快照模型。
2. 平台路径请求、应用端口和解析用例。
3. Windows 注册包与 `%LOCALAPPDATA%` 独立安装探测。
4. macOS 系统与用户 `Applications` 探测。
5. `CODEX_HOME`、用户主目录和 `inputcodex` 状态根目录解析。
6. 设置、最新状态和诊断日志文件路径派生，但不创建路径。
7. 功能目录、合同和来源索引的真实副作用修正。
8. Windows/macOS 定向测试、隐私测试和 hosted CI 证据。

### 不包含

1. Iced 视图、交互、临时 UI 或 Gemini 视觉工作。
2. 目录创建、文件写入、设置迁移、日志写入、SQLite、网络、遥测或缓存。
3. 应用概览、版本读取、AppUserModelID、启动、停止、Watcher 或安装更新。
4. Provider 导入路径、会话数据库定位或其他 feature 的隐藏实现。
5. 广告、远程推荐、注入脚本、Tauri/React 运行面。
6. 性能预算数值、公式、Ruleset、required check、Release、`upstream/` 或 AGOS 修改。

## 领域模型

### `PrivatePath`

- 构造时验证非空和绝对路径。
- 内部持有 `PathBuf`，不依赖平台 crate、异步运行时或 Iced。
- 不实现 `Serialize`、`Display` 或自动派生 `Debug`。
- 自定义 `Debug` 固定输出 `PrivatePath(<redacted>)`。
- 提供受控 `as_path()` 供内部文件系统操作。

### `CodexInstallation`

包含 `application_root: PrivatePath`、`executable: PrivatePath` 和 `source: ApplicationInstallSource`。不包含版本、包用户模型 ID 或启动参数。

### `PlatformPathsSnapshot`

包含：

- `codex_home`
- `inputcodex_state_root`
- `settings_file`
- `latest_status_file`
- `diagnostic_log_file`
- `codex_installation: Option<CodexInstallation>`

派生文件名固定为 `settings.json`、`latest-status.json` 和 `inputcodex.log`。旧目录 `.codex-session-delete` 与旧日志名 `codex-plus.log` 不进入新产品。

## 应用层合同

- `PlatformPathsRequest` 只包含可选显式 Codex 应用路径。
- `PlatformPathsPort` 由 `inputcodex-application` 定义、`inputcodex-platform` 实现。
- 应用层不得直接读取环境变量、文件系统或 Windows API。
- 解析成功返回 `LoadCompletion::Ready`；未安装时快照安装字段为 `None`。
- 配置、用户目录、平台或系统读取错误返回 `Failed`，不得伪造 `Empty`。
- 调用方继续使用现有 `RequestId`、`LoadCoordinator`、取消和过期结果规则。
- 平台端口保持一次有界同步读取，不引入 Tokio、后台线程或无法回收的任务。

## 共同解析规则

1. 确认当前平台受支持。
2. 解析用户主目录和平台原生 `inputcodex` 状态根目录。
3. 解析 `CODEX_HOME`。
4. 验证可选显式应用路径；无效时直接失败，不自动回退。
5. 未提供显式路径时执行平台自动探测。
6. 派生设置、状态和诊断文件路径。
7. 返回不含公开路径文本的快照。

`CODEX_HOME` 规则：

- 未设置或仅空白：使用用户主目录下的 `.codex`。
- 非空：必须为绝对、已存在的目录，否则返回 `CODEX_HOME_INVALID`。
- 用户主目录不可用：返回 `USER_HOME_UNAVAILABLE`。
- 永不使用相对 `.codex`，也不创建目录。

## Windows 语义

1. 支持 `OpenAI.Codex`、`OpenAI.CodexBeta`、`OpenAI.ChatGPT-Desktop`。
2. 注册包版本按数值段比较；同版本按上述身份顺序保证确定性。
3. 注册包未发现时依次检查：
   - `%LOCALAPPDATA%\OpenAI\Codex\bin`
   - `%LOCALAPPDATA%\OpenAI\Codex`
   - `%LOCALAPPDATA%\Programs\OpenAI\Codex`
4. 只接受 `Codex.exe`、`ChatGPT.exe` 及大小写等价名称。
5. 任何组成部分命中 `inputcodex`、`Codex++`、`CodexPlusPlus` 或管理器名称时拒绝识别。
6. `LOCALAPPDATA` 缺失时返回 `INPUTCODEX_STATE_ROOT_UNAVAILABLE`，不得退到当前目录。

Windows 只直接依赖已在锁文件中的 `windows 0.58.0`，启用 `Win32_Foundation` 与 `Win32_Storage_Packaging_Appx`。不新增 `directories`、`dirs`、Tokio、Serde 或外部命令依赖。

## macOS 语义

1. 自动探测顺序为 `/Applications`，然后 `$HOME/Applications`。
2. 每个根目录按固定顺序检查 `Codex.app`、`OpenAI Codex.app`、`OpenAI.Codex.app`、`ChatGPT.app`。
3. 显式路径可以是受支持 `.app` 或其 `Contents/MacOS` 可执行文件。
4. 可执行文件优先使用应用包中的安全名称；没有可用名称时使用 `Contents/MacOS/Codex`。
5. 用户主目录缺失时返回 `USER_HOME_UNAVAILABLE`。
6. 不扫描整个磁盘，不访问网络，不调用 `find`、`mdfind` 或 shell。

## 错误与隐私

稳定错误码至少包含：

- `PLATFORM_PATHS_UNSUPPORTED`
- `EXPLICIT_CODEX_PATH_INVALID`
- `CODEX_HOME_INVALID`
- `USER_HOME_UNAVAILABLE`
- `INPUTCODEX_STATE_ROOT_UNAVAILABLE`
- `PLATFORM_PATHS_FAILED`

平台不支持使用 `ErrorKind::Unsupported`；必需环境或目录不可用使用 `Unavailable`；取消和超时继续使用现有分类。错误、`Debug`、测试输出、CI Summary 和 Review 证据不得包含真实绝对路径。

允许的可观测字段仅有请求标识、耗时、结果类别、是否发现安装、安装来源和稳定错误码。

当前行为合同漏记环境与平台目录读取。本任务将 `platform-paths` 及对应 `app_paths`、`codex_home`、`paths` 来源入口从仅 `filesystem-read` 修正为 `filesystem-read + process-read`，并引用 Issue `#74`。

## 性能约束

1. 只读取固定环境键和固定候选根，不递归扫描磁盘。
2. Windows 注册包查询每次请求最多一次；首版不加全局缓存。
3. 包版本使用整数段解析，不做大范围正则扫描。
4. macOS 最多检查两个根目录和四个应用名称。
5. 不新增异步运行时、后台线程、网络或依赖家族。
6. 不修改预算指标；PR 和合并后主干继续执行现有非 required observation。

## TDD 顺序

1. **领域 RED**：拒绝相对路径，`Debug` 脱敏，快照不暴露序列化路径。
2. **应用 RED**：未安装为 Ready+None，显式无效路径失败，旧请求和取消结果不能覆盖新状态。
3. **共同规则 RED**：空白 `CODEX_HOME`、无效配置、用户目录缺失、状态路径和派生文件名。
4. **Windows RED**：三种身份、数值版本排序、独立安装、管理器路径拒绝和缺失 `LOCALAPPDATA`。
5. **macOS RED**：系统/用户目录顺序、四种名称、`.app` 归一化和未安装空结果。
6. **最小 GREEN**：只实现测试要求；平台差异限制在 `inputcodex-platform`，不建立未来框架。

## 验证合同

分支建立前的基线为 `fc1683aabda4afb27ca333387ec954b6a405d2df`。本机时间 `2026-07-27 16:59:34 +08:00` 已运行：

```powershell
cargo test --locked --offline --ignore-rust-version `
  -p inputcodex-domain `
  -p inputcodex-application `
  -p inputcodex-platform `
  -p inputcodex-parity
```

application `3/3`、domain `1/1`、platform `1/1`，parity 所有测试目标通过。

实施阶段本地只运行相关 crate 的定向 `test/check/clippy`、`cargo fmt --check`、parity repository test、仓库政策验证和 `git diff --check`。禁止本地 Iced 和全 Workspace 重型编译。

GitHub-hosted 验证要求标准 CI、Windows/macOS 真实编译测试和 Performance Baseline 全部成功；成功 Artifact 数为 `0`；Review 对话全部记录根因、处理方式和验证证据。

## 二十九路径候选范围

以下路径按 Windows PowerShell `Sort-Object` 排序；UTF-8 无 BOM、LF 连接并追加末尾 LF 后，SHA-256 为 `sha256:251f54063fafa368e5f134fd01d8a1b6ff3f1ff6f3b02a07b661aa5c0d6f523b`：

1. `AGENTS.md`
2. `build.md`
3. `Cargo.lock`
4. `Cargo.toml`
5. `CONTEXT.md`
6. `crates/inputcodex-application/src/lib.rs`
7. `crates/inputcodex-application/src/platform_paths.rs`
8. `crates/inputcodex-application/tests/platform_paths.rs`
9. `crates/inputcodex-domain/src/lib.rs`
10. `crates/inputcodex-domain/src/platform_paths.rs`
11. `crates/inputcodex-domain/tests/platform_paths.rs`
12. `crates/inputcodex-parity/tests/catalog_repository.rs`
13. `crates/inputcodex-platform/Cargo.toml`
14. `crates/inputcodex-platform/src/lib.rs`
15. `crates/inputcodex-platform/src/platform_paths.rs`
16. `crates/inputcodex-platform/src/platform_paths/macos.rs`
17. `crates/inputcodex-platform/src/platform_paths/windows.rs`
18. `crates/inputcodex-platform/tests/platform_paths.rs`
19. `docs/plans/2026-07-27-issue-75-gate-5-platform-paths.md`
20. `docs/plans/PROJECT-MASTER-PLAN.md`
21. `docs/plans/sessions/2026-07-27-issue-75-gate-5-platform-paths.md`
22. `docs/reports/issue-75-gate-5-platform-paths.md`
23. `docs/workflows/2026-07-27-issue-75-gate-5-platform-paths-runtime.md`
24. `err.md`
25. `parity/contracts/foundation-platform.yml`
26. `parity/features/foundation-platform.yml`
27. `parity/features/source-index.yml`
28. `parity/README.md`
29. `README.md`

任何新增、删除、重命名或移出集合的路径都必须重新计算哈希并取得项目所有者批准。路径在范围内也不代表可以进行与 Issue `#75` 无关的修改。

## 交付链

1. 项目所有者复核本规范、二十九路径和候选哈希。
2. 使用 `superpowers:writing-plans` 形成项目原生 Session Plan 与 Runtime Workflow。
3. 在 Issue `#75` 回写正式 `scope_hash`、允许操作和禁止操作。
4. TDD RED checkpoint、最小 GREEN、定向重构和本地轻量验证。
5. 普通推送、非 Draft PR、Review、标准 CI、双平台验证和性能 observation。
6. 项目所有者单独授权 Squash Merge。
7. 合并后核验主干 CI、Performance、单父提交、签名、Tree 和 Artifact。

## 停止条件

- 上游正式 Release 变化或 `release_audit` 不再为 `current`。
- 需要 UI、写入、网络、缓存、后台线程或新的依赖家族。
- 需要迁移应用概览、设置、会话、生命周期或其他 feature。
- 实际路径集合或 `scope_hash` 漂移。
- Windows/macOS 无法实现等价语义，或出现新的副作用、错误语义争议。
- Review/CI/性能失败根因未解决。
- 请求 Force Push、删除 `main`、修改 Ruleset、预算、`upstream/` 或 AGOS。

## 书面规范自审

```yaml
placeholder_scan: passed
internal_consistency: passed
scope_check: passed-single-feature
ambiguity_check: passed
ui_boundary: passed-no-ui
privacy_boundary: passed-no-public-absolute-path
dependency_check: passed-no-new-package-family
implementation_started: false
```

自审未发现未完成占位标记、占位值、矛盾要求或隐藏的第二功能。下一步只能等待项目所有者复核本文件；未获批准不得进入实现计划和 TDD。
