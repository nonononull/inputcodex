# inputcodex-platform 排错记录

## 当前合同

- Windows 返回 `PlatformKind::Windows`。
- macOS 返回 `PlatformKind::Macos`。
- 非发布目标返回 `Unsupported / PLATFORM_UNSUPPORTED`，不得伪造成功。

## 2026-07-22：Linux Clippy 报告条件分支类型导入未使用

- 现象：GitHub Actions 运行 `29910847062` 的 `linux-quality` 在 `cargo clippy --workspace --all-targets -- -D warnings` 失败，报告 `tests/platform_contract.rs` 的 `PlatformKind` 未使用；Windows 与 macOS Job 同时成功。
- 根因：测试文件无条件导入 `PlatformKind`，但 Linux 只执行 unsupported 分支，类型只在 Windows/macOS 的条件断言中使用。
- 处理：给 `PlatformKind` 导入增加与断言一致的 `#[cfg(any(target_os = "windows", target_os = "macos"))]`；`PlatformPort` 保持全平台导入。
- 验证：Windows 本地定向 Clippy 通过只能证明 Windows 分支不回归；Linux 修复必须由后续普通提交触发的新 `linux-quality` 运行验证，禁止 rerun 旧失败。

## 排错顺序

条件编译只允许留在本包或极薄启动层。若双平台语义分叉，先检查 application 统一类型，再检查目标条件；禁止复制两套业务规则。
