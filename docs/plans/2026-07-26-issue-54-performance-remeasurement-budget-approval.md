# Issue #54：五次性能复测与数值批准实施计划

## 目标

在不修改性能采集实现、预算 CI、Ruleset、性能优化或 Gate 5 的前提下，针对 Windows 与 macOS 分别缓存至少五次全新、独立、同可比队列的 GitHub-hosted `Performance Baseline` 样本，形成可重复验证的统计摘要和明确的 warning/blocking 数值控制面。

本 Issue 只批准数值与原始样本，仓库阶段仍保持 `observation`；把数值接入报告型 CI 的 `approved-observation`，必须使用后续独立 Issue/PR。

## 决策、根因与基线

- 项目所有者在 Windows 本机时间 `2026-07-26 06:19:35 +08:00` 已作出“按你推荐来 不用我二次批准 直接安排后续”决定。Issue `#54` 将其解释为：达到 ADR `0004` 的证据条件后，直接写入数值控制面，不再等待第二次口头数值批准。
- Issue `#55` 已以 PR `#56` 的 Squash 提交 `325bb2419548bc076502065dc583f54f4fddd582` 进入 `main`，显式 `workflow_dispatch mode=measure` 是唯一允许的复测入口；禁止删除 Issue `#32` Evidence 或修改 Workflow 语义来触发采样。
- Fresh 基线为 `main@325bb2419548bc076502065dc583f54f4fddd582`，`release_audit=current`。工作分支 `codex/issue-54-performance-remeasurement-budget-approval` 只以普通提交和普通 push 推进。
- AGOS 是可选外部辅助；此前 `needs-input/unregistered` 不阻塞本 Issue，且本 Issue 不修改任何 AGOS 文件。

## 数据模型与可比性合同

### 原始样本

1. 最多连续触发八次 `workflow_dispatch mode=measure`，严格等待当前 Run 的 `contract`、`windows`、`macos`、`required` 全部终态后才触发下一次。
2. 每个 Run 使用不同的 GitHub `run_id`，每个平台的临时成功 Artifact 仅下载为对应固定槽位 `run-01` 至 `run-08` 的原始 JSON；不得把 rerun attempt 记为独立 Run。
3. 每次下载后都校验平台、`status=complete`、`github-hosted`、同一 `run_id`、`source`、归一化 SHA-256、Artifact ID 与名称，并把所有成功或非成功的处理记录进 `benchmarks/results/issue-54/manifest.json`。

### 分类

- `evidence-invalid`：schema、哈希、采样数量、checksum、成功状态、Artifact 或双平台来源关系不满足合同；保留原始结果但不进入统计。
- `external-incident`：只有 GitHub Run/Job 失败且存在可复核 Actions 或官方状态证据时才能使用；不得用单次慢样本、IQR 标记或“Runner 抖动”替代根因。
- `new-cohort-valid`：结果完整但相对本平台首个有效队列的环境指纹变化；保留为新队列观察值，不进入旧队列预算。
- `comparable-valid`：结果完整且与本平台已选队列匹配，可进入本 Issue 的五次统计。

每个平台独立建立队列。硬键为 schema、platform、runner arch、`github-hosted`、镜像系列、Rust release/host、配置哈希、输入哈希与样本合同；队列指纹继续记录 image version、OS 描述、处理器、逻辑处理器数和内存档位。`implementation_sha256` 仅绑定实现，不作为可比键。

### 统计与数值

对每个平台的五个或更多 `comparable-valid` Run，分别保留全部 Run 的 median/P95，计算 median-of-medians、median-of-P95、min、max 与 MAD；不删除 IQR 标记或慢样本。

阻断候选仅包括：首次 view 的 median/P95、空闲 Working Set 的 median/P95，以及三个 Rust 场景的 median/P95。空闲 CPU 与桌面二进制大小只作为观察指标；两个 Release 构建耗时和基准二进制大小只作为诊断数据。

每个阻断候选的每个统计 lane 使用固定、可复算的上界公式：

```text
center = median(run_level_values)
mad = median(abs(value - center))
warning = round_up(center + max(3 * mad, 10% * center), quantum)
blocking = round_up(center + max(5 * mad, 20% * center), quantum)
```

`quantum` 固定为首次 view `1 ms`、Working Set `1 MiB`、Rust `0.001 ns/op`。预算控制面必须同时写入中心、MAD、安全裕量、warning、blocking、单位、队列指纹、来源 `run_id`、公式版本和所有者预授权引用；验证器只复算并比较已入库数值，不得自行静默生成或启用预算。

## 精确范围

以下排序后的二十九条路径是唯一允许写集合，`scope_hash` 为 `sha256:32c818aaf99efe550e9afc45d5871f9dceeef50ba314aaa91b617cd76158a38e`。`run-01` 至 `run-08` 是预留、固定命名的原始 Artifact 槽位；如果八次串行 Run 后任一平台仍没有五个同队列 `comparable-valid` 样本，必须停止、记录根因并建立新的范围，不得新建第九个槽位。

```text
AGENTS.md
benchmarks/budgets/issue-54-approved-observation.json
benchmarks/results/issue-54/manifest.json
benchmarks/results/issue-54/runs/run-01/macos.json
benchmarks/results/issue-54/runs/run-01/windows.json
benchmarks/results/issue-54/runs/run-02/macos.json
benchmarks/results/issue-54/runs/run-02/windows.json
benchmarks/results/issue-54/runs/run-03/macos.json
benchmarks/results/issue-54/runs/run-03/windows.json
benchmarks/results/issue-54/runs/run-04/macos.json
benchmarks/results/issue-54/runs/run-04/windows.json
benchmarks/results/issue-54/runs/run-05/macos.json
benchmarks/results/issue-54/runs/run-05/windows.json
benchmarks/results/issue-54/runs/run-06/macos.json
benchmarks/results/issue-54/runs/run-06/windows.json
benchmarks/results/issue-54/runs/run-07/macos.json
benchmarks/results/issue-54/runs/run-07/windows.json
benchmarks/results/issue-54/runs/run-08/macos.json
benchmarks/results/issue-54/runs/run-08/windows.json
build.md
docs/plans/2026-07-26-issue-54-performance-remeasurement-budget-approval.md
docs/plans/PROJECT-MASTER-PLAN.md
docs/plans/sessions/2026-07-26-issue-54-performance-remeasurement-budget-approval.md
docs/reports/issue-54-performance-remeasurement-budget-approval.md
docs/workflows/2026-07-26-issue-54-performance-remeasurement-budget-approval-runtime.md
err.md
README.md
scripts/performance/Build-InputcodexBudgetApproval.ps1
scripts/performance/Test-InputcodexBudgetApproval.ps1
```

## 实施任务

### 任务 1：冻结控制面并推送可采样 Head

- 新建本计划、Session Plan、Runtime Workflow 和初始报告；同步 `AGENTS.md`、`README.md`、`build.md`、`err.md` 与 Master Plan。
- 在 GitHub Issue `#54` 回写二十九路径、`scope_hash`、最大八次串行限制、数值公式和禁止面；普通提交并推送分支。
- 本地只运行 Git、文档、Evidence、CI 合同和仓库政策轻量验证；不运行完整 Rust Workspace、桌面 Release 或本地真实性能采集。

### 任务 2：串行 hosted 样本缓存

- 依次触发 `gh workflow run 'Performance Baseline' --ref codex/issue-54-performance-remeasurement-budget-approval -f mode=measure`。
- 对每个 Run 等待四个 Job 成功，查询 Artifact 元数据并下载 Windows/macOS JSON 到临时目录；失败 Run 不重跑 attempt 冒充新样本，而是登记为 `external-incident` 或 `evidence-invalid` 并给出证据。
- 使用 `apply_patch` 将每份已核验 JSON 写入下一个固定槽位；更新样本 manifest，直到每个平台具有五个同队列 `comparable-valid` 样本或达到八次上限。

### 任务 3：TDD 实现离线预算构建与验证

- 先创建 `Test-InputcodexBudgetApproval.ps1` 的失败合同：缺少五个可比样本、重复 run、环境队列漂移、checksum 不一致、预算公式或安全裕量不匹配必须失败。
- 以最小 `Build-InputcodexBudgetApproval.ps1` 读取原始 manifest/JSON，输出固定 schema 的统计与预算 JSON；它不访问网络、不触发 Workflow、不写 CI 配置。
- 用真实五次数据生成 `issue-54-approved-observation.json`，再由测试脚本复算所有值并确认观测/诊断指标没有被误列为阻断预算。

### 任务 4：收口与 PR

- 更新报告、长期入口和 `err.md`：每个 Run 的来源、分类、队列、统计、数值、裕量、预授权和未发生的禁止变更都必须可复核。
- 执行范围审计、样本/预算验证、Issue `#32` Evidence、CI 合同、仓库政策、`git diff --check`；提交、普通 push、创建 `Closes #54` 的非 Draft PR。
- 所有 Review 对话必须记录根因、处理和验证；PR CI 全绿后以 Squash Merge 合并。预算 CI、优化、Gate 5 与分支删除继续使用独立决策。

## 停止条件

- 二十九路径或 `scope_hash` 漂移，或八个固定 Run 槽位仍不能为任一平台提供五个同队列可比样本。
- 需要修改 Workflow、采集器、验证器、schema、配置、Cargo、Ruleset、上游、Release、预算 CI、性能优化或 Gate 5。
- 任一 Artifact/结果哈希、来源、队列或 checksum 不能闭环，或者本地/hosted/PR 验证失败且根因未解决。
