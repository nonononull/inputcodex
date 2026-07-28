# Issue #89：Gate 5 Relay 环境只读观察实施报告

## 当前结论

Issue `#89` 已在批准的三十路径内完成 Planning、Domain、Application、共享 Platform、
Windows/macOS Platform 与 Parity RED→GREEN checkpoint。实现只聚合固定代理环境变量名称与
来源、`CODEX_HOME/.env` 存在状态和四个固定 Clash Verge 候选的 TUN 状态，不返回变量值、
文件内容、注册表值或实际路径。

项目控制面、本地轻量验证、普通推送、非 Draft PR `#90`、Review/CI 根因闭环与 Artifact
核验已经完成。动态 Head、Run、Artifact 和 Review 证据保留在 GitHub；最终 Squash Merge
继续保留项目所有者独立授权门。

## 基线与批准

- 本机分支：`codex/issue-89-gate-5-relay-environment-observation`
- 基线提交：`db0c09b9df272887deb9407a5e344cf87a59dda8`
- 基线 tree：`7429a3cf1705239a47ac5bf7536e5541da401c51`
- 上游功能真源：`v1.2.43 @ 5036ff056b5c629f19356396b17d6eeb70da664c`
- 一致性决策：Issue `#88`，Relay 环境只读观察与网络环境总功能分拆，已按 `COMPLETED` 关闭
- 实施 Issue：Issue `#89`
- 批准范围：`30` 路径
- `candidate_scope_hash`：`sha256:0adc20d0ed4d73ae645a5ffb23d7208f7aaabfea92c4d6fd62e0da3a120e8f77`
- 范围批准：<https://github.com/nonononull/inputcodex/issues/89#issuecomment-5104867049>
- 最终 Squash Merge：待项目所有者针对 Final Head 单独授权

## 已实现语义

### Domain

- 只接受 `HTTP_PROXY`、`HTTPS_PROXY`、`ALL_PROXY`、`NO_PROXY` 与 `FTP_PROXY`，名称按 ASCII 大小写不敏感匹配并输出固定大写。
- 单个代理名称只保存 `RuntimeProcess`、`PersistentUser` 与 `PersistentSystem` 来源集合，稳定排序并去重，不保存原始值。
- 来源覆盖显式区分 `Observed`、`NotObserved` 与 `Unavailable`。
- Codex `.env` 固定为 `Absent`、`Present` 或 `Unavailable`，不表达内容。
- 四个 Clash 逻辑来源分别固定 `Absent`、`Disabled`、`Enabled`、`Unreadable` 或 `Invalid`。
- 聚合快照分别表达已观察风险和观察缺口；无风险不是空结果，局部不可用也不会被静默当成不存在。

### Application

- `RelayEnvironmentObservationRequest` 是零字段请求，不携带名称、值、内容、路径或凭据。
- `RelayEnvironmentObservationPort` 隔离平台读取，`ObserveRelayEnvironment` 成功始终返回 `Ready`。
- 零风险和零代理变量仍返回 `Ready`，不使用 `LoadCompletion::Empty`。
- 单个注册表、`.env` 或 Clash 候选不可用保留在领域状态中；无法形成可信聚合时才返回稳定 `Failed`。
- 超时、取消、新请求和迟到结果继续使用既有请求身份与过期结果隔离合同。

### 共享 Platform

- 复用平台路径层的窄 `CODEX_HOME` 解析入口，显式无效路径继续按既有安全语义失败。
- `.env` 只调用文件元数据，不打开或读取文件内容。
- 四个 Clash 逻辑候选先按实际路径去重，单个实际路径最多读取一次，再映射回固定逻辑来源。
- Clash 单候选最多读取 `64 KiB`，超限、非 UTF-8、YAML 非映射或顶层 `enable_tun_mode` 非布尔均为 `Invalid`。
- 候选缺失、读取失败、禁用和启用均保持独立状态，不返回实际路径或配置内容。
- 非 Windows/macOS 系统入口返回 `RELAY_ENVIRONMENT_OBSERVATION_UNSUPPORTED`。

### Windows/macOS

- Windows 与 macOS 每次请求都只扫描一次当前进程环境。
- Windows 使用 `windows-registry = 0.6.1` 安全读取 HKCU `Environment` 与 HKLM 固定系统环境键；单一注册表来源失败只把对应覆盖标记为 `Unavailable`。
- macOS 不读取 shell profile、不调用 `launchctl` 或子进程；用户级与系统级持久化来源固定为 `NotObserved`。
- 双平台共享五个代理名称、`.env` 与 Clash 状态结构，只允许真实覆盖差异，不伪造跨平台完全扫描。

### Parity

- 新增 `feature.provider-network.relay-environment-observation`，状态为 `implemented`。
- `core-module:relay_environment` 与 `tauri-command:check_relay_environment` 映射到新子能力，副作用固定为 `environment-read` 与 `filesystem-read`。
- 原 `feature.provider-network.network-environment` 保持 `unassessed`，只保留 `core-module:proxy` 及其 `network-read` 语义。
- 行为合同固定 `Ready`、局部不可用、双平台来源覆盖、五条稳定错误、无持久化和 `64 KiB` 上限。
- 功能数与合同数从 `37` 增加到 `38`；来源总数保持 `133`，覆盖缺口保持 `0`。

## TDD 与 Git 证据

| 阶段 | RED 根因 | GREEN 与 checkpoint |
| --- | --- | --- |
| Planning | 三份独立控制面与三十路径范围尚未冻结 | `d4805d3f81801d4d685a510d31566738bc9d3ff6` |
| Domain | 测试无法导入代理名称、来源覆盖、`.env` 与 Clash 状态 | `fba0e2e3e7ebb692b417b4e3a80800388f711a1f` |
| Application | 测试无法导入零字段请求、Port 与只读观察用例 | `13e1f349b8a4d2556864e67f1066884b0e4832a4` |
| 共享 Platform | 测试无法导入共享文件探针、有界解析和非目标平台错误 | `1d9b774a661af44f765cb8e9cb3d7223a56e594d` |
| Windows/macOS | 测试无法导入注册表覆盖与双平台系统适配器 | `1dae9843618bdffc95f16069d54b7e7440d21db8` |
| Parity | 新子能力、合同与两个 Relay 入口映射尚不存在 | `749d02e23f1c1fa8f042d598d8f5bb5e28a18638` |

## 依赖与根因记录

唯一新增直接依赖是 Windows target 专用 `windows-registry = 0.6.1`。`Cargo.lock` 只新增
`windows-registry 0.6.1`、`windows-result 0.4.1` 与 `windows-strings 0.5.1`；既有
windows-rs 包只因多版本共存增加名称消歧。

首次离线解析被本机陈旧稀疏索引中的 `futures-util 0.3.32` 假冲突阻断。刷新 registry 索引后，
解析器保留既有锁版本并只产生上述三个批准的新包；该可复用根因已写入 `err.md`。

PR `#90` 首个标准 CI Head 在 Linux Workspace Clippy 暴露目标平台系统探针的 `dead_code`：
纯选择逻辑需要 `cfg(test)`，但真实文件系统探针、Windows 注册表路径常量和系统注册表探针只应
在目标平台编译。修复只收紧这些系统 I/O 实体的 `cfg`，保留测试 trait 与内存探针，不使用
`allow/expect(dead_code)`，不修改 Workflow 或产品语义。

## 当前验证状态

```yaml
status: local-verified-ready-for-push-pr
recorded_at_local: 2026-07-28T22:44:34+08:00
domain_checkpoint_tests: passed
application_checkpoint_tests: passed
platform_checkpoint_tests: passed
parity_checkpoint_tests: passed
parity_catalog_repository: 17/17
four_crate_test: passed
four_crate_clippy: passed
fmt: passed
locked_offline_platform_check: passed
ci_contract: 35/35
release_audit: current
repository_policy_violations: 0
candidate_scope_count: 30
actual_changed_path_count: 30
scope_hash: sha256:0adc20d0ed4d73ae645a5ffb23d7208f7aaabfea92c4d6fd62e0da3a120e8f77
new_lock_packages: windows-registry-0.6.1,windows-result-0.4.1,windows-strings-0.5.1
protected_path_changes: 0
privacy_matches: 0
forbidden_capability_matches: 0
dotenv_content_reads: 0
clash_candidate_read_limit_bytes: 65536
runtime_environment_scans_per_target: 1
local_verification_checkpoint: b33bbbb7a6b2303c7a0c60725745528c92085c66
push: completed
pr: https://github.com/nonononull/inputcodex/pull/90
review: completed-no-conversations
hosted_ci: completed-evidence-on-github
final_merge_authorization: pending-separate-gate
```

## 远端交付边界

- 只允许普通推送，禁止 force push。
- PR 必须非 Draft 并关联 Issue `#88/#89`。
- Final Head 变化后必须重跑适用门禁。
- Review 对话必须写明根因、处理方式和验证证据后解决。
- 标准 CI 与 Performance Baseline 成功 Run 的 Artifact 数必须为 `0`。
- 未收到项目所有者针对具体 PR 与 Final Head 的单独授权，不得 Squash Merge 或删除分支。
- 本任务不创建递归 Closeout。
