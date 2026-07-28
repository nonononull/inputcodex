# Issue #81 Gate 5 版本与启动意图实施设计

## 控制元数据

```yaml
task_id: issue-81-gate-5-version-startup
tracking_issue_ref: https://github.com/nonononull/inputcodex/issues/81
approved_decision_ref: https://github.com/nonononull/inputcodex/issues/80#issuecomment-5095090077
approved_scope_ref: https://github.com/nonononull/inputcodex/issues/81#issuecomment-5100612634
baseline_main: ef69494d92c7c461b0cb858e95f6838404ae1a61
branch: codex/issue-81-gate-5-version-startup
release_truth: v1.2.43@5036ff056b5c629f19356396b17d6eeb70da664c
planning_scope_count: 3
planning_scope_hash: sha256:707b8a43199ffb69b71f18a9681e9432b02c94b8533dd7dcbc4cf2b1ad758579
candidate_scope_count: 23
candidate_scope_hash: sha256:c1ef2c00a445dd2bd60dc5f5b375cb27d1e467a3d457d7eb53b7ec82a304aafe
implementation_authorization: authorized
commit_push_pr_authorization: authorized
final_merge_authorization: pending-separate-gate
agos_status: bypassed-project-native-control-plane
```

## 目标

在不引入 UI、网络、文件写入、新依赖或旧产品运行语义的前提下，迁移
`feature.foundation-platform.version-and-startup`：

1. 公开 `inputcodex` 编译期版本；
2. 将当前进程启动请求解析为明确的 `StartupIntent`；
3. 对非法显式环境值返回稳定错误；
4. 通过既有加载协调器保留取消、过期结果和错误隔离语义；
5. 将功能目录与行为合同更新为 `implemented`。

## 已批准语义

- 版本来源只能是 `CARGO_PKG_VERSION`，并复用既有 `ApplicationVersion`。
- `StartupIntent` 只有 `Default` 与 `ShowUpdate`。
- 精确命令行参数 `--show-update` 产生 `ShowUpdate`。
- `INPUTCODEX_SHOW_UPDATE` 未设置、空值或 `0` 产生 `Default`，`1` 产生
  `ShowUpdate`。
- 其他非空值，包括无法转换为 Unicode 的值，返回
  `INVALID_STARTUP_OPTION`。
- 非法显式环境值优先失败，即使命令行同时含 `--show-update` 也不得掩盖。
- `CODEX_PLUS_SHOW_UPDATE` 不得进入生产运行语义。
- 本功能不打开界面、不联网、不执行更新、不写入、不缓存、不启动线程。
- Windows 与 macOS 行为一致；其他目标返回
  `VERSION_AND_STARTUP_UNSUPPORTED`。

## 分层设计

### Domain

新增 `version_startup` 模块：

- `StartupIntent`：稳定表达默认启动与请求展示更新的意图；
- `VersionStartupSnapshot`：组合 `ApplicationVersion` 与 `StartupIntent`；
- 只保存经过验证的领域值，不保存原始命令行或环境变量。

`ApplicationVersion` 继续由 `application_overview` 模块定义并从 crate 根导出；
本 Issue 不移动类型、不重构既有应用概览代码。

### Application

新增：

- 零敏感输入的 `VersionStartupRequest`；
- `VersionStartupPort`；
- `LoadVersionStartup<P>` 同步用例。

用例只返回 `LoadCompletion::Ready` 或 `LoadCompletion::Failed`，不得返回
`Empty`。共享错误模型新增 `ErrorKind::InvalidInput` 与
`ApplicationError::invalid_input`，用于精确表示非法显式启动配置。

### Platform

新增单文件 `version_startup` 适配器：

- `SystemVersionStartup` 实现 `VersionStartupPort`；
- Windows/macOS 读取 `std::env::args_os()` 与
  `std::env::var_os("INPUTCODEX_SHOW_UPDATE")`；
- 解析逻辑使用可注入参数的纯函数，供 Linux 质量测试和双平台共享测试；
- 不保存或格式化非法原始值，错误中只返回稳定诊断码；
- 非 Windows/macOS 的系统入口明确返回 Unsupported。

不创建 Windows/macOS 子模块，因为本切片没有平台专属系统调用。

### Parity

- `foundation-platform.yml`：状态改为 `implemented`，写入 Issue #80/#81 决策引用；
- `foundation-platform` 合同：固定 Ready/Failed、非法值、禁止能力和平台语义；
- 仓库测试：验证目录、合同、旧变量禁止、输出不产生 `Empty` 与副作用边界；共享合同 schema 仍枚举六种协调器状态，并明确 `LoadCompletion::Empty` 对本功能不可达；
- `source-index.yml` 已正确映射三个入口，本 Issue 不修改。

### 控制面

- 同步 Issue #78 / PR #79 已合并事实；
- 将 Issue #81 标记为当前第三个 Gate 5 产品切片；
- 新增本 Issue 的构建入口、Session Plan、Runtime Workflow 与实施报告；
- `err.md` 仅记录本 Issue 实际发生且已完成根因闭环的错误。

## TDD 顺序

1. Domain RED：缺少启动意图和快照类型；
2. Domain GREEN：最小领域模型与边界测试；
3. Application RED：缺少 Port、用例和 InvalidInput；
4. Application GREEN：Ready/Failed 与协调器过期规则；
5. Platform RED：默认、参数、环境、非法值、非 Unicode、旧变量禁止；
6. Platform GREEN：纯解析函数与系统适配器；
7. Parity RED：目录仍为 unassessed、合同仍为旧语义；
8. Parity GREEN：状态、合同、决策引用与禁止能力；
9. 控制面收口和完整轻量验证。

## 候选完整实施范围

```text
AGENTS.md
CONTEXT.md
README.md
build.md
crates/inputcodex-application/src/lib.rs
crates/inputcodex-application/src/version_startup.rs
crates/inputcodex-application/tests/version_startup.rs
crates/inputcodex-domain/src/lib.rs
crates/inputcodex-domain/src/version_startup.rs
crates/inputcodex-domain/tests/version_startup.rs
crates/inputcodex-parity/tests/catalog_repository.rs
crates/inputcodex-platform/src/lib.rs
crates/inputcodex-platform/src/version_startup.rs
crates/inputcodex-platform/tests/version_startup.rs
docs/plans/2026-07-28-issue-81-gate-5-version-startup.md
docs/plans/PROJECT-MASTER-PLAN.md
docs/plans/sessions/2026-07-28-issue-81-gate-5-version-startup.md
docs/reports/issue-81-gate-5-version-startup.md
docs/workflows/2026-07-28-issue-81-gate-5-version-startup-runtime.md
err.md
parity/README.md
parity/contracts/foundation-platform.yml
parity/features/foundation-platform.yml
```

规范化后共 `23` 路径，哈希为
`sha256:c1ef2c00a445dd2bd60dc5f5b375cb27d1e467a3d457d7eb53b7ec82a304aafe`。

## 禁止范围

- UI、Iced 展示、Tauri、WebView、TypeScript 或 JavaScript；
- 网络请求、更新检查、下载、安装或进程控制；
- 文件、数据库、注册表或剪贴板读写；
- 缓存、后台线程、定时器或新依赖；
- `upstream/`、Release、Ruleset、性能预算或 AGOS 修改；
- `CODEX_PLUS_SHOW_UPDATE` 兼容入口；
- force push、Merge Commit 或 Rebase Merge。

## 验收标准

- 四 crate 定向测试与 all-targets Clippy 通过；
- `cargo fmt --all -- --check` 通过；
- CI 合同、Release Audit、Repository Policy 通过；
- 实际变更严格位于批准的 `23` 路径；
- 生产代码不包含旧变量、网络、写入、线程或 `unsafe`；
- Windows/macOS Hosted CI 和非 required Performance observation 通过；
- 成功 Run Artifact 数为 `0`；
- Review 对话全部完成根因闭环；
- 最终停在项目所有者单独 Squash Merge 授权门。

## 实施进度

| 节点 | 状态 | 证据 |
| --- | --- | --- |
| Planning | completed | `72b03b1af1fd7ab1984a481af1dd30a20879bb43` |
| Domain RED→GREEN | completed | `391bfe9db9348518600e14c912333f221c3cfaca` |
| Application RED→GREEN | completed | `1eafa90866124e4c281eba127fd48bb701817ebd` |
| Platform RED→GREEN | completed | `f992890611ff86f1fe6ccf5f0dd86e19d0fb07de` |
| Parity RED→GREEN | completed | `bee9dcb97fe9c790f45082cb23f0286c89b1d815` |
| 控制面与本地轻量验证 | completed | 四 crate、Clippy、fmt、CI `35/35`、Audit、Policy、范围和禁止面全绿 |
| 最终 Git checkpoint | in-progress | 当前分支 `codex/issue-81-gate-5-version-startup` |
| 普通推送、非 Draft PR、Hosted Review/CI | pending | 最终 Squash Merge 仍为独立授权门 |
