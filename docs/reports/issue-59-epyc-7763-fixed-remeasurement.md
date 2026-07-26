# Issue #59：EPYC 7763 四次固定串行复测报告

status: CONTROL_PLANE_FROZEN_PENDING_RUNS

## 当前结论

项目所有者已批准方案 A。Issue `#59` 保持 ADR `0004` CPU 硬队列语义，以 `AMD EPYC 7763 64-Core Processor` 为 Windows 目标队列，固定执行四次新的 GitHub-hosted 双平台测量。四次后仍不足五份目标队列样本即硬停止，不创建 `run-05`。

## 历史证据

- Issue `#54` 的八次 Run、十六份原始 JSON 与 manifest 作为只读证据缓存；原始 `new-cohort-valid` 分类不会改写。
- 历史 manifest 归一化 SHA-256：`sha256:72567fe96f61d19d4eca8a5347e3d3fcea7df823975946ec3f464a43d229f1ae`。
- Windows 历史 EPYC 7763 候选为 `run-03`、`run-04`、`run-05`、`run-08`，共 `4` 次。
- macOS 历史同队列有效样本为 `8` 次。

## 新槽位

| 槽位 | GitHub Run | Windows CPU / 分类 | macOS 分类 | 状态 |
| --- | --- | --- | --- | --- |
| `run-01` | 待执行 | 待执行 | 待执行 | pending |
| `run-02` | 待执行 | 待执行 | 待执行 | pending |
| `run-03` | 待执行 | 待执行 | 待执行 | pending |
| `run-04` | 待执行 | 待执行 | 待执行 | pending |

## 数值状态

- warning/blocking：未生成。
- 预算 CI：未实施。
- Gate 5：继续锁定。

## 控制面证据

- Issue：`https://github.com/nonononull/inputcodex/issues/59`。
- 分支：`codex/issue-59-epyc-7763-fixed-remeasurement`。
- 基线：`main@d9d1ed77b9796ac6a99e250d1547217a39426aa9`。
- 精确范围：38 路径，`scope_hash=sha256:d0577e546d2209d10373eccdf335bbcf3cd4caad7906163838c88b461da0b570`。
- AGOS 默认入口：`needs-input/unregistered`，已按项目原生规则绕过，无跨仓写入。
