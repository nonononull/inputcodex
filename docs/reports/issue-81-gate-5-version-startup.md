# Issue #81：Gate 5 版本与启动意图实施报告

## 当前结论

Issue `#81` 已在批准的二十三路径允许范围内完成 Planning、Domain、Application、Platform 与 Parity RED→GREEN checkpoint。当前实现复用既有 `ApplicationVersion`，只从编译期 `CARGO_PKG_VERSION` 生成 `inputcodex` 版本；精确 `--show-update` 或合法 `INPUTCODEX_SHOW_UPDATE` 值解析为明确 `StartupIntent`。本切片不打开 UI、不联网、不检查、下载或执行更新、不读写文件、不缓存、不启动线程、不引入依赖，也不兼容旧启动变量。

控制面与最终本地轻量验证现已完成。尚未普通推送、尚未创建 PR，Hosted Review/CI 与最终 Squash Merge 均未完成；最终合并继续保留项目所有者独立授权门。

## 基线与批准

- 本机分支：`codex/issue-81-gate-5-version-startup`
- 基线提交：`ef69494d92c7c461b0cb858e95f6838404ae1a61`
- 基线 tree：`936cc74fbceae2a3ee8d98b924c836e13d9f7ae3`
- 上游功能真源：`v1.2.43 @ 5036ff056b5c629f19356396b17d6eeb70da664c`
- 方案决策：Issue `#80` 方案 A，已按 `COMPLETED` 关闭
- 实施 Issue：Issue `#81`
- 批准范围：`23` 路径
- `candidate_scope_hash`：`sha256:c1ef2c00a445dd2bd60dc5f5b375cb27d1e467a3d457d7eb53b7ec82a304aafe`
- 范围与实施授权：<https://github.com/nonononull/inputcodex/issues/81#issuecomment-5100612634>
- 最终 Squash Merge：待项目所有者针对 Final Head 单独授权

## 已实现语义

### Domain

- `StartupIntent` 只包含 `Default` 与 `ShowUpdate`。
- `VersionStartupSnapshot` 只保存经过校验的 `ApplicationVersion` 与启动意图，不保存原始命令行或环境值。
- 版本类型继续由应用概览领域模块定义；本 Issue 不移动或复制该类型。

### Application

- `VersionStartupRequest` 是零字段单元类型，不携带路径、参数、环境值或凭据。
- `VersionStartupPort` 隔离进程输入读取，`LoadVersionStartup` 只映射为 `LoadCompletion::Ready` 或 `LoadCompletion::Failed`。
- 共享错误模型新增 `ErrorKind::InvalidInput` 与 `ApplicationError::invalid_input`。
- 取消后的同步结果和较旧请求结果继续由 `LoadCoordinator` 判定为 `Stale`，不得覆盖当前状态。

### Platform

- `resolve_version_startup` 是可注入 `OsString` 参数与环境值的纯解析函数，Windows/macOS 共用同一语义。
- 跳过可执行文件名后，只匹配精确 `--show-update`；近似参数不改变默认意图。
- `INPUTCODEX_SHOW_UPDATE` 未设置、空值或 `0` 时继续读取命令行意图，`1` 产生 `ShowUpdate`。
- 其他非空值，包括非 Unicode 值，返回 `InvalidInput + INVALID_STARTUP_OPTION`，且错误不回显原值。
- 非法显式环境值优先失败，即使命令行同时包含 `--show-update` 也不得掩盖。
- 非 Windows/macOS 系统入口返回 `Unsupported + VERSION_AND_STARTUP_UNSUPPORTED`。
- 编译版本构造异常返回 `Internal + VERSION_AND_STARTUP_BUILD_VERSION_INVALID`。

### Parity

- `feature.foundation-platform.version-and-startup` 已更新为 `implemented`，决策引用固定 Issue `#80/#81`。
- 合同固定 `Ready(VersionStartupSnapshot)`、两个 `StartupIntent`、三个稳定错误、Windows/macOS 等价语义与 `process-read` 唯一副作用。
- 共享合同 schema 仍要求枚举六种协调器状态；合同已明确 `LoadCompletion::Empty` 对本功能不可达，产品用例不会产生 Empty。
- `core-module:version`、`tauri-command:backend_version` 与 `tauri-command:startup_options` 继续映射同一 feature，`source-index.yml` 未修改。

## TDD 证据

| 阶段 | RED 根因 | GREEN 与 checkpoint |
| --- | --- | --- |
| Planning | 三份控制面尚不存在 | `72b03b1af1fd7ab1984a481af1dd30a20879bb43` |
| Domain | 测试无法导入 `StartupIntent` 与 `VersionStartupSnapshot` | `391bfe9db9348518600e14c912333f221c3cfaca` |
| Application | 测试无法导入 Port、请求与用例 | `1eafa90866124e4c281eba127fd48bb701817ebd` |
| Platform | 测试无法导入纯解析函数与系统适配器 | `f992890611ff86f1fe6ccf5f0dd86e19d0fb07de` |
| Parity | 功能条目仍为 `unassessed` | `bee9dcb97fe9c790f45082cb23f0286c89b1d815` |

## 当前验证状态

```yaml
status: local-verified-final-git-checkpoint-pending
verified_at_local: 2026-07-28T14:44:08+08:00
four_crate_test: passed
four_crate_clippy: passed
fmt: passed-after-existing-root-cause-reuse
ci_contract: 35/35
release_audit: current
repository_policy_violations: 0
candidate_scope_count: 23
actual_changed_path_count: 22
scope_hash: sha256:c1ef2c00a445dd2bd60dc5f5b375cb27d1e467a3d457d7eb53b7ec82a304aafe
protected_path_changes: 0
privacy_matches: 0
legacy_variable_matches: 0
forbidden_capability_matches: 0
source_index_unchanged: verified-at-parity-checkpoint
push: pending
pr: pending
hosted_review_ci: pending
artifact_count: pending
```

## 排错复用

- `apply_patch.bat` 的 `Access is denied` 已命中 `err.md` 中桌面版 apply-patch 入口变化的既有根因；本 Issue 使用 `codex --codex-run-as-apply-patch`，不新增重复记录。
- Parity checkpoint 首次 PowerShell 命令把多个原生命令直接塞入表达式，触发 `ParserError`；该问题属于 `err.md` Issue `#75` 已记录的多命令组合与 PowerShell 解析脆弱性，改为先收集 `$allChanged` 再判定后通过，不修改产品代码。
- 首次完整轻量链在 `cargo fmt --all -- --check` 报告新 Rust 文件格式漂移；根因与 `err.md` Issue `#75`“格式检查没有紧跟代码批次”一致。运行一次 `cargo fmt --all` 后复验通过。
- 格式修复守卫首次只以未提交差异为基线，误把分支已提交但被 rustfmt 调整的批准路径当成新增路径；根因同样命中 Issue `#75` 已记录的 Git 统计漏计分支差异。改为合并 `origin/main...HEAD`、工作区 diff 与未跟踪文件后，确认所有 `22` 个实际路径均位于候选范围。
- 当前未发现需要新增 `err.md` 条目的产品根因。

## 剩余门禁

1. 创建控制面与最终本地验证 checkpoint，并使用本机默认 Git 时间提交。
2. 普通推送并创建关联 Issue `#80/#81` 的非 Draft PR。
3. 核验 Final Head Review、标准 CI、Performance Baseline observation、全部 Review 对话和成功 Run Artifact `0`。
4. 停在项目所有者单独 Squash Merge 授权门，未获授权不得合并。
