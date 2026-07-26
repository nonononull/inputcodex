# Issue #54：五次性能复测与数值批准报告

## 当前结论

本报告已建立 Issue `#54` 的原生审计面，但尚未写入任何 Issue `#54` hosted 样本、统计结果或 warning/blocking 数值。项目仍处于 `baseline-only`：Issue `#32` 的结果仅为种子证据，Issue `#55` 只修复了显式复测入口。

数值形成的所有前提已冻结：Windows/macOS 分开管理、每个平台至少五个独立同队列 `comparable-valid` Run、保留全部 IQR/无效/漂移/事故证据、run-level 稳健统计和显式安全裕量。预算 CI、性能优化、Ruleset 和 Gate 5 未获本 Issue 授权。

## 依据与授权

- 跟踪 Issue：`https://github.com/nonononull/inputcodex/issues/54`。
- 所有者数值预授权：Issue `#54` 记录项目所有者 Windows 本机时间 `2026-07-26 06:19:35 +08:00` 的“按你推荐来 不用我二次批准 直接安排后续”决定；该决定只在 ADR `0004` 的样本、分类和统计条件满足后用于落盘具体数值。
- 前置完成：Issue `#55` / PR `#56` 已以 Squash 提交 `325bb2419548bc076502065dc583f54f4fddd582` 进入 `main`，显式手工 `mode=measure` 可用；合并后 main CI `30180436105` 七 Job 与 Performance Baseline `30180436063` 四 Job 全绿，Artifact 均为 `0`。
- 当前基线：`main@325bb2419548bc076502065dc583f54f4fddd582`，`release_audit=current`。

## 执行合同

- 原始样本只能来自 `codex/issue-54-performance-remeasurement-budget-approval` 的串行 `workflow_dispatch mode=measure` Run；最多八次、不同 `run_id`，每次等待 Windows/macOS/contract/required 终态后才继续。
- 原始 Artifact 只缓存到固定 `run-01` 至 `run-08` 槽位；每次下载均复核 source、双平台 `run_id`、Artifact ID、归一化 SHA-256、环境、schema、状态和 checksum。
- 同平台的首个证据完整结果建立候选队列；环境指纹变化为 `new-cohort-valid`，合同错误为 `evidence-invalid`，外部事故必须有可复核 Actions/状态证据；只有 `comparable-valid` 进入五次统计。
- 数值控制面在每个平台至少五个可比样本后才生成。首次 view 使用 ms、Working Set 使用 MiB、Rust 场景使用 ns/op；每个 median/P95 lane 保存 center、MAD、min/max、安全裕量、warning、blocking、单位、来源和队列指纹。
- 本 Issue 保持 `budget_ci_enabled=false` 与观测阶段；后续独立预算 CI Issue 才能把已批准数值接入 `approved-observation`。

## 初始状态

| 平台 | comparable-valid | new-cohort-valid | evidence-invalid | external-incident | 数值状态 |
| --- | ---: | ---: | ---: | ---: | --- |
| Windows | 0 | 0 | 0 | 0 | 未生成 |
| macOS | 0 | 0 | 0 | 0 | 未生成 |

当每个平台达到五个可比样本后，本报告将回写每个 Run、分类、队列、统计、公式、安全裕量、数值和 PR/CI/Review 证据。若八次上限仍不足，报告只记录原因，不伪造样本、不扩展路径、不填写数值。
