# Issue #78：Gate 5 应用概览只读事实实施报告

## 当前结论

Issue `#78` 已在批准的二十九路径内完成 Domain、Application、Windows/macOS Platform、Parity RED→GREEN、项目控制面、最终本地轻量门禁和控制面 checkpoint。产品切片只返回安装事实、已安装版本或明确未知原因、`inputcodex` 构建版本、采集时间与 `LiveProcessState::NotObserved`；不读取历史启动状态，不枚举进程，不写文件，不联网，不缓存，不启动线程，也不引入 UI、Iced、新依赖、广告、远程推荐、遥测、shell 或 `unsafe`。当前只剩普通推送、非 Draft PR、Hosted Review/CI 与最终 Squash Merge；最终合并必须由项目所有者针对 Final Head 单独授权。

## 基线与批准

- 基线提交：`a06a97fd59ce125306a13202c8f1a07656c797a0`
- 基线 tree：`b669aa6610e976542a74f404ff4f87b36864816b`
- 上游功能真源：`v1.2.43 @ 5036ff056b5c629f19356396b17d6eeb70da664c`
- 一致性例外：Issue `#77`，已按 `COMPLETED` 关闭
- 实施 Issue：Issue `#78`
- 二十九路径哈希：`sha256:b46a940ff7dbf4bbc9bfdb69d04d755468e12409d9618837d8ff310490eb5ae4`
- 范围与远端写入批准：<https://github.com/nonononull/inputcodex/issues/78#issuecomment-5093844784>
- 最终 Squash Merge：待项目所有者针对 Final Head 单独授权

## 已实现语义

### Domain

- `ApplicationVersion` 拒绝空值、超过 `128` 字节和控制字符，并去除首尾空白。
- `InstallationState` 只区分 `Installed` 与 `NotInstalled`；已安装版本使用 `InstalledVersion::{Known, Unknown}`。
- `InstalledVersionUnknownReason` 固定为元数据缺失、不可读或非法，版本问题不否定已确认安装。
- `LiveProcessState` 当前只允许 `NotObserved`；`ApplicationOverview` 的路径调试输出继续由 `PrivatePath` 脱敏。

### Application

- `ApplicationOverviewRequest` 只携带可选显式应用路径，并在 `Debug` 中输出 `<redacted>`。
- `ApplicationOverviewPort` 隔离平台读取；`LoadApplicationOverview` 将已安装已知版本、已安装未知版本和未安装全部映射为 `Ready`。
- 本功能永不把正常未安装映射为 `Empty`；端口错误保持稳定 `Failed`，取消与过期结果复用既有 `LoadCoordinator` 语义。

### Platform

- 安装专用入口从完整平台路径聚合中外科式提取；显式路径不再依赖 `CODEX_HOME`、用户状态根或其他无关路径。
- Windows 按受控包目录、数字目录和单个 `version` 文件的优先级解析版本；最多读取一个文件和 `256` 字节。
- macOS 最多读取一个 `Contents/Info.plist` 和 `65536` 字节，短版本优先、构建版本回退；binary、非 UTF-8、超限和非法内容使用明确未知原因。
- 每次成功快照只采集一次系统时间，构建版本来自编译期 `CARGO_PKG_VERSION`；实时进程状态固定为 `NotObserved`。

### Parity

- `feature.foundation-platform.application-overview` 已更新为 `implemented`，决策引用固定 Issue `#77/#78`。
- `core-module:status` 已从应用概览移入未实现的应用生命周期 feature，并记录真实 `filesystem-read`、`filesystem-write` 副作用。
- 应用概览合同固定三个 Ready 成功面、`InstalledVersion::Unknown`、`LiveProcessState::NotObserved`、五个稳定错误和双平台 I/O 上限。
- 历史状态、写入、网络、广告和远程推荐均不进入该产品切片。

## 稳定错误

- `APPLICATION_OVERVIEW_UNSUPPORTED`
- `EXPLICIT_CODEX_PATH_INVALID`
- `APPLICATION_OVERVIEW_DISCOVERY_FAILED`
- `APPLICATION_OVERVIEW_TIME_UNAVAILABLE`
- `APPLICATION_OVERVIEW_BUILD_VERSION_INVALID`

版本元数据缺失、不可读或非法不进入整体 `ApplicationError`，而进入 `InstalledVersion::Unknown`。

## TDD 证据

| 阶段 | RED/GREEN 证据 | Git checkpoint |
| --- | --- | --- |
| Domain | 版本校验、安装/未安装、版本未知、`NotObserved`、采集时间和路径脱敏 | `f9db364b0fe21c105af427c878d00063e3f76886` |
| Application | 三个 Ready 成功面、Failed、非 Empty、请求脱敏、取消与过期结果 | `a1c09e59b7552a5f6dac9ba79e0025301cde39f6` |
| 平台安装入口与 Overview | 显式路径不依赖完整状态根；Windows/macOS 单文件有界读取、版本 Unknown 边界和统一系统适配器 | `26751c5a72009008652ac48ef6d0af7a7753c332` |
| Parity | 旧 `unassessed` RED；目录、合同和 source-index 更新后定向与完整 crate GREEN | `a4012ddbf7ab929d136e0ab5bc7ac1ae61284f7d` |

## 本地验证状态

```yaml
status: local-verification-passed-pr-pending
scope_count: 29
scope_hash: sha256:b46a940ff7dbf4bbc9bfdb69d04d755468e12409d9618837d8ff310490eb5ae4
four_crate_test: passed
four_crate_clippy: passed
fmt: passed-after-root-cause-fix
ci_contract: 35/35 passed
release_audit: current
repository_policy: passed with 0 violations
scope_and_privacy: passed; changed=29
history_and_forbidden_capabilities: passed
upstream_fresh: v1.2.43@5036ff056b5c629f19356396b17d6eeb70da664c
control_checkpoint: 27e8599edff2f0ddeacc24d1d188cfbeb6d85c68
pr_ref: pending
review_ci: pending
final_merge: not-authorized
```

`cargo fmt --all -- --check` 的首次组合命令暴露了纯格式 diff；根因是 PowerShell 不会因原生命令非零自动终止，后续成功脚本覆盖了最终退出码。已按 rustfmt 给出的最小 diff 修正并单独复验退出码为 `0`；最终门禁对每条原生命令显式检查 `$LASTEXITCODE`。

## 后续门禁

1. 完成最终四 crate、格式、CI 合同、Release Audit、Repository Policy、范围、隐私、历史依赖、禁止能力和 Git 空白验证。
2. 形成按层 Git checkpoint，普通推送分支并创建关联 Issue `#77/#78` 的非 Draft PR。
3. 核验 Final Head Review、标准 CI、Windows/macOS 真实编译、非 required Performance observation 与成功 Artifact `0`。
4. 全部门禁和 Review 对话根因闭环后停止，等待项目所有者单独授权 Squash Merge。
