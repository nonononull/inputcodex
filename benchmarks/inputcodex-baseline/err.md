# inputcodex-baseline 排错记录

## 先查原则

遇到失败先确认命令包含 `--manifest-path benchmarks/inputcodex-baseline/Cargo.toml`。本工程是独立 Workspace，禁止为解决局部问题修改根 Workspace 成员列表。

## 独立 `Cargo.lock` 缺失或过期

- 现象：带 `--locked` 的测试提示锁文件需要更新。
- 根因：独立 Workspace 的依赖解析结果尚未写入本目录 `Cargo.lock`，或清单已变更。
- 处理：确认清单改动已获 Issue 批准后，运行 `cargo generate-lockfile --manifest-path benchmarks/inputcodex-baseline/Cargo.toml --offline`。
- 验证：重新运行本目录 `build.md` 的格式与测试命令，并确认根 Cargo 文件哈希未变化。

## 构建产物出现在 Git 状态中

- 现象：`benchmarks/inputcodex-baseline/target/` 出现在未跟踪文件列表。
- 根因：独立 Workspace 使用自己的默认 target 目录，而根级 `/target/` 规则不会覆盖该目录。
- 处理：保留仓库根 `.gitignore` 中的 `/benchmarks/inputcodex-baseline/target/`；禁止提交或上传整个 `target/`。
- 验证：`git status --short` 不再列出该目录。

## Parity 场景报告仓库根无效

- 现象：CLI 返回“无效的 inputcodex 仓库根目录”。
- 根因：传入路径不存在，或路径下缺少 `parity/`。
- 处理：从仓库根传入 `.`，GitHub Actions 中使用 checkout 根目录。
- 验证：`parity-repository-validation` 合同测试通过；不得改为运行上游或半成品目录。

## `application-cancel-stale` 误测为陈旧取消

- 现象：初版场景只在 `Loading` 状态发送错误请求 ID 的取消，虽然 checksum 非零，但没有覆盖批准合同要求的“取消后陈旧完成不得覆盖取消状态”。
- 根因：把场景名称中的 `stale` 错误解释为陈旧取消，而不是取消已生效后的陈旧完成。
- 处理：每次迭代固定执行 `begin → cancel 当前请求 → complete 同一请求`，要求取消返回 `Applied`、完成返回 `Stale`，最终状态仍为 `Cancelling`。
- 验证：合同测试 `keeps_cancellation_state_when_stale_completion_arrives` 先以旧实现 RED，再以精确 checksum `3488675146315662320` 转为 GREEN；隔离工程 6 个测试全部通过。

## macOS 微场景 `ns/op` 被整数除法截断为零

- 现象：首次 hosted Run `30168904725` 虽然四 Job 全绿，但 macOS 的 `application-load-complete` 与 `application-cancel-stale` 多个原始样本显示 `nanoseconds_per_operation=0`。
- 根因：`ScenarioMeasurement` 使用 `u128` 保存 `total_nanoseconds / iterations`；当真实平均值小于 1 纳秒时，整数除法直接截断为零，导致结果失去精度。
- 处理：保留总纳秒整数与 checksum 不变，把每操作耗时改为 `f64`，CSV 固定输出六位小数；证据验证器新增 `SCENARIO_PRECISION_INVALID`，拒绝任何零或负的每操作耗时。
- 验证：回归测试固定 `1 ns / 2 ops = 0.5 ns/op` 与 CSV `0.500000`，先 RED 后 GREEN；隔离工程现为 7 个测试通过，本地 100000 次场景输出非零小数。
- 处置：Run `30168904725` 的成功 Artifact 只保留为临时根因证据，不写入固定结果；新 Head 必须重新采集两平台结果。
