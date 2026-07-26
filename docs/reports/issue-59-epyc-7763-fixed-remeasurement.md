# Issue #59：EPYC 7763 四次固定串行复测报告

status: RUNS_COMPLETED_TARGET_COHORT_READY

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
| `run-02` | [`30190945477`](https://github.com/nonononull/inputcodex/actions/runs/30190945477) | AMD EPYC 7763 / `comparable-valid` | `comparable-valid` | completed |
| `run-03` | [`30191335211`](https://github.com/nonononull/inputcodex/actions/runs/30191335211) | AMD EPYC 9V74 / `new-cohort-valid` | `comparable-valid` | completed |
| `run-04` | [`30191791435`](https://github.com/nonononull/inputcodex/actions/runs/30191791435) | AMD EPYC 7763 / `comparable-valid` | `comparable-valid` | completed |

## 数值状态

- Windows 严格目标队列：历史 `3` + 新命中 `2` = `5`，已满足数值生成前置条件。
- macOS 同队列：历史 `8` + 新有效 `4` = `12`，已满足数值生成前置条件。
- warning/blocking：等待离线复算器生成并验证。
- 预算 CI：未实施。
- Gate 5：继续锁定。

## run-01 证据

- 测量 Head：`735bef8a8b305056728cde072db0874769c95e19`；tree：`b0239a42453f49b106de412f5cb883df5265adc9`。
- `contract`、`windows`、`macos`、`required` 全部成功。
- Windows Artifact `8628415357`，归一化 SHA-256 `sha256:51aef5b03c97ce334c8fb60322cbbd6f13703709d6e325a053ffba5fe759ea79`，环境指纹 `sha256:e3900b38d3689b617e7b92891add66d62b05eefdb8bb0f87d0ec955cd72d8ebc`。
- macOS Artifact `8628392131`，归一化 SHA-256 `sha256:a44579973458d33bf8365526c5a108d5ebc3a3ff819f16b180f77ac28a45c324`，环境指纹 `sha256:e95c8f57f22013020911c87b7ddf74af763daf461d124cf4ae3f9f1e30805efa`。

## run-02 证据

- 测量 Head：`7bd79df652081be6ed3e4b4c41c528cdfdc4df84`；tree：`970aad82df28441050741841297f5ee4dc495976`。
- Run `30190945477` 的 `contract`、`windows`、`macos`、`required` 全部成功。
- Windows Artifact `8628595174`，归一化 SHA-256 `sha256:0c179768bdd5c6df2cf51bf675854584f21e769fd5c4f6d0a83cfd23505eb0b6`，环境指纹精确命中目标 `sha256:f3954543f3cec519568345d9f40341ddeb8991a7d93b3a274cc324b047fb00cb`。
- macOS Artifact `8628571759`，归一化 SHA-256 `sha256:c1b6f9823d4dd6d01b1a37c6c193abb75ecd129cdd9f308b4937d34438a48bbd`，环境指纹继续匹配历史队列。
- Windows 严格目标队列当前为历史 `3` + 新命中 `1` = `4`；按固定合同继续执行 `run-03`、`run-04`。

## run-03 证据

- 测量 Head：`61406e19f52fedbdcad119b1b81d2d72208474bf`；tree：`97f920425f960dd25ead422b95aee3d6f38d39b8`。
- Run `30191335211` 的 `contract`、`windows`、`macos`、`required` 全部成功。
- Windows Artifact `8628704321`，归一化 SHA-256 `sha256:761d3a7ced4d03c9e50286b7859e803b9337949a7490127a6a3e0ab8208b40a7`，处理器为 AMD EPYC 9V74，环境指纹 `sha256:a3097283cfc677908818cc7b665124d2735be8ae1670deff46590d9ab24c213c`，分类 `new-cohort-valid`。
- macOS Artifact `8628685677`，归一化 SHA-256 `sha256:022f94de2c1cd88a358a971abd710be2956002bfa89b0e341ed0ae0c6b542f31`，环境指纹继续匹配历史队列。
- Windows 严格目标队列仍为 `4`；按固定合同继续执行最后的 `run-04`。

## run-04 证据

- 测量 Head：`2e75baadfb641b40314e9690e94117b1cd149b7a`；tree：`bc7917b6665bff43ba8825b54c7d8c3c8c2feea5`。
- Run `30191791435` 的 `contract`、`windows`、`macos`、`required` 全部成功。
- Windows Artifact `8628860710`，归一化 SHA-256 `sha256:7b65505db2a6f25fc73b34de32cc96b001f8ede05e82d072c0c11d879f381f17`，处理器为 AMD EPYC 7763，完整环境指纹精确命中 `sha256:f3954543f3cec519568345d9f40341ddeb8991a7d93b3a274cc324b047fb00cb`，分类 `comparable-valid`。
- macOS Artifact `8628833820`，归一化 SHA-256 `sha256:b172165bdcb1a57b65c92e411759a297059f7ac9ebd06c4da234d6544ff8febe`，完整环境指纹继续匹配历史队列，分类 `comparable-valid`。
- 四个固定新槽位已全部完成；Windows 严格目标队列达到 `5`，macOS 同队列达到 `12`，允许进入离线预算数值生成，但预算 CI 与 Gate 5 仍未获授权。

## 控制面证据

- Issue：`https://github.com/nonononull/inputcodex/issues/59`。
- 分支：`codex/issue-59-epyc-7763-fixed-remeasurement`。
- 基线：`main@d9d1ed77b9796ac6a99e250d1547217a39426aa9`。
- 精确范围：38 路径，`scope_hash=sha256:d0577e546d2209d10373eccdf335bbcf3cd4caad7906163838c88b461da0b570`。
- AGOS 默认入口：`needs-input/unregistered`，已按项目原生规则绕过，无跨仓写入。
