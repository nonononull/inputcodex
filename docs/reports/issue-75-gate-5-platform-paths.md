# Issue #75：Gate 5 平台路径解析实施报告

## 当前结论

首个 Gate 5 产品切片已在批准的三十路径内完成领域、应用、Windows/macOS 平台适配器、Parity RED→GREEN 与最终本地轻量门禁。实现只读取平台、环境与文件系统元数据，不创建目录、不写文件、不联网、不缓存、不运行后台线程，也不引入 UI、Iced、广告、远程推荐、注入脚本或 `unsafe`。当前只剩控制面提交、非 Draft PR、Hosted Review/CI 和最终 Squash Merge 授权门。

## 基线与批准

- 基线提交：`fc1683aabda4afb27ca333387ec954b6a405d2df`
- 基线 tree：`d17a038fcb4fc986565f121283481eb38cdfbc33`
- 上游功能真源：`v1.2.43 @ 5036ff056b5c629f19356396b17d6eeb70da664c`
- 功能 ID：`feature.foundation-platform.platform-paths`
- 路径安全例外：Issue `#74`，已按 `COMPLETED` 关闭
- 实施 Issue：Issue `#75`
- 三十路径哈希：`sha256:ae5e0f5143355feee9b280da7c44fdd5cdf759ec2ae71fc69167040bf302cb37`
- 扩围批准证据：`https://github.com/nonononull/inputcodex/issues/75#issuecomment-5092021020`
- 最终 Squash Merge：待项目所有者针对 Final Head 单独授权

## 分层实现

### Domain

- `PrivatePath` 只接受绝对路径，不实现 `Display`、Serde 或自动派生 `Debug`。
- 自定义 `Debug` 固定输出脱敏值。
- `CodexInstallation` 与 `PlatformPathsSnapshot` 只保存受保护路径、安装来源和稳定派生文件。

### Application

- `PlatformPathsRequest` 只携带可选显式 Codex 应用路径。
- `PlatformPathsPort` 隔离平台读取；`ResolvePlatformPaths` 保持现有请求标识、取消和过期结果规则。
- 未安装受支持 Codex 是 `Ready + installation=None`，不是 `Empty`；系统或配置错误进入 `Failed`。

### Platform

- Windows 使用 `windows = 0.58.0` 安全 WinRT 查询三个 Package Family，并按四段版本降序选择；standalone 只扫描三个固定根。
- macOS 只扫描 `/Applications` 与 `$HOME/Applications` 的四个固定应用名，并归一化合法 `.app` 或 `Contents/MacOS` 可执行文件。
- 两端都拒绝管理器路径和无效显式路径回退；`CODEX_HOME`、用户目录、状态根和三个固定文件名共享同一错误合同。
- 非 Windows/macOS 目标明确返回 `PLATFORM_PATHS_UNSUPPORTED`。

## TDD 与 Git checkpoint

| 阶段 | RED/GREEN 证据 | 提交 |
| --- | --- | --- |
| Domain | 绝对路径、脱敏 Debug、安装来源与快照合同 | `67447913fa656f30dd2e6d3c65707acca7c20869` |
| Application | 请求、端口、Ready+None、失败、取消与过期结果 | `7e52ec2c4ea2667c22a66e3bae7888eb3cb9e2ce` |
| Platform | Windows 三 Package Family/standalone、macOS 两根四名称、共享根目录与固定错误码 | `593c447262f1b1aa0ea578bb4a6a0a65037799a6` |
| 范围扩展 | 新增验证器单一路径并冻结三十路径哈希 | `a6e4a28e00c976aa91bca14afb1729ae7e6af194` |
| Parity | RED `9/13`、四项统一 `InvalidInitialParityStatus`；GREEN `13/13` 且完整 crate 全绿 | `be5673c82154fe2777046283158a152d11ead62d` |

## 稳定错误与副作用

- `PLATFORM_PATHS_UNSUPPORTED`
- `EXPLICIT_CODEX_PATH_INVALID`
- `CODEX_HOME_INVALID`
- `USER_HOME_UNAVAILABLE`
- `INPUTCODEX_STATE_ROOT_UNAVAILABLE`
- `PLATFORM_PATHS_FAILED`
- 允许副作用仅为 `filesystem-read` 与 `process-read`。
- 禁止 `filesystem-write`、`network-read`、`network-write`、广告、远程推荐、遥测与注入。

## 本地验证状态

```yaml
status: local-verification-passed-pr-pending
scope_count: 30
scope_hash: sha256:ae5e0f5143355feee9b280da7c44fdd5cdf759ec2ae71fc69167040bf302cb37
domain_checkpoint: 67447913fa656f30dd2e6d3c65707acca7c20869
application_checkpoint: 7e52ec2c4ea2667c22a66e3bae7888eb3cb9e2ce
platform_checkpoint: 593c447262f1b1aa0ea578bb4a6a0a65037799a6
parity_checkpoint: be5673c82154fe2777046283158a152d11ead62d
parity_target: 13/13 passed
parity_crate: all test targets passed
parity_clippy: passed
fmt: passed
final_four_crate_test: passed
final_four_crate_clippy: passed
ci_contract: 35/35 passed
release_audit: current
repository_policy: passed with 0 violations
scope_and_privacy: passed; changed=30; private-user-identifier=0
local_verification_time: 2026-07-27 21:57:22 +08:00
local_verification_duration_seconds: 36.706
pr_ref: pending
review_ci: pending
final_merge: not-authorized
```

## 后续门禁

1. 提交并普通推送控制面 checkpoint，创建关联 `Closes #75` 和 Issue `#74` 的非 Draft PR。
2. 核验 Review 对话、标准 CI、Windows/macOS 真实编译、非 required Performance observation 与 Artifact `0`。
3. 全部门禁通过后停止，等待项目所有者针对 Final Head 单独授权 Squash Merge。
