# Issue #86 Gate 5 运行时环境冲突只读观察设计

## 文档状态

- `status`: `PLANNING_SCOPE_FROZEN_OWNER_APPROVAL_PENDING`
- `tracking_issue_ref`: `https://github.com/nonononull/inputcodex/issues/86`
- `approved_decision_ref`: `https://github.com/nonononull/inputcodex/issues/85`
- `owner_approval_evidence`: `https://github.com/nonononull/inputcodex/issues/85#issuecomment-5102509300`
- `written_review_ref`: `https://github.com/nonononull/inputcodex/issues/86#issuecomment-5102713590`
- `base_ref`: `3f2914cd81ace7afe28e0137c867c20fd346c3f9`
- `upstream_release`: `v1.2.43`
- `branch_ref`: `codex/issue-86-gate-5-runtime-environment-observation`
- `planning_scope_count`: `3`
- `planning_scope_hash`: `sha256:c3d16ff75e79d9fd2866db1bd59f4259089b7398bce46b20e2766fc2bccc6d34`
- `candidate_scope_count`: `23`
- `candidate_scope_hash`: `sha256:448587243eb7cf842f7412bba868347aaada01016b964424812d5b47a278d66e`
- `implementation_authorization`: `pending-owner-scope-approval`
- `commit_push_pr_authorization`: `pending-owner-scope-approval`
- `final_merge_authorization`: `pending-separate-gate`
- `agos_status`: `bypassed-project-native-control-plane`
- `written_on`: `2026-07-28`，时间判定只使用项目所有者 Windows 本机时间

## 目标

以纯 Rust 迁移第四个 Gate 5 产品切片：只读观察当前 `inputcodex` 进程实际继承的
`OPENAI_*` 环境变量冲突，并返回不含变量实际值的稳定领域结果。

本设计保证：

1. 不把只读检测冒充成上游“检测并清理环境变量”总功能已经迁移；
2. 不把删除环境变量、写备份、平台覆盖不一致或 `unsafe` 带入新架构；
3. Windows 与 macOS 提供同一产品能力，并遵守各自真实名称比较语义；
4. 结果明确说明来源覆盖，禁止把“当前进程无冲突”解释成“整机无冲突”；
5. 不联网、不写文件、不缓存、不启动线程或子进程，不引入新依赖。

## 已批准决策

Issue `#85` 已批准“观察与清理分离”：

- 新增 `feature.foundation-platform.runtime-environment-conflict-observation`；
- 原 `feature.foundation-platform.environment-conflicts` 保持 `unassessed`；
- 当前切片只观察当前进程环境；
- 删除、持久化来源扫描和恢复能力使用未来独立决策与实现 Issue；
- 不创建 Issue `#85` Closeout，也不为 Issue `#86` 预建递归 Closeout。

## 上游问题与根因

上游 `env_conflicts` 把以下职责放在同一功能中：

- 识别 `OPENAI_*` 环境变量；
- 读取当前进程环境；
- Windows 读取用户级持久化环境；
- 删除当前进程和用户环境变量；
- 在状态目录写入 JSON 备份。

该组合不能直接进入 `inputcodex`：

1. 只实现检测却把总功能标记为 `implemented`，会制造错误完成状态；
2. 清理路径修改全局进程状态和用户环境，属于破坏性副作用；
3. 上游非 Windows 平台不观察用户级持久化环境，却使用同一个结果；
4. 上游对名称执行 `trim`，可能把真实名称不同的变量误判为冲突；
5. 修改真实进程环境构造测试会产生并行竞态，Rust 2024 下还需要 `unsafe`。

根因不是缺少过滤函数，而是观察、覆盖声明、持久化来源和清理副作用被混成了单个语义。

## 功能身份

### 新子能力

- ID：`feature.foundation-platform.runtime-environment-conflict-observation`
- 名称：`运行时环境冲突观察`
- 状态：Domain、Application、Platform、Parity 与双平台 Hosted 验证全部通过后才可设为
  `implemented`。

### 原总功能

- ID：`feature.foundation-platform.environment-conflicts`
- 状态：继续为 `unassessed`
- 原因：持久化来源观察、删除、备份和恢复均未交付。

Parity 必须同时表达这两个事实，禁止通过描述文字掩盖状态不一致。

## 领域模型

### `EnvironmentVariableName`

- 只保存可无损表示的变量名称，不保存变量值；
- 名称使用操作系统原始名称，不执行首尾空白修剪；
- Windows 按大小写不敏感语义识别并生成大写规范名；
- macOS 按大小写敏感语义识别并保留原名；
- 只有真实前缀为 `OPENAI_` 的名称才是候选；
- `CUSTOM_OPENAI_API_KEY`、`X_OPENAI_API_KEY` 等不得误判；
- 命中名称无法无损转换时整次观察明确失败，不得静默丢弃。

### `EnvironmentValuePresence`

只表示：

- `Empty`：变量存在，但值为空；
- `NonEmpty`：变量存在，且值非空。

领域对象和错误不得持有原始值或其字符串副本。

### `EnvironmentSourceCoverage`

固定包含：

- `runtime_process: Observed`
- `persistent_user: NotObserved`
- `persistent_system: NotObserved`

覆盖信息属于产品结果，不是仅供调试的元数据。

### `RuntimeEnvironmentConflict`

包含规范化名称、值存在状态和固定来源 `RuntimeProcess`；不包含变量值、私人路径、PID、
用户名或机器标识。

### `RuntimeEnvironmentConflictObservation`

包含排序去重后的冲突列表、来源覆盖、扫描条目数量和冲突数量。没有冲突时仍返回合法观察
结果，不使用“没有数据”代替“观察完成且结果为空”。

## 应用层设计

新增：

- `RuntimeEnvironmentObservationRequest`：无业务参数的只读请求；
- `RuntimeEnvironmentObservationPort`：平台观察 Port；
- `ObserveRuntimeEnvironmentConflicts`：应用用例。

用例复用现有 `RequestId`、`LoadCoordinator`、`LoadCompletion`、陈旧结果隔离和取消状态。

状态语义：

- 开始后进入 `Loading`；
- 成功且没有冲突时进入 `Ready`，值中包含空列表和覆盖声明；
- `Empty` 不承载本能力结果；
- 调用方截止时间到达后进入 `Failed`，随后返回的同步结果按迟到结果丢弃；
- 取消只使当前请求失效，不启动清理工作；
- 旧请求结果不得覆盖新请求。

稳定错误：

- `RUNTIME_ENVIRONMENT_OBSERVATION_UNSUPPORTED`：目标不是 Windows 或 macOS；
- `RUNTIME_ENVIRONMENT_OBSERVATION_TIMEOUT`：观察超过调用方批准的截止时间；
- `RUNTIME_ENVIRONMENT_NAME_UNREPRESENTABLE`：命中名称不能无损进入领域模型。

超时不启动线程或抢占式终止系统调用，只使当前请求和迟到结果失效。不得把超时或名称转换
失败解释为没有冲突。

## 平台层设计

系统适配器只允许调用一次 `std::env::vars_os()`，并拆为：

1. 系统入口：读取环境后调用纯观察函数；
2. 纯观察函数：接收注入的 `(OsString, OsString)` 迭代器供测试；
3. Windows 策略：大小写不敏感识别，输出大写规范名；
4. macOS 策略：大小写敏感识别，输出原名。

禁止：

- `std::env::set_var`、`std::env::remove_var`；
- 注册表、shell profile、`.env`、系统代理或持久化来源读取；
- `reg.exe`、PowerShell、`launchctl`、`scutil` 或其他子进程；
- 文件系统、网络、缓存、后台线程；
- `unsafe` 或直接 Win32 FFI；
- 在非 Windows/macOS 平台伪造成功。

## 性能设计

- 对进程环境做一次线性扫描；
- 只为命中的名称建立领域对象；
- 只对命中集合排序和去重；
- 不复制或格式化变量实际值，只检查值是否为空。

本切片不增加性能预算数值，也不创建新的性能 Workflow。

## 隐私与诊断

产品结果可以向明确调用者返回冲突变量名；诊断证据只允许记录：

- `request_id`；
- 扫描条目数量；
- 冲突数量；
- 来源覆盖状态；
- 耗时；
- 结果分类或稳定错误码。

诊断证据禁止记录变量名称、变量值、用户名、私人路径、机器标识或未过滤 `Debug` 输出。

## Parity 设计

`parity/features/foundation-platform.yml`：

- 新增只读子能力；
- 证据只引用检测函数和 `check_env_conflicts` 命令；
- Windows/macOS 共同声明只读当前进程观察；
- 决策引用 Issue `#85` 与实现 Issue `#86`；
- 原总功能保持 `unassessed`，并追加 Issue `#85` 作为拆分依据。

`parity/contracts/foundation-platform.yml`：

- 新增只读观察合同；
- `side_effects` 为 `environment-read`；
- 持久化为 `none`；
- 明确 `Ready(empty)`、覆盖、值不泄露、取消和错误隔离；
- 明确调用方截止时间、稳定超时码和迟到结果失效；
- 不修改原总功能的清理合同完成状态。

`parity/features/source-index.yml` 和 `upstream/` 保持不变。

## TDD 验收矩阵

### Domain

- 接受真实 `OPENAI_` 前缀，拒绝非前缀和空名称；
- 空值与非空值状态不包含原始值；
- 排序、去重稳定；
- 覆盖范围不能伪造持久化来源已观察。

### Application

- Port 成功进入 `Ready`；
- 零冲突进入 `Ready` 空列表；
- `Empty` 不承载合法结果；
- 不支持平台和名称不可表示映射稳定错误；
- 超过调用方截止时间进入 `Failed`，迟到结果不得应用；
- 陈旧请求不会覆盖新请求；
- 取消不会触发第二次平台调用。

### Platform

- Windows 大小写不敏感，macOS 大小写敏感；
- 不修剪名称，排除 `CUSTOM_OPENAI_*`；
- 只检查值是否为空；
- 系统入口只调用一次环境采集；
- 测试只使用注入样本，不修改真实进程环境；
- Linux 返回不支持，不伪装双平台结果。

### Parity 与仓库政策

- 新子能力可进入 `implemented`，原总功能仍为 `unassessed`；
- 合同和目录 ID 一致，`source-index.yml` 未修改；
- Cargo、UI、网络、写入、线程、shell、`unsafe` 和值泄露守卫通过；
- Windows/macOS Hosted 测试和现有七 Job CI 合同通过。

## 预计文档更新

- `README.md`：新增第四个已迁移能力，明确只观察当前进程；
- `CONTEXT.md`：新增运行时冲突观察与来源覆盖术语；
- `AGENTS.md`：将 Issue `#86` 设为当前第四切片并锁定禁止面；
- `build.md`：增加 Issue `#86` 本地轻量验证和禁止项；
- `err.md`：只有出现新的可复用根因时才增加条目；
- `docs/plans/PROJECT-MASTER-PLAN.md`：记录活动任务、决策引用和下一阶段；
- Session Plan、Runtime Workflow 与实施报告使用 Issue `#86` 独立文件。

## 候选完整实施范围

```text
AGENTS.md
CONTEXT.md
README.md
build.md
crates/inputcodex-application/src/lib.rs
crates/inputcodex-application/src/runtime_environment_observation.rs
crates/inputcodex-application/tests/runtime_environment_observation.rs
crates/inputcodex-domain/src/lib.rs
crates/inputcodex-domain/src/runtime_environment_observation.rs
crates/inputcodex-domain/tests/runtime_environment_observation.rs
crates/inputcodex-parity/tests/catalog_repository.rs
crates/inputcodex-platform/src/lib.rs
crates/inputcodex-platform/src/runtime_environment_observation.rs
crates/inputcodex-platform/tests/runtime_environment_observation.rs
docs/plans/2026-07-28-issue-86-gate-5-runtime-environment-observation.md
docs/plans/PROJECT-MASTER-PLAN.md
docs/plans/sessions/2026-07-28-issue-86-gate-5-runtime-environment-observation.md
docs/reports/issue-86-gate-5-runtime-environment-observation.md
docs/workflows/2026-07-28-issue-86-gate-5-runtime-environment-observation-runtime.md
err.md
parity/README.md
parity/contracts/foundation-platform.yml
parity/features/foundation-platform.yml
```

规范化后共 `23` 路径，哈希为
`sha256:448587243eb7cf842f7412bba868347aaada01016b964424812d5b47a278d66e`。

规划控制面固定为设计稿、Session Plan 和 Runtime Workflow 三路径，哈希为
`sha256:c3d16ff75e79d9fd2866db1bd59f4259089b7398bce46b20e2766fc2bccc6d34`。

任何新增、删除、重命名或替换路径都必须重新计算哈希并取得项目所有者批准。

## 非目标

- 环境变量删除、恢复或备份；
- 用户级或系统级持久化环境扫描；
- 设置管理、网络环境诊断或日志诊断；
- UI、Iced 视图或第五个产品 feature；
- 新依赖、Cargo 清单或锁文件；
- 性能预算数值、Release、`upstream/`、Ruleset 或 AGOS；
- 递归 Closeout。

## 交付与审批门

1. 本设计稿落盘并完成自审；
2. 项目所有者审阅书面设计；
3. 再建立 Session Plan、Runtime Workflow、候选路径和 `candidate_scope_hash`；
4. 项目所有者批准精确路径和哈希；
5. 才允许进入 TDD 和实现；
6. 普通推送、非 Draft PR、Review/CI；
7. 最终 Squash Merge 保留单独授权门；
8. 动态证据只保留在 GitHub Issue/PR，不创建递归 Closeout。

## 书面设计自审清单

- [x] 与 Issue `#85` 决策一致；
- [x] 没有把原总功能标记为已实现；
- [x] 没有环境写入、文件写入、网络、线程、子进程或 `unsafe`；
- [x] 明确双平台名称比较差异但保持同一产品能力；
- [x] 明确空结果为 `Ready`；
- [x] 明确不返回或记录变量实际值；
- [x] 明确持久化来源未观察；
- [x] 没有占位符、未决设计或递归 Closeout；
- [x] 实现前仍保留书面审阅和精确范围批准门。
