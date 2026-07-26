# Issue #54：五次性能复测与数值批准报告

## 当前结论

Issue `#54` 已严格用尽 `run-01` 至 `run-08` 的八次 GitHub-hosted `workflow_dispatch mode=measure` 槽位。八次 Run 的 `contract`、`windows`、`macos`、`required` Job 均成功；原始 Artifact、source、归一化 SHA-256、样本合同、IQR 标记、环境与 Rust checksum 均已缓存和复核。

Windows 只有 `2` 个初始队列的 `comparable-valid` 样本，另有 `6` 个 `new-cohort-valid` 样本；macOS 有 `8` 个同队列 `comparable-valid` 样本。Windows 未满足 ADR `0004` 的每平台至少五个同队列样本硬约束，因此本 Issue 不生成 warning/blocking 数值、不创建离线预算脚本、不接入预算 CI，也不进入 Gate 5。

## 依据与授权

- 跟踪 Issue：`https://github.com/nonononull/inputcodex/issues/54`。
- 所有者数值预授权只在 ADR `0004` 的样本、分类和统计条件满足后生效；本次 Windows 条件未满足，因此预授权未被使用。
- Issue `#55` / PR `#56` 已以 Squash 提交 `325bb2419548bc076502065dc583f54f4fddd582` 进入 `main`；显式 `workflow_dispatch mode=measure` 是本次唯一复测入口。
- Issue `#57` 只做 hosted 队列异构性的可行性 Discovery；未经项目所有者决策，不修订队列语义、不追加测量、不使用 self-hosted 或付费 Runner。

## 已执行合同

- 每次只触发一个不同 `run_id`，并在 `contract`、`windows`、`macos`、`required` 终态成功后才开始下一次。
- 每份 Artifact 均核验双平台 source、Run/attempt、Artifact ID、归一化 SHA-256、schema、状态、`github-hosted`、样本合同、IQR 标记和 Rust checksum。
- 分类仅使用 `comparable-valid`、`new-cohort-valid`、`evidence-invalid`、`external-incident`；慢样本和 IQR 标记没有被删除或误分类。
- Windows 的硬键保持不变，但 GitHub-hosted Runner 出现四种 CPU 环境指纹；这是有效队列异构性，不是性能回归、采集错误或外部事故。

## 最终状态

| 平台 | comparable-valid | new-cohort-valid | evidence-invalid | external-incident | 数值状态 |
| --- | ---: | ---: | ---: | ---: | --- |
| Windows | 2 | 6 | 0 | 0 | 因八次上限不足五个同队列样本而未生成 |
| macOS | 8 | 0 | 0 | 0 | Windows 前提未满足，未生成 |

## 缓存 Run 证据

| 槽位 | Actions Run | Windows 分类 / CPU | macOS 分类 | Artifact |
| --- | --- | --- | --- | --- |
| `run-01` | [`30181234619`](https://github.com/nonononull/inputcodex/actions/runs/30181234619) | `comparable-valid` / AMD EPYC 9V74 | `comparable-valid` | Windows `8625649016`；macOS `8625629637` |
| `run-02` | [`30181776729`](https://github.com/nonononull/inputcodex/actions/runs/30181776729) | `new-cohort-valid` / Intel Xeon 8370C | `comparable-valid` | Windows `8625816830`；macOS `8625792535` |
| `run-03` | [`30182194802`](https://github.com/nonononull/inputcodex/actions/runs/30182194802) | `new-cohort-valid` / AMD EPYC 7763 | `comparable-valid` | Windows `8625950714`；macOS `8625928968` |
| `run-04` | [`30182672835`](https://github.com/nonononull/inputcodex/actions/runs/30182672835) | `new-cohort-valid` / AMD EPYC 7763 | `comparable-valid` | Windows `8626093665`；macOS `8626078369` |
| `run-05` | [`30182870392`](https://github.com/nonononull/inputcodex/actions/runs/30182870392) | `new-cohort-valid` / AMD EPYC 7763 | `comparable-valid` | Windows `8626153119`；macOS `8626132446` |
| `run-06` | [`30183070190`](https://github.com/nonononull/inputcodex/actions/runs/30183070190) | `new-cohort-valid` / Intel Xeon 6973P-C | `comparable-valid` | Windows `8626212794`；macOS `8626209442` |
| `run-07` | [`30183239399`](https://github.com/nonononull/inputcodex/actions/runs/30183239399) | `comparable-valid` / AMD EPYC 9V74 | `comparable-valid` | Windows `8626281401`；macOS `8626274294` |
| `run-08` | [`30183422699`](https://github.com/nonononull/inputcodex/actions/runs/30183422699) | `new-cohort-valid` / AMD EPYC 7763 | `comparable-valid` | Windows `8626351896`；macOS `8626330268` |

完整 source、tree、哈希、环境指纹、样本合同、IQR 和 checksum 见 `benchmarks/results/issue-54/manifest.json` 与各槽位原始 JSON。

## 停止结论

- Windows 初始 AMD EPYC 9V74 队列只有 `run-01`、`run-07` 两次；AMD EPYC 7763 观察队列有四次，另有两个 Intel 单次队列。即使选择次级队列也仅有四次，不能绕过五次硬约束。
- `run-09` 不存在，也不会创建；不会通过重跑、删除、混合队列、降低语义或改写历史来制造预算。
- Issue `#54` 的下一步不是 PR 合并预算，而是等待 Issue `#57` 的 Discovery、精确范围和项目所有者决策。预算 CI、优化和 Gate 5 继续锁定。
