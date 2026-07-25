# Issue #50 性能预算 Discovery 实施计划

## 目标

基于 Issue `#32` / PR `#49` 已入库的 Windows 与 macOS 性能证据，冻结 `inputcodex` 性能预算的可比环境、采样与统计方法、异常和失败语义、观察到阻断的升级条件，以及 Gate 5 解锁前置条件。

本任务只形成设计、长期控制面和后续 Issue 边界，不填写绝对预算数值，不修改性能实现或 CI，不实施优化，也不迁移任何上游业务功能。

## 已批准方案

项目所有者已批准方案 A：

1. 先使用独立 Discovery Issue 冻结预算方法和语义。
2. 再使用独立 Issue/PR 进行可比 hosted 复测和预算数值批准。
3. 预算 CI 实施继续保持独立 Issue/PR。
4. 性能前置条件闭环后，才允许建立 Gate 5 解锁与首个业务功能迁移 Issue。

批准引用：`user-message:按A方案开始-2026-07-25`。

## Fresh 基线

- 跟踪 Issue：`https://github.com/nonononull/inputcodex/issues/50`。
- 分支：`codex/issue-50-performance-budget-discovery`。
- 基线提交：`fd9db9ca1c150b7db34dda8acc09b6f0cc357a17`。
- 基线 tree：`3fc4a5a7697850f048edcedf6a9ec5e4f76c847c`。
- PR `#49` 已按单父 Squash 合并，Issue `#32` 已按 `COMPLETED` 关闭。
- 上游最新正式 Release：`v1.2.42@657cd33e009ad02515d30db6492cd4e669b06318`。
- Ruleset `19395456` 只允许 Squash、要求解决 Review 对话；当前人类维护者为 `1`，required approvals 为 `0`，但合并仍需项目所有者最终决策证据。

## 核心语义

1. Issue `#32` 的关闭只表示性能基线完成，不能解释为性能预算已经批准。
2. Windows 与 macOS 只能在同平台、同 Runner 类别和可比环境指纹内形成趋势；禁止跨平台排名、比例换算或共享数值阈值。
3. 原始样本中的 IQR 标记必须保留。Discovery 必须定义“标记异常”“样本无效”“整次 Run 不可比较”和“产品真实回归”的不同语义。
4. GitHub Actions 外部事故、Runner 镜像漂移、工具链漂移、证据合同失败和产品性能回归必须使用不同错误码与处理路径。
5. 预算从观察模式升级为阻断模式前，必须满足最小可比运行数量、稳定窗口和项目所有者数值批准。
6. Master Plan 中硬编码的 `v1.2.41-inputcodex.1` 与当前上游 Release 存在潜在冲突。本 Issue 只记录风险和分流条件；实际发布策略修改必须进入独立 `type:release` Issue。

## 候选交付物

- `docs/adr/0004-performance-budget-policy.md`：长期性能预算决策合同。
- `docs/reports/issue-50-performance-budget-discovery.md`：证据盘点、方法选择、拒绝方案和后续 Issue 边界。
- `AGENTS.md`、`README.md`、`build.md`、Master Plan：同步 PR `#49` 已合并、Issue `#32` 未批准预算、Issue `#50` 正在 Discovery 的稳定状态。
- 本计划、Session Plan 与 Runtime Workflow：记录批准、精确范围、执行批次和停止条件。

## 候选写集合

```text
AGENTS.md
README.md
build.md
docs/adr/0004-performance-budget-policy.md
docs/plans/PROJECT-MASTER-PLAN.md
docs/plans/2026-07-25-issue-50-performance-budget-discovery.md
docs/plans/sessions/2026-07-25-issue-50-performance-budget-discovery.md
docs/reports/issue-50-performance-budget-discovery.md
docs/workflows/2026-07-25-issue-50-performance-budget-discovery-runtime.md
```

路径按 Windows `Sort-Object` 默认大小写不敏感顺序排序，以 UTF-8 无 BOM、LF 连接并保留末尾 LF 后计算：

`scope_hash: sha256:af1c248c46d54741f9c77ab3621cd66ccd40e3fa50698d377c788fcb0b93205f`

## 执行任务

### Task 1：规划检查点

- 创建 Issue、隔离分支和工作树。
- 读取项目长期入口、Issue `#32` 报告、原始结果结构、既有 ADR 和排错记录。
- 形成任务计划、Session Plan、Runtime Workflow、候选九路径和 `scope_hash`。
- 在项目所有者批准九路径前停止，不写 ADR、报告或长期入口。

### Task 2：性能预算 ADR

- 冻结纳入预算、仅观察和明确排除的指标类别。
- 冻结环境可比键、最小可比 Run 数量、统计量、波动与异常处理。
- 冻结观察、候选阻断和强制阻断三个阶段及升级条件。
- 冻结失败语义、复测次数、事故分流和项目所有者批准证据。

### Task 3：长期状态同步

- 把 PR `#49` 的稳定合并事实写入长期入口，不复制会快速失效的完整 GitHub 动态详情。
- 明确 Issue `#32` 未批准预算，Gate 5 继续锁定。
- 登记 Issue `#50` 的 Discovery 状态和下一合法工作。
- 记录版本命名冲突必须分流独立 Release Issue，禁止顺手修改发布资产。

### Task 4：Discovery 报告

- 盘点现有 Windows/macOS 环境指纹、样本合同和已知异常。
- 比较“直接阻断”“先观察后阻断”“Gate 5 与预算并行”三种路径。
- 记录采用方案 A 的原因和另外两种方案的拒绝理由。
- 输出后续复测与数值批准 Issue、预算 CI 实施 Issue、Gate 5 解锁 Issue 的互斥边界。

### Task 5：验证与 PR

- 验证实际差异不越过批准九路径，重算 `scope_hash`。
- 运行性能 Evidence、CI 合同、Repository Policy 和 `git diff --check`。
- 本地不运行完整 Workspace、桌面 Release 或真实性能采集。
- 创建非 Draft PR，等待 Review/CI 与项目所有者单独 Squash Merge 授权。

## 明确禁止

- 根 `Cargo.toml`、根 `Cargo.lock`、`apps/`、`crates/`、`parity/`、`upstream/`、`benchmarks/`、`.github/workflows/`、Ruleset、Release 和 AGOS 零差异。
- 不创建预算配置，不修改验证器，不新增 CI Job，不触发收费或 self-hosted Runner。
- 不删除或平滑原始异常样本，不把跨平台数值差异写成优劣结论。
- 不迁移产品功能、不设计 UI、不处理一致性例外。
- 不 Force Push、不删除 `main`、不自动合并、不删除来源分支或工作树。

## 完成定义

- 九路径和 `scope_hash` 获得项目所有者明确批准。
- ADR 和报告给出无占位符、可执行且不含预算数值的决策合同。
- 长期入口与 GitHub 权威事实一致，Issue `#32` 的预算语义不再被误读。
- 所有本地轻量验证、Review 对话和适用 CI 完成根因闭环。
- PR 停在项目所有者最终 Squash Merge 决策前。
