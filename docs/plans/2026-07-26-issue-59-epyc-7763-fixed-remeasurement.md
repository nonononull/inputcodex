# Issue #59：EPYC 7763 四次固定串行复测实施计划

> **执行要求：** 后续按项目原生 Runtime Workflow 逐项执行；不得把 AGOS 状态提升为项目门禁，也不得在本任务修改 AGOS。

**目标：** 在保持 ADR `0004` CPU 硬队列语义的前提下，完整执行四次新的 GitHub-hosted 双平台测量，并对 AMD EPYC 7763 目标队列作一次确定性数值或硬停止判定。

**架构：** Issue `#54` 的 manifest 与十六份原始 JSON 以字节内容不变的方式缓存为历史证据；Issue `#59` 使用独立 manifest 和四个新槽位。预算值仅由历史目标队列样本与本 Issue 新命中样本共同推导，预算 CI 留给后续独立 Issue。

**技术栈：** Rust `1.97.1`、PowerShell 7、GitHub Actions 标准 hosted runners、JSON、Git/GitHub CLI。

## 全局约束

- 软件名称固定为 `inputcodex`；禁止广告、遥测、TypeScript、JavaScript 业务代码和 WebView。
- 不修改性能采集器、ADR `0004`、Workflow、产品源码、上游缓存、Ruleset、Release 或 AGOS。
- 本地只运行 `build.md` 定义的轻量验证；完整双平台测量和 CI 使用 GitHub-hosted runners。
- 所有本地时间以 Windows `Get-Date` 为准；不得覆写 Git 作者或提交时间。
- 普通提交、普通 push、非 Draft PR、全部 Review 对话根因闭环、最终仅 Squash Merge；禁止 force push 和删除 `main`。

## 已批准决策

- 跟踪 Issue：`https://github.com/nonononull/inputcodex/issues/59`。
- 所有者决定：`user-message:批准方案-A-EPYC-7763-四次固定串行复测-2026-07-26`。
- 目标 Windows 处理器：`AMD EPYC 7763 64-Core Processor`。
- 固定新槽位：`run-01`、`run-02`、`run-03`、`run-04`；四个槽位全部执行。
- 硬停止：四次后目标队列总数仍小于 `5` 时，不创建 `run-05`，不修改队列语义，不生成预算值。

## 历史证据模型

- 只读来源：分支 `codex/issue-54-performance-remeasurement-budget-approval` 的提交 `fef75c3c6a8ca0b561c0215e6ae41280d62ba048`、tree `85194d95e2efb8e6eea75937d9c953a5a4ad58ad`。
- 历史 manifest 归一化 SHA-256：`sha256:72567fe96f61d19d4eca8a5347e3d3fcea7df823975946ec3f464a43d229f1ae`。
- Windows 历史目标候选槽位：Issue `#54` 的 `run-03`、`run-04`、`run-05`、`run-08`；原始分类继续保持 `new-cohort-valid`，新批准队列只在汇总层引用它们。
- macOS 使用 Issue `#54` 的八个同队列有效 Run 与 Issue `#59` 的四个新有效 Run；禁止跨平台统计。

## 精确范围

以下按 Windows PowerShell `Sort-Object` 排序的三十八条路径是唯一允许写集合，`scope_hash` 为 `sha256:d0577e546d2209d10373eccdf335bbcf3cd4caad7906163838c88b461da0b570`。预算 JSON 与两个离线脚本只有在五次条件满足时才创建；实际路径始终可以是该集合的子集。

```text
AGENTS.md
benchmarks/budgets/issue-59-approved-observation.json
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
benchmarks/results/issue-59/manifest.json
benchmarks/results/issue-59/runs/run-01/macos.json
benchmarks/results/issue-59/runs/run-01/windows.json
benchmarks/results/issue-59/runs/run-02/macos.json
benchmarks/results/issue-59/runs/run-02/windows.json
benchmarks/results/issue-59/runs/run-03/macos.json
benchmarks/results/issue-59/runs/run-03/windows.json
benchmarks/results/issue-59/runs/run-04/macos.json
benchmarks/results/issue-59/runs/run-04/windows.json
build.md
docs/plans/2026-07-26-issue-59-epyc-7763-fixed-remeasurement.md
docs/plans/PROJECT-MASTER-PLAN.md
docs/plans/sessions/2026-07-26-issue-59-epyc-7763-fixed-remeasurement.md
docs/reports/issue-59-epyc-7763-fixed-remeasurement.md
docs/workflows/2026-07-26-issue-59-epyc-7763-fixed-remeasurement-runtime.md
err.md
README.md
scripts/performance/Build-InputcodexBudgetApproval.ps1
scripts/performance/Test-InputcodexBudgetApproval.ps1
```

## 执行任务

### Task 1：冻结控制面与历史证据

- [x] 创建 Issue `#59` 和分支 `codex/issue-59-epyc-7763-fixed-remeasurement`。
- [x] 写入本计划、Session Plan、Runtime Workflow、初始报告和 manifest。
- [ ] 以 `apply_patch` 缓存 Issue `#54` 的 manifest 与十六份 JSON，并验证归一化 SHA-256。
- [ ] 运行控制面轻量验证，提交并普通 push 稳定测量基线。

### Task 2：执行四次固定串行测量

- [ ] 对 `run-01` 至 `run-04` 逐个执行 `gh workflow run 'Performance Baseline' --repo nonononull/inputcodex --ref codex/issue-59-epyc-7763-fixed-remeasurement -f mode=measure`。
- [ ] 每次等待 `contract`、`windows`、`macos`、`required` 全部终态后再开始下一次。
- [ ] 下载双平台 Artifact，核验 source、Run、attempt、归一化哈希、schema、状态、hosted 环境、样本合同和 checksum。
- [ ] 每个槽位入库后形成普通 Git 快照并 push；不得并发或跳号。

### Task 3：分类与最终判定

- [ ] Windows 处理器精确等于目标值时计入新批准队列；其他有效处理器仅记录为 `new-cohort-valid`。
- [ ] macOS 按自己的硬键与队列指纹独立分类。
- [ ] 四次全部结束后计算目标队列总数；达到至少五次时生成预算 JSON 与离线复算器，不足五次时记录硬停止且不创建三个可选文件。

### Task 4：验证和 GitHub 交付

- [ ] 运行范围、历史证据、manifest、Evidence、CI 合同、Repository Policy、JSON/PowerShell 与 `git diff --check` 验证。
- [ ] 创建 `Closes #59` 的非 Draft PR，逐条闭环 Review 对话并等待全部 PR CI。
- [ ] 最终 Head 满足范围、Review、CI 与所有者合并证据后，仅使用 Squash Merge。

## 完成标准

- 四个新槽位全部执行且具有不同 `run_id`；不存在 `run-05`。
- 历史十六份 JSON 的内容、原始分类、来源和哈希没有被改写。
- 目标队列达到至少五次时，预算值可由固定公式离线复算；否则报告明确为硬停止。
- 本 Issue 不实施预算 CI、性能优化、产品迁移或 Runner 资源变更。
