# Issue #57 Hosted 队列异构性 Discovery 报告

## 结论

Issue `#54` 的预算数值预授权没有触发，原因是 Windows 的同队列五次硬约束未满足，而不是性能回归、采集失败或 GitHub Actions 事故。Issue `#57` 只完成后续治理路径的证据与比较；当前没有 A/B/C 路径被选择，预算、预算 CI、优化和 Gate 5 继续锁定。

## #54 复核证据

- 基线：`main@325bb2419548bc076502065dc583f54f4fddd582`；`max_serial_runs=8`。
- 清单：`benchmarks/results/issue-54/manifest.json`，SHA-256 为 `72567fe96f61d8d4eca8a5347e3d3fcea7df823975946ec3f464a43d229f1ae`。
- Job：八个 Run 均完成 `contract`、`windows`、`macos` 与 `required`；原始证据为 `16` 个成功 Artifact，`evidence-invalid=0`、`external-incident=0`。

| 槽位 | GitHub Run | Windows 分类与 CPU | macOS 分类 |
| --- | --- | --- | --- |
| `run-01` | [`30181234619`](https://github.com/nonononull/inputcodex/actions/runs/30181234619) | `comparable-valid` / AMD EPYC 9V74 | `comparable-valid` |
| `run-02` | [`30181776729`](https://github.com/nonononull/inputcodex/actions/runs/30181776729) | `new-cohort-valid` / Intel Xeon 8370C | `comparable-valid` |
| `run-03` | [`30182194802`](https://github.com/nonononull/inputcodex/actions/runs/30182194802) | `new-cohort-valid` / AMD EPYC 7763 | `comparable-valid` |
| `run-04` | [`30182672835`](https://github.com/nonononull/inputcodex/actions/runs/30182672835) | `new-cohort-valid` / AMD EPYC 7763 | `comparable-valid` |
| `run-05` | [`30182870392`](https://github.com/nonononull/inputcodex/actions/runs/30182870392) | `new-cohort-valid` / AMD EPYC 7763 | `comparable-valid` |
| `run-06` | [`30183070190`](https://github.com/nonononull/inputcodex/actions/runs/30183070190) | `new-cohort-valid` / Intel Xeon 6973P-C | `comparable-valid` |
| `run-07` | [`30183239399`](https://github.com/nonononull/inputcodex/actions/runs/30183239399) | `comparable-valid` / AMD EPYC 9V74 | `comparable-valid` |
| `run-08` | [`30183422699`](https://github.com/nonononull/inputcodex/actions/runs/30183422699) | `new-cohort-valid` / AMD EPYC 7763 | `comparable-valid` |

Windows 队列计数为 AMD EPYC 9V74=`2`、AMD EPYC 7763=`4`、Intel Xeon 8370C=`1`、Intel Xeon 6973P-C=`1`；即使改选最多的次级队列也只有四次，不能绕过 ADR `0004` 的五次硬约束。

## 路径比较

| 路径 | 可做什么 | 不能做什么 | 所有者下一决定 |
| --- | --- | --- | --- |
| A：保持硬队列 | 新建独立、有限槽位的复测 Issue，并为每次 Run 记录分类。 | 不能恢复 `#54`、不能无限采样、不能假设能指定 CPU。 | 是否为统计有效性接受新的有限采样上限及停止规则。 |
| B：一致性例外 | 在独立 Issue 中研究 CPU 是否仍应为硬键，并提供统计风险和 ADR/合同变更。 | 不能把现有 `new-cohort-valid` 样本直接改成可比。 | 是否接受明确记录的语义与统计风险。 |
| C：Runner 决策 | 在独立资源决策中比较 GitHub-hosted 可行性、成本、安全和维护责任。 | 不默认采用 Larger、付费或 self-hosted Runner。 | 是否批准单独资源调查或投入。 |

推荐候选为 A：它最大限度保持已经批准的可比性定义与失败可诊断性；它不是自动执行命令。若所有者未选择 A，预算工作仍停在此处。

## 禁止面与交接

- 本 Issue 没有发起新的 Performance Workflow，没有创建 `run-09`，没有复制或修改 `benchmarks/` 原始结果。
- 本 Issue 没有修改 ADR、采集器、验证器、Workflow、预算 JSON、预算 CI、Rust 源码、上游、Ruleset、Release、付费 Runner 或 AGOS。
- 关联 PR 使用 `Refs #57` 而不是关闭关键字；即使文档合并，Issue `#57` 仍保持开放，等待项目所有者作出实质路径决定。

## 本地验证证据

- 本机 Windows 时间 `2026-07-26 10:36:50 +08:00`：八路径审计通过，实际路径数为 `8`，`scope_hash=sha256:26cc8ba51b7926c0898be56f1cec23c623963b2e944295d14d3d46bf650cd953`。
- `pwsh -NoProfile -File scripts/performance/Test-InputcodexBaseline.ps1 -RepositoryRoot . -Mode Evidence` 返回 `ok=true`、`violation_count=0`。
- `pwsh -NoProfile -File scripts/ci/Test-CiScripts.ps1` 返回 `CI_CONTRACT_GREEN passed=34`。
- `pwsh -NoProfile -File scripts/ci/Verify-RepositoryPolicy.ps1 -RepositoryRoot .` 返回 `ok=true`、`violation_count=0`。
- `git diff --check` 与新增文档尾随空白审计均通过；本次没有执行全量 Rust 构建或 GitHub-hosted 测量。
