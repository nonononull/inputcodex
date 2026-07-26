# Issue #59：EPYC 7763 四次固定串行复测报告

status: RUNS_IN_PROGRESS_1_OF_4

## 当前结论

项目所有者已批准方案 A。Issue `#59` 保持 ADR `0004` 完整环境指纹语义，以 AMD EPYC 7763 主导指纹 `sha256:f3954543f3cec519568345d9f40341ddeb8991a7d93b3a274cc324b047fb00cb` 为 Windows 目标队列，固定执行四次新的 GitHub-hosted 双平台测量。四次后仍不足五份目标队列样本即硬停止，不创建 `run-05`。

## 历史证据

- Issue `#54` 的八次 Run、十六份原始 JSON 与 manifest 作为只读证据缓存；原始 `new-cohort-valid` 分类不会改写。
- 历史 manifest 归一化 SHA-256：`sha256:72567fe96f61d19d4eca8a5347e3d3fcea7df823975946ec3f464a43d229f1ae`。
- Windows 历史严格目标指纹样本为 `run-03`、`run-05`、`run-08`，共 `3` 次。`run-04` 的 CPU 相同，但总内存值为 `17178693632`，不同于目标的 `17174360064`，因此保持独立队列。
- macOS 历史同队列有效样本为 `8` 次。

## 新槽位

| 槽位 | GitHub Run | Windows CPU / 分类 | macOS 分类 | 状态 |
| --- | --- | --- | --- | --- |
| `run-01` | [`30190401855`](https://github.com/nonononull/inputcodex/actions/runs/30190401855) | Intel Xeon 8370C / `new-cohort-valid` | `comparable-valid` | completed |
| `run-02` | 待执行 | 待执行 | 待执行 | pending |
| `run-03` | 待执行 | 待执行 | 待执行 | pending |
| `run-04` | 待执行 | 待执行 | 待执行 | pending |

## 数值状态

- warning/blocking：未生成。
- 预算 CI：未实施。
- Gate 5：继续锁定。

## run-01 证据

- 测量 Head：`735bef8a8b305056728cde072db0874769c95e19`；tree：`b0239a42453f49b106de412f5cb883df5265adc9`。
- `contract`、`windows`、`macos`、`required` 全部成功。
- Windows Artifact `8628415357`，归一化 SHA-256 `sha256:51aef5b03c97ce334c8fb60322cbbd6f13703709d6e325a053ffba5fe759ea79`，环境指纹 `sha256:e3900b38d3689b617e7b92891add66d62b05eefdb8bb0f87d0ec955cd72d8ebc`。
- macOS Artifact `8628392131`，归一化 SHA-256 `sha256:a44579973458d33bf8365526c5a108d5ebc3a3ff819f16b180f77ac28a45c324`，环境指纹 `sha256:e95c8f57f22013020911c87b7ddf74af763daf461d124cf4ae3f9f1e30805efa`。

## 控制面证据

- Issue：`https://github.com/nonononull/inputcodex/issues/59`。
- 分支：`codex/issue-59-epyc-7763-fixed-remeasurement`。
- 基线：`main@d9d1ed77b9796ac6a99e250d1547217a39426aa9`。
- 精确范围：38 路径，`scope_hash=sha256:d0577e546d2209d10373eccdf335bbcf3cd4caad7906163838c88b461da0b570`。
- AGOS 默认入口：`needs-input/unregistered`，已按项目原生规则绕过，无跨仓写入。
