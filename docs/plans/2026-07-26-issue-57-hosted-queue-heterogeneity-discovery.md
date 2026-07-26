# Issue #57：Hosted 队列异构性 Discovery 计划

## 目标

将 Issue `#54` 的严格停止结果转化为可复核的后续决策材料。此 Issue 只比较路径、风险、停止规则和所有者决策点；不启动新采样，不修改性能合同或 ADR，不产生预算数值。

## 已冻结事实

- 基线为 `main@325bb2419548bc076502065dc583f54f4fddd582`，`release_audit=current`。
- Issue `#54` 严格串行完成 `run-01` 至 `run-08`，每次的 `contract`、`windows`、`macos`、`required` Job 都成功；原始结果共 `16` 个 Artifact。
- macOS 的 `8` 次结果均为 `comparable-valid`。Windows 只有 AMD EPYC 9V74 的 `2` 次 `comparable-valid`；AMD EPYC 7763 有 `4` 次 `new-cohort-valid`，Intel Xeon 8370C 和 Intel Xeon 6973P-C 各有 `1` 次 `new-cohort-valid`。
- Issue `#54` 的清单 SHA-256 为 `72567fe96f61d8d4eca8a5347e3d3fcea7df823975946ec3f464a43d229f1ae`。它已经到达 `max_serial_runs=8`，不能创建 `run-09`。
- ADR `0004` 将处理器纳入队列指纹；把不同 CPU 的 Windows 样本混成一个预算群组会降低可诊断性，不能作为隐式修复。

## 候选路径与推荐

| 路径 | 内容 | 优点 | 风险与前置决策 |
| --- | --- | --- | --- |
| A（推荐候选） | 保持现有硬队列语义，另建受控复测 Issue，预先冻结有限槽位上限、停止条件和每次分类证据。 | 不改变已批准的统计含义，不把环境噪声写成产品预算。 | 公开 hosted Runner 不能选择 CPU；必须由所有者决定是否值得追加有限采样，达到上限仍不足五次时必须再次停止。 |
| B | 先建立一致性例外，审查 CPU 是否应从硬队列键移出或降级。 | 可能提高样本利用率。 | 会改变 ADR/合同语义，可能把 CPU 性能差异混入预算；必须有独立 Issue、统计风险说明、ADR/合同变更与所有者明确决定。 |
| C | 单独评估更均质的 Runner。 | 理论上可提高环境一致性。 | 可能涉及 Larger Runner、付费或 self-hosted 资源及安全/维护成本；必须先证明 hosted 不可行并由所有者在独立资源决策中批准，不能默认采用。 |

推荐 A 只表示“语义上最保守且最符合现有 ADR”，不构成路径选择。Issue `#57` 合并后仍保持开放，直到项目所有者明确选择 A、B、C 之一，或决定永久不继续预算采样。

## 精确范围

以下排序后的八条路径是本 Issue 唯一允许写集合，`scope_hash` 为 `sha256:26cc8ba51b7926c0898be56f1cec23c623963b2e944295d14d3d46bf650cd953`：

```text
AGENTS.md
README.md
docs/plans/2026-07-26-issue-57-hosted-queue-heterogeneity-discovery.md
docs/plans/PROJECT-MASTER-PLAN.md
docs/plans/sessions/2026-07-26-issue-57-hosted-queue-heterogeneity-discovery.md
docs/reports/issue-57-hosted-queue-heterogeneity-discovery.md
docs/workflows/2026-07-26-issue-57-hosted-queue-heterogeneity-discovery-runtime.md
err.md
```

明确禁止修改 `benchmarks/`、`.github/workflows/`、`scripts/`、`Cargo.toml`、`Cargo.lock`、产品源码、上游缓存、Ruleset、Release 和任何 AGOS 文件。

## 执行批次

1. 以本机 Windows 时间、GitHub API、Issue `#54` 分支的只读清单、ADR `0004` 和现有 `err.md` 核验停止根因。
2. 将八次结果、三条路径、推荐依据、禁止面和所有者决策点写入项目原生计划、Session Plan、Runtime Workflow、报告和总控文档。
3. 运行本地轻量 Evidence、CI 合同、仓库政策、空白和八路径审计；不运行全量 Rust 构建，不发起 Performance Workflow。
4. 以普通提交和普通 push 创建关联但不关闭 Issue `#57` 的非 Draft PR；Review 对话必须逐条写明根因、处理和验证。满足全部门禁且实际路径严格匹配八路径时，才可按当前会话的项目所有者直接授权进行 Squash Merge。

## 停止条件

- 需要选择 A、B、C 任一路径，或需要采样、修改 ADR/合同、使用付费/Larger/self-hosted Runner。
- 实际差异超出八路径或 `scope_hash` 漂移。
- 任何本地验证、PR CI、Review 对话、Git 远端连通性或仓库政策不能闭环。
- 发现 Issue `#54` 的原始结果、分类或来源 Run 无法复核。
