# Issue #86：Gate 5 运行时环境冲突只读观察实施报告

## 当前结论

Issue `#86` 已在批准的二十四路径内完成 Planning、Domain、Application、Platform 与 Parity
RED→GREEN checkpoint。当前实现只读观察当前 `inputcodex` 进程继承的 `OPENAI_*` 环境变量，
只返回名称、`Empty | NonEmpty` 和明确来源覆盖，不返回或持久化原始值。

项目控制面与完整本地轻量验证已经完成；尚未普通推送、尚未创建 PR，Hosted Review/CI 与最终
Squash Merge 均未完成。最终合并继续保留项目所有者独立授权门。

## 基线与批准

- 本机分支：`codex/issue-86-gate-5-runtime-environment-observation`
- 基线提交：`3f2914cd81ace7afe28e0137c867c20fd346c3f9`
- 基线 tree：`564d130cdc946fc32d9005f1fd02bfdf26339300`
- 上游功能真源：`v1.2.43 @ 5036ff056b5c629f19356396b17d6eeb70da664c`
- 一致性决策：Issue `#85`，“观察与清理分离”，已按 `COMPLETED` 关闭
- 实施 Issue：Issue `#86`
- 批准范围：`24` 路径
- `candidate_scope_hash`：`sha256:dd1d784ffe3149bf130c6bd678050d6aea3059f33a405abee5e2cc3f9735bb59`
- 范围批准：<https://github.com/nonononull/inputcodex/issues/86#issuecomment-5103198917>
- 最终 Squash Merge：待项目所有者针对 Final Head 单独授权

## 已实现语义

### Domain

- `EnvironmentVariableName` 只保存可无损表示的 `OPENAI_*` 名称，不执行首尾空白修剪。
- `EnvironmentValuePresence` 只表达 `Empty` 或 `NonEmpty`，不保存原始环境内容。
- `EnvironmentSourceCoverage` 固定为运行时进程 `Observed`、用户级和系统级持久化来源 `NotObserved`。
- 冲突按名称排序去重，同名样本中 `NonEmpty` 优先。

### Application

- `RuntimeEnvironmentObservationRequest` 是零字段请求，不携带名称、原始环境内容、路径或凭据。
- `RuntimeEnvironmentObservationPort` 隔离平台读取，`ObserveRuntimeEnvironmentConflicts` 成功只返回 `Ready`。
- 零冲突返回 `Ready(empty)`，不使用 `LoadCompletion::Empty`。
- 超时使用 `RUNTIME_ENVIRONMENT_OBSERVATION_TIMEOUT`；取消、旧请求和迟到结果继续由既有协调器隔离。

### Platform

- 系统入口每次请求精确调用一次 `std::env::vars_os()`，测试使用注入的 `OsString` 样本。
- Windows 名称比较大小写不敏感并输出大写规范名；macOS 名称比较大小写敏感并保留原名。
- 只有真实 `OPENAI_` 前缀命中；尾随空格属于名称本身，不会被静默修剪。
- 值只检查 `OsStr::is_empty`，不转换、不格式化、不返回、不记录。
- 名称无法无损表示时返回 `Internal + RUNTIME_ENVIRONMENT_NAME_UNREPRESENTABLE`。
- 非 Windows/macOS 系统入口返回 `Unsupported + RUNTIME_ENVIRONMENT_OBSERVATION_UNSUPPORTED`。

### Parity

- 新增 `feature.foundation-platform.runtime-environment-conflict-observation`，状态为 `implemented`。
- 原 `feature.foundation-platform.environment-conflicts` 保持 `unassessed`，继续包含 `core-module:env_conflicts` 与 `tauri-command:remove_env_conflicts`。
- `tauri-command:check_env_conflicts` 只修正为 `environment-read` 并映射到新只读子能力。
- 行为合同固定 `Ready(empty)`、三项来源覆盖、三条稳定错误和无持久化语义。
- 功能数与合同数从 `36` 增加到 `37`，入口总数、排除数、例外数和 fixture 数保持不变。

## TDD 证据

| 阶段 | RED 根因 | GREEN 与 checkpoint |
| --- | --- | --- |
| Planning | 三份独立控制面需要冻结 | `8119f921c061e5019336322a7ad4a4504ff8e16b` |
| Domain | 测试无法导入运行时环境领域类型 | `6591882dc23596a502833d38aed08d585b4acc08` |
| Application | 测试无法导入请求、Port 与用例 | `55b84b6c2b45d00fdf3f6e42aaa1e86d1635557e` |
| Platform | 测试无法导入双平台纯观察函数与系统适配器 | `cd41fa8ef739b1481cfbfc491ef42e26369f0b4e` |
| 范围修订 | Parity 入口归属无法在原二十三路径内合法修正 | `f177b9d6f17ee31d40bb6568f8e9bdf6bec901b5` |
| Parity RED | 新子能力、合同与单入口映射尚不存在 | `d5c711d9071aa9e9c65d5214531a96e04dddda98` |
| Parity GREEN | 新子能力与合同通过完整目录验证 | `a320086f00bd16c65ae5172c28f4bd8c40a7c110` |

## 根因记录

Platform 测试曾把名称 `OPENAI_API_KEY ` 误认为“不应命中”。复核后确认该名称仍以真实字节前缀
`OPENAI_` 开头，尾随空格只是名称的一部分；因此修正测试预期，生产实现保持不修剪。该结论已被
本 Issue 的设计和测试固定，不形成跨任务通用构建故障，`err.md` 保持未修改。

## 当前验证状态

```yaml
status: local-verified-ready-for-push-pr
recorded_at_local: 2026-07-28T19:17:48+08:00
domain_tests: passed
application_tests: passed
platform_tests: passed
parity_catalog_repository: 16/16
four_crate_test: passed
four_crate_clippy: passed
fmt: passed-after-one-line-rustfmt-root-cause-fix
ci_contract: 35/35
release_audit: current
repository_policy_violations: 0
candidate_scope_count: 24
actual_changed_path_count: 23
scope_hash: sha256:dd1d784ffe3149bf130c6bd678050d6aea3059f33a405abee5e2cc3f9735bb59
source_index_change: approved-single-entry-two-line-replacement
protected_path_changes: 0
privacy_matches: 0
forbidden_capability_matches: 0
system_vars_os_call_count: 1
local_verification_checkpoint: c6829fa40b7bf4cf9828f88e2dfe68c552536844
push: pending
pr: pending
review: pending
hosted_ci: pending
final_merge_authorization: pending-separate-gate
```

## 远端交付边界

- 只允许普通推送，禁止 force push。
- PR 必须非 Draft 并关联 Issue `#85/#86`。
- Final Head 变化后必须重跑适用门禁。
- Review 对话必须写明根因、处理方式和验证证据后解决。
- 标准 CI 与 Performance Baseline 成功 Run 的 Artifact 数必须为 `0`。
- 未收到项目所有者针对具体 PR 与 Final Head 的单独授权，不得 Squash Merge 或删除分支。
