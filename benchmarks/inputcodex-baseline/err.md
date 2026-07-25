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
