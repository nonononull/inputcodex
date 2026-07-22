# inputcodex-platform 排错记录

## 当前合同

- Windows 返回 `PlatformKind::Windows`。
- macOS 返回 `PlatformKind::Macos`。
- 非发布目标返回 `Unsupported / PLATFORM_UNSUPPORTED`，不得伪造成功。

## 2026-07-22：Linux Clippy 报告条件分支类型导入未使用

- 现象：GitHub Actions 运行 `29910847062` 的 `linux-quality` 在 `cargo clippy --workspace --all-targets -- -D warnings` 失败，报告 `tests/platform_contract.rs` 的 `PlatformKind` 未使用；Windows 与 macOS Job 同时成功。
- 根因：测试文件无条件导入 `PlatformKind`，但 Linux 只执行 unsupported 分支，类型只在 Windows/macOS 的条件断言中使用。
- 处理：给 `PlatformKind` 导入增加与断言一致的 `#[cfg(any(target_os = "windows", target_os = "macos"))]`；`PlatformPort` 保持全平台导入。
- 验证：修复提交 `bd4610f6e98dc597bddf02c584d0f0fc616cac7b` 触发运行 `29911337652`，Linux、Windows、macOS 与 `required` 全绿；未 rerun 旧失败。

## 2026-07-22：Windows 条件编译失败语义

- 现象：运行 `29916309635` 仅 Windows 与 `required` 失败，Linux/macOS 成功。
- 根因：受控 Windows cfg 探针触发稳定标记 `GATE3_WINDOWS_CONDITIONAL_COMPILE_FAILURE`。
- 处理：后续普通提交 `436f7273b589f0dcca0c574aae611bf919d687f8` 删除探针，不修改 `PlatformKind` 语义。
- 验证：运行 `29916670916` 六 Job 全绿；失败 Artifact 只有 Windows build/toolchain 与 `required.json` 白名单。

## 2026-07-22：macOS 条件编译失败语义

- 现象：运行 `29917061781` 仅 macOS 与 `required` 失败，Linux/Windows 成功。
- 根因：受控 macOS cfg 探针触发稳定标记 `GATE3_MACOS_CONDITIONAL_COMPILE_FAILURE`。
- 处理：后续普通提交 `41c0cc2924a45f3d8e2a5fe2e47e2e254a9dbb3b` 删除探针，不修改 `PlatformKind` 语义。
- 验证：运行 `29917649550` 六 Job 全绿；失败 Artifact 只有 macOS build/toolchain 与 `required.json` 白名单。

## 排错顺序

条件编译只允许留在本包或极薄启动层。若双平台语义分叉，先检查 application 统一类型，再检查目标条件；禁止复制两套业务规则。
